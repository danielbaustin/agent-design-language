use super::common::unique_temp_path;
use super::*;
use std::fs;
use std::sync::OnceLock;

fn flagship_artifacts() -> RuntimeV2ObservatoryFlagshipArtifacts {
    static ARTIFACTS: OnceLock<RuntimeV2ObservatoryFlagshipArtifacts> = OnceLock::new();
    ARTIFACTS
        .get_or_init(|| {
            runtime_v2_observatory_flagship_contract().expect("observatory flagship artifacts")
        })
        .clone()
}

#[test]
fn runtime_v2_observatory_flagship_contract_is_stable() {
    let artifacts = flagship_artifacts();
    artifacts
        .validate()
        .expect("valid observatory flagship artifacts");

    assert_eq!(
        artifacts.proof_packet.schema_version,
        RUNTIME_V2_OBSERVATORY_FLAGSHIP_PROOF_SCHEMA
    );
    assert_eq!(artifacts.proof_packet.demo_id, "D12");
    assert_eq!(artifacts.proof_packet.milestone, "v0.90.3");
    assert_eq!(
        artifacts.proof_packet.artifact_path,
        "runtime_v2/observatory/flagship_proof_packet.json"
    );
    assert_eq!(
        artifacts.proof_packet.operator_report_ref,
        "runtime_v2/observatory/flagship_operator_report.md"
    );
    assert_eq!(artifacts.proof_packet.proof_classification, "proving");
    assert_eq!(artifacts.proof_packet.lens_sequence.len(), 7);
    assert!(artifacts
        .proof_packet
        .actor_roster
        .iter()
        .any(|actor| actor.standing_class == "citizen"));
    assert!(artifacts
        .proof_packet
        .required_artifact_refs
        .iter()
        .any(|artifact| artifact == "runtime_v2/private_state/continuity_witnesses.json"));
    assert!(artifacts
        .proof_packet
        .required_artifact_refs
        .iter()
        .any(|artifact| artifact == "runtime_v2/private_state/citizen_receipts.json"));
    assert!(artifacts
        .proof_packet
        .required_artifact_refs
        .iter()
        .any(|artifact| artifact == "runtime_v2/observatory/private_state_projection_packet.json"));
    assert!(artifacts
        .proof_packet
        .required_artifact_refs
        .iter()
        .any(|artifact| artifact == "runtime_v2/access_control/access_events.json"));
    assert!(artifacts
        .proof_packet
        .required_artifact_refs
        .iter()
        .any(|artifact| artifact == "runtime_v2/challenge/challenge_artifact.json"));
    assert!(artifacts
        .proof_packet
        .reviewer_command
        .contains("observatory-flagship-demo"));
    assert!(artifacts
        .operator_report_markdown
        .contains("D12 Inhabited CSM Observatory Flagship"));
}

#[test]
#[ignore = "full D12 filesystem smoke is validated by the explicit observatory-flagship-demo command; keep always-on coverage bounded"]
fn runtime_v2_observatory_flagship_writes_integrated_artifacts_without_path_leakage() {
    let temp_root = unique_temp_path("observatory-flagship");
    let artifacts = flagship_artifacts();

    artifacts
        .write_to_root(&temp_root)
        .expect("write observatory flagship artifacts");

    let proof_path = temp_root.join("runtime_v2/observatory/flagship_proof_packet.json");
    assert!(proof_path.is_file());
    assert!(temp_root
        .join("runtime_v2/private_state/continuity_witnesses.json")
        .is_file());
    assert!(temp_root
        .join("runtime_v2/private_state/citizen_receipts.json")
        .is_file());
    assert!(temp_root
        .join("runtime_v2/observatory/private_state_projection_packet.json")
        .is_file());
    assert!(temp_root
        .join("runtime_v2/access_control/access_events.json")
        .is_file());
    assert!(temp_root
        .join("runtime_v2/challenge/challenge_artifact.json")
        .is_file());
    assert!(temp_root
        .join("runtime_v2/observatory/flagship_operator_report.md")
        .is_file());
    assert!(temp_root
        .join("runtime_v2/observatory/flagship_walkthrough.jsonl")
        .is_file());

    let proof_text = fs::read_to_string(proof_path).expect("proof text");
    assert!(!proof_text.contains(temp_root.to_string_lossy().as_ref()));
    assert!(proof_text
        .contains("\"schema_version\": \"runtime_v2.observatory_flagship_proof_packet.v1\""));
    assert!(proof_text.contains("\"demo_id\": \"D12\""));
    assert!(proof_text.contains("bounded local D12 citizen-state Observatory evidence package"));

    let report_text =
        fs::read_to_string(temp_root.join("runtime_v2/observatory/flagship_operator_report.md"))
            .expect("report text");
    assert!(report_text.contains("citizen receipt set"));
    assert!(report_text.contains("Non-claims"));
    assert!(!report_text.contains("private_payload_b64"));
    assert!(!report_text.contains("sealed_payload_b64"));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_observatory_flagship_validation_rejects_overclaim_or_missing_evidence() {
    let mut artifacts = flagship_artifacts();
    artifacts.proof_packet.proof_classification = "non_proving".to_string();
    assert!(artifacts
        .validate()
        .expect_err("non-proving classification should fail")
        .to_string()
        .contains("classified as proving"));

    let mut artifacts = flagship_artifacts();
    artifacts.proof_packet.claim_boundary = "personhood proven".to_string();
    assert!(artifacts
        .validate()
        .expect_err("overclaim boundary should fail")
        .to_string()
        .contains("bounded D12 claim boundary"));

    let mut artifacts = flagship_artifacts();
    artifacts
        .proof_packet
        .required_artifact_refs
        .retain(|artifact| artifact != "runtime_v2/private_state/citizen_receipts.json");
    assert!(artifacts
        .validate()
        .expect_err("missing receipt should fail")
        .to_string()
        .contains("citizen_receipts.json"));

    let mut artifacts = flagship_artifacts();
    artifacts
        .operator_report_markdown
        .push_str("\nsealed_payload_b64");
    assert!(artifacts
        .validate()
        .expect_err("leakage token should fail")
        .to_string()
        .contains("leaked forbidden private-state token"));
}
