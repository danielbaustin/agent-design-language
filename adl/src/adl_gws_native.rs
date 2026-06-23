use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use google_workspace::discovery::{fetch_discovery_document, RestDescription, RestMethod};
use google_workspace::services::resolve_service;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub const ADL_GWS_LIVE_MODE_ENV: &str = "ADL_GWS_LIVE_MODE";
pub const ADL_GWS_WRITE_APPROVAL_ENV: &str = "ADL_GWS_WRITE_APPROVAL";
pub const ADL_GWS_CREDENTIALS_FILE_ENV: &str = "ADL_GWS_CREDENTIALS_FILE";
pub const ADL_GWS_TOKEN_ENV: &str = "ADL_GWS_TOKEN";
pub const ADL_GWS_PROJECT_ID_ENV: &str = "ADL_GWS_PROJECT_ID";
pub const ADL_GWS_DEFAULT_SCOPE: &str = "https://www.googleapis.com/auth/drive";
pub const ADL_GWS_SCOPE_DRIVE_FILE: &str = "https://www.googleapis.com/auth/drive.file";
pub const ADL_GWS_SCOPE_DOCS: &str = "https://www.googleapis.com/auth/documents";
pub const ADL_GWS_SCOPE_SHEETS: &str = "https://www.googleapis.com/auth/spreadsheets";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceExecutionMode {
    FixtureBacked,
    DryRun,
    Execute,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceSkipReason {
    DryRunOnly,
    MissingAuth,
    MissingScopes,
    MissingBinding,
    RuntimeUnavailable,
    VerificationMismatch,
    AmbiguousTarget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceScopeBinding {
    pub root_folder_id: String,
    pub folder_path: Vec<String>,
    pub file_name: Option<String>,
    pub file_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceFileRef {
    pub file_id: String,
    pub name: String,
    pub mime_type: String,
    pub parent_ids: Vec<String>,
    pub modified_time: Option<String>,
    pub web_view_link: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAuthContext {
    pub source: String,
    pub quota_project: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceMethodDescriptor {
    pub method_id: String,
    pub http_method: String,
    pub path: String,
    pub upload_path: Option<String>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceDriveMethodCatalog {
    pub service_name: String,
    pub service_version: String,
    pub list: WorkspaceMethodDescriptor,
    pub get: WorkspaceMethodDescriptor,
    pub create: WorkspaceMethodDescriptor,
    pub update: WorkspaceMethodDescriptor,
}

#[async_trait]
pub trait WorkspaceAccessTokenProvider: Send + Sync {
    async fn access_token(&self, scopes: &[&str]) -> Result<(String, WorkspaceAuthContext)>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultWorkspaceAccessTokenProvider;

#[derive(Debug)]
enum WorkspaceCredential {
    AuthorizedUser(yup_oauth2::authorized_user::AuthorizedUserSecret),
    ServiceAccount(yup_oauth2::ServiceAccountKey),
}

#[async_trait]
impl WorkspaceAccessTokenProvider for DefaultWorkspaceAccessTokenProvider {
    async fn access_token(&self, scopes: &[&str]) -> Result<(String, WorkspaceAuthContext)> {
        if let Ok(token) = std::env::var(ADL_GWS_TOKEN_ENV) {
            if !token.trim().is_empty() {
                return Ok((
                    token,
                    WorkspaceAuthContext {
                        source: ADL_GWS_TOKEN_ENV.to_string(),
                        quota_project: quota_project_hint(),
                    },
                ));
            }
        }

        let credentials = load_workspace_credentials().await?;
        let token = get_token_from_credentials(scopes, credentials).await?;
        Ok((
            token,
            WorkspaceAuthContext {
                source: "credential_store".to_string(),
                quota_project: quota_project_hint(),
            },
        ))
    }
}

pub fn parse_workspace_execution_mode_from_env() -> WorkspaceExecutionMode {
    match std::env::var(ADL_GWS_LIVE_MODE_ENV)
        .unwrap_or_else(|_| "fixture".to_string())
        .to_ascii_lowercase()
        .as_str()
    {
        "execute" | "enabled" | "live" => WorkspaceExecutionMode::Execute,
        "dry_run" | "dry-run" => WorkspaceExecutionMode::DryRun,
        _ => WorkspaceExecutionMode::FixtureBacked,
    }
}

pub fn parse_workspace_write_approval_from_env() -> bool {
    matches!(
        std::env::var(ADL_GWS_WRITE_APPROVAL_ENV)
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str(),
        "1" | "true" | "yes" | "approve" | "approved"
    )
}

pub fn tracked_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(relative)
}

pub async fn fetch_workspace_drive_method_catalog(
    cache_dir: Option<&Path>,
) -> Result<WorkspaceDriveMethodCatalog> {
    let (service_name, service_version) =
        resolve_service("drive").map_err(|e| anyhow!(e.to_string()))?;
    let doc = fetch_discovery_document(&service_name, &service_version, cache_dir)
        .await
        .with_context(|| "fetch drive discovery document".to_string())?;
    parse_workspace_drive_method_catalog(&doc)
}

pub fn parse_workspace_drive_method_catalog(
    doc: &RestDescription,
) -> Result<WorkspaceDriveMethodCatalog> {
    let files = doc
        .resources
        .get("files")
        .ok_or_else(|| anyhow!("drive discovery missing files resource"))?;
    let list = workspace_method_descriptor(
        "drive.files.list",
        files
            .methods
            .get("list")
            .ok_or_else(|| anyhow!("drive discovery missing files.list"))?,
    )?;
    let get = workspace_method_descriptor(
        "drive.files.get",
        files
            .methods
            .get("get")
            .ok_or_else(|| anyhow!("drive discovery missing files.get"))?,
    )?;
    let create = workspace_method_descriptor(
        "drive.files.create",
        files
            .methods
            .get("create")
            .ok_or_else(|| anyhow!("drive discovery missing files.create"))?,
    )?;
    let update = workspace_method_descriptor(
        "drive.files.update",
        files
            .methods
            .get("update")
            .ok_or_else(|| anyhow!("drive discovery missing files.update"))?,
    )?;
    Ok(WorkspaceDriveMethodCatalog {
        service_name: doc.name.clone(),
        service_version: doc.version.clone(),
        list,
        get,
        create,
        update,
    })
}

pub fn default_drive_method_catalog() -> WorkspaceDriveMethodCatalog {
    WorkspaceDriveMethodCatalog {
        service_name: "drive".to_string(),
        service_version: "v3".to_string(),
        list: WorkspaceMethodDescriptor {
            method_id: "drive.files.list".to_string(),
            http_method: "GET".to_string(),
            path: "drive/v3/files".to_string(),
            upload_path: None,
            scopes: vec![ADL_GWS_DEFAULT_SCOPE.to_string()],
        },
        get: WorkspaceMethodDescriptor {
            method_id: "drive.files.get".to_string(),
            http_method: "GET".to_string(),
            path: "drive/v3/files/{fileId}".to_string(),
            upload_path: None,
            scopes: vec![ADL_GWS_DEFAULT_SCOPE.to_string()],
        },
        create: WorkspaceMethodDescriptor {
            method_id: "drive.files.create".to_string(),
            http_method: "POST".to_string(),
            path: "drive/v3/files".to_string(),
            upload_path: Some("/upload/drive/v3/files".to_string()),
            scopes: vec![ADL_GWS_DEFAULT_SCOPE.to_string()],
        },
        update: WorkspaceMethodDescriptor {
            method_id: "drive.files.update".to_string(),
            http_method: "PATCH".to_string(),
            path: "drive/v3/files/{fileId}".to_string(),
            upload_path: Some("/upload/drive/v3/files/{fileId}".to_string()),
            scopes: vec![ADL_GWS_DEFAULT_SCOPE.to_string()],
        },
    }
}

pub fn workspace_method_descriptor(
    method_id: &str,
    method: &RestMethod,
) -> Result<WorkspaceMethodDescriptor> {
    let upload_path = method
        .media_upload
        .as_ref()
        .and_then(|upload| upload.protocols.as_ref())
        .and_then(|protocols| protocols.simple.as_ref())
        .map(|simple| simple.path.clone());

    Ok(WorkspaceMethodDescriptor {
        method_id: method_id.to_string(),
        http_method: method.http_method.clone(),
        path: method.path.clone(),
        upload_path,
        scopes: if method.scopes.is_empty() {
            vec![ADL_GWS_DEFAULT_SCOPE.to_string()]
        } else {
            method.scopes.clone()
        },
    })
}

fn quota_project_hint() -> Option<String> {
    if let Ok(project_id) = std::env::var(ADL_GWS_PROJECT_ID_ENV) {
        if !project_id.is_empty() {
            return Some(project_id);
        }
    }

    let adc_path = std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
        .ok()
        .map(PathBuf::from)
        .or_else(adc_well_known_path)?;
    let content = std::fs::read_to_string(adc_path).ok()?;
    let value: serde_json::Value = serde_json::from_str(&content).ok()?;
    value
        .get("quota_project_id")
        .and_then(|v| v.as_str())
        .map(ToString::to_string)
}

fn adc_well_known_path() -> Option<PathBuf> {
    dirs::home_dir().map(|dir| {
        dir.join(".config")
            .join("gcloud")
            .join("application_default_credentials.json")
    })
}

async fn get_token_from_credentials(
    scopes: &[&str],
    credentials: WorkspaceCredential,
) -> Result<String> {
    match credentials {
        WorkspaceCredential::AuthorizedUser(secret) => {
            let auth = yup_oauth2::AuthorizedUserAuthenticator::builder(secret)
                .build()
                .await
                .context("build authorized user authenticator")?;
            let token = auth.token(scopes).await.context("fetch oauth token")?;
            Ok(token
                .token()
                .ok_or_else(|| anyhow!("authorized-user token missing access token"))?
                .to_string())
        }
        WorkspaceCredential::ServiceAccount(secret) => {
            let auth = yup_oauth2::ServiceAccountAuthenticator::builder(secret)
                .build()
                .await
                .context("build service account authenticator")?;
            let token = auth.token(scopes).await.context("fetch service token")?;
            Ok(token
                .token()
                .ok_or_else(|| anyhow!("service-account token missing access token"))?
                .to_string())
        }
    }
}

async fn load_workspace_credentials() -> Result<WorkspaceCredential> {
    if let Ok(path) = std::env::var(ADL_GWS_CREDENTIALS_FILE_ENV) {
        return load_workspace_credential_file(Path::new(&path)).await;
    }

    if let Ok(path) = std::env::var("GOOGLE_WORKSPACE_CLI_CREDENTIALS_FILE") {
        return load_workspace_credential_file(Path::new(&path)).await;
    }

    if let Some(default_path) =
        dirs::home_dir().map(|dir| dir.join(".config").join("gws").join("credentials.json"))
    {
        if default_path.exists() {
            return load_workspace_credential_file(&default_path).await;
        }
    }

    if let Ok(path) = std::env::var("GOOGLE_APPLICATION_CREDENTIALS") {
        return load_workspace_credential_file(Path::new(&path)).await;
    }

    if let Some(path) = adc_well_known_path() {
        if path.exists() {
            return load_workspace_credential_file(&path).await;
        }
    }

    bail!(
        "no Google Workspace credentials found; set {} or GOOGLE_APPLICATION_CREDENTIALS",
        ADL_GWS_CREDENTIALS_FILE_ENV
    )
}

async fn load_workspace_credential_file(path: &Path) -> Result<WorkspaceCredential> {
    let body = tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("read credential file '{}'", path.display()))?;
    let value: serde_json::Value = serde_json::from_str(&body)
        .with_context(|| format!("parse credential file '{}'", path.display()))?;
    if value.get("type").and_then(|v| v.as_str()) == Some("service_account") {
        let key = yup_oauth2::parse_service_account_key(&body)
            .with_context(|| format!("parse service-account file '{}'", path.display()))?;
        return Ok(WorkspaceCredential::ServiceAccount(key));
    }
    let secret: yup_oauth2::authorized_user::AuthorizedUserSecret = serde_json::from_value(value)
        .with_context(|| {
        format!("parse authorized-user credential file '{}'", path.display())
    })?;
    Ok(WorkspaceCredential::AuthorizedUser(secret))
}

pub fn parse_rfc3339(value: &str) -> Result<DateTime<Utc>> {
    Ok(DateTime::parse_from_rfc3339(value)
        .with_context(|| format!("parse RFC3339 timestamp '{value}'"))?
        .with_timezone(&Utc))
}

pub fn scopes_as_refs(scopes: &[String]) -> Vec<&str> {
    scopes.iter().map(String::as_str).collect()
}

pub fn drive_cache_dir() -> PathBuf {
    tracked_path(".adl/tmp/google_workspace_cms/discovery_cache")
}

pub fn metadata_fields_selector() -> &'static str {
    "id,name,mimeType,parents,modifiedTime,webViewLink"
}

pub fn method_map(doc: &RestDescription) -> HashMap<String, WorkspaceMethodDescriptor> {
    let mut methods = HashMap::new();
    if let Some(files) = doc.resources.get("files") {
        for (name, method) in &files.methods {
            let id = format!("drive.files.{name}");
            if let Ok(descriptor) = workspace_method_descriptor(&id, method) {
                methods.insert(id, descriptor);
            }
        }
    }
    methods
}

#[cfg(test)]
mod tests {
    use super::{
        default_drive_method_catalog, drive_cache_dir, load_workspace_credential_file,
        load_workspace_credentials, metadata_fields_selector, method_map, parse_rfc3339,
        parse_workspace_drive_method_catalog, parse_workspace_execution_mode_from_env,
        parse_workspace_write_approval_from_env, quota_project_hint, scopes_as_refs, tracked_path,
        workspace_method_descriptor, WorkspaceCredential, ADL_GWS_CREDENTIALS_FILE_ENV,
        ADL_GWS_LIVE_MODE_ENV, ADL_GWS_PROJECT_ID_ENV, ADL_GWS_WRITE_APPROVAL_ENV,
    };
    use crate::gws_live_test_support::{lock_gws_live_test_env, EnvVarGuard};
    use google_workspace::discovery::RestDescription;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_path(prefix: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("valid time")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nanos}.json"))
    }

    fn sample_drive_discovery() -> RestDescription {
        serde_json::from_str(
            r#"{
              "name":"drive",
              "version":"v3",
              "rootUrl":"https://www.googleapis.com/",
              "servicePath":"",
              "resources":{
                "files":{
                  "methods":{
                    "list":{"httpMethod":"GET","path":"drive/v3/files","scopes":["https://www.googleapis.com/auth/drive"]},
                    "get":{"httpMethod":"GET","path":"drive/v3/files/{fileId}","scopes":["https://www.googleapis.com/auth/drive.readonly"]},
                    "create":{"httpMethod":"POST","path":"drive/v3/files","scopes":["https://www.googleapis.com/auth/drive"],"supportsMediaUpload":true,"mediaUpload":{"protocols":{"simple":{"path":"/upload/drive/v3/files"}}}},
                    "update":{"httpMethod":"PATCH","path":"drive/v3/files/{fileId}","scopes":["https://www.googleapis.com/auth/drive"],"supportsMediaUpload":true,"mediaUpload":{"protocols":{"simple":{"path":"/upload/drive/v3/files/{fileId}"}}}}
                  }
                }
              }
            }"#,
        )
        .expect("parse sample discovery")
    }

    #[test]
    fn workspace_execution_mode_parses_aliases() {
        let _lock = lock_gws_live_test_env();
        let _mode = EnvVarGuard::set(ADL_GWS_LIVE_MODE_ENV, "enabled");
        assert_eq!(
            parse_workspace_execution_mode_from_env(),
            super::WorkspaceExecutionMode::Execute
        );
        let _mode = EnvVarGuard::set(ADL_GWS_LIVE_MODE_ENV, "dry-run");
        assert_eq!(
            parse_workspace_execution_mode_from_env(),
            super::WorkspaceExecutionMode::DryRun
        );
        let _mode = EnvVarGuard::set(ADL_GWS_LIVE_MODE_ENV, "nope");
        assert_eq!(
            parse_workspace_execution_mode_from_env(),
            super::WorkspaceExecutionMode::FixtureBacked
        );
    }

    #[test]
    fn workspace_write_approval_parses_aliases() {
        let _lock = lock_gws_live_test_env();
        let _approval = EnvVarGuard::set(ADL_GWS_WRITE_APPROVAL_ENV, "approved");
        assert!(parse_workspace_write_approval_from_env());
        let _approval = EnvVarGuard::set(ADL_GWS_WRITE_APPROVAL_ENV, "0");
        assert!(!parse_workspace_write_approval_from_env());
    }

    #[test]
    fn workspace_drive_catalog_parses_required_methods() {
        let doc = sample_drive_discovery();
        let catalog =
            parse_workspace_drive_method_catalog(&doc).expect("parse drive method catalog");
        assert_eq!(catalog.service_name, "drive");
        assert_eq!(
            catalog.create.upload_path.as_deref(),
            Some("/upload/drive/v3/files")
        );
        assert_eq!(
            catalog.update.upload_path.as_deref(),
            Some("/upload/drive/v3/files/{fileId}")
        );
    }

    #[test]
    fn workspace_method_descriptor_keeps_scope_and_upload_shape() {
        let doc = sample_drive_discovery();
        let method = doc
            .resources
            .get("files")
            .expect("files")
            .methods
            .get("create")
            .expect("create");
        let descriptor =
            workspace_method_descriptor("drive.files.create", method).expect("descriptor");
        assert_eq!(descriptor.http_method, "POST");
        assert_eq!(descriptor.scopes.len(), 1);
        assert_eq!(
            descriptor.upload_path.as_deref(),
            Some("/upload/drive/v3/files")
        );
    }

    #[test]
    fn workspace_method_descriptor_uses_default_scope_when_missing() {
        let doc: RestDescription = serde_json::from_str(
            r#"{
              "name":"drive",
              "version":"v3",
              "rootUrl":"https://www.googleapis.com/",
              "servicePath":"",
              "resources":{
                "files":{"methods":{"list":{"httpMethod":"GET","path":"drive/v3/files","scopes":[]}}}
              }
            }"#,
        )
        .expect("parse discovery");
        let method = doc
            .resources
            .get("files")
            .expect("files")
            .methods
            .get("list")
            .expect("list");
        let descriptor =
            workspace_method_descriptor("drive.files.list", method).expect("descriptor");
        assert_eq!(
            descriptor.scopes,
            vec![super::ADL_GWS_DEFAULT_SCOPE.to_string()]
        );
    }

    #[test]
    fn default_drive_catalog_has_expected_method_ids() {
        let catalog = default_drive_method_catalog();
        assert_eq!(catalog.list.method_id, "drive.files.list");
        assert_eq!(catalog.update.method_id, "drive.files.update");
    }

    #[test]
    fn method_map_collects_file_methods() {
        let methods = method_map(&sample_drive_discovery());
        assert!(methods.contains_key("drive.files.list"));
        assert!(methods.contains_key("drive.files.update"));
    }

    #[test]
    fn metadata_selector_matches_expected_fields() {
        assert_eq!(
            metadata_fields_selector(),
            "id,name,mimeType,parents,modifiedTime,webViewLink"
        );
    }

    #[test]
    fn parse_rfc3339_roundtrips() {
        let parsed = parse_rfc3339("2026-06-21T23:25:15Z").expect("parse timestamp");
        assert_eq!(parsed.to_rfc3339(), "2026-06-21T23:25:15+00:00");
    }

    #[test]
    fn scopes_as_refs_preserves_order() {
        let scopes = vec!["one".to_string(), "two".to_string()];
        assert_eq!(scopes_as_refs(&scopes), vec!["one", "two"]);
    }

    #[test]
    fn parse_workspace_drive_catalog_requires_files_resource() {
        let doc: RestDescription = serde_json::from_str(
            r#"{
              "name":"drive",
              "version":"v3",
              "rootUrl":"https://www.googleapis.com/",
              "servicePath":"",
              "resources":{}
            }"#,
        )
        .expect("parse discovery");
        let error = parse_workspace_drive_method_catalog(&doc).expect_err("missing files resource");
        assert!(error.to_string().contains("missing files resource"));
    }

    #[test]
    fn drive_cache_dir_stays_under_repo_tmp() {
        assert_eq!(
            drive_cache_dir(),
            tracked_path(".adl/tmp/google_workspace_cms/discovery_cache")
        );
    }

    #[test]
    fn quota_project_hint_prefers_explicit_env() {
        let _lock = lock_gws_live_test_env();
        let _project = EnvVarGuard::set(ADL_GWS_PROJECT_ID_ENV, "quota-project");
        assert_eq!(quota_project_hint().as_deref(), Some("quota-project"));
    }

    #[test]
    fn quota_project_hint_reads_adc_json_when_env_missing() {
        let _lock = lock_gws_live_test_env();
        let adc_path = unique_temp_path("workspace-adc");
        std::fs::write(
            &adc_path,
            r#"{"quota_project_id":"adc-project","client_id":"id"}"#,
        )
        .expect("write adc file");
        let _adc = EnvVarGuard::set(
            "GOOGLE_APPLICATION_CREDENTIALS",
            adc_path.display().to_string(),
        );
        assert_eq!(quota_project_hint().as_deref(), Some("adc-project"));
        std::fs::remove_file(&adc_path).expect("remove adc file");
    }

    #[tokio::test]
    async fn load_workspace_credential_file_parses_authorized_user_secret() {
        let credential_path = unique_temp_path("workspace-authorized-user");
        tokio::fs::write(
            &credential_path,
            r#"{"client_id":"client","client_secret":"secret","refresh_token":"refresh","type":"authorized_user"}"#,
        )
        .await
        .expect("write credentials");
        let credentials = load_workspace_credential_file(&credential_path)
            .await
            .expect("load authorized user credentials");
        assert!(matches!(
            credentials,
            WorkspaceCredential::AuthorizedUser(_)
        ));
        tokio::fs::remove_file(&credential_path)
            .await
            .expect("remove credentials");
    }

    #[allow(clippy::await_holding_lock)]
    #[tokio::test]
    async fn load_workspace_credentials_prefers_explicit_adl_env_file() {
        let _lock = lock_gws_live_test_env();
        let credential_path = unique_temp_path("workspace-credentials-env");
        tokio::fs::write(
            &credential_path,
            r#"{"client_id":"client","client_secret":"secret","refresh_token":"refresh","type":"authorized_user"}"#,
        )
        .await
        .expect("write credentials");
        let _path = EnvVarGuard::set(
            ADL_GWS_CREDENTIALS_FILE_ENV,
            credential_path.display().to_string(),
        );
        let credentials = load_workspace_credentials()
            .await
            .expect("load credentials from env");
        assert!(matches!(
            credentials,
            WorkspaceCredential::AuthorizedUser(_)
        ));
        tokio::fs::remove_file(&credential_path)
            .await
            .expect("remove credentials");
    }

    #[allow(clippy::await_holding_lock)]
    #[tokio::test]
    async fn load_workspace_credentials_falls_back_to_google_application_credentials() {
        let _lock = lock_gws_live_test_env();
        let credential_path = unique_temp_path("workspace-adc-credentials");
        tokio::fs::write(
            &credential_path,
            r#"{"client_id":"client","client_secret":"secret","refresh_token":"refresh","type":"authorized_user"}"#,
        )
        .await
        .expect("write credentials");
        let _path = EnvVarGuard::set(
            "GOOGLE_APPLICATION_CREDENTIALS",
            credential_path.display().to_string(),
        );
        let credentials = load_workspace_credentials()
            .await
            .expect("load credentials from adc env");
        assert!(matches!(
            credentials,
            WorkspaceCredential::AuthorizedUser(_)
        ));
        tokio::fs::remove_file(&credential_path)
            .await
            .expect("remove credentials");
    }
}
