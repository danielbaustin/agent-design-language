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
fn runtime_v2_observatory_flagship_serializes_review_surfaces() {
    let artifacts = flagship_artifacts();

    let proof_json: serde_json::Value = serde_json::from_slice(
        &artifacts
            .proof_packet_pretty_json_bytes()
            .expect("proof packet json"),
    )
    .expect("proof packet should be json");
    assert_eq!(
        proof_json["schema_version"],
        RUNTIME_V2_OBSERVATORY_FLAGSHIP_PROOF_SCHEMA
    );
    assert_eq!(proof_json["demo_id"], "D12");

    let walkthrough = String::from_utf8(
        artifacts
            .walkthrough_jsonl_bytes()
            .expect("walkthrough jsonl"),
    )
    .expect("walkthrough should be utf8");
    let steps: Vec<RuntimeV2ObservatoryFlagshipWalkthroughStep> = walkthrough
        .lines()
        .map(|line| serde_json::from_str(line).expect("walkthrough step json"))
        .collect();
    assert_eq!(steps.len(), 7);
    assert_eq!(steps[0].room, "World / Reality");
    assert_eq!(steps[6].room, "Corporate Investor");

    let summary = artifacts.execution_summary().expect("execution summary");
    assert!(summary.contains("D12 inhabited CSM Observatory flagship proof"));
    assert!(summary.contains("World / Reality"));
    assert!(summary.contains("Corporate Investor"));
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

#[test]
fn runtime_v2_observatory_flagship_shape_rejects_malformed_fields() {
    let packet = flagship_artifacts().proof_packet;

    let mut bad_schema = packet.clone();
    bad_schema.schema_version = "runtime_v2.observatory_flagship_proof_packet.v0".to_string();
    assert!(bad_schema
        .validate_shape()
        .expect_err("bad schema should fail")
        .to_string()
        .contains("unsupported Runtime v2 Observatory flagship proof schema"));

    let mut bad_demo = packet.clone();
    bad_demo.demo_id = "D99".to_string();
    assert!(bad_demo
        .validate_shape()
        .expect_err("bad demo id should fail")
        .to_string()
        .contains("demo matrix row D12"));

    let mut bad_milestone = packet.clone();
    bad_milestone.milestone = "v0.91.0".to_string();
    assert!(bad_milestone
        .validate_shape()
        .expect_err("bad milestone should fail")
        .to_string()
        .contains("must target v0.90.3"));

    let mut absolute_path = packet.clone();
    absolute_path.artifact_path = "/tmp/flagship.json".to_string();
    assert!(absolute_path
        .validate_shape()
        .expect_err("absolute proof path should fail")
        .to_string()
        .contains("observatory_flagship.artifact_path"));

    let mut duplicate_ref = packet.clone();
    duplicate_ref
        .required_artifact_refs
        .push(duplicate_ref.required_artifact_refs[0].clone());
    assert!(duplicate_ref
        .validate_shape()
        .expect_err("duplicate artifact refs should fail")
        .to_string()
        .contains("duplicate path"));
}

#[test]
fn runtime_v2_observatory_flagship_shape_rejects_missing_review_claims() {
    let packet = flagship_artifacts().proof_packet;

    let mut missing_command = packet.clone();
    missing_command.validation_commands.clear();
    missing_command.validation_commands.push(
        "cargo test --manifest-path adl/Cargo.toml runtime_v2_observatory_flagship".to_string(),
    );
    assert!(missing_command
        .validate_shape()
        .expect_err("missing runnable demo command should fail")
        .to_string()
        .contains("runnable demo validation command"));

    let mut missing_summary_phrase = packet.clone();
    missing_summary_phrase.proof_summary =
        "D12 integrates witness, receipt, and redacted projection evidence.".to_string();
    assert!(missing_summary_phrase
        .validate_shape()
        .expect_err("missing WP phrase should fail")
        .to_string()
        .contains("must mention WP-03"));

    let mut missing_personhood_non_claim = packet.clone();
    missing_personhood_non_claim.non_claims = vec![
        "does not expose canonical private citizen state".to_string(),
        "does not claim first true Godel-agent birthday".to_string(),
    ];
    assert!(missing_personhood_non_claim
        .validate_shape()
        .expect_err("missing personhood non-claim should fail")
        .to_string()
        .contains("personhood non-claim"));

    let mut missing_birthday_non_claim = packet.clone();
    missing_birthday_non_claim.non_claims = vec!["does not prove personhood".to_string()];
    assert!(missing_birthday_non_claim
        .validate_shape()
        .expect_err("missing birthday non-claim should fail")
        .to_string()
        .contains("first-birthday non-claim"));
}

#[test]
fn runtime_v2_observatory_flagship_shape_rejects_bad_actor_roster_and_walkthrough() {
    let packet = flagship_artifacts().proof_packet;

    let mut missing_actor = packet.clone();
    missing_actor.actor_roster.pop();
    assert!(missing_actor
        .validate_shape()
        .expect_err("missing actor should fail")
        .to_string()
        .contains("citizen, guest, service, and operator actors"));

    let mut missing_standing = packet.clone();
    missing_standing.actor_roster[0].standing_class = "operator".to_string();
    assert!(missing_standing
        .validate_shape()
        .expect_err("missing citizen standing should fail")
        .to_string()
        .contains("missing citizen standing"));

    let mut bad_walkthrough_schema = packet.clone();
    bad_walkthrough_schema.lens_sequence[0].schema_version =
        "runtime_v2.observatory_flagship_walkthrough_step.v0".to_string();
    assert!(bad_walkthrough_schema
        .validate_shape()
        .expect_err("bad walkthrough schema should fail")
        .to_string()
        .contains("unsupported Runtime v2 Observatory flagship walkthrough schema"));

    let mut bad_sequence = packet.clone();
    bad_sequence.lens_sequence[1].sequence = 7;
    assert!(bad_sequence
        .validate_shape()
        .expect_err("bad sequence should fail")
        .to_string()
        .contains("sequence must be contiguous"));

    let mut missing_room = packet.clone();
    missing_room.lens_sequence[6].room = "Operator / Governance".to_string();
    assert!(missing_room
        .validate_shape()
        .expect_err("missing room should fail")
        .to_string()
        .contains("missing expected room 'Corporate Investor'"));
}
