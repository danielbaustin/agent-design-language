use super::*;

#[test]
 fn runtime_v2_private_state_sanctuary_helpers_contract_is_stable() {
    let artifacts = runtime_v2_private_state_sanctuary_contract().expect("sanctuary artifacts");
    artifacts.validate().expect("valid sanctuary artifacts");

    assert_eq!(
        artifacts.state_policy.schema_version,
        RUNTIME_V2_PRIVATE_STATE_SANCTUARY_STATE_POLICY_SCHEMA
    );
    assert_eq!(
        artifacts.ambiguous_wake.schema_version,
        RUNTIME_V2_PRIVATE_STATE_AMBIGUOUS_WAKE_FIXTURE_SCHEMA
    );
    assert_eq!(
        artifacts.quarantine_artifact.schema_version,
        RUNTIME_V2_PRIVATE_STATE_SANCTUARY_QUARANTINE_ARTIFACT_SCHEMA
    );
    assert_eq!(
        artifacts.operator_report.schema_version,
        RUNTIME_V2_PRIVATE_STATE_SANCTUARY_OPERATOR_REPORT_SCHEMA
    );
    assert_eq!(artifacts.ambiguous_wake.demo_id, "D8");
    assert_eq!(
        artifacts.quarantine_artifact.safety_state,
        "sanctuary_or_quarantine_pending_review"
    );
}

#[test]
fn runtime_v2_private_state_sanctuary_state_policy_matches_golden_fixture() {
    let artifacts = runtime_v2_private_state_sanctuary_contract().expect("sanctuary artifacts");
    let json = String::from_utf8(
        artifacts
            .state_policy
            .pretty_json_bytes()
            .expect("state policy json"),
    )
    .expect("utf8 state policy");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/private_state/sanctuary_quarantine_state_policy.json"
        )
        .trim_end()
    );
}

#[test]
 fn runtime_v2_private_state_sanctuary_helpers_ambiguous_wake_matches_golden_fixture() {
    let artifacts = runtime_v2_private_state_sanctuary_contract().expect("sanctuary artifacts");
    let json = String::from_utf8(
        artifacts
            .ambiguous_wake
            .pretty_json_bytes()
            .expect("ambiguous wake json"),
    )
    .expect("utf8 ambiguous wake");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/private_state/ambiguous_wake_fixture.json"
        )
        .trim_end()
    );
}

#[test]
 fn runtime_v2_private_state_sanctuary_reports_quarantine_matches_golden_fixture() {
    let artifacts = runtime_v2_private_state_sanctuary_contract().expect("sanctuary artifacts");
    let json = String::from_utf8(
        artifacts
            .quarantine_artifact
            .pretty_json_bytes()
            .expect("quarantine json"),
    )
    .expect("utf8 quarantine");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/private_state/sanctuary_quarantine_artifact.json"
        )
        .trim_end()
    );
}

#[test]
 fn runtime_v2_private_state_sanctuary_reports_operator_report_matches_golden_fixture() {
    let artifacts = runtime_v2_private_state_sanctuary_contract().expect("sanctuary artifacts");
    let json = String::from_utf8(
        artifacts
            .operator_report
            .pretty_json_bytes()
            .expect("operator report json"),
    )
    .expect("utf8 operator report");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/private_state/sanctuary_quarantine_operator_report.json"
        )
        .trim_end()
    );
}

#[test]
 fn runtime_v2_private_state_sanctuary_reports_negative_cases_match_golden_fixture() {
    let artifacts = runtime_v2_private_state_sanctuary_contract().expect("sanctuary artifacts");
    let json = String::from_utf8(
        artifacts
            .negative_cases
            .pretty_json_bytes()
            .expect("negative cases json"),
    )
    .expect("utf8 negative cases");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/private_state/sanctuary_quarantine_negative_cases.json"
        )
        .trim_end()
    );
}

#[test]
 fn runtime_v2_private_state_sanctuary_helpers_ambiguous_wake_blocks_activation() {
    let artifacts = runtime_v2_private_state_sanctuary_contract().expect("sanctuary artifacts");

    assert!(!artifacts.ambiguous_wake.activation_allowed);
    assert!(!artifacts.quarantine_artifact.activation_allowed);
    assert!(!artifacts.operator_report.safe_to_activate);
    assert!(artifacts
        .quarantine_artifact
        .blocked_actions
        .contains(&"activate_ambiguous_wake".to_string()));

    let mut unsafe_fixture = artifacts.ambiguous_wake.clone();
    unsafe_fixture.activation_allowed = true;
    assert!(unsafe_fixture
        .validate_against(
            &artifacts.anti_equivocation_artifacts.conflict,
            &artifacts.anti_equivocation_artifacts.disposition,
        )
        .expect_err("activation should fail")
        .to_string()
        .contains("block activation"));
}

#[test]
 fn runtime_v2_private_state_sanctuary_reports_quarantine_preserves_evidence_and_is_not_recovery_success() {
    let artifacts = runtime_v2_private_state_sanctuary_contract().expect("sanctuary artifacts");
    let conflict = &artifacts.anti_equivocation_artifacts.conflict;
    let disposition = &artifacts.anti_equivocation_artifacts.disposition;
    let quarantine = &artifacts.quarantine_artifact;

    assert!(!quarantine.recovery_success);
    assert!(quarantine
        .validate_preserves_evidence(conflict, disposition)
        .is_ok());
    for candidate in &conflict.candidates {
        assert!(quarantine
            .preserved_evidence
            .iter()
            .any(|evidence| evidence.artifact_ref == candidate.envelope_ref));
        assert!(quarantine
            .preserved_evidence
            .iter()
            .any(|evidence| evidence.artifact_ref == candidate.sealed_checkpoint_ref));
    }

    let mut recovery_success = quarantine.clone();
    recovery_success.recovery_success = true;
    assert!(recovery_success
        .validate_against(&artifacts.ambiguous_wake, conflict, disposition)
        .expect_err("quarantine cannot be recovery success")
        .to_string()
        .contains("cannot be recovery success"));

    let mut missing_candidate_evidence = quarantine.clone();
    let candidate_ref = &conflict.candidates[0].envelope_ref;
    let candidate_ref_index = missing_candidate_evidence
        .preserved_evidence
        .iter()
        .position(|evidence| evidence.artifact_ref == *candidate_ref)
        .expect("candidate envelope evidence");
    missing_candidate_evidence.preserved_evidence[candidate_ref_index].artifact_ref =
        conflict.ledger_ref.clone();
    missing_candidate_evidence.artifact_hash = missing_candidate_evidence
        .computed_hash()
        .expect("rehash after evidence mutation");
    assert!(missing_candidate_evidence
        .validate_against(&artifacts.ambiguous_wake, conflict, disposition)
        .expect_err("missing evidence should fail")
        .to_string()
        .contains("candidate envelope and checkpoint evidence"));
}

#[test]
 fn runtime_v2_private_state_sanctuary_reports_operator_report_reviews_all_preserved_evidence() {
    let artifacts = runtime_v2_private_state_sanctuary_contract().expect("sanctuary artifacts");
    let mut incomplete_report = artifacts.operator_report.clone();
    let last_index = incomplete_report.reviewed_evidence_refs.len() - 1;
    incomplete_report.reviewed_evidence_refs[last_index] =
        incomplete_report.reviewed_evidence_refs[0].clone();

    assert!(incomplete_report
        .validate_against(&artifacts.quarantine_artifact)
        .expect_err("incomplete report should fail")
        .to_string()
        .contains("must review preserved evidence"));
}

#[test]
 fn runtime_v2_private_state_sanctuary_reports_write_to_root_materializes_fixtures() {
    let artifacts = runtime_v2_private_state_sanctuary_contract().expect("sanctuary artifacts");
    let root = common::unique_temp_path("private-state-sanctuary-write");

    artifacts
        .write_to_root(&root)
        .expect("write sanctuary artifacts");

    for rel_path in [
        RUNTIME_V2_PRIVATE_STATE_SANCTUARY_STATE_POLICY_PATH,
        RUNTIME_V2_PRIVATE_STATE_AMBIGUOUS_WAKE_FIXTURE_PATH,
        RUNTIME_V2_PRIVATE_STATE_SANCTUARY_QUARANTINE_ARTIFACT_PATH,
        RUNTIME_V2_PRIVATE_STATE_SANCTUARY_OPERATOR_REPORT_PATH,
        RUNTIME_V2_PRIVATE_STATE_SANCTUARY_PROOF_PATH,
    ] {
        let text = std::fs::read_to_string(root.join(rel_path)).expect("fixture text");
        assert!(text.contains("\"demo_id\": \"D8\""));
        assert!(text.contains("first true Godel-agent birth"));
        assert!(!text.contains(root.to_string_lossy().as_ref()));
    }

    std::fs::remove_dir_all(root).expect("cleanup sanctuary temp root");
}
