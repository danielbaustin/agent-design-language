use super::*;

#[test]
fn runtime_v2_continuity_challenge_contract_is_stable() {
    let artifacts = runtime_v2_continuity_challenge_contract().expect("challenge artifacts");
    artifacts.validate().expect("valid challenge artifacts");

    assert_eq!(
        artifacts.challenge.schema_version,
        RUNTIME_V2_CONTINUITY_CHALLENGE_SCHEMA
    );
    assert_eq!(
        artifacts.freeze.schema_version,
        RUNTIME_V2_CONTINUITY_CHALLENGE_FREEZE_SCHEMA
    );
    assert_eq!(
        artifacts.appeal_review.schema_version,
        RUNTIME_V2_CONTINUITY_APPEAL_REVIEW_SCHEMA
    );
    assert_eq!(
        artifacts.threat_model.schema_version,
        RUNTIME_V2_CITIZEN_STATE_THREAT_MODEL_SCHEMA
    );
    assert_eq!(
        artifacts.economics_placement.schema_version,
        RUNTIME_V2_ECONOMICS_PLACEMENT_SCHEMA
    );
    assert_eq!(artifacts.challenge.demo_id, "D11");
    assert_eq!(artifacts.threat_model.threats.len(), 7);
}

#[test]
fn runtime_v2_continuity_challenge_serializes_and_matches_golden_fixtures() {
    let artifacts = runtime_v2_continuity_challenge_contract().expect("challenge artifacts");
    let challenge_json = String::from_utf8(
        artifacts
            .challenge
            .pretty_json_bytes()
            .expect("challenge json"),
    )
    .expect("utf8 challenge");
    assert_eq!(
        challenge_json,
        include_str!("../../../tests/fixtures/runtime_v2/challenge/challenge_artifact.json")
            .trim_end()
    );
    let freeze_json = String::from_utf8(artifacts.freeze.pretty_json_bytes().expect("freeze json"))
        .expect("utf8 freeze");
    assert_eq!(
        freeze_json,
        include_str!("../../../tests/fixtures/runtime_v2/challenge/freeze_artifact.json")
            .trim_end()
    );
    let appeal_json = String::from_utf8(
        artifacts
            .appeal_review
            .pretty_json_bytes()
            .expect("appeal json"),
    )
    .expect("utf8 appeal");
    assert_eq!(
        appeal_json,
        include_str!("../../../tests/fixtures/runtime_v2/challenge/appeal_review_artifact.json")
            .trim_end()
    );
    let threat_model_json = String::from_utf8(
        artifacts
            .threat_model
            .pretty_json_bytes()
            .expect("threat model json"),
    )
    .expect("utf8 threat model");
    assert_eq!(
        threat_model_json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/challenge/citizen_state_threat_model.json"
        )
        .trim_end()
    );
    let economics_json = String::from_utf8(
        artifacts
            .economics_placement
            .pretty_json_bytes()
            .expect("economics json"),
    )
    .expect("utf8 economics");
    assert_eq!(
        economics_json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/challenge/economics_placement_record.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_continuity_challenge_rejects_unsafe_freeze_and_appeal_mutations() {
    let artifacts = runtime_v2_continuity_challenge_contract().expect("challenge artifacts");
    let mut destructive = artifacts.freeze.clone();
    destructive.destructive_transition_allowed = true;
    destructive.continuity_sequence_after += 1;
    destructive.freeze_hash = destructive.computed_hash();
    assert!(destructive
        .validate_against(&artifacts.challenge)
        .expect_err("unsafe destructive transition should fail")
        .to_string()
        .contains("challenged destructive transition freezes safely"));

    let mut projection = artifacts.freeze.clone();
    projection.projection_publication_allowed = true;
    projection.freeze_hash = projection.computed_hash();
    assert!(projection
        .validate_against(&artifacts.challenge)
        .expect_err("unsafe projection publication should fail")
        .to_string()
        .contains("challenged projection publication must freeze safely"));

    let mut active_head = artifacts.freeze.clone();
    active_head.active_head_changed = true;
    active_head.freeze_hash = active_head.computed_hash();
    assert!(active_head
        .validate_against(&artifacts.challenge)
        .expect_err("active-head mutation should fail")
        .to_string()
        .contains("freeze must not change the active citizen head"));

    let mut appeal = artifacts.appeal_review.clone();
    appeal.release_allowed = true;
    appeal.destructive_transition_allowed = true;
    assert!(appeal
        .validate_against(&artifacts.challenge, &artifacts.freeze)
        .expect_err("appeal release without proof should fail")
        .to_string()
        .contains("appeal cannot release or permit destructive transition"));

    let mut threat_model = artifacts.threat_model.clone();
    threat_model
        .threats
        .retain(|threat| threat.threat_id != "compromised_key");
    assert!(threat_model
        .validate_against(
            &artifacts.challenge,
            &artifacts.freeze,
            &artifacts.appeal_review,
            &artifacts.access_control_artifacts,
            &artifacts.sanctuary_artifacts,
        )
        .expect_err("missing threat should fail")
        .to_string()
        .contains("threat model must cover insider/operator abuse"));

    let mut economics = artifacts.economics_placement.clone();
    economics.markets_implemented = true;
    assert!(economics
        .validate()
        .expect_err("market implementation should fail")
        .to_string()
        .contains("economics record does not implement markets or payment rails"));
}

#[test]
fn runtime_v2_continuity_challenge_write_to_root_materializes_fixtures() {
    let artifacts = runtime_v2_continuity_challenge_contract().expect("challenge artifacts");
    let root = common::unique_temp_path("continuity-challenge-write");

    artifacts
        .write_to_root(&root)
        .expect("write challenge artifacts");

    for rel_path in [
        RUNTIME_V2_CONTINUITY_CHALLENGE_PATH,
        RUNTIME_V2_CONTINUITY_CHALLENGE_FREEZE_PATH,
        RUNTIME_V2_CONTINUITY_APPEAL_REVIEW_PATH,
        RUNTIME_V2_CITIZEN_STATE_THREAT_MODEL_PATH,
        RUNTIME_V2_ECONOMICS_PLACEMENT_PATH,
    ] {
        let text = std::fs::read_to_string(root.join(rel_path)).expect("artifact text");
        assert!(text.contains("D11"));
        assert!(text.contains("WP-13") || text.contains("challenge"));
        assert!(!text.contains(root.to_string_lossy().as_ref()));
    }

    std::fs::remove_dir_all(root).expect("cleanup challenge temp root");
}
