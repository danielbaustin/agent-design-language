#[cfg(feature = "slow-proof-tests")]
use super::common::unique_temp_path;
use super::*;
#[cfg(feature = "slow-proof-tests")]
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

#[cfg(feature = "slow-proof-tests")]
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

#[test]
fn runtime_v2_csm_governed_episode_record_validators_reject_contract_drift() {
    let artifacts = runtime_v2_csm_governed_episode_contract().expect("governed episode artifacts");

    let mut pressure = artifacts.resource_pressure.clone();
    pressure.schema_version = "runtime_v2.csm_resource_pressure_fixture.v0".to_string();
    assert!(pressure
        .validate()
        .expect_err("unsupported pressure schema should fail")
        .to_string()
        .contains("unsupported Runtime v2 CSM resource pressure schema"));

    let mut pressure = artifacts.resource_pressure.clone();
    pressure.demo_id = "D5".to_string();
    assert!(pressure
        .validate()
        .expect_err("wrong pressure demo should fail")
        .to_string()
        .contains("must map to D4"));

    let mut pressure = artifacts.resource_pressure.clone();
    pressure.pressure_kind = "memory_pressure".to_string();
    assert!(pressure
        .validate()
        .expect_err("unsupported pressure kind should fail")
        .to_string()
        .contains("pressure_kind"));

    let mut pressure = artifacts.resource_pressure.clone();
    pressure.available_compute_tokens = 0;
    assert!(pressure
        .validate()
        .expect_err("zero pressure budget should fail")
        .to_string()
        .contains("budgets must be positive"));

    let mut pressure = artifacts.resource_pressure.clone();
    pressure.requested_compute_tokens = pressure.available_compute_tokens;
    assert!(pressure
        .validate()
        .expect_err("non-exceeding pressure fixture should fail")
        .to_string()
        .contains("exceed available resources"));

    let mut candidate = artifacts.resource_pressure.candidates[0].clone();
    candidate.identity_handle = "citizen://proto-citizen-alpha".to_string();
    assert!(candidate
        .validate()
        .expect_err("wrong identity scheme should fail")
        .to_string()
        .contains("runtime-v2 scheme"));

    let mut candidate = artifacts.resource_pressure.candidates[0].clone();
    candidate.priority = 0;
    assert!(candidate
        .validate()
        .expect_err("zero priority should fail")
        .to_string()
        .contains("priority must be positive"));

    let mut candidate = artifacts.resource_pressure.candidates[0].clone();
    candidate.estimated_compute_tokens = 0;
    assert!(candidate
        .validate()
        .expect_err("zero estimate should fail")
        .to_string()
        .contains("positive compute and time budgets"));

    let mut candidate = artifacts.resource_pressure.candidates[0].clone();
    candidate.safety_class = "unbounded".to_string();
    assert!(candidate
        .validate()
        .expect_err("unsupported safety class should fail")
        .to_string()
        .contains("safety_class"));

    let mut decision = artifacts.scheduling_decision.clone();
    decision.schema_version = "runtime_v2.csm_scheduling_decision.v0".to_string();
    assert!(decision
        .validate()
        .expect_err("unsupported scheduling schema should fail")
        .to_string()
        .contains("unsupported Runtime v2 CSM scheduling decision schema"));

    let mut decision = artifacts.scheduling_decision.clone();
    decision.scheduling_outcome = "skipped".to_string();
    assert!(decision
        .validate()
        .expect_err("unsupported scheduling outcome should fail")
        .to_string()
        .contains("scheduling_outcome"));

    let mut decision = artifacts.scheduling_decision.clone();
    decision.deferred_episode_ids.clear();
    assert!(decision
        .validate()
        .expect_err("empty deferred ids should fail")
        .to_string()
        .contains("deferred_episode_ids must not be empty"));

    let mut event = artifacts.first_run_trace[0].clone();
    event.event_sequence = 0;
    assert!(event
        .validate()
        .expect_err("zero trace sequence should fail")
        .to_string()
        .contains("sequence must be positive"));

    let mut event = artifacts.first_run_trace[0].clone();
    event.outcome = "teleported".to_string();
    assert!(event
        .validate()
        .expect_err("unsupported trace outcome should fail")
        .to_string()
        .contains("outcome"));
}
