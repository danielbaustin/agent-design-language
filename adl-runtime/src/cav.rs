//! CSM Continuous Adversarial Verification component contract.

use serde::Serialize;

pub const CSM_CAV_COMPONENT: &str = "cav";
pub const CSM_CAV_STATUS_SCHEMA: &str = "adl.csm.cav.status.v1";
pub const CSM_CAV_CHANNELS_SCHEMA: &str = "adl.csm.cav.channels.v1";
pub const CSM_CAV_DECISION_SCHEMA: &str = "adl.csm.cav.decision.v1";
pub const CSM_CAV_STATUS_REF: &str = "csm_cav_status.json";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CsmCavChannels {
    pub schema: &'static str,
    pub observation_input: &'static str,
    pub policy_input: &'static str,
    pub decision_output: &'static str,
    pub retained_evidence_output: &'static str,
    pub coordination_output: &'static str,
}

impl Default for CsmCavChannels {
    fn default() -> Self {
        Self {
            schema: CSM_CAV_CHANNELS_SCHEMA,
            observation_input: "csm.cav.observation.v1",
            policy_input: "csm.cav.policy.v1",
            decision_output: "csm.cav.decision.v1",
            retained_evidence_output: "csm.cav.retained_evidence.v1",
            coordination_output: "csm.cav.coordination.v1",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CsmCavComponentStatus {
    pub schema: &'static str,
    pub runtime_owner: &'static str,
    pub component: &'static str,
    pub status: &'static str,
    pub readiness: &'static str,
    pub process_model: &'static str,
    pub risk_posture: &'static str,
    pub fail_closed_on_missing_evidence: bool,
    pub fail_closed_on_policy_conflict: bool,
    pub secrets_redacted: bool,
    pub no_separate_binary: bool,
    pub channels: CsmCavChannels,
    pub supervision_policy: &'static str,
    pub retained_status_ref: &'static str,
}

impl Default for CsmCavComponentStatus {
    fn default() -> Self {
        Self {
            schema: CSM_CAV_STATUS_SCHEMA,
            runtime_owner: "csm",
            component: CSM_CAV_COMPONENT,
            status: "integrated",
            readiness: "healthy",
            process_model: "in_process_csm_runtime_component",
            risk_posture: "fail_closed_security_gate",
            fail_closed_on_missing_evidence: true,
            fail_closed_on_policy_conflict: true,
            secrets_redacted: true,
            no_separate_binary: true,
            channels: CsmCavChannels::default(),
            supervision_policy:
                "restart_with_backoff_and_block_security_ready_until_evidence_recovers",
            retained_status_ref: CSM_CAV_STATUS_REF,
        }
    }
}

impl CsmCavComponentStatus {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.schema != CSM_CAV_STATUS_SCHEMA {
            return Err("invalid_cav_status_schema");
        }
        if self.runtime_owner != "csm" || self.component != CSM_CAV_COMPONENT {
            return Err("invalid_cav_runtime_owner_or_component");
        }
        if self.process_model != "in_process_csm_runtime_component" || !self.no_separate_binary {
            return Err("cav_must_be_embedded_runtime_component");
        }
        if !self.fail_closed_on_missing_evidence || !self.fail_closed_on_policy_conflict {
            return Err("cav_must_fail_closed");
        }
        if !self.secrets_redacted {
            return Err("cav_status_must_redact_secrets");
        }
        if self.channels.schema != CSM_CAV_CHANNELS_SCHEMA {
            return Err("invalid_cav_channels_schema");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_cav_component_status_is_valid_and_embedded() {
        let status = CsmCavComponentStatus::default();
        status.validate().expect("default CAV status");
        assert_eq!(status.component, "cav");
        assert!(status.no_separate_binary);
        assert!(status.fail_closed_on_missing_evidence);
        assert_eq!(status.retained_status_ref, "csm_cav_status.json");
    }

    #[test]
    fn cav_component_contract_rejects_non_fail_closed_status() {
        let mut status = CsmCavComponentStatus::default();
        status.fail_closed_on_policy_conflict = false;
        assert_eq!(status.validate(), Err("cav_must_fail_closed"));
    }

    #[test]
    fn cav_component_contract_rejects_sidecar_model() {
        let mut status = CsmCavComponentStatus::default();
        status.no_separate_binary = false;
        assert_eq!(
            status.validate(),
            Err("cav_must_be_embedded_runtime_component")
        );
    }
}
