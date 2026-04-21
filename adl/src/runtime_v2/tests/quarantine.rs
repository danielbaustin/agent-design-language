use super::common::unique_temp_path;
use super::*;
use std::fs;

#[test]
fn runtime_v2_csm_quarantine_contract_is_stable() {
    let artifacts = runtime_v2_csm_quarantine_contract().expect("quarantine artifacts");
    artifacts.validate().expect("valid quarantine artifacts");

    assert_eq!(
        artifacts.unsafe_recovery_fixture.schema_version,
        RUNTIME_V2_CSM_QUARANTINE_FIXTURE_SCHEMA
    );
    assert_eq!(artifacts.unsafe_recovery_fixture.demo_id, "D8");
    assert_eq!(
        artifacts.unsafe_recovery_fixture.source_decision_ref,
        "runtime_v2/recovery/quarantine_required_decision.json"
    );
    assert_eq!(
        artifacts.quarantine_artifact.schema_version,
        RUNTIME_V2_CSM_QUARANTINE_ARTIFACT_SCHEMA
    );
    assert_eq!(
        artifacts.quarantine_artifact.quarantine_state,
        "execution_blocked_pending_operator_review"
    );
    assert_eq!(artifacts.quarantine_artifact.triggers.len(), 2);
    assert!(artifacts
        .quarantine_artifact
        .blocked_actions
        .contains(&"resume_without_operator_review".to_string()));
    assert_eq!(artifacts.quarantine_artifact.state_machine.len(), 3);
    assert_eq!(
        artifacts.quarantine_artifact.state_machine[2].to_state,
        "execution_blocked_pending_operator_review"
    );
    assert_eq!(
        artifacts.evidence_preservation.schema_version,
        RUNTIME_V2_CSM_QUARANTINE_EVIDENCE_SCHEMA
    );
    assert_eq!(artifacts.evidence_preservation.evidence_count, 5);
    assert!(!artifacts.evidence_preservation.mutation_allowed);
    assert!(!artifacts.evidence_preservation.prune_allowed_before_review);
}

#[test]
fn runtime_v2_csm_quarantine_contract_matches_golden_fixtures() {
    let artifacts = runtime_v2_csm_quarantine_contract().expect("quarantine artifacts");
    let unsafe_fixture = String::from_utf8(
        artifacts
            .unsafe_recovery_fixture_pretty_json_bytes()
            .expect("fixture json"),
    )
    .expect("utf8 fixture");
    let quarantine = String::from_utf8(
        artifacts
            .quarantine_artifact_pretty_json_bytes()
            .expect("quarantine json"),
    )
    .expect("utf8 quarantine");
    let evidence = String::from_utf8(
        artifacts
            .evidence_preservation_pretty_json_bytes()
            .expect("evidence json"),
    )
    .expect("utf8 evidence");

    assert_eq!(
        unsafe_fixture,
        include_str!("../../../tests/fixtures/runtime_v2/quarantine/unsafe_recovery_fixture.json")
            .trim_end()
    );
    assert_eq!(
        quarantine,
        include_str!("../../../tests/fixtures/runtime_v2/quarantine/quarantine_artifact.json")
            .trim_end()
    );
    assert_eq!(
        evidence,
        include_str!(
            "../../../tests/fixtures/runtime_v2/quarantine/evidence_preservation_artifact.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_csm_quarantine_writes_without_path_leakage() {
    let temp_root = unique_temp_path("csm-quarantine");
    let artifacts = runtime_v2_csm_quarantine_contract().expect("quarantine artifacts");

    artifacts
        .write_to_root(&temp_root)
        .expect("write quarantine artifacts");

    let fixture_path = temp_root.join(&artifacts.unsafe_recovery_fixture.artifact_path);
    let quarantine_path = temp_root.join(&artifacts.quarantine_artifact.artifact_path);
    let evidence_path = temp_root.join(&artifacts.evidence_preservation.artifact_path);
    assert!(fixture_path.is_file());
    assert!(quarantine_path.is_file());
    assert!(evidence_path.is_file());

    let temp_root_text = temp_root.to_string_lossy();
    for path in [fixture_path, quarantine_path, evidence_path] {
        let text = fs::read_to_string(path).expect("artifact text");
        assert!(!text.contains(temp_root_text.as_ref()));
        assert!(text.contains("\"demo_id\": \"D8\""));
        assert!(text.contains("first true Godel-agent birth"));
    }

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_csm_quarantine_validation_rejects_unsafe_or_ambiguous_state() {
    let mut artifacts = runtime_v2_csm_quarantine_contract().expect("quarantine artifacts");
    artifacts.quarantine_artifact.artifact_path =
        "/tmp/runtime_v2/quarantine/quarantine_artifact.json".to_string();
    assert!(artifacts
        .validate()
        .expect_err("absolute quarantine path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut artifacts = runtime_v2_csm_quarantine_contract().expect("quarantine artifacts");
    artifacts.unsafe_recovery_fixture.attempted_predecessor_ref =
        Some("runtime_v2/snapshots/snapshot-0001.json".to_string());
    assert!(artifacts
        .validate()
        .expect_err("unsafe fixture with predecessor should fail")
        .to_string()
        .contains("ambiguous predecessor linkage"));

    let mut artifacts = runtime_v2_csm_quarantine_contract().expect("quarantine artifacts");
    artifacts.quarantine_artifact.state_machine[2].to_state = "active".to_string();
    assert!(artifacts
        .validate()
        .expect_err("direct active transition should fail")
        .to_string()
        .contains("active state"));

    let mut artifacts = runtime_v2_csm_quarantine_contract().expect("quarantine artifacts");
    artifacts.evidence_preservation.mutation_allowed = true;
    assert!(artifacts
        .validate()
        .expect_err("mutable evidence should fail")
        .to_string()
        .contains("immutable until operator review"));

    let mut artifacts = runtime_v2_csm_quarantine_contract().expect("quarantine artifacts");
    artifacts.quarantine_artifact.claim_boundary =
        "live Runtime v2 quarantine succeeded".to_string();
    assert!(artifacts
        .validate()
        .expect_err("overclaim should fail")
        .to_string()
        .contains("non-claim"));
}
