use super::common::unique_temp_path;
use super::*;
use std::fs;

#[test]
fn runtime_v2_kernel_loop_contract_matches_manifold_refs() {
    let manifold = runtime_v2_manifold_contract().expect("manifold");
    let loop_artifacts = RuntimeV2KernelLoopArtifacts::prototype(&manifold).expect("loop");

    assert_eq!(
        loop_artifacts.registry.registry_path,
        manifold.kernel_service_refs.registry_path
    );
    assert_eq!(
        loop_artifacts.service_loop_path,
        manifold.kernel_service_refs.service_loop_path
    );
    assert_eq!(
        loop_artifacts.state.service_state_path,
        manifold.kernel_service_refs.service_state_path
    );
    assert_eq!(loop_artifacts.registry.services.len(), 8);
    assert_eq!(loop_artifacts.events.len(), 8);
    assert_eq!(
        loop_artifacts.events[0].event_sequence,
        manifold.trace_root.next_event_sequence
    );
    assert_eq!(loop_artifacts.state.loop_status, "bounded_tick_complete");
    assert!(loop_artifacts
        .registry
        .services
        .iter()
        .any(|service| service.service_id == "operator_control_interface"));
}

#[test]
fn runtime_v2_kernel_loop_artifacts_match_golden_fixtures() {
    let loop_artifacts = runtime_v2_kernel_loop_contract().expect("loop");
    let registry = String::from_utf8(
        loop_artifacts
            .registry_pretty_json_bytes()
            .expect("registry json"),
    )
    .expect("utf8 registry");
    let state = String::from_utf8(
        loop_artifacts
            .state_pretty_json_bytes()
            .expect("state json"),
    )
    .expect("utf8 state");
    let loop_jsonl = String::from_utf8(
        loop_artifacts
            .service_loop_jsonl_bytes()
            .expect("loop jsonl"),
    )
    .expect("utf8 loop jsonl");

    assert_eq!(
        registry,
        include_str!("../../../tests/fixtures/runtime_v2/kernel/service_registry.json").trim_end()
    );
    assert_eq!(
        state,
        include_str!("../../../tests/fixtures/runtime_v2/kernel/service_state.json").trim_end()
    );
    assert_eq!(
        loop_jsonl,
        include_str!("../../../tests/fixtures/runtime_v2/kernel/service_loop.jsonl")
    );
}

#[test]
fn runtime_v2_kernel_loop_writes_artifacts_without_path_leakage() {
    let temp_root = unique_temp_path("kernel-loop");
    let loop_artifacts = runtime_v2_kernel_loop_contract().expect("loop");

    loop_artifacts
        .write_to_root(&temp_root)
        .expect("write loop artifacts");

    let registry_path = temp_root.join(&loop_artifacts.registry.registry_path);
    let state_path = temp_root.join(&loop_artifacts.state.service_state_path);
    let loop_path = temp_root.join(&loop_artifacts.service_loop_path);
    assert!(registry_path.is_file());
    assert!(state_path.is_file());
    assert!(loop_path.is_file());

    let registry = fs::read_to_string(registry_path).expect("registry text");
    let state = fs::read_to_string(state_path).expect("state text");
    let loop_jsonl = fs::read_to_string(loop_path).expect("loop text");
    let temp_root_text = temp_root.to_string_lossy();
    assert!(!registry.contains(temp_root_text.as_ref()));
    assert!(!state.contains(temp_root_text.as_ref()));
    assert!(!loop_jsonl.contains(temp_root_text.as_ref()));
    assert_eq!(loop_jsonl.lines().count(), 8);

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_kernel_loop_validation_rejects_unsafe_or_ambiguous_state() {
    let mut loop_artifacts = runtime_v2_kernel_loop_contract().expect("loop");
    loop_artifacts.registry.services[1].service_id = "clock_service".to_string();
    assert!(loop_artifacts
        .validate()
        .expect_err("duplicate service should fail")
        .to_string()
        .contains("duplicate service"));

    let mut loop_artifacts = runtime_v2_kernel_loop_contract().expect("loop");
    loop_artifacts.events[1].event_sequence = 10;
    assert!(loop_artifacts
        .validate()
        .expect_err("non-contiguous event order should fail")
        .to_string()
        .contains("contiguous"));

    let mut loop_artifacts = runtime_v2_kernel_loop_contract().expect("loop");
    loop_artifacts.service_loop_path = "/tmp/service_loop.jsonl".to_string();
    assert!(loop_artifacts
        .validate()
        .expect_err("absolute path should fail")
        .to_string()
        .contains("repository-relative path"));
}

#[test]
fn runtime_v2_kernel_loop_validation_rejects_disordered_state() {
    let mut loop_artifacts = runtime_v2_kernel_loop_contract().expect("loop");
    loop_artifacts.events[0].event_sequence = 0;
    assert!(loop_artifacts
        .validate()
        .expect_err("zero event sequence should fail")
        .to_string()
        .contains("must be positive"));

    let mut loop_artifacts = runtime_v2_kernel_loop_contract().expect("loop");
    loop_artifacts.registry.manifold_id = "other-manifold".to_string();
    assert!(loop_artifacts
        .validate()
        .expect_err("manifold mismatch should fail")
        .to_string()
        .contains("must match"));

    let mut loop_artifacts = runtime_v2_kernel_loop_contract().expect("loop");
    loop_artifacts.events[0].service_id = "unregistered_service".to_string();
    assert!(loop_artifacts
        .validate()
        .expect_err("unknown event service should fail")
        .to_string()
        .contains("unknown service"));
}

#[test]
fn runtime_v2_kernel_loop_validation_rejects_invalid_event_payloads() {
    let mut loop_artifacts = runtime_v2_kernel_loop_contract().expect("loop");
    loop_artifacts.events[0].outcome = "invalid".to_string();
    assert!(loop_artifacts
        .validate()
        .expect_err("invalid event outcome should fail")
        .to_string()
        .contains("unsupported kernel_loop_event.outcome"));

    let mut loop_artifacts = runtime_v2_kernel_loop_contract().expect("loop");
    loop_artifacts.state.services[0].blocked_reason = Some("   ".to_string());
    assert!(loop_artifacts
        .validate()
        .expect_err("blank blocked reason should fail")
        .to_string()
        .contains("blocked_reason must not be empty"));

    let mut loop_artifacts = runtime_v2_kernel_loop_contract().expect("loop");
    loop_artifacts.state.services[0].last_event_sequence = 0;
    assert!(loop_artifacts
        .validate()
        .expect_err("zero event cursor should fail")
        .to_string()
        .contains("last_event_sequence must be positive"));
}
