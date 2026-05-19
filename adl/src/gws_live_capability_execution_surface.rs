use crate::rust_native_gws_adapter_boundary::{WorkspaceAuthority, WorkspaceCmsRootRef};
use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use serde_json::Value;
use std::{env, fs, path::Path, process::Command};

pub const GWS_LIVE_CAPABILITY_EXECUTION_REPORT_ARTIFACT_PATH: &str =
    "docs/milestones/v0.91.2/review/google_workspace_cms_bridge/gws_live_capability_execution_report.json";
pub const GWS_LIVE_CAPABILITY_EXECUTION_SNAPSHOT_ARTIFACT_PATH: &str =
    "docs/milestones/v0.91.2/review/google_workspace_cms_bridge/gws_live_capability_execution_snapshot.json";
pub const GWS_LIVE_CAPABILITY_EXECUTION_SCHEMA_VERSION: &str =
    "gws_live_capability_execution_surface.v1";
pub const WORKSPACE_CMS_LIVE_SNAPSHOT_SCHEMA_VERSION: &str = "workspace_cms_live_snapshot.v1";
pub const GWS_LIVE_CAPABILITY_EXECUTION_PROMPT_VERSION: &str =
    "wp3091.gws_live_capability_execution_surface.v1";
pub const GWS_LIVE_ENABLE_ENV: &str = "ADL_GWS_LIVE_MODE";
pub const GWS_DRIVE_FOLDER_ID_ENV: &str = "ADL_GWS_DRIVE_FOLDER_ID";
pub const GWS_DOC_ID_ENV: &str = "ADL_GWS_DOC_ID";
pub const GWS_SHEET_ID_ENV: &str = "ADL_GWS_SHEET_ID";
pub const GWS_SHEET_RANGE_ENV: &str = "ADL_GWS_SHEET_RANGE";
#[cfg(test)]
const HOST_PATH_MARKER: &str = "/Users/daniel/";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsLiveCapabilityPromptRecord {
    pub prompt_version: &'static str,
    pub issue_number: u32,
    pub depends_on_issue_number: u32,
    pub summary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GwsLiveMode {
    Disabled,
    DryRun,
    Execute,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GwsCapabilityResult {
    Proving,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GwsLiveSkipReason {
    LiveModeDisabled,
    DryRunOnly,
    GwsUnavailable,
    MissingScopeBinding,
    MissingAuth,
    MissingScopes,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsLiveScopeBinding {
    pub drive_folder_id: String,
    pub doc_id: String,
    pub sheet_id: String,
    pub sheet_range: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsLiveFileRecord {
    pub file_id: String,
    pub name: String,
    pub mime_type: String,
    pub modified_time: Option<String>,
    pub web_view_link: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsLiveDocRecord {
    pub document_id: String,
    pub title: String,
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsLiveSheetPreviewRecord {
    pub spreadsheet_id: String,
    pub range: String,
    pub row_count: usize,
    pub header_row: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsLiveCommandTraceRecord {
    pub capability_name: &'static str,
    pub argv: Vec<String>,
    pub mode: GwsLiveMode,
    pub result: GwsCapabilityResult,
    pub skipped_reason: Option<GwsLiveSkipReason>,
    pub exit_code: Option<i32>,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceCmsLiveSnapshot {
    pub schema_version: &'static str,
    pub mode: GwsLiveMode,
    pub cms_root: WorkspaceCmsRootRef,
    pub scope_binding: Option<GwsLiveScopeBinding>,
    pub folder_inventory: Vec<GwsLiveFileRecord>,
    pub selected_doc: Option<GwsLiveDocRecord>,
    pub content_card_sheet: Option<GwsLiveSheetPreviewRecord>,
    pub skipped_reason: Option<GwsLiveSkipReason>,
    pub non_claims: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsLiveCapabilityExecutionReport {
    pub schema_version: &'static str,
    pub prompt_record: GwsLiveCapabilityPromptRecord,
    pub live_mode: GwsLiveMode,
    pub fixture_snapshot_artifact_path: &'static str,
    pub safety_package_artifact_path: &'static str,
    pub live_snapshot_artifact_path: String,
    pub scope_binding: Option<GwsLiveScopeBinding>,
    pub command_traces: Vec<GwsLiveCommandTraceRecord>,
    pub live_snapshot_result: GwsCapabilityResult,
    pub live_snapshot_skipped_reason: Option<GwsLiveSkipReason>,
    pub fixture_mode_remains_canonical: bool,
    pub dry_run_posture_supported: bool,
    pub non_claims: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GwsCommandOutput {
    exit_code: i32,
    stdout: String,
    stderr: String,
}

trait GwsCommandRunner {
    fn run(&self, argv: &[String]) -> Result<GwsCommandOutput>;
}

struct SystemGwsCommandRunner;

impl GwsCommandRunner for SystemGwsCommandRunner {
    fn run(&self, argv: &[String]) -> Result<GwsCommandOutput> {
        let output = Command::new("gws")
            .args(argv)
            .output()
            .with_context(|| format!("run `gws {}`", argv.join(" ")))?;
        Ok(GwsCommandOutput {
            exit_code: output.status.code().unwrap_or(1),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}

fn prompt_record() -> GwsLiveCapabilityPromptRecord {
    GwsLiveCapabilityPromptRecord {
        prompt_version: GWS_LIVE_CAPABILITY_EXECUTION_PROMPT_VERSION,
        issue_number: 3091,
        depends_on_issue_number: 3008,
        summary:
            "WP-3091 promotes the bounded Workspace CMS bridge into a live-capable governed `gws` surface for one explicit folder/doc/sheet scope while preserving fixture-first proof and truthful skipped-state behavior.",
    }
}

fn tracked_path(relative: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(relative)
}

fn parse_live_mode_from_env() -> GwsLiveMode {
    match env::var(GWS_LIVE_ENABLE_ENV)
        .unwrap_or_else(|_| "disabled".to_string())
        .to_ascii_lowercase()
        .as_str()
    {
        "dry_run" | "dry-run" => GwsLiveMode::DryRun,
        "execute" | "live" | "enabled" => GwsLiveMode::Execute,
        _ => GwsLiveMode::Disabled,
    }
}

fn parse_scope_binding_from_env() -> Option<GwsLiveScopeBinding> {
    let drive_folder_id = env::var(GWS_DRIVE_FOLDER_ID_ENV).ok()?;
    let doc_id = env::var(GWS_DOC_ID_ENV).ok()?;
    let sheet_id = env::var(GWS_SHEET_ID_ENV).ok()?;
    let sheet_range = env::var(GWS_SHEET_RANGE_ENV).ok()?;
    Some(GwsLiveScopeBinding {
        drive_folder_id,
        doc_id,
        sheet_id,
        sheet_range,
    })
}

fn default_root(scope_binding: Option<&GwsLiveScopeBinding>) -> WorkspaceCmsRootRef {
    WorkspaceCmsRootRef {
        display_name: "Live bounded Workspace CMS scope".to_string(),
        folder_ref: scope_binding
            .map(|scope| format!("gws://drive/folders/{}", scope.drive_folder_id))
            .unwrap_or_else(|| "gws://drive/folders/unbound".to_string()),
        authority: WorkspaceAuthority::DraftOnly,
    }
}

fn skipped_trace(
    capability_name: &'static str,
    argv: Vec<String>,
    mode: GwsLiveMode,
    reason: GwsLiveSkipReason,
    summary: impl Into<String>,
) -> GwsLiveCommandTraceRecord {
    GwsLiveCommandTraceRecord {
        capability_name,
        argv,
        mode,
        result: GwsCapabilityResult::Skipped,
        skipped_reason: Some(reason),
        exit_code: None,
        summary: summary.into(),
    }
}

fn proving_trace(
    capability_name: &'static str,
    argv: Vec<String>,
    mode: GwsLiveMode,
    exit_code: i32,
    summary: impl Into<String>,
) -> GwsLiveCommandTraceRecord {
    GwsLiveCommandTraceRecord {
        capability_name,
        argv,
        mode,
        result: GwsCapabilityResult::Proving,
        skipped_reason: None,
        exit_code: Some(exit_code),
        summary: summary.into(),
    }
}

fn drive_inventory_args(scope: &GwsLiveScopeBinding) -> Vec<String> {
    vec![
        "drive".to_string(),
        "files".to_string(),
        "list".to_string(),
        "--params".to_string(),
        serde_json::json!({
            "q": format!("'{}' in parents and trashed = false", scope.drive_folder_id),
            "fields": "files(id,name,mimeType,modifiedTime,webViewLink)",
            "pageSize": 100
        })
        .to_string(),
    ]
}

fn docs_get_args(scope: &GwsLiveScopeBinding) -> Vec<String> {
    vec![
        "docs".to_string(),
        "documents".to_string(),
        "get".to_string(),
        "--params".to_string(),
        serde_json::json!({
            "documentId": scope.doc_id
        })
        .to_string(),
    ]
}

fn sheets_values_get_args(scope: &GwsLiveScopeBinding) -> Vec<String> {
    vec![
        "sheets".to_string(),
        "spreadsheets".to_string(),
        "values".to_string(),
        "get".to_string(),
        "--params".to_string(),
        serde_json::json!({
            "spreadsheetId": scope.sheet_id,
            "range": scope.sheet_range
        })
        .to_string(),
    ]
}

fn classify_command_failure(output: &GwsCommandOutput) -> GwsLiveSkipReason {
    let body = format!("{}\n{}", output.stdout, output.stderr).to_ascii_lowercase();
    if body.contains("scope")
        || body.contains("permission")
        || body.contains("forbidden")
        || body.contains("insufficient")
    {
        GwsLiveSkipReason::MissingScopes
    } else if body.contains("credential")
        || body.contains("login")
        || body.contains("oauth")
        || body.contains("token")
        || body.contains("unauth")
        || body.contains("auth")
    {
        GwsLiveSkipReason::MissingAuth
    } else {
        GwsLiveSkipReason::GwsUnavailable
    }
}

fn classify_runner_error(error: &anyhow::Error) -> GwsLiveSkipReason {
    let body = error.to_string().to_ascii_lowercase();
    if body.contains("scope")
        || body.contains("permission")
        || body.contains("forbidden")
        || body.contains("insufficient")
    {
        GwsLiveSkipReason::MissingScopes
    } else if body.contains("credential")
        || body.contains("login")
        || body.contains("oauth")
        || body.contains("token")
        || body.contains("unauth")
        || body.contains("auth")
    {
        GwsLiveSkipReason::MissingAuth
    } else {
        GwsLiveSkipReason::GwsUnavailable
    }
}

fn parse_drive_inventory(stdout: &str) -> Result<Vec<GwsLiveFileRecord>> {
    let value: Value = serde_json::from_str(stdout).context("parse drive inventory json")?;
    let files = value
        .get("files")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("drive inventory missing files array"))?;
    let mut records = Vec::with_capacity(files.len());
    for file in files {
        records.push(GwsLiveFileRecord {
            file_id: file
                .get("id")
                .and_then(Value::as_str)
                .ok_or_else(|| anyhow!("drive inventory file missing id"))?
                .to_string(),
            name: file
                .get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| anyhow!("drive inventory file missing name"))?
                .to_string(),
            mime_type: file
                .get("mimeType")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
                .to_string(),
            modified_time: file
                .get("modifiedTime")
                .and_then(Value::as_str)
                .map(ToString::to_string),
            web_view_link: file
                .get("webViewLink")
                .and_then(Value::as_str)
                .map(ToString::to_string),
        });
    }
    Ok(records)
}

fn parse_doc(stdout: &str) -> Result<GwsLiveDocRecord> {
    let value: Value = serde_json::from_str(stdout).context("parse docs get json")?;
    Ok(GwsLiveDocRecord {
        document_id: value
            .get("documentId")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("docs get missing documentId"))?
            .to_string(),
        title: value
            .get("title")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("docs get missing title"))?
            .to_string(),
        revision_id: value
            .get("revisionId")
            .and_then(Value::as_str)
            .map(ToString::to_string),
    })
}

fn parse_sheet(stdout: &str, scope: &GwsLiveScopeBinding) -> Result<GwsLiveSheetPreviewRecord> {
    let value: Value = serde_json::from_str(stdout).context("parse sheets values get json")?;
    let values = value
        .get("values")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("sheets values get missing values array"))?;
    let header_row = values
        .first()
        .and_then(Value::as_array)
        .map(|row| {
            row.iter()
                .filter_map(Value::as_str)
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(GwsLiveSheetPreviewRecord {
        spreadsheet_id: scope.sheet_id.clone(),
        range: value
            .get("range")
            .and_then(Value::as_str)
            .unwrap_or(scope.sheet_range.as_str())
            .to_string(),
        row_count: values.len(),
        header_row,
    })
}

fn run_execute_mode_with_runner(
    scope: &GwsLiveScopeBinding,
    runner: &dyn GwsCommandRunner,
) -> Result<(WorkspaceCmsLiveSnapshot, Vec<GwsLiveCommandTraceRecord>)> {
    let drive_args = drive_inventory_args(scope);
    let drive_output = runner.run(&drive_args);
    let drive_output = match drive_output {
        Ok(output) => output,
        Err(error) => {
            let reason = classify_runner_error(&error);
            return Ok((
                WorkspaceCmsLiveSnapshot {
                    schema_version: WORKSPACE_CMS_LIVE_SNAPSHOT_SCHEMA_VERSION,
                    mode: GwsLiveMode::Execute,
                    cms_root: default_root(Some(scope)),
                    scope_binding: Some(scope.clone()),
                    folder_inventory: vec![],
                    selected_doc: None,
                    content_card_sheet: None,
                    skipped_reason: Some(reason.clone()),
                    non_claims: default_non_claims(),
                },
                vec![skipped_trace(
                    "gws.drive.folder_inventory",
                    drive_args,
                    GwsLiveMode::Execute,
                    reason,
                    "The `gws` binary was unavailable, so live Workspace execution could not start.",
                )],
            ));
        }
    };
    if drive_output.exit_code != 0 {
        let reason = classify_command_failure(&drive_output);
        return Ok((
            WorkspaceCmsLiveSnapshot {
                schema_version: WORKSPACE_CMS_LIVE_SNAPSHOT_SCHEMA_VERSION,
                mode: GwsLiveMode::Execute,
                cms_root: default_root(Some(scope)),
                scope_binding: Some(scope.clone()),
                folder_inventory: vec![],
                selected_doc: None,
                content_card_sheet: None,
                skipped_reason: Some(reason.clone()),
                non_claims: default_non_claims(),
            },
            vec![skipped_trace(
                "gws.drive.folder_inventory",
                drive_args,
                GwsLiveMode::Execute,
                reason,
                "Drive folder inventory did not complete because the live `gws` request was not authorized or bounded correctly.",
            )],
        ));
    }
    let folder_inventory = parse_drive_inventory(&drive_output.stdout)?;
    let mut traces = vec![proving_trace(
        "gws.drive.folder_inventory",
        drive_args,
        GwsLiveMode::Execute,
        drive_output.exit_code,
        format!(
            "Inventoried {} live Drive files for the bounded folder scope.",
            folder_inventory.len()
        ),
    )];

    let doc_args = docs_get_args(scope);
    let doc_output = match runner.run(&doc_args) {
        Ok(output) => output,
        Err(error) => {
            let reason = classify_runner_error(&error);
            traces.push(skipped_trace(
                "gws.docs.read_snapshot",
                doc_args,
                GwsLiveMode::Execute,
                reason.clone(),
                "Document snapshot read could not start through `gws`, so the live run is recorded as skipped instead of failing generically.",
            ));
            return Ok((
                WorkspaceCmsLiveSnapshot {
                    schema_version: WORKSPACE_CMS_LIVE_SNAPSHOT_SCHEMA_VERSION,
                    mode: GwsLiveMode::Execute,
                    cms_root: default_root(Some(scope)),
                    scope_binding: Some(scope.clone()),
                    folder_inventory,
                    selected_doc: None,
                    content_card_sheet: None,
                    skipped_reason: Some(reason),
                    non_claims: default_non_claims(),
                },
                traces,
            ));
        }
    };
    if doc_output.exit_code != 0 {
        let reason = classify_command_failure(&doc_output);
        traces.push(skipped_trace(
            "gws.docs.read_snapshot",
            doc_args,
            GwsLiveMode::Execute,
            reason.clone(),
            "Document snapshot read did not complete because the live `gws` request lacked working auth or scope.",
        ));
        return Ok((
            WorkspaceCmsLiveSnapshot {
                schema_version: WORKSPACE_CMS_LIVE_SNAPSHOT_SCHEMA_VERSION,
                mode: GwsLiveMode::Execute,
                cms_root: default_root(Some(scope)),
                scope_binding: Some(scope.clone()),
                folder_inventory,
                selected_doc: None,
                content_card_sheet: None,
                skipped_reason: Some(reason),
                non_claims: default_non_claims(),
            },
            traces,
        ));
    }
    let selected_doc = parse_doc(&doc_output.stdout)?;
    traces.push(proving_trace(
        "gws.docs.read_snapshot",
        doc_args,
        GwsLiveMode::Execute,
        doc_output.exit_code,
        format!(
            "Read one bounded live document snapshot for '{}'.",
            selected_doc.title
        ),
    ));

    let sheet_args = sheets_values_get_args(scope);
    let sheet_output = match runner.run(&sheet_args) {
        Ok(output) => output,
        Err(error) => {
            let reason = classify_runner_error(&error);
            traces.push(skipped_trace(
                "gws.sheets.read_content_cards",
                sheet_args,
                GwsLiveMode::Execute,
                reason.clone(),
                "Content-card sheet read could not start through `gws`, so the live run is recorded as skipped instead of failing generically.",
            ));
            return Ok((
                WorkspaceCmsLiveSnapshot {
                    schema_version: WORKSPACE_CMS_LIVE_SNAPSHOT_SCHEMA_VERSION,
                    mode: GwsLiveMode::Execute,
                    cms_root: default_root(Some(scope)),
                    scope_binding: Some(scope.clone()),
                    folder_inventory,
                    selected_doc: Some(selected_doc),
                    content_card_sheet: None,
                    skipped_reason: Some(reason),
                    non_claims: default_non_claims(),
                },
                traces,
            ));
        }
    };
    if sheet_output.exit_code != 0 {
        let reason = classify_command_failure(&sheet_output);
        traces.push(skipped_trace(
            "gws.sheets.read_content_cards",
            sheet_args,
            GwsLiveMode::Execute,
            reason.clone(),
            "Content-card sheet read did not complete because the live `gws` request lacked working auth or scope.",
        ));
        return Ok((
            WorkspaceCmsLiveSnapshot {
                schema_version: WORKSPACE_CMS_LIVE_SNAPSHOT_SCHEMA_VERSION,
                mode: GwsLiveMode::Execute,
                cms_root: default_root(Some(scope)),
                scope_binding: Some(scope.clone()),
                folder_inventory,
                selected_doc: Some(selected_doc),
                content_card_sheet: None,
                skipped_reason: Some(reason),
                non_claims: default_non_claims(),
            },
            traces,
        ));
    }
    let content_card_sheet = parse_sheet(&sheet_output.stdout, scope)?;
    traces.push(proving_trace(
        "gws.sheets.read_content_cards",
        sheet_args,
        GwsLiveMode::Execute,
        sheet_output.exit_code,
        format!(
            "Read one bounded live content-card sheet range with {} rows.",
            content_card_sheet.row_count
        ),
    ));

    Ok((
        WorkspaceCmsLiveSnapshot {
            schema_version: WORKSPACE_CMS_LIVE_SNAPSHOT_SCHEMA_VERSION,
            mode: GwsLiveMode::Execute,
            cms_root: default_root(Some(scope)),
            scope_binding: Some(scope.clone()),
            folder_inventory,
            selected_doc: Some(selected_doc),
            content_card_sheet: Some(content_card_sheet),
            skipped_reason: None,
            non_claims: default_non_claims(),
        },
        traces,
    ))
}

fn run_live_snapshot_with_runner(
    live_mode: GwsLiveMode,
    scope_binding: Option<GwsLiveScopeBinding>,
    runner: &dyn GwsCommandRunner,
) -> Result<(WorkspaceCmsLiveSnapshot, Vec<GwsLiveCommandTraceRecord>)> {
    match live_mode {
        GwsLiveMode::Disabled => Ok((
            WorkspaceCmsLiveSnapshot {
                schema_version: WORKSPACE_CMS_LIVE_SNAPSHOT_SCHEMA_VERSION,
                mode: GwsLiveMode::Disabled,
                cms_root: default_root(scope_binding.as_ref()),
                scope_binding,
                folder_inventory: vec![],
                selected_doc: None,
                content_card_sheet: None,
                skipped_reason: Some(GwsLiveSkipReason::LiveModeDisabled),
                non_claims: default_non_claims(),
            },
            vec![],
        )),
        GwsLiveMode::DryRun => {
            let scope_binding = match scope_binding {
                Some(scope) => scope,
                None => {
                    return Ok((
                        WorkspaceCmsLiveSnapshot {
                            schema_version: WORKSPACE_CMS_LIVE_SNAPSHOT_SCHEMA_VERSION,
                            mode: GwsLiveMode::DryRun,
                            cms_root: default_root(None),
                            scope_binding: None,
                            folder_inventory: vec![],
                            selected_doc: None,
                            content_card_sheet: None,
                            skipped_reason: Some(GwsLiveSkipReason::MissingScopeBinding),
                            non_claims: default_non_claims(),
                        },
                        vec![],
                    ));
                }
            };
            Ok((
                WorkspaceCmsLiveSnapshot {
                    schema_version: WORKSPACE_CMS_LIVE_SNAPSHOT_SCHEMA_VERSION,
                    mode: GwsLiveMode::DryRun,
                    cms_root: default_root(Some(&scope_binding)),
                    scope_binding: Some(scope_binding.clone()),
                    folder_inventory: vec![],
                    selected_doc: None,
                    content_card_sheet: None,
                    skipped_reason: Some(GwsLiveSkipReason::DryRunOnly),
                    non_claims: default_non_claims(),
                },
                vec![
                    skipped_trace(
                        "gws.drive.folder_inventory",
                        drive_inventory_args(&scope_binding),
                        GwsLiveMode::DryRun,
                        GwsLiveSkipReason::DryRunOnly,
                        "Dry-run posture records the bounded Drive inventory command plan without executing live Workspace reads.",
                    ),
                    skipped_trace(
                        "gws.docs.read_snapshot",
                        docs_get_args(&scope_binding),
                        GwsLiveMode::DryRun,
                        GwsLiveSkipReason::DryRunOnly,
                        "Dry-run posture records the bounded document snapshot command plan without executing a live Workspace read.",
                    ),
                    skipped_trace(
                        "gws.sheets.read_content_cards",
                        sheets_values_get_args(&scope_binding),
                        GwsLiveMode::DryRun,
                        GwsLiveSkipReason::DryRunOnly,
                        "Dry-run posture records the bounded sheet-read command plan without executing a live Workspace read.",
                    ),
                ],
            ))
        }
        GwsLiveMode::Execute => match scope_binding {
            Some(scope) => run_execute_mode_with_runner(&scope, runner),
            None => Ok((
                WorkspaceCmsLiveSnapshot {
                    schema_version: WORKSPACE_CMS_LIVE_SNAPSHOT_SCHEMA_VERSION,
                    mode: GwsLiveMode::Execute,
                    cms_root: default_root(None),
                    scope_binding: None,
                    folder_inventory: vec![],
                    selected_doc: None,
                    content_card_sheet: None,
                    skipped_reason: Some(GwsLiveSkipReason::MissingScopeBinding),
                    non_claims: default_non_claims(),
                },
                vec![],
            )),
        },
    }
}

fn default_non_claims() -> Vec<&'static str> {
    vec![
        "This surface does not make Google Workspace canonical repo truth.",
        "This surface does not authorize direct tracked repository mutation from Workspace state.",
        "Fixture mode remains the proving path for ordinary CI and PR validation.",
    ]
}

fn build_report(
    live_mode: GwsLiveMode,
    scope_binding: Option<GwsLiveScopeBinding>,
    snapshot_artifact_path: String,
    snapshot: &WorkspaceCmsLiveSnapshot,
    command_traces: Vec<GwsLiveCommandTraceRecord>,
) -> GwsLiveCapabilityExecutionReport {
    GwsLiveCapabilityExecutionReport {
        schema_version: GWS_LIVE_CAPABILITY_EXECUTION_SCHEMA_VERSION,
        prompt_record: prompt_record(),
        live_mode,
        fixture_snapshot_artifact_path:
            crate::rust_native_gws_adapter_boundary::WORKSPACE_CMS_SNAPSHOT_ARTIFACT_PATH,
        safety_package_artifact_path:
            crate::gws_live_safety_package::GWS_LIVE_SAFETY_PACKAGE_REPORT_ARTIFACT_PATH,
        live_snapshot_artifact_path: snapshot_artifact_path,
        scope_binding,
        command_traces,
        live_snapshot_result: if snapshot.skipped_reason.is_some() {
            GwsCapabilityResult::Skipped
        } else {
            GwsCapabilityResult::Proving
        },
        live_snapshot_skipped_reason: snapshot.skipped_reason.clone(),
        fixture_mode_remains_canonical: true,
        dry_run_posture_supported: true,
        non_claims: vec![
            "This report does not prove live Workspace writes.",
            "This report does not require live secrets for ordinary validation.",
            "This report does not override the bounded authority and revision-mismatch rules already recorded by the bridge feature.",
        ],
    }
}

pub fn run_gws_live_capability_execution_report(
) -> Result<(GwsLiveCapabilityExecutionReport, WorkspaceCmsLiveSnapshot)> {
    let live_mode = parse_live_mode_from_env();
    let scope_binding = parse_scope_binding_from_env();
    let runner = SystemGwsCommandRunner;
    let (snapshot, command_traces) =
        run_live_snapshot_with_runner(live_mode.clone(), scope_binding.clone(), &runner)?;
    let report = build_report(
        live_mode,
        scope_binding,
        GWS_LIVE_CAPABILITY_EXECUTION_SNAPSHOT_ARTIFACT_PATH.to_string(),
        &snapshot,
        command_traces,
    );
    Ok((report, snapshot))
}

pub fn write_gws_live_capability_execution_report(
    report_path: impl AsRef<Path>,
) -> Result<GwsLiveCapabilityExecutionReport> {
    let report_path = report_path.as_ref();
    let snapshot_path = derive_snapshot_path(report_path);
    write_gws_live_capability_execution_artifacts(report_path, snapshot_path)
        .map(|(report, _snapshot)| report)
}

fn derive_snapshot_path(report_path: &Path) -> std::path::PathBuf {
    let tracked_report_path = tracked_path(GWS_LIVE_CAPABILITY_EXECUTION_REPORT_ARTIFACT_PATH);
    if report_path == tracked_report_path {
        return tracked_path(GWS_LIVE_CAPABILITY_EXECUTION_SNAPSHOT_ARTIFACT_PATH);
    }
    let stem = report_path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("gws-live-capability-execution-report");
    let derived_name = if stem.contains("report") {
        stem.replacen("report", "snapshot", 1)
    } else {
        format!("{stem}-snapshot")
    };
    let extension = report_path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("json");
    report_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(format!("{derived_name}.{extension}"))
}

pub fn write_gws_live_capability_execution_artifacts(
    report_path: impl AsRef<Path>,
    snapshot_path: impl AsRef<Path>,
) -> Result<(GwsLiveCapabilityExecutionReport, WorkspaceCmsLiveSnapshot)> {
    let (report, snapshot) = run_gws_live_capability_execution_report()?;
    let report_path = report_path.as_ref();
    let snapshot_path = snapshot_path.as_ref();
    let report = build_report(
        report.live_mode.clone(),
        report.scope_binding.clone(),
        snapshot_path.display().to_string(),
        &snapshot,
        report.command_traces.clone(),
    );
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!("create parent directories for '{}'", report_path.display())
        })?;
    }
    if let Some(parent) = snapshot_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "create parent directories for '{}'",
                snapshot_path.display()
            )
        })?;
    }
    fs::write(report_path, serde_json::to_string_pretty(&report)?)
        .with_context(|| format!("write '{}'", report_path.display()))?;
    fs::write(snapshot_path, serde_json::to_string_pretty(&snapshot)?)
        .with_context(|| format!("write '{}'", snapshot_path.display()))?;
    Ok((report, snapshot))
}

#[cfg(test)]
mod tests {
    use super::{
        build_report, drive_inventory_args, parse_doc, parse_drive_inventory,
        parse_live_mode_from_env, parse_scope_binding_from_env, parse_sheet,
        run_live_snapshot_with_runner, write_gws_live_capability_execution_artifacts,
        GwsCapabilityResult, GwsCommandOutput, GwsCommandRunner, GwsLiveMode, GwsLiveScopeBinding,
        GwsLiveSkipReason, GWS_DOC_ID_ENV, GWS_DRIVE_FOLDER_ID_ENV, GWS_LIVE_ENABLE_ENV,
        GWS_SHEET_ID_ENV, GWS_SHEET_RANGE_ENV, HOST_PATH_MARKER,
    };
    use crate::gws_live_test_support::{lock_gws_live_test_env, EnvVarGuard};
    use std::collections::VecDeque;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Default)]
    struct FakeRunner {
        outputs: VecDeque<Result<GwsCommandOutput, anyhow::Error>>,
    }

    impl FakeRunner {
        fn with_outputs(outputs: Vec<Result<GwsCommandOutput, anyhow::Error>>) -> Self {
            Self {
                outputs: outputs.into(),
            }
        }
    }

    impl GwsCommandRunner for FakeRunner {
        fn run(&self, _argv: &[String]) -> anyhow::Result<GwsCommandOutput> {
            unreachable!("mutable fake runner required")
        }
    }

    impl FakeRunner {
        fn run_mut(&mut self, _argv: &[String]) -> anyhow::Result<GwsCommandOutput> {
            self.outputs.pop_front().expect("fake output should exist")
        }
    }

    struct MutableRunner(std::cell::RefCell<FakeRunner>);

    impl GwsCommandRunner for MutableRunner {
        fn run(&self, argv: &[String]) -> anyhow::Result<GwsCommandOutput> {
            self.0.borrow_mut().run_mut(argv)
        }
    }

    fn scope_binding() -> GwsLiveScopeBinding {
        GwsLiveScopeBinding {
            drive_folder_id: "folder-123".to_string(),
            doc_id: "doc-456".to_string(),
            sheet_id: "sheet-789".to_string(),
            sheet_range: "ContentCards!A1:F5".to_string(),
        }
    }

    fn unique_temp_path(prefix: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nanos}.json"))
    }

    #[test]
    fn gws_live_capability_execution_env_helpers_cover_aliases_and_scope_binding() {
        let _lock = lock_gws_live_test_env();
        let _mode = EnvVarGuard::set(GWS_LIVE_ENABLE_ENV, "enabled");
        let _drive = EnvVarGuard::set(GWS_DRIVE_FOLDER_ID_ENV, "folder-live");
        let _doc = EnvVarGuard::set(GWS_DOC_ID_ENV, "doc-live");
        let _sheet = EnvVarGuard::set(GWS_SHEET_ID_ENV, "sheet-live");
        let _range = EnvVarGuard::set(GWS_SHEET_RANGE_ENV, "ContentCards!A1:F5");

        assert_eq!(parse_live_mode_from_env(), GwsLiveMode::Execute);
        assert_eq!(
            parse_scope_binding_from_env(),
            Some(GwsLiveScopeBinding {
                drive_folder_id: "folder-live".to_string(),
                doc_id: "doc-live".to_string(),
                sheet_id: "sheet-live".to_string(),
                sheet_range: "ContentCards!A1:F5".to_string(),
            })
        );
    }

    #[test]
    fn gws_live_capability_execution_disabled_mode_is_truthfully_skipped() {
        let runner = MutableRunner(std::cell::RefCell::new(FakeRunner::default()));
        let (snapshot, traces) =
            run_live_snapshot_with_runner(GwsLiveMode::Disabled, None, &runner)
                .expect("disabled mode should succeed");
        assert_eq!(
            snapshot.skipped_reason,
            Some(GwsLiveSkipReason::LiveModeDisabled)
        );
        assert!(traces.is_empty());
    }

    #[test]
    fn gws_live_capability_execution_dry_run_records_planned_commands() {
        let runner = MutableRunner(std::cell::RefCell::new(FakeRunner::default()));
        let scope = scope_binding();
        let (snapshot, traces) =
            run_live_snapshot_with_runner(GwsLiveMode::DryRun, Some(scope.clone()), &runner)
                .expect("dry-run mode should succeed");
        assert_eq!(snapshot.skipped_reason, Some(GwsLiveSkipReason::DryRunOnly));
        assert_eq!(traces.len(), 3);
        assert_eq!(traces[0].argv, drive_inventory_args(&scope));
    }

    #[test]
    fn gws_live_capability_execution_execute_mode_normalizes_live_outputs() {
        let drive_output = Ok(GwsCommandOutput {
            exit_code: 0,
            stdout: serde_json::json!({
                "files": [
                    {
                        "id": "doc-456",
                        "name": "Review Packet Draft",
                        "mimeType": "application/vnd.google-apps.document",
                        "modifiedTime": "2026-05-15T00:00:00Z",
                        "webViewLink": "https://docs.google.com/document/d/doc-456/edit"
                    }
                ]
            })
            .to_string(),
            stderr: String::new(),
        });
        let doc_output = Ok(GwsCommandOutput {
            exit_code: 0,
            stdout: serde_json::json!({
                "documentId": "doc-456",
                "title": "Review Packet Draft",
                "revisionId": "rev-77"
            })
            .to_string(),
            stderr: String::new(),
        });
        let sheet_output = Ok(GwsCommandOutput {
            exit_code: 0,
            stdout: serde_json::json!({
                "range": "ContentCards!A1:F5",
                "values": [
                    ["doc_id", "title", "status"],
                    ["doc-456", "Review Packet Draft", "ready_for_repo_promotion"]
                ]
            })
            .to_string(),
            stderr: String::new(),
        });
        let runner = MutableRunner(std::cell::RefCell::new(FakeRunner::with_outputs(vec![
            drive_output,
            doc_output,
            sheet_output,
        ])));
        let (snapshot, traces) =
            run_live_snapshot_with_runner(GwsLiveMode::Execute, Some(scope_binding()), &runner)
                .expect("execute mode should succeed");
        assert_eq!(snapshot.skipped_reason, None);
        assert_eq!(snapshot.folder_inventory.len(), 1);
        assert_eq!(
            snapshot
                .selected_doc
                .as_ref()
                .expect("doc")
                .revision_id
                .as_deref(),
            Some("rev-77")
        );
        assert_eq!(
            snapshot
                .content_card_sheet
                .as_ref()
                .expect("sheet")
                .row_count,
            2
        );
        assert_eq!(traces.len(), 3);
        assert!(traces
            .iter()
            .all(|trace| trace.result == GwsCapabilityResult::Proving));
    }

    #[test]
    fn gws_live_capability_execution_auth_failure_is_skipped_not_failed() {
        let runner = MutableRunner(std::cell::RefCell::new(FakeRunner::with_outputs(vec![Ok(
            GwsCommandOutput {
                exit_code: 1,
                stdout: String::new(),
                stderr: "Please login: missing OAuth credentials".to_string(),
            },
        )])));
        let (snapshot, traces) =
            run_live_snapshot_with_runner(GwsLiveMode::Execute, Some(scope_binding()), &runner)
                .expect("auth failure should classify as skipped");
        assert_eq!(
            snapshot.skipped_reason,
            Some(GwsLiveSkipReason::MissingAuth)
        );
        assert_eq!(traces.len(), 1);
        assert_eq!(
            traces[0].skipped_reason,
            Some(GwsLiveSkipReason::MissingAuth)
        );
    }

    #[test]
    fn gws_live_capability_execution_late_runner_failure_is_skipped_not_error() {
        let drive_output = Ok(GwsCommandOutput {
            exit_code: 0,
            stdout: serde_json::json!({
                "files": [{"id":"doc-456","name":"Review Packet Draft","mimeType":"application/vnd.google-apps.document"}]
            })
            .to_string(),
            stderr: String::new(),
        });
        let doc_runner_error = Err(anyhow::anyhow!(
            "run `gws docs documents get`: auth token missing"
        ));
        let runner = MutableRunner(std::cell::RefCell::new(FakeRunner::with_outputs(vec![
            drive_output,
            doc_runner_error,
        ])));
        let (snapshot, traces) =
            run_live_snapshot_with_runner(GwsLiveMode::Execute, Some(scope_binding()), &runner)
                .expect("late runner failure should classify as skipped");
        assert_eq!(
            snapshot.skipped_reason,
            Some(GwsLiveSkipReason::MissingAuth)
        );
        assert_eq!(traces.len(), 2);
        assert_eq!(
            traces[1].skipped_reason,
            Some(GwsLiveSkipReason::MissingAuth)
        );
    }

    #[test]
    fn gws_live_capability_execution_write_artifacts_creates_expected_json_files() {
        let report_path = unique_temp_path("gws-live-capability-report");
        let snapshot_path = unique_temp_path("gws-live-capability-snapshot");
        let (report, snapshot) =
            write_gws_live_capability_execution_artifacts(&report_path, &snapshot_path)
                .expect("write artifacts");
        assert_eq!(
            report.schema_version,
            super::GWS_LIVE_CAPABILITY_EXECUTION_SCHEMA_VERSION
        );
        assert_eq!(
            snapshot.schema_version,
            super::WORKSPACE_CMS_LIVE_SNAPSHOT_SCHEMA_VERSION
        );
        let report_body = fs::read_to_string(&report_path).expect("read report");
        let snapshot_body = fs::read_to_string(&snapshot_path).expect("read snapshot");
        assert!(report_body.contains(super::GWS_LIVE_CAPABILITY_EXECUTION_SCHEMA_VERSION));
        assert!(report_body.contains(&snapshot_path.display().to_string()));
        assert!(snapshot_body.contains("workspace_cms_live_snapshot.v1"));
        assert!(!report_body.contains(HOST_PATH_MARKER));
        assert!(!snapshot_body.contains(HOST_PATH_MARKER));
        fs::remove_file(report_path).expect("remove report");
        fs::remove_file(snapshot_path).expect("remove snapshot");
    }

    #[test]
    fn gws_live_capability_execution_parsers_enforce_expected_shape() {
        let drive = parse_drive_inventory("{\"files\":[]}").expect("drive parse");
        let doc = parse_doc("{\"documentId\":\"doc-1\",\"title\":\"Doc\"}").expect("doc parse");
        let sheet = parse_sheet(
            "{\"range\":\"A1:B2\",\"values\":[[\"a\",\"b\"],[\"1\",\"2\"]]}",
            &scope_binding(),
        )
        .expect("sheet parse");
        assert!(drive.is_empty());
        assert_eq!(doc.title, "Doc");
        assert_eq!(sheet.header_row, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn gws_live_capability_execution_report_tracks_skipped_snapshot_result() {
        let runner = MutableRunner(std::cell::RefCell::new(FakeRunner::default()));
        let (snapshot, traces) =
            run_live_snapshot_with_runner(GwsLiveMode::Disabled, None, &runner)
                .expect("disabled mode should succeed");
        let report = build_report(
            GwsLiveMode::Disabled,
            None,
            super::GWS_LIVE_CAPABILITY_EXECUTION_SNAPSHOT_ARTIFACT_PATH.to_string(),
            &snapshot,
            traces,
        );
        assert_eq!(report.live_snapshot_result, GwsCapabilityResult::Skipped);
        assert_eq!(
            report.live_snapshot_skipped_reason,
            Some(GwsLiveSkipReason::LiveModeDisabled)
        );
    }

    #[test]
    fn gws_live_capability_execution_custom_report_path_uses_sidecar_snapshot() {
        let report_path = std::path::Path::new("tmp/custom-live-report.json");
        let snapshot_path = super::derive_snapshot_path(report_path);
        assert_eq!(
            snapshot_path,
            std::path::PathBuf::from("tmp/custom-live-snapshot.json")
        );
    }
}
