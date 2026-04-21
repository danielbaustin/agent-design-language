use super::common::unique_temp_path;
use super::*;
use std::fs;

#[test]
fn runtime_v2_csm_governed_episode_contract_is_stable() {
    let artifacts = runtime_v2_csm_governed_episode_contract().expect("governed episode artifacts");
    artifacts
        .validate()
        .expect("valid governed episode artifacts");

    assert_eq!(
        artifacts.resource_pressure.schema_version,
        RUNTIME_V2_CSM_RESOURCE_PRESSURE_SCHEMA
    );
    assert_eq!(artifacts.resource_pressure.demo_id, "D4");
    assert_eq!(artifacts.resource_pressure.manifold_id, "proto-csm-01");
    assert_eq!(
        artifacts.resource_pressure.artifact_path,
        "runtime_v2/csm_run/resource_pressure_fixture.json"
    );
    assert_eq!(artifacts.resource_pressure.candidates.len(), 2);
    assert!(
        artifacts.resource_pressure.requested_compute_tokens
            > artifacts.resource_pressure.available_compute_tokens
    );
    assert_eq!(
        artifacts.scheduling_decision.schema_version,
        RUNTIME_V2_CSM_SCHEDULING_DECISION_SCHEMA
    );
    assert_eq!(
        artifacts.scheduling_decision.selected_episode_id,
        "episode-0001"
    );
    assert_eq!(
        artifacts.scheduling_decision.selected_citizen_id,
        "proto-citizen-alpha"
    );
    assert_eq!(artifacts.first_run_trace.len(), 4);
    assert!(artifacts
        .scheduling_decision
        .claim_boundary
        .contains("WP-06 resource scheduling only"));
}

#[test]
fn runtime_v2_csm_governed_episode_contract_matches_golden_fixtures() {
    let artifacts = runtime_v2_csm_governed_episode_contract().expect("governed episode artifacts");
    let resource_pressure = String::from_utf8(
        artifacts
            .resource_pressure
            .to_pretty_json_bytes()
            .expect("json"),
    )
    .expect("utf8");
    let scheduling_decision = String::from_utf8(
        artifacts
            .scheduling_decision
            .to_pretty_json_bytes()
            .expect("json"),
    )
    .expect("utf8");

    assert_eq!(
        resource_pressure,
        include_str!("../../../tests/fixtures/runtime_v2/csm_run/resource_pressure_fixture.json")
            .trim_end()
    );
    assert_eq!(
        scheduling_decision,
        include_str!("../../../tests/fixtures/runtime_v2/csm_run/scheduling_decision.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_csm_governed_episode_writes_without_path_leakage() {
    let temp_root = unique_temp_path("csm-governed-episode");
    let artifacts = runtime_v2_csm_governed_episode_contract().expect("governed episode artifacts");

    artifacts
        .write_to_root(&temp_root)
        .expect("write governed episode artifacts");

    let pressure_path = temp_root.join(&artifacts.resource_pressure.artifact_path);
    let decision_path = temp_root.join(&artifacts.scheduling_decision.artifact_path);
    let trace_path = temp_root.join(&artifacts.first_run_trace_path);
    assert!(pressure_path.is_file());
    assert!(decision_path.is_file());
    assert!(trace_path.is_file());
    let pressure_text = fs::read_to_string(pressure_path).expect("pressure text");
    let decision_text = fs::read_to_string(decision_path).expect("decision text");
    let trace_text = fs::read_to_string(trace_path).expect("trace text");
    let temp_root_text = temp_root.to_string_lossy();
    assert!(!pressure_text.contains(temp_root_text.as_ref()));
    assert!(!decision_text.contains(temp_root_text.as_ref()));
    assert!(!trace_text.contains(temp_root_text.as_ref()));
    assert!(pressure_text
        .contains("\"schema_version\": \"runtime_v2.csm_resource_pressure_fixture.v1\""));
    assert_eq!(trace_text.lines().count(), 4);

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_csm_governed_episode_validation_rejects_unsafe_or_ambiguous_state() {
    let mut artifacts =
        runtime_v2_csm_governed_episode_contract().expect("governed episode artifacts");
    artifacts.resource_pressure.artifact_path =
        "/tmp/runtime_v2/csm_run/resource_pressure_fixture.json".to_string();
    assert!(artifacts
        .validate()
        .expect_err("absolute pressure path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut artifacts =
        runtime_v2_csm_governed_episode_contract().expect("governed episode artifacts");
    artifacts.resource_pressure.candidates[1].can_execute_episodes = true;
    assert!(artifacts
        .validate()
        .expect_err("duplicate executable candidate should fail")
        .to_string()
        .contains("exactly one executable candidate"));

    let mut artifacts =
        runtime_v2_csm_governed_episode_contract().expect("governed episode artifacts");
    artifacts.first_run_trace[2].event_sequence = 9;
    assert!(artifacts
        .validate()
        .expect_err("non-contiguous first-run trace should fail")
        .to_string()
        .contains("contiguous"));

    let mut artifacts =
        runtime_v2_csm_governed_episode_contract().expect("governed episode artifacts");
    artifacts.scheduling_decision.claim_boundary = "full live birth and mediation".to_string();
    assert!(artifacts
        .validate()
        .expect_err("overclaim should fail")
        .to_string()
        .contains("WP-06-only non-claims"));
}
