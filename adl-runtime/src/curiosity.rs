//! CSM Curiosity Engine runtime contracts.
//!
//! Curiosity is an embedded CSM component. It proposes bounded investigations
//! through typed channels and cannot admit proposals without sibling governance
//! components.

use serde::{Deserialize, Serialize};

pub const CSM_CURIOSITY_COMPONENT: &str = "curiosity_engine";
pub const CSM_CURIOSITY_STATUS_SCHEMA: &str = "adl.csm.curiosity_engine.status.v1";
pub const CSM_CURIOSITY_PROPOSAL_SCHEMA: &str = "adl.csm.curiosity_engine.proposal.v1";
pub const CSM_CURIOSITY_CHANNELS_SCHEMA: &str = "adl.csm.curiosity_engine.channels.v1";
pub const CSM_CURIOSITY_STATUS_REF: &str = "csm_curiosity_engine_status.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CsmCuriosityReadiness {
    Ready,
    Blocked,
    Degraded,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CsmCuriosityProposalStatus {
    Proposed,
    ReadyForReview,
    RejectedByGovernance,
    Deferred,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmCuriosityChannels {
    pub schema: String,
    pub observations: String,
    pub proposal_requests: String,
    pub proposal_decisions: String,
    pub rejected_proposals: String,
    pub observability: String,
    pub lifelog: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmCuriosityConstraintHooks {
    pub freedom_gate_required: bool,
    pub cav_required: bool,
    pub constructability_required: bool,
    pub missing_constraint_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmCuriosityProposal {
    pub schema: String,
    pub proposal_id: String,
    pub source_signal_id: String,
    pub question: String,
    pub hypothesis: String,
    pub experiment_plan: Vec<String>,
    pub expected_artifacts: Vec<String>,
    pub gated_by: Vec<String>,
    pub status: CsmCuriosityProposalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmCuriosityComponentStatus {
    pub schema: String,
    pub runtime_owner: String,
    pub component: String,
    pub hosted_core_schema: String,
    pub hosted_core_ref: String,
    pub status: String,
    pub readiness: CsmCuriosityReadiness,
    pub process_model: String,
    pub channels: CsmCuriosityChannels,
    pub constraint_hooks: CsmCuriosityConstraintHooks,
    pub proposals: Vec<CsmCuriosityProposal>,
    pub retained_status_ref: String,
}

impl CsmCuriosityChannels {
    pub fn new(component: &str) -> Self {
        Self {
            schema: CSM_CURIOSITY_CHANNELS_SCHEMA.to_string(),
            observations: format!("csm.{component}.observations"),
            proposal_requests: format!("csm.{component}.proposal_requests"),
            proposal_decisions: format!("csm.{component}.proposal_decisions"),
            rejected_proposals: format!("csm.{component}.rejected_proposals"),
            observability: format!("csm.{component}.observability"),
            lifelog: format!("csm.{component}.lifelog"),
        }
    }
}

impl CsmCuriosityConstraintHooks {
    pub fn required() -> Self {
        Self {
            freedom_gate_required: true,
            cav_required: true,
            constructability_required: true,
            missing_constraint_policy: "fail_closed".to_string(),
        }
    }
}

impl CsmCuriosityProposal {
    pub fn validate(&self) -> Result<(), String> {
        require_exact(
            &self.schema,
            CSM_CURIOSITY_PROPOSAL_SCHEMA,
            "proposal.schema",
        )?;
        require_non_empty(&self.proposal_id, "proposal.proposal_id")?;
        require_non_empty(&self.source_signal_id, "proposal.source_signal_id")?;
        require_non_empty(&self.question, "proposal.question")?;
        require_non_empty(&self.hypothesis, "proposal.hypothesis")?;
        if self.experiment_plan.is_empty() {
            return Err("proposal.experiment_plan must not be empty".to_string());
        }
        for gate in ["freedom_gate", "cav", "constructability_anchor"] {
            if !self.gated_by.iter().any(|value| value == gate) {
                return Err(format!("proposal must be gated by {gate}"));
            }
        }
        Ok(())
    }
}

impl CsmCuriosityComponentStatus {
    pub fn validate(&self) -> Result<(), String> {
        require_exact(&self.schema, CSM_CURIOSITY_STATUS_SCHEMA, "schema")?;
        require_exact(&self.runtime_owner, "csm", "runtime_owner")?;
        require_exact(&self.component, CSM_CURIOSITY_COMPONENT, "component")?;
        require_exact(
            &self.hosted_core_schema,
            "runtime_v2.curiosity_engine.v1",
            "hosted_core_schema",
        )?;
        require_exact(
            &self.hosted_core_ref,
            "adl/src/runtime_v2/curiosity_engine.rs",
            "hosted_core_ref",
        )?;
        require_exact(
            &self.process_model,
            "embedded_csm_runtime_component",
            "process_model",
        )?;
        require_exact(
            &self.channels.schema,
            CSM_CURIOSITY_CHANNELS_SCHEMA,
            "channels.schema",
        )?;
        if !self.constraint_hooks.freedom_gate_required
            || !self.constraint_hooks.cav_required
            || !self.constraint_hooks.constructability_required
            || self.constraint_hooks.missing_constraint_policy != "fail_closed"
        {
            return Err(
                "Curiosity Engine must require Freedom Gate, CAV, Constructability, and fail-closed missing constraints"
                    .to_string(),
            );
        }
        require_exact(
            &self.retained_status_ref,
            CSM_CURIOSITY_STATUS_REF,
            "retained_status_ref",
        )?;
        for proposal in &self.proposals {
            proposal.validate()?;
        }
        Ok(())
    }
}

fn require_non_empty(value: &str, field: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{field} must not be empty"));
    }
    Ok(())
}

fn require_exact(value: &str, expected: &str, field: &str) -> Result<(), String> {
    if value != expected {
        return Err(format!("{field} must be {expected}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn proposal() -> CsmCuriosityProposal {
        CsmCuriosityProposal {
            schema: CSM_CURIOSITY_PROPOSAL_SCHEMA.to_string(),
            proposal_id: "proposal-runtime-gap".to_string(),
            source_signal_id: "signal-runtime-gap".to_string(),
            question: "Which bounded runtime proof is missing?".to_string(),
            hypothesis: "A missing proof can be routed as a governed proposal.".to_string(),
            experiment_plan: vec!["write a retained proof artifact".to_string()],
            expected_artifacts: vec!["curiosity_proposal.json".to_string()],
            gated_by: vec![
                "freedom_gate".to_string(),
                "cav".to_string(),
                "constructability_anchor".to_string(),
            ],
            status: CsmCuriosityProposalStatus::ReadyForReview,
        }
    }

    #[test]
    fn curiosity_status_requires_embedded_fail_closed_component() {
        let status = CsmCuriosityComponentStatus {
            schema: CSM_CURIOSITY_STATUS_SCHEMA.to_string(),
            runtime_owner: "csm".to_string(),
            component: CSM_CURIOSITY_COMPONENT.to_string(),
            hosted_core_schema: "runtime_v2.curiosity_engine.v1".to_string(),
            hosted_core_ref: "adl/src/runtime_v2/curiosity_engine.rs".to_string(),
            status: "idle".to_string(),
            readiness: CsmCuriosityReadiness::Ready,
            process_model: "embedded_csm_runtime_component".to_string(),
            channels: CsmCuriosityChannels::new(CSM_CURIOSITY_COMPONENT),
            constraint_hooks: CsmCuriosityConstraintHooks::required(),
            proposals: vec![proposal()],
            retained_status_ref: CSM_CURIOSITY_STATUS_REF.to_string(),
        };
        status.validate().expect("valid curiosity status");
    }

    #[test]
    fn curiosity_proposals_reject_missing_governance_gate() {
        let mut proposal = proposal();
        proposal.gated_by.retain(|gate| gate != "cav");
        assert!(proposal
            .validate()
            .expect_err("cav gate is required")
            .contains("cav"));
    }
}
