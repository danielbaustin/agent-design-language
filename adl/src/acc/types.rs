use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const ACC_SCHEMA_VERSION_V1: &str = "acc.v1";
pub const ACC_SCHEMA_VERSION_V1_0: &str = ACC_SCHEMA_VERSION_V1;
pub const ACC_SCHEMA_VERSION_V1_1: &str = "acc.v1.1";
pub const ACC_MAX_DELEGATION_DEPTH_V1: u8 = 8;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccActorKindV1 {
    Human,
    Agent,
    Service,
    Operator,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccAuthorityEvidenceKindV1 {
    Credential,
    OperatorGrant,
    RegistryGrant,
    PolicyRecord,
    DelegationRecord,
    ModelClaim,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccAuthorityEvidenceV1 {
    pub evidence_id: String,
    pub kind: AccAuthorityEvidenceKindV1,
    pub issuer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccActorIdentityV1 {
    pub actor_id: String,
    pub actor_kind: AccActorKindV1,
    pub authenticated: bool,
    pub authority_evidence: Vec<AccAuthorityEvidenceV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccGrantStatusV1 {
    Active,
    Denied,
    Delegated,
    Revoked,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccAuthorityGrantV1 {
    pub grant_id: String,
    pub grantor_actor_id: String,
    pub grantee_actor_id: String,
    pub capability_id: String,
    pub scope: String,
    pub status: AccGrantStatusV1,
    #[serde(default)]
    pub revocation_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccRoleStandingV1 {
    pub role: String,
    pub standing: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccDelegationStepV1 {
    pub delegation_id: String,
    pub grantor_actor_id: String,
    pub delegate_actor_id: String,
    pub grant_id: String,
    pub depth: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccCapabilityRequirementV1 {
    pub capability_id: String,
    pub side_effect_class: String,
    pub resource_type: String,
    pub resource_scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccDecisionV1 {
    Allowed,
    Denied,
    Delegated,
    Revoked,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccPolicyCheckV1 {
    pub policy_id: String,
    pub decision: AccDecisionV1,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccConfirmationRequirementV1 {
    pub required: bool,
    #[serde(default)]
    pub confirmed_by_actor_id: Option<String>,
    #[serde(default)]
    pub confirmation_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccFreedomGateDecisionV1 {
    NotRequired,
    Allowed,
    Denied,
    Deferred,
    Challenged,
    Escalated,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccFreedomGateRequirementV1 {
    pub required: bool,
    pub decision: AccFreedomGateDecisionV1,
    #[serde(default)]
    pub event_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccExecutionSemanticsV1 {
    pub adapter_id: String,
    pub environment: String,
    pub dry_run: bool,
    pub approved_for_execution: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccTraceReplayV1 {
    pub trace_id: String,
    pub replay_allowed: bool,
    pub replay_posture: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccVisibilityPolicyV1 {
    pub actor_view: String,
    pub operator_view: String,
    pub reviewer_view: String,
    pub public_report_view: String,
    pub observatory_projection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AccVisibilityAudienceV1 {
    Actor,
    Operator,
    Reviewer,
    PublicReport,
    ObservatoryProjection,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccVisibilityLevelV1 {
    Full,
    Redacted,
    Aggregate,
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccVisibilityMatrixEntryV1 {
    pub audience: AccVisibilityAudienceV1,
    pub level: AccVisibilityLevelV1,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccRedactionSurfaceV1 {
    Arguments,
    Results,
    Errors,
    Traces,
    Projections,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccRedactionExampleV1 {
    pub surface: AccRedactionSurfaceV1,
    pub source_shape: String,
    pub redacted_shape: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccTracePrivacyPolicyV1 {
    pub exposes_citizen_private_state: bool,
    pub protected_state_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccPrivacyRedactionV1 {
    pub data_sensitivity: String,
    pub visibility: AccVisibilityPolicyV1,
    pub redaction_rules: Vec<String>,
    pub visibility_matrix: Vec<AccVisibilityMatrixEntryV1>,
    pub redaction_examples: Vec<AccRedactionExampleV1>,
    pub trace_privacy: AccTracePrivacyPolicyV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccFailurePolicyV1 {
    pub failure_code: String,
    pub message: String,
    pub retryable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccToolReferenceV1 {
    pub tool_name: String,
    pub tool_version: String,
    pub registry_tool_id: String,
    pub adapter_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AdlCapabilityContractV1 {
    pub schema_version: String,
    pub contract_id: String,
    pub tool: AccToolReferenceV1,
    pub actor: AccActorIdentityV1,
    pub authority_grant: AccAuthorityGrantV1,
    pub role_standing: AccRoleStandingV1,
    pub delegation_chain: Vec<AccDelegationStepV1>,
    pub capability: AccCapabilityRequirementV1,
    pub policy_checks: Vec<AccPolicyCheckV1>,
    pub confirmation: AccConfirmationRequirementV1,
    pub freedom_gate: AccFreedomGateRequirementV1,
    pub execution: AccExecutionSemanticsV1,
    pub trace_replay: AccTraceReplayV1,
    pub privacy_redaction: AccPrivacyRedactionV1,
    pub failure_policy: AccFailurePolicyV1,
    pub decision: AccDecisionV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AccDelegationConstraintsV1_1 {
    pub max_depth: u8,
    pub allow_redelegation: bool,
    #[serde(default)]
    pub scope_ceiling: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AdlCapabilityContractV1_1 {
    pub schema_version: String,
    #[serde(default)]
    pub compatible_versions: Option<Vec<String>>,
    #[serde(default)]
    pub governance_profile: Option<String>,
    pub contract_id: String,
    pub tool: AccToolReferenceV1,
    pub actor: AccActorIdentityV1,
    pub authority_grant: AccAuthorityGrantV1,
    pub role_standing: AccRoleStandingV1,
    pub delegation_chain: Vec<AccDelegationStepV1>,
    #[serde(default)]
    pub delegation_constraints: Option<AccDelegationConstraintsV1_1>,
    pub capability: AccCapabilityRequirementV1,
    pub policy_checks: Vec<AccPolicyCheckV1>,
    pub confirmation: AccConfirmationRequirementV1,
    pub freedom_gate: AccFreedomGateRequirementV1,
    pub execution: AccExecutionSemanticsV1,
    pub trace_replay: AccTraceReplayV1,
    pub privacy_redaction: AccPrivacyRedactionV1,
    pub failure_policy: AccFailurePolicyV1,
    pub decision: AccDecisionV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccValidationError {
    pub code: &'static str,
    pub field: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccValidationReport {
    pub errors: Vec<AccValidationError>,
}

impl AccValidationReport {
    pub fn codes(&self) -> Vec<&'static str> {
        self.errors.iter().map(|error| error.code).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccExpectedFixtureOutcomeV1 {
    Accepted,
    Rejected(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccAuthorityFixtureV1 {
    pub id: &'static str,
    pub contract: AdlCapabilityContractV1,
    pub expected: AccExpectedFixtureOutcomeV1,
}
