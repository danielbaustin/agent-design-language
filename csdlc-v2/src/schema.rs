use serde_json::{json, Value};

use crate::doctor::DoctorReport;
use crate::lifecycle::{BindRequest, BindResult, HeartbeatRequest, RecoverClaimRequest};
use crate::migration::{ImportReport, LegacyImportRequest, NormalizedOutcome, ShadowComparison};
use crate::model::IssueRecord;
use crate::publication::{PublicationIntent, PublicationRequest, RemotePullRequest};
use crate::pvf::{ExecutionReport, ExecutionRequest, PvfManifest, ScheduleReport, ShepherdReport};
use crate::readiness::{ReadinessReport, ReadinessRequest, TerminalObservation};
use crate::review::{
    PublicationReviewReport, ReviewAssignmentRequest, ReviewRecordRequest, ReviewRecoveryRequest,
};
use crate::store::ApproveDesignRequest;
use crate::store::{BootstrapRequest, EditRequest};

pub fn public_schema_bundle() -> Value {
    json!({
        "schema": "csdlc.public_schema_bundle.v1",
        "bootstrap_request": schemars::schema_for!(BootstrapRequest),
        "approve_design_request": schemars::schema_for!(ApproveDesignRequest),
        "edit_request": schemars::schema_for!(EditRequest),
        "bind_request": schemars::schema_for!(BindRequest),
        "bind_result": schemars::schema_for!(BindResult),
        "recover_claim_request": schemars::schema_for!(RecoverClaimRequest),
        "heartbeat_request": schemars::schema_for!(HeartbeatRequest),
        "issue_record": schemars::schema_for!(IssueRecord),
        "doctor_report": schemars::schema_for!(DoctorReport),
        "pvf_manifest": schemars::schema_for!(PvfManifest),
        "pvf_execution_request": schemars::schema_for!(ExecutionRequest),
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
        "terminal_observation": schemars::schema_for!(TerminalObservation),
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
        assert!(public_schema_bundle().get("heartbeat_request").is_some());
    }
}
