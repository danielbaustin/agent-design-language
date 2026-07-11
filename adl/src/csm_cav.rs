//! Continuous Adversarial Verification runtime component surfaces.

use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

pub use adl_runtime::cav::{
    CsmCavComponentStatus, CSM_CAV_CHANNELS_SCHEMA, CSM_CAV_COMPONENT, CSM_CAV_DECISION_SCHEMA,
    CSM_CAV_STATUS_REF, CSM_CAV_STATUS_SCHEMA,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CsmCavObservation {
    pub observation_id: String,
    pub source_component: String,
    pub observation_kind: String,
    pub evidence_ref: Option<String>,
    pub severity: String,
    pub policy_context_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CsmCavDecision {
    pub schema: String,
    pub component: String,
    pub observation_id: String,
    pub decision: String,
    pub risk_level: String,
    pub reason_code: String,
    pub readiness_effect: String,
    pub retained_evidence_ref: Option<String>,
}

pub fn runtime_capability() -> Value {
    let component = CsmCavComponentStatus::default();
    json!({
        "status": component.status,
        "component": component.component,
        "component_class": "csm_security_verification_component",
        "process_model": component.process_model,
        "risk_posture": component.risk_posture,
        "decision_schema": CSM_CAV_DECISION_SCHEMA,
        "channels_schema": CSM_CAV_CHANNELS_SCHEMA,
        "channels": component.channels,
        "fail_closed": {
            "missing_evidence": component.fail_closed_on_missing_evidence,
            "policy_conflict": component.fail_closed_on_policy_conflict,
            "malformed_observation": true
        },
        "declared_coordination_channels": {
            "freedom_gate": "security_decision_channel_declared_for_admission_integration",
            "constructability_gate": "constructability_decision_channel_declared_for_follow_on_integration",
            "curiosity_engine": "probe_observation_channel_declared_for_follow_on_integration",
            "acip": "security_notice_channel_declared_for_follow_on_integration"
        },
        "retained_status_ref": CSM_CAV_STATUS_REF,
        "redaction": {
            "secret_material": "not_returned",
            "cloud_account_identifiers": "not_returned",
            "host_private_paths": "not_returned"
        }
    })
}

pub fn evaluate_observation(
    observation: &CsmCavObservation,
    policy_conflict: bool,
) -> CsmCavDecision {
    let (decision, risk_level, reason_code, readiness_effect) =
        if observation.observation_id.trim().is_empty()
            || observation.source_component.trim().is_empty()
            || observation.observation_kind.trim().is_empty()
        {
            (
                "blocked",
                "critical",
                "malformed_observation",
                "security_blocked",
            )
        } else if observation
            .evidence_ref
            .as_deref()
            .is_none_or(str::is_empty)
        {
            ("blocked", "high", "missing_evidence", "security_blocked")
        } else if policy_conflict {
            ("blocked", "high", "policy_conflict", "security_blocked")
        } else if matches!(observation.severity.as_str(), "critical" | "high") {
            (
                "degraded",
                observation.severity.as_str(),
                "adversarial_risk_observed",
                "security_degraded",
            )
        } else {
            ("accepted", "low", "evidence_present_policy_clear", "ready")
        };
    CsmCavDecision {
        schema: CSM_CAV_DECISION_SCHEMA.to_string(),
        component: CSM_CAV_COMPONENT.to_string(),
        observation_id: observation.observation_id.clone(),
        decision: decision.to_string(),
        risk_level: risk_level.to_string(),
        reason_code: reason_code.to_string(),
        readiness_effect: readiness_effect.to_string(),
        retained_evidence_ref: observation.evidence_ref.clone(),
    }
}

pub fn build_status_snapshot(agent_instance_id: &str) -> Value {
    let component = CsmCavComponentStatus::default();
    let safe_observation = CsmCavObservation {
        observation_id: "cav-bootstrap-safe-observation".to_string(),
        source_component: "runtime_api".to_string(),
        observation_kind: "component_health".to_string(),
        evidence_ref: Some(CSM_CAV_STATUS_REF.to_string()),
        severity: "info".to_string(),
        policy_context_ref: Some("runtime_policy.cav.v1".to_string()),
    };
    let missing_evidence = CsmCavObservation {
        observation_id: "cav-bootstrap-missing-evidence".to_string(),
        evidence_ref: None,
        ..safe_observation.clone()
    };
    json!({
        "schema": CSM_CAV_STATUS_SCHEMA,
        "runtime_owner": "csm",
        "component": CSM_CAV_COMPONENT,
        "agent_instance_id": agent_instance_id,
        "status": component.status,
        "readiness": component.readiness,
        "process_model": component.process_model,
        "risk_posture": component.risk_posture,
        "fail_closed_on_missing_evidence": component.fail_closed_on_missing_evidence,
        "fail_closed_on_policy_conflict": component.fail_closed_on_policy_conflict,
        "no_separate_binary": component.no_separate_binary,
        "channels": component.channels,
        "supervision_policy": component.supervision_policy,
        "decision_proofs": [
            evaluate_observation(&safe_observation, false),
            evaluate_observation(&missing_evidence, false),
            evaluate_observation(&safe_observation, true)
        ],
        "negative_case_policy": {
            "missing_evidence": "blocked",
            "malformed_observation": "blocked",
            "policy_conflict": "blocked",
            "degraded_runtime_state": "security_degraded"
        },
        "coordination": {
            "freedom_gate": "security_decision_channel_declared_for_admission_integration",
            "constructability_gate": "constructability_decision_channel_declared_for_follow_on_integration",
            "curiosity_engine": "probe_observation_channel_declared_for_follow_on_integration",
            "acip": "security_notice_channel_declared_for_follow_on_integration"
        },
        "retention": {
            "status_ref": CSM_CAV_STATUS_REF,
            "lifelog_required": true,
            "observability_required": true
        },
        "redaction": {
            "secret_material": "not_returned",
            "cloud_account_identifiers": "not_returned",
            "host_private_paths": "not_returned"
        },
        "updated_at": Utc::now()
    })
}

pub fn write_status_snapshot(state_root: &Path, agent_instance_id: &str) -> Result<Value> {
    fs::create_dir_all(state_root)
        .with_context(|| format!("create CSM CAV state root {}", state_root.display()))?;
    let snapshot = build_status_snapshot(agent_instance_id);
    let path = state_root.join(CSM_CAV_STATUS_REF);
    fs::write(&path, serde_json::to_vec_pretty(&snapshot)?)
        .with_context(|| format!("write CSM CAV status {}", path.display()))?;
    Ok(snapshot)
}

pub fn api_status(agent_instance_id: &str, artifact: &Value, runtime_capability: Value) -> Value {
    let fallback = build_status_snapshot(agent_instance_id);
    let artifact_state = artifact
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("missing");
    let value = artifact
        .get("value")
        .cloned()
        .unwrap_or_else(|| fallback.clone());
    let validation = if artifact_state == "serialized" {
        validate_retained_status(&value)
    } else {
        Err("cav_retained_status_missing_or_unreadable")
    };
    let status = if validation.is_ok() {
        artifact
            .get("status")
            .cloned()
            .unwrap_or_else(|| json!("missing"))
    } else {
        json!("blocked")
    };
    json!({
        "status": status,
        "ref": CSM_CAV_STATUS_REF,
        "schema": value.get("schema").cloned().unwrap_or_else(|| json!(CSM_CAV_STATUS_SCHEMA)),
        "runtime_owner": "csm",
        "component": CSM_CAV_COMPONENT,
        "capability": runtime_capability,
        "readiness": if validation.is_ok() { value.get("readiness").cloned().unwrap_or_else(|| json!("unknown")) } else { json!("blocked") },
        "validation": match validation {
            Ok(()) => json!({"status": "valid"}),
            Err(reason) => json!({"status": "fail_closed", "reason": reason}),
        },
        "decision_proofs": value.get("decision_proofs").cloned().unwrap_or_else(|| fallback["decision_proofs"].clone()),
        "negative_case_policy": value.get("negative_case_policy").cloned().unwrap_or_else(|| fallback["negative_case_policy"].clone()),
        "coordination": value.get("coordination").cloned().unwrap_or_else(|| fallback["coordination"].clone())
    })
}

pub fn validate_retained_status(value: &Value) -> Result<(), &'static str> {
    if value.get("schema").and_then(Value::as_str) != Some(CSM_CAV_STATUS_SCHEMA) {
        return Err("invalid_cav_status_schema");
    }
    if value.get("runtime_owner").and_then(Value::as_str) != Some("csm")
        || value.get("component").and_then(Value::as_str) != Some(CSM_CAV_COMPONENT)
    {
        return Err("invalid_cav_runtime_owner_or_component");
    }
    if value.get("process_model").and_then(Value::as_str)
        != Some("in_process_csm_runtime_component")
        || value.get("no_separate_binary").and_then(Value::as_bool) != Some(true)
    {
        return Err("cav_must_be_embedded_runtime_component");
    }
    if value
        .get("fail_closed_on_missing_evidence")
        .and_then(Value::as_bool)
        != Some(true)
        || value
            .get("fail_closed_on_policy_conflict")
            .and_then(Value::as_bool)
            != Some(true)
    {
        return Err("cav_must_fail_closed");
    }
    let Some(proofs) = value.get("decision_proofs").and_then(Value::as_array) else {
        return Err("cav_decision_proofs_missing");
    };
    if !proofs.iter().any(|proof| {
        proof.get("reason_code").and_then(Value::as_str) == Some("missing_evidence")
            && proof.get("decision").and_then(Value::as_str) == Some("blocked")
    }) {
        return Err("cav_missing_evidence_negative_case_absent");
    }
    if !proofs.iter().any(|proof| {
        proof.get("reason_code").and_then(Value::as_str) == Some("policy_conflict")
            && proof.get("decision").and_then(Value::as_str) == Some("blocked")
    }) {
        return Err("cav_policy_conflict_negative_case_absent");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cav_runtime_capability_is_embedded_and_fail_closed() {
        let capability = runtime_capability();
        assert_eq!(capability["component"], "cav");
        assert_eq!(
            capability["process_model"],
            "in_process_csm_runtime_component"
        );
        assert_eq!(capability["fail_closed"]["missing_evidence"], true);
        assert_eq!(capability["retained_status_ref"], CSM_CAV_STATUS_REF);
    }

    #[test]
    fn cav_blocks_missing_evidence_and_policy_conflicts() {
        let observation = CsmCavObservation {
            observation_id: "obs-1".to_string(),
            source_component: "freedom_gate".to_string(),
            observation_kind: "execution_admission".to_string(),
            evidence_ref: None,
            severity: "info".to_string(),
            policy_context_ref: Some("policy".to_string()),
        };
        let missing = evaluate_observation(&observation, false);
        assert_eq!(missing.decision, "blocked");
        assert_eq!(missing.reason_code, "missing_evidence");

        let with_evidence = CsmCavObservation {
            evidence_ref: Some("operator_events.jsonl".to_string()),
            ..observation
        };
        let conflict = evaluate_observation(&with_evidence, true);
        assert_eq!(conflict.decision, "blocked");
        assert_eq!(conflict.reason_code, "policy_conflict");
    }

    #[test]
    fn cav_status_snapshot_contains_required_negative_cases() {
        let status = build_status_snapshot("polis-alpha");
        validate_retained_status(&status).expect("valid CAV status");
        assert_eq!(status["no_separate_binary"], true);
        assert_eq!(
            status["negative_case_policy"]["malformed_observation"],
            "blocked"
        );
    }

    #[test]
    fn cav_api_fails_closed_on_partial_retained_artifact() {
        let artifact = json!({
            "status": "serialized",
            "value": {
                "schema": CSM_CAV_STATUS_SCHEMA,
                "runtime_owner": "csm",
                "component": "cav",
                "process_model": "in_process_csm_runtime_component",
                "no_separate_binary": true,
                "fail_closed_on_missing_evidence": false
            }
        });
        let status = api_status("polis-alpha", &artifact, runtime_capability());
        assert_eq!(status["status"], "blocked");
        assert_eq!(status["validation"]["status"], "fail_closed");
    }

    #[test]
    fn cav_api_fails_closed_when_retained_artifact_is_missing() {
        let artifact = json!({"status": "missing"});
        let status = api_status("polis-alpha", &artifact, runtime_capability());
        assert_eq!(status["status"], "blocked");
        assert_eq!(status["readiness"], "blocked");
        assert_eq!(
            status["validation"]["reason"],
            "cav_retained_status_missing_or_unreadable"
        );
    }
}
