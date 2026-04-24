#[cfg(feature = "slow-proof-tests")]
use super::common::unique_temp_path;
use super::*;
#[cfg(feature = "slow-proof-tests")]
use std::fs;

#[test]
fn runtime_v2_csm_hardening_contract_is_stable() {
    let artifacts = runtime_v2_csm_hardening_contract().expect("hardening artifacts");
    artifacts.validate().expect("valid hardening artifacts");

    assert_eq!(
        artifacts.rules.schema_version,
        RUNTIME_V2_CSM_ADVERSARIAL_RULES_SCHEMA
    );
    assert_eq!(artifacts.rules.demo_id, "D9");
    assert!(artifacts
        .rules
        .forbidden_behaviors
        .contains(&"mutate_committed_state".to_string()));
    assert_eq!(
        artifacts.hook.schema_version,
        RUNTIME_V2_CSM_ADVERSARIAL_HOOK_SCHEMA
    );
    assert_eq!(
        artifacts.hook.actual_outcome,
        "contained_by_quarantine_execution_block"
    );
    assert!(!artifacts.hook.state_mutation_allowed);
    assert_eq!(
        artifacts.duplicate_activation_probe.actual_result,
        "duplicate_activation_refused"
    );
    assert_eq!(
        artifacts.snapshot_integrity_probe.actual_result,
        "snapshot_integrity_refused"
    );
    assert_eq!(
        artifacts.trace_replay_gap_probe.actual_result,
        "trace_replay_gap_refused"
    );
    assert_eq!(artifacts.proof_packet.proof_classification, "proving");
    assert_eq!(artifacts.proof_packet.probe_refs.len(), 3);
}

#[test]
fn runtime_v2_csm_hardening_contract_matches_golden_fixtures() {
    let artifacts = runtime_v2_csm_hardening_contract().expect("hardening artifacts");
    let rules =
        String::from_utf8(artifacts.rules_pretty_json_bytes().expect("rules json")).expect("utf8");
    let hook =
        String::from_utf8(artifacts.hook_pretty_json_bytes().expect("hook json")).expect("utf8");
    let duplicate = String::from_utf8(
        artifacts
            .duplicate_activation_probe_pretty_json_bytes()
            .expect("duplicate probe json"),
    )
    .expect("utf8 duplicate probe");
    let snapshot = String::from_utf8(
        artifacts
            .snapshot_integrity_probe_pretty_json_bytes()
            .expect("snapshot probe json"),
    )
    .expect("utf8 snapshot probe");
    let trace = String::from_utf8(
        artifacts
            .trace_replay_gap_probe_pretty_json_bytes()
            .expect("trace probe json"),
    )
    .expect("utf8 trace probe");
    let proof = String::from_utf8(
        artifacts
            .proof_packet_pretty_json_bytes()
            .expect("proof packet json"),
    )
    .expect("utf8 proof");

    assert_eq!(
        rules,
        include_str!("../../../tests/fixtures/runtime_v2/hardening/rules_of_engagement.json")
            .trim_end()
    );
    assert_eq!(
        hook,
        include_str!("../../../tests/fixtures/runtime_v2/hardening/adversarial_hook_packet.json")
            .trim_end()
    );
    assert_eq!(
        duplicate,
        include_str!(
            "../../../tests/fixtures/runtime_v2/hardening/duplicate_activation_probe.json"
        )
        .trim_end()
    );
    assert_eq!(
        snapshot,
        include_str!("../../../tests/fixtures/runtime_v2/hardening/snapshot_integrity_probe.json")
            .trim_end()
    );
    assert_eq!(
        trace,
        include_str!("../../../tests/fixtures/runtime_v2/hardening/trace_replay_gap_probe.json")
            .trim_end()
    );
    assert_eq!(
        proof,
        include_str!("../../../tests/fixtures/runtime_v2/hardening/hardening_proof_packet.json")
            .trim_end()
    );
}

#[cfg(feature = "slow-proof-tests")]
#[test]
fn runtime_v2_csm_hardening_writes_without_path_leakage() {
    let temp_root = unique_temp_path("csm-hardening");
    let artifacts = runtime_v2_csm_hardening_contract().expect("hardening artifacts");

    artifacts
        .write_to_root(&temp_root)
        .expect("write hardening artifacts");

    let paths = [
        temp_root.join(&artifacts.rules.artifact_path),
        temp_root.join(&artifacts.hook.artifact_path),
        temp_root.join(&artifacts.duplicate_activation_probe.artifact_path),
        temp_root.join(&artifacts.snapshot_integrity_probe.artifact_path),
        temp_root.join(&artifacts.trace_replay_gap_probe.artifact_path),
        temp_root.join(&artifacts.proof_packet.artifact_path),
    ];
    let temp_root_text = temp_root.to_string_lossy();
    for path in paths {
        assert!(path.is_file());
        let text = fs::read_to_string(path).expect("artifact text");
        assert!(!text.contains(temp_root_text.as_ref()));
        assert!(text.contains("\"demo_id\": \"D9\""));
        assert!(text.contains("first true Godel-agent birth"));
    }

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_csm_hardening_validation_rejects_unsafe_or_ambiguous_state() {
    let mut artifacts = runtime_v2_csm_hardening_contract().expect("hardening artifacts");
    artifacts.rules.artifact_path = "/tmp/runtime_v2/hardening/rules.json".to_string();
    assert!(artifacts
        .validate()
        .expect_err("absolute rules path should fail")
        .to_string()
        .contains("repository-relative path"));

    let mut artifacts = runtime_v2_csm_hardening_contract().expect("hardening artifacts");
    artifacts.hook.state_mutation_allowed = true;
    assert!(artifacts
        .validate()
        .expect_err("state mutation should fail")
        .to_string()
        .contains("must not allow state mutation"));

    let mut artifacts = runtime_v2_csm_hardening_contract().expect("hardening artifacts");
    artifacts.duplicate_activation_probe.blocked_before_commit = false;
    assert!(artifacts
        .validate()
        .expect_err("unblocked probe should fail")
        .to_string()
        .contains("blocked_before_commit"));

    let mut artifacts = runtime_v2_csm_hardening_contract().expect("hardening artifacts");
    artifacts.proof_packet.probe_refs.pop();
    assert!(artifacts
        .validate()
        .expect_err("missing probe ref should fail")
        .to_string()
        .contains("exactly three hardening probes"));

    let mut artifacts = runtime_v2_csm_hardening_contract().expect("hardening artifacts");
    artifacts.proof_packet.claim_boundary = "live Runtime v2 hardening succeeded".to_string();
    assert!(artifacts
        .validate()
        .expect_err("overclaim should fail")
        .to_string()
        .contains("non-claim"));
}
