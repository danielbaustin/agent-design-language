#[cfg(feature = "slow-proof-tests")]
use super::common::unique_temp_path;
use super::*;
#[cfg(feature = "slow-proof-tests")]
use std::fs;

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
        include_str!("../../../tests/fixtures/runtime_v2/security_boundary/proof_packet.json")
            .trim_end()
    );
}

#[cfg(feature = "slow-proof-tests")]
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
