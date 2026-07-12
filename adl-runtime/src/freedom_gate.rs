//! CSM Freedom Gate runtime component contract.

use serde::{Deserialize, Serialize};

pub const CSM_FREEDOM_GATE_COMPONENT: &str = "freedom_gate";
pub const CSM_FREEDOM_GATE_STATUS_SCHEMA: &str = "adl.csm.freedom_gate.status.v1";
pub const CSM_FREEDOM_GATE_CHANNELS_SCHEMA: &str = "adl.csm.freedom_gate.channels.v1";
pub const CSM_FREEDOM_GATE_DECISION_SCHEMA: &str = "adl.csm.freedom_gate.decision.v1";
pub const CSM_FREEDOM_GATE_STATUS_REF: &str = "csm_freedom_gate_status.json";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CsmFreedomGateChannels {
    pub schema: String,
    pub candidate_action_input: String,
    pub policy_context_input: String,
    pub decision_output: String,
    pub refusal_defer_escalation_output: String,
    pub retained_evidence_output: String,
}

impl Default for CsmFreedomGateChannels {
    fn default() -> Self {
        Self {
            schema: CSM_FREEDOM_GATE_CHANNELS_SCHEMA.to_string(),
            candidate_action_input: "scheduler.reasoning_runtime.candidate_action".to_string(),
            policy_context_input: "governance.freedom_gate.policy_context".to_string(),
            decision_output: "freedom_gate.decision".to_string(),
            refusal_defer_escalation_output: "freedom_gate.non_execution_decision".to_string(),
            retained_evidence_output: "observability.freedom_gate.decision_event".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CsmFreedomGateComponentStatus {
    pub schema: String,
    pub runtime_owner: String,
    pub component: String,
    pub status: String,
    pub readiness: String,
    pub process_model: String,
    pub mediation_position: String,
    pub executor_requires_gate_decision: bool,
    pub unmediated_execution_allowed: bool,
    pub channels: CsmFreedomGateChannels,
    pub supervision_policy: String,
    pub retained_status_ref: String,
}

impl Default for CsmFreedomGateComponentStatus {
    fn default() -> Self {
        Self {
            schema: CSM_FREEDOM_GATE_STATUS_SCHEMA.to_string(),
            runtime_owner: "csm".to_string(),
            component: CSM_FREEDOM_GATE_COMPONENT.to_string(),
            status: "integrated".to_string(),
            readiness: "available".to_string(),
            process_model: "in_process_csm_runtime_component".to_string(),
            mediation_position: "between_scheduler_reasoning_runtime_and_aee_executor".to_string(),
            executor_requires_gate_decision: true,
            unmediated_execution_allowed: false,
            channels: CsmFreedomGateChannels::default(),
            supervision_policy: "restart_with_backoff_fail_closed_to_executor".to_string(),
            retained_status_ref: CSM_FREEDOM_GATE_STATUS_REF.to_string(),
        }
    }
}

impl CsmFreedomGateComponentStatus {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != CSM_FREEDOM_GATE_STATUS_SCHEMA {
            return Err("freedom_gate status schema mismatch".to_string());
        }
        if self.runtime_owner != "csm" {
            return Err("freedom_gate must be owned by csm runtime".to_string());
        }
        if self.component != CSM_FREEDOM_GATE_COMPONENT {
            return Err("freedom_gate component id mismatch".to_string());
        }
        if self.readiness != "available" {
            return Err("freedom_gate must report available readiness".to_string());
        }
        if !self.executor_requires_gate_decision || self.unmediated_execution_allowed {
            return Err("freedom_gate must fail closed before executor invocation".to_string());
        }
        self.channels.validate()
    }
}

impl CsmFreedomGateChannels {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != CSM_FREEDOM_GATE_CHANNELS_SCHEMA {
            return Err("freedom_gate channel schema mismatch".to_string());
        }
        for (name, value) in [
            ("candidate_action_input", &self.candidate_action_input),
            ("policy_context_input", &self.policy_context_input),
            ("decision_output", &self.decision_output),
            (
                "refusal_defer_escalation_output",
                &self.refusal_defer_escalation_output,
            ),
            ("retained_evidence_output", &self.retained_evidence_output),
        ] {
            if value.trim().is_empty() {
                return Err(format!("freedom_gate channel {name} is required"));
            }
        }
        Ok(())
    }
}

pub fn default_freedom_gate_status() -> CsmFreedomGateComponentStatus {
    CsmFreedomGateComponentStatus::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_freedom_gate_status_is_csm_owned_and_fail_closed() {
        let status = default_freedom_gate_status();
        status.validate().expect("valid freedom gate status");
        assert_eq!(status.runtime_owner, "csm");
        assert!(status.executor_requires_gate_decision);
        assert!(!status.unmediated_execution_allowed);
    }

    #[test]
    fn freedom_gate_rejects_unmediated_executor_access() {
        let mut status = default_freedom_gate_status();
        status.unmediated_execution_allowed = true;
        let err = status
            .validate()
            .expect_err("unmediated execution rejected");
        assert!(err.contains("fail closed"));
    }
}
