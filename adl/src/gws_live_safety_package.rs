use anyhow::{Context, Result};
use serde::Serialize;
use std::{fs, path::Path};

pub const GWS_LIVE_SAFETY_PACKAGE_REPORT_ARTIFACT_PATH: &str =
    "docs/milestones/v0.91.2/review/google_workspace_cms_bridge/gws_live_safety_package_report.json";
pub const GWS_LIVE_SAFETY_PACKAGE_SCHEMA_VERSION: &str = "gws_live_safety_package.v1";
pub const GWS_LIVE_SAFETY_PACKAGE_PROMPT_VERSION: &str = "wp3092.gws_live_safety_package.v1";
#[cfg(test)]
const HOST_PATH_MARKER: &str = "/Users/daniel/";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsLiveSafetyPromptRecord {
    pub prompt_version: &'static str,
    pub issue_number: u32,
    pub depends_on_issue_number: u32,
    pub summary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceAuthMode {
    FixtureOnly,
    OperatorOauthUser,
    ServiceAccountScoped,
    ExternalCliCredentialStore,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceScopeKind {
    DriveFolderInventoryRead,
    DriveFileMetadataRead,
    DocsReadSnapshot,
    SheetsReadContentCards,
    SheetsPreviewContentCardUpdate,
    SheetsApplyContentCardUpdate,
    DriveRecordRevisionAnchor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceDataSensitivity {
    MetadataOnly,
    RedactedExcerpt,
    PrivateBody,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceTracePosture {
    MetadataOnly,
    RedactedContentOnly,
    IssueScopedLinkOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceSkippedReason {
    LiveModeDisabled,
    GwsUnavailable,
    MissingAuth,
    MissingScopes,
    NetworkUnavailable,
    OperatorDeclined,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceStopCondition {
    RevisionMismatch,
    AuthorityDrift,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceAuthProfileRecord {
    pub auth_mode: WorkspaceAuthMode,
    pub permitted_for_live_project_use: bool,
    pub secret_recording_policy: &'static str,
    pub notes: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceScopePolicyRecord {
    pub capability_name: &'static str,
    pub scope_kind: WorkspaceScopeKind,
    pub minimum_scope_summary: &'static str,
    pub write_capability: bool,
    pub operator_approval_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceRedactionRuleRecord {
    pub scenario: &'static str,
    pub data_sensitivity: WorkspaceDataSensitivity,
    pub trace_posture: WorkspaceTracePosture,
    pub default_public_artifact_behavior: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceSkippedStateRecord {
    pub reason: WorkspaceSkippedReason,
    pub classification: &'static str,
    pub operator_message: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceStopConditionRecord {
    pub condition: WorkspaceStopCondition,
    pub classification: &'static str,
    pub operator_message: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceRunbookRecord {
    pub stage: &'static str,
    pub required_actions: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsLiveSafetyPackageReport {
    pub schema_version: &'static str,
    pub prompt_record: GwsLiveSafetyPromptRecord,
    pub auth_profiles: Vec<WorkspaceAuthProfileRecord>,
    pub scope_policies: Vec<WorkspaceScopePolicyRecord>,
    pub redaction_rules: Vec<WorkspaceRedactionRuleRecord>,
    pub skipped_states: Vec<WorkspaceSkippedStateRecord>,
    pub stop_conditions: Vec<WorkspaceStopConditionRecord>,
    pub operator_runbook: Vec<WorkspaceRunbookRecord>,
    pub non_claims: Vec<&'static str>,
}

fn prompt_record() -> GwsLiveSafetyPromptRecord {
    GwsLiveSafetyPromptRecord {
        prompt_version: GWS_LIVE_SAFETY_PACKAGE_PROMPT_VERSION,
        issue_number: 3092,
        depends_on_issue_number: 3008,
        summary: "WP-3092 defines the auth, scope, redaction, trace, and skipped-state contract required to use the bounded Workspace CMS bridge safely on real projects.",
    }
}

fn auth_profiles() -> Vec<WorkspaceAuthProfileRecord> {
    vec![
        WorkspaceAuthProfileRecord {
            auth_mode: WorkspaceAuthMode::FixtureOnly,
            permitted_for_live_project_use: false,
            secret_recording_policy: "no live credentials are used or recorded",
            notes: vec![
                "Fixture mode remains the default proof path for CI and PR validation.",
                "Fixture mode is proving for schema and workflow truth, not for live Workspace reachability.",
            ],
        },
        WorkspaceAuthProfileRecord {
            auth_mode: WorkspaceAuthMode::OperatorOauthUser,
            permitted_for_live_project_use: true,
            secret_recording_policy: "record auth mode only; never persist tokens, refresh tokens, or raw credential files",
            notes: vec![
                "Use when one human operator is performing a bounded project workflow.",
                "Prefer a narrowly scoped Workspace identity rather than a broad personal account.",
            ],
        },
        WorkspaceAuthProfileRecord {
            auth_mode: WorkspaceAuthMode::ServiceAccountScoped,
            permitted_for_live_project_use: true,
            secret_recording_policy: "record service-account mode and issuing system only; never persist key material",
            notes: vec![
                "Use only when the project can prove the account is narrowed to the exact Drive/Docs/Sheets scope.",
                "Treat service-account write scopes as higher risk than read-only bounded demos.",
            ],
        },
        WorkspaceAuthProfileRecord {
            auth_mode: WorkspaceAuthMode::ExternalCliCredentialStore,
            permitted_for_live_project_use: true,
            secret_recording_policy: "record that external CLI credential storage was used without copying any secret-bearing path or content",
            notes: vec![
                "This is the likely first live adapter posture for `gws`-backed project use.",
                "The bridge may rely on the operator-visible CLI auth flow, but ADL must still record only the mode, not the secret.",
            ],
        },
    ]
}

fn scope_policies() -> Vec<WorkspaceScopePolicyRecord> {
    vec![
        WorkspaceScopePolicyRecord {
            capability_name: "workspace.drive.list_folder",
            scope_kind: WorkspaceScopeKind::DriveFolderInventoryRead,
            minimum_scope_summary: "one explicit Drive folder inventory scope",
            write_capability: false,
            operator_approval_required: false,
        },
        WorkspaceScopePolicyRecord {
            capability_name: "workspace.drive.read_file_metadata",
            scope_kind: WorkspaceScopeKind::DriveFileMetadataRead,
            minimum_scope_summary:
                "metadata read for files already inside the bounded folder scope",
            write_capability: false,
            operator_approval_required: false,
        },
        WorkspaceScopePolicyRecord {
            capability_name: "workspace.docs.read_snapshot",
            scope_kind: WorkspaceScopeKind::DocsReadSnapshot,
            minimum_scope_summary: "one explicit document snapshot read scope",
            write_capability: false,
            operator_approval_required: false,
        },
        WorkspaceScopePolicyRecord {
            capability_name: "workspace.sheets.read_content_cards",
            scope_kind: WorkspaceScopeKind::SheetsReadContentCards,
            minimum_scope_summary: "one explicit sheet range read scope for content cards only",
            write_capability: false,
            operator_approval_required: false,
        },
        WorkspaceScopePolicyRecord {
            capability_name: "workspace.sheets.preview_content_card_update",
            scope_kind: WorkspaceScopeKind::SheetsPreviewContentCardUpdate,
            minimum_scope_summary: "one explicit sheet range preview scope with no live mutation",
            write_capability: false,
            operator_approval_required: false,
        },
        WorkspaceScopePolicyRecord {
            capability_name: "workspace.sheets.apply_content_card_update",
            scope_kind: WorkspaceScopeKind::SheetsApplyContentCardUpdate,
            minimum_scope_summary:
                "one explicit sheet range write scope for one bounded content-card update",
            write_capability: true,
            operator_approval_required: true,
        },
        WorkspaceScopePolicyRecord {
            capability_name: "workspace.drive.record_revision_anchor",
            scope_kind: WorkspaceScopeKind::DriveRecordRevisionAnchor,
            minimum_scope_summary:
                "one bounded revision-anchor recording scope tied to the reviewed source doc",
            write_capability: false,
            operator_approval_required: false,
        },
    ]
}

fn redaction_rules() -> Vec<WorkspaceRedactionRuleRecord> {
    vec![
        WorkspaceRedactionRuleRecord {
            scenario: "drive folder inventory",
            data_sensitivity: WorkspaceDataSensitivity::MetadataOnly,
            trace_posture: WorkspaceTracePosture::MetadataOnly,
            default_public_artifact_behavior: "record folder ref, document ids, and bounded metadata only",
        },
        WorkspaceRedactionRuleRecord {
            scenario: "docs snapshot read",
            data_sensitivity: WorkspaceDataSensitivity::PrivateBody,
            trace_posture: WorkspaceTracePosture::RedactedContentOnly,
            default_public_artifact_behavior: "publish only redacted excerpts or issue-scoped references, never full private document bodies",
        },
        WorkspaceRedactionRuleRecord {
            scenario: "sheet content-card read",
            data_sensitivity: WorkspaceDataSensitivity::MetadataOnly,
            trace_posture: WorkspaceTracePosture::MetadataOnly,
            default_public_artifact_behavior: "publish only lifecycle/status rows that are already intended for repo-facing management",
        },
        WorkspaceRedactionRuleRecord {
            scenario: "comments and suggestions",
            data_sensitivity: WorkspaceDataSensitivity::PrivateBody,
            trace_posture: WorkspaceTracePosture::IssueScopedLinkOnly,
            default_public_artifact_behavior: "record that collaboration state exists without copying private discussion into public artifacts",
        },
        WorkspaceRedactionRuleRecord {
            scenario: "promotion packet preparation",
            data_sensitivity: WorkspaceDataSensitivity::RedactedExcerpt,
            trace_posture: WorkspaceTracePosture::MetadataOnly,
            default_public_artifact_behavior: "record title, revision anchor, target repo path, and issue/PR linkage without body-level leakage",
        },
    ]
}

fn skipped_states() -> Vec<WorkspaceSkippedStateRecord> {
    vec![
        WorkspaceSkippedStateRecord {
            reason: WorkspaceSkippedReason::LiveModeDisabled,
            classification: "skipped",
            operator_message: "Live Workspace execution is disabled for this run; fixture proof remains valid.",
        },
        WorkspaceSkippedStateRecord {
            reason: WorkspaceSkippedReason::GwsUnavailable,
            classification: "skipped",
            operator_message: "The `gws` adapter is unavailable; do not fail fixture-backed validation because live execution could not start.",
        },
        WorkspaceSkippedStateRecord {
            reason: WorkspaceSkippedReason::MissingAuth,
            classification: "skipped",
            operator_message: "No live Workspace credentials are available; record the missing auth posture without retrying ambient authority.",
        },
        WorkspaceSkippedStateRecord {
            reason: WorkspaceSkippedReason::MissingScopes,
            classification: "skipped",
            operator_message: "The available Workspace credentials do not cover the bounded requested capability scope.",
        },
        WorkspaceSkippedStateRecord {
            reason: WorkspaceSkippedReason::NetworkUnavailable,
            classification: "skipped",
            operator_message: "Workspace live execution is unreachable; preserve fixture proof and record the network unavailability separately.",
        },
        WorkspaceSkippedStateRecord {
            reason: WorkspaceSkippedReason::OperatorDeclined,
            classification: "skipped",
            operator_message: "The operator declined a live write or wider scope escalation; stop cleanly without treating that as a bridge defect.",
        },
    ]
}

fn stop_conditions() -> Vec<WorkspaceStopConditionRecord> {
    vec![
        WorkspaceStopConditionRecord {
            condition: WorkspaceStopCondition::RevisionMismatch,
            classification: "stop",
            operator_message: "The recorded Workspace revision anchor no longer matches the reviewed source state; stop before preview, apply, or promotion work continues.",
        },
        WorkspaceStopConditionRecord {
            condition: WorkspaceStopCondition::AuthorityDrift,
            classification: "stop",
            operator_message: "The requested live Workspace action exceeds the declared issue-scoped authority or bounded folder/doc/sheet scope.",
        },
    ]
}

fn operator_runbook() -> Vec<WorkspaceRunbookRecord> {
    vec![
        WorkspaceRunbookRecord {
            stage: "preflight",
            required_actions: vec![
                "Confirm the project really needs live Workspace access rather than fixture proof.",
                "Declare one explicit folder/doc/sheet scope before proposing any live capability call.",
                "Record the intended auth mode and verify that secrets themselves will not enter repo artifacts or cards.",
            ],
        },
        WorkspaceRunbookRecord {
            stage: "live read path",
            required_actions: vec![
                "Use inventory and snapshot reads first, with metadata-only or redacted trace posture.",
                "Treat private document bodies as withheld by default unless the operator explicitly approves a redacted excerpt.",
                "If auth, scope, or `gws` availability is missing, classify the live path as skipped and keep fixture proof separate.",
            ],
        },
        WorkspaceRunbookRecord {
            stage: "live write path",
            required_actions: vec![
                "Require preview before any content-card mutation apply step.",
                "Require explicit operator approval for bounded sheet-range writes.",
                "Record revision anchors and issue/PR linkage before any promotion-oriented action.",
            ],
        },
        WorkspaceRunbookRecord {
            stage: "promotion boundary",
            required_actions: vec![
                "Prepare promotion packets as metadata-rich handoff artifacts, not as direct tracked-file edits.",
                "Keep GitHub issue and PR flow as canonical authority after promotion.",
                "Stop before any silent repo mutation from Workspace state.",
            ],
        },
    ]
}

pub fn run_gws_live_safety_package_report() -> GwsLiveSafetyPackageReport {
    GwsLiveSafetyPackageReport {
        schema_version: GWS_LIVE_SAFETY_PACKAGE_SCHEMA_VERSION,
        prompt_record: prompt_record(),
        auth_profiles: auth_profiles(),
        scope_policies: scope_policies(),
        redaction_rules: redaction_rules(),
        skipped_states: skipped_states(),
        stop_conditions: stop_conditions(),
        operator_runbook: operator_runbook(),
        non_claims: vec![
            "This package does not enable broad ambient Google Workspace authority.",
            "This package does not make Google Workspace canonical repo truth.",
            "This package does not authorize private document body export into public artifacts by default.",
        ],
    }
}

pub fn write_gws_live_safety_package_report(
    path: impl AsRef<Path>,
) -> Result<GwsLiveSafetyPackageReport> {
    let report = run_gws_live_safety_package_report();
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create parent directories for '{}'", path.display()))?;
    }
    let body = serde_json::to_string_pretty(&report)?;
    fs::write(path, body).with_context(|| format!("write '{}'", path.display()))?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::{
        run_gws_live_safety_package_report, write_gws_live_safety_package_report,
        WorkspaceAuthMode, WorkspaceSkippedReason, WorkspaceStopCondition,
        GWS_LIVE_SAFETY_PACKAGE_REPORT_ARTIFACT_PATH,
    };
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_path(prefix: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nanos}.json"))
    }

    #[test]
    fn gws_live_safety_package_covers_required_auth_scope_and_skipped_state_contracts() {
        let report = run_gws_live_safety_package_report();
        assert!(report
            .auth_profiles
            .iter()
            .any(|profile| profile.auth_mode == WorkspaceAuthMode::OperatorOauthUser));
        assert!(report
            .scope_policies
            .iter()
            .any(|scope| scope.write_capability && scope.operator_approval_required));
        assert!(report
            .skipped_states
            .iter()
            .any(|state| state.reason == WorkspaceSkippedReason::MissingAuth));
        assert!(report
            .stop_conditions
            .iter()
            .any(|condition| condition.condition == WorkspaceStopCondition::RevisionMismatch));
    }

    #[test]
    fn gws_live_safety_package_serializes_deterministically_without_host_paths() {
        let first = serde_json::to_string_pretty(&run_gws_live_safety_package_report())
            .expect("serialize first");
        let second = serde_json::to_string_pretty(&run_gws_live_safety_package_report())
            .expect("serialize second");
        assert_eq!(first, second);
        assert!(!first.contains(super::HOST_PATH_MARKER));
    }

    #[test]
    fn gws_live_safety_package_report_writer_emits_portable_json() {
        let report_path = unique_temp_path("gws-live-safety-package");
        let report = write_gws_live_safety_package_report(&report_path).expect("write report");
        let body = fs::read_to_string(&report_path).expect("read report");
        assert!(body.contains("gws_live_safety_package.v1"));
        assert!(body.contains("live_mode_disabled"));
        assert_eq!(report.schema_version, "gws_live_safety_package.v1");
        fs::remove_file(&report_path).expect("remove report");
    }

    #[test]
    fn gws_live_safety_package_artifact_path_is_repo_relative_and_bounded() {
        assert!(GWS_LIVE_SAFETY_PACKAGE_REPORT_ARTIFACT_PATH.starts_with("docs/"));
        assert!(!GWS_LIVE_SAFETY_PACKAGE_REPORT_ARTIFACT_PATH.starts_with('/'));
    }

    #[test]
    fn gws_live_safety_package_writer_adds_context_on_failure() {
        let dir = std::env::temp_dir().join(format!(
            "gws-live-safety-package-dir-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be valid")
                .as_nanos()
        ));
        fs::create_dir_all(&dir).expect("create temp dir");

        let error =
            write_gws_live_safety_package_report(&dir).expect_err("directory path should fail");
        let message = error.to_string();
        let context = error
            .source()
            .map(|source| source.to_string())
            .unwrap_or_default();
        assert!(message.contains(&dir.display().to_string()) || context.contains("Is a directory"));

        fs::remove_dir_all(&dir).expect("remove temp dir");
    }
}
