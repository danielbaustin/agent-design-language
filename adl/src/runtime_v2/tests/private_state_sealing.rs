use super::*;
use base64::Engine;
use sha2::Digest;

const B64: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;

#[test]
fn runtime_v2_private_state_sealing_contract_is_stable() {
    let artifacts = runtime_v2_private_state_sealing_contract().expect("sealing artifacts");
    artifacts.validate().expect("valid sealing artifacts");

    assert_eq!(
        artifacts.key_policy.schema_version,
        RUNTIME_V2_PRIVATE_STATE_KEY_POLICY_SCHEMA
    );
    assert_eq!(
        artifacts.backend_seam.schema_version,
        RUNTIME_V2_PRIVATE_STATE_BACKEND_SEAM_SCHEMA
    );
    assert_eq!(
        artifacts.sealed_checkpoint.schema_version,
        RUNTIME_V2_PRIVATE_STATE_SEALED_CHECKPOINT_SCHEMA
    );
    assert_eq!(artifacts.negative_cases.demo_id, "D4");
    assert_eq!(
        artifacts.sealed_checkpoint.sealing_algorithm,
        "deterministic_sha256_stream_fixture"
    );
}

#[test]
fn runtime_v2_private_state_key_policy_matches_golden_fixture() {
    let artifacts = runtime_v2_private_state_sealing_contract().expect("sealing artifacts");
    let json = String::from_utf8(
        artifacts
            .key_policy
            .pretty_json_bytes()
            .expect("key policy json"),
    )
    .expect("utf8 key policy json");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/private_state/key_policy.json").trim_end()
    );
}

#[test]
fn runtime_v2_private_state_backend_seam_matches_golden_fixture() {
    let artifacts = runtime_v2_private_state_sealing_contract().expect("sealing artifacts");
    let json = String::from_utf8(
        artifacts
            .backend_seam
            .pretty_json_bytes()
            .expect("backend seam json"),
    )
    .expect("utf8 backend seam json");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/private_state/sealing_backend_seam.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_sealed_checkpoint_matches_golden_fixture() {
    let artifacts = runtime_v2_private_state_sealing_contract().expect("sealing artifacts");
    let json = String::from_utf8(
        artifacts
            .sealed_checkpoint
            .pretty_json_bytes()
            .expect("sealed checkpoint json"),
    )
    .expect("utf8 sealed checkpoint json");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/private_state/proto-citizen-alpha.sealed-checkpoint.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_sealing_negative_cases_match_golden_fixture() {
    let artifacts = runtime_v2_private_state_sealing_contract().expect("sealing artifacts");
    let json = String::from_utf8(
        artifacts
            .negative_cases
            .pretty_json_bytes()
            .expect("negative cases json"),
    )
    .expect("utf8 sealing negative cases json");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/private_state/sealing_negative_cases.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_sealing_opens_with_correct_local_key() {
    let artifacts = runtime_v2_private_state_sealing_contract().expect("sealing artifacts");
    let backend = RuntimeV2DeterministicPrivateStateSealingBackend::fixture_active();
    let opened = artifacts
        .sealed_checkpoint
        .open_with_backend(
            &artifacts.envelope_artifacts,
            &artifacts.key_policy,
            &artifacts.backend_seam,
            &backend,
        )
        .expect("open sealed checkpoint");

    assert_eq!(
        opened,
        artifacts
            .envelope_artifacts
            .private_state
            .canonical_state
            .canonical_bytes()
            .expect("canonical bytes")
    );
}

#[test]
fn runtime_v2_private_state_sealing_rejects_required_negative_cases() {
    let artifacts = runtime_v2_private_state_sealing_contract().expect("sealing artifacts");
    let backend = RuntimeV2DeterministicPrivateStateSealingBackend::fixture_active();

    let mut unavailable_policy = artifacts.key_policy.clone();
    unavailable_policy
        .unavailable_key_ids
        .push("local-seal-key-0001".to_string());
    assert!(artifacts
        .sealed_checkpoint
        .open_with_backend(
            &artifacts.envelope_artifacts,
            &unavailable_policy,
            &artifacts.backend_seam,
            &backend,
        )
        .expect_err("unavailable key should fail")
        .to_string()
        .contains("unavailable sealing key"));

    let wrong_backend =
        RuntimeV2DeterministicPrivateStateSealingBackend::fixture_wrong_material_same_key();
    assert!(artifacts
        .sealed_checkpoint
        .open_with_backend(
            &artifacts.envelope_artifacts,
            &artifacts.key_policy,
            &artifacts.backend_seam,
            &wrong_backend,
        )
        .expect_err("wrong key should fail")
        .to_string()
        .contains("wrong key material"));

    let mut raw_json_payload = artifacts.sealed_checkpoint.clone();
    let raw = br#"{"schema_version":"not_sealed"}"#;
    raw_json_payload.sealed_payload_b64 = B64.encode(raw);
    raw_json_payload.sealed_payload_hash = format!("sha256:{:x}", sha2::Sha256::digest(raw));
    assert!(raw_json_payload
        .validate_shape()
        .expect_err("raw JSON payload should fail")
        .to_string()
        .contains("must not be raw JSON"));
}

#[test]
fn runtime_v2_private_state_sealing_rejects_checkpoint_drift() {
    let artifacts = runtime_v2_private_state_sealing_contract().expect("sealing artifacts");

    let mut envelope_hash_drift = artifacts.sealed_checkpoint.clone();
    envelope_hash_drift.envelope_hash =
        "sha256:1111111111111111111111111111111111111111111111111111111111111111".to_string();
    assert!(envelope_hash_drift
        .validate_against(
            &artifacts.envelope_artifacts,
            &artifacts.key_policy,
            &artifacts.backend_seam,
        )
        .expect_err("envelope hash drift should fail")
        .to_string()
        .contains("envelope hash mismatch"));

    let mut associated_data_drift = artifacts.sealed_checkpoint.clone();
    associated_data_drift.associated_data_hash =
        "sha256:2222222222222222222222222222222222222222222222222222222222222222".to_string();
    assert!(associated_data_drift
        .validate_against(
            &artifacts.envelope_artifacts,
            &artifacts.key_policy,
            &artifacts.backend_seam,
        )
        .expect_err("associated data drift should fail")
        .to_string()
        .contains("associated data hash mismatch"));

    let mut payload_hash_drift = artifacts.sealed_checkpoint.clone();
    payload_hash_drift.sealed_payload_hash =
        "sha256:3333333333333333333333333333333333333333333333333333333333333333".to_string();
    assert!(payload_hash_drift
        .validate_shape()
        .expect_err("payload hash drift should fail")
        .to_string()
        .contains("payload hash mismatch"));
}

#[test]
fn runtime_v2_private_state_sealing_payload_is_not_raw_json_or_canonical_bytes() {
    let artifacts = runtime_v2_private_state_sealing_contract().expect("sealing artifacts");
    let sealed_payload = B64
        .decode(artifacts.sealed_checkpoint.sealed_payload_b64.trim())
        .expect("decode sealed payload");
    let canonical = artifacts
        .envelope_artifacts
        .private_state
        .canonical_state
        .canonical_bytes()
        .expect("canonical bytes");

    assert!(!sealed_payload.starts_with(b"{"));
    assert!(!sealed_payload.starts_with(b"["));
    assert!(!sealed_payload.starts_with(b"ADLPSv1"));
    assert_ne!(sealed_payload, canonical);
}

#[test]
fn runtime_v2_private_state_sealing_write_to_root_materializes_fixtures() {
    let artifacts = runtime_v2_private_state_sealing_contract().expect("sealing artifacts");
    let root = common::unique_temp_path("private-state-sealing-write");

    artifacts
        .write_to_root(&root)
        .expect("write sealing artifacts");

    let policy = std::fs::read_to_string(root.join(RUNTIME_V2_PRIVATE_STATE_KEY_POLICY_PATH))
        .expect("policy");
    let seam = std::fs::read_to_string(root.join(RUNTIME_V2_PRIVATE_STATE_BACKEND_SEAM_PATH))
        .expect("backend seam");
    let checkpoint =
        std::fs::read_to_string(root.join(RUNTIME_V2_PRIVATE_STATE_SEALED_CHECKPOINT_PATH))
            .expect("sealed checkpoint");
    let proof = std::fs::read_to_string(root.join(RUNTIME_V2_PRIVATE_STATE_SEALING_PROOF_PATH))
        .expect("sealing proof");

    assert!(policy.contains(RUNTIME_V2_PRIVATE_STATE_KEY_POLICY_SCHEMA));
    assert!(seam.contains("secure_enclave"));
    assert!(checkpoint.contains(RUNTIME_V2_PRIVATE_STATE_SEALED_CHECKPOINT_SCHEMA));
    assert!(proof.contains("unavailable_key"));

    std::fs::remove_dir_all(root).expect("cleanup sealing temp root");
}
