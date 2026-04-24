use super::*;

#[test]
fn runtime_v2_delegation_subcontract_artifacts_are_stable() {
    let artifacts =
        runtime_v2_delegation_subcontract_model().expect("delegation subcontract artifacts");
    artifacts
        .validate()
        .expect("valid delegation subcontract artifacts");

    assert_eq!(
        artifacts.subcontract.schema_version,
        RUNTIME_V2_SUBCONTRACT_ARTIFACT_SCHEMA
    );
    assert_eq!(artifacts.subcontract.demo_id, "D7");
    assert_eq!(artifacts.subcontract.wp_id, "WP-09");
    assert_eq!(artifacts.delegated_output.demo_id, "D7");
    assert_eq!(artifacts.parent_integration.demo_id, "D7");
    assert_eq!(artifacts.negative_cases.required_negative_cases.len(), 5);
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_delegation_subcontract_matches_golden_fixture() {
    let artifacts =
        runtime_v2_delegation_subcontract_model().expect("delegation subcontract artifacts");
    let json = String::from_utf8(
        artifacts
            .subcontract
            .pretty_json_bytes()
            .expect("subcontract json"),
    )
    .expect("utf8 subcontract");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/contract_market/delegation_subcontract.json"
        )
        .trim_end()
    );
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_delegated_output_matches_golden_fixture() {
    let artifacts =
        runtime_v2_delegation_subcontract_model().expect("delegation subcontract artifacts");
    let json = String::from_utf8(
        artifacts
            .delegated_output
            .pretty_json_bytes()
            .expect("delegated output json"),
    )
    .expect("utf8 delegated output");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/contract_market/delegated_output.json")
            .trim_end()
    );
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_parent_integration_matches_golden_fixture() {
    let artifacts =
        runtime_v2_delegation_subcontract_model().expect("delegation subcontract artifacts");
    let json = String::from_utf8(
        artifacts
            .parent_integration
            .pretty_json_bytes()
            .expect("parent integration json"),
    )
    .expect("utf8 parent integration");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/contract_market/parent_integration.json")
            .trim_end()
    );
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_delegation_negative_cases_match_golden_fixture() {
    let artifacts =
        runtime_v2_delegation_subcontract_model().expect("delegation subcontract artifacts");
    let json = String::from_utf8(
        artifacts
            .negative_cases
            .pretty_json_bytes()
            .expect("delegation negative cases json"),
    )
    .expect("utf8 delegation negative cases");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/contract_market/delegation_negative_cases.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_delegation_subcontract_preserves_authority_and_parent_accountability() {
    let artifacts =
        runtime_v2_delegation_subcontract_model().expect("delegation subcontract artifacts");

    assert!(!artifacts.subcontract.inherited_parent_authority);
    assert!(artifacts.subcontract.parent_review_required);
    assert!(artifacts.subcontract.parent_responsibility_retained);
    assert_eq!(
        artifacts.subcontract.delegating_counterparty_id,
        "counterparty-alpha"
    );
    assert_eq!(
        artifacts.subcontract.subcontractor_selection_basis_ref,
        "runtime_v2/contract_market/evaluation_selection.json#runner-up-bid"
    );
    assert_eq!(
        artifacts.subcontract.subcontractor_counterparty_id,
        "counterparty-bravo"
    );
    assert_eq!(
        artifacts.parent_integration.parent_responsibility_status,
        "retained_and_reviewable"
    );
}

#[test]
fn runtime_v2_delegation_subcontract_selection_basis_is_order_independent() {
    let artifacts =
        runtime_v2_delegation_subcontract_model().expect("delegation subcontract artifacts");
    let contract = runtime_v2_contract_schema_contract()
        .expect("contract artifacts")
        .contract;
    let selection_artifacts =
        RuntimeV2EvaluationSelectionArtifacts::prototype().expect("selection artifacts");
    let mut reordered_bids = selection_artifacts.valid_bids.clone();
    reordered_bids.reverse();
    let mut reordered_counterparties = runtime_v2_external_counterparty_model()
        .expect("counterparty artifacts")
        .model;
    reordered_counterparties.records.reverse();

    artifacts
        .subcontract
        .validate_against(
            &contract,
            &selection_artifacts.selection,
            &reordered_bids,
            &reordered_counterparties,
        )
        .expect("runner-up selection basis should not depend on bid or record order");
    assert_eq!(
        artifacts.subcontract.subcontractor_counterparty_id,
        "counterparty-bravo"
    );
}

#[test]
fn runtime_v2_delegation_subcontract_tool_constraints_remain_non_authoritative() {
    let artifacts =
        runtime_v2_delegation_subcontract_model().expect("delegation subcontract artifacts");

    assert!(artifacts
        .subcontract
        .delegated_tool_constraints
        .iter()
        .all(|constraint| constraint.governed_authority_required));
    assert!(artifacts
        .subcontract
        .delegated_tool_constraints
        .iter()
        .all(|constraint| !constraint.execution_authority_granted));
    assert!(artifacts
        .delegated_output
        .delegated_tool_usage
        .iter()
        .all(|constraint| !constraint.execution_authority_granted));
}

#[test]
fn runtime_v2_delegation_subcontract_negative_cases_fail_for_expected_reasons() {
    let artifacts =
        runtime_v2_delegation_subcontract_model().expect("delegation subcontract artifacts");
    let contract = runtime_v2_contract_schema_contract()
        .expect("contract artifacts")
        .contract;
    let selection = RuntimeV2EvaluationSelectionArtifacts::prototype()
        .expect("selection artifacts")
        .selection;
    let bids = runtime_v2_bid_schema_contract()
        .expect("bid artifacts")
        .valid_bids;
    let counterparties = runtime_v2_external_counterparty_model()
        .expect("counterparty artifacts")
        .model;

    for case in &artifacts.negative_cases.required_negative_cases {
        validate_negative_case(case, &contract, &selection, &bids, &counterparties)
            .expect("negative case should fail for the expected reason");
    }
}

#[test]
fn runtime_v2_delegation_subcontract_rejects_reference_and_review_drift() {
    let artifacts =
        runtime_v2_delegation_subcontract_model().expect("delegation subcontract artifacts");
    let contract = runtime_v2_contract_schema_contract()
        .expect("contract artifacts")
        .contract;
    let selection_artifacts =
        RuntimeV2EvaluationSelectionArtifacts::prototype().expect("selection artifacts");
    let counterparties = runtime_v2_external_counterparty_model()
        .expect("counterparty artifacts")
        .model;

    let mut bad_selection_ref = artifacts.subcontract.clone();
    bad_selection_ref.selection_ref =
        "runtime_v2/contract_market/other_evaluation_selection.json".to_string();
    assert!(bad_selection_ref
        .validate_against(
            &contract,
            &selection_artifacts.selection,
            &selection_artifacts.valid_bids,
            &counterparties,
        )
        .expect_err("selection drift should fail")
        .to_string()
        .contains("selection_ref must bind the selection artifact"));

    let mut bad_basis_ref = artifacts.subcontract.clone();
    bad_basis_ref.subcontractor_selection_basis_ref =
        "runtime_v2/contract_market/evaluation_selection.json#selected-bid".to_string();
    assert!(bad_basis_ref
        .validate_against(
            &contract,
            &selection_artifacts.selection,
            &selection_artifacts.valid_bids,
            &counterparties,
        )
        .expect_err("subcontractor basis drift should fail")
        .to_string()
        .contains(
            "subcontractor_selection_basis_ref must bind the selection artifact runner-up bid"
        ));

    let mut bad_record_ref = artifacts.subcontract.clone();
    bad_record_ref.subcontractor_record_ref =
        "runtime_v2/contract_market/external_counterparty_model.json#counterparty-alpha-record"
            .to_string();
    assert!(bad_record_ref
        .validate_against(
            &contract,
            &selection_artifacts.selection,
            &selection_artifacts.valid_bids,
            &counterparties,
        )
        .expect_err("subcontractor record drift should fail")
        .to_string()
        .contains("subcontract.subcontractor_record_ref must bind the subcontractor record"));

    let mut bad_output = artifacts.delegated_output.clone();
    bad_output.review_status = "submitted_for_parent_review".to_string();
    bad_output.parent_review_ref = Some(
        "runtime_v2/contract_market/evaluation_selection.json#delegated-review-approved"
            .to_string(),
    );
    assert!(bad_output
        .validate_against(&artifacts.subcontract, &contract)
        .expect_err("pending review output should not claim approval ref")
        .to_string()
        .contains("awaiting review must not claim a parent review ref"));

    let mut bad_integration = artifacts.parent_integration.clone();
    bad_integration.parent_responsibility_status = "transferred".to_string();
    assert!(bad_integration
        .validate_against(
            &artifacts.subcontract,
            &artifacts.delegated_output,
            &contract
        )
        .expect_err("responsibility transfer should fail")
        .to_string()
        .contains("must preserve retained accountability"));
}

#[test]
fn runtime_v2_delegation_subcontract_rejects_subcontractor_outside_runner_up_basis() {
    let artifacts =
        runtime_v2_delegation_subcontract_model().expect("delegation subcontract artifacts");
    let contract = runtime_v2_contract_schema_contract()
        .expect("contract artifacts")
        .contract;
    let selection_artifacts =
        RuntimeV2EvaluationSelectionArtifacts::prototype().expect("selection artifacts");
    let mut counterparties = runtime_v2_external_counterparty_model()
        .expect("counterparty artifacts")
        .model;
    let mut extra_record = counterparties
        .records
        .iter()
        .find(|record| record.counterparty_id == "counterparty-bravo")
        .expect("bravo counterparty")
        .clone();
    extra_record.record_id = "counterparty-charlie-record".to_string();
    extra_record.counterparty_id = "counterparty-charlie".to_string();
    extra_record.linked_bid_refs.clear();
    counterparties.records.push(extra_record);

    let mut wrong_subcontractor = artifacts.subcontract.clone();
    wrong_subcontractor.subcontractor_counterparty_id = "counterparty-charlie".to_string();
    wrong_subcontractor.subcontractor_record_ref =
        "runtime_v2/contract_market/external_counterparty_model.json#counterparty-charlie-record"
            .to_string();

    assert!(wrong_subcontractor
        .validate_against(
            &contract,
            &selection_artifacts.selection,
            &selection_artifacts.valid_bids,
            &counterparties,
        )
        .expect_err("selected bidder should not satisfy runner-up subcontractor basis")
        .to_string()
        .contains("subcontractor must match the runner-up bid counterparty selected by subcontractor_selection_basis_ref"));
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_delegation_subcontract_write_to_root_materializes_fixtures() {
    let artifacts =
        runtime_v2_delegation_subcontract_model().expect("delegation subcontract artifacts");
    let fixture_refresh_root = std::env::var("ADL_RUNTIME_V2_WRITE_ROOT").ok();
    let root = fixture_refresh_root
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| common::unique_temp_path("delegation-subcontract-write"));

    artifacts
        .write_to_root(&root)
        .expect("write delegation subcontract artifacts");

    for rel_path in [
        RUNTIME_V2_SUBCONTRACT_ARTIFACT_PATH,
        RUNTIME_V2_DELEGATED_OUTPUT_ARTIFACT_PATH,
        RUNTIME_V2_PARENT_INTEGRATION_ARTIFACT_PATH,
        RUNTIME_V2_DELEGATION_NEGATIVE_CASES_PATH,
    ] {
        let text = std::fs::read_to_string(root.join(rel_path)).expect("artifact text");
        assert!(text.contains("D7"));
        assert!(!text.contains(root.to_string_lossy().as_ref()));
    }

    if fixture_refresh_root.is_none() {
        std::fs::remove_dir_all(root).expect("cleanup delegation subcontract temp root");
    }
}
