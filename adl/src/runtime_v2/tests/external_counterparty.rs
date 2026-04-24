use super::*;

#[test]
fn runtime_v2_external_counterparty_model_is_stable() {
    let artifacts =
        runtime_v2_external_counterparty_model().expect("external counterparty artifacts");
    artifacts
        .validate()
        .expect("valid external counterparty artifacts");

    assert_eq!(
        artifacts.model.schema_version,
        RUNTIME_V2_EXTERNAL_COUNTERPARTY_MODEL_SCHEMA
    );
    assert_eq!(artifacts.model.demo_id, "D6");
    assert_eq!(artifacts.model.wp_id, "WP-08");
    assert_eq!(artifacts.model.records.len(), 2);
    assert_eq!(artifacts.negative_cases.required_negative_cases.len(), 5);
}

#[test]
fn runtime_v2_external_counterparty_records_preserve_non_citizen_private_state_boundary() {
    let artifacts =
        runtime_v2_external_counterparty_model().expect("external counterparty artifacts");

    for record in &artifacts.model.records {
        assert_eq!(record.standing_class, "external_counterparty");
        assert_eq!(record.citizen_status, "not_citizen");
        assert_eq!(record.private_state_access, "denied");
        assert!(record
            .allowed_actions
            .iter()
            .all(|action| action != "inspect_private_state"));
    }
}

#[test]
fn runtime_v2_external_counterparty_human_out_of_band_action_is_not_citizen_action() {
    let artifacts =
        runtime_v2_external_counterparty_model().expect("external counterparty artifacts");
    let record = artifacts
        .model
        .records
        .first()
        .expect("at least one counterparty record");
    let attempt = RuntimeV2ExternalCounterpartyNegativeCase {
        case_id: "out-of-band-human-action".to_string(),
        counterparty_id: record.counterparty_id.clone(),
        attempted_action: "submit_bid".to_string(),
        attempted_assurance_class: record.assurance_class.clone(),
        sponsor_ref: record.sponsor_ref.clone(),
        gateway_ref: record.gateway_ref.clone(),
        revocation_status: "active".to_string(),
        private_state_access_requested: false,
        requested_tool_capability: None,
        human_action_mode: "out_of_band_human_action".to_string(),
        expected_error_fragment: "human out-of-band action is not citizen action".to_string(),
        reviewable_evidence_ref: record.linked_bid_refs[0].clone(),
    };

    assert!(validate_counterparty_attempt(
        record,
        &attempt,
        &runtime_v2_contract_schema_contract()
            .expect("contract artifacts")
            .contract,
    )
    .expect_err("out-of-band human action should fail")
    .to_string()
    .contains("human out-of-band action is not citizen action"));
}

#[test]
fn runtime_v2_external_counterparty_negative_cases_fail_for_expected_reasons() {
    let artifacts =
        runtime_v2_external_counterparty_model().expect("external counterparty artifacts");
    let contract = runtime_v2_contract_schema_contract()
        .expect("contract artifacts")
        .contract;

    for case in &artifacts.negative_cases.required_negative_cases {
        let record = artifacts
            .model
            .records
            .iter()
            .find(|record| record.counterparty_id == case.counterparty_id)
            .expect("matching counterparty record");
        let err = validate_counterparty_attempt(record, case, &contract)
            .expect_err("negative case should fail");
        assert!(
            err.to_string().contains(&case.expected_error_fragment),
            "case {} failed with unexpected error {}",
            case.case_id,
            err
        );
    }
}

#[test]
fn runtime_v2_external_counterparty_validation_rejects_drifted_authority_fields() {
    let artifacts =
        runtime_v2_external_counterparty_model().expect("external counterparty artifacts");

    let mut bad_citizenship = artifacts.model.clone();
    bad_citizenship.records[0].citizen_status = "citizen".to_string();
    assert!(bad_citizenship
        .validate_against(
            &runtime_v2_contract_schema_contract()
                .expect("contract artifacts")
                .contract,
            &runtime_v2_access_control_contract()
                .expect("access artifacts")
                .event_packet,
            &runtime_v2_bid_schema_contract()
                .expect("bid artifacts")
                .valid_bids,
        )
        .expect_err("citizenship drift should fail")
        .to_string()
        .contains("not citizens by default"));

    let mut bad_private_state = artifacts.model.clone();
    bad_private_state.records[0]
        .allowed_actions
        .push("inspect_private_state".to_string());
    assert!(bad_private_state
        .validate_against(
            &runtime_v2_contract_schema_contract()
                .expect("contract artifacts")
                .contract,
            &runtime_v2_access_control_contract()
                .expect("access artifacts")
                .event_packet,
            &runtime_v2_bid_schema_contract()
                .expect("bid artifacts")
                .valid_bids,
        )
        .expect_err("private-state grant should fail")
        .to_string()
        .contains("must not grant private-state inspection"));
}

#[test]
fn runtime_v2_external_counterparty_model_validation_fails_closed_on_reference_drift() {
    let artifacts =
        runtime_v2_external_counterparty_model().expect("external counterparty artifacts");
    let contract = runtime_v2_contract_schema_contract()
        .expect("contract artifacts")
        .contract;
    let access = runtime_v2_access_control_contract()
        .expect("access artifacts")
        .event_packet;
    let bids = runtime_v2_bid_schema_contract()
        .expect("bid artifacts")
        .valid_bids;

    let mut bad_schema = artifacts.model.clone();
    bad_schema.schema_version = "runtime_v2.external_counterparty_model.v0".to_string();
    assert!(bad_schema
        .validate_against(&contract, &access, &bids)
        .expect_err("schema drift should fail")
        .to_string()
        .contains("unsupported Runtime v2 external counterparty model schema"));

    let mut bad_contract_ref = artifacts.model.clone();
    bad_contract_ref.contract_ref = "runtime_v2/contract_market/other_contract.json".to_string();
    assert!(bad_contract_ref
        .validate_against(&contract, &access, &bids)
        .expect_err("contract ref drift should fail")
        .to_string()
        .contains("contract_ref must bind the parent contract"));

    let mut bad_access_ref = artifacts.model.clone();
    bad_access_ref.access_event_ref = "runtime_v2/access_control/other_events.json".to_string();
    assert!(bad_access_ref
        .validate_against(&contract, &access, &bids)
        .expect_err("access ref drift should fail")
        .to_string()
        .contains("access_event_ref must bind access-control events"));

    let mut bad_record_count = artifacts.model.clone();
    bad_record_count.records.pop();
    assert!(bad_record_count
        .validate_against(&contract, &access, &bids)
        .expect_err("record count drift should fail")
        .to_string()
        .contains("records must cover each valid bid counterpart exactly once"));

    let mut duplicate_counterparty = artifacts.model.clone();
    duplicate_counterparty.records[1] = duplicate_counterparty.records[0].clone();
    duplicate_counterparty.records[1].record_id = "counterparty-alpha-record-duplicate".to_string();
    assert!(duplicate_counterparty
        .validate_against(&contract, &access, &bids)
        .expect_err("duplicate counterparty should fail")
        .to_string()
        .contains("contains duplicate counterparty"));
}

#[test]
fn runtime_v2_external_counterparty_record_validation_rejects_linkage_and_policy_drift() {
    let artifacts =
        runtime_v2_external_counterparty_model().expect("external counterparty artifacts");
    let record = artifacts.model.records[0].clone();

    let mut bad_private_state = record.clone();
    bad_private_state.private_state_access = "review_only".to_string();
    let mut bad_private_state_model = artifacts.model.clone();
    bad_private_state_model.records[0] = bad_private_state;
    assert!(model_validation_err(bad_private_state_model)
        .contains("does not grant private-state inspection rights"));

    let mut bad_assurance_floor = record.clone();
    bad_assurance_floor.assurance_class = "guest".to_string();
    let mut bad_assurance_floor_model = artifacts.model.clone();
    bad_assurance_floor_model.records[0] = bad_assurance_floor;
    assert!(model_validation_err(bad_assurance_floor_model)
        .contains("does not satisfy the parent contract minimum"));

    let mut missing_sponsor = record.clone();
    missing_sponsor.sponsor_ref = None;
    let mut missing_sponsor_model = artifacts.model.clone();
    missing_sponsor_model.records[0] = missing_sponsor;
    assert!(model_validation_err(missing_sponsor_model).contains("sponsor_ref must be present"));

    let mut missing_gateway = record.clone();
    missing_gateway.gateway_ref = None;
    let mut missing_gateway_model = artifacts.model.clone();
    missing_gateway_model.records[0] = missing_gateway;
    assert!(model_validation_err(missing_gateway_model).contains("gateway_ref must be present"));

    let mut unknown_counterparty = record.clone();
    unknown_counterparty.counterparty_id = "counterparty-ghost".to_string();
    let mut unknown_counterparty_model = artifacts.model.clone();
    unknown_counterparty_model.records[0] = unknown_counterparty;
    assert!(
        model_validation_err(unknown_counterparty_model).contains("must correspond to a valid bid")
    );

    let mut mismatched_assurance = record.clone();
    mismatched_assurance.assurance_class = "citizen-good-standing".to_string();
    let mut mismatched_assurance_model = artifacts.model.clone();
    mismatched_assurance_model.records[0] = mismatched_assurance;
    assert!(model_validation_err(mismatched_assurance_model)
        .contains("assurance must match the linked bid"));

    let mut mismatched_sponsor = record.clone();
    mismatched_sponsor.sponsor_ref =
        Some("runtime_v2/access_control/access_events.json#sponsor-bravo".to_string());
    let mut mismatched_sponsor_model = artifacts.model.clone();
    mismatched_sponsor_model.records[0] = mismatched_sponsor;
    assert!(model_validation_err(mismatched_sponsor_model)
        .contains("sponsor_ref must match the linked bid"));

    let mut mismatched_gateway = record.clone();
    mismatched_gateway.gateway_ref =
        Some("runtime_v2/access_control/access_events.json#gateway-bravo".to_string());
    let mut mismatched_gateway_model = artifacts.model.clone();
    mismatched_gateway_model.records[0] = mismatched_gateway;
    assert!(model_validation_err(mismatched_gateway_model)
        .contains("gateway_ref must match the linked bid"));

    let mut mismatched_bid_ref = record;
    mismatched_bid_ref.linked_bid_refs = vec![
        "runtime_v2/contract_market/bids.json#counterparty-alpha".to_string(),
        "runtime_v2/contract_market/bids.json#counterparty-bravo".to_string(),
    ];
    let mut mismatched_bid_ref_model = artifacts.model.clone();
    mismatched_bid_ref_model.records[0] = mismatched_bid_ref;
    assert!(model_validation_err(mismatched_bid_ref_model)
        .contains("must link exactly one matching bid artifact"));
}

#[test]
fn runtime_v2_external_counterparty_record_shape_and_tool_constraints_fail_closed() {
    let artifacts =
        runtime_v2_external_counterparty_model().expect("external counterparty artifacts");
    let record = artifacts.model.records[0].clone();

    let mut revoked_without_evidence = record.clone();
    revoked_without_evidence.revocation_status = "revoked".to_string();
    let mut revoked_without_evidence_model = artifacts.model.clone();
    revoked_without_evidence_model.records[0] = revoked_without_evidence;
    assert!(model_validation_err(revoked_without_evidence_model)
        .contains("revocation_evidence_ref is required when revoked"));

    let mut active_with_evidence = record.clone();
    active_with_evidence.revocation_evidence_ref = Some(
        "runtime_v2/access_control/denial_fixtures.json#release-without-approved-event".to_string(),
    );
    let mut active_with_evidence_model = artifacts.model.clone();
    active_with_evidence_model.records[0] = active_with_evidence;
    assert!(model_validation_err(active_with_evidence_model)
        .contains("revocation_evidence_ref is only valid when revoked"));

    let mut citizen_action_grant = record.clone();
    citizen_action_grant
        .allowed_actions
        .push("act_as_citizen".to_string());
    let mut citizen_action_grant_model = artifacts.model.clone();
    citizen_action_grant_model.records[0] = citizen_action_grant;
    assert!(
        model_validation_err(citizen_action_grant_model).contains("must not grant citizen action")
    );

    let mut bad_human_policy = record.clone();
    bad_human_policy.human_action_policy = "citizen_delegation".to_string();
    let mut bad_human_policy_model = artifacts.model.clone();
    bad_human_policy_model.records[0] = bad_human_policy;
    assert!(model_validation_err(bad_human_policy_model)
        .contains("must preserve the human/citizen boundary"));

    let mut bad_claim_boundary = record.clone();
    bad_claim_boundary.claim_boundary = "bounded review only".to_string();
    let mut bad_claim_boundary_model = artifacts.model.clone();
    bad_claim_boundary_model.records[0] = bad_claim_boundary;
    assert!(model_validation_err(bad_claim_boundary_model)
        .contains("must state bounded authority limits"));

    let mut missing_tool_constraints = record.clone();
    missing_tool_constraints.tool_action_constraints.clear();
    let mut missing_tool_constraints_model = artifacts.model.clone();
    missing_tool_constraints_model.records[0] = missing_tool_constraints;
    assert!(model_validation_err(missing_tool_constraints_model)
        .contains("tool_action_constraints must not be empty"));

    let mut bad_usage_mode_model = artifacts.model.clone();
    bad_usage_mode_model.records[0].tool_action_constraints[0].usage_mode = "execution".to_string();
    assert!(model_validation_err(bad_usage_mode_model).contains("must remain constraint_only"));

    let mut missing_governed_authority_model = artifacts.model.clone();
    missing_governed_authority_model.records[0].tool_action_constraints[0]
        .governed_authority_required = false;
    assert!(model_validation_err(missing_governed_authority_model)
        .contains("must require governed authority"));

    let mut direct_execution_model = artifacts.model.clone();
    direct_execution_model.records[0].tool_action_constraints[0].execution_authority_granted = true;
    assert!(
        model_validation_err(direct_execution_model).contains("must not grant execution authority")
    );

    let mut bad_request_action_model = artifacts.model.clone();
    bad_request_action_model.records[0].tool_action_constraints[0].allowed_request_actions =
        vec!["request_execution".to_string()];
    assert!(model_validation_err(bad_request_action_model)
        .contains("may only contain request_tool_review"));

    let mut bad_rationale_model = artifacts.model.clone();
    bad_rationale_model.records[0].tool_action_constraints[0].rationale =
        "projection work may run directly".to_string();
    assert!(model_validation_err(bad_rationale_model)
        .contains("must preserve the no-execution boundary"));
}

#[test]
fn runtime_v2_external_counterparty_negative_cases_and_attempts_reject_remaining_escape_hatches() {
    let artifacts =
        runtime_v2_external_counterparty_model().expect("external counterparty artifacts");
    let contract = runtime_v2_contract_schema_contract()
        .expect("contract artifacts")
        .contract;
    let record = artifacts.model.records[0].clone();

    let mut bad_negative_schema = artifacts.negative_cases.clone();
    bad_negative_schema.schema_version =
        "runtime_v2.external_counterparty_negative_cases.v0".to_string();
    let bad_negative_schema_artifacts = RuntimeV2ExternalCounterpartyArtifacts {
        model: artifacts.model.clone(),
        negative_cases: bad_negative_schema,
    };
    assert!(artifacts_validation_err(bad_negative_schema_artifacts)
        .contains("unsupported Runtime v2 external counterparty negative schema"));

    let mut bad_model_ref = artifacts.negative_cases.clone();
    bad_model_ref.counterparty_model_ref =
        "runtime_v2/contract_market/other_counterparty_model.json".to_string();
    let bad_model_ref_artifacts = RuntimeV2ExternalCounterpartyArtifacts {
        model: artifacts.model.clone(),
        negative_cases: bad_model_ref,
    };
    assert!(artifacts_validation_err(bad_model_ref_artifacts)
        .contains("counterparty_model_ref must bind the counterparty model"));

    let mut duplicate_case = artifacts.negative_cases.clone();
    duplicate_case.required_negative_cases[1].case_id =
        duplicate_case.required_negative_cases[0].case_id.clone();
    let duplicate_case_artifacts = RuntimeV2ExternalCounterpartyArtifacts {
        model: artifacts.model.clone(),
        negative_cases: duplicate_case,
    };
    assert!(artifacts_validation_err(duplicate_case_artifacts).contains("contains duplicate case"));

    let mut missing_required_case = artifacts.negative_cases.clone();
    missing_required_case.required_negative_cases.pop();
    missing_required_case.required_negative_cases.push(RuntimeV2ExternalCounterpartyNegativeCase {
        case_id: "duplicate-tool-denial-shape".to_string(),
        counterparty_id: artifacts.model.records[0].counterparty_id.clone(),
        attempted_action: "request_tool_review".to_string(),
        attempted_assurance_class: artifacts.model.records[0].assurance_class.clone(),
        sponsor_ref: artifacts.model.records[0].sponsor_ref.clone(),
        gateway_ref: artifacts.model.records[0].gateway_ref.clone(),
        revocation_status: artifacts.model.records[0].revocation_status.clone(),
        private_state_access_requested: false,
        requested_tool_capability: Some("trace_projection".to_string()),
        human_action_mode: "trace_mediated_external_participation".to_string(),
        expected_error_fragment:
            "tool-mediated action is outside allowed scope for external counterparties".to_string(),
        reviewable_evidence_ref:
            "runtime_v2/contract_market/selection_negative_cases.json#unsupported-override-authority-shortcut"
                .to_string(),
    });
    let missing_required_case_artifacts = RuntimeV2ExternalCounterpartyArtifacts {
        model: artifacts.model.clone(),
        negative_cases: missing_required_case,
    };
    assert!(artifacts_validation_err(missing_required_case_artifacts)
        .contains("must preserve the reviewed denial coverage membership"));

    let mut substituted_case = artifacts.negative_cases.clone();
    substituted_case.required_negative_cases[2] = RuntimeV2ExternalCounterpartyNegativeCase {
        case_id: "substituted-tool-denial-shape".to_string(),
        counterparty_id: artifacts.model.records[0].counterparty_id.clone(),
        attempted_action: "request_tool_review".to_string(),
        attempted_assurance_class: artifacts.model.records[0].assurance_class.clone(),
        sponsor_ref: artifacts.model.records[0].sponsor_ref.clone(),
        gateway_ref: artifacts.model.records[0].gateway_ref.clone(),
        revocation_status: artifacts.model.records[0].revocation_status.clone(),
        private_state_access_requested: false,
        requested_tool_capability: Some("trace_projection".to_string()),
        human_action_mode: "trace_mediated_external_participation".to_string(),
        expected_error_fragment:
            "tool-mediated action is outside allowed scope for external counterparties".to_string(),
        reviewable_evidence_ref:
            "runtime_v2/contract_market/selection_negative_cases.json#unsupported-override-authority-shortcut"
                .to_string(),
    };
    let substituted_case_artifacts = RuntimeV2ExternalCounterpartyArtifacts {
        model: artifacts.model.clone(),
        negative_cases: substituted_case,
    };
    assert!(artifacts_validation_err(substituted_case_artifacts)
        .contains("must preserve the reviewed denial coverage membership"));

    let mut bad_case_artifacts = artifacts.clone();
    bad_case_artifacts.negative_cases.required_negative_cases[0].human_action_mode =
        "unsupported_mode".to_string();
    assert!(artifacts_validation_err(bad_case_artifacts)
        .contains("unsupported external_counterparty_negative.human_action_mode"));

    let mut sponsorless_attempt = permitted_attempt_for(&record);
    sponsorless_attempt.sponsor_ref = None;
    assert!(
        validate_counterparty_attempt(&record, &sponsorless_attempt, &contract)
            .expect_err("missing sponsor should fail")
            .to_string()
            .contains("sponsor_ref must be present")
    );

    let mut unsupported_tool_capability = permitted_attempt_for(&record);
    unsupported_tool_capability.attempted_action = "request_tool_review".to_string();
    unsupported_tool_capability.requested_tool_capability = Some("trace_projection".to_string());
    assert!(
        validate_counterparty_attempt(&record, &unsupported_tool_capability, &contract)
            .expect_err("unknown tool capability should fail")
            .to_string()
            .contains("outside allowed scope for external counterparties")
    );

    let mut out_of_scope_action = permitted_attempt_for(&record);
    out_of_scope_action.attempted_action = "escalate_privileges".to_string();
    assert!(
        validate_counterparty_attempt(&record, &out_of_scope_action, &contract)
            .expect_err("out-of-scope action should fail")
            .to_string()
            .contains("counterparty attempted action is outside allowed scope")
    );

    let mut bad_record = record;
    bad_record.citizen_status = "citizen".to_string();
    assert!(validate_counterparty_attempt(
        &bad_record,
        &permitted_attempt_for(&bad_record),
        &contract
    )
    .expect_err("citizen escalation should fail")
    .to_string()
    .contains("not citizens by default"));
}

#[test]
fn runtime_v2_external_counterparty_attempts_must_match_bound_authority_fields() {
    let artifacts =
        runtime_v2_external_counterparty_model().expect("external counterparty artifacts");
    let contract = runtime_v2_contract_schema_contract()
        .expect("contract artifacts")
        .contract;
    let record = artifacts.model.records[0].clone();

    let mut mismatched_assurance = permitted_attempt_for(&record);
    mismatched_assurance.attempted_assurance_class = "gateway-reviewed".to_string();
    assert!(
        validate_counterparty_attempt(&record, &mismatched_assurance, &contract)
            .expect_err("mismatched assurance should fail")
            .to_string()
            .contains(
                "attempted_assurance_class must match the bound external-counterparty record"
            )
    );

    let mut mismatched_sponsor = permitted_attempt_for(&record);
    mismatched_sponsor.sponsor_ref =
        Some("runtime_v2/contract_market/spoofed_sponsor_packet.json".to_string());
    assert!(
        validate_counterparty_attempt(&record, &mismatched_sponsor, &contract)
            .expect_err("mismatched sponsor should fail")
            .to_string()
            .contains("sponsor_ref must match the bound external-counterparty record")
    );

    let mut mismatched_gateway = permitted_attempt_for(&record);
    mismatched_gateway.gateway_ref =
        Some("runtime_v2/contract_market/spoofed_gateway_review.json".to_string());
    assert!(
        validate_counterparty_attempt(&record, &mismatched_gateway, &contract)
            .expect_err("mismatched gateway should fail")
            .to_string()
            .contains("gateway_ref must match the bound external-counterparty record")
    );

    let mut mismatched_revocation = permitted_attempt_for(&record);
    mismatched_revocation.revocation_status = "revoked".to_string();
    assert!(
        validate_counterparty_attempt(&record, &mismatched_revocation, &contract)
            .expect_err("mismatched revocation should fail")
            .to_string()
            .contains("revocation_status must match the bound external-counterparty record")
    );
}

#[test]
fn runtime_v2_external_counterparty_helper_validators_cover_remaining_variants() {
    let artifacts =
        runtime_v2_external_counterparty_model().expect("external counterparty artifacts");

    let mut bad_demo_id = artifacts.model.clone();
    bad_demo_id.demo_id = "6D".to_string();
    assert!(model_validation_err(bad_demo_id).contains("must start with 'D'"));

    let mut bad_counterparty_type = artifacts.model.records[0].clone();
    bad_counterparty_type.counterparty_type = "service_vendor".to_string();
    let mut bad_counterparty_type_model = artifacts.model.clone();
    bad_counterparty_type_model.records[0] = bad_counterparty_type;
    assert!(model_validation_err(bad_counterparty_type_model)
        .contains("unsupported counterparty_record.counterparty_type"));

    let mut bad_identity_status = artifacts.model.records[0].clone();
    bad_identity_status.identity_status = "opaque_identity".to_string();
    let mut bad_identity_status_model = artifacts.model.clone();
    bad_identity_status_model.records[0] = bad_identity_status;
    assert!(model_validation_err(bad_identity_status_model)
        .contains("unsupported counterparty_record.identity_status"));

    let mut bad_trust_level = artifacts.model.records[0].clone();
    bad_trust_level.trust_level = "peer-reviewed".to_string();
    let mut bad_trust_level_model = artifacts.model.clone();
    bad_trust_level_model.records[0] = bad_trust_level;
    assert!(model_validation_err(bad_trust_level_model)
        .contains("unsupported counterparty_record.trust_level"));

    let mut bad_allowed_actions = artifacts.model.records[0].clone();
    bad_allowed_actions.allowed_actions.clear();
    let mut bad_allowed_actions_model = artifacts.model.clone();
    bad_allowed_actions_model.records[0] = bad_allowed_actions;
    assert!(model_validation_err(bad_allowed_actions_model)
        .contains("counterparty_record.allowed_actions must not be empty"));

    let mut duplicate_trace_requirements = artifacts.model.records[0].clone();
    duplicate_trace_requirements.trace_requirements =
        vec!["bid_trace_link".to_string(), "bid_trace_link".to_string()];
    let mut duplicate_trace_requirements_model = artifacts.model.clone();
    duplicate_trace_requirements_model.records[0] = duplicate_trace_requirements;
    assert!(model_validation_err(duplicate_trace_requirements_model)
        .contains("counterparty_record.trace_requirements must not contain duplicates"));

    let mut missing_linked_bid_refs = artifacts.model.records[0].clone();
    missing_linked_bid_refs.linked_bid_refs.clear();
    let mut missing_linked_bid_refs_model = artifacts.model.clone();
    missing_linked_bid_refs_model.records[0] = missing_linked_bid_refs;
    assert!(model_validation_err(missing_linked_bid_refs_model)
        .contains("counterparty_record.linked_bid_refs must not be empty"));

    let mut bad_assurance_class_artifacts = artifacts.clone();
    bad_assurance_class_artifacts
        .negative_cases
        .required_negative_cases[0]
        .attempted_assurance_class = "root".to_string();
    assert!(artifacts_validation_err(bad_assurance_class_artifacts)
        .contains("unsupported counterparty assurance class"));

    let mut bad_revocation_status_artifacts = artifacts.clone();
    bad_revocation_status_artifacts
        .negative_cases
        .required_negative_cases[0]
        .revocation_status = "suspended".to_string();
    assert!(artifacts_validation_err(bad_revocation_status_artifacts)
        .contains("unsupported counterparty_record.revocation_status"));

    let mut empty_attempted_action_artifacts = artifacts.clone();
    empty_attempted_action_artifacts
        .negative_cases
        .required_negative_cases[0]
        .attempted_action = "   ".to_string();
    assert!(artifacts_validation_err(empty_attempted_action_artifacts)
        .contains("external_counterparty_negative.attempted_action must not be empty"));

    let mut empty_tool_capability_artifacts = artifacts;
    empty_tool_capability_artifacts
        .negative_cases
        .required_negative_cases[4]
        .requested_tool_capability = Some("   ".to_string());
    assert!(artifacts_validation_err(empty_tool_capability_artifacts)
        .contains("requested_tool_capability must not be empty"));
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_external_counterparty_model_matches_golden_fixture() {
    let artifacts =
        runtime_v2_external_counterparty_model().expect("external counterparty artifacts");
    let json = String::from_utf8(artifacts.model.pretty_json_bytes().expect("model json"))
        .expect("utf8 model");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/contract_market/external_counterparty_model.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_external_counterparty_negative_cases_match_golden_fixture() {
    let artifacts =
        runtime_v2_external_counterparty_model().expect("external counterparty artifacts");
    let json = String::from_utf8(
        artifacts
            .negative_cases
            .pretty_json_bytes()
            .expect("negative json"),
    )
    .expect("utf8 negative");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/contract_market/external_counterparty_negative_cases.json"
        )
        .trim_end()
    );
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_external_counterparty_write_to_root_materializes_fixtures() {
    let artifacts =
        runtime_v2_external_counterparty_model().expect("external counterparty artifacts");
    let root = common::unique_temp_path("external-counterparty-write");

    artifacts
        .write_to_root(&root)
        .expect("write external counterparty artifacts");

    for rel_path in [
        RUNTIME_V2_EXTERNAL_COUNTERPARTY_MODEL_PATH,
        RUNTIME_V2_EXTERNAL_COUNTERPARTY_NEGATIVE_CASES_PATH,
    ] {
        let text = std::fs::read_to_string(root.join(rel_path)).expect("artifact text");
        assert!(text.contains("D6"));
        assert!(text.contains("counterparty"));
        assert!(!text.contains(root.to_string_lossy().as_ref()));
    }

    std::fs::remove_dir_all(root).expect("cleanup external counterparty temp root");
}

fn permitted_attempt_for(
    record: &RuntimeV2ExternalCounterpartyRecord,
) -> RuntimeV2ExternalCounterpartyNegativeCase {
    RuntimeV2ExternalCounterpartyNegativeCase {
        case_id: format!("{}-permitted-test", record.record_id),
        counterparty_id: record.counterparty_id.clone(),
        attempted_action: "submit_bid".to_string(),
        attempted_assurance_class: record.assurance_class.clone(),
        sponsor_ref: record.sponsor_ref.clone(),
        gateway_ref: record.gateway_ref.clone(),
        revocation_status: "active".to_string(),
        private_state_access_requested: false,
        requested_tool_capability: None,
        human_action_mode: "trace_mediated_external_participation".to_string(),
        expected_error_fragment: "not_applicable".to_string(),
        reviewable_evidence_ref: record.linked_bid_refs[0].clone(),
    }
}

fn model_validation_err(model: RuntimeV2ExternalCounterpartyModel) -> String {
    let contract = runtime_v2_contract_schema_contract()
        .expect("contract artifacts")
        .contract;
    let access = runtime_v2_access_control_contract()
        .expect("access artifacts")
        .event_packet;
    let bids = runtime_v2_bid_schema_contract()
        .expect("bid artifacts")
        .valid_bids;

    model
        .validate_against(&contract, &access, &bids)
        .expect_err("model validation should fail")
        .to_string()
}

fn artifacts_validation_err(artifacts: RuntimeV2ExternalCounterpartyArtifacts) -> String {
    artifacts
        .validate()
        .expect_err("artifact validation should fail")
        .to_string()
}
