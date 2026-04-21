use super::common::unique_temp_path;
use super::*;
use std::fs;

#[test]
fn runtime_v2_csm_integrated_run_contract_is_stable() {
    let artifacts = runtime_v2_csm_integrated_run_contract().expect("integrated run artifacts");
    artifacts
        .validate()
        .expect("valid integrated run artifacts");

    assert_eq!(
        artifacts.proof_packet.schema_version,
        RUNTIME_V2_CSM_INTEGRATED_RUN_PROOF_SCHEMA
    );
    assert_eq!(artifacts.proof_packet.demo_id, "D10");
    assert_eq!(artifacts.proof_packet.manifold_id, "proto-csm-01");
    assert_eq!(
        artifacts.proof_packet.artifact_path,
        "runtime_v2/csm_run/integrated_first_run_proof_packet.json"
    );
    assert_eq!(
        artifacts.proof_packet.execution_transcript_ref,
        "runtime_v2/csm_run/integrated_first_run_transcript.jsonl"
    );
    assert_eq!(artifacts.execution_transcript.len(), 10);
    assert!(artifacts
        .execution_summary()
        .expect("execution summary")
        .contains("observatory_rendered"));
    assert_eq!(artifacts.proof_packet.proof_classification, "proving");
    assert!(artifacts
        .proof_packet
        .integrated_stage_refs
        .iter()
        .any(|artifact| artifact == "runtime_v2/hardening/hardening_proof_packet.json"));
    assert!(artifacts
        .proof_packet
        .validation_commands
        .iter()
        .any(|command| command.contains("integrated-csm-run-demo")));
}

#[test]
fn runtime_v2_csm_integrated_run_contract_matches_golden_fixture() {
    let artifacts = runtime_v2_csm_integrated_run_contract().expect("integrated run artifacts");
    let proof = String::from_utf8(
        artifacts
            .proof_packet_pretty_json_bytes()
            .expect("proof json"),
    )
    .expect("utf8 proof");

    assert_eq!(
        proof,
        include_str!(
            "../../../tests/fixtures/runtime_v2/csm_run/integrated_first_run_proof_packet.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_csm_integrated_run_writes_without_path_leakage() {
    let temp_root = unique_temp_path("csm-integrated-run");
    let artifacts = runtime_v2_csm_integrated_run_contract().expect("integrated run artifacts");

    artifacts
        .write_to_root(&temp_root)
        .expect("write integrated run artifacts");

    let proof_path = temp_root.join(&artifacts.proof_packet.artifact_path);
    assert!(proof_path.is_file());
    assert!(temp_root
        .join("runtime_v2/observatory/visibility_packet.json")
        .is_file());
    assert!(temp_root
        .join("runtime_v2/csm_run/integrated_first_run_transcript.jsonl")
        .is_file());
    assert!(temp_root
        .join("runtime_v2/hardening/hardening_proof_packet.json")
        .is_file());
    let text = fs::read_to_string(proof_path).expect("proof text");
    assert!(!text.contains(temp_root.to_string_lossy().as_ref()));
    assert!(text.contains("\"schema_version\": \"runtime_v2.csm_integrated_run_proof_packet.v1\""));
    assert!(text.contains("\"demo_id\": \"D10\""));
    assert!(text.contains("bounded D10 integrated first-run evidence package"));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_csm_integrated_run_validation_rejects_unsafe_or_ambiguous_state() {
    let mut artifacts = runtime_v2_csm_integrated_run_contract().expect("integrated run artifacts");
    artifacts.proof_packet.artifact_path =
        "/tmp/runtime_v2/csm_run/integrated_first_run_proof_packet.json".to_string();
    assert!(artifacts
        .validate()
        .expect_err("absolute path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut artifacts = runtime_v2_csm_integrated_run_contract().expect("integrated run artifacts");
    artifacts
        .proof_packet
        .hardening_refs
        .retain(|artifact| artifact != "runtime_v2/hardening/hardening_proof_packet.json");
    assert!(artifacts
        .validate()
        .expect_err("missing hardening proof should fail")
        .to_string()
        .contains("hardening_proof_packet"));

    let mut artifacts = runtime_v2_csm_integrated_run_contract().expect("integrated run artifacts");
    artifacts.proof_packet.proof_classification = "non_proving".to_string();
    assert!(artifacts
        .validate()
        .expect_err("non-proving classification should fail")
        .to_string()
        .contains("classified as proving"));

    let mut artifacts = runtime_v2_csm_integrated_run_contract().expect("integrated run artifacts");
    artifacts.proof_packet.claim_boundary = "live birthday achieved".to_string();
    assert!(artifacts
        .validate()
        .expect_err("overclaim should fail")
        .to_string()
        .contains("bounded D10 claim boundary"));
}
