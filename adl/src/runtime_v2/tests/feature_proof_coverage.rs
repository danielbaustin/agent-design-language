use super::*;

#[test]
fn runtime_v2_feature_proof_coverage_contract_is_stable() {
    let packet =
        runtime_v2_feature_proof_coverage_contract().expect("feature proof coverage packet");
    packet.validate().expect("valid feature proof coverage");

    assert_eq!(
        packet.schema_version,
        RUNTIME_V2_FEATURE_PROOF_COVERAGE_SCHEMA
    );
    assert_eq!(packet.demo_id, "D11");
    assert_eq!(packet.entries.len(), 11);
    assert_eq!(packet.entries[0].feature_id, "D1");
    assert_eq!(packet.entries[10].feature_id, "D11");
    assert!(packet.entries.iter().any(|entry| entry.feature_id == "D10"
        && entry.coverage_kind == "runnable_demo_command"
        && entry
            .validation_refs
            .iter()
            .any(|value| value.contains("integrated-csm-run-demo"))));
    assert!(packet
        .validation_commands
        .iter()
        .any(|command| command.contains("feature-proof-coverage")));
}

#[test]
fn runtime_v2_feature_proof_coverage_contract_matches_golden_fixture() {
    let packet =
        runtime_v2_feature_proof_coverage_contract().expect("feature proof coverage packet");
    let json = String::from_utf8(packet.pretty_json_bytes().expect("coverage json"))
        .expect("utf8 coverage json");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/feature_proof_coverage/feature_proof_coverage.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_feature_proof_coverage_validation_rejects_gaps() {
    let mut packet =
        runtime_v2_feature_proof_coverage_contract().expect("feature proof coverage packet");
    packet.entries.pop();
    assert!(packet
        .validate()
        .expect_err("missing feature should fail")
        .to_string()
        .contains("D1 through D11"));

    let mut packet =
        runtime_v2_feature_proof_coverage_contract().expect("feature proof coverage packet");
    packet.entries[9].coverage_kind = "unreviewed_claim".to_string();
    assert!(packet
        .validate()
        .expect_err("unsupported coverage kind should fail")
        .to_string()
        .contains("unsupported feature proof coverage kind"));

    let mut packet =
        runtime_v2_feature_proof_coverage_contract().expect("feature proof coverage packet");
    packet.entries[0].primary_evidence_refs = vec!["/tmp/leak.json".to_string()];
    assert!(packet
        .validate()
        .expect_err("absolute evidence path should fail")
        .to_string()
        .contains("repository-relative path"));
}
