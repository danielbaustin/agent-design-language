use super::*;

#[test]
fn runtime_v2_transition_authority_contract_is_stable() {
    let artifacts =
        runtime_v2_transition_authority_model().expect("transition authority artifacts");
    artifacts
        .validate()
        .expect("valid transition authority artifacts");

    assert_eq!(artifacts.matrix.demo_id, "D5");
    assert_eq!(artifacts.matrix.lifecycle_states.len(), 10);
    assert_eq!(artifacts.matrix.rows.len(), 15);
    assert_eq!(artifacts.authority_basis.entries.len(), 15);
    assert_eq!(artifacts.negative_cases.required_negative_cases.len(), 6);
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_transition_authority_matrix_matches_golden_fixture() {
    let artifacts =
        runtime_v2_transition_authority_model().expect("transition authority artifacts");
    let json = String::from_utf8(artifacts.matrix.pretty_json_bytes().expect("matrix json"))
        .expect("utf8 matrix");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/contract_market/transition_authority_matrix.json"
        )
        .trim_end()
    );
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_transition_authority_basis_matches_golden_fixture() {
    let artifacts =
        runtime_v2_transition_authority_model().expect("transition authority artifacts");
    let json = String::from_utf8(
        artifacts
            .authority_basis
            .pretty_json_bytes()
            .expect("basis json"),
    )
    .expect("utf8 basis");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/contract_market/transition_authority_basis.json"
        )
        .trim_end()
    );
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_transition_authority_negative_cases_match_golden_fixture() {
    let artifacts =
        runtime_v2_transition_authority_model().expect("transition authority artifacts");
    let json = String::from_utf8(
        artifacts
            .negative_cases
            .pretty_json_bytes()
            .expect("negative cases json"),
    )
    .expect("utf8 negative cases");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/contract_market/transition_authority_negative_cases.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_transition_authority_allowed_transitions_require_explicit_actor_and_basis() {
    let artifacts =
        runtime_v2_transition_authority_model().expect("transition authority artifacts");

    for entry in &artifacts.authority_basis.entries {
        let allowed_attempt = RuntimeV2TransitionAuthorityNegativeCase {
            case_id: format!("allowed-{}", entry.transition_id),
            attempted_transition_id: entry.transition_id.clone(),
            from_state: entry.from_state.clone(),
            to_state: entry.to_state.clone(),
            actor_role: entry.actor_role.clone(),
            provided_authority_basis_ref: Some(entry.basis_ref.clone()),
            provided_artifact_refs: entry.required_artifact_refs.clone(),
            requested_tool_execution: false,
            governed_tool_authority_ref: None,
            expected_error_fragment: String::new(),
            resulting_state: "transition_refused_state_unchanged".to_string(),
            reviewable_evidence_ref: format!(
                "{RUNTIME_V2_TRANSITION_AUTHORITY_NEGATIVE_CASES_PATH}#allowed-{}",
                entry.transition_id
            ),
        };
        validate_transition_attempt(&allowed_attempt, &artifacts.authority_basis)
            .expect("allowed transition should pass");
    }
}

#[test]
fn runtime_v2_transition_authority_denied_transitions_fail_safely_and_leave_reviewable_evidence() {
    let artifacts =
        runtime_v2_transition_authority_model().expect("transition authority artifacts");

    for case in &artifacts.negative_cases.required_negative_cases {
        let err = validate_transition_attempt(case, &artifacts.authority_basis)
            .expect_err("negative case should fail");
        assert!(
            err.to_string().contains(&case.expected_error_fragment),
            "case {} failed with unexpected error {}",
            case.case_id,
            err
        );
        assert_eq!(case.resulting_state, "transition_refused_state_unchanged");
        assert!(case.reviewable_evidence_ref.contains(&case.case_id));
    }
}

#[test]
fn runtime_v2_transition_authority_records_governed_tool_boundary() {
    let artifacts =
        runtime_v2_transition_authority_model().expect("transition authority artifacts");

    assert!(artifacts
        .matrix
        .rows
        .iter()
        .all(|row| !row.tool_execution_allowed));
    assert!(artifacts
        .authority_basis
        .entries
        .iter()
        .all(|entry| !entry.tool_execution_allowed));
    assert!(artifacts
        .negative_cases
        .required_negative_cases
        .iter()
        .any(|case| case.case_id == "tool_execution_without_governed_authority"));
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_transition_authority_write_to_root_materializes_fixtures() {
    let artifacts =
        runtime_v2_transition_authority_model().expect("transition authority artifacts");
    let root = common::unique_temp_path("transition-authority-write");

    artifacts
        .write_to_root(&root)
        .expect("write transition authority artifacts");

    for rel_path in [
        RUNTIME_V2_TRANSITION_AUTHORITY_MATRIX_PATH,
        RUNTIME_V2_TRANSITION_AUTHORITY_BASIS_PATH,
        RUNTIME_V2_TRANSITION_AUTHORITY_NEGATIVE_CASES_PATH,
    ] {
        let text = std::fs::read_to_string(root.join(rel_path)).expect("artifact text");
        assert!(text.contains("D5"));
        assert!(text.contains("transition"));
        assert!(!text.contains(root.to_string_lossy().as_ref()));
    }

    std::fs::remove_dir_all(root).expect("cleanup transition authority temp root");
}
