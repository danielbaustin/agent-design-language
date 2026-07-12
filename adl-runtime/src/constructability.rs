//! Typed contracts for the embedded CSM Constructability Gate.

use serde::{Deserialize, Serialize};

pub const CSM_CONSTRUCTABILITY_COMPONENT: &str = "constructability_gate";
pub const CSM_CONSTRUCTABILITY_REQUEST_SCHEMA: &str = "adl.csm.constructability.request.v1";
pub const CSM_CONSTRUCTABILITY_EVIDENCE_SCHEMA: &str = "adl.csm.constructability.evidence.v1";
pub const CSM_CONSTRUCTABILITY_DECISION_SCHEMA: &str = "adl.csm.constructability.decision.v1";
pub const CSM_CONSTRUCTABILITY_STATUS_SCHEMA: &str = "adl.csm.constructability.status.v1";
pub const CSM_CONSTRUCTABILITY_CHANNELS_SCHEMA: &str = "adl.csm.constructability.channels.v1";
pub const CSM_CONSTRUCTABILITY_STATUS_REF: &str = "csm_constructability_status.json";
pub const CSM_CONSTRUCTABILITY_DECISIONS_REF: &str = "csm_constructability_decisions.jsonl";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum CsmConstructabilityPublicationScope {
    InternalTraceOnly,
    ReviewPacket,
    SharedReality,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CsmConstructabilityEvidenceKind {
    RetainedArtifact,
    RuntimeTrace,
    OperatorApproval,
    ExternalRecord,
    RepositoryState,
    RuntimeResource,
    ValidationResult,
    IntegrationState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CsmConstructabilityEvidenceState {
    Available,
    Unavailable,
    Rejected,
    Malformed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CsmConstructabilityGateState {
    Allow,
    Defer,
    Block,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CsmConstructabilityOutcome {
    Allow,
    Defer,
    Block,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CsmConstructabilityReadiness {
    Active,
    Degraded,
    Blocked,
    Unavailable,
    NoEvidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CsmConstructabilityEvidenceMode {
    Live,
    Fixture,
    Mock,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmConstructabilityEvidence {
    pub schema: String,
    pub evidence_id: String,
    pub kind: CsmConstructabilityEvidenceKind,
    pub state: CsmConstructabilityEvidenceState,
    pub source_ref: String,
    pub summary: String,
    pub retryable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmConstructabilityGateInputs {
    pub freedom_gate: CsmConstructabilityGateState,
    pub cav: CsmConstructabilityGateState,
    pub curiosity: CsmConstructabilityGateState,
    pub missing_gate_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmConstructabilityRequest {
    pub schema: String,
    pub request_id: String,
    pub proposal_id: String,
    pub source_component: String,
    pub source_ref: String,
    pub proposed_action: String,
    pub evidence_mode: CsmConstructabilityEvidenceMode,
    pub publication_scope: CsmConstructabilityPublicationScope,
    pub required_evidence_kinds: Vec<CsmConstructabilityEvidenceKind>,
    pub evidence: Vec<CsmConstructabilityEvidence>,
    pub gates: CsmConstructabilityGateInputs,
    pub acip_publication_requested: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmConstructabilityDecision {
    pub schema: String,
    pub request_id: String,
    pub proposal_id: String,
    pub outcome: CsmConstructabilityOutcome,
    pub reason_codes: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub remediation_hints: Vec<String>,
    pub anchor_validator_schema: String,
    pub anchor_validator_ref: String,
    pub anchor_validator_outcome: String,
    pub gates: CsmConstructabilityGateInputs,
    pub acip_publication_allowed: bool,
    pub deterministic_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmConstructabilityChannels {
    pub schema: String,
    pub request_capacity: usize,
    pub decision_capacity: usize,
    pub requests: String,
    pub decisions: String,
    pub deferred: String,
    pub blocked: String,
    pub checkpoint: String,
    pub lifelog: String,
    pub observability: String,
    pub loss_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmConstructabilityComponentStatus {
    pub schema: String,
    pub runtime_owner: String,
    pub component: String,
    pub process_model: String,
    pub status: String,
    pub readiness: CsmConstructabilityReadiness,
    pub channels: CsmConstructabilityChannels,
    pub hosted_anchor_validator_schema: String,
    pub hosted_anchor_validator_ref: String,
    pub last_decision: Option<CsmConstructabilityDecision>,
    pub retained_status_ref: String,
    pub retained_decisions_ref: String,
}

impl CsmConstructabilityChannels {
    pub fn bounded() -> Self {
        Self {
            schema: CSM_CONSTRUCTABILITY_CHANNELS_SCHEMA.to_string(),
            request_capacity: 256,
            decision_capacity: 256,
            requests: "csm.constructability.requests".to_string(),
            decisions: "csm.constructability.decisions".to_string(),
            deferred: "csm.constructability.deferred".to_string(),
            blocked: "csm.constructability.blocked".to_string(),
            checkpoint: "csm.constructability.checkpoint".to_string(),
            lifelog: "csm.constructability.lifelog".to_string(),
            observability: "csm.constructability.observability".to_string(),
            loss_policy: "never_silent_drop_block_or_defer_with_retained_decision".to_string(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        require_exact(
            &self.schema,
            CSM_CONSTRUCTABILITY_CHANNELS_SCHEMA,
            "channels.schema",
        )?;
        if self.request_capacity == 0 || self.decision_capacity == 0 {
            return Err("constructability channel capacities must be positive".to_string());
        }
        require_exact(
            &self.loss_policy,
            "never_silent_drop_block_or_defer_with_retained_decision",
            "channels.loss_policy",
        )
    }
}

impl CsmConstructabilityEvidence {
    pub fn validate(&self) -> Result<(), String> {
        require_exact(
            &self.schema,
            CSM_CONSTRUCTABILITY_EVIDENCE_SCHEMA,
            "evidence.schema",
        )?;
        require_non_empty(&self.evidence_id, "evidence.evidence_id")?;
        validate_relative_ref(&self.source_ref, "evidence.source_ref")?;
        require_non_empty(&self.summary, "evidence.summary")
    }
}

impl CsmConstructabilityGateInputs {
    pub fn validate(&self) -> Result<(), String> {
        require_exact(
            &self.missing_gate_policy,
            "fail_closed",
            "gates.missing_gate_policy",
        )
    }
}

impl CsmConstructabilityRequest {
    pub fn validate(&self) -> Result<(), String> {
        require_exact(
            &self.schema,
            CSM_CONSTRUCTABILITY_REQUEST_SCHEMA,
            "request.schema",
        )?;
        require_stable_id(&self.request_id, "request.request_id")?;
        require_stable_id(&self.proposal_id, "request.proposal_id")?;
        require_non_empty(&self.source_component, "request.source_component")?;
        validate_relative_ref(&self.source_ref, "request.source_ref")?;
        require_non_empty(&self.proposed_action, "request.proposed_action")?;
        if self.required_evidence_kinds.is_empty() {
            return Err("request.required_evidence_kinds must not be empty".to_string());
        }
        self.gates.validate()?;
        for evidence in &self.evidence {
            evidence.validate()?;
        }
        Ok(())
    }
}

impl CsmConstructabilityDecision {
    pub fn validate(&self) -> Result<(), String> {
        require_exact(
            &self.schema,
            CSM_CONSTRUCTABILITY_DECISION_SCHEMA,
            "decision.schema",
        )?;
        require_stable_id(&self.request_id, "decision.request_id")?;
        require_stable_id(&self.proposal_id, "decision.proposal_id")?;
        if self.reason_codes.is_empty() {
            return Err("decision.reason_codes must not be empty".to_string());
        }
        if self.outcome == CsmConstructabilityOutcome::Allow && self.evidence_refs.is_empty() {
            return Err("allow decision must retain evidence refs".to_string());
        }
        if self.acip_publication_allowed && self.outcome != CsmConstructabilityOutcome::Allow {
            return Err("ACIP publication requires an allow decision".to_string());
        }
        self.gates.validate()?;
        require_non_empty(
            &self.anchor_validator_schema,
            "decision.anchor_validator_schema",
        )?;
        require_non_empty(&self.anchor_validator_ref, "decision.anchor_validator_ref")?;
        require_non_empty(
            &self.anchor_validator_outcome,
            "decision.anchor_validator_outcome",
        )?;
        require_non_empty(&self.deterministic_key, "decision.deterministic_key")
    }
}

impl CsmConstructabilityComponentStatus {
    pub fn validate(&self) -> Result<(), String> {
        require_exact(
            &self.schema,
            CSM_CONSTRUCTABILITY_STATUS_SCHEMA,
            "status.schema",
        )?;
        require_exact(&self.runtime_owner, "csm", "status.runtime_owner")?;
        require_exact(
            &self.component,
            CSM_CONSTRUCTABILITY_COMPONENT,
            "status.component",
        )?;
        require_exact(
            &self.process_model,
            "embedded_csm_runtime_component",
            "status.process_model",
        )?;
        self.channels.validate()?;
        require_non_empty(
            &self.hosted_anchor_validator_schema,
            "status.hosted_anchor_validator_schema",
        )?;
        require_non_empty(
            &self.hosted_anchor_validator_ref,
            "status.hosted_anchor_validator_ref",
        )?;
        require_exact(
            &self.retained_status_ref,
            CSM_CONSTRUCTABILITY_STATUS_REF,
            "status.retained_status_ref",
        )?;
        require_exact(
            &self.retained_decisions_ref,
            CSM_CONSTRUCTABILITY_DECISIONS_REF,
            "status.retained_decisions_ref",
        )?;
        if let Some(decision) = &self.last_decision {
            decision.validate()?;
        }
        Ok(())
    }
}

fn require_non_empty(value: &str, field: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{field} must not be empty"))
    } else {
        Ok(())
    }
}

fn require_stable_id(value: &str, field: &str) -> Result<(), String> {
    require_non_empty(value, field)?;
    if value.contains(['/', '\\', ':']) {
        Err(format!("{field} must be a stable identifier, not a path"))
    } else {
        Ok(())
    }
}

fn validate_relative_ref(value: &str, field: &str) -> Result<(), String> {
    require_non_empty(value, field)?;
    if value.starts_with('/') || value.contains("..") || value.contains("\\") {
        Err(format!("{field} must be a repository-relative reference"))
    } else {
        Ok(())
    }
}

fn require_exact(value: &str, expected: &str, field: &str) -> Result<(), String> {
    if value == expected {
        Ok(())
    } else {
        Err(format!("{field} must be {expected}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_rejects_host_paths_and_missing_required_evidence_kinds() {
        let request = CsmConstructabilityRequest {
            schema: CSM_CONSTRUCTABILITY_REQUEST_SCHEMA.to_string(),
            request_id: "request-1".to_string(),
            proposal_id: "proposal-1".to_string(),
            source_component: "curiosity_engine".to_string(),
            source_ref: "/tmp/proposal.json".to_string(),
            proposed_action: "publish a review packet".to_string(),
            evidence_mode: CsmConstructabilityEvidenceMode::Live,
            publication_scope: CsmConstructabilityPublicationScope::ReviewPacket,
            required_evidence_kinds: vec![],
            evidence: vec![],
            gates: CsmConstructabilityGateInputs {
                freedom_gate: CsmConstructabilityGateState::Allow,
                cav: CsmConstructabilityGateState::Allow,
                curiosity: CsmConstructabilityGateState::Allow,
                missing_gate_policy: "fail_closed".to_string(),
            },
            acip_publication_requested: false,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn channels_never_silently_drop_constructability_decisions() {
        let channels = CsmConstructabilityChannels::bounded();
        channels.validate().expect("valid bounded channels");
        assert!(channels.loss_policy.contains("never_silent_drop"));
    }
}
