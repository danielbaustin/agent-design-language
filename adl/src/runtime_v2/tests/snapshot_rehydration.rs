use super::common::unique_temp_path;
use super::*;
use std::fs;

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
        include_str!("../../../tests/fixtures/runtime_v2/snapshots/snapshot-0001.json").trim_end()
    );
    assert_eq!(
        rehydration,
        include_str!("../../../tests/fixtures/runtime_v2/rehydration_report.json").trim_end()
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
