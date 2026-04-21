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
    assert_eq!(report.commands[2].outcome, "deferred");
    assert_eq!(
        report.commands[2].post_state.manifold_lifecycle_state,
        "active"
    );
    assert_eq!(report.commands[3].command, "resume_manifold");
    assert_eq!(report.commands[3].outcome, "refused");
    assert_eq!(
        report.commands[4].post_state.latest_snapshot_id.as_deref(),
        None
    );
    assert_eq!(report.commands[4].outcome, "deferred");
    assert_eq!(report.commands[5].command, "inspect_last_failures");
    assert!(report.commands[5]
        .trace_event_ref
        .contains("violation-0001"));
    assert_eq!(
        report.commands[6].post_state.manifold_lifecycle_state,
        "active"
    );
    assert_eq!(report.commands[6].outcome, "deferred");
    assert_eq!(report.commands[6].post_state.active_citizen_count, 1);
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
    report.commands[2].outcome = "allowed".to_string();
    assert!(report
        .validate()
        .expect_err("mutating command with unchanged state should fail")
        .to_string()
        .contains("mutating control commands must change post_state"));

    let mut report = runtime_v2_operator_control_report_contract().expect("operator report");
    report.commands[6].post_state.manifold_lifecycle_state = "terminated".to_string();
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

#[test]
fn runtime_v2_csm_run_packet_contract_is_stable() {
    let contract = runtime_v2_csm_run_packet_contract().expect("csm run contract");
    contract.validate().expect("valid CSM run contract");

    assert_eq!(
        contract.schema_version,
        RUNTIME_V2_CSM_RUN_PACKET_CONTRACT_SCHEMA
    );
    assert_eq!(contract.demo_id, "D2");
    assert_eq!(contract.manifold_id, "proto-csm-01");
    assert_eq!(
        contract.artifact_path,
        "runtime_v2/csm_run/run_packet_contract.json"
    );
    assert!(contract
        .artifact_requirements
        .iter()
        .any(|artifact| artifact.artifact_id == "run_packet_fixture"
            && artifact.must_exist_before_live_run));
    assert!(contract
        .artifact_requirements
        .iter()
        .any(|artifact| artifact.artifact_id == "observatory_packet"
            && !artifact.must_exist_before_live_run));
    assert_eq!(contract.stages.len(), 5);
    assert_eq!(contract.stages[0].owner_wp, "WP-03");
    assert_eq!(contract.stages[4].owner_wp, "WP-09-WP-10");
    assert!(contract
        .claim_boundary
        .contains("not a live Runtime v2 execution artifact"));
}

#[test]
fn runtime_v2_csm_run_packet_contract_matches_golden_fixture() {
    let contract = runtime_v2_csm_run_packet_contract().expect("csm run contract");
    let generated =
        String::from_utf8(contract.to_pretty_json_bytes().expect("json")).expect("utf8");

    assert_eq!(
        generated,
        include_str!("../../tests/fixtures/runtime_v2/csm_run/run_packet_contract.json").trim_end()
    );
}

#[test]
fn runtime_v2_csm_run_packet_contract_writes_without_path_leakage() {
    let temp_root = unique_temp_path("csm-run-contract");
    let contract = runtime_v2_csm_run_packet_contract().expect("csm run contract");

    contract
        .write_to_root(&temp_root)
        .expect("write CSM run contract");

    let contract_path = temp_root.join(&contract.artifact_path);
    assert!(contract_path.is_file());
    let text = fs::read_to_string(contract_path).expect("contract text");
    assert!(!text.contains(temp_root.to_string_lossy().as_ref()));
    assert!(text.contains("\"schema_version\": \"runtime_v2.csm_run_packet_contract.v1\""));
    assert!(text.contains("\"demo_id\": \"D2\""));
    assert!(text.contains("not a live Runtime v2 execution artifact"));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_csm_run_packet_validation_rejects_unsafe_or_ambiguous_state() {
    let mut contract = runtime_v2_csm_run_packet_contract().expect("csm run contract");
    contract.artifact_path = "/tmp/runtime_v2/csm_run/run_packet_contract.json".to_string();
    assert!(contract
        .validate()
        .expect_err("absolute path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut contract = runtime_v2_csm_run_packet_contract().expect("csm run contract");
    contract.stages[1].sequence = 9;
    assert!(contract
        .validate()
        .expect_err("non-contiguous stage order should fail")
        .to_string()
        .contains("contiguous sequence order"));

    let mut contract = runtime_v2_csm_run_packet_contract().expect("csm run contract");
    contract
        .artifact_requirements
        .retain(|artifact| artifact.artifact_id != "violation_schema");
    assert!(contract
        .validate()
        .expect_err("missing violation schema should fail")
        .to_string()
        .contains("violation_schema"));

    let mut contract = runtime_v2_csm_run_packet_contract().expect("csm run contract");
    contract.claim_boundary = "live run succeeded".to_string();
    assert!(contract
        .validate()
        .expect_err("overclaim should fail")
        .to_string()
        .contains("non-live claim boundary"));
}

#[test]
fn runtime_v2_csm_run_packet_fixture_is_reviewable_and_bounded() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../demos/fixtures/csm_run/proto-csm-01-run-packet.json"
    ))
    .expect("parse CSM run packet fixture");

    assert_eq!(fixture["schema"], "adl.csm_run_packet_fixture.v1");
    assert_eq!(fixture["manifold_id"], "proto-csm-01");
    assert_eq!(fixture["demo_id"], "D2");
    assert_eq!(
        fixture["contract_ref"],
        "runtime_v2/csm_run/run_packet_contract.json"
    );
    assert!(fixture["claim_boundary"]
        .as_str()
        .expect("claim boundary")
        .contains("does not prove that Runtime v2 has executed the run"));
    assert_eq!(
        fixture["stage_plan"]
            .as_array()
            .expect("stage plan array")
            .len(),
        5
    );
    assert!(fixture["required_before_live_run"]
        .as_array()
        .expect("required artifacts")
        .iter()
        .any(|value| value == "runtime_v2/invariants/csm_run_invariant_map.json"));

    let text = include_str!("../../../demos/fixtures/csm_run/proto-csm-01-run-packet.json");
    assert!(!text.contains(&["/", "Users/"].concat()));
    assert!(!text.contains(&["/", "private/"].concat()));
    assert!(!text.contains("BEGIN "));
}

#[test]
fn runtime_v2_invariant_and_violation_contract_is_stable() {
    let artifacts =
        runtime_v2_invariant_and_violation_contract().expect("invariant and violation artifacts");
    artifacts.validate().expect("valid WP-04 artifacts");

    assert_eq!(
        artifacts.invariant_map.schema_version,
        RUNTIME_V2_CSM_RUN_INVARIANT_MAP_SCHEMA
    );
    assert_eq!(artifacts.invariant_map.demo_id, "D2");
    assert_eq!(artifacts.invariant_map.manifold_id, "proto-csm-01");
    assert_eq!(
        artifacts.invariant_map.artifact_path,
        "runtime_v2/invariants/csm_run_invariant_map.json"
    );
    assert_eq!(artifacts.invariant_map.coverage_entries.len(), 5);
    assert!(artifacts
        .invariant_map
        .coverage_entries
        .iter()
        .any(
            |entry| entry.invariant_id == "invalid_action_must_be_refused_before_commit"
                && entry.coverage_status == "negative_fixture_backed"
        ));
    assert_eq!(
        artifacts.violation_schema.schema_version,
        RUNTIME_V2_VIOLATION_ARTIFACT_SCHEMA_CONTRACT
    );
    assert_eq!(
        artifacts.violation_schema.artifact_schema_version,
        RUNTIME_V2_INVARIANT_VIOLATION_SCHEMA
    );
    assert_eq!(
        artifacts.violation_schema.negative_fixture_ref,
        "runtime_v2/invariants/violation-0001.json"
    );
}

#[test]
fn runtime_v2_invariant_and_violation_contract_matches_golden_fixtures() {
    let artifacts =
        runtime_v2_invariant_and_violation_contract().expect("invariant and violation artifacts");
    let invariant_map = String::from_utf8(
        artifacts
            .invariant_map
            .to_pretty_json_bytes()
            .expect("json"),
    )
    .expect("utf8");
    let violation_schema = String::from_utf8(
        artifacts
            .violation_schema
            .to_pretty_json_bytes()
            .expect("json"),
    )
    .expect("utf8");

    assert_eq!(
        invariant_map,
        include_str!("../../tests/fixtures/runtime_v2/invariants/csm_run_invariant_map.json")
            .trim_end()
    );
    assert_eq!(
        violation_schema,
        include_str!("../../tests/fixtures/runtime_v2/violations/violation_artifact_schema.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_invariant_and_violation_contract_writes_without_path_leakage() {
    let temp_root = unique_temp_path("wp04-contract");
    let artifacts =
        runtime_v2_invariant_and_violation_contract().expect("invariant and violation artifacts");

    artifacts
        .write_to_root(&temp_root)
        .expect("write WP-04 artifacts");

    let invariant_path = temp_root.join(&artifacts.invariant_map.artifact_path);
    let violation_schema_path = temp_root.join(&artifacts.violation_schema.artifact_path);
    assert!(invariant_path.is_file());
    assert!(violation_schema_path.is_file());
    let invariant_text = fs::read_to_string(invariant_path).expect("invariant map text");
    let violation_text = fs::read_to_string(violation_schema_path).expect("violation schema text");
    let temp_root_text = temp_root.to_string_lossy();
    assert!(!invariant_text.contains(temp_root_text.as_ref()));
    assert!(!violation_text.contains(temp_root_text.as_ref()));
    assert!(invariant_text.contains("\"schema_version\": \"runtime_v2.csm_run_invariant_map.v1\""));
    assert!(violation_text
        .contains("\"schema_version\": \"runtime_v2.violation_artifact_schema_contract.v1\""));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_invariant_and_violation_validation_rejects_unsafe_or_ambiguous_state() {
    let mut artifacts =
        runtime_v2_invariant_and_violation_contract().expect("invariant and violation artifacts");
    artifacts.invariant_map.artifact_path =
        "/tmp/runtime_v2/invariants/csm_run_invariant_map.json".to_string();
    assert!(artifacts
        .validate()
        .expect_err("absolute invariant path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut artifacts =
        runtime_v2_invariant_and_violation_contract().expect("invariant and violation artifacts");
    artifacts
        .invariant_map
        .coverage_entries
        .retain(|entry| entry.invariant_id != "invalid_action_must_be_refused_before_commit");
    assert!(artifacts
        .validate()
        .expect_err("missing invariant should fail")
        .to_string()
        .contains("D2 invariant set"));

    let mut artifacts =
        runtime_v2_invariant_and_violation_contract().expect("invariant and violation artifacts");
    artifacts
        .violation_schema
        .required_fields
        .retain(|field| field.field_name != "result");
    assert!(artifacts
        .validate()
        .expect_err("missing result field should fail")
        .to_string()
        .contains("every required field"));

    let mut artifacts =
        runtime_v2_invariant_and_violation_contract().expect("invariant and violation artifacts");
    artifacts.invariant_map.claim_boundary = "live Runtime v2 run executed".to_string();
    assert!(artifacts
        .validate()
        .expect_err("overclaim should fail")
        .to_string()
        .contains("non-live claim boundary"));
}

#[test]
fn runtime_v2_invariant_and_violation_fixtures_are_positive_and_negative_pair() {
    let positive: serde_json::Value = serde_json::from_str(include_str!(
        "../../tests/fixtures/runtime_v2/csm_run/run_packet_contract.json"
    ))
    .expect("parse positive fixture");
    let negative: serde_json::Value = serde_json::from_str(include_str!(
        "../../tests/fixtures/runtime_v2/invariants/violation-0001.json"
    ))
    .expect("parse negative fixture");

    assert_eq!(
        positive["schema_version"],
        "runtime_v2.csm_run_packet_contract.v1"
    );
    assert_eq!(
        negative["schema_version"],
        "runtime_v2.invariant_violation.v1"
    );
    assert_eq!(
        negative["result"]["blocked_before_commit"],
        serde_json::Value::Bool(true)
    );
    assert_eq!(
        negative["result"]["resulting_state"],
        "transition_refused_state_unchanged"
    );
}

#[test]
fn runtime_v2_csm_boot_admission_contract_is_stable() {
    let artifacts = runtime_v2_csm_boot_admission_contract().expect("boot admission artifacts");
    artifacts
        .validate()
        .expect("valid boot admission artifacts");

    assert_eq!(
        artifacts.boot_manifest.schema_version,
        RUNTIME_V2_CSM_BOOT_MANIFEST_SCHEMA
    );
    assert_eq!(artifacts.boot_manifest.demo_id, "D3");
    assert_eq!(artifacts.boot_manifest.manifold_id, "proto-csm-01");
    assert_eq!(
        artifacts.boot_manifest.artifact_path,
        "runtime_v2/csm_run/boot_manifest.json"
    );
    assert_eq!(artifacts.boot_manifest.admitted_citizens.len(), 2);
    assert!(
        artifacts
            .boot_manifest
            .admitted_citizens
            .iter()
            .any(|receipt| receipt.citizen_id == "proto-citizen-alpha"
                && receipt.can_execute_episodes)
    );
    assert!(
        artifacts
            .boot_manifest
            .admitted_citizens
            .iter()
            .any(|receipt| receipt.citizen_id == "proto-citizen-beta"
                && !receipt.can_execute_episodes)
    );
    assert_eq!(artifacts.citizen_roster.entries.len(), 2);
    assert_eq!(artifacts.admission_trace.len(), 4);
    assert!(artifacts
        .boot_manifest
        .claim_boundary
        .contains("not a true Godel-agent birthday"));
}

#[test]
fn runtime_v2_csm_boot_admission_contract_matches_golden_fixtures() {
    let artifacts = runtime_v2_csm_boot_admission_contract().expect("boot admission artifacts");
    let boot_manifest = String::from_utf8(
        artifacts
            .boot_manifest
            .to_pretty_json_bytes()
            .expect("json"),
    )
    .expect("utf8");
    let citizen_roster = String::from_utf8(
        artifacts
            .citizen_roster
            .to_pretty_json_bytes()
            .expect("json"),
    )
    .expect("utf8");
    let admission_trace = String::from_utf8(
        artifacts
            .admission_trace_jsonl_bytes()
            .expect("trace jsonl"),
    )
    .expect("utf8");

    assert_eq!(
        boot_manifest,
        include_str!("../../tests/fixtures/runtime_v2/csm_run/boot_manifest.json").trim_end()
    );
    assert_eq!(
        citizen_roster,
        include_str!("../../tests/fixtures/runtime_v2/csm_run/citizen_roster.json").trim_end()
    );
    assert_eq!(
        admission_trace,
        include_str!("../../tests/fixtures/runtime_v2/csm_run/boot_admission_trace.jsonl")
    );
}

#[test]
fn runtime_v2_csm_boot_admission_writes_without_path_leakage() {
    let temp_root = unique_temp_path("csm-boot-admission");
    let artifacts = runtime_v2_csm_boot_admission_contract().expect("boot admission artifacts");

    artifacts
        .write_to_root(&temp_root)
        .expect("write boot admission artifacts");

    let boot_path = temp_root.join(&artifacts.boot_manifest.artifact_path);
    let roster_path = temp_root.join(&artifacts.citizen_roster.artifact_path);
    let trace_path = temp_root.join(&artifacts.admission_trace_path);
    assert!(boot_path.is_file());
    assert!(roster_path.is_file());
    assert!(trace_path.is_file());
    let boot_text = fs::read_to_string(boot_path).expect("boot text");
    let roster_text = fs::read_to_string(roster_path).expect("roster text");
    let trace_text = fs::read_to_string(trace_path).expect("trace text");
    let temp_root_text = temp_root.to_string_lossy();
    assert!(!boot_text.contains(temp_root_text.as_ref()));
    assert!(!roster_text.contains(temp_root_text.as_ref()));
    assert!(!trace_text.contains(temp_root_text.as_ref()));
    assert!(boot_text.contains("\"schema_version\": \"runtime_v2.csm_boot_manifest.v1\""));
    assert_eq!(trace_text.lines().count(), 4);

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_csm_boot_admission_validation_rejects_unsafe_or_ambiguous_state() {
    let mut artifacts = runtime_v2_csm_boot_admission_contract().expect("boot admission artifacts");
    artifacts.boot_manifest.artifact_path =
        "/tmp/runtime_v2/csm_run/boot_manifest.json".to_string();
    assert!(artifacts
        .validate()
        .expect_err("absolute boot path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut artifacts = runtime_v2_csm_boot_admission_contract().expect("boot admission artifacts");
    artifacts
        .boot_manifest
        .admitted_citizens
        .retain(|receipt| receipt.citizen_id != "proto-citizen-beta");
    assert!(artifacts
        .validate()
        .expect_err("missing beta should fail")
        .to_string()
        .contains("exactly two worker citizens"));

    let mut artifacts = runtime_v2_csm_boot_admission_contract().expect("boot admission artifacts");
    artifacts.admission_trace[1].event_sequence = 9;
    assert!(artifacts
        .validate()
        .expect_err("non-contiguous trace should fail")
        .to_string()
        .contains("contiguous"));

    let mut artifacts = runtime_v2_csm_boot_admission_contract().expect("boot admission artifacts");
    artifacts.boot_manifest.claim_boundary = "first true Godel-agent birthday".to_string();
    assert!(artifacts
        .validate()
        .expect_err("birthday overclaim should fail")
        .to_string()
        .contains("non-claims"));
}

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
        include_str!("../../tests/fixtures/runtime_v2/csm_run/resource_pressure_fixture.json")
            .trim_end()
    );
    assert_eq!(
        scheduling_decision,
        include_str!("../../tests/fixtures/runtime_v2/csm_run/scheduling_decision.json").trim_end()
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
        include_str!("../../tests/fixtures/runtime_v2/csm_run/citizen_action_fixture.json")
            .trim_end()
    );
    assert_eq!(
        freedom_gate_decision,
        include_str!("../../tests/fixtures/runtime_v2/csm_run/freedom_gate_decision.json")
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
        include_str!("../../tests/fixtures/runtime_v2/csm_run/invalid_action_fixture.json")
            .trim_end()
    );
    assert_eq!(
        violation_packet,
        include_str!("../../tests/fixtures/runtime_v2/csm_run/invalid_action_violation.json")
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
fn runtime_v2_csm_wake_continuity_contract_is_stable() {
    let artifacts = runtime_v2_csm_wake_continuity_contract().expect("wake continuity artifacts");
    artifacts
        .validate()
        .expect("valid wake continuity artifacts");

    assert_eq!(
        artifacts.wake_continuity_proof.schema_version,
        RUNTIME_V2_CSM_WAKE_CONTINUITY_PROOF_SCHEMA
    );
    assert_eq!(artifacts.wake_continuity_proof.demo_id, "D6");
    assert_eq!(
        artifacts.wake_continuity_proof.snapshot_ref,
        "runtime_v2/snapshots/snapshot-0001.json"
    );
    assert_eq!(
        artifacts.wake_continuity_proof.rehydration_report_ref,
        "runtime_v2/rehydration_report.json"
    );
    assert_eq!(
        artifacts.wake_continuity_proof.proof_outcome,
        "wake_allowed_unique_active_head"
    );
    assert_eq!(
        artifacts.wake_continuity_proof.wake_trace_sequence,
        artifacts
            .snapshot_rehydration
            .rehydration_report
            .trace_resume_sequence
    );
    assert!(
        !artifacts
            .wake_continuity_proof
            .duplicate_activation_guard
            .duplicate_active_citizen_detected
    );
    assert_eq!(artifacts.first_run_trace.len(), 9);
    assert!(artifacts.first_run_trace.iter().any(|event| {
        event.event_id == "csm_citizens_woken_without_duplicate_activation"
            && event.outcome == "woken_without_duplicate"
            && event.artifact_ref == artifacts.wake_continuity_proof.artifact_path
    }));
    assert!(artifacts
        .wake_continuity_proof
        .claim_boundary
        .contains("WP-09 D6 snapshot rehydrate wake continuity"));
}

#[test]
fn runtime_v2_csm_wake_continuity_contract_matches_golden_fixtures() {
    let artifacts = runtime_v2_csm_wake_continuity_contract().expect("wake continuity artifacts");
    let wake_continuity_proof = String::from_utf8(
        artifacts
            .wake_continuity_proof_pretty_json_bytes()
            .expect("json"),
    )
    .expect("utf8");
    let first_run_trace = String::from_utf8(
        artifacts
            .first_run_trace_jsonl_bytes()
            .expect("trace jsonl"),
    )
    .expect("utf8");

    assert_eq!(
        wake_continuity_proof,
        include_str!("../../tests/fixtures/runtime_v2/csm_run/wake_continuity_proof.json")
            .trim_end()
    );
    assert_eq!(
        first_run_trace,
        include_str!("../../tests/fixtures/runtime_v2/csm_run/first_run_trace.jsonl")
    );
}

#[test]
fn runtime_v2_csm_wake_continuity_writes_without_path_leakage() {
    let temp_root = unique_temp_path("csm-wake-continuity");
    let artifacts = runtime_v2_csm_wake_continuity_contract().expect("wake continuity artifacts");

    artifacts
        .write_to_root(&temp_root)
        .expect("write wake continuity artifacts");

    let snapshot_path = temp_root.join(&artifacts.snapshot_rehydration.snapshot.snapshot_path);
    let rehydration_path = temp_root.join(
        &artifacts
            .snapshot_rehydration
            .rehydration_report
            .report_path,
    );
    let proof_path = temp_root.join(&artifacts.wake_continuity_proof.artifact_path);
    let trace_path = temp_root.join(&artifacts.first_run_trace_path);
    assert!(snapshot_path.is_file());
    assert!(rehydration_path.is_file());
    assert!(proof_path.is_file());
    assert!(trace_path.is_file());
    let proof_text = fs::read_to_string(proof_path).expect("proof text");
    let trace_text = fs::read_to_string(trace_path).expect("trace text");
    let temp_root_text = temp_root.to_string_lossy();
    assert!(!proof_text.contains(temp_root_text.as_ref()));
    assert!(!trace_text.contains(temp_root_text.as_ref()));
    assert!(proof_text.contains("\"schema_version\": \"runtime_v2.csm_wake_continuity_proof.v1\""));
    assert_eq!(trace_text.lines().count(), 9);

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_csm_wake_continuity_validation_rejects_unsafe_or_ambiguous_state() {
    let mut artifacts =
        runtime_v2_csm_wake_continuity_contract().expect("wake continuity artifacts");
    artifacts.wake_continuity_proof.artifact_path =
        "/tmp/runtime_v2/csm_run/wake_continuity_proof.json".to_string();
    assert!(artifacts
        .validate()
        .expect_err("absolute proof path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut artifacts =
        runtime_v2_csm_wake_continuity_contract().expect("wake continuity artifacts");
    artifacts
        .snapshot_rehydration
        .rehydration_report
        .duplicate_active_citizen_detected = true;
    assert!(artifacts
        .validate()
        .expect_err("duplicate active wake should fail")
        .to_string()
        .contains("duplicate active citizen"));

    let mut artifacts =
        runtime_v2_csm_wake_continuity_contract().expect("wake continuity artifacts");
    artifacts
        .wake_continuity_proof
        .restored_active_citizens
        .push("proto-citizen-alpha".to_string());
    assert!(artifacts
        .validate()
        .expect_err("duplicate restored citizen should fail")
        .to_string()
        .contains("restored citizens"));

    let mut artifacts =
        runtime_v2_csm_wake_continuity_contract().expect("wake continuity artifacts");
    artifacts.first_run_trace[8].artifact_ref = "runtime_v2/csm_run/other.json".to_string();
    assert!(artifacts
        .validate()
        .expect_err("trace missing proof should fail")
        .to_string()
        .contains("present in the first-run trace"));

    let mut artifacts =
        runtime_v2_csm_wake_continuity_contract().expect("wake continuity artifacts");
    artifacts
        .wake_continuity_proof
        .continuity_checks
        .retain(|check| check.invariant_id != "no_duplicate_active_citizen_instance");
    assert!(artifacts
        .validate()
        .expect_err("missing duplicate-head guard should fail")
        .to_string()
        .contains("wake continuity proof"));

    let mut artifacts =
        runtime_v2_csm_wake_continuity_contract().expect("wake continuity artifacts");
    artifacts.wake_continuity_proof.claim_boundary = "live birthday".to_string();
    assert!(artifacts
        .validate()
        .expect_err("overclaim should fail")
        .to_string()
        .contains("non-claims"));
}
