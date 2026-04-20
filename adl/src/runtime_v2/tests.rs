use super::*;
use std::{
    env, fs,
    time::{SystemTime, UNIX_EPOCH},
};

fn unique_temp_path(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    env::temp_dir().join(format!("runtime-v2-{label}-{}-{nanos}", std::process::id()))
}

#[test]
fn runtime_v2_manifold_root_contract_is_stable() {
    let root = runtime_v2_manifold_contract().expect("contract");
    root.validate().expect("valid manifold root");

    assert_eq!(root.schema_version, RUNTIME_V2_MANIFOLD_SCHEMA);
    assert_eq!(root.manifold_id, "proto-csm-01");
    assert_eq!(root.lifecycle_state, "initialized");
    assert_eq!(root.artifact_path, DEFAULT_MANIFOLD_ARTIFACT_PATH);
    assert_eq!(root.clock_anchor.monotonic_tick, 0);
    assert_eq!(root.trace_root.next_event_sequence, 1);
    assert_eq!(root.snapshot_root.latest_snapshot_id, None);
    assert!(root
        .invariant_policy_refs
        .blocking_invariants
        .contains(&"single_active_manifold_instance".to_string()));
    assert!(root
        .review_surface
        .downstream_boundaries
        .iter()
        .any(|boundary| boundary.contains("WP-06")));
}

#[test]
fn runtime_v2_manifold_root_round_trips_without_path_leakage() {
    let temp_root = unique_temp_path("roundtrip");
    let path = temp_root.join(DEFAULT_MANIFOLD_ARTIFACT_PATH);
    let root = runtime_v2_manifold_contract().expect("contract");

    root.write_to_path(&path).expect("write manifest");
    let loaded = RuntimeV2ManifoldRoot::read_from_path(&path).expect("read manifest");
    assert_eq!(loaded, root);

    let text = fs::read_to_string(&path).expect("manifest text");
    assert!(text.contains("\"schema_version\": \"runtime_v2.manifold.v1\""));
    assert!(text.contains("\"artifact_path\": \"runtime_v2/manifold.json\""));
    assert!(!text.contains(temp_root.to_string_lossy().as_ref()));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_manifold_root_matches_golden_manifest_fixture() {
    let root = runtime_v2_manifold_contract().expect("contract");
    let generated = String::from_utf8(root.to_pretty_json_bytes().expect("json")).expect("utf8");
    let expected = include_str!("../../tests/fixtures/runtime_v2/manifold.json");

    assert_eq!(generated, expected.trim_end());
}

#[test]
fn runtime_v2_manifold_validation_rejects_unsafe_or_ambiguous_roots() {
    let mut root = runtime_v2_manifold_contract().expect("contract");
    root.manifold_id = " ".to_string();
    assert!(root
        .validate()
        .expect_err("empty id should fail")
        .to_string()
        .contains("manifold_id must not be empty"));

    let mut root = runtime_v2_manifold_contract().expect("contract");
    root.artifact_path = "/tmp/runtime_v2/manifold.json".to_string();
    assert!(root
        .validate()
        .expect_err("absolute path should fail")
        .to_string()
        .contains("artifact_path must be a repository-relative path"));

    let mut root = runtime_v2_manifold_contract().expect("contract");
    root.trace_root.next_event_sequence = 0;
    assert!(root
        .validate()
        .expect_err("zero sequence should fail")
        .to_string()
        .contains("trace_root.next_event_sequence must be positive"));
}

#[test]
fn runtime_v2_manifold_root_does_not_claim_later_wp_outputs() {
    let root = runtime_v2_manifold_contract().expect("contract");
    let json = String::from_utf8(root.to_pretty_json_bytes().expect("json")).expect("utf8");

    assert!(json.contains("WP-07 owns provisional citizen record materialization"));
    assert!(json.contains("WP-08 owns snapshot writing, sealing, and rehydration"));
    assert!(json.contains("no true Godel-agent birthday"));
    assert!(!json.contains("citizen_id"));
    assert!(!json.contains("snapshot_hash"));
    assert!(!json.contains("kernel_tick_completed"));
}

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
        include_str!("../../tests/fixtures/runtime_v2/kernel/service_registry.json").trim_end()
    );
    assert_eq!(
        state,
        include_str!("../../tests/fixtures/runtime_v2/kernel/service_state.json").trim_end()
    );
    assert_eq!(
        loop_jsonl,
        include_str!("../../tests/fixtures/runtime_v2/kernel/service_loop.jsonl")
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

#[test]
fn runtime_v2_citizen_lifecycle_contract_matches_manifold_refs() {
    let manifold = runtime_v2_manifold_contract().expect("manifold");
    let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold).expect("citizens");

    assert_eq!(citizens.active_index.manifold_id, manifold.manifold_id);
    assert_eq!(citizens.pending_index.manifold_id, manifold.manifold_id);
    assert_eq!(
        citizens.active_index.registry_root,
        manifold.citizen_registry_refs.registry_root
    );
    assert_eq!(
        citizens.active_index.index_path,
        manifold.citizen_registry_refs.active_index
    );
    assert_eq!(
        citizens.pending_index.index_path,
        manifold.citizen_registry_refs.pending_index
    );
    assert_eq!(citizens.records.len(), 2);
    assert_eq!(citizens.active_index.citizens.len(), 1);
    assert_eq!(citizens.pending_index.citizens.len(), 1);
    assert!(citizens
        .records
        .iter()
        .any(|record| record.lifecycle_state == "active" && record.can_execute_episodes));
    assert!(citizens
        .records
        .iter()
        .any(|record| record.lifecycle_state == "proposed" && !record.can_execute_episodes));
}

#[test]
fn runtime_v2_citizen_lifecycle_artifacts_match_golden_fixtures() {
    let citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");
    let alpha = citizens
        .records
        .iter()
        .find(|record| record.citizen_id == "proto-citizen-alpha")
        .expect("alpha");
    let beta = citizens
        .records
        .iter()
        .find(|record| record.citizen_id == "proto-citizen-beta")
        .expect("beta");
    let alpha_json = String::from_utf8(
        RuntimeV2CitizenLifecycleArtifacts::record_pretty_json_bytes(alpha).expect("alpha json"),
    )
    .expect("utf8 alpha");
    let beta_json = String::from_utf8(
        RuntimeV2CitizenLifecycleArtifacts::record_pretty_json_bytes(beta).expect("beta json"),
    )
    .expect("utf8 beta");
    let active_index_json = String::from_utf8(
        RuntimeV2CitizenLifecycleArtifacts::index_pretty_json_bytes(&citizens.active_index)
            .expect("active index json"),
    )
    .expect("utf8 active index");
    let pending_index_json = String::from_utf8(
        RuntimeV2CitizenLifecycleArtifacts::index_pretty_json_bytes(&citizens.pending_index)
            .expect("pending index json"),
    )
    .expect("utf8 pending index");

    assert_eq!(
        alpha_json,
        include_str!("../../tests/fixtures/runtime_v2/citizens/proto-citizen-alpha.json")
            .trim_end()
    );
    assert_eq!(
        beta_json,
        include_str!("../../tests/fixtures/runtime_v2/citizens/proto-citizen-beta.json").trim_end()
    );
    assert_eq!(
        active_index_json,
        include_str!("../../tests/fixtures/runtime_v2/citizens/active_index.json").trim_end()
    );
    assert_eq!(
        pending_index_json,
        include_str!("../../tests/fixtures/runtime_v2/citizens/pending_index.json").trim_end()
    );
}

#[test]
fn runtime_v2_citizen_lifecycle_writes_artifacts_without_path_leakage() {
    let temp_root = unique_temp_path("citizens");
    let citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");

    citizens
        .write_to_root(&temp_root)
        .expect("write citizen artifacts");

    let alpha_path = temp_root.join(&citizens.records[0].record_path);
    let beta_path = temp_root.join(&citizens.records[1].record_path);
    let active_index_path = temp_root.join(&citizens.active_index.index_path);
    let pending_index_path = temp_root.join(&citizens.pending_index.index_path);
    assert!(alpha_path.is_file());
    assert!(beta_path.is_file());
    assert!(active_index_path.is_file());
    assert!(pending_index_path.is_file());

    let alpha = fs::read_to_string(alpha_path).expect("alpha text");
    let index = fs::read_to_string(active_index_path).expect("index text");
    let temp_root_text = temp_root.to_string_lossy();
    assert!(!alpha.contains(temp_root_text.as_ref()));
    assert!(!index.contains(temp_root_text.as_ref()));
    assert!(index.contains("\"index_kind\": \"active\""));
    assert!(index.contains("\"citizens\""));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_citizen_lifecycle_validation_rejects_unsafe_or_ambiguous_state() {
    let mut citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");
    citizens.records[1].citizen_id = "proto-citizen-alpha".to_string();
    assert!(citizens
        .validate()
        .expect_err("duplicate citizen should fail")
        .to_string()
        .contains("duplicate citizen"));

    let mut citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");
    citizens.records[1].lifecycle_state = "paused".to_string();
    citizens.records[1].can_execute_episodes = true;
    assert!(citizens
        .validate()
        .expect_err("inactive executor should fail")
        .to_string()
        .contains("true only for active citizens"));

    let mut citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");
    citizens.records[1].lifecycle_state = "waking".to_string();
    assert!(citizens
        .validate()
        .expect_err("waking without rehydration proof should fail")
        .to_string()
        .contains("rehydration validation"));

    let mut citizens = runtime_v2_citizen_lifecycle_contract().expect("citizens");
    citizens.records[1].resources_released = true;
    assert!(citizens
        .validate()
        .expect_err("resource release without termination proof should fail")
        .to_string()
        .contains("before termination is recorded"));
}

#[test]
fn runtime_v2_snapshot_rehydration_contract_matches_upstream_refs() {
    let manifold = runtime_v2_manifold_contract().expect("manifold");
    let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold).expect("kernel");
    let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold).expect("citizens");
    let artifacts =
        RuntimeV2SnapshotAndRehydrationArtifacts::prototype(&manifold, &kernel, &citizens)
            .expect("snapshot");

    assert_eq!(
        artifacts.snapshot.schema_version,
        RUNTIME_V2_SNAPSHOT_MANIFEST_SCHEMA
    );
    assert_eq!(
        artifacts.rehydration_report.schema_version,
        RUNTIME_V2_REHYDRATION_REPORT_SCHEMA
    );
    assert_eq!(artifacts.snapshot.manifold_id, manifold.manifold_id);
    assert_eq!(
        artifacts.snapshot.last_trace_cursor,
        kernel.state.completed_through_event_sequence
    );
    assert_eq!(artifacts.snapshot.citizen_records, citizens.records);
    assert_eq!(artifacts.snapshot.active_index, citizens.active_index);
    assert_eq!(
        artifacts.rehydration_report.trace_resume_sequence,
        artifacts.snapshot.last_trace_cursor + 1
    );
    assert!(artifacts.rehydration_report.wake_allowed);
}

#[test]
fn runtime_v2_snapshot_rehydration_artifacts_match_golden_fixtures() {
    let artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");
    let snapshot = String::from_utf8(
        artifacts
            .snapshot_pretty_json_bytes()
            .expect("snapshot json"),
    )
    .expect("utf8 snapshot");
    let rehydration = String::from_utf8(
        artifacts
            .rehydration_report_pretty_json_bytes()
            .expect("rehydration json"),
    )
    .expect("utf8 rehydration");

    assert_eq!(
        snapshot,
        include_str!("../../tests/fixtures/runtime_v2/snapshots/snapshot-0001.json").trim_end()
    );
    assert_eq!(
        rehydration,
        include_str!("../../tests/fixtures/runtime_v2/rehydration_report.json").trim_end()
    );
}

#[test]
fn runtime_v2_snapshot_rehydration_writes_artifacts_without_path_leakage() {
    let temp_root = unique_temp_path("snapshot");
    let artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");

    artifacts
        .write_to_root(&temp_root)
        .expect("write snapshot artifacts");

    let snapshot_path = temp_root.join(&artifacts.snapshot.snapshot_path);
    let report_path = temp_root.join(&artifacts.rehydration_report.report_path);
    assert!(snapshot_path.is_file());
    assert!(report_path.is_file());

    let snapshot = fs::read_to_string(snapshot_path).expect("snapshot text");
    let report = fs::read_to_string(report_path).expect("report text");
    let temp_root_text = temp_root.to_string_lossy();
    assert!(!snapshot.contains(temp_root_text.as_ref()));
    assert!(!report.contains(temp_root_text.as_ref()));
    assert!(snapshot.contains("\"structural_checksum\": \"fnv1a64:"));
    assert!(report.contains("\"wake_allowed\": true"));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_snapshot_rehydration_validation_rejects_unsafe_or_ambiguous_state() {
    let mut artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");
    artifacts.snapshot.structural_checksum = "fnv1a64:0000000000000000".to_string();
    assert!(artifacts
        .validate()
        .expect_err("checksum drift should fail")
        .to_string()
        .contains("checksum mismatch"));

    let mut artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");
    artifacts.rehydration_report.restored_manifold_id = "other-manifold".to_string();
    assert!(artifacts
        .validate()
        .expect_err("wrong restored manifold should fail")
        .to_string()
        .contains("restored manifold id"));

    let mut artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");
    artifacts.rehydration_report.trace_resume_sequence = artifacts.snapshot.last_trace_cursor;
    assert!(artifacts
        .validate()
        .expect_err("non-advancing trace should fail")
        .to_string()
        .contains("resume after the snapshot cursor"));

    let mut artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");
    artifacts
        .rehydration_report
        .restored_active_citizens
        .push("proto-citizen-alpha".to_string());
    assert!(artifacts
        .validate()
        .expect_err("duplicate active citizen should fail")
        .to_string()
        .contains("duplicate"));

    let mut artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");
    artifacts.snapshot.invariant_status[0].status = "failed".to_string();
    artifacts.snapshot.structural_checksum = artifacts
        .snapshot
        .compute_structural_checksum()
        .expect("checksum");
    assert!(artifacts
        .validate()
        .expect_err("failed invariant should fail")
        .to_string()
        .contains("invariant checks must pass"));

    let mut artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");
    artifacts.snapshot.schema_version = "runtime_v2.snapshot.v0".to_string();
    artifacts.snapshot.manifold_id = "other-manifold".to_string();
    assert!(artifacts
        .validate()
        .expect_err("snapshot schema and manifold mismatch should fail")
        .to_string()
        .contains("unsupported Runtime v2 snapshot schema"));

    let mut artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");
    artifacts.snapshot.manifold_state.lifecycle_state = "active".to_string();
    assert!(artifacts
        .validate()
        .expect_err("snapshot lifecycle should require snapshotting")
        .to_string()
        .contains("must be captured while snapshotting"));

    let mut artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");
    artifacts
        .snapshot
        .manifold_state
        .snapshot_root
        .latest_snapshot_id = Some("wrong".to_string());
    assert!(artifacts
        .validate()
        .expect_err("stale latest snapshot id should fail")
        .to_string()
        .contains("latest snapshot id"));

    let mut artifacts = runtime_v2_snapshot_rehydration_contract().expect("snapshot");
    artifacts.rehydration_report.wake_allowed = true;
    artifacts.rehydration_report.wake_refused_reason = Some("retry later".to_string());
    assert!(artifacts
        .validate()
        .expect_err("woke with reason should fail")
        .to_string()
        .contains("must be absent when wake is allowed"));
}

#[test]
fn runtime_v2_invariant_violation_contract_records_rejected_transition() {
    let manifold = runtime_v2_manifold_contract().expect("manifold");
    let kernel = RuntimeV2KernelLoopArtifacts::prototype(&manifold).expect("kernel");
    let citizens = RuntimeV2CitizenLifecycleArtifacts::prototype(&manifold).expect("citizens");
    let violation = RuntimeV2InvariantViolationArtifact::duplicate_active_citizen_prototype(
        &manifold, &kernel, &citizens,
    )
    .expect("violation");

    assert_eq!(
        violation.schema_version,
        RUNTIME_V2_INVARIANT_VIOLATION_SCHEMA
    );
    assert_eq!(violation.manifold_id, manifold.manifold_id);
    assert_eq!(
        violation.invariant_id,
        "no_duplicate_active_citizen_instance"
    );
    assert_eq!(violation.invariant_owner_service_id, "invariant_checker");
    assert_eq!(violation.severity, "blocking");
    assert_eq!(
        violation.policy_enforcement_mode,
        "fail_closed_before_activation"
    );
    assert_eq!(violation.affected_citizens, vec!["proto-citizen-alpha"]);
    assert!(violation.result.blocked_before_commit);
    assert!(violation.source_error.contains("duplicate citizen"));
    assert!(violation
        .evaluated_refs
        .iter()
        .any(|evaluated_ref| evaluated_ref.ref_kind == "kernel_state"));
}

#[test]
fn runtime_v2_invariant_violation_artifact_matches_golden_fixture() {
    let violation = runtime_v2_invariant_violation_contract().expect("violation");
    let generated =
        String::from_utf8(violation.to_pretty_json_bytes().expect("json")).expect("utf8");

    assert_eq!(
        generated,
        include_str!("../../tests/fixtures/runtime_v2/invariants/violation-0001.json").trim_end()
    );
}

#[test]
fn runtime_v2_invariant_violation_writes_artifact_without_path_leakage() {
    let temp_root = unique_temp_path("invariant");
    let violation = runtime_v2_invariant_violation_contract().expect("violation");

    violation
        .write_to_root(&temp_root)
        .expect("write violation artifact");

    let violation_path = temp_root.join(&violation.artifact_path);
    assert!(violation_path.is_file());
    let text = fs::read_to_string(violation_path).expect("violation text");
    assert!(!text.contains(temp_root.to_string_lossy().as_ref()));
    assert!(text.contains("\"schema_version\": \"runtime_v2.invariant_violation.v1\""));
    assert!(text.contains("\"blocked_before_commit\": true"));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_invariant_violation_validation_rejects_unsafe_or_ambiguous_state() {
    let mut violation = runtime_v2_invariant_violation_contract().expect("violation");
    violation.artifact_path = "/tmp/violation.json".to_string();
    assert!(violation
        .validate()
        .expect_err("absolute path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut violation = runtime_v2_invariant_violation_contract().expect("violation");
    violation.result.blocked_before_commit = false;
    assert!(violation
        .validate()
        .expect_err("unblocked violation should fail")
        .to_string()
        .contains("before commit"));

    let mut violation = runtime_v2_invariant_violation_contract().expect("violation");
    violation
        .evaluated_refs
        .push(violation.evaluated_refs[0].clone());
    assert!(violation
        .validate()
        .expect_err("duplicate evaluated refs should fail")
        .to_string()
        .contains("duplicate ref"));

    let mut violation = runtime_v2_invariant_violation_contract().expect("violation");
    violation.refusal_reason = " ".to_string();
    assert!(violation
        .validate()
        .expect_err("empty refusal reason should fail")
        .to_string()
        .contains("refusal_reason must not be empty"));
}

#[test]
fn runtime_v2_operator_control_report_records_bounded_controls() {
    let report = runtime_v2_operator_control_report_contract().expect("operator report");

    assert_eq!(
        report.schema_version,
        RUNTIME_V2_OPERATOR_CONTROL_REPORT_SCHEMA
    );
    assert_eq!(report.manifold_id, "proto-csm-01");
    assert_eq!(
        report.control_interface_service_id,
        "operator_control_interface"
    );
    assert_eq!(report.commands.len(), 7);
    assert_eq!(report.commands[0].command, "inspect_manifold");
    assert_eq!(report.commands[2].command, "pause_manifold");
    assert_eq!(
        report.commands[2].post_state.manifold_lifecycle_state,
        "paused"
    );
    assert_eq!(report.commands[3].command, "resume_manifold");
    assert_eq!(
        report.commands[4].post_state.latest_snapshot_id.as_deref(),
        Some("snapshot-0001")
    );
    assert_eq!(report.commands[5].command, "inspect_last_failures");
    assert!(report.commands[5]
        .trace_event_ref
        .contains("violation-0001"));
    assert_eq!(
        report.commands[6].post_state.manifold_lifecycle_state,
        "terminated"
    );
    assert_eq!(report.commands[6].post_state.active_citizen_count, 0);
}

#[test]
fn runtime_v2_operator_control_report_matches_golden_fixture() {
    let report = runtime_v2_operator_control_report_contract().expect("operator report");
    let generated = String::from_utf8(report.to_pretty_json_bytes().expect("json")).expect("utf8");

    assert_eq!(
        generated,
        include_str!("../../tests/fixtures/runtime_v2/operator/control_report.json").trim_end()
    );
}

#[test]
fn runtime_v2_operator_control_report_writes_without_path_leakage() {
    let temp_root = unique_temp_path("operator-controls");
    let report = runtime_v2_operator_control_report_contract().expect("operator report");

    report
        .write_to_root(&temp_root)
        .expect("write operator report");

    let report_path = temp_root.join(&report.artifact_path);
    assert!(report_path.is_file());
    let text = fs::read_to_string(report_path).expect("operator report text");
    assert!(!text.contains(temp_root.to_string_lossy().as_ref()));
    assert!(text.contains("\"schema_version\": \"runtime_v2.operator_control_report.v1\""));
    assert!(text.contains("\"command\": \"pause_manifold\""));
    assert!(text.contains("\"command\": \"terminate_manifold\""));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_operator_control_validation_rejects_unsafe_or_ambiguous_state() {
    let mut report = runtime_v2_operator_control_report_contract().expect("operator report");
    report.artifact_path = "/tmp/operator/control_report.json".to_string();
    assert!(report
        .validate()
        .expect_err("absolute path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut report = runtime_v2_operator_control_report_contract().expect("operator report");
    report.commands[0].command = "inspect_citizens".to_string();
    assert!(report
        .validate()
        .expect_err("command order should fail")
        .to_string()
        .contains("deterministic command order"));

    let mut report = runtime_v2_operator_control_report_contract().expect("operator report");
    report.commands[2].post_state = report.commands[2].pre_state.clone();
    assert!(report
        .validate()
        .expect_err("mutating command with unchanged state should fail")
        .to_string()
        .contains("mutating control commands must change post_state"));

    let mut report = runtime_v2_operator_control_report_contract().expect("operator report");
    report.commands[6].post_state.active_citizen_count = 1;
    assert!(report
        .validate()
        .expect_err("terminated state with active citizens should fail")
        .to_string()
        .contains("terminated state must not retain active citizens"));
}

#[test]
fn runtime_v2_security_boundary_proof_records_refused_invalid_action() {
    let proof = runtime_v2_security_boundary_proof_contract().expect("security proof");

    assert_eq!(
        proof.schema_version,
        RUNTIME_V2_SECURITY_BOUNDARY_PROOF_SCHEMA
    );
    assert_eq!(proof.manifold_id, "proto-csm-01");
    assert_eq!(proof.boundary_service_id, "operator_control_interface");
    assert_eq!(
        proof.attempt.attempted_action,
        "resume_manifold_without_fresh_invariant_pass"
    );
    assert!(!proof.result.allowed);
    assert_eq!(
        proof.result.resulting_state.manifold_lifecycle_state,
        "paused"
    );
    assert!(proof
        .evaluated_rules
        .iter()
        .any(|rule| rule.rule_kind == "blocking_invariant"));
    assert!(proof
        .related_artifacts
        .contains(&"runtime_v2/operator/control_report.json".to_string()));
    assert!(proof
        .related_artifacts
        .contains(&"runtime_v2/invariants/violation-0001.json".to_string()));
}

#[test]
fn runtime_v2_security_boundary_proof_matches_golden_fixture() {
    let proof = runtime_v2_security_boundary_proof_contract().expect("security proof");
    let generated = String::from_utf8(proof.to_pretty_json_bytes().expect("json")).expect("utf8");

    assert_eq!(
        generated,
        include_str!("../../tests/fixtures/runtime_v2/security_boundary/proof_packet.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_security_boundary_proof_writes_without_path_leakage() {
    let temp_root = unique_temp_path("security-boundary");
    let proof = runtime_v2_security_boundary_proof_contract().expect("security proof");

    proof
        .write_to_root(&temp_root)
        .expect("write security proof");

    let proof_path = temp_root.join(&proof.artifact_path);
    assert!(proof_path.is_file());
    let text = fs::read_to_string(proof_path).expect("security proof text");
    assert!(!text.contains(temp_root.to_string_lossy().as_ref()));
    assert!(text.contains("\"schema_version\": \"runtime_v2.security_boundary_proof.v1\""));
    assert!(text.contains("\"allowed\": false"));
    assert!(text.contains("resume_manifold_without_fresh_invariant_pass"));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_security_boundary_validation_rejects_unsafe_or_ambiguous_state() {
    let mut proof = runtime_v2_security_boundary_proof_contract().expect("security proof");
    proof.artifact_path = "/tmp/security/proof.json".to_string();
    assert!(proof
        .validate()
        .expect_err("absolute path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut proof = runtime_v2_security_boundary_proof_contract().expect("security proof");
    proof.result.allowed = true;
    assert!(proof
        .validate()
        .expect_err("allowed invalid action should fail")
        .to_string()
        .contains("must be refused"));

    let mut proof = runtime_v2_security_boundary_proof_contract().expect("security proof");
    proof.evaluated_rules.remove(1);
    assert!(proof
        .validate()
        .expect_err("missing invariant coverage should fail")
        .to_string()
        .contains("must include operator, invariant, and kernel checks"));

    let mut proof = runtime_v2_security_boundary_proof_contract().expect("security proof");
    proof
        .related_artifacts
        .retain(|artifact| artifact != "runtime_v2/operator/control_report.json");
    assert!(proof
        .validate()
        .expect_err("missing operator evidence should fail")
        .to_string()
        .contains("operator control evidence"));
}
