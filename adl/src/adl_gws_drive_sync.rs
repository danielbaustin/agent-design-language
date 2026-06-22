use crate::adl_gws_native::{
    default_drive_method_catalog, metadata_fields_selector, scopes_as_refs,
    WorkspaceAccessTokenProvider, WorkspaceAuthContext, WorkspaceDriveMethodCatalog,
    WorkspaceExecutionMode, WorkspaceFileRef, WorkspaceScopeBinding, WorkspaceSkipReason,
    ADL_GWS_DEFAULT_SCOPE,
};
use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use chrono::Utc;
use google_workspace::client::send_with_retry;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub const ADL_GWS_NATIVE_DRIVE_SYNC_REPORT_ARTIFACT_PATH: &str =
    ".adl/tmp/google_workspace_cms/adl_gws_native_drive_sync_report.json";
pub const DRIVE_FOLDER_MIME_TYPE: &str = "application/vnd.google-apps.folder";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceDriveFileSyncDisposition {
    Created,
    Updated,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceDriveSyncPolicy {
    CreateOrUpdate,
    CreateOnly,
    UpdateOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceDriveFileSyncRequest {
    pub source_file: String,
    pub target: WorkspaceScopeBinding,
    pub target_file_name: String,
    pub mime_type: String,
    pub policy: WorkspaceDriveSyncPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceDriveFileSyncResult {
    pub source_file: String,
    pub target_folder_id: Option<String>,
    pub target_file_name: String,
    pub disposition: WorkspaceDriveFileSyncDisposition,
    pub file_ref: Option<WorkspaceFileRef>,
    pub skip_reason: Option<WorkspaceSkipReason>,
    pub verification_ok: bool,
    pub verification_message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceDriveSyncTraceRecord {
    pub capability_name: &'static str,
    pub summary: String,
    pub result: WorkspaceDriveFileSyncDisposition,
    pub skip_reason: Option<WorkspaceSkipReason>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceDriveSyncReport {
    pub schema_version: &'static str,
    pub live_mode: WorkspaceExecutionMode,
    pub write_approval_present: bool,
    pub request: WorkspaceDriveFileSyncRequest,
    pub auth_context: Option<WorkspaceAuthContext>,
    pub traces: Vec<WorkspaceDriveSyncTraceRecord>,
    pub result: WorkspaceDriveFileSyncResult,
    pub non_claims: Vec<&'static str>,
}

#[async_trait]
pub trait WorkspaceDriveTransport: Send + Sync {
    async fn list_children(&self, parent_id: &str) -> Result<Vec<WorkspaceFileRef>>;
    async fn read_file_metadata(&self, file_id: &str) -> Result<WorkspaceFileRef>;
    async fn create_folder(&self, parent_id: &str, name: &str) -> Result<WorkspaceFileRef>;
    async fn create_file(
        &self,
        parent_id: &str,
        name: &str,
        mime_type: &str,
        bytes: &[u8],
    ) -> Result<WorkspaceFileRef>;
    async fn update_file(
        &self,
        file_id: &str,
        name: &str,
        mime_type: &str,
        bytes: &[u8],
    ) -> Result<WorkspaceFileRef>;
}

pub struct NativeWorkspaceDriveTransport<P> {
    client: reqwest::Client,
    token_provider: P,
    methods: WorkspaceDriveMethodCatalog,
}

impl<P> NativeWorkspaceDriveTransport<P> {
    pub fn new(token_provider: P) -> Result<Self> {
        Ok(Self {
            client: google_workspace::client::shared_client()
                .map_err(|e| anyhow!(e.to_string()))?,
            token_provider,
            methods: default_drive_method_catalog(),
        })
    }

    async fn authorized_request(
        &self,
        scopes: &[String],
        build: impl Fn(String) -> reqwest::RequestBuilder,
    ) -> Result<(reqwest::Response, WorkspaceAuthContext)>
    where
        P: WorkspaceAccessTokenProvider,
    {
        let scopes = if scopes.is_empty() {
            vec![ADL_GWS_DEFAULT_SCOPE.to_string()]
        } else {
            scopes.to_vec()
        };
        let (token, auth_context) = self
            .token_provider
            .access_token(&scopes_as_refs(&scopes))
            .await?;
        let quota_project = auth_context.quota_project.clone();
        let response = send_with_retry(|| {
            let mut request = build(token.clone()).header("Accept", "application/json");
            if let Some(project) = &quota_project {
                request = request.header("x-goog-user-project", project);
            }
            request
        })
        .await
        .context("send Drive API request")?;
        Ok((response, auth_context))
    }
}

#[derive(Clone, Default)]
pub struct InMemoryDriveTransportForDemo {
    state: Arc<Mutex<InMemoryDriveState>>,
}

#[derive(Default)]
struct InMemoryDriveState {
    files: HashMap<String, WorkspaceFileRef>,
    file_bytes: HashMap<String, Vec<u8>>,
    children: HashMap<String, Vec<String>>,
    next_id: u32,
}

impl InMemoryDriveTransportForDemo {
    pub fn new() -> Self {
        let transport = Self::default();
        {
            let mut state = transport.state.lock().expect("lock drive state");
            state.next_id = 1;
            state.files.insert(
                "demo-root".to_string(),
                WorkspaceFileRef {
                    file_id: "demo-root".to_string(),
                    name: "demo-root".to_string(),
                    mime_type: DRIVE_FOLDER_MIME_TYPE.to_string(),
                    parent_ids: vec![],
                    modified_time: Some("2026-06-21T23:00:00Z".to_string()),
                    web_view_link: None,
                },
            );
        }
        transport
    }

    fn insert_file(&self, parent_id: &str, name: &str, mime_type: &str, bytes: &[u8]) -> String {
        let mut state = self.state.lock().expect("lock drive state");
        let id = format!("demo-id-{}", state.next_id);
        state.next_id += 1;
        state.files.insert(
            id.clone(),
            WorkspaceFileRef {
                file_id: id.clone(),
                name: name.to_string(),
                mime_type: mime_type.to_string(),
                parent_ids: vec![parent_id.to_string()],
                modified_time: Some("2026-06-21T23:00:00Z".to_string()),
                web_view_link: Some(format!("https://example.test/{id}")),
            },
        );
        state.file_bytes.insert(id.clone(), bytes.to_vec());
        state
            .children
            .entry(parent_id.to_string())
            .or_default()
            .push(id.clone());
        id
    }
}

#[async_trait]
impl WorkspaceDriveTransport for InMemoryDriveTransportForDemo {
    async fn list_children(&self, parent_id: &str) -> Result<Vec<WorkspaceFileRef>> {
        let state = self.state.lock().expect("lock drive state");
        let ids = state.children.get(parent_id).cloned().unwrap_or_default();
        Ok(ids
            .iter()
            .filter_map(|id| state.files.get(id).cloned())
            .collect())
    }

    async fn read_file_metadata(&self, file_id: &str) -> Result<WorkspaceFileRef> {
        self.state
            .lock()
            .expect("lock drive state")
            .files
            .get(file_id)
            .cloned()
            .ok_or_else(|| anyhow!("missing file {file_id}"))
    }

    async fn create_folder(&self, parent_id: &str, name: &str) -> Result<WorkspaceFileRef> {
        let id = self.insert_file(parent_id, name, DRIVE_FOLDER_MIME_TYPE, &[]);
        self.read_file_metadata(&id).await
    }

    async fn create_file(
        &self,
        parent_id: &str,
        name: &str,
        mime_type: &str,
        bytes: &[u8],
    ) -> Result<WorkspaceFileRef> {
        let id = self.insert_file(parent_id, name, mime_type, bytes);
        self.read_file_metadata(&id).await
    }

    async fn update_file(
        &self,
        file_id: &str,
        name: &str,
        mime_type: &str,
        bytes: &[u8],
    ) -> Result<WorkspaceFileRef> {
        let mut state = self.state.lock().expect("lock drive state");
        let updated = {
            let file = state
                .files
                .get_mut(file_id)
                .ok_or_else(|| anyhow!("missing file {file_id}"))?;
            file.name = name.to_string();
            file.mime_type = mime_type.to_string();
            file.modified_time = Some("2026-06-21T23:59:59Z".to_string());
            file.clone()
        };
        state.file_bytes.insert(file_id.to_string(), bytes.to_vec());
        Ok(updated)
    }
}

#[async_trait]
impl<P> WorkspaceDriveTransport for NativeWorkspaceDriveTransport<P>
where
    P: WorkspaceAccessTokenProvider,
{
    async fn list_children(&self, parent_id: &str) -> Result<Vec<WorkspaceFileRef>> {
        let query = format!(
            "'{}' in parents and trashed = false",
            escape_drive_query_literal(parent_id)
        );
        let fields = format!("nextPageToken,files({})", metadata_fields_selector());
        let url = "https://www.googleapis.com/drive/v3/files";
        let mut files = Vec::new();
        let mut page_token: Option<String> = None;
        loop {
            let page_token_for_request = page_token.clone();
            let (response, _) = self
                .authorized_request(&self.methods.list.scopes, |token| {
                    let mut request = self.client.get(url).bearer_auth(token).query(&[
                        ("q", query.clone()),
                        ("fields", fields.clone()),
                        ("pageSize", "1000".to_string()),
                    ]);
                    if let Some(token) = &page_token_for_request {
                        request = request.query(&[("pageToken", token.clone())]);
                    }
                    request
                })
                .await?;
            let page = parse_drive_file_list_page(response).await?;
            files.extend(page.files);
            if let Some(next_page_token) = page.next_page_token {
                page_token = Some(next_page_token);
                continue;
            }
            break;
        }
        Ok(files)
    }

    async fn read_file_metadata(&self, file_id: &str) -> Result<WorkspaceFileRef> {
        let url = format!("https://www.googleapis.com/drive/v3/files/{file_id}");
        let (response, _) = self
            .authorized_request(&self.methods.get.scopes, |token| {
                self.client
                    .get(&url)
                    .bearer_auth(token)
                    .query(&[("fields", metadata_fields_selector().to_string())])
            })
            .await?;
        parse_drive_file(response).await
    }

    async fn create_folder(&self, parent_id: &str, name: &str) -> Result<WorkspaceFileRef> {
        let url = "https://www.googleapis.com/drive/v3/files";
        let body = serde_json::json!({
            "name": name,
            "mimeType": DRIVE_FOLDER_MIME_TYPE,
            "parents": [parent_id],
        });
        let (response, _) = self
            .authorized_request(&self.methods.create.scopes, |token| {
                self.client
                    .post(url)
                    .bearer_auth(token)
                    .query(&[("fields", metadata_fields_selector().to_string())])
                    .json(&body)
            })
            .await?;
        parse_drive_file(response).await
    }

    async fn create_file(
        &self,
        parent_id: &str,
        name: &str,
        mime_type: &str,
        bytes: &[u8],
    ) -> Result<WorkspaceFileRef> {
        let url = "https://www.googleapis.com/upload/drive/v3/files";
        let metadata = serde_json::json!({
            "name": name,
            "parents": [parent_id],
            "mimeType": mime_type,
        });
        let (body, content_type) = build_multipart_related(&metadata, bytes, mime_type)?;
        let (response, _) = self
            .authorized_request(&self.methods.create.scopes, |token| {
                self.client
                    .post(url)
                    .bearer_auth(token)
                    .query(&[
                        ("uploadType", "multipart".to_string()),
                        ("fields", metadata_fields_selector().to_string()),
                    ])
                    .header("Content-Type", content_type.clone())
                    .body(body.clone())
            })
            .await?;
        parse_drive_file(response).await
    }

    async fn update_file(
        &self,
        file_id: &str,
        name: &str,
        mime_type: &str,
        bytes: &[u8],
    ) -> Result<WorkspaceFileRef> {
        let url = format!("https://www.googleapis.com/upload/drive/v3/files/{file_id}");
        let metadata = serde_json::json!({
            "name": name,
            "mimeType": mime_type,
        });
        let (body, content_type) = build_multipart_related(&metadata, bytes, mime_type)?;
        let (response, _) = self
            .authorized_request(&self.methods.update.scopes, |token| {
                self.client
                    .patch(&url)
                    .bearer_auth(token)
                    .query(&[
                        ("uploadType", "multipart".to_string()),
                        ("fields", metadata_fields_selector().to_string()),
                    ])
                    .header("Content-Type", content_type.clone())
                    .body(body.clone())
            })
            .await?;
        parse_drive_file(response).await
    }
}

pub async fn ensure_folder_path<T: WorkspaceDriveTransport>(
    transport: &T,
    root_folder_id: &str,
    folder_path: &[String],
) -> Result<(String, Vec<WorkspaceDriveSyncTraceRecord>)> {
    let mut current_parent = root_folder_id.to_string();
    let mut traces = Vec::new();
    for segment in folder_path {
        let children = transport.list_children(&current_parent).await?;
        let matches: Vec<WorkspaceFileRef> = children
            .into_iter()
            .filter(|child| child.name == *segment && child.mime_type == DRIVE_FOLDER_MIME_TYPE)
            .collect();
        if matches.len() > 1 {
            traces.push(skipped_trace(
                "workspace.drive.ensure_folder_path",
                format!(
                    "Found multiple folder candidates named '{}' under parent '{}'.",
                    segment, current_parent
                ),
                WorkspaceSkipReason::AmbiguousTarget,
            ));
            bail!(
                "ambiguous folder path segment '{}' under parent '{}'",
                segment,
                current_parent
            );
        }
        if let Some(folder) = matches.into_iter().next() {
            current_parent = folder.file_id;
            traces.push(proving_trace(
                "workspace.drive.ensure_folder_path",
                format!("Reused existing folder '{}'.", segment),
                WorkspaceDriveFileSyncDisposition::Updated,
            ));
            continue;
        }
        let folder = transport.create_folder(&current_parent, segment).await?;
        current_parent = folder.file_id;
        traces.push(proving_trace(
            "workspace.drive.ensure_folder_path",
            format!("Created bounded folder '{}'.", segment),
            WorkspaceDriveFileSyncDisposition::Created,
        ));
    }
    Ok((current_parent, traces))
}

pub async fn find_file_in_folder<T: WorkspaceDriveTransport>(
    transport: &T,
    parent_id: &str,
    file_name: &str,
) -> Result<Option<WorkspaceFileRef>> {
    let children = transport.list_children(parent_id).await?;
    let matches: Vec<WorkspaceFileRef> = children
        .into_iter()
        .filter(|child| child.name == file_name && child.mime_type != DRIVE_FOLDER_MIME_TYPE)
        .collect();
    if matches.len() > 1 {
        bail!(
            "ambiguous file target '{}' under parent '{}'",
            file_name,
            parent_id
        );
    }
    Ok(matches.into_iter().next())
}

async fn resolve_existing_target<T: WorkspaceDriveTransport>(
    transport: &T,
    request: &WorkspaceDriveFileSyncRequest,
    target_folder_id: &str,
) -> Result<Option<WorkspaceFileRef>> {
    if let Some(file_id) = request.target.file_id.as_deref() {
        let metadata = transport.read_file_metadata(file_id).await?;
        if !metadata.parent_ids.iter().any(|id| id == target_folder_id) {
            bail!(
                "configured file_id '{}' is not within bounded target folder '{}'",
                file_id,
                target_folder_id
            );
        }
        return Ok(Some(metadata));
    }
    find_file_in_folder(transport, target_folder_id, &request.target_file_name).await
}

pub async fn sync_drive_file_with_transport<T: WorkspaceDriveTransport>(
    live_mode: WorkspaceExecutionMode,
    write_approval_present: bool,
    request: WorkspaceDriveFileSyncRequest,
    transport: &T,
) -> Result<WorkspaceDriveSyncReport> {
    let mut traces = Vec::new();
    if matches!(
        live_mode,
        WorkspaceExecutionMode::DryRun | WorkspaceExecutionMode::FixtureBacked
    ) {
        let (skip_reason, verification_message, trace_summary) =
            if matches!(live_mode, WorkspaceExecutionMode::FixtureBacked) {
                (
                WorkspaceSkipReason::DryRunOnly,
                "fixture-backed posture records the bounded Drive sync plan without live mutation"
                    .to_string(),
                "Fixture-backed posture recorded the Drive sync request without executing it.",
            )
            } else {
                (
                    WorkspaceSkipReason::DryRunOnly,
                    "dry-run posture records the bounded Drive sync plan without live mutation"
                        .to_string(),
                    "Dry-run posture recorded the Drive sync request without executing it.",
                )
            };
        let result = WorkspaceDriveFileSyncResult {
            source_file: request.source_file.clone(),
            target_folder_id: Some(request.target.root_folder_id.clone()),
            target_file_name: request.target_file_name.clone(),
            disposition: WorkspaceDriveFileSyncDisposition::Skipped,
            file_ref: None,
            skip_reason: Some(skip_reason.clone()),
            verification_ok: false,
            verification_message,
        };
        traces.push(skipped_trace(
            "workspace.drive.update_file",
            trace_summary,
            skip_reason,
        ));
        return Ok(build_report(
            live_mode,
            write_approval_present,
            request,
            None,
            traces,
            result,
        ));
    }

    if matches!(live_mode, WorkspaceExecutionMode::Execute) && !write_approval_present {
        let result = WorkspaceDriveFileSyncResult {
            source_file: request.source_file.clone(),
            target_folder_id: Some(request.target.root_folder_id.clone()),
            target_file_name: request.target_file_name.clone(),
            disposition: WorkspaceDriveFileSyncDisposition::Skipped,
            file_ref: None,
            skip_reason: Some(WorkspaceSkipReason::MissingBinding),
            verification_ok: false,
            verification_message:
                "execute mode requires ADL_GWS_WRITE_APPROVAL before Drive mutation".to_string(),
        };
        traces.push(skipped_trace(
            "workspace.drive.update_file",
            "Missing write approval stopped the Drive sync before mutation.",
            WorkspaceSkipReason::MissingBinding,
        ));
        return Ok(build_report(
            live_mode,
            write_approval_present,
            request,
            None,
            traces,
            result,
        ));
    }

    let source_bytes = tokio::fs::read(&request.source_file)
        .await
        .with_context(|| format!("read source file '{}'", request.source_file))?;
    let (target_folder_id, mut folder_traces) = ensure_folder_path(
        transport,
        &request.target.root_folder_id,
        &request.target.folder_path,
    )
    .await?;
    traces.append(&mut folder_traces);

    let existing = match resolve_existing_target(transport, &request, &target_folder_id).await {
        Ok(found) => found,
        Err(error) if error.to_string().contains("ambiguous file target") => {
            let result = WorkspaceDriveFileSyncResult {
                source_file: request.source_file.clone(),
                target_folder_id: Some(target_folder_id),
                target_file_name: request.target_file_name.clone(),
                disposition: WorkspaceDriveFileSyncDisposition::Skipped,
                file_ref: None,
                skip_reason: Some(WorkspaceSkipReason::AmbiguousTarget),
                verification_ok: false,
                verification_message: error.to_string(),
            };
            traces.push(skipped_trace(
                "workspace.drive.find_file",
                "Multiple Drive files matched the bounded target name; refusing silent selection.",
                WorkspaceSkipReason::AmbiguousTarget,
            ));
            return Ok(build_report(
                live_mode,
                write_approval_present,
                request,
                None,
                traces,
                result,
            ));
        }
        Err(error)
            if error
                .to_string()
                .contains("is not within bounded target folder") =>
        {
            let verification_message = error.to_string();
            let result = WorkspaceDriveFileSyncResult {
                source_file: request.source_file.clone(),
                target_folder_id: Some(target_folder_id),
                target_file_name: request.target_file_name.clone(),
                disposition: WorkspaceDriveFileSyncDisposition::Skipped,
                file_ref: None,
                skip_reason: Some(WorkspaceSkipReason::MissingBinding),
                verification_ok: false,
                verification_message,
            };
            traces.push(skipped_trace(
                    "workspace.drive.read_file_metadata",
                    "Configured Drive file_id was outside the bounded target folder; refusing silent retargeting.",
                    WorkspaceSkipReason::MissingBinding,
                ));
            return Ok(build_report(
                live_mode,
                write_approval_present,
                request,
                None,
                traces,
                result,
            ));
        }
        Err(error) => return Err(error),
    };

    let auth_context = None;
    let (synced, disposition) = match existing {
        Some(file) => match request.policy {
            WorkspaceDriveSyncPolicy::CreateOnly => {
                let result = WorkspaceDriveFileSyncResult {
                    source_file: request.source_file.clone(),
                    target_folder_id: Some(target_folder_id),
                    target_file_name: request.target_file_name.clone(),
                    disposition: WorkspaceDriveFileSyncDisposition::Skipped,
                    file_ref: Some(file),
                    skip_reason: Some(WorkspaceSkipReason::AmbiguousTarget),
                    verification_ok: false,
                    verification_message: "target file already exists but policy is create-only"
                        .to_string(),
                };
                traces.push(skipped_trace(
                    "workspace.drive.create_file",
                    "Create-only policy refused to overwrite an existing Drive file.",
                    WorkspaceSkipReason::AmbiguousTarget,
                ));
                return Ok(build_report(
                    live_mode,
                    write_approval_present,
                    request,
                    auth_context,
                    traces,
                    result,
                ));
            }
            _ => {
                let updated = transport
                    .update_file(
                        &file.file_id,
                        &request.target_file_name,
                        &request.mime_type,
                        &source_bytes,
                    )
                    .await?;
                traces.push(proving_trace(
                    "workspace.drive.update_file",
                    format!(
                        "Updated existing Drive file '{}'.",
                        request.target_file_name
                    ),
                    WorkspaceDriveFileSyncDisposition::Updated,
                ));
                (updated, WorkspaceDriveFileSyncDisposition::Updated)
            }
        },
        None => match request.policy {
            WorkspaceDriveSyncPolicy::UpdateOnly => {
                let result = WorkspaceDriveFileSyncResult {
                    source_file: request.source_file.clone(),
                    target_folder_id: Some(target_folder_id),
                    target_file_name: request.target_file_name.clone(),
                    disposition: WorkspaceDriveFileSyncDisposition::Skipped,
                    file_ref: None,
                    skip_reason: Some(WorkspaceSkipReason::MissingBinding),
                    verification_ok: false,
                    verification_message: "target file is missing but policy is update-only"
                        .to_string(),
                };
                traces.push(skipped_trace(
                    "workspace.drive.update_file",
                    "Update-only policy refused to create a missing Drive file.",
                    WorkspaceSkipReason::MissingBinding,
                ));
                return Ok(build_report(
                    live_mode,
                    write_approval_present,
                    request,
                    auth_context,
                    traces,
                    result,
                ));
            }
            _ => {
                let created = transport
                    .create_file(
                        &target_folder_id,
                        &request.target_file_name,
                        &request.mime_type,
                        &source_bytes,
                    )
                    .await?;
                traces.push(proving_trace(
                    "workspace.drive.create_file",
                    format!("Created bounded Drive file '{}'.", request.target_file_name),
                    WorkspaceDriveFileSyncDisposition::Created,
                ));
                (created, WorkspaceDriveFileSyncDisposition::Created)
            }
        },
    };

    let verified = transport.read_file_metadata(&synced.file_id).await?;
    let verification_ok = verified.name == request.target_file_name
        && verified.parent_ids.iter().any(|id| id == &target_folder_id);
    let verification_message = if verification_ok {
        "Drive metadata verification passed after sync.".to_string()
    } else {
        "Drive metadata verification failed after sync.".to_string()
    };
    if !verification_ok {
        traces.push(skipped_trace(
            "workspace.drive.verify_file_state",
            "Post-write metadata verification did not match the bounded target.",
            WorkspaceSkipReason::VerificationMismatch,
        ));
    } else {
        traces.push(proving_trace(
            "workspace.drive.verify_file_state",
            "Verified post-write Drive metadata for the bounded target file.",
            disposition.clone(),
        ));
    }

    let result = WorkspaceDriveFileSyncResult {
        source_file: request.source_file.clone(),
        target_folder_id: Some(target_folder_id),
        target_file_name: request.target_file_name.clone(),
        disposition,
        file_ref: Some(verified),
        skip_reason: if verification_ok {
            None
        } else {
            Some(WorkspaceSkipReason::VerificationMismatch)
        },
        verification_ok,
        verification_message,
    };
    Ok(build_report(
        live_mode,
        write_approval_present,
        request,
        auth_context,
        traces,
        result,
    ))
}

fn build_report(
    live_mode: WorkspaceExecutionMode,
    write_approval_present: bool,
    request: WorkspaceDriveFileSyncRequest,
    auth_context: Option<WorkspaceAuthContext>,
    traces: Vec<WorkspaceDriveSyncTraceRecord>,
    result: WorkspaceDriveFileSyncResult,
) -> WorkspaceDriveSyncReport {
    WorkspaceDriveSyncReport {
        schema_version: "adl_gws_native_drive_sync.v1",
        live_mode,
        write_approval_present,
        request,
        auth_context,
        traces,
        result,
        non_claims: vec![
            "This native Drive sync surface does not make Google Workspace canonical repo truth.",
            "This native Drive sync surface does not authorize direct tracked repo mutation from Workspace state.",
            "Fixture-backed validation remains the default proving mode for ordinary development and review.",
        ],
    }
}

fn proving_trace(
    capability_name: &'static str,
    summary: impl Into<String>,
    result: WorkspaceDriveFileSyncDisposition,
) -> WorkspaceDriveSyncTraceRecord {
    WorkspaceDriveSyncTraceRecord {
        capability_name,
        summary: summary.into(),
        result,
        skip_reason: None,
    }
}

fn skipped_trace(
    capability_name: &'static str,
    summary: impl Into<String>,
    skip_reason: WorkspaceSkipReason,
) -> WorkspaceDriveSyncTraceRecord {
    WorkspaceDriveSyncTraceRecord {
        capability_name,
        summary: summary.into(),
        result: WorkspaceDriveFileSyncDisposition::Skipped,
        skip_reason: Some(skip_reason),
    }
}

fn escape_drive_query_literal(value: &str) -> String {
    value.replace('\\', "\\\\").replace('\'', "\\'")
}

fn build_multipart_related(
    metadata: &Value,
    bytes: &[u8],
    mime_type: &str,
) -> Result<(Vec<u8>, String)> {
    let boundary = format!(
        "adl-gws-boundary-{}",
        Utc::now().timestamp_nanos_opt().unwrap_or(0)
    );
    let mut body = Vec::new();
    body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    body.extend_from_slice(b"Content-Type: application/json; charset=UTF-8\r\n\r\n");
    body.extend_from_slice(serde_json::to_string(metadata)?.as_bytes());
    body.extend_from_slice(b"\r\n");
    body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    body.extend_from_slice(format!("Content-Type: {mime_type}\r\n\r\n").as_bytes());
    body.extend_from_slice(bytes);
    body.extend_from_slice(b"\r\n");
    body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    Ok((body, format!("multipart/related; boundary={boundary}")))
}

struct DriveFileListPage {
    files: Vec<WorkspaceFileRef>,
    next_page_token: Option<String>,
}

async fn parse_drive_file_list_page(response: reqwest::Response) -> Result<DriveFileListPage> {
    let response = response
        .error_for_status()
        .context("Drive list request failed")?;
    let value: Value = response.json().await.context("parse Drive list response")?;
    parse_drive_file_list_page_value(&value)
}

fn parse_drive_file_list_page_value(value: &Value) -> Result<DriveFileListPage> {
    let files = value
        .get("files")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("Drive list response missing files array"))?;
    let files = files
        .iter()
        .map(value_to_file_ref)
        .collect::<Result<Vec<_>>>()?;
    let next_page_token = value
        .get("nextPageToken")
        .and_then(Value::as_str)
        .map(ToString::to_string);
    Ok(DriveFileListPage {
        files,
        next_page_token,
    })
}

async fn parse_drive_file(response: reqwest::Response) -> Result<WorkspaceFileRef> {
    let response = response
        .error_for_status()
        .context("Drive metadata request failed")?;
    let value: Value = response.json().await.context("parse Drive file response")?;
    value_to_file_ref(&value)
}

fn value_to_file_ref(value: &Value) -> Result<WorkspaceFileRef> {
    let parent_ids = value
        .get("parents")
        .and_then(Value::as_array)
        .map(|parents| {
            parents
                .iter()
                .filter_map(Value::as_str)
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(WorkspaceFileRef {
        file_id: value
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("Drive file response missing id"))?
            .to_string(),
        name: value
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("Drive file response missing name"))?
            .to_string(),
        mime_type: value
            .get("mimeType")
            .and_then(Value::as_str)
            .unwrap_or("application/octet-stream")
            .to_string(),
        parent_ids,
        modified_time: value
            .get("modifiedTime")
            .and_then(Value::as_str)
            .map(ToString::to_string),
        web_view_link: value
            .get("webViewLink")
            .and_then(Value::as_str)
            .map(ToString::to_string),
    })
}

pub async fn write_workspace_drive_sync_report(
    report_path: impl AsRef<Path>,
    report: &WorkspaceDriveSyncReport,
) -> Result<()> {
    let report_path = report_path.as_ref();
    if let Some(parent) = report_path.parent() {
        tokio::fs::create_dir_all(parent).await.with_context(|| {
            format!("create parent directories for '{}'", report_path.display())
        })?;
    }
    tokio::fs::write(report_path, serde_json::to_string_pretty(report)?)
        .await
        .with_context(|| format!("write '{}'", report_path.display()))
}

#[cfg(test)]
mod tests {
    use super::{
        build_multipart_related, ensure_folder_path, find_file_in_folder,
        parse_drive_file_list_page_value, sync_drive_file_with_transport,
        write_workspace_drive_sync_report, WorkspaceDriveFileSyncDisposition,
        WorkspaceDriveFileSyncRequest, WorkspaceDriveSyncPolicy, WorkspaceDriveTransport,
        DRIVE_FOLDER_MIME_TYPE,
    };
    use crate::adl_gws_native::{
        WorkspaceExecutionMode, WorkspaceFileRef, WorkspaceScopeBinding, WorkspaceSkipReason,
    };
    use anyhow::{anyhow, Result};
    use async_trait::async_trait;
    use serde_json::json;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Clone, Default)]
    struct InMemoryDriveTransport {
        state: Arc<Mutex<DriveState>>,
    }

    #[derive(Default)]
    struct DriveState {
        files: HashMap<String, WorkspaceFileRef>,
        file_bytes: HashMap<String, Vec<u8>>,
        children: HashMap<String, Vec<String>>,
        next_id: u32,
    }

    impl InMemoryDriveTransport {
        fn new() -> Self {
            let transport = Self::default();
            {
                let mut state = transport.state.lock().expect("lock drive state");
                state.next_id = 1;
                state.files.insert(
                    "root".to_string(),
                    WorkspaceFileRef {
                        file_id: "root".to_string(),
                        name: "root".to_string(),
                        mime_type: DRIVE_FOLDER_MIME_TYPE.to_string(),
                        parent_ids: vec![],
                        modified_time: Some("2026-06-21T23:00:00Z".to_string()),
                        web_view_link: None,
                    },
                );
            }
            transport
        }

        fn insert_file(
            &self,
            parent_id: &str,
            name: &str,
            mime_type: &str,
            bytes: &[u8],
        ) -> String {
            let mut state = self.state.lock().expect("lock drive state");
            let id = format!("id-{}", state.next_id);
            state.next_id += 1;
            state.files.insert(
                id.clone(),
                WorkspaceFileRef {
                    file_id: id.clone(),
                    name: name.to_string(),
                    mime_type: mime_type.to_string(),
                    parent_ids: vec![parent_id.to_string()],
                    modified_time: Some("2026-06-21T23:00:00Z".to_string()),
                    web_view_link: Some(format!("https://example.test/{id}")),
                },
            );
            state.file_bytes.insert(id.clone(), bytes.to_vec());
            state
                .children
                .entry(parent_id.to_string())
                .or_default()
                .push(id.clone());
            id
        }
    }

    #[async_trait]
    impl WorkspaceDriveTransport for InMemoryDriveTransport {
        async fn list_children(&self, parent_id: &str) -> Result<Vec<WorkspaceFileRef>> {
            let state = self.state.lock().expect("lock drive state");
            let ids = state.children.get(parent_id).cloned().unwrap_or_default();
            Ok(ids
                .iter()
                .filter_map(|id| state.files.get(id).cloned())
                .collect())
        }

        async fn read_file_metadata(&self, file_id: &str) -> Result<WorkspaceFileRef> {
            self.state
                .lock()
                .expect("lock drive state")
                .files
                .get(file_id)
                .cloned()
                .ok_or_else(|| anyhow!("missing file {file_id}"))
        }

        async fn create_folder(&self, parent_id: &str, name: &str) -> Result<WorkspaceFileRef> {
            let id = self.insert_file(parent_id, name, DRIVE_FOLDER_MIME_TYPE, &[]);
            self.read_file_metadata(&id).await
        }

        async fn create_file(
            &self,
            parent_id: &str,
            name: &str,
            mime_type: &str,
            bytes: &[u8],
        ) -> Result<WorkspaceFileRef> {
            let id = self.insert_file(parent_id, name, mime_type, bytes);
            self.read_file_metadata(&id).await
        }

        async fn update_file(
            &self,
            file_id: &str,
            name: &str,
            mime_type: &str,
            bytes: &[u8],
        ) -> Result<WorkspaceFileRef> {
            let mut state = self.state.lock().expect("lock drive state");
            let updated = {
                let file = state
                    .files
                    .get_mut(file_id)
                    .ok_or_else(|| anyhow!("missing file {file_id}"))?;
                file.name = name.to_string();
                file.mime_type = mime_type.to_string();
                file.modified_time = Some("2026-06-21T23:59:59Z".to_string());
                file.clone()
            };
            state.file_bytes.insert(file_id.to_string(), bytes.to_vec());
            Ok(updated)
        }
    }

    fn unique_temp_path(prefix: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("valid time")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nanos}.json"))
    }

    #[tokio::test]
    async fn ensure_folder_path_creates_missing_segments_in_order() {
        let transport = InMemoryDriveTransport::new();
        let (folder_id, traces) = ensure_folder_path(
            &transport,
            "root",
            &["docs".to_string(), "seed".to_string()],
        )
        .await
        .expect("ensure folder path");
        assert!(!folder_id.is_empty());
        assert_eq!(traces.len(), 2);
        assert_eq!(traces[0].result, WorkspaceDriveFileSyncDisposition::Created);
        assert_eq!(traces[1].result, WorkspaceDriveFileSyncDisposition::Created);
    }

    #[tokio::test]
    async fn find_file_in_folder_returns_existing_match() {
        let transport = InMemoryDriveTransport::new();
        let folder = transport
            .create_folder("root", "docs")
            .await
            .expect("create folder");
        transport.insert_file(&folder.file_id, "state.md", "text/markdown", b"hello");
        let found = find_file_in_folder(&transport, &folder.file_id, "state.md")
            .await
            .expect("find file");
        assert!(found.is_some());
        assert_eq!(found.expect("file").name, "state.md");
    }

    #[tokio::test]
    async fn drive_sync_file_id_binding_overrides_ambiguous_name_lookup() {
        let transport = InMemoryDriveTransport::new();
        let folder = transport
            .create_folder("root", "docs")
            .await
            .expect("create folder");
        let target_id = transport.insert_file(&folder.file_id, "state.md", "text/markdown", b"old");
        transport.insert_file(&folder.file_id, "state.md", "text/markdown", b"other");
        let source = unique_temp_path("workspace-drive-source-file-id");
        tokio::fs::write(&source, b"new body")
            .await
            .expect("write source");

        let report = sync_drive_file_with_transport(
            WorkspaceExecutionMode::Execute,
            true,
            WorkspaceDriveFileSyncRequest {
                source_file: source.display().to_string(),
                target: WorkspaceScopeBinding {
                    root_folder_id: "root".to_string(),
                    folder_path: vec!["docs".to_string()],
                    file_name: Some("state.md".to_string()),
                    file_id: Some(target_id.clone()),
                },
                target_file_name: "state.md".to_string(),
                mime_type: "text/markdown".to_string(),
                policy: WorkspaceDriveSyncPolicy::CreateOrUpdate,
            },
            &transport,
        )
        .await
        .expect("sync by file id");

        assert_eq!(
            report
                .result
                .file_ref
                .as_ref()
                .map(|file| file.file_id.as_str()),
            Some(target_id.as_str())
        );
        assert_eq!(
            report.result.disposition,
            WorkspaceDriveFileSyncDisposition::Updated
        );
        tokio::fs::remove_file(&source)
            .await
            .expect("remove source");
    }

    #[tokio::test]
    async fn drive_sync_creates_missing_target_file() {
        let transport = InMemoryDriveTransport::new();
        let source = unique_temp_path("workspace-drive-source-create");
        tokio::fs::write(&source, b"seed body")
            .await
            .expect("write source");

        let request = WorkspaceDriveFileSyncRequest {
            source_file: source.display().to_string(),
            target: WorkspaceScopeBinding {
                root_folder_id: "root".to_string(),
                folder_path: vec!["docs".to_string()],
                file_name: Some("state.md".to_string()),
                file_id: None,
            },
            target_file_name: "state.md".to_string(),
            mime_type: "text/markdown".to_string(),
            policy: WorkspaceDriveSyncPolicy::CreateOrUpdate,
        };

        let report = sync_drive_file_with_transport(
            WorkspaceExecutionMode::Execute,
            true,
            request,
            &transport,
        )
        .await
        .expect("sync drive file");

        assert_eq!(
            report.result.disposition,
            WorkspaceDriveFileSyncDisposition::Created
        );
        assert!(report.result.verification_ok);
        tokio::fs::remove_file(&source)
            .await
            .expect("remove source");
    }

    #[tokio::test]
    async fn drive_sync_updates_existing_target_file() {
        let transport = InMemoryDriveTransport::new();
        let (folder_id, _) = ensure_folder_path(&transport, "root", &["docs".to_string()])
            .await
            .expect("ensure folder");
        transport.insert_file(&folder_id, "state.md", "text/markdown", b"old");

        let source = unique_temp_path("workspace-drive-source-update");
        tokio::fs::write(&source, b"new body")
            .await
            .expect("write source");

        let request = WorkspaceDriveFileSyncRequest {
            source_file: source.display().to_string(),
            target: WorkspaceScopeBinding {
                root_folder_id: "root".to_string(),
                folder_path: vec!["docs".to_string()],
                file_name: Some("state.md".to_string()),
                file_id: None,
            },
            target_file_name: "state.md".to_string(),
            mime_type: "text/markdown".to_string(),
            policy: WorkspaceDriveSyncPolicy::CreateOrUpdate,
        };

        let report = sync_drive_file_with_transport(
            WorkspaceExecutionMode::Execute,
            true,
            request,
            &transport,
        )
        .await
        .expect("sync drive file");

        assert_eq!(
            report.result.disposition,
            WorkspaceDriveFileSyncDisposition::Updated
        );
        assert!(report.result.verification_ok);
        tokio::fs::remove_file(&source)
            .await
            .expect("remove source");
    }

    #[tokio::test]
    async fn drive_sync_dry_run_records_skip_without_mutation() {
        let transport = InMemoryDriveTransport::new();
        let source = unique_temp_path("workspace-drive-source-dry-run");
        tokio::fs::write(&source, b"dry run")
            .await
            .expect("write source");

        let report = sync_drive_file_with_transport(
            WorkspaceExecutionMode::DryRun,
            false,
            WorkspaceDriveFileSyncRequest {
                source_file: source.display().to_string(),
                target: WorkspaceScopeBinding {
                    root_folder_id: "root".to_string(),
                    folder_path: vec![],
                    file_name: Some("state.md".to_string()),
                    file_id: None,
                },
                target_file_name: "state.md".to_string(),
                mime_type: "text/markdown".to_string(),
                policy: WorkspaceDriveSyncPolicy::CreateOrUpdate,
            },
            &transport,
        )
        .await
        .expect("dry run report");

        assert_eq!(
            report.result.skip_reason,
            Some(WorkspaceSkipReason::DryRunOnly)
        );
        tokio::fs::remove_file(&source)
            .await
            .expect("remove source");
    }

    #[tokio::test]
    async fn drive_sync_fixture_backed_records_skip_without_mutation() {
        let transport = InMemoryDriveTransport::new();
        let source = unique_temp_path("workspace-drive-source-fixture");
        tokio::fs::write(&source, b"fixture")
            .await
            .expect("write source");

        let report = sync_drive_file_with_transport(
            WorkspaceExecutionMode::FixtureBacked,
            false,
            WorkspaceDriveFileSyncRequest {
                source_file: source.display().to_string(),
                target: WorkspaceScopeBinding {
                    root_folder_id: "root".to_string(),
                    folder_path: vec![],
                    file_name: Some("state.md".to_string()),
                    file_id: None,
                },
                target_file_name: "state.md".to_string(),
                mime_type: "text/markdown".to_string(),
                policy: WorkspaceDriveSyncPolicy::CreateOrUpdate,
            },
            &transport,
        )
        .await
        .expect("fixture-backed report");

        assert_eq!(
            report.result.skip_reason,
            Some(WorkspaceSkipReason::DryRunOnly)
        );
        assert_eq!(
            transport
                .list_children("root")
                .await
                .expect("list children")
                .len(),
            0
        );
        tokio::fs::remove_file(&source)
            .await
            .expect("remove source");
    }

    #[tokio::test]
    async fn drive_sync_stops_without_write_approval() {
        let transport = InMemoryDriveTransport::new();
        let source = unique_temp_path("workspace-drive-source-no-approval");
        tokio::fs::write(&source, b"execute")
            .await
            .expect("write source");
        let report = sync_drive_file_with_transport(
            WorkspaceExecutionMode::Execute,
            false,
            WorkspaceDriveFileSyncRequest {
                source_file: source.display().to_string(),
                target: WorkspaceScopeBinding {
                    root_folder_id: "root".to_string(),
                    folder_path: vec![],
                    file_name: Some("state.md".to_string()),
                    file_id: None,
                },
                target_file_name: "state.md".to_string(),
                mime_type: "text/markdown".to_string(),
                policy: WorkspaceDriveSyncPolicy::CreateOrUpdate,
            },
            &transport,
        )
        .await
        .expect("execute no approval report");
        assert_eq!(
            report.result.skip_reason,
            Some(WorkspaceSkipReason::MissingBinding)
        );
        tokio::fs::remove_file(&source)
            .await
            .expect("remove source");
    }

    #[tokio::test]
    async fn drive_sync_refuses_duplicate_target_names() {
        let transport = InMemoryDriveTransport::new();
        let (folder_id, _) = ensure_folder_path(&transport, "root", &["docs".to_string()])
            .await
            .expect("ensure folder");
        transport.insert_file(&folder_id, "state.md", "text/markdown", b"one");
        transport.insert_file(&folder_id, "state.md", "text/markdown", b"two");
        let source = unique_temp_path("workspace-drive-source-duplicate");
        tokio::fs::write(&source, b"execute")
            .await
            .expect("write source");
        let report = sync_drive_file_with_transport(
            WorkspaceExecutionMode::Execute,
            true,
            WorkspaceDriveFileSyncRequest {
                source_file: source.display().to_string(),
                target: WorkspaceScopeBinding {
                    root_folder_id: "root".to_string(),
                    folder_path: vec!["docs".to_string()],
                    file_name: Some("state.md".to_string()),
                    file_id: None,
                },
                target_file_name: "state.md".to_string(),
                mime_type: "text/markdown".to_string(),
                policy: WorkspaceDriveSyncPolicy::CreateOrUpdate,
            },
            &transport,
        )
        .await
        .expect("duplicate report");
        assert_eq!(
            report.result.skip_reason,
            Some(WorkspaceSkipReason::AmbiguousTarget)
        );
        tokio::fs::remove_file(&source)
            .await
            .expect("remove source");
    }

    #[tokio::test]
    async fn drive_sync_create_only_skips_when_target_exists() {
        let transport = InMemoryDriveTransport::new();
        let (folder_id, _) = ensure_folder_path(&transport, "root", &["docs".to_string()])
            .await
            .expect("ensure folder");
        transport.insert_file(&folder_id, "state.md", "text/markdown", b"old");
        let source = unique_temp_path("workspace-drive-source-create-only");
        tokio::fs::write(&source, b"new body")
            .await
            .expect("write source");
        let report = sync_drive_file_with_transport(
            WorkspaceExecutionMode::Execute,
            true,
            WorkspaceDriveFileSyncRequest {
                source_file: source.display().to_string(),
                target: WorkspaceScopeBinding {
                    root_folder_id: "root".to_string(),
                    folder_path: vec!["docs".to_string()],
                    file_name: Some("state.md".to_string()),
                    file_id: None,
                },
                target_file_name: "state.md".to_string(),
                mime_type: "text/markdown".to_string(),
                policy: WorkspaceDriveSyncPolicy::CreateOnly,
            },
            &transport,
        )
        .await
        .expect("create only report");
        assert_eq!(
            report.result.disposition,
            WorkspaceDriveFileSyncDisposition::Skipped
        );
        assert_eq!(
            report.result.skip_reason,
            Some(WorkspaceSkipReason::AmbiguousTarget)
        );
        tokio::fs::remove_file(&source)
            .await
            .expect("remove source");
    }

    #[tokio::test]
    async fn drive_sync_update_only_skips_when_target_missing() {
        let transport = InMemoryDriveTransport::new();
        let source = unique_temp_path("workspace-drive-source-update-only");
        tokio::fs::write(&source, b"new body")
            .await
            .expect("write source");
        let report = sync_drive_file_with_transport(
            WorkspaceExecutionMode::Execute,
            true,
            WorkspaceDriveFileSyncRequest {
                source_file: source.display().to_string(),
                target: WorkspaceScopeBinding {
                    root_folder_id: "root".to_string(),
                    folder_path: vec!["docs".to_string()],
                    file_name: Some("state.md".to_string()),
                    file_id: None,
                },
                target_file_name: "state.md".to_string(),
                mime_type: "text/markdown".to_string(),
                policy: WorkspaceDriveSyncPolicy::UpdateOnly,
            },
            &transport,
        )
        .await
        .expect("update only report");
        assert_eq!(
            report.result.disposition,
            WorkspaceDriveFileSyncDisposition::Skipped
        );
        assert_eq!(
            report.result.skip_reason,
            Some(WorkspaceSkipReason::MissingBinding)
        );
        tokio::fs::remove_file(&source)
            .await
            .expect("remove source");
    }

    #[tokio::test]
    async fn drive_sync_file_id_outside_bounded_folder_skips() {
        let transport = InMemoryDriveTransport::new();
        let sibling = transport
            .create_folder("root", "other")
            .await
            .expect("create sibling folder");
        let wrong_id = transport.insert_file(&sibling.file_id, "state.md", "text/markdown", b"old");
        let source = unique_temp_path("workspace-drive-source-file-id-mismatch");
        tokio::fs::write(&source, b"new body")
            .await
            .expect("write source");

        let report = sync_drive_file_with_transport(
            WorkspaceExecutionMode::Execute,
            true,
            WorkspaceDriveFileSyncRequest {
                source_file: source.display().to_string(),
                target: WorkspaceScopeBinding {
                    root_folder_id: "root".to_string(),
                    folder_path: vec!["docs".to_string()],
                    file_name: Some("state.md".to_string()),
                    file_id: Some(wrong_id),
                },
                target_file_name: "state.md".to_string(),
                mime_type: "text/markdown".to_string(),
                policy: WorkspaceDriveSyncPolicy::CreateOrUpdate,
            },
            &transport,
        )
        .await
        .expect("mismatched file id report");

        assert_eq!(
            report.result.disposition,
            WorkspaceDriveFileSyncDisposition::Skipped
        );
        assert_eq!(
            report.result.skip_reason,
            Some(WorkspaceSkipReason::MissingBinding)
        );
        assert!(report
            .result
            .verification_message
            .contains("not within bounded target folder"));
        tokio::fs::remove_file(&source)
            .await
            .expect("remove source");
    }

    #[tokio::test]
    async fn drive_sync_reports_verification_mismatch_when_metadata_differs() {
        #[derive(Clone, Default)]
        struct VerificationMismatchTransport {
            inner: InMemoryDriveTransport,
        }

        #[async_trait]
        impl WorkspaceDriveTransport for VerificationMismatchTransport {
            async fn list_children(&self, parent_id: &str) -> Result<Vec<WorkspaceFileRef>> {
                self.inner.list_children(parent_id).await
            }

            async fn read_file_metadata(&self, file_id: &str) -> Result<WorkspaceFileRef> {
                let mut file = self.inner.read_file_metadata(file_id).await?;
                if file_id != "root" {
                    file.parent_ids = vec!["wrong-parent".to_string()];
                }
                Ok(file)
            }

            async fn create_folder(&self, parent_id: &str, name: &str) -> Result<WorkspaceFileRef> {
                self.inner.create_folder(parent_id, name).await
            }

            async fn create_file(
                &self,
                parent_id: &str,
                name: &str,
                mime_type: &str,
                bytes: &[u8],
            ) -> Result<WorkspaceFileRef> {
                self.inner
                    .create_file(parent_id, name, mime_type, bytes)
                    .await
            }

            async fn update_file(
                &self,
                file_id: &str,
                name: &str,
                mime_type: &str,
                bytes: &[u8],
            ) -> Result<WorkspaceFileRef> {
                self.inner
                    .update_file(file_id, name, mime_type, bytes)
                    .await
            }
        }

        let transport = VerificationMismatchTransport::default();
        let source = unique_temp_path("workspace-drive-source-verification");
        tokio::fs::write(&source, b"seed body")
            .await
            .expect("write source");
        let report = sync_drive_file_with_transport(
            WorkspaceExecutionMode::Execute,
            true,
            WorkspaceDriveFileSyncRequest {
                source_file: source.display().to_string(),
                target: WorkspaceScopeBinding {
                    root_folder_id: "root".to_string(),
                    folder_path: vec!["docs".to_string()],
                    file_name: Some("state.md".to_string()),
                    file_id: None,
                },
                target_file_name: "state.md".to_string(),
                mime_type: "text/markdown".to_string(),
                policy: WorkspaceDriveSyncPolicy::CreateOrUpdate,
            },
            &transport,
        )
        .await
        .expect("verification mismatch report");
        assert!(!report.result.verification_ok);
        assert_eq!(
            report.result.skip_reason,
            Some(WorkspaceSkipReason::VerificationMismatch)
        );
        tokio::fs::remove_file(&source)
            .await
            .expect("remove source");
    }

    #[tokio::test]
    async fn write_drive_sync_report_serializes_json() {
        let report_path = unique_temp_path("workspace-drive-report");
        let report = sync_drive_file_with_transport(
            WorkspaceExecutionMode::DryRun,
            false,
            WorkspaceDriveFileSyncRequest {
                source_file: "/tmp/source.md".to_string(),
                target: WorkspaceScopeBinding {
                    root_folder_id: "root".to_string(),
                    folder_path: vec![],
                    file_name: Some("state.md".to_string()),
                    file_id: None,
                },
                target_file_name: "state.md".to_string(),
                mime_type: "text/markdown".to_string(),
                policy: WorkspaceDriveSyncPolicy::CreateOrUpdate,
            },
            &InMemoryDriveTransport::new(),
        )
        .await
        .expect("report");
        write_workspace_drive_sync_report(&report_path, &report)
            .await
            .expect("write report");
        let body = tokio::fs::read_to_string(&report_path)
            .await
            .expect("read report");
        assert!(body.contains("adl_gws_native_drive_sync.v1"));
        tokio::fs::remove_file(&report_path)
            .await
            .expect("remove report");
    }

    #[test]
    fn multipart_builder_contains_metadata_and_payload() {
        let (body, content_type) =
            build_multipart_related(&json!({"name":"state.md"}), b"hello", "text/markdown")
                .expect("multipart");
        let body_str = String::from_utf8_lossy(&body);
        assert!(content_type.starts_with("multipart/related; boundary="));
        assert!(body_str.contains("\"state.md\""));
        assert!(body_str.contains("text/markdown"));
        assert!(body_str.contains("hello"));
    }

    #[test]
    fn parse_drive_file_list_reads_expected_shape() {
        let page = parse_drive_file_list_page_value(
            &json!({"nextPageToken":"token-2","files":[{"id":"1","name":"state.md","mimeType":"text/markdown","parents":["root"],"modifiedTime":"2026-06-21T23:00:00Z"}]}),
        )
        .expect("files");
        assert_eq!(page.files.len(), 1);
        assert_eq!(page.files[0].name, "state.md");
        assert_eq!(page.next_page_token.as_deref(), Some("token-2"));
    }

    #[test]
    fn parse_drive_file_list_rejects_missing_files_array() {
        let error = match parse_drive_file_list_page_value(&json!({"nextPageToken":"token-2"})) {
            Ok(_) => panic!("missing files array should fail"),
            Err(error) => error,
        };
        assert!(error.to_string().contains("missing files array"));
    }

    #[test]
    fn escape_drive_query_literal_escapes_special_characters() {
        assert_eq!(
            super::escape_drive_query_literal(r"drive\root'one"),
            r"drive\\root\'one"
        );
    }
}
