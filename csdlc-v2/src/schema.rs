use serde_json::{json, Value};

use crate::doctor::DoctorReport;
use crate::github::{GithubActionRequest, GithubActionResult, GithubIssuePacket, PrStatePacket};
use crate::lifecycle::{
    AmendClaimScopeRequest, BindRequest, BindResult, HeartbeatRequest, ReacquireClaimRequest,
    ReacquireClaimResult, RecoverClaimRequest, ReleaseClosedClaimRequest, RevokeActiveClaimRequest,
    RevokeActiveClaimResult, TransitionActiveClaimRequest,
};
use crate::merge::{MergeRequest, MergeResult};
use crate::migration::{ImportReport, LegacyImportRequest, NormalizedOutcome, ShadowComparison};
use crate::model::IssueRecord;
use crate::model::{
    ReconcileTerminalRequest, TerminalDesignRepairRequest, TerminalPlanStepRepairRequest,
    TerminalReceipt, TerminalSorArtifactRepairRequest, TerminalSorValidationRepairRequest,
};
use crate::publication::{
    MergedPublicationReconciliationRequest, PublicationIntent, PublicationRequest,
    ReadyPublicationReconciliationRequest, ReadyPublicationRequest, RemotePullRequest,
};
use crate::pvf::{
    ExecutionReport, ExecutionRequest, FinalizeRequest, PvfManifest, ScheduleReport, ShepherdReport,
};
use crate::readiness::{ReadinessReport, ReadinessRequest, TerminalObservation};
use crate::review::{
    PublicationReviewReport, ReviewAssignmentRequest, ReviewRecordRequest, ReviewRecoveryRequest,
};
use crate::store::ApproveDesignRequest;
use crate::store::{BootstrapRequest, EditRequest, RepairIdentityRequest};

pub fn public_schema_bundle() -> Value {
    json!({
        "schema": "csdlc.public_schema_bundle.v1",
        "bootstrap_request": schemars::schema_for!(BootstrapRequest),
        "approve_design_request": schemars::schema_for!(ApproveDesignRequest),
        "edit_request": schemars::schema_for!(EditRequest),
        "repair_identity_request": schemars::schema_for!(RepairIdentityRequest),
        "bind_request": schemars::schema_for!(BindRequest),
        "bind_result": schemars::schema_for!(BindResult),
        "recover_claim_request": schemars::schema_for!(RecoverClaimRequest),
        "reacquire_claim_request": schemars::schema_for!(ReacquireClaimRequest),
        "reacquire_claim_result": schemars::schema_for!(ReacquireClaimResult),
        "release_closed_claim_request": schemars::schema_for!(ReleaseClosedClaimRequest),
        "revoke_active_claim_request": schemars::schema_for!(RevokeActiveClaimRequest),
        "revoke_active_claim_result": schemars::schema_for!(RevokeActiveClaimResult),
        "amend_claim_scope_request": schemars::schema_for!(AmendClaimScopeRequest),
        "transition_active_claim_request": schemars::schema_for!(TransitionActiveClaimRequest),
        "heartbeat_request": schemars::schema_for!(HeartbeatRequest),
        "issue_record": schemars::schema_for!(IssueRecord),
        "terminal_receipt": schemars::schema_for!(TerminalReceipt),
        "reconcile_terminal_request": schemars::schema_for!(ReconcileTerminalRequest),
        "terminal_design_repair_request": schemars::schema_for!(TerminalDesignRepairRequest),
        "terminal_plan_step_repair_request": schemars::schema_for!(TerminalPlanStepRepairRequest),
        "terminal_sor_artifact_repair_request": schemars::schema_for!(TerminalSorArtifactRepairRequest),
        "terminal_sor_validation_repair_request": schemars::schema_for!(TerminalSorValidationRepairRequest),
        "doctor_report": schemars::schema_for!(DoctorReport),
        "github_action_request": schemars::schema_for!(GithubActionRequest),
        "github_action_result": schemars::schema_for!(GithubActionResult),
        "github_issue_packet": schemars::schema_for!(GithubIssuePacket),
        "github_pr_state_packet": schemars::schema_for!(PrStatePacket),
        "pvf_manifest": schemars::schema_for!(PvfManifest),
        "pvf_execution_request": schemars::schema_for!(ExecutionRequest),
        "finalize_request": schemars::schema_for!(FinalizeRequest),
        "pvf_execution_report": schemars::schema_for!(ExecutionReport),
        "scheduler_report": schemars::schema_for!(ScheduleReport),
        "shepherd_report": schemars::schema_for!(ShepherdReport),
        "review_assignment_request": schemars::schema_for!(ReviewAssignmentRequest),
        "review_record_request": schemars::schema_for!(ReviewRecordRequest),
        "review_recovery_request": schemars::schema_for!(ReviewRecoveryRequest),
        "publication_review_report": schemars::schema_for!(PublicationReviewReport),
        "publication_request": schemars::schema_for!(PublicationRequest),
        "ready_publication_request": schemars::schema_for!(ReadyPublicationRequest),
        "ready_publication_reconciliation_request": schemars::schema_for!(ReadyPublicationReconciliationRequest),
        "merged_publication_reconciliation_request": schemars::schema_for!(MergedPublicationReconciliationRequest),
        "publication_intent": schemars::schema_for!(PublicationIntent),
        "remote_pull_request": schemars::schema_for!(RemotePullRequest),
        "readiness_request": schemars::schema_for!(ReadinessRequest),
        "readiness_report": schemars::schema_for!(ReadinessReport),
        "terminal_observation": schemars::schema_for!(TerminalObservation),
        "merge_request": schemars::schema_for!(MergeRequest),
        "merge_result": schemars::schema_for!(MergeResult),
        "legacy_import_request": schemars::schema_for!(LegacyImportRequest),
        "legacy_import_report": schemars::schema_for!(ImportReport),
        "normalized_outcome": schemars::schema_for!(NormalizedOutcome),
        "shadow_comparison": schemars::schema_for!(ShadowComparison),
        "deletion_eligibility": crate::eligibility::eligibility_schema_bundle(),
    })
}

#[cfg(test)]
mod tests {
    use super::public_schema_bundle;

    #[test]
    fn exposes_heartbeat_request_schema() {
        let bundle = public_schema_bundle();
        assert!(bundle.get("heartbeat_request").is_some());
        assert!(bundle.get("terminal_plan_step_repair_request").is_some());
        assert!(bundle.get("terminal_sor_artifact_repair_request").is_some());
        assert!(bundle.get("transition_active_claim_request").is_some());
    }
}
