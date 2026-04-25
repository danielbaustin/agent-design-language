use super::*;
use std::collections::BTreeSet;

#[test]
fn runtime_v2_evaluation_selection_contract_is_stable() {
    let artifacts =
        RuntimeV2EvaluationSelectionArtifacts::prototype().expect("evaluation selection");
    artifacts.validate().expect("valid evaluation selection");

    assert_eq!(
        artifacts.selection.schema_version,
        RUNTIME_V2_EVALUATION_SELECTION_SCHEMA
    );
    assert_eq!(artifacts.selection.demo_id, "D4");
    assert_eq!(artifacts.selection.bid_evaluations.len(), 2);
    assert_eq!(artifacts.negative_cases.required_negative_cases.len(), 3);
    assert_eq!(
        required_case_ids(&artifacts.negative_cases.required_negative_cases),
        BTreeSet::from([
            "selected-bid-loses-mandatory-criterion".to_string(),
            "top-score-tie-without-rationale".to_string(),
            "unsupported-override-authority-shortcut".to_string(),
        ])
    );
    assert_eq!(
        artifacts.selection.recommendation.selected_bid_id,
        artifacts.valid_bids[0].bid_id
    );
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_evaluation_selection_matches_golden_fixture() {
    let artifacts =
        RuntimeV2EvaluationSelectionArtifacts::prototype().expect("evaluation selection");
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
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_selection_negative_cases_match_golden_fixture() {
    let artifacts =
        RuntimeV2EvaluationSelectionArtifacts::prototype().expect("evaluation selection");
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
    let artifacts =
        RuntimeV2EvaluationSelectionArtifacts::prototype().expect("evaluation selection");
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
    let artifacts =
        RuntimeV2EvaluationSelectionArtifacts::prototype().expect("evaluation selection");

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
    let artifacts =
        RuntimeV2EvaluationSelectionArtifacts::prototype().expect("evaluation selection");
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
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_evaluation_selection_write_to_root_materializes_fixtures() {
    let artifacts =
        RuntimeV2EvaluationSelectionArtifacts::prototype().expect("evaluation selection");
    let root = common::unique_temp_path("evaluation-selection-write");

    artifacts
        .write_to_root(&root)
        .expect("write evaluation selection artifacts");

    for rel_path in [
        RUNTIME_V2_EVALUATION_SELECTION_PATH,
        RUNTIME_V2_SELECTION_NEGATIVE_CASES_PATH,
    ] {
        let text = std::fs::read_to_string(root.join(rel_path)).expect("artifact text");
        assert!(text.contains("D4"));
        assert!(text.contains("selection"));
        assert!(!text.contains(root.to_string_lossy().as_ref()));
    }

    std::fs::remove_dir_all(root).expect("cleanup evaluation selection temp root");
}

#[test]
fn runtime_v2_evaluation_selection_validation_rejects_bad_demo_and_claim_boundary() {
    let artifacts =
        RuntimeV2EvaluationSelectionArtifacts::prototype().expect("evaluation selection");

    let mut bad_demo = artifacts.selection.clone();
    bad_demo.demo_id = "3".to_string();
    assert!(bad_demo
        .validate_against(&artifacts.contract, &artifacts.valid_bids)
        .expect_err("bad demo id should fail")
        .to_string()
        .contains("must start with 'D'"));

    let mut bad_claim = artifacts.selection.clone();
    bad_claim.claim_boundary = "selection writes a nice report".to_string();
    assert!(bad_claim
        .validate_against(&artifacts.contract, &artifacts.valid_bids)
        .expect_err("missing claim-boundary limits should fail")
        .to_string()
        .contains("does not settle payment"));
}

#[test]
fn runtime_v2_evaluation_selection_bid_validation_rejects_bad_tool_state() {
    let artifacts =
        RuntimeV2EvaluationSelectionArtifacts::prototype().expect("evaluation selection");
    let mut invalid = artifacts.selection.clone();
    let selected = invalid
        .bid_evaluations
        .iter_mut()
        .find(|evaluation| evaluation.bid_id == invalid.recommendation.selected_bid_id)
        .expect("selected evaluation");

    selected.tool_readiness_status = "adapter_ready_now".to_string();
    assert!(invalid
        .validate_against(&artifacts.contract, &artifacts.valid_bids)
        .expect_err("unsupported tool readiness should fail")
        .to_string()
        .contains("unsupported bid_evaluation.tool_readiness_status"));

    let mut invalid = artifacts.selection.clone();
    let selected = invalid
        .bid_evaluations
        .iter_mut()
        .find(|evaluation| evaluation.bid_id == invalid.recommendation.selected_bid_id)
        .expect("selected evaluation");
    selected.tool_readiness_notes.clear();
    assert!(invalid
        .validate_against(&artifacts.contract, &artifacts.valid_bids)
        .expect_err("missing tool readiness notes should fail")
        .to_string()
        .contains("tool_readiness_notes must explain tool readiness status"));
}

#[test]
fn runtime_v2_evaluation_selection_recommendation_requires_traceable_selection_logic() {
    let artifacts =
        RuntimeV2EvaluationSelectionArtifacts::prototype().expect("evaluation selection");

    let mut wrong_runner_up_score = artifacts.selection.clone();
    wrong_runner_up_score
        .recommendation
        .runner_up_score_basis_points += 1;
    assert!(wrong_runner_up_score
        .validate_against(&artifacts.contract, &artifacts.valid_bids)
        .expect_err("runner-up score drift should fail")
        .to_string()
        .contains("runner_up_score_basis_points must match"));

    let mut unexplained_override = artifacts.selection.clone();
    unexplained_override.recommendation.selected_bid_id =
        unexplained_override.recommendation.runner_up_bid_id.clone();
    unexplained_override.recommendation.selected_bid_ref =
        artifacts.valid_bids[1].artifact_path.clone();
    unexplained_override
        .recommendation
        .winning_score_basis_points = unexplained_override
        .recommendation
        .runner_up_score_basis_points;
    unexplained_override.recommendation.override_applied = false;
    unexplained_override.recommendation.override_rationale = None;
    assert!(unexplained_override
        .validate_against(&artifacts.contract, &artifacts.valid_bids)
        .expect_err("non-traceable override should fail")
        .to_string()
        .contains("must follow the highest-ranked bid unless a traceable override is recorded"));
}

#[test]
fn runtime_v2_evaluation_selection_rejects_unnecessary_tie_breaks_and_negative_case_drift() {
    let artifacts =
        RuntimeV2EvaluationSelectionArtifacts::prototype().expect("evaluation selection");

    let mut unnecessary_tie_break = artifacts.selection.clone();
    unnecessary_tie_break.recommendation.tie_break_rationale =
        Some("No tie exists, but pretend one does.".to_string());
    assert!(unnecessary_tie_break
        .validate_against(&artifacts.contract, &artifacts.valid_bids)
        .expect_err("tie-break rationale without tie should fail")
        .to_string()
        .contains("tie_break_rationale is only valid for top-score ties"));

    let mut stray_override_rationale = artifacts.selection.clone();
    stray_override_rationale.recommendation.override_rationale =
        Some("Operator just prefers this bid.".to_string());
    assert!(stray_override_rationale
        .validate_against(&artifacts.contract, &artifacts.valid_bids)
        .expect_err("override rationale without override should fail")
        .to_string()
        .contains("override_rationale is only valid when override_applied is true"));

    let mut bad_negative_cases = artifacts.negative_cases.clone();
    bad_negative_cases.required_negative_cases.pop();
    assert!(bad_negative_cases
        .validate_against(
            &artifacts.contract,
            &artifacts.valid_bids,
            &artifacts.selection
        )
        .expect_err("negative case count drift should fail")
        .to_string()
        .contains("must contain three required mutations"));
}

#[test]
fn runtime_v2_evaluation_selection_requires_named_negative_case_membership() {
    let artifacts =
        RuntimeV2EvaluationSelectionArtifacts::prototype().expect("evaluation selection");
    let mut invalid = artifacts.negative_cases.clone();
    invalid.required_negative_cases[0] = invalid.required_negative_cases[2].clone();
    invalid.required_negative_cases[0].case_id = "arbitrary-failing-case".to_string();

    assert!(invalid
        .validate_against(
            &artifacts.contract,
            &artifacts.valid_bids,
            &artifacts.selection
        )
        .expect_err("negative-case membership drift should fail")
        .to_string()
        .contains("must contain the required case-id set"));
}

fn required_case_ids(cases: &[RuntimeV2SelectionNegativeCase]) -> BTreeSet<String> {
    cases.iter().map(|case| case.case_id.clone()).collect()
}
