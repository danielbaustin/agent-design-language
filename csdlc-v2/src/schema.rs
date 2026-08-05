use serde_json::{json, Value};

use crate::cleanup::{
    CleanupRequest, CleanupResult, LegacyTerminalIndex, LegacyTerminalIndexRequest,
    TerminalCensusReport,
};
use crate::doctor::DoctorReport;
use crate::finish::{
    DerivedTerminalEnvelope, FinishRequest, FinishResult, IssueTerminalObservation,
};
use crate::github::{GithubActionRequest, GithubActionResult, GithubIssuePacket, PrStatePacket};
use crate::lifecycle::{
    AmendClaimScopeRequest, HeartbeatRequest, RecoverClaimRequest, ReleaseClosedClaimRequest,
    RevokeActiveClaimRequest, RevokeActiveClaimResult, TransitionActiveClaimRequest,
};
use crate::migration::{ImportReport, LegacyImportRequest, NormalizedOutcome, ShadowComparison};
use crate::model::IssueRecord;
use crate::model::TerminalReceipt;
use crate::preparation::{
    BindReleaseRequest, BindReleaseResult, BindingIntent, DerivedBindRequest, DerivedBindResult,
    ExecutionReadinessReceipt, IssueCreateRequest, IssueDraft, LegacyPreparationMigrationRequest,
    LegacyPreparationMigrationResult, LegacyPreparationRepairRequest,
    LegacyPreparationRepairResult, PreparationManifest, PrepareBatchRequest, PrepareBatchResult,
    PrepareRunRequest, PrepareRunResult, PrepareSealRequest, PrepareSyncRequest,
    PreparedGeneration,
};
use crate::publication::{PublicationIntent, PublicationRequest, RemotePullRequest};
use crate::pvf::{
    ExecutionReport, ExecutionRequest, FinalizeRequest, PvfManifest, ScheduleReport, ShepherdReport,
};
use crate::readiness::{ReadinessReport, ReadinessRequest};
use crate::review::{
    PublicationReviewReport, ReviewAssignmentRequest, ReviewRecordRequest, ReviewRecoveryRequest,
};
use crate::store::ApproveDesignRequest;
use crate::store::EditRequest;

pub fn public_schema_bundle() -> Value {
    json!({
        "schema": "csdlc.public_schema_bundle.v1",
        "cleanup_request": schemars::schema_for!(CleanupRequest),
        "cleanup_result": schemars::schema_for!(CleanupResult),
        "legacy_terminal_index_request": schemars::schema_for!(LegacyTerminalIndexRequest),
        "legacy_terminal_index": schemars::schema_for!(LegacyTerminalIndex),
        "terminal_census_report": schemars::schema_for!(TerminalCensusReport),
        "approve_design_request": schemars::schema_for!(ApproveDesignRequest),
        "edit_request": schemars::schema_for!(EditRequest),
        "issue_create_request": schemars::schema_for!(IssueCreateRequest),
        "issue_draft": schemars::schema_for!(IssueDraft),
        "prepare_sync_request": schemars::schema_for!(PrepareSyncRequest),
        "prepare_seal_request": schemars::schema_for!(PrepareSealRequest),
        "prepare_run_request": schemars::schema_for!(PrepareRunRequest),
        "prepare_run_result": schemars::schema_for!(PrepareRunResult),
        "prepare_batch_request": schemars::schema_for!(PrepareBatchRequest),
        "prepare_batch_result": schemars::schema_for!(PrepareBatchResult),
        "prepared_generation": schemars::schema_for!(PreparedGeneration),
        "preparation_manifest": schemars::schema_for!(PreparationManifest),
        "legacy_preparation_migration_request": schemars::schema_for!(LegacyPreparationMigrationRequest),
        "legacy_preparation_migration_result": schemars::schema_for!(LegacyPreparationMigrationResult),
        "legacy_preparation_repair_request": schemars::schema_for!(LegacyPreparationRepairRequest),
        "legacy_preparation_repair_result": schemars::schema_for!(LegacyPreparationRepairResult),
        "execution_readiness_receipt": schemars::schema_for!(ExecutionReadinessReceipt),
        "derived_bind_request": schemars::schema_for!(DerivedBindRequest),
        "derived_bind_result": schemars::schema_for!(DerivedBindResult),
        "binding_intent": schemars::schema_for!(BindingIntent),
        "bind_release_request": schemars::schema_for!(BindReleaseRequest),
        "bind_release_result": schemars::schema_for!(BindReleaseResult),
        "recover_claim_request": schemars::schema_for!(RecoverClaimRequest),
        "release_closed_claim_request": schemars::schema_for!(ReleaseClosedClaimRequest),
        "revoke_active_claim_request": schemars::schema_for!(RevokeActiveClaimRequest),
        "revoke_active_claim_result": schemars::schema_for!(RevokeActiveClaimResult),
        "amend_claim_scope_request": schemars::schema_for!(AmendClaimScopeRequest),
        "transition_active_claim_request": schemars::schema_for!(TransitionActiveClaimRequest),
        "heartbeat_request": schemars::schema_for!(HeartbeatRequest),
        "issue_record": schemars::schema_for!(IssueRecord),
        "terminal_receipt": schemars::schema_for!(TerminalReceipt),
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
        "publication_intent": schemars::schema_for!(PublicationIntent),
        "remote_pull_request": schemars::schema_for!(RemotePullRequest),
        "readiness_request": schemars::schema_for!(ReadinessRequest),
        "readiness_report": schemars::schema_for!(ReadinessReport),
        "finish_request": schemars::schema_for!(FinishRequest),
        "finish_result": schemars::schema_for!(FinishResult),
        "derived_terminal_envelope": schemars::schema_for!(DerivedTerminalEnvelope),
        "issue_terminal_observation": schemars::schema_for!(IssueTerminalObservation),
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
        assert!(bundle.get("terminal_receipt").is_some());
        assert!(bundle.get("terminal_plan_step_repair_request").is_none());
        assert!(bundle.get("terminal_sor_artifact_repair_request").is_none());
        assert!(bundle
            .get("corrupt_historical_merged_recovery_request")
            .is_none());
        assert!(bundle.get("transition_active_claim_request").is_some());
    }
}
