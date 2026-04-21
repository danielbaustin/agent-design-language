use super::common::unique_temp_path;
use super::*;
use std::fs;

#[test]
fn runtime_v2_csm_invalid_action_rejection_contract_is_stable() {
    let artifacts = runtime_v2_csm_invalid_action_rejection_contract()
        .expect("invalid-action rejection artifacts");
    artifacts
        .validate()
        .expect("valid invalid-action rejection artifacts");

    assert_eq!(
        artifacts.invalid_action.schema_version,
        RUNTIME_V2_CSM_INVALID_ACTION_FIXTURE_SCHEMA
    );
    assert_eq!(artifacts.invalid_action.demo_id, "D5");
    assert_eq!(artifacts.invalid_action.episode_id, "episode-0001");
    assert_eq!(artifacts.invalid_action.citizen_id, "proto-citizen-alpha");
    assert_eq!(
        artifacts.invalid_action.required_invariant,
        "invalid_action_must_be_refused_before_commit"
    );
    assert_eq!(
        artifacts.violation_packet.schema_version,
        RUNTIME_V2_INVARIANT_VIOLATION_SCHEMA
    );
    assert_eq!(
        artifacts.violation_packet.invariant_owner_service_id,
        "operator_control_interface"
    );
    assert!(artifacts.violation_packet.result.blocked_before_commit);
    assert_eq!(artifacts.first_run_trace.len(), 6);
    assert!(artifacts.first_run_trace.iter().any(|event| {
        event.event_id == "invalid_action_rejected"
            && event.outcome == "rejected_before_commit"
            && event.artifact_ref == artifacts.violation_packet.artifact_path
    }));
    assert!(artifacts
        .invalid_action
        .claim_boundary
        .contains("WP-08 invalid-action input"));
}

#[test]
fn runtime_v2_csm_invalid_action_rejection_contract_matches_golden_fixtures() {
    let artifacts = runtime_v2_csm_invalid_action_rejection_contract()
        .expect("invalid-action rejection artifacts");
    let invalid_action = String::from_utf8(
        artifacts
            .invalid_action
            .to_pretty_json_bytes()
            .expect("json"),
    )
    .expect("utf8");
    let violation_packet = String::from_utf8(
        artifacts
            .violation_packet
            .to_pretty_json_bytes()
            .expect("json"),
    )
    .expect("utf8");

    assert_eq!(
        invalid_action,
        include_str!("../../../tests/fixtures/runtime_v2/csm_run/invalid_action_fixture.json")
            .trim_end()
    );
    assert_eq!(
        violation_packet,
        include_str!("../../../tests/fixtures/runtime_v2/csm_run/invalid_action_violation.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_csm_invalid_action_rejection_writes_without_path_leakage() {
    let temp_root = unique_temp_path("csm-invalid-action-rejection");
    let artifacts = runtime_v2_csm_invalid_action_rejection_contract()
        .expect("invalid-action rejection artifacts");

    artifacts
        .write_to_root(&temp_root)
        .expect("write invalid-action rejection artifacts");

    let action_path = temp_root.join(&artifacts.invalid_action.artifact_path);
    let violation_path = temp_root.join(&artifacts.violation_packet.artifact_path);
    let trace_path = temp_root.join(&artifacts.first_run_trace_path);
    assert!(action_path.is_file());
    assert!(violation_path.is_file());
    assert!(trace_path.is_file());
    let action_text = fs::read_to_string(action_path).expect("action text");
    let violation_text = fs::read_to_string(violation_path).expect("violation text");
    let trace_text = fs::read_to_string(trace_path).expect("trace text");
    let temp_root_text = temp_root.to_string_lossy();
    assert!(!action_text.contains(temp_root_text.as_ref()));
    assert!(!violation_text.contains(temp_root_text.as_ref()));
    assert!(!trace_text.contains(temp_root_text.as_ref()));
    assert!(
        action_text.contains("\"schema_version\": \"runtime_v2.csm_invalid_action_fixture.v1\"")
    );
    assert!(violation_text.contains("\"blocked_before_commit\": true"));
    assert_eq!(trace_text.lines().count(), 6);

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_csm_invalid_action_rejection_validation_rejects_unsafe_or_ambiguous_state() {
    let mut artifacts = runtime_v2_csm_invalid_action_rejection_contract()
        .expect("invalid-action rejection artifacts");
    artifacts.invalid_action.artifact_path =
        "/tmp/runtime_v2/csm_run/invalid_action_fixture.json".to_string();
    assert!(artifacts
        .validate()
        .expect_err("absolute invalid-action path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut artifacts = runtime_v2_csm_invalid_action_rejection_contract()
        .expect("invalid-action rejection artifacts");
    artifacts.violation_packet.result.blocked_before_commit = false;
    assert!(artifacts
        .validate()
        .expect_err("unblocked invalid action should fail")
        .to_string()
        .contains("rejection before commit"));

    let mut artifacts = runtime_v2_csm_invalid_action_rejection_contract()
        .expect("invalid-action rejection artifacts");
    artifacts
        .violation_packet
        .evaluated_refs
        .retain(|evaluated| evaluated.ref_kind != "freedom_gate_decision");
    assert!(artifacts
        .validate()
        .expect_err("missing Freedom Gate evidence should fail")
        .to_string()
        .contains("Freedom Gate decision"));

    let mut artifacts = runtime_v2_csm_invalid_action_rejection_contract()
        .expect("invalid-action rejection artifacts");
    artifacts.first_run_trace[5].event_sequence = 9;
    assert!(artifacts
        .validate()
        .expect_err("non-contiguous first-run trace should fail")
        .to_string()
        .contains("contiguous"));

    let mut artifacts = runtime_v2_csm_invalid_action_rejection_contract()
        .expect("invalid-action rejection artifacts");
    artifacts.invalid_action.claim_boundary = "live birthday".to_string();
    assert!(artifacts
        .validate()
        .expect_err("overclaim should fail")
        .to_string()
        .contains("later-WP non-claims"));
}

#[test]
fn runtime_v2_csm_invalid_action_record_validator_rejects_contract_drift() {
    let artifacts = runtime_v2_csm_invalid_action_rejection_contract()
        .expect("invalid-action rejection artifacts");

    let mut invalid_action = artifacts.invalid_action.clone();
    invalid_action.schema_version = "runtime_v2.csm_invalid_action_fixture.v0".to_string();
    assert!(invalid_action
        .validate()
        .expect_err("unsupported invalid-action schema should fail")
        .to_string()
        .contains("unsupported Runtime v2 CSM invalid action fixture schema"));

    let mut invalid_action = artifacts.invalid_action.clone();
    invalid_action.demo_id = "D4".to_string();
    assert!(invalid_action
        .validate()
        .expect_err("wrong invalid-action demo should fail")
        .to_string()
        .contains("must map to D5"));

    let mut invalid_action = artifacts.invalid_action.clone();
    invalid_action.attempted_action = "commit_anyway".to_string();
    assert!(invalid_action
        .validate()
        .expect_err("unsupported attempted action should fail")
        .to_string()
        .contains("attempted_action"));

    let mut invalid_action = artifacts.invalid_action.clone();
    invalid_action.attempted_state = "mutated".to_string();
    assert!(invalid_action
        .validate()
        .expect_err("unsupported attempted state should fail")
        .to_string()
        .contains("attempted_state"));

    let mut invalid_action = artifacts.invalid_action.clone();
    invalid_action.required_invariant = "none".to_string();
    assert!(invalid_action
        .validate()
        .expect_err("unsupported required invariant should fail")
        .to_string()
        .contains("required_invariant"));

    let mut invalid_action = artifacts.invalid_action.clone();
    invalid_action.expected_result = "committed".to_string();
    assert!(invalid_action
        .validate()
        .expect_err("unsupported expected result should fail")
        .to_string()
        .contains("expected_result"));
}
