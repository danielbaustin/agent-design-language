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
