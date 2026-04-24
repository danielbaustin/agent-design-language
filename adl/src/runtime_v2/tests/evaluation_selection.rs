use super::*;

#[test]
fn runtime_v2_evaluation_selection_contract_is_stable() {
    let artifacts = runtime_v2_evaluation_selection_contract().expect("evaluation selection");
    artifacts.validate().expect("valid evaluation selection");

    assert_eq!(
        artifacts.selection.schema_version,
        RUNTIME_V2_EVALUATION_SELECTION_SCHEMA
    );
    assert_eq!(artifacts.selection.demo_id, "D3");
    assert_eq!(artifacts.selection.bid_evaluations.len(), 2);
    assert_eq!(artifacts.negative_cases.required_negative_cases.len(), 3);
    assert_eq!(
        artifacts.selection.recommendation.selected_bid_id,
        artifacts.valid_bids[0].bid_id
    );
}

#[test]
fn runtime_v2_evaluation_selection_matches_golden_fixture() {
    let artifacts = runtime_v2_evaluation_selection_contract().expect("evaluation selection");
    let json = String::from_utf8(
        artifacts
            .selection
            .pretty_json_bytes()
            .expect("selection json"),
    )
    .expect("utf8 selection");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/contract_market/evaluation_selection.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_selection_negative_cases_match_golden_fixture() {
    let artifacts = runtime_v2_evaluation_selection_contract().expect("evaluation selection");
    let json = String::from_utf8(
        artifacts
            .negative_cases
            .pretty_json_bytes()
            .expect("selection negative json"),
    )
    .expect("utf8 negative cases");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/contract_market/selection_negative_cases.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_evaluation_selection_explains_the_winner_and_keeps_tool_warning_bounded() {
    let artifacts = runtime_v2_evaluation_selection_contract().expect("evaluation selection");
    let recommendation = &artifacts.selection.recommendation;
    let selected = artifacts
        .selection
        .bid_evaluations
        .iter()
        .find(|evaluation| evaluation.bid_id == recommendation.selected_bid_id)
        .expect("selected evaluation");

    assert!(recommendation.explanation.contains("Recommend alpha"));
    assert!(!recommendation.execution_authority_granted);
    assert_eq!(
        selected.tool_readiness_status,
        "deferred_governed_tool_authority"
    );
    assert!(selected
        .tool_readiness_notes
        .iter()
        .any(|note| note.contains("authority")));
}

#[test]
fn runtime_v2_evaluation_selection_rejects_negative_cases_for_expected_reasons() {
    let artifacts = runtime_v2_evaluation_selection_contract().expect("evaluation selection");

    for case in &artifacts.negative_cases.required_negative_cases {
        let err = case
            .invalid_selection
            .validate_against(&artifacts.contract, &artifacts.valid_bids)
            .expect_err("invalid selection should fail");
        assert!(
            err.to_string().contains(&case.expected_error_fragment),
            "case {} failed with unexpected error {}",
            case.case_id,
            err
        );
    }
}

#[test]
fn runtime_v2_evaluation_selection_does_not_treat_tool_availability_as_authority() {
    let artifacts = runtime_v2_evaluation_selection_contract().expect("evaluation selection");
    let mut invalid = artifacts.selection.clone();
    invalid.recommendation.override_applied = true;
    invalid.recommendation.override_rationale = Some(
        "Adapter availability and model confidence authorize immediate execution.".to_string(),
    );

    assert!(invalid
        .validate_against(&artifacts.contract, &artifacts.valid_bids)
        .expect_err("authority shortcut should fail")
        .to_string()
        .contains("adapter availability, model confidence, or valid JSON"));
}

#[test]
fn runtime_v2_evaluation_selection_write_to_root_materializes_fixtures() {
    let artifacts = runtime_v2_evaluation_selection_contract().expect("evaluation selection");
    let root = common::unique_temp_path("evaluation-selection-write");

    artifacts
        .write_to_root(&root)
        .expect("write evaluation selection artifacts");

    for rel_path in [
        RUNTIME_V2_EVALUATION_SELECTION_PATH,
        RUNTIME_V2_SELECTION_NEGATIVE_CASES_PATH,
    ] {
        let text = std::fs::read_to_string(root.join(rel_path)).expect("artifact text");
        assert!(text.contains("D3"));
        assert!(text.contains("selection"));
        assert!(!text.contains(root.to_string_lossy().as_ref()));
    }

    std::fs::remove_dir_all(root).expect("cleanup evaluation selection temp root");
}
