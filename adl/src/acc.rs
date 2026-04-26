use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

pub const ACC_SCHEMA_VERSION_V1: &str = "acc.v1";
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

fn push_error(
    errors: &mut Vec<AccValidationError>,
    code: &'static str,
    field: &'static str,
    message: impl Into<String>,
) {
    errors.push(AccValidationError {
        code,
        field,
        message: message.into(),
    });
}

fn valid_token(value: &str) -> bool {
    !value.trim().is_empty()
        && value.chars().all(|ch| {
            ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '-' | '_' | '.')
        })
}

fn has_model_claim_evidence(actor: &AccActorIdentityV1) -> bool {
    actor
        .authority_evidence
        .iter()
        .any(|evidence| matches!(evidence.kind, AccAuthorityEvidenceKindV1::ModelClaim))
}

fn has_non_model_authority_evidence(actor: &AccActorIdentityV1) -> bool {
    actor
        .authority_evidence
        .iter()
        .any(|evidence| !matches!(evidence.kind, AccAuthorityEvidenceKindV1::ModelClaim))
}

fn visibility_is_complete(visibility: &AccVisibilityPolicyV1) -> bool {
    [
        &visibility.actor_view,
        &visibility.operator_view,
        &visibility.reviewer_view,
        &visibility.public_report_view,
        &visibility.observatory_projection,
    ]
    .iter()
    .all(|value| !value.trim().is_empty())
}

fn redaction_examples_cover_required_surfaces(examples: &[AccRedactionExampleV1]) -> bool {
    [
        AccRedactionSurfaceV1::Arguments,
        AccRedactionSurfaceV1::Results,
        AccRedactionSurfaceV1::Errors,
        AccRedactionSurfaceV1::Traces,
        AccRedactionSurfaceV1::Projections,
    ]
    .iter()
    .all(|surface| examples.iter().any(|example| &example.surface == surface))
}

fn redaction_examples_are_safe(examples: &[AccRedactionExampleV1]) -> bool {
    examples.iter().all(|example| {
        !example.source_shape.trim().is_empty()
            && !example.redacted_shape.trim().is_empty()
            && example.source_shape != example.redacted_shape
            && !contains_private_state_marker(&example.redacted_shape)
    })
}

fn visibility_matrix_covers_required_audiences(matrix: &[AccVisibilityMatrixEntryV1]) -> bool {
    [
        AccVisibilityAudienceV1::Actor,
        AccVisibilityAudienceV1::Operator,
        AccVisibilityAudienceV1::Reviewer,
        AccVisibilityAudienceV1::PublicReport,
        AccVisibilityAudienceV1::ObservatoryProjection,
    ]
    .iter()
    .all(|audience| matrix.iter().any(|entry| &entry.audience == audience))
}

fn visibility_matrix_fails_closed(matrix: &[AccVisibilityMatrixEntryV1]) -> bool {
    matrix.iter().all(|entry| {
        !entry.rationale.trim().is_empty()
            && match entry.audience {
                AccVisibilityAudienceV1::PublicReport
                | AccVisibilityAudienceV1::ObservatoryProjection => {
                    !matches!(entry.level, AccVisibilityLevelV1::Full)
                }
                _ => true,
            }
    })
}

fn contains_private_state_marker(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("citizen.private")
        || lower.contains("citizen_private")
        || lower.contains("private_state")
        || lower.contains("private-state")
}

fn policy_evidence_ref_is_known(contract: &AdlCapabilityContractV1, evidence_ref: &str) -> bool {
    contract
        .actor
        .authority_evidence
        .iter()
        .any(|evidence| evidence.evidence_id == evidence_ref)
        || contract.authority_grant.grant_id == evidence_ref
        || contract
            .delegation_chain
            .iter()
            .any(|step| step.delegation_id == evidence_ref)
}

pub fn validate_acc_v1(contract: &AdlCapabilityContractV1) -> Result<(), AccValidationReport> {
    let mut errors = Vec::new();

    if contract.schema_version != ACC_SCHEMA_VERSION_V1 {
        push_error(
            &mut errors,
            "unsupported_schema_version",
            "schema_version",
            format!("schema_version must be {ACC_SCHEMA_VERSION_V1}"),
        );
    }
    if !valid_token(&contract.contract_id) {
        push_error(
            &mut errors,
            "invalid_contract_id",
            "contract_id",
            "contract_id must be a non-empty token",
        );
    }
    if !valid_token(&contract.tool.tool_name)
        || !valid_token(&contract.tool.registry_tool_id)
        || !valid_token(&contract.tool.adapter_id)
    {
        push_error(
            &mut errors,
            "invalid_tool_reference",
            "tool",
            "tool reference must name a registered tool and adapter",
        );
    }
    if contract.actor.actor_id.trim().is_empty() || !contract.actor.authenticated {
        push_error(
            &mut errors,
            "missing_accountable_actor_identity",
            "actor",
            "ACC requires an authenticated accountable actor identity",
        );
    }
    if contract.actor.authority_evidence.is_empty() {
        push_error(
            &mut errors,
            "missing_authority_evidence",
            "actor.authority_evidence",
            "ACC authority must be grounded in non-model evidence",
        );
    }
    if has_model_claim_evidence(&contract.actor) {
        push_error(
            &mut errors,
            "model_self_reported_authority",
            "actor.authority_evidence",
            "model claims cannot establish ACC authority",
        );
    }
    if !has_non_model_authority_evidence(&contract.actor) {
        push_error(
            &mut errors,
            "missing_non_model_authority_evidence",
            "actor.authority_evidence",
            "at least one credential, grant, policy, registry, or delegation record is required",
        );
    }
    for evidence in &contract.actor.authority_evidence {
        if evidence.evidence_id.trim().is_empty() || evidence.issuer.trim().is_empty() {
            push_error(
                &mut errors,
                "invalid_authority_evidence",
                "actor.authority_evidence",
                "authority evidence must include an evidence id and issuer",
            );
        }
    }
    if contract.authority_grant.grant_id.trim().is_empty()
        || contract.authority_grant.grantor_actor_id.trim().is_empty()
        || contract.authority_grant.grantee_actor_id != contract.actor.actor_id
        || contract.authority_grant.capability_id != contract.capability.capability_id
    {
        push_error(
            &mut errors,
            "invalid_authority_grant",
            "authority_grant",
            "grant must name grantor, grantee actor, and matching capability",
        );
    }
    if matches!(contract.decision, AccDecisionV1::Allowed)
        && !matches!(contract.authority_grant.status, AccGrantStatusV1::Active)
    {
        push_error(
            &mut errors,
            "allowed_requires_active_grant",
            "authority_grant.status",
            "allowed ACC decisions require an active authority grant",
        );
    }
    if matches!(contract.decision, AccDecisionV1::Revoked)
        && !matches!(contract.authority_grant.status, AccGrantStatusV1::Revoked)
    {
        push_error(
            &mut errors,
            "revoked_requires_revoked_grant",
            "authority_grant.status",
            "revoked ACC decisions must carry a revoked grant",
        );
    }
    if contract.role_standing.role.trim().is_empty()
        || contract.role_standing.standing.trim().is_empty()
    {
        push_error(
            &mut errors,
            "missing_role_standing",
            "role_standing",
            "actor role and standing are required",
        );
    }
    for step in &contract.delegation_chain {
        if step.delegation_id.trim().is_empty()
            || step.grantor_actor_id.trim().is_empty()
            || step.delegate_actor_id.trim().is_empty()
            || step.grant_id.trim().is_empty()
            || step.depth == 0
            || step.depth > ACC_MAX_DELEGATION_DEPTH_V1
        {
            push_error(
                &mut errors,
                "invalid_delegation_step",
                "delegation_chain",
                "delegation steps must preserve attribution and bounded depth",
            );
        }
    }
    if matches!(contract.authority_grant.status, AccGrantStatusV1::Delegated) {
        if contract.delegation_chain.is_empty() {
            push_error(
                &mut errors,
                "hidden_delegation",
                "delegation_chain",
                "delegated grants require an explicit delegation chain",
            );
        } else if !contract.delegation_chain.iter().any(|step| {
            step.grant_id == contract.authority_grant.grant_id
                && step.grantor_actor_id == contract.authority_grant.grantor_actor_id
                && step.delegate_actor_id == contract.authority_grant.grantee_actor_id
        }) {
            push_error(
                &mut errors,
                "misattributed_delegation_chain",
                "delegation_chain",
                "delegation chain must bind the authority grant to its grantor and grantee",
            );
        }
    }
    if contract.capability.capability_id.trim().is_empty()
        || contract.capability.side_effect_class.trim().is_empty()
        || contract.capability.resource_type.trim().is_empty()
        || contract.capability.resource_scope.trim().is_empty()
    {
        push_error(
            &mut errors,
            "missing_capability_requirement",
            "capability",
            "capability, side effect, and resource boundaries are required",
        );
    }
    if contract.policy_checks.is_empty() {
        push_error(
            &mut errors,
            "missing_policy_checks",
            "policy_checks",
            "at least one policy check is required",
        );
    }
    for check in &contract.policy_checks {
        if check.policy_id.trim().is_empty() || check.evidence_ref.trim().is_empty() {
            push_error(
                &mut errors,
                "invalid_policy_check",
                "policy_checks",
                "policy checks must name a policy id and evidence reference",
            );
        }
        if check.decision != contract.decision {
            push_error(
                &mut errors,
                "policy_decision_mismatch",
                "policy_checks",
                "policy check decisions must agree with the ACC decision",
            );
        }
        if !policy_evidence_ref_is_known(contract, &check.evidence_ref) {
            push_error(
                &mut errors,
                "unknown_policy_evidence_ref",
                "policy_checks",
                "policy checks must cite authority grant, delegation, or actor authority evidence",
            );
        }
    }
    if contract.confirmation.required
        && (contract
            .confirmation
            .confirmed_by_actor_id
            .as_deref()
            .unwrap_or_default()
            .trim()
            .is_empty()
            || contract
                .confirmation
                .confirmation_id
                .as_deref()
                .unwrap_or_default()
                .trim()
                .is_empty())
    {
        push_error(
            &mut errors,
            "missing_confirmation",
            "confirmation",
            "required confirmations must name the confirming actor and confirmation id",
        );
    }
    if contract.freedom_gate.required
        && matches!(
            contract.freedom_gate.decision,
            AccFreedomGateDecisionV1::NotRequired
        )
    {
        push_error(
            &mut errors,
            "missing_freedom_gate_decision",
            "freedom_gate",
            "required Freedom Gate mediation must record a decision",
        );
    }
    if matches!(contract.decision, AccDecisionV1::Allowed)
        && !contract.execution.approved_for_execution
    {
        push_error(
            &mut errors,
            "allowed_requires_execution_approval",
            "execution.approved_for_execution",
            "allowed ACC decisions must explicitly approve execution",
        );
    }
    if contract.execution.adapter_id.trim().is_empty()
        || contract.execution.adapter_id != contract.tool.adapter_id
    {
        push_error(
            &mut errors,
            "execution_adapter_mismatch",
            "execution.adapter_id",
            "execution adapter must match the declared tool adapter",
        );
    }
    if !matches!(contract.decision, AccDecisionV1::Allowed)
        && contract.execution.approved_for_execution
    {
        push_error(
            &mut errors,
            "rejected_contract_cannot_execute",
            "execution.approved_for_execution",
            "denied, delegated, or revoked ACC decisions must not approve execution",
        );
    }
    if contract.trace_replay.trace_id.trim().is_empty()
        || contract.trace_replay.evidence_refs.is_empty()
    {
        push_error(
            &mut errors,
            "missing_trace_replay_evidence",
            "trace_replay",
            "ACC requires trace and replay evidence references",
        );
    }
    if !visibility_is_complete(&contract.privacy_redaction.visibility)
        || contract.privacy_redaction.redaction_rules.is_empty()
    {
        push_error(
            &mut errors,
            "unsafe_visibility_policy",
            "privacy_redaction",
            "visibility and redaction policy must be complete before execution",
        );
    }
    if !visibility_matrix_covers_required_audiences(&contract.privacy_redaction.visibility_matrix)
        || !visibility_matrix_fails_closed(&contract.privacy_redaction.visibility_matrix)
    {
        push_error(
            &mut errors,
            "unsafe_visibility_matrix",
            "privacy_redaction.visibility_matrix",
            "visibility matrix must cover all audiences and fail closed for public projections",
        );
    }
    if !redaction_examples_cover_required_surfaces(&contract.privacy_redaction.redaction_examples)
        || !redaction_examples_are_safe(&contract.privacy_redaction.redaction_examples)
    {
        push_error(
            &mut errors,
            "missing_redaction_examples",
            "privacy_redaction.redaction_examples",
            "redaction examples must cover arguments, results, errors, traces, projections, and remove private-state markers",
        );
    }
    if contract
        .privacy_redaction
        .trace_privacy
        .exposes_citizen_private_state
        || contract
            .privacy_redaction
            .trace_privacy
            .protected_state_refs
            .is_empty()
        || contract
            .trace_replay
            .evidence_refs
            .iter()
            .any(|evidence_ref| contains_private_state_marker(evidence_ref))
        || contains_private_state_marker(&contract.trace_replay.trace_id)
        || contains_private_state_marker(&contract.trace_replay.replay_posture)
    {
        push_error(
            &mut errors,
            "private_state_trace_exposure",
            "privacy_redaction.trace_privacy",
            "tool traces must not expose citizen or private-state surfaces",
        );
    }
    if contract.failure_policy.failure_code.trim().is_empty()
        || contract.failure_policy.message.trim().is_empty()
    {
        push_error(
            &mut errors,
            "missing_failure_policy",
            "failure_policy",
            "ACC requires failure policy for reviewable denial or execution errors",
        );
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(AccValidationReport { errors })
    }
}

pub fn acc_v1_schema_json() -> JsonValue {
    serde_json::to_value(schema_for!(AdlCapabilityContractV1))
        .expect("ACC v1 schema should serialize")
}

fn base_contract(id: &'static str) -> AdlCapabilityContractV1 {
    AdlCapabilityContractV1 {
        schema_version: ACC_SCHEMA_VERSION_V1.to_string(),
        contract_id: id.to_string(),
        tool: AccToolReferenceV1 {
            tool_name: "fixture.safe_read".to_string(),
            tool_version: "1.0.0".to_string(),
            registry_tool_id: "registry.fixture.safe_read".to_string(),
            adapter_id: "adapter.fixture.safe_read.dry_run".to_string(),
        },
        actor: AccActorIdentityV1 {
            actor_id: "actor.operator.alice".to_string(),
            actor_kind: AccActorKindV1::Operator,
            authenticated: true,
            authority_evidence: vec![AccAuthorityEvidenceV1 {
                evidence_id: "credential.operator.alice".to_string(),
                kind: AccAuthorityEvidenceKindV1::Credential,
                issuer: "adl.local-identity-fixture".to_string(),
            }],
        },
        authority_grant: AccAuthorityGrantV1 {
            grant_id: "grant.fixture.safe-read".to_string(),
            grantor_actor_id: "actor.operator.alice".to_string(),
            grantee_actor_id: "actor.operator.alice".to_string(),
            capability_id: "capability.fixture.safe-read".to_string(),
            scope: "fixture.readonly".to_string(),
            status: AccGrantStatusV1::Active,
            revocation_reason: None,
        },
        role_standing: AccRoleStandingV1 {
            role: "operator".to_string(),
            standing: "active".to_string(),
        },
        delegation_chain: Vec::new(),
        capability: AccCapabilityRequirementV1 {
            capability_id: "capability.fixture.safe-read".to_string(),
            side_effect_class: "read".to_string(),
            resource_type: "fixture".to_string(),
            resource_scope: "readonly".to_string(),
        },
        policy_checks: vec![AccPolicyCheckV1 {
            policy_id: "policy.fixture.readonly".to_string(),
            decision: AccDecisionV1::Allowed,
            evidence_ref: "credential.operator.alice".to_string(),
        }],
        confirmation: AccConfirmationRequirementV1 {
            required: false,
            confirmed_by_actor_id: None,
            confirmation_id: None,
        },
        freedom_gate: AccFreedomGateRequirementV1 {
            required: false,
            decision: AccFreedomGateDecisionV1::NotRequired,
            event_id: None,
        },
        execution: AccExecutionSemanticsV1 {
            adapter_id: "adapter.fixture.safe_read.dry_run".to_string(),
            environment: "fixture_dry_run".to_string(),
            dry_run: true,
            approved_for_execution: true,
        },
        trace_replay: AccTraceReplayV1 {
            trace_id: format!("trace.{id}"),
            replay_allowed: true,
            replay_posture: "deterministic_fixture".to_string(),
            evidence_refs: vec!["policy.fixture.readonly".to_string()],
        },
        privacy_redaction: AccPrivacyRedactionV1 {
            data_sensitivity: "internal".to_string(),
            visibility: AccVisibilityPolicyV1 {
                actor_view: "tool request and result summary".to_string(),
                operator_view: "full fixture request and result".to_string(),
                reviewer_view: "redacted fixture payload and policy evidence".to_string(),
                public_report_view: "aggregate pass/fail only".to_string(),
                observatory_projection: "redacted governance event".to_string(),
            },
            redaction_rules: vec!["redact_fixture_payload_for_public_report".to_string()],
            visibility_matrix: acc_v1_visibility_matrix(),
            redaction_examples: acc_v1_redaction_examples(),
            trace_privacy: AccTracePrivacyPolicyV1 {
                exposes_citizen_private_state: false,
                protected_state_refs: vec![
                    "citizen.private_state".to_string(),
                    "operator.private_state".to_string(),
                ],
            },
        },
        failure_policy: AccFailurePolicyV1 {
            failure_code: "fixture_unavailable".to_string(),
            message: "Fixture adapter could not provide the requested safe-read data.".to_string(),
            retryable: false,
        },
        decision: AccDecisionV1::Allowed,
    }
}

pub fn acc_v1_visibility_matrix() -> Vec<AccVisibilityMatrixEntryV1> {
    vec![
        AccVisibilityMatrixEntryV1 {
            audience: AccVisibilityAudienceV1::Actor,
            level: AccVisibilityLevelV1::Redacted,
            rationale: "actor can inspect request status without private-state internals"
                .to_string(),
        },
        AccVisibilityMatrixEntryV1 {
            audience: AccVisibilityAudienceV1::Operator,
            level: AccVisibilityLevelV1::Full,
            rationale: "operator may inspect full fixture evidence for accountability".to_string(),
        },
        AccVisibilityMatrixEntryV1 {
            audience: AccVisibilityAudienceV1::Reviewer,
            level: AccVisibilityLevelV1::Redacted,
            rationale: "reviewer receives policy evidence with protected payloads redacted"
                .to_string(),
        },
        AccVisibilityMatrixEntryV1 {
            audience: AccVisibilityAudienceV1::PublicReport,
            level: AccVisibilityLevelV1::Aggregate,
            rationale: "public report receives aggregate pass/fail and denial taxonomy only"
                .to_string(),
        },
        AccVisibilityMatrixEntryV1 {
            audience: AccVisibilityAudienceV1::ObservatoryProjection,
            level: AccVisibilityLevelV1::Redacted,
            rationale: "Observatory projection receives redacted governance events".to_string(),
        },
    ]
}

pub fn acc_v1_redaction_examples() -> Vec<AccRedactionExampleV1> {
    vec![
        AccRedactionExampleV1 {
            surface: AccRedactionSurfaceV1::Arguments,
            source_shape: r#"{"path":"citizen.private_state/memory.json"}"#.to_string(),
            redacted_shape: r#"{"path":"[redacted-protected-ref]"}"#.to_string(),
        },
        AccRedactionExampleV1 {
            surface: AccRedactionSurfaceV1::Results,
            source_shape: r#"{"content":"fixture payload"}"#.to_string(),
            redacted_shape: r#"{"content":"[redacted-result-summary]"}"#.to_string(),
        },
        AccRedactionExampleV1 {
            surface: AccRedactionSurfaceV1::Errors,
            source_shape: r#"{"error":"adapter failed reading private_state"}"#.to_string(),
            redacted_shape: r#"{"error":"adapter failed reading protected state"}"#.to_string(),
        },
        AccRedactionExampleV1 {
            surface: AccRedactionSurfaceV1::Traces,
            source_shape: r#"{"trace_ref":"citizen.private_state.step"}"#.to_string(),
            redacted_shape: r#"{"trace_ref":"[redacted-trace-ref]"}"#.to_string(),
        },
        AccRedactionExampleV1 {
            surface: AccRedactionSurfaceV1::Projections,
            source_shape: r#"{"observatory":"private_state_changed"}"#.to_string(),
            redacted_shape: r#"{"observatory":"governance_event_recorded"}"#.to_string(),
        },
    ]
}

pub fn acc_v1_authority_fixtures() -> Vec<AccAuthorityFixtureV1> {
    let allowed = base_contract("acc.fixture.allowed_safe_read");

    let mut denied = base_contract("acc.fixture.denied_untrusted_actor");
    denied.actor.authenticated = false;
    denied.actor.authority_evidence = Vec::new();
    denied.authority_grant.status = AccGrantStatusV1::Denied;
    denied.execution.approved_for_execution = false;
    denied.policy_checks[0].decision = AccDecisionV1::Denied;
    denied.decision = AccDecisionV1::Denied;
    denied.failure_policy.failure_code = "missing_accountable_actor_identity".to_string();
    denied.failure_policy.message =
        "The proposed capability lacks an authenticated accountable actor.".to_string();

    let mut delegated = base_contract("acc.fixture.delegated_safe_read");
    delegated.actor.actor_id = "actor.agent.reviewer".to_string();
    delegated.actor.actor_kind = AccActorKindV1::Agent;
    delegated.actor.authority_evidence = vec![AccAuthorityEvidenceV1 {
        evidence_id: "delegation.operator-to-reviewer".to_string(),
        kind: AccAuthorityEvidenceKindV1::DelegationRecord,
        issuer: "actor.operator.alice".to_string(),
    }];
    delegated.authority_grant.status = AccGrantStatusV1::Delegated;
    delegated.authority_grant.grant_id = "grant.delegated.safe-read".to_string();
    delegated.authority_grant.grantor_actor_id = "actor.operator.alice".to_string();
    delegated.authority_grant.grantee_actor_id = "actor.agent.reviewer".to_string();
    delegated.delegation_chain = vec![AccDelegationStepV1 {
        delegation_id: "delegation.operator-to-reviewer".to_string(),
        grantor_actor_id: "actor.operator.alice".to_string(),
        delegate_actor_id: "actor.agent.reviewer".to_string(),
        grant_id: "grant.delegated.safe-read".to_string(),
        depth: 1,
    }];
    delegated.policy_checks[0].decision = AccDecisionV1::Delegated;
    delegated.policy_checks[0].evidence_ref = "delegation.operator-to-reviewer".to_string();
    delegated.decision = AccDecisionV1::Delegated;
    delegated.execution.approved_for_execution = false;
    delegated.failure_policy.failure_code = "delegated_requires_policy_evaluation".to_string();
    delegated.failure_policy.message =
        "Delegated authority is recorded but not directly executable in WP-06.".to_string();

    let mut revoked = base_contract("acc.fixture.revoked_safe_read");
    revoked.authority_grant.status = AccGrantStatusV1::Revoked;
    revoked.authority_grant.revocation_reason = Some("operator_revoked_fixture_access".to_string());
    revoked.policy_checks[0].decision = AccDecisionV1::Revoked;
    revoked.execution.approved_for_execution = false;
    revoked.decision = AccDecisionV1::Revoked;
    revoked.failure_policy.failure_code = "revoked_authority".to_string();
    revoked.failure_policy.message =
        "The authority grant was revoked before execution could be approved.".to_string();

    vec![
        AccAuthorityFixtureV1 {
            id: "allowed.safe_read",
            contract: allowed,
            expected: AccExpectedFixtureOutcomeV1::Accepted,
        },
        AccAuthorityFixtureV1 {
            id: "denied.untrusted_actor",
            contract: denied,
            expected: AccExpectedFixtureOutcomeV1::Rejected("missing_accountable_actor_identity"),
        },
        AccAuthorityFixtureV1 {
            id: "delegated.safe_read",
            contract: delegated,
            expected: AccExpectedFixtureOutcomeV1::Accepted,
        },
        AccAuthorityFixtureV1 {
            id: "revoked.safe_read",
            contract: revoked,
            expected: AccExpectedFixtureOutcomeV1::Accepted,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn acc_v1_allowed_authority_fixture_passes() {
        let contract = base_contract("acc.fixture.allowed_safe_read");

        validate_acc_v1(&contract).expect("allowed authority fixture should pass");
        assert_eq!(contract.decision, AccDecisionV1::Allowed);
        assert!(contract.execution.approved_for_execution);
    }

    #[test]
    fn acc_v1_authority_fixtures_have_expected_outcomes() {
        let fixtures = acc_v1_authority_fixtures();
        let ids: Vec<&str> = fixtures.iter().map(|fixture| fixture.id).collect();

        assert_eq!(
            ids,
            vec![
                "allowed.safe_read",
                "denied.untrusted_actor",
                "delegated.safe_read",
                "revoked.safe_read"
            ]
        );

        for fixture in fixtures {
            let result = validate_acc_v1(&fixture.contract);
            match fixture.expected {
                AccExpectedFixtureOutcomeV1::Accepted => {
                    result.unwrap_or_else(|err| {
                        panic!("{} should pass; errors: {:?}", fixture.id, err.codes())
                    });
                }
                AccExpectedFixtureOutcomeV1::Rejected(expected) => {
                    let err = match result {
                        Ok(()) => panic!("{} should fail", fixture.id),
                        Err(err) => err,
                    };
                    assert!(
                        err.codes().contains(&expected),
                        "{} should fail with {expected}; got {:?}",
                        fixture.id,
                        err.codes()
                    );
                }
            }
        }
    }

    #[test]
    fn acc_v1_requires_accountable_actor_identity() {
        let mut contract = base_contract("acc.fixture.missing_actor");
        contract.actor.actor_id = String::new();
        contract.actor.authenticated = false;

        let err = validate_acc_v1(&contract).expect_err("missing actor identity should fail");

        assert!(err.codes().contains(&"missing_accountable_actor_identity"));
    }

    #[test]
    fn acc_v1_rejects_model_self_reported_authority() {
        let mut contract = base_contract("acc.fixture.model_claim");
        contract.actor.authority_evidence = vec![AccAuthorityEvidenceV1 {
            evidence_id: "model.claimed.admin".to_string(),
            kind: AccAuthorityEvidenceKindV1::ModelClaim,
            issuer: "model-output".to_string(),
        }];

        let err = validate_acc_v1(&contract).expect_err("model authority claim should fail");
        let codes = err.codes();

        assert!(codes.contains(&"model_self_reported_authority"));
        assert!(codes.contains(&"missing_non_model_authority_evidence"));
    }

    #[test]
    fn acc_v1_rejects_allowed_decision_without_active_grant() {
        let mut contract = base_contract("acc.fixture.revoked_but_allowed");
        contract.authority_grant.status = AccGrantStatusV1::Revoked;

        let err = validate_acc_v1(&contract).expect_err("allowed revoked grant should fail");

        assert!(err.codes().contains(&"allowed_requires_active_grant"));
    }

    #[test]
    fn acc_v1_rejects_hidden_delegation() {
        let mut contract = base_contract("acc.fixture.hidden_delegation");
        contract.authority_grant.status = AccGrantStatusV1::Delegated;

        let err = validate_acc_v1(&contract).expect_err("hidden delegation should fail");

        assert!(err.codes().contains(&"hidden_delegation"));
    }

    #[test]
    fn acc_v1_rejects_misattributed_delegation_chain() {
        let mut contract = base_contract("acc.fixture.misattributed_delegation");
        contract.actor.actor_id = "actor.agent.reviewer".to_string();
        contract.actor.actor_kind = AccActorKindV1::Agent;
        contract.actor.authority_evidence = vec![AccAuthorityEvidenceV1 {
            evidence_id: "delegation.operator-to-reviewer".to_string(),
            kind: AccAuthorityEvidenceKindV1::DelegationRecord,
            issuer: "actor.operator.alice".to_string(),
        }];
        contract.authority_grant.status = AccGrantStatusV1::Delegated;
        contract.authority_grant.grant_id = "grant.delegated.safe-read".to_string();
        contract.authority_grant.grantor_actor_id = "actor.operator.alice".to_string();
        contract.authority_grant.grantee_actor_id = "actor.agent.reviewer".to_string();
        contract.delegation_chain = vec![AccDelegationStepV1 {
            delegation_id: "delegation.unrelated".to_string(),
            grantor_actor_id: "actor.operator.mallory".to_string(),
            delegate_actor_id: "actor.agent.other".to_string(),
            grant_id: "grant.unrelated".to_string(),
            depth: 1,
        }];
        contract.policy_checks[0].decision = AccDecisionV1::Delegated;
        contract.policy_checks[0].evidence_ref = "delegation.operator-to-reviewer".to_string();
        contract.decision = AccDecisionV1::Delegated;
        contract.execution.approved_for_execution = false;

        let err =
            validate_acc_v1(&contract).expect_err("misattributed delegation chain should fail");

        assert!(err.codes().contains(&"misattributed_delegation_chain"));
    }

    #[test]
    fn acc_v1_rejects_policy_decision_and_evidence_drift() {
        let mut contract = base_contract("acc.fixture.policy_drift");
        contract.policy_checks[0].decision = AccDecisionV1::Denied;
        contract.policy_checks[0].evidence_ref = "credential.unknown".to_string();

        let err = validate_acc_v1(&contract).expect_err("policy drift should fail");
        let codes = err.codes();

        assert!(codes.contains(&"policy_decision_mismatch"));
        assert!(codes.contains(&"unknown_policy_evidence_ref"));
    }

    #[test]
    fn acc_v1_rejects_execution_adapter_drift() {
        let mut contract = base_contract("acc.fixture.adapter_drift");
        contract.execution.adapter_id = "adapter.fixture.other".to_string();

        let err = validate_acc_v1(&contract).expect_err("adapter drift should fail");

        assert!(err.codes().contains(&"execution_adapter_mismatch"));
    }

    #[test]
    fn acc_v1_reports_structural_authority_gaps() {
        let mut contract = base_contract("ACC Fixture Structural Gaps");
        contract.schema_version = "acc.future".to_string();
        contract.tool.tool_name = "Unsafe Tool".to_string();
        contract.tool.registry_tool_id.clear();
        contract.actor.authority_evidence[0].evidence_id.clear();
        contract.authority_grant.grant_id.clear();
        contract.authority_grant.grantor_actor_id.clear();
        contract.authority_grant.grantee_actor_id = "actor.other".to_string();
        contract.authority_grant.capability_id = "capability.other".to_string();
        contract.authority_grant.status = AccGrantStatusV1::Active;
        contract.decision = AccDecisionV1::Revoked;
        contract.role_standing.role.clear();
        contract.delegation_chain = vec![AccDelegationStepV1 {
            delegation_id: String::new(),
            grantor_actor_id: String::new(),
            delegate_actor_id: "actor.agent.reviewer".to_string(),
            grant_id: String::new(),
            depth: 1,
        }];
        contract.capability.side_effect_class.clear();
        contract.policy_checks.clear();
        contract.confirmation.required = true;
        contract.freedom_gate.required = true;
        contract.execution.approved_for_execution = true;
        contract.trace_replay.trace_id.clear();
        contract.trace_replay.evidence_refs.clear();
        contract.failure_policy.message.clear();

        let err = validate_acc_v1(&contract).expect_err("structural gaps should fail");
        let codes = err.codes();

        for expected in [
            "unsupported_schema_version",
            "invalid_contract_id",
            "invalid_tool_reference",
            "invalid_authority_evidence",
            "invalid_authority_grant",
            "revoked_requires_revoked_grant",
            "missing_role_standing",
            "invalid_delegation_step",
            "missing_capability_requirement",
            "missing_policy_checks",
            "missing_confirmation",
            "missing_freedom_gate_decision",
            "rejected_contract_cannot_execute",
            "missing_trace_replay_evidence",
            "missing_failure_policy",
        ] {
            assert!(codes.contains(&expected), "missing {expected}: {codes:?}");
        }
    }

    #[test]
    fn acc_v1_rejects_unsafe_visibility_policy() {
        let mut contract = base_contract("acc.fixture.unsafe_visibility");
        contract.privacy_redaction.visibility.public_report_view = String::new();
        contract.privacy_redaction.redaction_rules.clear();

        let err = validate_acc_v1(&contract).expect_err("unsafe visibility should fail");

        assert!(err.codes().contains(&"unsafe_visibility_policy"));
    }

    #[test]
    fn acc_v1_visibility_matrix_and_redaction_examples_cover_wp07_surfaces() {
        let matrix = acc_v1_visibility_matrix();
        let examples = acc_v1_redaction_examples();

        assert!(visibility_matrix_covers_required_audiences(&matrix));
        assert!(visibility_matrix_fails_closed(&matrix));
        assert!(redaction_examples_cover_required_surfaces(&examples));
        assert!(redaction_examples_are_safe(&examples));
        assert!(matrix.iter().any(|entry| entry.audience
            == AccVisibilityAudienceV1::ObservatoryProjection
            && entry.level == AccVisibilityLevelV1::Redacted));
    }

    #[test]
    fn acc_v1_rejects_unsafe_visibility_matrix() {
        let mut contract = base_contract("acc.fixture.unsafe_visibility_matrix");
        contract
            .privacy_redaction
            .visibility_matrix
            .retain(|entry| entry.audience != AccVisibilityAudienceV1::Reviewer);
        contract
            .privacy_redaction
            .visibility_matrix
            .push(AccVisibilityMatrixEntryV1 {
                audience: AccVisibilityAudienceV1::PublicReport,
                level: AccVisibilityLevelV1::Full,
                rationale: "unsafe public full view".to_string(),
            });

        let err = validate_acc_v1(&contract).expect_err("unsafe visibility matrix should fail");

        assert!(err.codes().contains(&"unsafe_visibility_matrix"));
    }

    #[test]
    fn acc_v1_rejects_missing_redaction_examples() {
        let mut contract = base_contract("acc.fixture.missing_redaction_examples");
        contract
            .privacy_redaction
            .redaction_examples
            .retain(|example| example.surface != AccRedactionSurfaceV1::Traces);

        let err = validate_acc_v1(&contract).expect_err("missing redaction example should fail");

        assert!(err.codes().contains(&"missing_redaction_examples"));
    }

    #[test]
    fn acc_v1_rejects_redacted_examples_that_expose_private_state_markers() {
        let mut contract = base_contract("acc.fixture.leaky_redacted_example");
        let trace_example = contract
            .privacy_redaction
            .redaction_examples
            .iter_mut()
            .find(|example| example.surface == AccRedactionSurfaceV1::Traces)
            .expect("trace redaction example should exist");
        trace_example.redacted_shape = r#"{"trace_ref":"citizen.private_state.step"}"#.to_string();

        let err =
            validate_acc_v1(&contract).expect_err("leaky redacted example should fail validation");

        assert!(err.codes().contains(&"missing_redaction_examples"));
    }

    #[test]
    fn acc_v1_rejects_private_state_trace_exposure() {
        let mut contract = base_contract("acc.fixture.private_state_trace_exposure");
        contract.trace_replay.trace_id = "trace.citizen.private_state.step".to_string();
        contract.trace_replay.replay_posture = "replay uses private_state marker".to_string();
        contract
            .trace_replay
            .evidence_refs
            .push("citizen.private_state.step".to_string());
        contract
            .privacy_redaction
            .trace_privacy
            .exposes_citizen_private_state = true;

        let err = validate_acc_v1(&contract).expect_err("private-state trace exposure should fail");

        assert!(err.codes().contains(&"private_state_trace_exposure"));
    }

    #[test]
    fn acc_v1_schema_generation_exposes_authority_surface() {
        let schema = acc_v1_schema_json();
        let properties = schema
            .get("properties")
            .and_then(JsonValue::as_object)
            .expect("generated ACC schema should expose properties");

        for key in [
            "schema_version",
            "contract_id",
            "tool",
            "actor",
            "authority_grant",
            "role_standing",
            "delegation_chain",
            "capability",
            "policy_checks",
            "confirmation",
            "freedom_gate",
            "execution",
            "trace_replay",
            "privacy_redaction",
            "failure_policy",
            "decision",
        ] {
            assert!(
                properties.contains_key(key),
                "ACC schema missing property {key}"
            );
        }
    }

    #[test]
    fn acc_v1_rejects_unknown_runtime_authority_fields() {
        let mut value =
            serde_json::to_value(base_contract("acc.fixture.unknown_field")).expect("json");
        value["model_confidence_grants_authority"] = json!(true);

        let err = serde_json::from_value::<AdlCapabilityContractV1>(value)
            .expect_err("unknown authority field should not deserialize");

        assert!(err.to_string().contains("unknown field"));
    }
}
