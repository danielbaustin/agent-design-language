//! Curiosity Engine embedded CSM component surfaces.

use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

use adl_runtime::curiosity::{
    CsmCuriosityChannels, CsmCuriosityComponentStatus, CsmCuriosityConstraintHooks,
    CsmCuriosityProposal, CsmCuriosityProposalStatus, CsmCuriosityReadiness,
    CSM_CURIOSITY_COMPONENT, CSM_CURIOSITY_PROPOSAL_SCHEMA, CSM_CURIOSITY_STATUS_REF,
    CSM_CURIOSITY_STATUS_SCHEMA,
};

use crate::runtime_v2::{
    runtime_v2_curiosity_engine_contract, RuntimeV2CuriosityEnginePacket,
    RuntimeV2CuriosityProposal, RuntimeV2CuriosityProposalStatus,
    RUNTIME_V2_CURIOSITY_ENGINE_SCHEMA,
};

pub const CSM_CURIOSITY_DECISION_SCHEMA: &str = "adl.csm.curiosity_engine.decision.v1";
pub const CSM_CURIOSITY_OBSERVATION_SCHEMA: &str = "adl.csm.curiosity_engine.observation.v1";
pub const CSM_CURIOSITY_HOSTED_CORE_REF: &str = "adl/src/runtime_v2/curiosity_engine.rs";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmCuriosityObservation {
    pub schema: String,
    pub observation_id: String,
    pub observation_kind: String,
    pub summary: String,
    pub novelty_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmCuriosityGovernanceDecision {
    pub schema: String,
    pub freedom_gate: String,
    pub cav: String,
    pub constructability: String,
}

pub fn runtime_capability() -> Value {
    json!({
        "status": "integrated",
        "component": CSM_CURIOSITY_COMPONENT,
        "component_class": "embedded_csm_runtime_component",
        "process_model": "in_process_no_sidecar_no_separate_binary",
        "hosted_core": {
            "schema": RUNTIME_V2_CURIOSITY_ENGINE_SCHEMA,
            "module_ref": CSM_CURIOSITY_HOSTED_CORE_REF,
            "source_wp": "WP-10",
            "source_issue": "4692"
        },
        "proposal_schema": CSM_CURIOSITY_PROPOSAL_SCHEMA,
        "decision_schema": CSM_CURIOSITY_DECISION_SCHEMA,
        "observation_schema": CSM_CURIOSITY_OBSERVATION_SCHEMA,
        "channels": CsmCuriosityChannels::new(CSM_CURIOSITY_COMPONENT),
        "constraint_hooks": CsmCuriosityConstraintHooks::required(),
        "retained_status_ref": CSM_CURIOSITY_STATUS_REF,
        "governance": {
            "freedom_gate_required": true,
            "cav_required": true,
            "constructability_required": true,
            "missing_constraint_policy": "fail_closed",
            "proposal_authority": "reviewable_proposal_only"
        },
        "non_claims": [
            "does_not_execute_external_actions",
            "does_not_bypass_sibling_governance",
            "does_not_claim_autonomous_learning_success_without_retained_evidence"
        ]
    })
}

pub fn evaluate_observation(
    observation: &CsmCuriosityObservation,
    governance: &CsmCuriosityGovernanceDecision,
) -> CsmCuriosityProposal {
    let all_allowed = governance.freedom_gate == "allow"
        && governance.cav == "allow"
        && governance.constructability == "allow";
    let status = if all_allowed {
        CsmCuriosityProposalStatus::ReadyForReview
    } else {
        CsmCuriosityProposalStatus::RejectedByGovernance
    };
    CsmCuriosityProposal {
        schema: CSM_CURIOSITY_PROPOSAL_SCHEMA.to_string(),
        proposal_id: format!("proposal-{}", observation.observation_id),
        source_signal_id: observation.observation_id.clone(),
        question: format!(
            "What bounded investigation follows from {}?",
            observation.summary
        ),
        hypothesis: "A governed inquiry can reduce runtime uncertainty without bypassing policy."
            .to_string(),
        experiment_plan: vec!["retain a reviewable curiosity proposal".to_string()],
        expected_artifacts: vec![
            "curiosity_proposal.json".to_string(),
            "operator_events.jsonl".to_string(),
        ],
        gated_by: vec![
            "freedom_gate".to_string(),
            "cav".to_string(),
            "constructability_anchor".to_string(),
        ],
        status,
    }
}

fn hosted_core_status(
    upstream_constraints_available: bool,
) -> Result<(Value, Vec<CsmCuriosityProposal>)> {
    let packet = runtime_v2_curiosity_engine_contract()?;
    let proposals = packet
        .proposals
        .iter()
        .map(|proposal| hosted_core_proposal(proposal, upstream_constraints_available))
        .collect();
    Ok((serde_json::to_value(packet)?, proposals))
}

fn hosted_core_proposal(
    proposal: &RuntimeV2CuriosityProposal,
    upstream_constraints_available: bool,
) -> CsmCuriosityProposal {
    let status = if upstream_constraints_available {
        match proposal.status {
            RuntimeV2CuriosityProposalStatus::ReadyForReview => {
                CsmCuriosityProposalStatus::ReadyForReview
            }
            RuntimeV2CuriosityProposalStatus::BlockedByBudget => {
                CsmCuriosityProposalStatus::Deferred
            }
            RuntimeV2CuriosityProposalStatus::BlockedByGovernance => {
                CsmCuriosityProposalStatus::RejectedByGovernance
            }
            RuntimeV2CuriosityProposalStatus::Proposed => CsmCuriosityProposalStatus::Proposed,
        }
    } else {
        CsmCuriosityProposalStatus::RejectedByGovernance
    };
    let mut gated_by = proposal
        .gated_by
        .iter()
        .map(|gate| {
            if gate == "cav_review" {
                "cav".to_string()
            } else {
                gate.clone()
            }
        })
        .collect::<Vec<_>>();
    if !gated_by.iter().any(|gate| gate == "cav") {
        gated_by.push("cav".to_string());
    }
    CsmCuriosityProposal {
        schema: CSM_CURIOSITY_PROPOSAL_SCHEMA.to_string(),
        proposal_id: proposal.proposal_id.clone(),
        source_signal_id: proposal.source_signal_id.clone(),
        question: proposal.question.clone(),
        hypothesis: proposal.hypothesis.clone(),
        experiment_plan: proposal.experiment_plan.clone(),
        expected_artifacts: proposal.expected_artifacts.clone(),
        gated_by,
        status,
    }
}

pub fn build_status_snapshot(
    agent_instance_id: &str,
    daemon_state: &str,
    agent_state: Option<&str>,
    upstream_constraints_available: bool,
) -> Value {
    let readiness = if upstream_constraints_available {
        CsmCuriosityReadiness::Ready
    } else {
        CsmCuriosityReadiness::Blocked
    };
    let status = if upstream_constraints_available {
        "idle"
    } else {
        "blocked"
    };
    let (hosted_core, proposals) = hosted_core_status(upstream_constraints_available)
        .unwrap_or_else(|err| {
            (
                json!({"status": "unavailable", "error": err.to_string()}),
                vec![],
            )
        });
    let component = CsmCuriosityComponentStatus {
        schema: CSM_CURIOSITY_STATUS_SCHEMA.to_string(),
        runtime_owner: "csm".to_string(),
        component: CSM_CURIOSITY_COMPONENT.to_string(),
        hosted_core_schema: RUNTIME_V2_CURIOSITY_ENGINE_SCHEMA.to_string(),
        hosted_core_ref: CSM_CURIOSITY_HOSTED_CORE_REF.to_string(),
        status: status.to_string(),
        readiness,
        process_model: "embedded_csm_runtime_component".to_string(),
        channels: CsmCuriosityChannels::new(CSM_CURIOSITY_COMPONENT),
        constraint_hooks: CsmCuriosityConstraintHooks::required(),
        proposals,
        retained_status_ref: CSM_CURIOSITY_STATUS_REF.to_string(),
    };
    let validation = component
        .validate()
        .map(|_| json!({"status": "passed"}))
        .unwrap_or_else(|err| json!({"status": "fail_closed", "reason": err}));
    json!({
        "schema": CSM_CURIOSITY_STATUS_SCHEMA,
        "runtime_owner": "csm",
        "component": CSM_CURIOSITY_COMPONENT,
        "agent_instance_id": agent_instance_id,
        "hosted_core_schema": component.hosted_core_schema,
        "hosted_core_ref": component.hosted_core_ref,
        "hosted_core": hosted_core,
        "status": component.status,
        "readiness": component.readiness,
        "process_model": component.process_model,
        "channels": component.channels,
        "constraint_hooks": component.constraint_hooks,
        "proposals": component.proposals,
        "observed_inputs": {
            "daemon_state": daemon_state,
            "agent_state": agent_state.unwrap_or("unknown"),
            "upstream_constraints_available": upstream_constraints_available
        },
        "validation": validation,
        "retention": {
            "status_ref": CSM_CURIOSITY_STATUS_REF,
            "lifelog_required": true,
            "observability_required": true
        },
        "retained_status_ref": CSM_CURIOSITY_STATUS_REF,
        "updated_at": Utc::now()
    })
}

pub fn write_status_snapshot(
    state_root: &Path,
    agent_instance_id: &str,
    daemon_state: &str,
    agent_state: Option<&str>,
    upstream_constraints_available: bool,
) -> Result<Value> {
    fs::create_dir_all(state_root)
        .with_context(|| format!("create CSM Curiosity state root {}", state_root.display()))?;
    let snapshot = build_status_snapshot(
        agent_instance_id,
        daemon_state,
        agent_state,
        upstream_constraints_available,
    );
    let path = state_root.join(CSM_CURIOSITY_STATUS_REF);
    fs::write(&path, serde_json::to_vec_pretty(&snapshot)?)
        .with_context(|| format!("write CSM Curiosity status {}", path.display()))?;
    Ok(snapshot)
}

pub fn api_status(
    agent_instance_id: &str,
    artifact: &Value,
    runtime_capability: Value,
    daemon_state: &str,
    agent_state: &str,
) -> Value {
    if artifact.get("status").and_then(Value::as_str) != Some("serialized") {
        let mut fallback =
            build_status_snapshot(agent_instance_id, daemon_state, Some(agent_state), false);
        if let Some(map) = fallback.as_object_mut() {
            map.insert("status".to_string(), json!("unavailable"));
            map.insert("readiness".to_string(), json!("blocked"));
            map.insert(
                "validation".to_string(),
                json!({
                    "status": "fail_closed",
                    "reason": "retained_curiosity_status_missing_or_unreadable"
                }),
            );
        }
        let validation = json!({
            "status": "fail_closed",
            "reason": "retained_curiosity_status_missing_or_unreadable"
        });
        return json!({
            "status": artifact.get("status").cloned().unwrap_or_else(|| json!("missing")),
            "ref": CSM_CURIOSITY_STATUS_REF,
            "schema": CSM_CURIOSITY_STATUS_SCHEMA,
            "runtime_owner": "csm",
            "component": CSM_CURIOSITY_COMPONENT,
            "capability": runtime_capability,
            "value": fallback,
            "validation": validation
        });
    }

    let value = artifact.get("value").cloned().unwrap_or_else(|| {
        build_status_snapshot(agent_instance_id, daemon_state, Some(agent_state), false)
    });
    let validation = validate_retained_status_for_agent(&value, agent_instance_id)
        .map(|_| json!({"status": "passed"}))
        .unwrap_or_else(|err| json!({"status": "fail_closed", "reason": err.to_string()}));
    json!({
        "status": artifact.get("status").cloned().unwrap_or_else(|| json!("missing")),
        "ref": CSM_CURIOSITY_STATUS_REF,
        "schema": value.get("schema").cloned().unwrap_or_else(|| json!(CSM_CURIOSITY_STATUS_SCHEMA)),
        "runtime_owner": "csm",
        "component": CSM_CURIOSITY_COMPONENT,
        "capability": runtime_capability,
        "value": value,
        "validation": validation
    })
}

pub fn validate_retained_status(value: &Value) -> Result<()> {
    validate_retained_status_core(value)
}

pub fn validate_retained_status_for_agent(value: &Value, agent_instance_id: &str) -> Result<()> {
    validate_retained_status_core(value)?;
    let retained_agent = value
        .get("agent_instance_id")
        .and_then(Value::as_str)
        .context("retained CSM Curiosity status missing agent_instance_id")?;
    if retained_agent != agent_instance_id {
        anyhow::bail!("retained CSM Curiosity status belongs to a different agent_instance_id");
    }
    Ok(())
}

fn validate_retained_status_core(value: &Value) -> Result<()> {
    let status: CsmCuriosityComponentStatus =
        serde_json::from_value(value.clone()).context("parse retained CSM Curiosity status")?;
    status.validate().map_err(anyhow::Error::msg)?;
    let hosted_core = value
        .get("hosted_core")
        .cloned()
        .context("retained CSM Curiosity status missing hosted_core")?;
    let hosted_packet: RuntimeV2CuriosityEnginePacket = serde_json::from_value(hosted_core)
        .context("parse retained CSM Curiosity hosted_core packet")?;
    hosted_packet
        .validate()
        .context("validate retained CSM Curiosity hosted_core packet")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curiosity_rejects_proposals_when_governance_is_unavailable() {
        let observation = CsmCuriosityObservation {
            schema: CSM_CURIOSITY_OBSERVATION_SCHEMA.to_string(),
            observation_id: "runtime-gap".to_string(),
            observation_kind: "runtime_state".to_string(),
            summary: "missing runtime evidence".to_string(),
            novelty_score: 1,
        };
        let decision = CsmCuriosityGovernanceDecision {
            schema: CSM_CURIOSITY_DECISION_SCHEMA.to_string(),
            freedom_gate: "allow".to_string(),
            cav: "deny".to_string(),
            constructability: "allow".to_string(),
        };
        let proposal = evaluate_observation(&observation, &decision);
        assert_eq!(
            proposal.status,
            CsmCuriosityProposalStatus::RejectedByGovernance
        );
        proposal.validate().expect("governed rejection is typed");
    }

    #[test]
    fn missing_curiosity_artifact_fails_closed_for_api_status() {
        let status = api_status(
            "polis-alpha",
            &json!({"status": "missing"}),
            runtime_capability(),
            "running",
            "running",
        );
        assert_eq!(status["value"]["status"], "unavailable");
        assert_eq!(status["value"]["readiness"], "blocked");
        assert_eq!(status["value"]["validation"]["status"], "fail_closed");
    }

    #[test]
    fn curiosity_snapshot_blocks_when_constraints_are_not_proven_available() {
        let status = build_status_snapshot("polis-alpha", "running", Some("running"), false);
        assert_eq!(status["status"], "blocked");
        assert_eq!(status["readiness"], "blocked");
        assert_eq!(
            status["observed_inputs"]["upstream_constraints_available"],
            false
        );
        assert_eq!(status["proposals"][0]["status"], "rejected_by_governance");
    }

    #[test]
    fn retained_curiosity_status_must_belong_to_current_agent() {
        let status = build_status_snapshot("polis-alpha", "running", Some("running"), true);
        validate_retained_status_for_agent(&status, "polis-alpha")
            .expect("matching agent status validates");
        let err = validate_retained_status_for_agent(&status, "polis-beta")
            .expect_err("cross-agent retained status must fail closed");
        assert!(err.to_string().contains("different agent_instance_id"));
    }

    #[test]
    fn retained_curiosity_status_must_validate_hosted_core_packet() {
        let mut status = build_status_snapshot("polis-alpha", "running", Some("running"), true);
        status
            .as_object_mut()
            .expect("snapshot is an object")
            .remove("hosted_core");
        let err = validate_retained_status_for_agent(&status, "polis-alpha")
            .expect_err("missing hosted core must fail closed");
        assert!(err.to_string().contains("missing hosted_core"));

        let mut drifted = build_status_snapshot("polis-alpha", "running", Some("running"), true);
        drifted["hosted_core"]["schema_version"] = json!("runtime_v2.curiosity_engine.drifted");
        let err = validate_retained_status_for_agent(&drifted, "polis-alpha")
            .expect_err("drifted hosted core must fail closed");
        assert!(
            err.to_string().contains("hosted_core packet"),
            "unexpected error: {err}"
        );
    }
}
