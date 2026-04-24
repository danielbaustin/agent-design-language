use super::*;

#[test]
fn runtime_v2_contract_lifecycle_state_machine_is_stable() {
    let artifacts =
        runtime_v2_contract_lifecycle_state_model().expect("contract lifecycle artifacts");
    artifacts
        .validate()
        .expect("valid contract lifecycle artifacts");

    assert_eq!(
        artifacts.state_machine.schema_version,
        RUNTIME_V2_CONTRACT_LIFECYCLE_STATE_MACHINE_SCHEMA
    );
    assert_eq!(artifacts.state_machine.demo_id, "D5");
    assert_eq!(artifacts.state_machine.wp_id, "WP-07");
    assert_eq!(artifacts.state_machine.scenarios.len(), 4);
    assert_eq!(artifacts.state_machine.terminal_states.len(), 3);
    assert_eq!(artifacts.negative_cases.required_negative_cases.len(), 3);
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_contract_lifecycle_state_machine_matches_golden_fixture() {
    let artifacts =
        runtime_v2_contract_lifecycle_state_model().expect("contract lifecycle artifacts");
    let json = String::from_utf8(
        artifacts
            .state_machine
            .pretty_json_bytes()
            .expect("state machine json"),
    )
    .expect("utf8 state machine");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/contract_market/contract_lifecycle_state_machine.json"
        )
        .trim_end()
    );
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_contract_lifecycle_negative_cases_match_golden_fixture() {
    let artifacts =
        runtime_v2_contract_lifecycle_state_model().expect("contract lifecycle artifacts");
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
            "../../../tests/fixtures/runtime_v2/contract_market/contract_lifecycle_negative_cases.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_contract_lifecycle_events_include_anchor_trace_and_validation_result() {
    let artifacts =
        runtime_v2_contract_lifecycle_state_model().expect("contract lifecycle artifacts");

    for scenario in &artifacts.state_machine.scenarios {
        for event in &scenario.events {
            assert!(event.temporal_anchor_utc.ends_with('Z'));
            assert!(event.trace_link_ref.contains(".json#"));
            assert_eq!(event.validation_result, "pass");
        }
    }
}

#[test]
fn runtime_v2_contract_lifecycle_terminal_states_cannot_be_silently_reopened() {
    let artifacts =
        runtime_v2_contract_lifecycle_state_model().expect("contract lifecycle artifacts");
    let authority = runtime_v2_transition_authority_model().expect("transition authority");

    for case in &artifacts.negative_cases.required_negative_cases {
        let err = validate_terminal_reopen_attempt(case, &authority.authority_basis)
            .expect_err("terminal reopen attempt should fail");
        assert!(
            err.to_string().contains(&case.expected_error_fragment),
            "case {} failed with unexpected error {}",
            case.case_id,
            err
        );
        assert_eq!(case.resulting_state, case.prior_terminal_state);
    }
}

#[test]
fn runtime_v2_contract_lifecycle_negative_cases_require_full_terminal_reopen_matrix() {
    let artifacts =
        runtime_v2_contract_lifecycle_state_model().expect("contract lifecycle artifacts");

    let mut missing_terminal_case = artifacts.negative_cases.clone();
    missing_terminal_case.required_negative_cases.pop();
    assert!(RuntimeV2ContractLifecycleArtifacts {
        state_machine: artifacts.state_machine.clone(),
        negative_cases: missing_terminal_case,
    }
    .validate()
    .expect_err("missing terminal reopen case should fail")
    .to_string()
    .contains("must cover every terminal state exactly once"));

    let mut duplicate_terminal_case = artifacts.negative_cases.clone();
    duplicate_terminal_case.required_negative_cases[2].prior_terminal_state =
        duplicate_terminal_case.required_negative_cases[1]
            .prior_terminal_state
            .clone();
    duplicate_terminal_case.required_negative_cases[2].from_state = duplicate_terminal_case
        .required_negative_cases[1]
        .from_state
        .clone();
    duplicate_terminal_case.required_negative_cases[2].resulting_state = duplicate_terminal_case
        .required_negative_cases[1]
        .resulting_state
        .clone();
    assert!(RuntimeV2ContractLifecycleArtifacts {
        state_machine: artifacts.state_machine,
        negative_cases: duplicate_terminal_case,
    }
    .validate()
    .expect_err("duplicate terminal reopen coverage should fail")
    .to_string()
    .contains("must contain exactly one reopen denial per terminal state"));
}

#[test]
#[cfg(feature = "slow-proof-tests")]
fn runtime_v2_contract_lifecycle_write_to_root_materializes_fixtures() {
    let artifacts =
        runtime_v2_contract_lifecycle_state_model().expect("contract lifecycle artifacts");
    let root = common::unique_temp_path("contract-lifecycle-write");

    artifacts
        .write_to_root(&root)
        .expect("write contract lifecycle artifacts");

    for rel_path in [
        RUNTIME_V2_CONTRACT_LIFECYCLE_STATE_MACHINE_PATH,
        RUNTIME_V2_CONTRACT_LIFECYCLE_NEGATIVE_CASES_PATH,
    ] {
        let text = std::fs::read_to_string(root.join(rel_path)).expect("artifact text");
        assert!(text.contains("D5"));
        assert!(text.contains("terminal"));
        assert!(!text.contains(root.to_string_lossy().as_ref()));
    }

    std::fs::remove_dir_all(root).expect("cleanup contract lifecycle temp root");
}
