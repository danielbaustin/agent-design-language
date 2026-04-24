use super::*;

#[test]
fn runtime_v2_private_state_envelope_contract_is_stable() {
    let artifacts =
        runtime_v2_private_state_envelope_contract().expect("private-state envelope artifacts");
    artifacts.validate().expect("valid envelope artifacts");

    assert_eq!(
        artifacts.envelope.schema_version,
        RUNTIME_V2_PRIVATE_STATE_ENVELOPE_SCHEMA
    );
    assert_eq!(
        artifacts.trust_root.schema_version,
        RUNTIME_V2_PRIVATE_STATE_TRUST_ROOT_SCHEMA
    );
    assert_eq!(
        artifacts.negative_cases.schema_version,
        RUNTIME_V2_PRIVATE_STATE_ENVELOPE_PROOF_SCHEMA
    );
    assert_eq!(artifacts.envelope.signature_algorithm, "ed25519");
    assert_eq!(artifacts.envelope.signature_key_id, "local-root-key-0001");
}

#[test]
fn runtime_v2_private_state_envelope_matches_golden_fixture() {
    let artifacts =
        runtime_v2_private_state_envelope_contract().expect("private-state envelope artifacts");
    let envelope_json = String::from_utf8(
        artifacts
            .envelope
            .pretty_json_bytes()
            .expect("envelope json"),
    )
    .expect("utf8 envelope json");

    assert_eq!(
        envelope_json,
        include_str!("../../../tests/fixtures/runtime_v2/private_state/envelope.json").trim_end()
    );
}

#[test]
fn runtime_v2_private_state_trust_root_matches_golden_fixture() {
    let artifacts =
        runtime_v2_private_state_envelope_contract().expect("private-state envelope artifacts");
    let trust_root_json = String::from_utf8(
        artifacts
            .trust_root
            .pretty_json_bytes()
            .expect("trust root json"),
    )
    .expect("utf8 trust root json");

    assert_eq!(
        trust_root_json,
        include_str!("../../../tests/fixtures/runtime_v2/private_state/trust_root.json").trim_end()
    );
}

#[test]
fn runtime_v2_private_state_envelope_negative_cases_match_golden_fixture() {
    let artifacts =
        runtime_v2_private_state_envelope_contract().expect("private-state envelope artifacts");
    let negative_cases_json = String::from_utf8(
        artifacts
            .negative_cases
            .pretty_json_bytes()
            .expect("negative cases json"),
    )
    .expect("utf8 negative cases json");

    assert_eq!(
        negative_cases_json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/private_state/envelope_negative_cases.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_envelope_rejects_required_negative_cases() {
    let artifacts =
        runtime_v2_private_state_envelope_contract().expect("private-state envelope artifacts");
    let state = &artifacts.private_state.canonical_state;
    let trust_root = &artifacts.trust_root;

    let mut missing_signature = artifacts.envelope.clone();
    missing_signature.signature_b64.clear();
    assert!(missing_signature
        .validate_against_state(state, trust_root)
        .expect_err("missing signature should fail")
        .to_string()
        .contains("missing signature"));

    let mut unknown_key = artifacts.envelope.clone();
    unknown_key.signature_key_id = "unknown-root-key".to_string();
    assert!(unknown_key
        .validate_against_state(state, trust_root)
        .expect_err("unknown key should fail")
        .to_string()
        .contains("unknown key id"));

    let mut revoked_key = artifacts.envelope.clone();
    revoked_key.signature_key_id = "revoked-root-key-0001".to_string();
    assert!(revoked_key
        .validate_against_state(state, trust_root)
        .expect_err("revoked key should fail")
        .to_string()
        .contains("revoked key id"));

    let mut content_hash_mismatch = artifacts.envelope.clone();
    content_hash_mismatch.content_hash =
        "sha256:1111111111111111111111111111111111111111111111111111111111111111".to_string();
    assert!(content_hash_mismatch
        .validate_against_state(state, trust_root)
        .expect_err("content hash mismatch should fail")
        .to_string()
        .contains("content hash mismatch"));

    let mut sequence_regression = artifacts.envelope.clone();
    sequence_regression.state_sequence = 0;
    assert!(sequence_regression
        .validate_against_state(state, trust_root)
        .expect_err("sequence regression should fail")
        .to_string()
        .contains("sequence"));

    let mut broken_predecessor = artifacts.envelope.clone();
    broken_predecessor.predecessor_state_hash =
        "sha256:2222222222222222222222222222222222222222222222222222222222222222".to_string();
    assert!(broken_predecessor
        .validate_against_state(state, trust_root)
        .expect_err("broken predecessor should fail")
        .to_string()
        .contains("predecessor hash mismatch"));
}

#[test]
fn runtime_v2_private_state_envelope_rejects_signature_and_trust_policy_drift() {
    let artifacts =
        runtime_v2_private_state_envelope_contract().expect("private-state envelope artifacts");
    let state = &artifacts.private_state.canonical_state;

    let mut signature_mismatch = artifacts.envelope.clone();
    signature_mismatch.writer_identity = "other-writer".to_string();
    assert!(signature_mismatch
        .validate_against_state(state, &artifacts.trust_root)
        .expect_err("writer mismatch should fail")
        .to_string()
        .contains("writer identity"));

    let mut trust_root = artifacts.trust_root.clone();
    trust_root.allowed_algorithms.clear();
    assert!(trust_root
        .validate()
        .expect_err("missing algorithm should fail")
        .to_string()
        .contains("allowed_algorithms"));

    let mut trust_root = artifacts.trust_root.clone();
    trust_root.trusted_keys[0].allowed_artifact_kinds.clear();
    assert!(trust_root
        .validate()
        .expect_err("missing allowed artifact kind should fail")
        .to_string()
        .contains("allowed_artifact_kinds"));

    let mut proof = artifacts.negative_cases.clone();
    proof.required_negative_cases.pop();
    assert!(proof
        .validate()
        .expect_err("missing negative case should fail")
        .to_string()
        .contains("missing negative case"));
}

#[cfg(feature = "slow-proof-tests")]
#[test]
fn runtime_v2_private_state_envelope_write_to_root_materializes_fixtures() {
    let artifacts =
        runtime_v2_private_state_envelope_contract().expect("private-state envelope artifacts");
    let root = common::unique_temp_path("private-state-envelope-write");

    artifacts
        .write_to_root(&root)
        .expect("write envelope artifacts");

    let envelope = std::fs::read_to_string(root.join(RUNTIME_V2_PRIVATE_STATE_ENVELOPE_PATH))
        .expect("envelope");
    let trust_root = std::fs::read_to_string(root.join(RUNTIME_V2_PRIVATE_STATE_TRUST_ROOT_PATH))
        .expect("trust root");
    let proof = std::fs::read_to_string(root.join(RUNTIME_V2_PRIVATE_STATE_ENVELOPE_PROOF_PATH))
        .expect("negative proof");

    assert!(envelope.contains(RUNTIME_V2_PRIVATE_STATE_ENVELOPE_SCHEMA));
    assert!(trust_root.contains("local-root-key-0001"));
    assert!(proof.contains("missing_signature"));

    std::fs::remove_dir_all(root).expect("cleanup envelope temp root");
}
