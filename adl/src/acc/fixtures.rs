use super::{
    AccActorIdentityV1, AccActorKindV1, AccAuthorityEvidenceKindV1, AccAuthorityEvidenceV1,
    AccAuthorityFixtureV1, AccAuthorityGrantV1, AccCapabilityRequirementV1,
    AccConfirmationRequirementV1, AccDecisionV1, AccDelegationStepV1, AccExecutionSemanticsV1,
    AccExpectedFixtureOutcomeV1, AccFailurePolicyV1, AccFreedomGateDecisionV1,
    AccFreedomGateRequirementV1, AccGrantStatusV1, AccPolicyCheckV1, AccPrivacyRedactionV1,
    AccRedactionExampleV1, AccRedactionSurfaceV1, AccRoleStandingV1, AccToolReferenceV1,
    AccTracePrivacyPolicyV1, AccTraceReplayV1, AccVisibilityAudienceV1, AccVisibilityLevelV1,
    AccVisibilityMatrixEntryV1, AccVisibilityPolicyV1, AdlCapabilityContractV1,
    ACC_SCHEMA_VERSION_V1,
};

pub(crate) fn base_contract(id: &'static str) -> AdlCapabilityContractV1 {
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
