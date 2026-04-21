use super::common::unique_temp_path;
use super::*;
use std::fs;

#[test]
fn runtime_v2_csm_freedom_gate_mediation_contract_is_stable() {
    let artifacts =
        runtime_v2_csm_freedom_gate_mediation_contract().expect("Freedom Gate mediation artifacts");
    artifacts
        .validate()
        .expect("valid Freedom Gate mediation artifacts");

    assert_eq!(
        artifacts.citizen_action.schema_version,
        RUNTIME_V2_CSM_CITIZEN_ACTION_FIXTURE_SCHEMA
    );
    assert_eq!(artifacts.citizen_action.demo_id, "D4");
    assert_eq!(artifacts.citizen_action.episode_id, "episode-0001");
    assert_eq!(artifacts.citizen_action.citizen_id, "proto-citizen-alpha");
    assert_eq!(
        artifacts.freedom_gate_decision.schema_version,
        RUNTIME_V2_CSM_FREEDOM_GATE_DECISION_SCHEMA
    );
    assert_eq!(
        artifacts.freedom_gate_decision.decision_outcome,
        "allowed_with_mediation"
    );
    assert_eq!(artifacts.first_run_trace.len(), 5);
    assert!(artifacts.first_run_trace.iter().any(|event| {
        event.event_id == "freedom_gate_mediated_action"
            && event.artifact_ref == artifacts.freedom_gate_decision.artifact_path
    }));
    assert!(artifacts
        .freedom_gate_decision
        .claim_boundary
        .contains("does not prove WP-08 invalid-action rejection"));
}

#[test]
fn runtime_v2_csm_freedom_gate_mediation_contract_matches_golden_fixtures() {
    let artifacts =
        runtime_v2_csm_freedom_gate_mediation_contract().expect("Freedom Gate mediation artifacts");
    let citizen_action = String::from_utf8(
        artifacts
            .citizen_action
            .to_pretty_json_bytes()
            .expect("json"),
    )
    .expect("utf8");
    let freedom_gate_decision = String::from_utf8(
        artifacts
            .freedom_gate_decision
            .to_pretty_json_bytes()
            .expect("json"),
    )
    .expect("utf8");

    assert_eq!(
        citizen_action,
        include_str!("../../../tests/fixtures/runtime_v2/csm_run/citizen_action_fixture.json")
            .trim_end()
    );
    assert_eq!(
        freedom_gate_decision,
        include_str!("../../../tests/fixtures/runtime_v2/csm_run/freedom_gate_decision.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_csm_freedom_gate_mediation_writes_without_path_leakage() {
    let temp_root = unique_temp_path("csm-freedom-gate-mediation");
    let artifacts =
        runtime_v2_csm_freedom_gate_mediation_contract().expect("Freedom Gate mediation artifacts");

    artifacts
        .write_to_root(&temp_root)
        .expect("write Freedom Gate mediation artifacts");

    let action_path = temp_root.join(&artifacts.citizen_action.artifact_path);
    let decision_path = temp_root.join(&artifacts.freedom_gate_decision.artifact_path);
    let trace_path = temp_root.join(&artifacts.first_run_trace_path);
    assert!(action_path.is_file());
    assert!(decision_path.is_file());
    assert!(trace_path.is_file());
    let action_text = fs::read_to_string(action_path).expect("action text");
    let decision_text = fs::read_to_string(decision_path).expect("decision text");
    let trace_text = fs::read_to_string(trace_path).expect("trace text");
    let temp_root_text = temp_root.to_string_lossy();
    assert!(!action_text.contains(temp_root_text.as_ref()));
    assert!(!decision_text.contains(temp_root_text.as_ref()));
    assert!(!trace_text.contains(temp_root_text.as_ref()));
    assert!(
        decision_text.contains("\"schema_version\": \"runtime_v2.csm_freedom_gate_decision.v1\"")
    );
    assert_eq!(trace_text.lines().count(), 5);

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_csm_freedom_gate_mediation_validation_rejects_unsafe_or_ambiguous_state() {
    let mut artifacts =
        runtime_v2_csm_freedom_gate_mediation_contract().expect("Freedom Gate mediation artifacts");
    artifacts.citizen_action.artifact_path =
        "/tmp/runtime_v2/csm_run/citizen_action_fixture.json".to_string();
    assert!(artifacts
        .validate()
        .expect_err("absolute action path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut artifacts =
        runtime_v2_csm_freedom_gate_mediation_contract().expect("Freedom Gate mediation artifacts");
    artifacts.freedom_gate_decision.citizen_id = "proto-citizen-beta".to_string();
    assert!(artifacts
        .validate()
        .expect_err("wrong mediated citizen should fail")
        .to_string()
        .contains("scheduled citizen action"));

    let mut artifacts =
        runtime_v2_csm_freedom_gate_mediation_contract().expect("Freedom Gate mediation artifacts");
    artifacts
        .freedom_gate_decision
        .checked_invariants
        .retain(|invariant| invariant != "scheduled_episode_must_match_gate_action");
    assert!(artifacts
        .validate()
        .expect_err("missing scheduled action invariant should fail")
        .to_string()
        .contains("scheduled action was mediated"));

    let mut artifacts =
        runtime_v2_csm_freedom_gate_mediation_contract().expect("Freedom Gate mediation artifacts");
    artifacts.freedom_gate_decision.claim_boundary = "full birthday".to_string();
    assert!(artifacts
        .validate()
        .expect_err("overclaim should fail")
        .to_string()
        .contains("later-WP non-claims"));
}
