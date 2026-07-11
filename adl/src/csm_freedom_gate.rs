//! CSM Freedom Gate runtime component projection.
//!
//! The runtime owns the component contract in `adl-runtime`; this module adapts
//! the existing Freedom Gate evaluator into retained CSM daemon/API evidence.

use anyhow::{Context, Result};
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

use crate::freedom_gate::{
    evaluate_tool_candidate_freedom_gate_v1, FreedomGateToolCandidateV1,
    FreedomGateToolDecisionEventV1, FreedomGateToolGateContextV1,
};

pub use adl_runtime::freedom_gate::{
    CSM_FREEDOM_GATE_COMPONENT, CSM_FREEDOM_GATE_DECISION_SCHEMA, CSM_FREEDOM_GATE_STATUS_REF,
    CSM_FREEDOM_GATE_STATUS_SCHEMA,
};

pub fn runtime_capability() -> Value {
    let status = adl_runtime::freedom_gate::default_freedom_gate_status();
    json!({
        "status": status.status,
        "component": CSM_FREEDOM_GATE_COMPONENT,
        "component_class": "csm_policy_gate",
        "process_model": status.process_model,
        "mediation_position": status.mediation_position,
        "executor_requires_gate_decision": status.executor_requires_gate_decision,
        "unmediated_execution_allowed": status.unmediated_execution_allowed,
        "decision_schema": CSM_FREEDOM_GATE_DECISION_SCHEMA,
        "retained_status_ref": CSM_FREEDOM_GATE_STATUS_REF,
        "channels": status.channels,
        "enforcement": {
            "allowed": "executor_invocation_requires_allow_decision",
            "denied": "stop_before_executor",
            "deferred": "stop_before_executor_and_request_review",
            "challenged": "stop_before_executor_and_route_challenge",
            "escalated": "stop_before_executor_and_route_escalation",
            "invalid_or_unredacted": "deny_before_executor"
        }
    })
}

pub fn build_status_snapshot(agent_instance_id: &str) -> Value {
    let status = adl_runtime::freedom_gate::default_freedom_gate_status();
    status
        .validate()
        .expect("default CSM Freedom Gate contract is valid");
    json!({
        "schema": CSM_FREEDOM_GATE_STATUS_SCHEMA,
        "runtime_owner": "csm",
        "component": CSM_FREEDOM_GATE_COMPONENT,
        "agent_instance_id": agent_instance_id,
        "status": status.status,
        "readiness": status.readiness,
        "process_model": status.process_model,
        "mediation_position": status.mediation_position,
        "executor_requires_gate_decision": status.executor_requires_gate_decision,
        "unmediated_execution_allowed": status.unmediated_execution_allowed,
        "channels": status.channels,
        "supervision_policy": status.supervision_policy,
        "retained_status_ref": status.retained_status_ref,
        "decision_proofs": decision_proofs(),
        "negative_case_policy": {
            "missing_gate_decision": "deny_before_executor",
            "invalid_gate_trace_context": "deny_before_executor",
            "private_arguments_not_redacted": "deny_before_executor",
            "policy_denied": "stop_before_executor",
            "operator_review_required": "defer_before_executor"
        },
        "retention": {
            "status_ref": CSM_FREEDOM_GATE_STATUS_REF,
            "lifelog_required": true,
            "observability_required": true,
            "decision_events_retained": true
        },
        "updated_at": Utc::now()
    })
}

pub fn write_status_snapshot(state_root: &Path, agent_instance_id: &str) -> Result<Value> {
    fs::create_dir_all(state_root).with_context(|| {
        format!(
            "create CSM Freedom Gate state root {}",
            state_root.display()
        )
    })?;
    let snapshot = build_status_snapshot(agent_instance_id);
    let path = state_root.join(CSM_FREEDOM_GATE_STATUS_REF);
    fs::write(&path, serde_json::to_vec_pretty(&snapshot)?)
        .with_context(|| format!("write CSM Freedom Gate status {}", path.display()))?;
    Ok(snapshot)
}

pub fn adl_workflow_admission_event(
    agent_instance_id: &str,
    cycle_id: &str,
    policy_decision: Option<&str>,
) -> FreedomGateToolDecisionEventV1 {
    let candidate = FreedomGateToolCandidateV1 {
        candidate_id: format!("candidate.adl-workflow.{cycle_id}"),
        proposal_id: format!("proposal.adl-workflow.{cycle_id}"),
        normalized_proposal_ref: format!("normalized.adl-workflow.{cycle_id}"),
        acc_contract_id: "acc.csm.adl-workflow.runtime".to_string(),
        policy_evidence_ref: format!("policy.freedom-gate.{agent_instance_id}"),
        action_kind: "adl_workflow_execute".to_string(),
        risk_class: "medium".to_string(),
        operator_actor_id: "actor.csm.runtime".to_string(),
        citizen_boundary_ref: format!("citizen.boundary.{agent_instance_id}"),
        private_argument_digest: format!("sha256:{}", "2".repeat(64)),
    };
    let context = FreedomGateToolGateContextV1 {
        policy_decision: policy_decision.unwrap_or("allowed").to_string(),
        ..tool_context("allowed")
    };
    evaluate_tool_candidate_freedom_gate_v1(&candidate, &context)
}

pub fn api_status(artifact: &Value, runtime_capability: Value, agent_instance_id: &str) -> Value {
    let fallback = build_status_snapshot(agent_instance_id);
    let raw_value = artifact
        .get("value")
        .cloned()
        .unwrap_or_else(|| fallback.clone());
    let validation = validate_status_value(&raw_value);
    let value = if validation.is_ok() {
        raw_value
    } else {
        fallback.clone()
    };
    let status = if validation.is_ok() {
        artifact
            .get("status")
            .cloned()
            .unwrap_or_else(|| json!("computed_fallback"))
    } else {
        json!("invalid_retained_artifact_fail_closed")
    };
    json!({
        "status": status,
        "ref": CSM_FREEDOM_GATE_STATUS_REF,
        "schema": value.get("schema").cloned().unwrap_or_else(|| json!(CSM_FREEDOM_GATE_STATUS_SCHEMA)),
        "runtime_owner": "csm",
        "component": CSM_FREEDOM_GATE_COMPONENT,
        "capability": runtime_capability,
        "readiness": value.get("readiness").cloned().unwrap_or_else(|| json!("available")),
        "mediation_position": value.get("mediation_position").cloned().unwrap_or_else(|| json!("between_scheduler_reasoning_runtime_and_aee_executor")),
        "executor_requires_gate_decision": value.get("executor_requires_gate_decision").cloned().unwrap_or_else(|| json!(true)),
        "unmediated_execution_allowed": value.get("unmediated_execution_allowed").cloned().unwrap_or_else(|| json!(false)),
        "channels": value.get("channels").cloned().unwrap_or_else(|| fallback["channels"].clone()),
        "decision_proofs": value.get("decision_proofs").cloned().unwrap_or_else(|| fallback["decision_proofs"].clone()),
        "negative_case_policy": value.get("negative_case_policy").cloned().unwrap_or_else(|| fallback["negative_case_policy"].clone()),
        "retained_artifact_validation": validation.err().map(|reason| json!({
            "status": "rejected_fail_closed",
            "reason": reason
        })).unwrap_or_else(|| json!({"status": "accepted"}))
    })
}

fn validate_status_value(value: &Value) -> std::result::Result<(), String> {
    let contract_value = json!({
        "schema": required_value(value, "schema")?,
        "runtime_owner": required_value(value, "runtime_owner")?,
        "component": required_value(value, "component")?,
        "status": required_value(value, "status")?,
        "readiness": required_value(value, "readiness")?,
        "process_model": required_value(value, "process_model")?,
        "mediation_position": required_value(value, "mediation_position")?,
        "executor_requires_gate_decision": required_value(value, "executor_requires_gate_decision")?,
        "unmediated_execution_allowed": required_value(value, "unmediated_execution_allowed")?,
        "channels": required_value(value, "channels")?,
        "supervision_policy": required_value(value, "supervision_policy")?,
        "retained_status_ref": required_value(value, "retained_status_ref")?,
    });
    let contract: adl_runtime::freedom_gate::CsmFreedomGateComponentStatus =
        serde_json::from_value(contract_value)
            .map_err(|err| format!("contract_shape_mismatch:{err}"))?;
    contract.validate()?;
    if contract.status != "integrated" {
        return Err("status_not_integrated".to_string());
    }
    if contract.process_model != "in_process_csm_runtime_component" {
        return Err("process_model_mismatch".to_string());
    }
    if contract.mediation_position != "between_scheduler_reasoning_runtime_and_aee_executor" {
        return Err("mediation_position_mismatch".to_string());
    }
    if contract.retained_status_ref != CSM_FREEDOM_GATE_STATUS_REF {
        return Err("retained_status_ref_mismatch".to_string());
    }
    validate_decision_proofs(required_value(value, "decision_proofs")?)?;
    validate_negative_case_policy(required_value(value, "negative_case_policy")?)?;
    Ok(())
}

fn required_value<'a>(value: &'a Value, name: &str) -> std::result::Result<&'a Value, String> {
    value
        .get(name)
        .ok_or_else(|| format!("required_field_missing:{name}"))
}

fn validate_decision_proofs(value: &Value) -> std::result::Result<(), String> {
    if value.get("schema").and_then(Value::as_str) != Some(CSM_FREEDOM_GATE_DECISION_SCHEMA) {
        return Err("decision_proofs_schema_mismatch".to_string());
    }
    let summary = value
        .get("proof_summary")
        .ok_or_else(|| "decision_proofs_summary_missing".to_string())?;
    for field in [
        "allowed_reaches_executor",
        "denied_stops_before_executor",
        "deferred_stops_before_executor",
        "invalid_stops_before_executor",
    ] {
        if summary.get(field).and_then(Value::as_bool) != Some(true) {
            return Err(format!("decision_proof_missing:{field}"));
        }
    }
    Ok(())
}

fn validate_negative_case_policy(value: &Value) -> std::result::Result<(), String> {
    for (field, expected) in [
        ("missing_gate_decision", "deny_before_executor"),
        ("invalid_gate_trace_context", "deny_before_executor"),
        ("private_arguments_not_redacted", "deny_before_executor"),
        ("policy_denied", "stop_before_executor"),
        ("operator_review_required", "defer_before_executor"),
    ] {
        if value.get(field).and_then(Value::as_str) != Some(expected) {
            return Err(format!("negative_case_policy_mismatch:{field}"));
        }
    }
    Ok(())
}

fn decision_proofs() -> Value {
    let candidate = tool_candidate("candidate.tool.safe-read", "low");
    let denied = evaluate_tool_candidate_freedom_gate_v1(&candidate, &tool_context("denied"));
    let deferred = evaluate_tool_candidate_freedom_gate_v1(&candidate, &tool_context("deferred"));
    let challenged =
        evaluate_tool_candidate_freedom_gate_v1(&candidate, &tool_context("challenged"));
    let allowed = evaluate_tool_candidate_freedom_gate_v1(&candidate, &tool_context("allowed"));
    let escalated = evaluate_tool_candidate_freedom_gate_v1(
        &tool_candidate("candidate.tool.high-risk", "high"),
        &FreedomGateToolGateContextV1 {
            escalation_available: true,
            ..tool_context("allowed")
        },
    );
    let invalid = evaluate_tool_candidate_freedom_gate_v1(
        &FreedomGateToolCandidateV1 {
            private_argument_digest: "secret-payload".to_string(),
            ..candidate.clone()
        },
        &tool_context("allowed"),
    );
    let allowed_reaches_executor = allowed.executor_invocation_ref.is_some();
    let denied_stops_before_executor = denied.stopped_before_executor;
    let deferred_stops_before_executor = deferred.stopped_before_executor;
    let challenged_stops_before_executor = challenged.stopped_before_executor;
    let escalated_stops_before_executor = escalated.stopped_before_executor;
    let invalid_stops_before_executor = invalid.stopped_before_executor;
    json!({
        "schema": CSM_FREEDOM_GATE_DECISION_SCHEMA,
        "allowed": allowed,
        "denied": denied,
        "deferred": deferred,
        "challenged": challenged,
        "escalated": escalated,
        "invalid_or_unredacted": invalid,
        "proof_summary": {
            "allowed_reaches_executor": allowed_reaches_executor,
            "denied_stops_before_executor": denied_stops_before_executor,
            "deferred_stops_before_executor": deferred_stops_before_executor,
            "challenged_stops_before_executor": challenged_stops_before_executor,
            "escalated_stops_before_executor": escalated_stops_before_executor,
            "invalid_stops_before_executor": invalid_stops_before_executor
        }
    })
}

fn tool_candidate(candidate_id: &str, risk_class: &str) -> FreedomGateToolCandidateV1 {
    FreedomGateToolCandidateV1 {
        candidate_id: candidate_id.to_string(),
        proposal_id: "proposal.fixture.safe-read".to_string(),
        normalized_proposal_ref: "normalized.proposal.fixture.safe-read".to_string(),
        acc_contract_id: "acc.compiler.proposal.fixture.safe-read".to_string(),
        policy_evidence_ref: "policy.wp07a.freedom-gate.fixture".to_string(),
        action_kind: "fixture_read".to_string(),
        risk_class: risk_class.to_string(),
        operator_actor_id: "actor.operator.alice".to_string(),
        citizen_boundary_ref: "citizen.boundary.fixture".to_string(),
        private_argument_digest: format!("sha256:{}", "1".repeat(64)),
    }
}

fn tool_context(policy_decision: &str) -> FreedomGateToolGateContextV1 {
    FreedomGateToolGateContextV1 {
        policy_decision: policy_decision.to_string(),
        requires_operator_review: false,
        requires_human_challenge: false,
        escalation_available: false,
        citizen_action_boundary_intact: true,
        operator_action_boundary_intact: true,
        private_arguments_redacted: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csm_freedom_gate_status_fails_closed_before_executor() {
        let status = build_status_snapshot("agent-1");
        assert_eq!(status["runtime_owner"], "csm");
        assert_eq!(status["component"], "freedom_gate");
        assert_eq!(status["executor_requires_gate_decision"], true);
        assert_eq!(status["unmediated_execution_allowed"], false);
        assert_eq!(
            status["decision_proofs"]["proof_summary"]["allowed_reaches_executor"],
            true
        );
        assert_eq!(
            status["decision_proofs"]["proof_summary"]["denied_stops_before_executor"],
            true
        );
        assert_eq!(
            status["decision_proofs"]["proof_summary"]["invalid_stops_before_executor"],
            true
        );
    }

    #[test]
    fn csm_freedom_gate_runtime_capability_exposes_csm_policy_gate_contract() {
        let capability = runtime_capability();
        assert_eq!(capability["status"], "integrated");
        assert_eq!(capability["component"], CSM_FREEDOM_GATE_COMPONENT);
        assert_eq!(capability["component_class"], "csm_policy_gate");
        assert_eq!(
            capability["mediation_position"],
            "between_scheduler_reasoning_runtime_and_aee_executor"
        );
        assert_eq!(capability["executor_requires_gate_decision"], true);
        assert_eq!(capability["unmediated_execution_allowed"], false);
        assert_eq!(
            capability["enforcement"]["invalid_or_unredacted"],
            "deny_before_executor"
        );
    }

    #[test]
    fn csm_freedom_gate_status_snapshot_writes_retained_artifact() {
        let root = std::env::temp_dir().join(format!(
            "adl-csm-freedom-gate-status-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);

        let snapshot =
            write_status_snapshot(&root, "agent-1").expect("write freedom gate status snapshot");
        let path = root.join(CSM_FREEDOM_GATE_STATUS_REF);
        let retained: Value = serde_json::from_slice(
            &fs::read(&path).expect("retained freedom gate status should be written"),
        )
        .expect("retained freedom gate status should be json");

        assert_eq!(snapshot["component"], CSM_FREEDOM_GATE_COMPONENT);
        assert_eq!(retained["component"], CSM_FREEDOM_GATE_COMPONENT);
        assert_eq!(retained["retained_status_ref"], CSM_FREEDOM_GATE_STATUS_REF);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn csm_freedom_gate_api_prefers_retained_artifact() {
        let retained = json!({
            "status": "serialized",
            "value": build_status_snapshot("agent-1")
        });
        let api = api_status(&retained, runtime_capability(), "agent-1");
        assert_eq!(api["status"], "serialized");
        assert_eq!(api["schema"], CSM_FREEDOM_GATE_STATUS_SCHEMA);
        assert_eq!(
            api["decision_proofs"]["proof_summary"]["denied_stops_before_executor"],
            true
        );
    }

    #[test]
    fn csm_freedom_gate_api_rejects_unsafe_retained_artifact_fail_closed() {
        let retained = json!({
            "status": "serialized",
            "value": {
                "schema": CSM_FREEDOM_GATE_STATUS_SCHEMA,
                "runtime_owner": "csm",
                "component": "freedom_gate",
                "executor_requires_gate_decision": false,
                "unmediated_execution_allowed": true
            }
        });
        let api = api_status(&retained, runtime_capability(), "agent-1");
        assert_eq!(api["status"], "invalid_retained_artifact_fail_closed");
        assert_eq!(api["executor_requires_gate_decision"], true);
        assert_eq!(api["unmediated_execution_allowed"], false);
        assert_eq!(
            api["retained_artifact_validation"]["status"],
            "rejected_fail_closed"
        );
    }

    #[test]
    fn csm_freedom_gate_api_rejects_partial_retained_artifact_fail_closed() {
        let retained = json!({
            "status": "serialized",
            "value": {
                "schema": CSM_FREEDOM_GATE_STATUS_SCHEMA,
                "runtime_owner": "csm",
                "component": "freedom_gate",
                "status": "integrated",
                "executor_requires_gate_decision": true,
                "unmediated_execution_allowed": false
            }
        });
        let api = api_status(&retained, runtime_capability(), "agent-1");
        assert_eq!(api["status"], "invalid_retained_artifact_fail_closed");
        assert_eq!(
            api["retained_artifact_validation"]["status"],
            "rejected_fail_closed"
        );
        assert_eq!(
            api["retained_artifact_validation"]["reason"],
            "required_field_missing:readiness"
        );
    }

    #[test]
    fn csm_freedom_gate_api_rejects_contract_drift_fail_closed() {
        for (field, value, reason) in [
            (
                "schema",
                json!("adl.csm.freedom_gate.status.v0"),
                "freedom_gate status schema mismatch",
            ),
            (
                "runtime_owner",
                json!("control_plane"),
                "freedom_gate must be owned by csm runtime",
            ),
            (
                "component",
                json!("not_freedom_gate"),
                "freedom_gate component id mismatch",
            ),
            ("status", json!("degraded"), "status_not_integrated"),
            (
                "executor_requires_gate_decision",
                json!(false),
                "freedom_gate must fail closed before executor invocation",
            ),
            (
                "unmediated_execution_allowed",
                json!(true),
                "freedom_gate must fail closed before executor invocation",
            ),
            (
                "retained_status_ref",
                json!("other.json"),
                "retained_status_ref_mismatch",
            ),
        ] {
            let mut snapshot = build_status_snapshot("agent-1");
            snapshot[field] = value;
            let retained = json!({
                "status": "serialized",
                "value": snapshot
            });
            let api = api_status(&retained, runtime_capability(), "agent-1");
            assert_eq!(api["status"], "invalid_retained_artifact_fail_closed");
            assert_eq!(
                api["retained_artifact_validation"]["status"],
                "rejected_fail_closed"
            );
            assert_eq!(api["retained_artifact_validation"]["reason"], reason);
            assert_eq!(api["component"], CSM_FREEDOM_GATE_COMPONENT);
        }
    }

    #[test]
    fn csm_freedom_gate_api_rejects_decision_proof_and_policy_drift() {
        let mut decision_drift = build_status_snapshot("agent-1");
        decision_drift["decision_proofs"]["schema"] = json!("wrong.schema");
        let decision_api = api_status(
            &json!({"status": "serialized", "value": decision_drift}),
            runtime_capability(),
            "agent-1",
        );
        assert_eq!(
            decision_api["status"],
            "invalid_retained_artifact_fail_closed"
        );
        assert_eq!(
            decision_api["retained_artifact_validation"]["reason"],
            "decision_proofs_schema_mismatch"
        );

        let mut policy_drift = build_status_snapshot("agent-1");
        policy_drift["negative_case_policy"]["policy_denied"] = json!("allow_executor");
        let policy_api = api_status(
            &json!({"status": "serialized", "value": policy_drift}),
            runtime_capability(),
            "agent-1",
        );
        assert_eq!(
            policy_api["status"],
            "invalid_retained_artifact_fail_closed"
        );
        assert_eq!(
            policy_api["retained_artifact_validation"]["reason"],
            "negative_case_policy_mismatch:policy_denied"
        );
    }

    #[test]
    fn csm_freedom_gate_admission_denies_policy_block_before_executor() {
        let event = adl_workflow_admission_event("agent-1", "cycle-000001", Some("denied"));
        assert!(event.stopped_before_executor);
        assert_eq!(event.executor_invocation_ref, None);
        assert_eq!(event.reason_code, "policy_denied");
    }

    #[test]
    fn csm_freedom_gate_admission_defaults_to_allowed_policy() {
        let event = adl_workflow_admission_event("agent-1", "cycle-000001", None);
        assert!(!event.stopped_before_executor);
        assert_eq!(
            event.decision,
            crate::freedom_gate::FreedomGateToolDecisionV1::Allowed
        );
        assert_eq!(
            event.executor_invocation_ref.as_deref(),
            Some("executor.candidate.adl-workflow.cycle-000001")
        );
        assert_eq!(event.reason_code, "gate_allowed");
    }
}
