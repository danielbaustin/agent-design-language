use super::*;

#[test]
fn runtime_v2_private_state_anti_equivocation_contract_is_stable() {
    let artifacts =
        runtime_v2_private_state_anti_equivocation_contract().expect("anti-equivocation artifacts");
    artifacts
        .validate()
        .expect("valid anti-equivocation artifacts");

    assert_eq!(
        artifacts.conflict.schema_version,
        RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_CONFLICT_SCHEMA
    );
    assert_eq!(
        artifacts.disposition.schema_version,
        RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_DISPOSITION_SCHEMA
    );
    assert_eq!(artifacts.negative_cases.demo_id, "D7");
    assert_eq!(artifacts.conflict.candidates.len(), 2);
    assert_eq!(artifacts.disposition.disposition, "sanctuary_or_quarantine");
}

#[test]
fn runtime_v2_private_state_anti_equivocation_conflict_matches_golden_fixture() {
    let artifacts =
        runtime_v2_private_state_anti_equivocation_contract().expect("anti-equivocation artifacts");
    let json = String::from_utf8(
        artifacts
            .conflict
            .pretty_json_bytes()
            .expect("conflict json"),
    )
    .expect("utf8 conflict json");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/private_state/anti_equivocation_conflict.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_anti_equivocation_disposition_matches_golden_fixture() {
    let artifacts =
        runtime_v2_private_state_anti_equivocation_contract().expect("anti-equivocation artifacts");
    let json = String::from_utf8(
        artifacts
            .disposition
            .pretty_json_bytes()
            .expect("disposition json"),
    )
    .expect("utf8 disposition json");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/private_state/anti_equivocation_disposition.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_anti_equivocation_negative_cases_match_golden_fixture() {
    let artifacts =
        runtime_v2_private_state_anti_equivocation_contract().expect("anti-equivocation artifacts");
    let json = String::from_utf8(
        artifacts
            .negative_cases
            .pretty_json_bytes()
            .expect("negative cases json"),
    )
    .expect("utf8 negative cases json");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/private_state/anti_equivocation_negative_cases.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_anti_equivocation_rejects_dual_activation() {
    let artifacts =
        runtime_v2_private_state_anti_equivocation_contract().expect("anti-equivocation artifacts");

    let one_active = vec![artifacts.conflict.candidates[0].candidate_id.clone()];
    artifacts
        .conflict
        .validate_activation_attempt(&one_active)
        .expect("single active candidate is structurally allowed");

    let both_active = artifacts
        .conflict
        .candidates
        .iter()
        .map(|candidate| candidate.candidate_id.clone())
        .collect::<Vec<_>>();
    assert!(artifacts
        .conflict
        .validate_activation_attempt(&both_active)
        .expect_err("dual active successors should fail")
        .to_string()
        .contains("cannot both become active"));
}

#[test]
fn runtime_v2_private_state_anti_equivocation_preserves_evidence_for_review() {
    let artifacts =
        runtime_v2_private_state_anti_equivocation_contract().expect("anti-equivocation artifacts");
    let disposition = &artifacts.disposition;

    assert!(!disposition.activation_allowed);
    assert!(disposition.active_candidate_id.is_none());
    assert!(disposition
        .evidence_refs
        .contains(&artifacts.conflict.ledger_ref));
    assert!(disposition
        .evidence_refs
        .contains(&artifacts.conflict.witness_set_ref));
    assert_eq!(
        disposition.preserved_candidate_claim_hashes.len(),
        artifacts.conflict.candidates.len()
    );
    assert_eq!(
        disposition.destructive_transition_policy,
        "block_activation_and_preserve_evidence_until_review"
    );
}

#[test]
fn runtime_v2_private_state_anti_equivocation_rejects_tamper_and_missing_evidence() {
    let artifacts =
        runtime_v2_private_state_anti_equivocation_contract().expect("anti-equivocation artifacts");

    let mut tampered_candidate = artifacts.conflict.clone();
    tampered_candidate.candidates[0].predecessor_state_hash =
        "sha256:1111111111111111111111111111111111111111111111111111111111111111".to_string();
    tampered_candidate.candidates[0].entry_hash = tampered_candidate.candidates[0]
        .computed_entry_hash()
        .expect("tampered candidate entry hash");
    tampered_candidate.candidates[0].claim_hash = tampered_candidate.candidates[0]
        .computed_claim_hash()
        .expect("tampered candidate claim hash");
    tampered_candidate.conflict_hash = tampered_candidate
        .computed_hash()
        .expect("tampered conflict hash");
    assert!(tampered_candidate
        .validate_against(&artifacts.witness_artifacts)
        .expect_err("candidate not bound to head should fail")
        .to_string()
        .contains("candidate does not target the contested successor position"));

    let mut missing_evidence = artifacts.disposition.clone();
    missing_evidence.preserved_candidate_claim_hashes[1] =
        missing_evidence.preserved_candidate_claim_hashes[0].clone();
    missing_evidence.disposition_hash = missing_evidence
        .computed_hash()
        .expect("missing evidence disposition hash");
    assert!(missing_evidence
        .validate_against(&artifacts.conflict)
        .expect_err("missing evidence should fail")
        .to_string()
        .contains("preserve all candidate evidence"));
}

#[test]
fn runtime_v2_private_state_anti_equivocation_rejects_non_conflicting_claims() {
    let artifacts =
        runtime_v2_private_state_anti_equivocation_contract().expect("anti-equivocation artifacts");
    let mut non_conflict = artifacts.conflict.clone();
    non_conflict.candidates[1] = non_conflict.candidates[0].clone();
    non_conflict.candidates[1].candidate_id = "candidate-alpha-snapshot-duplicate".to_string();
    non_conflict.candidates[1].claim_hash = non_conflict.candidates[0].claim_hash.clone();
    non_conflict.conflict_hash = non_conflict.computed_hash().expect("non-conflict hash");

    assert!(non_conflict
        .detect_conflicting_successors()
        .expect_err("duplicate claims should not count as equivocation")
        .to_string()
        .contains("must contain conflicting signed successors"));
}

#[test]
fn runtime_v2_private_state_anti_equivocation_write_to_root_materializes_fixtures() {
    let artifacts =
        runtime_v2_private_state_anti_equivocation_contract().expect("anti-equivocation artifacts");
    let root = common::unique_temp_path("private-state-anti-equivocation-write");

    artifacts
        .write_to_root(&root)
        .expect("write anti-equivocation artifacts");

    let conflict = std::fs::read_to_string(
        root.join(RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_CONFLICT_PATH),
    )
    .expect("conflict");
    let disposition = std::fs::read_to_string(
        root.join(RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_DISPOSITION_PATH),
    )
    .expect("disposition");
    let proof =
        std::fs::read_to_string(root.join(RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_PROOF_PATH))
            .expect("proof");

    assert!(conflict.contains(RUNTIME_V2_PRIVATE_STATE_ANTI_EQUIVOCATION_CONFLICT_SCHEMA));
    assert!(disposition.contains("sanctuary_or_quarantine"));
    assert!(proof.contains("dual_active_successors"));

    std::fs::remove_dir_all(root).expect("cleanup anti-equivocation temp root");
}
