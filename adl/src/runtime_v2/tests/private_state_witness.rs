use super::*;

#[test]
fn runtime_v2_private_state_witness_contract_is_stable() {
    let artifacts = runtime_v2_private_state_witness_contract().expect("witness artifacts");
    artifacts.validate().expect("valid witness artifacts");

    assert_eq!(
        artifacts.witness_set.schema_version,
        RUNTIME_V2_PRIVATE_STATE_CONTINUITY_WITNESS_SET_SCHEMA
    );
    assert_eq!(
        artifacts.receipt_set.schema_version,
        RUNTIME_V2_PRIVATE_STATE_CITIZEN_RECEIPT_SET_SCHEMA
    );
    assert_eq!(artifacts.negative_cases.demo_id, "D6");
    assert_eq!(artifacts.witness_set.witnesses.len(), 5);
    assert_eq!(artifacts.receipt_set.receipts.len(), 5);
}

#[test]
fn runtime_v2_private_state_continuity_witnesses_match_golden_fixture() {
    let artifacts = runtime_v2_private_state_witness_contract().expect("witness artifacts");
    let json = String::from_utf8(
        artifacts
            .witness_set
            .pretty_json_bytes()
            .expect("witness set json"),
    )
    .expect("utf8 witness set json");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/private_state/continuity_witnesses.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_citizen_receipts_match_golden_fixture() {
    let artifacts = runtime_v2_private_state_witness_contract().expect("witness artifacts");
    let json = String::from_utf8(
        artifacts
            .receipt_set
            .pretty_json_bytes()
            .expect("receipt set json"),
    )
    .expect("utf8 receipt set json");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/private_state/citizen_receipts.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_witness_negative_cases_match_golden_fixture() {
    let artifacts = runtime_v2_private_state_witness_contract().expect("witness artifacts");
    let json = String::from_utf8(
        artifacts
            .negative_cases
            .pretty_json_bytes()
            .expect("negative case json"),
    )
    .expect("utf8 negative case json");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/private_state/witness_receipt_negative_cases.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_witnesses_cover_required_transitions() {
    let artifacts = runtime_v2_private_state_witness_contract().expect("witness artifacts");
    let transitions = artifacts
        .witness_set
        .witnesses
        .iter()
        .map(|witness| witness.transition_type.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        transitions,
        vec![
            "admission",
            "snapshot",
            "wake",
            "quarantine",
            "release-from-quarantine"
        ]
    );
}

#[test]
fn runtime_v2_private_state_witnesses_bind_ledger_envelope_and_checkpoint_evidence() {
    let artifacts = runtime_v2_private_state_witness_contract().expect("witness artifacts");
    let head = artifacts
        .lineage_artifacts
        .ledger
        .accepted_head()
        .expect("accepted head");

    for witness in &artifacts.witness_set.witnesses {
        assert_eq!(
            witness.ledger_ref,
            artifacts.lineage_artifacts.ledger.artifact_path
        );
        assert_eq!(
            witness.ledger_root_hash,
            artifacts.lineage_artifacts.ledger.ledger_root_hash
        );
        assert_eq!(witness.lineage_entry_hash, head.entry_hash);
        assert_eq!(witness.envelope_hash, head.envelope_hash);
        assert_eq!(witness.sealed_checkpoint_hash, head.sealed_checkpoint_hash);
        assert_eq!(witness.canonical_state_hash, head.canonical_state_hash);
    }
}

#[test]
fn runtime_v2_private_state_witnesses_reject_tampered_hash_and_ledger_root() {
    let artifacts = runtime_v2_private_state_witness_contract().expect("witness artifacts");

    let mut tampered_hash = artifacts.witness_set.witnesses[0].clone();
    tampered_hash.witness_hash =
        "sha256:1111111111111111111111111111111111111111111111111111111111111111".to_string();
    assert!(tampered_hash
        .validate_shape()
        .expect_err("tampered witness hash should fail")
        .to_string()
        .contains("witness hash mismatch"));

    let mut mismatched_ledger = artifacts.witness_set.witnesses[0].clone();
    mismatched_ledger.ledger_root_hash =
        "sha256:2222222222222222222222222222222222222222222222222222222222222222".to_string();
    mismatched_ledger.witness_hash = mismatched_ledger
        .computed_hash()
        .expect("mismatched ledger witness hash");
    assert!(mismatched_ledger
        .validate_against(&artifacts.lineage_artifacts)
        .expect_err("mismatched ledger root should fail")
        .to_string()
        .contains("must bind to ledger and envelope evidence"));
}

#[test]
fn runtime_v2_private_state_receipts_explain_continuity_without_private_leakage() {
    let artifacts = runtime_v2_private_state_witness_contract().expect("witness artifacts");
    let sealed_payload = artifacts
        .lineage_artifacts
        .sealing_artifacts
        .sealed_checkpoint
        .sealed_payload_b64
        .clone();

    for receipt in &artifacts.receipt_set.receipts {
        receipt
            .validate_explains_continuity()
            .expect("receipt explains continuity");
        receipt
            .validate_no_private_leakage(&sealed_payload)
            .expect("receipt does not leak private material");
    }
}

#[test]
fn runtime_v2_private_state_receipts_reject_leakage_and_missing_explanation() {
    let artifacts = runtime_v2_private_state_witness_contract().expect("witness artifacts");
    let sealed_payload = artifacts
        .lineage_artifacts
        .sealing_artifacts
        .sealed_checkpoint
        .sealed_payload_b64
        .clone();

    let mut leaking = artifacts.receipt_set.receipts[0].clone();
    leaking
        .citizen_visible_evidence
        .push(format!("sealed_payload_b64={sealed_payload}"));
    leaking.receipt_hash = leaking.computed_hash().expect("leaking receipt hash");
    assert!(leaking
        .validate_no_private_leakage(&sealed_payload)
        .expect_err("sealed payload leakage should fail")
        .to_string()
        .contains("leaked private or sealed material"));

    let mut unexplained = artifacts.receipt_set.receipts[0].clone();
    unexplained.continuity_explanation =
        vec!["The transition was accepted by an opaque operator.".to_string()];
    unexplained.receipt_hash = unexplained
        .computed_hash()
        .expect("unexplained receipt hash");
    assert!(unexplained
        .validate_explains_continuity()
        .expect_err("missing continuity explanation should fail")
        .to_string()
        .contains("must explain valid continuation"));
}

#[cfg(feature = "slow-proof-tests")]
#[test]
fn runtime_v2_private_state_witness_write_to_root_materializes_fixtures() {
    let artifacts = runtime_v2_private_state_witness_contract().expect("witness artifacts");
    let root = common::unique_temp_path("private-state-witness-write");

    artifacts
        .write_to_root(&root)
        .expect("write witness artifacts");

    let witnesses =
        std::fs::read_to_string(root.join(RUNTIME_V2_PRIVATE_STATE_CONTINUITY_WITNESSES_PATH))
            .expect("witnesses");
    let receipts =
        std::fs::read_to_string(root.join(RUNTIME_V2_PRIVATE_STATE_CITIZEN_RECEIPTS_PATH))
            .expect("receipts");
    let proof =
        std::fs::read_to_string(root.join(RUNTIME_V2_PRIVATE_STATE_WITNESS_RECEIPT_PROOF_PATH))
            .expect("proof");

    assert!(witnesses.contains(RUNTIME_V2_PRIVATE_STATE_CONTINUITY_WITNESS_SET_SCHEMA));
    assert!(receipts.contains(RUNTIME_V2_PRIVATE_STATE_CITIZEN_RECEIPT_SET_SCHEMA));
    assert!(proof.contains("receipt_leaks_sealed_payload"));

    std::fs::remove_dir_all(root).expect("cleanup witness temp root");
}
