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
fn acc_v1_1_upgraded_allowed_authority_fixture_passes() {
    let contract = upgrade_acc_v1_to_v1_1(base_contract("acc.fixture.allowed_safe_read"));

    validate_acc_v1_1(&contract).expect("upgraded authority fixture should pass");
    assert_eq!(contract.schema_version, ACC_SCHEMA_VERSION_V1_1);
    assert_eq!(
        contract
            .compatible_versions
            .as_ref()
            .expect("compatible versions"),
        &vec!["acc.v1".to_string(), "acc.v1.1".to_string()]
    );
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

    let err = validate_acc_v1(&contract).expect_err("misattributed delegation chain should fail");

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
    assert!(visibility_is_complete(
        &base_contract("acc.fixture.visibility")
            .privacy_redaction
            .visibility
    ));
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
        .and_then(serde_json::Value::as_object)
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
fn acc_v1_1_schema_generation_exposes_additive_governance_surface() {
    let schema = acc_v1_1_schema_json();
    let properties = schema
        .get("properties")
        .and_then(serde_json::Value::as_object)
        .expect("generated ACC v1.1 schema should expose properties");

    for key in [
        "schema_version",
        "compatible_versions",
        "governance_profile",
        "delegation_constraints",
    ] {
        assert!(
            properties.contains_key(key),
            "ACC v1.1 schema missing property {key}"
        );
    }
}

#[test]
fn acc_v1_rejects_unknown_runtime_authority_fields() {
    let mut value = serde_json::to_value(base_contract("acc.fixture.unknown_field")).expect("json");
    value["model_confidence_grants_authority"] = json!(true);

    let err = serde_json::from_value::<AdlCapabilityContractV1>(value)
        .expect_err("unknown authority field should not deserialize");

    assert!(err.to_string().contains("unknown field"));
}
