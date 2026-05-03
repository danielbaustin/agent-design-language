use serde::{Deserialize, Serialize};

pub(crate) const CODE_REVIEW_PACKET_SCHEMA: &str = "adl.pr_review_packet.v1";
pub(crate) const CODE_REVIEW_RESULT_SCHEMA: &str = "adl.pr_review_result.v1";
pub(crate) const CODE_REVIEW_GATE_SCHEMA: &str = "adl.pr_review_gate.v1";
pub(crate) const CODE_REVIEW_SUMMARY_SCHEMA: &str = "adl.pr_review_run_summary.v1";
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const PACKET_SCHEMA: &str = CODE_REVIEW_PACKET_SCHEMA;
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const DEFAULT_REVIEW_EXCERPT_BYTES: usize = 12_000;
pub(crate) const MAX_REVIEW_EXCERPT_BYTES: usize = 100_000;
pub(crate) const MAX_REVIEW_DIFF_FILES: usize = 40;
pub(crate) const MAX_REVIEW_CONTEXT_FILES: usize = 24;

#[derive(Debug)]
pub(crate) struct CodeReviewArgs {
    pub(crate) out: std::path::PathBuf,
    pub(crate) backend: ReviewerBackend,
    pub(crate) visibility_mode: VisibilityMode,
    pub(crate) base_ref: String,
    pub(crate) head_ref: String,
    pub(crate) issue_number: Option<u32>,
    pub(crate) writer_session: String,
    pub(crate) reviewer_session: Option<String>,
    pub(crate) model: Option<String>,
    pub(crate) allow_live_ollama: bool,
    pub(crate) ollama_url: String,
    pub(crate) timeout_secs: u64,
    pub(crate) include_working_tree: bool,
    pub(crate) fixture_case: FixtureCase,
    pub(crate) max_diff_bytes: usize,
    pub(crate) include_files: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ReviewerBackend {
    Fixture,
    Ollama,
}

impl ReviewerBackend {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Fixture => "fixture",
            Self::Ollama => "ollama",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum VisibilityMode {
    PacketOnly,
    ReadOnlyRepo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FixtureCase {
    Clean,
    Blocked,
}

#[derive(Debug, Serialize)]
pub(crate) struct ReviewPacket {
    pub(crate) schema_version: &'static str,
    pub(crate) issue_number: Option<u32>,
    pub(crate) branch: String,
    pub(crate) base_ref: String,
    pub(crate) head_ref: String,
    pub(crate) visibility_mode: VisibilityMode,
    pub(crate) changed_files: Vec<String>,
    pub(crate) diff_summary: DiffSummary,
    pub(crate) focused_diff_hunks: Vec<DiffHunk>,
    pub(crate) file_contexts: Vec<FileContext>,
    pub(crate) validation_evidence: Vec<ValidationEvidence>,
    pub(crate) static_analysis_evidence: Vec<ValidationEvidence>,
    pub(crate) repo_slice_manifest: RepoSliceManifest,
    pub(crate) review_scope: String,
    pub(crate) non_scope: Vec<String>,
    pub(crate) known_risks: Vec<String>,
    pub(crate) redaction_status: RedactionStatus,
}

#[derive(Debug, Serialize)]
pub(crate) struct DiffSummary {
    pub(crate) files_changed: usize,
    pub(crate) max_diff_bytes: usize,
    pub(crate) max_diff_files: usize,
    pub(crate) max_context_files: usize,
    pub(crate) file_limit_truncated: bool,
    pub(crate) truncated_hunks: bool,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DiffHunk {
    pub(crate) file: String,
    pub(crate) diff_excerpt: String,
    pub(crate) truncated: bool,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct FileContext {
    pub(crate) file: String,
    pub(crate) current_excerpt: String,
    pub(crate) truncated: bool,
    pub(crate) read_error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ValidationEvidence {
    pub(crate) command: String,
    pub(crate) status: String,
    pub(crate) summary: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct RepoSliceManifest {
    pub(crate) read_only: bool,
    pub(crate) write_allowed: bool,
    pub(crate) tool_execution_allowed: bool,
    pub(crate) files: Vec<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct RedactionStatus {
    pub(crate) absolute_host_paths_present: bool,
    pub(crate) secret_like_values_present: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ReviewResult {
    pub(crate) schema_version: String,
    pub(crate) review_id: String,
    pub(crate) reviewer_backend: String,
    pub(crate) reviewer_model: String,
    pub(crate) reviewer_session: String,
    pub(crate) writer_session: String,
    pub(crate) same_session_as_writer: bool,
    pub(crate) visibility_mode: VisibilityMode,
    pub(crate) repo_access: RepoAccess,
    pub(crate) packet_id: String,
    pub(crate) static_analysis_summary: Vec<String>,
    pub(crate) findings: Vec<ReviewFinding>,
    pub(crate) disposition: ReviewDisposition,
    pub(crate) residual_risk: Vec<String>,
    pub(crate) validation_claims: Vec<String>,
    pub(crate) non_claims: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RepoAccess {
    pub(crate) read_only: bool,
    pub(crate) write_allowed: bool,
    pub(crate) tool_execution_allowed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ReviewFinding {
    pub(crate) title: String,
    pub(crate) priority: String,
    pub(crate) file: String,
    pub(crate) line: Option<u32>,
    pub(crate) body: String,
    pub(crate) evidence: Vec<String>,
    pub(crate) heuristic_ids: Vec<String>,
    pub(crate) confidence: String,
    pub(crate) blocking: bool,
    pub(crate) suggested_fix_scope: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ReviewDisposition {
    Blessed,
    Blocked,
    NonProving,
    Skipped,
}

#[derive(Debug, Serialize)]
pub(crate) struct GateResult {
    pub(crate) schema_version: &'static str,
    pub(crate) gate_disposition: String,
    pub(crate) pr_open_allowed: bool,
    pub(crate) reasons: Vec<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct RunSummary {
    pub(crate) schema_version: &'static str,
    pub(crate) packet_path: String,
    pub(crate) result_path: String,
    pub(crate) gate_path: String,
    pub(crate) backend: String,
    pub(crate) visibility_mode: VisibilityMode,
    pub(crate) pr_open_allowed: bool,
}
