use schemars::schema_for;
use serde_json::Value as JsonValue;

use super::{
    AccActorIdentityV1, AccAuthorityEvidenceKindV1, AccDecisionV1, AccGrantStatusV1,
    AccRedactionExampleV1, AccValidationError, AccValidationReport, AccVisibilityAudienceV1,
    AccVisibilityLevelV1, AccVisibilityMatrixEntryV1, AccVisibilityPolicyV1,
    AdlCapabilityContractV1, AdlCapabilityContractV1_1, ACC_MAX_DELEGATION_DEPTH_V1,
    ACC_SCHEMA_VERSION_V1, ACC_SCHEMA_VERSION_V1_1,
};

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

pub(crate) fn visibility_is_complete(visibility: &AccVisibilityPolicyV1) -> bool {
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

pub(crate) fn redaction_examples_cover_required_surfaces(
    examples: &[AccRedactionExampleV1],
) -> bool {
    use super::AccRedactionSurfaceV1::{Arguments, Errors, Projections, Results, Traces};

    [Arguments, Results, Errors, Traces, Projections]
        .iter()
        .all(|surface| examples.iter().any(|example| &example.surface == surface))
}

pub(crate) fn redaction_examples_are_safe(examples: &[AccRedactionExampleV1]) -> bool {
    examples.iter().all(|example| {
        !example.source_shape.trim().is_empty()
            && !example.redacted_shape.trim().is_empty()
            && example.source_shape != example.redacted_shape
            && !contains_private_state_marker(&example.redacted_shape)
    })
}

pub(crate) fn visibility_matrix_covers_required_audiences(
    matrix: &[AccVisibilityMatrixEntryV1],
) -> bool {
    use super::AccVisibilityAudienceV1::{
        Actor, ObservatoryProjection, Operator, PublicReport, Reviewer,
    };

    [
        Actor,
        Operator,
        Reviewer,
        PublicReport,
        ObservatoryProjection,
    ]
    .iter()
    .all(|audience| matrix.iter().any(|entry| &entry.audience == audience))
}

pub(crate) fn visibility_matrix_fails_closed(matrix: &[AccVisibilityMatrixEntryV1]) -> bool {
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

fn project_acc_v1_1_to_v1(contract: &AdlCapabilityContractV1_1) -> AdlCapabilityContractV1 {
    AdlCapabilityContractV1 {
        schema_version: ACC_SCHEMA_VERSION_V1.to_string(),
        contract_id: contract.contract_id.clone(),
        tool: contract.tool.clone(),
        actor: contract.actor.clone(),
        authority_grant: contract.authority_grant.clone(),
        role_standing: contract.role_standing.clone(),
        delegation_chain: contract.delegation_chain.clone(),
        capability: contract.capability.clone(),
        policy_checks: contract.policy_checks.clone(),
        confirmation: contract.confirmation.clone(),
        freedom_gate: contract.freedom_gate.clone(),
        execution: contract.execution.clone(),
        trace_replay: contract.trace_replay.clone(),
        privacy_redaction: contract.privacy_redaction.clone(),
        failure_policy: contract.failure_policy.clone(),
        decision: contract.decision.clone(),
    }
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
            super::AccFreedomGateDecisionV1::NotRequired
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

pub fn upgrade_acc_v1_to_v1_1(contract: AdlCapabilityContractV1) -> AdlCapabilityContractV1_1 {
    AdlCapabilityContractV1_1 {
        schema_version: ACC_SCHEMA_VERSION_V1_1.to_string(),
        compatible_versions: Some(vec![
            ACC_SCHEMA_VERSION_V1.to_string(),
            ACC_SCHEMA_VERSION_V1_1.to_string(),
        ]),
        governance_profile: Some("standard_reviewed_runtime".to_string()),
        contract_id: contract.contract_id,
        tool: contract.tool,
        actor: contract.actor,
        authority_grant: contract.authority_grant,
        role_standing: contract.role_standing,
        delegation_chain: contract.delegation_chain,
        delegation_constraints: None,
        capability: contract.capability,
        policy_checks: contract.policy_checks,
        confirmation: contract.confirmation,
        freedom_gate: contract.freedom_gate,
        execution: contract.execution,
        trace_replay: contract.trace_replay,
        privacy_redaction: contract.privacy_redaction,
        failure_policy: contract.failure_policy,
        decision: contract.decision,
    }
}

impl From<AdlCapabilityContractV1> for AdlCapabilityContractV1_1 {
    fn from(contract: AdlCapabilityContractV1) -> Self {
        upgrade_acc_v1_to_v1_1(contract)
    }
}

pub fn validate_acc_v1_1(contract: &AdlCapabilityContractV1_1) -> Result<(), AccValidationReport> {
    let mut projected = project_acc_v1_1_to_v1(contract);
    projected.schema_version = ACC_SCHEMA_VERSION_V1.to_string();
    let mut errors = match validate_acc_v1(&projected) {
        Ok(()) => Vec::new(),
        Err(report) => report.errors,
    };

    if contract.schema_version != ACC_SCHEMA_VERSION_V1_1 {
        push_error(
            &mut errors,
            "unsupported_schema_version",
            "schema_version",
            format!("schema_version must be {ACC_SCHEMA_VERSION_V1_1}"),
        );
    }

    if let Some(compatible_versions) = &contract.compatible_versions {
        if compatible_versions.is_empty() {
            push_error(
                &mut errors,
                "invalid_compatible_versions",
                "compatible_versions",
                "compatible_versions must not be empty when present",
            );
        } else if !compatible_versions
            .iter()
            .any(|version| version == ACC_SCHEMA_VERSION_V1_1)
        {
            push_error(
                &mut errors,
                "missing_self_compatible_version",
                "compatible_versions",
                "compatible_versions must include acc.v1.1 when present",
            );
        }
    }

    if contract
        .governance_profile
        .as_deref()
        .is_some_and(|profile| !valid_token(profile))
    {
        push_error(
            &mut errors,
            "invalid_governance_profile",
            "governance_profile",
            "governance_profile must be a non-empty token",
        );
    }

    if let Some(constraints) = &contract.delegation_constraints {
        if constraints.max_depth == 0 || constraints.max_depth > ACC_MAX_DELEGATION_DEPTH_V1 {
            push_error(
                &mut errors,
                "invalid_delegation_constraints",
                "delegation_constraints.max_depth",
                "delegation_constraints.max_depth must stay within the ACC delegation bound",
            );
        }
        if constraints
            .scope_ceiling
            .as_deref()
            .is_some_and(|scope| scope.trim().is_empty())
        {
            push_error(
                &mut errors,
                "invalid_delegation_constraints",
                "delegation_constraints.scope_ceiling",
                "delegation_constraints.scope_ceiling must be omitted or non-empty",
            );
        }
        if contract
            .delegation_chain
            .iter()
            .any(|step| step.depth > constraints.max_depth)
        {
            push_error(
                &mut errors,
                "delegation_constraints_exceeded",
                "delegation_constraints.max_depth",
                "delegation chain depth must not exceed delegation_constraints.max_depth",
            );
        }
        if !constraints.allow_redelegation && contract.delegation_chain.len() > 1 {
            push_error(
                &mut errors,
                "redelegation_not_allowed",
                "delegation_constraints.allow_redelegation",
                "delegation chain cannot redelegate when allow_redelegation is false",
            );
        }
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

pub fn acc_v1_1_schema_json() -> JsonValue {
    serde_json::to_value(schema_for!(AdlCapabilityContractV1_1))
        .expect("ACC v1.1 schema should serialize")
}
