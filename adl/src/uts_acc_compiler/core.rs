use super::frontend::{
    normalized_arguments_evidence, proposal_arguments_evidence, registry_evidence,
};
use super::{
    evidence, reject, UtsAccCompilerDecisionV1, UtsAccCompilerEvidenceStageV1,
    UtsAccCompilerInputV1, UtsAccCompilerOutcomeV1, UtsAccCompilerRejectionCodeV1,
    UtsAccPolicyContextV1,
};
use crate::acc::{
    acc_v1_redaction_examples, acc_v1_visibility_matrix, validate_acc_v1, AccActorIdentityV1,
    AccActorKindV1, AccAuthorityEvidenceKindV1, AccAuthorityEvidenceV1, AccAuthorityGrantV1,
    AccCapabilityRequirementV1, AccConfirmationRequirementV1, AccDecisionV1, AccDelegationStepV1,
    AccExecutionSemanticsV1, AccFailurePolicyV1, AccFreedomGateDecisionV1,
    AccFreedomGateRequirementV1, AccGrantStatusV1, AccPolicyCheckV1, AccPrivacyRedactionV1,
    AccRoleStandingV1, AccToolReferenceV1, AccTracePrivacyPolicyV1, AccTraceReplayV1,
    AccVisibilityPolicyV1, AdlCapabilityContractV1, ACC_SCHEMA_VERSION_V1,
};
use crate::tool_registry::{
    bind_tool_registry_v1, RegisteredToolV1, ToolBindingDecisionV1, ToolBindingRequestV1,
    ToolBindingSourceV1, ToolBindingV1, ToolRegistryV1,
};
use crate::uts::{
    validate_uts_v1, UniversalToolSchemaV1, UtsDataSensitivityV1, UtsExecutionEnvironmentKindV1,
    UtsExfiltrationRiskV1, UtsReplaySafetyV1, UtsResourceRequirementV1, UtsSideEffectClassV1,
};

fn side_effect_label(side_effect: &UtsSideEffectClassV1) -> &'static str {
    match side_effect {
        UtsSideEffectClassV1::Read => "read",
        UtsSideEffectClassV1::LocalWrite => "local_write",
        UtsSideEffectClassV1::ExternalRead => "external_read",
        UtsSideEffectClassV1::ExternalWrite => "external_write",
        UtsSideEffectClassV1::Process => "process",
        UtsSideEffectClassV1::Network => "network",
        UtsSideEffectClassV1::Destructive => "destructive",
        UtsSideEffectClassV1::Exfiltration => "exfiltration",
    }
}

fn environment_label(environment: &UtsExecutionEnvironmentKindV1) -> &'static str {
    match environment {
        UtsExecutionEnvironmentKindV1::Fixture => "fixture",
        UtsExecutionEnvironmentKindV1::DryRun => "dry_run",
        UtsExecutionEnvironmentKindV1::Local => "local",
        UtsExecutionEnvironmentKindV1::ExternalService => "external_service",
        UtsExecutionEnvironmentKindV1::Process => "process",
        UtsExecutionEnvironmentKindV1::Network => "network",
    }
}

fn first_resource(schema: &UniversalToolSchemaV1) -> Option<&UtsResourceRequirementV1> {
    schema.resources.first()
}

fn proposal_token_like(value: &str) -> bool {
    !value.trim().is_empty()
        && value.chars().all(|ch| {
            ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '-' | '_' | '.')
        })
}

fn registered_tool<'a>(
    registry: &'a ToolRegistryV1,
    tool_name: &str,
    tool_version: &str,
) -> Option<&'a RegisteredToolV1> {
    registry
        .tools
        .iter()
        .find(|tool| tool.tool_name == tool_name && tool.tool_version == tool_version)
}

fn compile_decision(policy: &UtsAccPolicyContextV1) -> AccDecisionV1 {
    match policy.grant_status {
        AccGrantStatusV1::Delegated => AccDecisionV1::Delegated,
        AccGrantStatusV1::Revoked => AccDecisionV1::Revoked,
        AccGrantStatusV1::Denied => AccDecisionV1::Denied,
        AccGrantStatusV1::Active => AccDecisionV1::Allowed,
    }
}

fn policy_evidence_ref(policy: &UtsAccPolicyContextV1) -> String {
    policy
        .delegation
        .as_ref()
        .map(|delegation| delegation.delegation_id.clone())
        .unwrap_or_else(|| policy.grant_id.clone())
}

fn build_acc(
    input: &UtsAccCompilerInputV1,
    tool: &RegisteredToolV1,
    binding: &ToolBindingV1,
) -> AdlCapabilityContractV1 {
    let schema = &tool.uts;
    let resource = first_resource(schema).expect("validated UTS should include a resource");
    let decision = compile_decision(&input.policy_context);
    let delegation_chain = input
        .policy_context
        .delegation
        .as_ref()
        .map(|delegation| {
            vec![AccDelegationStepV1 {
                delegation_id: delegation.delegation_id.clone(),
                grantor_actor_id: delegation.grantor_actor_id.clone(),
                delegate_actor_id: delegation.delegate_actor_id.clone(),
                grant_id: input.policy_context.grant_id.clone(),
                depth: delegation.depth,
            }]
        })
        .unwrap_or_default();

    AdlCapabilityContractV1 {
        schema_version: ACC_SCHEMA_VERSION_V1.to_string(),
        contract_id: format!("acc.compiler.{}", input.proposal.proposal_id),
        tool: AccToolReferenceV1 {
            tool_name: input.proposal.tool_name.clone(),
            tool_version: input.proposal.tool_version.clone(),
            registry_tool_id: binding.registry_tool_id.clone(),
            adapter_id: binding.adapter_id.clone(),
        },
        actor: AccActorIdentityV1 {
            actor_id: input.policy_context.actor_id.clone(),
            actor_kind: AccActorKindV1::Operator,
            authenticated: input.policy_context.authenticated,
            authority_evidence: vec![
                AccAuthorityEvidenceV1 {
                    evidence_id: format!("credential.{}", input.policy_context.actor_id),
                    kind: AccAuthorityEvidenceKindV1::Credential,
                    issuer: "adl.wp09.fixture".to_string(),
                },
                AccAuthorityEvidenceV1 {
                    evidence_id: "registry.wp08.binding".to_string(),
                    kind: AccAuthorityEvidenceKindV1::RegistryGrant,
                    issuer: "adl.tool_registry.v1".to_string(),
                },
            ],
        },
        authority_grant: AccAuthorityGrantV1 {
            grant_id: input.policy_context.grant_id.clone(),
            grantor_actor_id: input.policy_context.grantor_actor_id.clone(),
            grantee_actor_id: input.policy_context.actor_id.clone(),
            capability_id: binding.capability_id.clone(),
            scope: resource.scope.clone(),
            status: input.policy_context.grant_status.clone(),
            revocation_reason: None,
        },
        role_standing: AccRoleStandingV1 {
            role: input.policy_context.role.clone(),
            standing: input.policy_context.standing.clone(),
        },
        delegation_chain,
        capability: AccCapabilityRequirementV1 {
            capability_id: binding.capability_id.clone(),
            side_effect_class: side_effect_label(&schema.side_effect_class).to_string(),
            resource_type: resource.resource_type.clone(),
            resource_scope: resource.scope.clone(),
        },
        policy_checks: vec![AccPolicyCheckV1 {
            policy_id: "policy.wp09.compiler".to_string(),
            decision: decision.clone(),
            evidence_ref: policy_evidence_ref(&input.policy_context),
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
            adapter_id: binding.adapter_id.clone(),
            environment: environment_label(&schema.execution_environment.kind).to_string(),
            dry_run: binding.dry_run,
            approved_for_execution: matches!(decision, AccDecisionV1::Allowed)
                && input.policy_context.execution_approved,
        },
        trace_replay: AccTraceReplayV1 {
            trace_id: format!("trace.compiler.{}", input.proposal.proposal_id),
            replay_allowed: input.policy_context.replay_allowed,
            replay_posture: "deterministic_fixture_compiler".to_string(),
            evidence_refs: vec![
                "compiler.validation".to_string(),
                "compiler.normalization".to_string(),
                "compiler.registry_binding".to_string(),
                "compiler.policy".to_string(),
            ],
        },
        privacy_redaction: AccPrivacyRedactionV1 {
            data_sensitivity: format!("{:?}", schema.data_sensitivity).to_ascii_lowercase(),
            visibility: AccVisibilityPolicyV1 {
                actor_view: "compiled ACC request status".to_string(),
                operator_view: "full compiler fixture evidence".to_string(),
                reviewer_view: "redacted compiler evidence and policy result".to_string(),
                public_report_view: "aggregate compiler pass/fail only".to_string(),
                observatory_projection: "redacted compiler governance event".to_string(),
            },
            redaction_rules: vec!["redact_compiler_fixture_payload".to_string()],
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
            failure_code: "compiler_rejection".to_string(),
            message: "Compiler emitted a reviewable ACC or rejection.".to_string(),
            retryable: false,
        },
        decision,
    }
}

pub fn compile_uts_to_acc_v1(input: &UtsAccCompilerInputV1) -> UtsAccCompilerOutcomeV1 {
    let mut evidence_log = vec![evidence(
        UtsAccCompilerEvidenceStageV1::Validation,
        "compiler input received",
    )];

    if !proposal_token_like(&input.proposal.proposal_id)
        || !proposal_token_like(&input.proposal.tool_name)
        || input.proposal.tool_version.trim().is_empty()
        || !proposal_token_like(&input.proposal.adapter_id)
    {
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::Normalization,
            "proposal identifiers are not normalizable",
        ));
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::Policy,
            "policy not evaluated because proposal normalization failed",
        ));
        return reject(
            UtsAccCompilerRejectionCodeV1::InvalidProposal,
            "proposal identifiers must be stable repository tokens",
            evidence_log,
        );
    }

    evidence_log.push(evidence(
        UtsAccCompilerEvidenceStageV1::Normalization,
        proposal_arguments_evidence(&input.proposal.arguments),
    ));

    let binding_request = ToolBindingRequestV1 {
        source: ToolBindingSourceV1::RegistryCompiler,
        tool_name: input.proposal.tool_name.clone(),
        tool_version: input.proposal.tool_version.clone(),
        adapter_id: input.proposal.adapter_id.clone(),
        dry_run_requested: input.proposal.dry_run_requested,
    };
    let binding_outcome = bind_tool_registry_v1(&input.registry, &binding_request);
    if !matches!(binding_outcome.decision, ToolBindingDecisionV1::Bound) {
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::RegistryBinding,
            format!("registry rejected: {:?}", binding_outcome.rejection_code),
        ));
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::Policy,
            "policy not evaluated because registry binding failed",
        ));
        return reject(
            UtsAccCompilerRejectionCodeV1::RegistryBindingRejected,
            "registry binding failed before ACC construction",
            evidence_log,
        );
    }

    let Some(tool) = registered_tool(
        &input.registry,
        &input.proposal.tool_name,
        &input.proposal.tool_version,
    ) else {
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::RegistryBinding,
            "registered tool disappeared after binding",
        ));
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::Policy,
            "policy not evaluated because registry binding was inconsistent",
        ));
        return reject(
            UtsAccCompilerRejectionCodeV1::RegistryBindingRejected,
            "registered tool missing after binding",
            evidence_log,
        );
    };

    if let Err(report) = validate_uts_v1(&tool.uts) {
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::Validation,
            format!("UTS validation errors: {:?}", report.codes()),
        ));
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::Policy,
            "policy not evaluated because UTS validation failed",
        ));
        return reject(
            UtsAccCompilerRejectionCodeV1::InvalidUts,
            "UTS failed validation",
            evidence_log,
        );
    }

    evidence_log.push(evidence(
        UtsAccCompilerEvidenceStageV1::Validation,
        "UTS validation passed",
    ));

    let normalized_arguments =
        match super::normalize_tool_proposal_arguments_v1(&tool.uts, &input.proposal.arguments) {
            Ok(arguments) => arguments,
            Err(report) => {
                evidence_log.push(evidence(
                    UtsAccCompilerEvidenceStageV1::Normalization,
                    format!("argument_normalization rejected codes={:?}", report.codes()),
                ));
                evidence_log.push(evidence(
                    UtsAccCompilerEvidenceStageV1::Policy,
                    "policy not evaluated because argument normalization failed",
                ));
                return reject(
                    UtsAccCompilerRejectionCodeV1::InvalidProposal,
                    "proposal arguments failed normalization before policy evaluation",
                    evidence_log,
                );
            }
        };
    evidence_log.push(evidence(
        UtsAccCompilerEvidenceStageV1::Normalization,
        normalized_arguments_evidence(&normalized_arguments),
    ));
    evidence_log.push(evidence(
        UtsAccCompilerEvidenceStageV1::RegistryBinding,
        registry_evidence(&input.registry),
    ));

    if input.proposal.ambiguous {
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::Policy,
            "policy not evaluated because proposal is ambiguous",
        ));
        return reject(
            UtsAccCompilerRejectionCodeV1::AmbiguousProposal,
            "ambiguous proposal rejected before ACC construction",
            evidence_log,
        );
    }

    evidence_log.push(evidence(
        UtsAccCompilerEvidenceStageV1::Policy,
        "policy context evaluated",
    ));

    if tool.uts.resources.len() != 1 {
        return reject(
            UtsAccCompilerRejectionCodeV1::ResourceConstraintUnsatisfied,
            "ACC v1 compiler fixture requires exactly one representable resource scope",
            evidence_log,
        );
    }

    if !input.policy_context.authenticated
        || !matches!(
            input.policy_context.grant_status,
            AccGrantStatusV1::Active | AccGrantStatusV1::Delegated
        )
        || !input
            .policy_context
            .allowed_side_effects
            .contains(&tool.uts.side_effect_class)
    {
        return reject(
            UtsAccCompilerRejectionCodeV1::UnsatisfiableAuthority,
            "authority context does not allow this tool proposal",
            evidence_log,
        );
    }

    if tool.uts.resources.iter().any(|resource| {
        !input
            .policy_context
            .allowed_resource_scopes
            .contains(&resource.scope)
    }) {
        return reject(
            UtsAccCompilerRejectionCodeV1::ResourceConstraintUnsatisfied,
            "resource scope is not allowed by policy context",
            evidence_log,
        );
    }

    if !input.policy_context.allow_sensitive_data
        && (matches!(
            tool.uts.data_sensitivity,
            UtsDataSensitivityV1::Secret
                | UtsDataSensitivityV1::ProtectedPrompt
                | UtsDataSensitivityV1::PrivateState
        ) || matches!(tool.uts.exfiltration_risk, UtsExfiltrationRiskV1::High))
    {
        return reject(
            UtsAccCompilerRejectionCodeV1::PrivacyConstraintUnsatisfied,
            "privacy context does not allow sensitive or exfiltrating tool proposal",
            evidence_log,
        );
    }

    if !input.policy_context.visibility_constructible {
        return reject(
            UtsAccCompilerRejectionCodeV1::VisibilityConstraintUnsatisfied,
            "visibility matrix could not be constructed safely",
            evidence_log,
        );
    }

    if !input.policy_context.replay_allowed
        || !matches!(tool.uts.replay_safety, UtsReplaySafetyV1::ReplaySafe)
    {
        return reject(
            UtsAccCompilerRejectionCodeV1::ReplayConstraintUnsatisfied,
            "replay constraints are not satisfied",
            evidence_log,
        );
    }

    if !input.policy_context.execution_approved
        && !matches!(
            input.policy_context.grant_status,
            AccGrantStatusV1::Delegated
        )
    {
        return reject(
            UtsAccCompilerRejectionCodeV1::ExecutionConstraintUnsatisfied,
            "execution approval is required for non-delegated ACC construction",
            evidence_log,
        );
    }

    let binding = binding_outcome
        .binding
        .as_ref()
        .expect("bound outcome should carry binding");
    let acc = build_acc(input, tool, binding);
    if let Err(report) = validate_acc_v1(&acc) {
        evidence_log.push(evidence(
            UtsAccCompilerEvidenceStageV1::Validation,
            format!("ACC validation errors: {:?}", report.codes()),
        ));
        return reject(
            UtsAccCompilerRejectionCodeV1::ExecutionConstraintUnsatisfied,
            "compiled ACC failed validation",
            evidence_log,
        );
    }

    UtsAccCompilerOutcomeV1 {
        decision: UtsAccCompilerDecisionV1::AccEmitted,
        acc: Some(acc),
        rejection: None,
        evidence: evidence_log,
    }
}
