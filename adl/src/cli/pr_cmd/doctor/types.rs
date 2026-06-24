use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct DoctorPreflightJsonPullRequest {
    pub(crate) number: u32,
    pub(crate) head_ref_name: String,
    pub(crate) state: &'static str,
    pub(crate) queue: Option<String>,
    pub(crate) url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DoctorPreflightResult {
    pub(crate) target_queue: String,
    pub(crate) target_queue_source: &'static str,
    pub(crate) open_pr_scan_status: &'static str,
    pub(crate) open_pr_count: Option<usize>,
    pub(crate) open_prs: Vec<DoctorPreflightJsonPullRequest>,
    pub(crate) status: &'static str,
    pub(crate) block_kind: &'static str,
    pub(crate) guidance: &'static str,
    pub(crate) session_ledger: DoctorSessionLedgerJson,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct DoctorSessionLedgerClaimJson {
    pub(crate) claim_id: String,
    pub(crate) session_id: String,
    pub(crate) owner: String,
    pub(crate) resource_kind: String,
    pub(crate) resource_id: String,
    pub(crate) mode: &'static str,
    pub(crate) classification: &'static str,
    pub(crate) issue: Option<u64>,
    pub(crate) branch: Option<String>,
    pub(crate) worktree_path: Option<String>,
    pub(crate) matches_issue: bool,
    pub(crate) matches_branch: bool,
    pub(crate) matches_worktree: bool,
    pub(crate) self_claim: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct DoctorSessionLedgerJson {
    pub(crate) status: &'static str,
    pub(crate) block_kind: &'static str,
    pub(crate) guidance: &'static str,
    pub(crate) ledger_path: String,
    pub(crate) current_session_id: Option<String>,
    pub(crate) relevant_claim_count: usize,
    pub(crate) relevant_claims: Vec<DoctorSessionLedgerClaimJson>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DoctorReadyResult {
    pub(crate) lifecycle_state: &'static str,
    pub(crate) worktree: Option<String>,
    pub(crate) source: String,
    pub(crate) root_stp: String,
    pub(crate) root_input: String,
    pub(crate) root_output: String,
    pub(crate) wt_stp: Option<String>,
    pub(crate) wt_input: Option<String>,
    pub(crate) wt_output: Option<String>,
    pub(crate) card_lifecycle: DoctorCardLifecycleJson,
    pub(crate) status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct DoctorCardLifecycleJson {
    pub(crate) order: Vec<&'static str>,
    pub(crate) active_stage: &'static str,
    pub(crate) next_required_stage: Option<&'static str>,
    pub(crate) pr_run_readiness: &'static str,
    pub(crate) pr_finish_readiness: &'static str,
    pub(crate) stages: Vec<DoctorCardStageJson>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct DoctorCardStageJson {
    pub(crate) stage: &'static str,
    pub(crate) path: String,
    pub(crate) state: &'static str,
    pub(crate) complete: bool,
    pub(crate) design_time_complete: bool,
    pub(crate) final_ready: bool,
    pub(crate) next_editor: Option<&'static str>,
    pub(crate) detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct DoctorJsonOutput {
    pub(crate) schema: &'static str,
    pub(crate) issue: u32,
    pub(crate) version: String,
    pub(crate) slug: String,
    pub(crate) branch: String,
    pub(crate) mode: &'static str,
    pub(crate) target_queue: String,
    pub(crate) target_queue_source: &'static str,
    pub(crate) preflight_status: &'static str,
    pub(crate) preflight_block_kind: &'static str,
    pub(crate) preflight_guidance: &'static str,
    pub(crate) open_pr_scan_status: &'static str,
    pub(crate) open_pr_count: Option<usize>,
    pub(crate) open_prs: Vec<DoctorPreflightJsonPullRequest>,
    pub(crate) session_ledger: DoctorSessionLedgerJson,
    pub(crate) lifecycle_state: Option<&'static str>,
    pub(crate) ready_status: Option<&'static str>,
    pub(crate) worktree: Option<String>,
    pub(crate) source: Option<String>,
    pub(crate) root_stp: Option<String>,
    pub(crate) root_input: Option<String>,
    pub(crate) root_output: Option<String>,
    pub(crate) wt_stp: Option<String>,
    pub(crate) wt_input: Option<String>,
    pub(crate) wt_output: Option<String>,
    pub(crate) card_lifecycle: Option<DoctorCardLifecycleJson>,
    pub(crate) doctor_status: &'static str,
}
