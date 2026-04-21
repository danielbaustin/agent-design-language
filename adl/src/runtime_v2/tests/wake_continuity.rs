use super::common::unique_temp_path;
use super::*;
use std::fs;

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
        include_str!("../../../tests/fixtures/runtime_v2/csm_run/wake_continuity_proof.json")
            .trim_end()
    );
    assert_eq!(
        first_run_trace,
        include_str!("../../../tests/fixtures/runtime_v2/csm_run/first_run_trace.jsonl")
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
fn runtime_v2_csm_wake_continuity_proof_standalone_writes_without_path_leakage() {
    let temp_root = unique_temp_path("csm-wake-proof-standalone");
    let artifacts = runtime_v2_csm_wake_continuity_contract().expect("wake continuity artifacts");

    let proof_json = String::from_utf8(
        artifacts
            .wake_continuity_proof
            .to_pretty_json_bytes()
            .expect("standalone proof json"),
    )
    .expect("utf8 proof json");
    assert!(proof_json.contains("\"proof_outcome\": \"wake_allowed_unique_active_head\""));

    artifacts
        .wake_continuity_proof
        .write_to_root(&temp_root)
        .expect("write standalone wake proof");

    let proof_path = temp_root.join(&artifacts.wake_continuity_proof.artifact_path);
    assert!(proof_path.is_file());
    let proof_text = fs::read_to_string(proof_path).expect("proof text");
    assert!(!proof_text.contains(temp_root.to_string_lossy().as_ref()));
    assert!(proof_text.contains("\"demo_id\": \"D6\""));

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

#[test]
fn runtime_v2_csm_wake_continuity_validation_rejects_guard_and_lineage_drift() {
    let mut artifacts =
        runtime_v2_csm_wake_continuity_contract().expect("wake continuity artifacts");
    artifacts
        .wake_continuity_proof
        .duplicate_activation_guard
        .attempted_duplicate_active_heads = true;
    assert!(artifacts
        .validate()
        .expect_err("attempted duplicate active head should fail")
        .to_string()
        .contains("unique active head"));

    let mut artifacts =
        runtime_v2_csm_wake_continuity_contract().expect("wake continuity artifacts");
    artifacts
        .wake_continuity_proof
        .duplicate_activation_guard
        .guard_result = "quarantined_duplicate_head".to_string();
    assert!(artifacts
        .validate()
        .expect_err("unsupported guard result should fail")
        .to_string()
        .contains("guard_result"));

    let mut artifacts =
        runtime_v2_csm_wake_continuity_contract().expect("wake continuity artifacts");
    artifacts.wake_continuity_proof.citizen_continuity[0].restored_record_ref =
        "runtime_v2/citizens/other.json".to_string();
    assert!(artifacts
        .validate()
        .expect_err("restored record drift should fail")
        .to_string()
        .contains("citizen refs"));

    let mut artifacts =
        runtime_v2_csm_wake_continuity_contract().expect("wake continuity artifacts");
    artifacts.wake_continuity_proof.citizen_continuity[0].continuity_status =
        "ambiguous_branch".to_string();
    assert!(artifacts
        .validate()
        .expect_err("ambiguous continuity status should fail")
        .to_string()
        .contains("continuity_status"));
}
