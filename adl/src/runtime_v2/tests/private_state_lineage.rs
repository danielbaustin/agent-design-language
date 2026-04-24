use super::*;

#[test]
fn runtime_v2_private_state_lineage_contract_is_stable() {
    let artifacts = runtime_v2_private_state_lineage_contract().expect("lineage artifacts");
    artifacts.validate().expect("valid lineage artifacts");

    assert_eq!(
        artifacts.ledger.schema_version,
        RUNTIME_V2_PRIVATE_STATE_LINEAGE_LEDGER_SCHEMA
    );
    assert_eq!(
        artifacts.materialized_head.schema_version,
        RUNTIME_V2_PRIVATE_STATE_MATERIALIZED_HEAD_SCHEMA
    );
    assert_eq!(artifacts.negative_cases.demo_id, "D5");
    assert_eq!(artifacts.ledger.entries.len(), 1);
    assert_eq!(
        artifacts.ledger.accepted_head_entry_hash,
        artifacts.ledger.entries[0].entry_hash
    );
}

#[test]
fn runtime_v2_private_state_lineage_ledger_matches_golden_fixture() {
    let artifacts = runtime_v2_private_state_lineage_contract().expect("lineage artifacts");
    let json = String::from_utf8(
        artifacts
            .ledger
            .pretty_json_bytes()
            .expect("lineage ledger json"),
    )
    .expect("utf8 lineage ledger json");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/private_state/lineage_ledger.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_materialized_head_matches_golden_fixture() {
    let artifacts = runtime_v2_private_state_lineage_contract().expect("lineage artifacts");
    let json = String::from_utf8(
        artifacts
            .materialized_head
            .pretty_json_bytes()
            .expect("materialized head json"),
    )
    .expect("utf8 materialized head json");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/private_state/materialized_head.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_lineage_negative_cases_match_golden_fixture() {
    let artifacts = runtime_v2_private_state_lineage_contract().expect("lineage artifacts");
    let json = String::from_utf8(
        artifacts
            .negative_cases
            .pretty_json_bytes()
            .expect("lineage proof json"),
    )
    .expect("utf8 lineage proof json");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/private_state/lineage_negative_cases.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_lineage_calculates_accepted_head() {
    let artifacts = runtime_v2_private_state_lineage_contract().expect("lineage artifacts");
    let head = artifacts.ledger.accepted_head().expect("accepted head");

    assert_eq!(
        head.state_sequence,
        artifacts.materialized_head.state_sequence
    );
    assert_eq!(head.entry_hash, artifacts.materialized_head.head_entry_hash);
    assert_eq!(
        head.canonical_state_hash,
        artifacts
            .sealing_artifacts
            .envelope_artifacts
            .private_state
            .canonical_state
            .content_hash()
            .expect("state hash")
    );
}

#[test]
fn runtime_v2_private_state_lineage_accepts_matching_head_and_successor_candidate() {
    let artifacts = runtime_v2_private_state_lineage_contract().expect("lineage artifacts");
    let head = artifacts.ledger.accepted_head().expect("accepted head");

    let disposition = artifacts
        .materialized_head
        .disposition_against_ledger(&artifacts.ledger, &artifacts.sealing_artifacts)
        .expect("accepted head disposition");
    assert_eq!(disposition.disposition, "accepted");
    assert!(disposition
        .reason
        .contains("matches append-only ledger accepted head"));

    let successor = RuntimeV2PrivateStateLineageEntry::new_accepted(
        "lineage-entry-proto-citizen-alpha-0002".to_string(),
        head.entry_hash.clone(),
        "snapshot".to_string(),
        artifacts.ledger.citizen_id.clone(),
        artifacts.ledger.manifold_id.clone(),
        artifacts.ledger.lineage_id.clone(),
        head.state_sequence + 1,
        head.canonical_state_hash.clone(),
        head.envelope_ref.clone(),
        head.envelope_hash.clone(),
        head.sealed_checkpoint_ref.clone(),
        head.sealed_checkpoint_hash.clone(),
        "sha256:4444444444444444444444444444444444444444444444444444444444444444".to_string(),
        head.writer_identity.clone(),
        Some("runtime_v2/private_state/witnesses/lineage-entry-0002.json".to_string()),
        Some("runtime_v2/private_state/receipts/lineage-entry-0002.json".to_string()),
        head.recorded_at_logical_tick + 1,
    )
    .expect("valid successor");

    let candidates = artifacts
        .ledger
        .fork_candidates(successor)
        .expect("non-fork successor accepted as candidate");
    assert_eq!(candidates.len(), artifacts.ledger.entries.len() + 1);
}

#[test]
fn runtime_v2_private_state_lineage_rejects_tamper_truncation_fork_and_replay() {
    let artifacts = runtime_v2_private_state_lineage_contract().expect("lineage artifacts");

    let mut tampered = artifacts.ledger.clone();
    tampered.entries[0].canonical_state_hash =
        "sha256:1111111111111111111111111111111111111111111111111111111111111111".to_string();
    assert!(tampered
        .validate_shape()
        .expect_err("tampered entry should fail")
        .to_string()
        .contains("entry hash mismatch"));

    let mut truncated = artifacts.ledger.clone();
    truncated.entries.clear();
    assert!(truncated
        .validate_shape()
        .expect_err("truncated ledger should fail")
        .to_string()
        .contains("at least one accepted entry"));

    let mut fork = artifacts.ledger.entries[0].clone();
    fork.entry_id = "lineage-entry-proto-citizen-alpha-0001-fork".to_string();
    fork.canonical_state_hash =
        "sha256:2222222222222222222222222222222222222222222222222222222222222222".to_string();
    fork.entry_hash = fork.computed_hash().expect("fork hash");
    assert!(artifacts
        .ledger
        .fork_candidates(fork)
        .expect_err("forked successor should fail")
        .to_string()
        .contains("forked successor"));

    let mut replay = artifacts.ledger.clone();
    replay.entries.push(replay.entries[0].clone());
    assert!(replay
        .validate_shape()
        .expect_err("replay should fail")
        .to_string()
        .contains("previous hash mismatch"));
}

#[test]
fn runtime_v2_private_state_lineage_head_disagreement_enters_recovery_or_quarantine() {
    let artifacts = runtime_v2_private_state_lineage_contract().expect("lineage artifacts");
    let mut wrong_head = artifacts.materialized_head.clone();
    wrong_head.head_entry_hash =
        "sha256:3333333333333333333333333333333333333333333333333333333333333333".to_string();

    assert!(wrong_head
        .validate_against_ledger(&artifacts.ledger, &artifacts.sealing_artifacts)
        .expect_err("head disagreement should fail")
        .to_string()
        .contains("recovery_or_quarantine"));

    let disposition = wrong_head
        .disposition_against_ledger(&artifacts.ledger, &artifacts.sealing_artifacts)
        .expect("head disagreement disposition");
    assert_eq!(disposition.disposition, "recovery_or_quarantine");
    assert!(disposition
        .required_next_step
        .contains("reconstruct_from_ledger_or_quarantine"));
}

#[cfg(feature = "slow-proof-tests")]
#[test]
fn runtime_v2_private_state_lineage_write_to_root_materializes_fixtures() {
    let artifacts = runtime_v2_private_state_lineage_contract().expect("lineage artifacts");
    let root = common::unique_temp_path("private-state-lineage-write");

    artifacts
        .write_to_root(&root)
        .expect("write lineage artifacts");

    let ledger = std::fs::read_to_string(root.join(RUNTIME_V2_PRIVATE_STATE_LINEAGE_LEDGER_PATH))
        .expect("ledger");
    let head = std::fs::read_to_string(root.join(RUNTIME_V2_PRIVATE_STATE_MATERIALIZED_HEAD_PATH))
        .expect("head");
    let proof = std::fs::read_to_string(root.join(RUNTIME_V2_PRIVATE_STATE_LINEAGE_PROOF_PATH))
        .expect("proof");

    assert!(ledger.contains(RUNTIME_V2_PRIVATE_STATE_LINEAGE_LEDGER_SCHEMA));
    assert!(head.contains(RUNTIME_V2_PRIVATE_STATE_MATERIALIZED_HEAD_SCHEMA));
    assert!(proof.contains("tampered_entry"));

    std::fs::remove_dir_all(root).expect("cleanup lineage temp root");
}
