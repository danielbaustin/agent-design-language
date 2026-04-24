use super::*;

#[test]
fn runtime_v2_contract_schema_contract_is_stable() {
    let artifacts = runtime_v2_contract_schema_contract().expect("contract schema artifacts");
    artifacts
        .validate()
        .expect("valid contract schema artifacts");

    assert_eq!(
        artifacts.contract.schema_version,
        RUNTIME_V2_CONTRACT_ARTIFACT_SCHEMA
    );
    assert_eq!(artifacts.contract.demo_id, "D2");
    assert_eq!(artifacts.contract.lifecycle_state, "bidding");
    assert_eq!(artifacts.contract.evaluation_criteria.len(), 3);
    assert_eq!(artifacts.negative_cases.required_negative_cases.len(), 5);
}

#[test]
fn runtime_v2_contract_schema_matches_golden_fixture() {
    let artifacts = runtime_v2_contract_schema_contract().expect("contract schema artifacts");
    let json = String::from_utf8(
        artifacts
            .contract
            .pretty_json_bytes()
            .expect("contract json"),
    )
    .expect("utf8 contract");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/contract_market/parent_contract.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_contract_schema_negative_cases_match_golden_fixture() {
    let artifacts = runtime_v2_contract_schema_contract().expect("contract schema artifacts");
    let json = String::from_utf8(
        artifacts
            .negative_cases
            .pretty_json_bytes()
            .expect("negative cases json"),
    )
    .expect("utf8 negative cases");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/contract_market/contract_negative_cases.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_contract_schema_records_tool_requirements_as_constraints() {
    let artifacts = runtime_v2_contract_schema_contract().expect("contract schema artifacts");

    assert!(artifacts
        .contract
        .tool_requirements
        .iter()
        .all(|requirement| requirement.usage_mode == "constraint"));
    assert!(artifacts
        .contract
        .tool_requirements
        .iter()
        .all(|requirement| !requirement.direct_execution_allowed));
    assert!(artifacts
        .contract
        .claim_boundary
        .contains("does not grant citizen standing"));
}

#[test]
fn runtime_v2_contract_schema_rejects_invalid_contract_fixtures_for_expected_reasons() {
    let artifacts = runtime_v2_contract_schema_contract().expect("contract schema artifacts");

    for case in &artifacts.negative_cases.required_negative_cases {
        let err = case
            .invalid_contract
            .validate()
            .expect_err("invalid contract should fail");
        assert!(
            err.to_string().contains(&case.expected_error_fragment),
            "case {} failed with unexpected error {}",
            case.case_id,
            err
        );
    }
}

#[test]
fn runtime_v2_contract_schema_write_to_root_materializes_fixtures() {
    let artifacts = runtime_v2_contract_schema_contract().expect("contract schema artifacts");
    let root = common::unique_temp_path("contract-schema-write");

    artifacts
        .write_to_root(&root)
        .expect("write contract schema artifacts");

    for rel_path in [
        RUNTIME_V2_PARENT_CONTRACT_PATH,
        RUNTIME_V2_CONTRACT_NEGATIVE_CASES_PATH,
    ] {
        let text = std::fs::read_to_string(root.join(rel_path)).expect("artifact text");
        assert!(text.contains("D2"));
        assert!(text.contains("contract"));
        assert!(!text.contains(root.to_string_lossy().as_ref()));
    }

    std::fs::remove_dir_all(root).expect("cleanup contract schema temp root");
}
