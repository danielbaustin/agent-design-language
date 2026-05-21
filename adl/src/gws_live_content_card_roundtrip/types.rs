use crate::gws_live_capability_execution_surface::{
    GwsCapabilityResult, GwsLiveDocRecord, GwsLiveMode, GwsLiveScopeBinding,
};
use crate::rust_native_gws_adapter_boundary::{
    WorkspaceContentCardUpdateApplyResult, WorkspaceContentCardUpdatePreview,
};
use serde::Serialize;

pub const GWS_LIVE_CONTENT_CARD_ROUNDTRIP_REPORT_ARTIFACT_PATH: &str =
    "docs/milestones/v0.91.2/review/google_workspace_cms_bridge/gws_live_content_card_roundtrip_report.json";
pub const GWS_LIVE_CONTENT_CARD_ROUNDTRIP_SCHEMA_VERSION: &str =
    "gws_live_content_card_roundtrip.v1";
pub const GWS_LIVE_CONTENT_CARD_ROUNDTRIP_PROMPT_VERSION: &str =
    "wp3093.gws_live_content_card_roundtrip.v1";
pub const GWS_WRITE_APPROVAL_ENV: &str = "ADL_GWS_WRITE_APPROVAL";
#[cfg(test)]
pub const HOST_PATH_MARKER: &str = "/Users/daniel/";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsLiveRoundtripPromptRecord {
    pub prompt_version: &'static str,
    pub issue_number: u32,
    pub depends_on_issue_number: u32,
    pub summary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GwsRoundtripSkipReason {
    LiveModeDisabled,
    DryRunOnly,
    GwsUnavailable,
    MissingScopeBinding,
    MissingAuth,
    MissingScopes,
    MissingWriteApproval,
    RevisionMismatch,
    TargetContentCardMissing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GwsRevisionCheckStatus {
    Skipped,
    Matched,
    Mismatched,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsLiveMutationCommandTraceRecord {
    pub capability_name: &'static str,
    pub argv: Vec<String>,
    pub mode: GwsLiveMode,
    pub result: GwsCapabilityResult,
    pub skipped_reason: Option<GwsRoundtripSkipReason>,
    pub exit_code: Option<i32>,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsRevisionAnchorRecord {
    pub expected_revision_anchor: String,
    pub live_revision_anchor: Option<String>,
    pub check_status: GwsRevisionCheckStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsLiveContentCardSheetRecord {
    pub spreadsheet_id: String,
    pub range: String,
    pub row_count: usize,
    pub header_row: Vec<String>,
    pub values: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsLiveApplyOutcomeRecord {
    pub result: GwsCapabilityResult,
    pub skipped_reason: Option<GwsRoundtripSkipReason>,
    pub update_range: String,
    pub preview: WorkspaceContentCardUpdatePreview,
    pub apply_result: WorkspaceContentCardUpdateApplyResult,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsLiveWriteApprovalRecord {
    pub approval_required: bool,
    pub approval_checked: bool,
    pub approval_present: bool,
    pub approval_env_var: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsPromotionPacketHandoffRecord {
    pub doc_id: String,
    pub title: String,
    pub target_repo_path: String,
    pub workspace_revision_anchor: String,
    pub issue_route: String,
    pub pr_route: String,
    pub canonical_authority: &'static str,
    pub stop_boundary: &'static str,
    pub tracked_packet_consistent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GwsLiveContentCardRoundtripReport {
    pub schema_version: &'static str,
    pub prompt_record: GwsLiveRoundtripPromptRecord,
    pub live_mode: GwsLiveMode,
    pub live_scope_binding: Option<GwsLiveScopeBinding>,
    pub write_approval: GwsLiveWriteApprovalRecord,
    pub expected_content_card_doc_id: String,
    pub content_card_sheet_preview: Option<GwsLiveContentCardSheetRecord>,
    pub revision_anchor: GwsRevisionAnchorRecord,
    pub live_doc_snapshot: Option<GwsLiveDocRecord>,
    pub apply_outcome: GwsLiveApplyOutcomeRecord,
    pub promotion_packet_handoff: GwsPromotionPacketHandoffRecord,
    pub command_traces: Vec<GwsLiveMutationCommandTraceRecord>,
    pub roundtrip_result: GwsCapabilityResult,
    pub roundtrip_skipped_reason: Option<GwsRoundtripSkipReason>,
    pub non_claims: Vec<&'static str>,
}
