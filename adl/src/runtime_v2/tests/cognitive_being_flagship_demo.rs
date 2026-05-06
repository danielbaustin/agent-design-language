use super::common::unique_temp_path;
use super::*;
use std::fs;
use std::sync::OnceLock;

fn flagship_artifacts() -> &'static RuntimeV2CognitiveBeingFlagshipArtifacts {
    static ARTIFACTS: OnceLock<RuntimeV2CognitiveBeingFlagshipArtifacts> = OnceLock::new();
    ARTIFACTS.get_or_init(|| {
        runtime_v2_cognitive_being_flagship_demo_contract()
            .expect("cognitive-being flagship artifacts")
    })
}

#[test]
fn runtime_v2_cognitive_being_flagship_demo_review_surfaces_are_stable() {
    let artifacts = flagship_artifacts();
    artifacts
        .validate()
        .expect("valid cognitive-being flagship artifacts");

    assert_eq!(
        artifacts.proof_packet.schema_version,
        RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_PROOF_SCHEMA
    );
    assert_eq!(artifacts.proof_packet.demo_id, "D13");
    assert_eq!(artifacts.proof_packet.milestone, "v0.91");
    assert_eq!(
        artifacts.proof_packet.artifact_path,
        RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_PROOF_PATH
    );
    assert_eq!(
        artifacts.proof_packet.reviewer_report_ref,
        RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_REPORT_PATH
    );
    assert_eq!(artifacts.proof_packet.section_ids.len(), 6);
    assert_eq!(
        artifacts.proof_packet.section_ids,
        vec![
            "moral_trace_and_trajectory".to_string(),
            "anti_harm_and_wellbeing".to_string(),
            "kindness_affect_reframing".to_string(),
            "moral_resources_and_cultivation".to_string(),
            "structured_planning_and_review".to_string(),
            "secure_local_comms".to_string(),
        ]
    );
    assert!(artifacts
        .proof_packet
        .reviewer_command
        .contains("cognitive-being-flagship-demo"));
    assert!(artifacts
        .proof_packet
        .required_artifact_refs
        .contains(&RUNTIME_V2_COGNITIVE_BEING_SUPPORT_MORAL_TRACE_PATH.to_string()));
    assert!(artifacts
        .proof_packet
        .required_artifact_refs
        .contains(&RUNTIME_V2_COGNITIVE_BEING_SUPPORT_SPP_PATH.to_string()));
    assert!(artifacts
        .proof_packet
        .required_artifact_refs
        .contains(&RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ACIP_A2A_PATH.to_string()));
    assert!(artifacts
        .reviewer_report_markdown
        .contains("D13 Cognitive-Being Flagship Demo"));
    assert!(artifacts
        .reviewer_report_markdown
        .contains("Structured Planning And Review"));
    assert!(artifacts
        .reviewer_report_markdown
        .contains("Secure Local Comms"));
    assert!(artifacts
        .proof_packet
        .non_claims
        .iter()
        .any(|claim| claim.contains("cross-polis")));

    let summary = artifacts.execution_summary().expect("execution summary");
    assert!(summary.contains("D13 cognitive-being flagship proof"));
    assert!(summary.contains("moral_trace_and_trajectory"));
    assert!(summary.contains("secure_local_comms"));
}

#[test]
fn runtime_v2_cognitive_being_flagship_demo_writes_bundle_without_path_leakage() {
    let root = unique_temp_path("cognitive-being-flagship");
    let artifacts = flagship_artifacts();

    artifacts
        .write_to_root(&root)
        .expect("write cognitive-being flagship bundle");

    for rel_path in [
        RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_PROOF_PATH,
        RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_SECTION_PATH,
        RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_REPORT_PATH,
        RUNTIME_V2_COGNITIVE_BEING_SUPPORT_MORAL_TRACE_PATH,
        RUNTIME_V2_COGNITIVE_BEING_SUPPORT_OUTCOME_LINKAGE_PATH,
        RUNTIME_V2_COGNITIVE_BEING_SUPPORT_TRAJECTORY_PATH,
        RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ANTI_HARM_PATH,
        RUNTIME_V2_COGNITIVE_BEING_SUPPORT_WELLBEING_PATH,
        RUNTIME_V2_COGNITIVE_BEING_SUPPORT_KINDNESS_PATH,
        RUNTIME_V2_COGNITIVE_BEING_SUPPORT_HUMOR_PATH,
        RUNTIME_V2_COGNITIVE_BEING_SUPPORT_AFFECT_PATH,
        RUNTIME_V2_COGNITIVE_BEING_SUPPORT_MORAL_RESOURCES_PATH,
        RUNTIME_V2_COGNITIVE_BEING_SUPPORT_CULTIVATION_PATH,
        RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ACIP_PROOF_PATH,
        RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ACIP_INVOCATION_PATH,
        RUNTIME_V2_COGNITIVE_BEING_SUPPORT_ACIP_A2A_PATH,
        RUNTIME_V2_COGNITIVE_BEING_SUPPORT_SPP_PATH,
        RUNTIME_V2_COGNITIVE_BEING_SUPPORT_SRP_PATH,
    ] {
        assert!(root.join(rel_path).is_file(), "missing {rel_path}");
    }

    let proof_text = fs::read_to_string(root.join(RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_PROOF_PATH))
        .expect("proof text");
    assert!(proof_text.contains("\"demo_id\": \"D13\""));
    assert!(!proof_text.contains(root.to_string_lossy().as_ref()));

    let report_text =
        fs::read_to_string(root.join(RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_REPORT_PATH))
            .expect("report text");
    for forbidden in ["/Users/", "/home/", "/tmp/", "C:\\\\", "sk-live-"] {
        assert!(
            !report_text.contains(forbidden),
            "reviewer report must not contain {forbidden}"
        );
    }

    fs::remove_dir_all(root).ok();
}

#[test]
fn runtime_v2_cognitive_being_flagship_demo_rejects_shape_drift() {
    let packet = flagship_artifacts().proof_packet.clone();

    let mut bad_demo = packet.clone();
    bad_demo.demo_id = "D99".to_string();
    assert!(bad_demo
        .validate_against(&flagship_artifacts().sections)
        .expect_err("bad demo id should fail")
        .to_string()
        .contains("demo matrix row D13"));

    let mut bad_command = packet.clone();
    bad_command.reviewer_command = "adl runtime-v2 observatory-flagship-demo --out x".to_string();
    assert!(bad_command
        .validate_against(&flagship_artifacts().sections)
        .expect_err("wrong reviewer command should fail")
        .to_string()
        .contains("cognitive-being-flagship-demo"));

    let mut bad_non_claims = packet.clone();
    bad_non_claims.non_claims = vec![
        "does not claim legal personhood".to_string(),
        "does not expose private wellbeing".to_string(),
    ];
    assert!(bad_non_claims
        .validate_against(&flagship_artifacts().sections)
        .expect_err("missing non-claims should fail")
        .to_string()
        .contains("birthday"));

    let mut bad_sections = flagship_artifacts().sections.clone();
    bad_sections.pop();
    assert!(
        RuntimeV2CognitiveBeingFlagshipProofPacket::from_sections(&bad_sections)
            .expect_err("missing section should fail")
            .to_string()
            .contains("canonical D13 section roster")
    );

    let mut bad_section = flagship_artifacts().sections[4].clone();
    bad_section.primary_artifact_refs.clear();
    assert!(bad_section
        .validate()
        .expect_err("section without artifacts should fail")
        .to_string()
        .contains("primary_artifact_refs"));

    let mut bad_report = flagship_artifacts().clone();
    bad_report.reviewer_report_markdown = bad_report
        .reviewer_report_markdown
        .replace("## Replay", "## Replays");
    assert!(bad_report
        .validate()
        .expect_err("report structure drift should fail")
        .to_string()
        .contains("## Replay"));
}
