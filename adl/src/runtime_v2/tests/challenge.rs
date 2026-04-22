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
fn runtime_v2_continuity_challenge_artifact_matches_golden_fixture() {
    let artifacts = runtime_v2_continuity_challenge_contract().expect("challenge artifacts");
    let json = String::from_utf8(
        artifacts
            .challenge
            .pretty_json_bytes()
            .expect("challenge json"),
    )
    .expect("utf8 challenge");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/challenge/challenge_artifact.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_continuity_challenge_freeze_matches_golden_fixture() {
    let artifacts = runtime_v2_continuity_challenge_contract().expect("challenge artifacts");
    let json = String::from_utf8(artifacts.freeze.pretty_json_bytes().expect("freeze json"))
        .expect("utf8 freeze");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/challenge/freeze_artifact.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_continuity_challenge_appeal_matches_golden_fixture() {
    let artifacts = runtime_v2_continuity_challenge_contract().expect("challenge artifacts");
    let json = String::from_utf8(
        artifacts
            .appeal_review
            .pretty_json_bytes()
            .expect("appeal json"),
    )
    .expect("utf8 appeal");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/challenge/appeal_review_artifact.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_continuity_challenge_threat_model_matches_golden_fixture() {
    let artifacts = runtime_v2_continuity_challenge_contract().expect("challenge artifacts");
    let json = String::from_utf8(
        artifacts
            .threat_model
            .pretty_json_bytes()
            .expect("threat model json"),
    )
    .expect("utf8 threat model");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/challenge/citizen_state_threat_model.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_continuity_challenge_economics_matches_golden_fixture() {
    let artifacts = runtime_v2_continuity_challenge_contract().expect("challenge artifacts");
    let json = String::from_utf8(
        artifacts
            .economics_placement
            .pretty_json_bytes()
            .expect("economics json"),
    )
    .expect("utf8 economics");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/challenge/economics_placement_record.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_continuity_challenge_freezes_destructive_transition() {
    let artifacts = runtime_v2_continuity_challenge_contract().expect("challenge artifacts");
    let mut freeze = artifacts.freeze.clone();
    freeze.destructive_transition_allowed = true;
    freeze.continuity_sequence_after += 1;
    freeze.freeze_hash = freeze.computed_hash();

    assert!(freeze
        .validate_against(&artifacts.challenge)
        .expect_err("unsafe destructive transition should fail")
        .to_string()
        .contains("challenged destructive transition freezes safely"));
}

#[test]
fn runtime_v2_continuity_challenge_freezes_projection_publication() {
    let artifacts = runtime_v2_continuity_challenge_contract().expect("challenge artifacts");
    let mut freeze = artifacts.freeze.clone();
    freeze.projection_publication_allowed = true;
    freeze.freeze_hash = freeze.computed_hash();

    assert!(freeze
        .validate_against(&artifacts.challenge)
        .expect_err("unsafe projection publication should fail")
        .to_string()
        .contains("challenged projection publication must freeze safely"));
}

#[test]
fn runtime_v2_continuity_challenge_cannot_change_active_head() {
    let artifacts = runtime_v2_continuity_challenge_contract().expect("challenge artifacts");
    let mut freeze = artifacts.freeze.clone();
    freeze.active_head_changed = true;
    freeze.freeze_hash = freeze.computed_hash();

    assert!(freeze
        .validate_against(&artifacts.challenge)
        .expect_err("active-head mutation should fail")
        .to_string()
        .contains("freeze must not change the active citizen head"));
}

#[test]
fn runtime_v2_continuity_challenge_appeal_cannot_release_without_resolution() {
    let artifacts = runtime_v2_continuity_challenge_contract().expect("challenge artifacts");
    let mut appeal = artifacts.appeal_review.clone();
    appeal.release_allowed = true;
    appeal.destructive_transition_allowed = true;

    assert!(appeal
        .validate_against(&artifacts.challenge, &artifacts.freeze)
        .expect_err("appeal release without proof should fail")
        .to_string()
        .contains("appeal cannot release or permit destructive transition"));
}

#[test]
fn runtime_v2_continuity_challenge_threat_model_covers_required_abuse_paths() {
    let artifacts = runtime_v2_continuity_challenge_contract().expect("challenge artifacts");
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
}

#[test]
fn runtime_v2_continuity_challenge_economics_record_does_not_implement_markets() {
    let artifacts = runtime_v2_continuity_challenge_contract().expect("challenge artifacts");
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
