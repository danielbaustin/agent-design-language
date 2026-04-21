use super::*;

#[test]
fn runtime_v2_private_state_contract_is_stable() {
    let artifacts = runtime_v2_private_state_contract().expect("private-state artifacts");
    artifacts.validate().expect("valid private-state artifacts");

    assert_eq!(
        artifacts.format_decision.schema_version,
        RUNTIME_V2_PRIVATE_STATE_FORMAT_DECISION_SCHEMA
    );
    assert_eq!(
        artifacts.canonical_state.schema_version,
        RUNTIME_V2_PRIVATE_CITIZEN_STATE_SCHEMA
    );
    assert_eq!(
        artifacts.projection.schema_version,
        RUNTIME_V2_PRIVATE_STATE_PROJECTION_SCHEMA
    );
    assert_eq!(artifacts.canonical_state.citizen_id, "proto-citizen-alpha");
    assert_eq!(
        artifacts.projection.authority_status,
        "projection_not_authority"
    );
    assert!(artifacts
        .format_decision
        .authority_rule
        .contains("canonical binary"));
}

#[test]
fn runtime_v2_private_state_canonical_binary_is_deterministic_and_not_json() {
    let artifacts = runtime_v2_private_state_contract().expect("private-state artifacts");
    let first = artifacts
        .canonical_state
        .canonical_bytes()
        .expect("first canonical bytes");
    let second = artifacts
        .canonical_state
        .canonical_bytes()
        .expect("second canonical bytes");

    assert_eq!(first, second);
    assert!(first.starts_with(b"ADLPSv1\0"));
    assert_ne!(first.first(), Some(&b'{'));
    assert_eq!(
        artifacts
            .canonical_state
            .content_hash()
            .expect("content hash"),
        artifacts.projection.source_state_hash
    );
}

#[test]
fn runtime_v2_private_state_projection_matches_golden_fixture() {
    let artifacts = runtime_v2_private_state_contract().expect("private-state artifacts");
    let projection_json = String::from_utf8(
        artifacts
            .projection
            .pretty_json_bytes()
            .expect("projection json"),
    )
    .expect("utf8 projection json");

    assert_eq!(
        projection_json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/private_state/proto-citizen-alpha.projection.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_format_decision_matches_golden_fixture() {
    let artifacts = runtime_v2_private_state_contract().expect("private-state artifacts");
    let decision_json = String::from_utf8(
        artifacts
            .format_decision
            .pretty_json_bytes()
            .expect("decision json"),
    )
    .expect("utf8 decision json");

    assert_eq!(
        decision_json,
        include_str!("../../../tests/fixtures/runtime_v2/private_state/format_decision.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_private_state_validation_rejects_missing_required_boundaries() {
    let artifacts = runtime_v2_private_state_contract().expect("private-state artifacts");

    let mut missing_identity = artifacts.canonical_state.clone();
    missing_identity.citizen_id.clear();
    assert!(missing_identity
        .validate()
        .expect_err("missing citizen id should fail")
        .to_string()
        .contains("citizen_id"));

    let mut missing_lineage = artifacts.canonical_state.clone();
    missing_lineage.lineage_id.clear();
    assert!(missing_lineage
        .validate()
        .expect_err("missing lineage should fail")
        .to_string()
        .contains("lineage_id"));

    let mut wrong_schema = artifacts.canonical_state.clone();
    wrong_schema.schema_version = "runtime_v2.private_citizen_state.v0".to_string();
    assert!(wrong_schema
        .validate()
        .expect_err("wrong schema should fail")
        .to_string()
        .contains("unsupported private citizen state schema"));

    let mut missing_projection_boundary = artifacts.canonical_state.clone();
    missing_projection_boundary
        .projection_schema_version
        .clear();
    assert!(missing_projection_boundary
        .validate()
        .expect_err("missing projection schema should fail")
        .to_string()
        .contains("projection schema version"));

    let mut projection_as_authority = artifacts.projection.clone();
    projection_as_authority.authority_status = "authoritative_state".to_string();
    assert!(projection_as_authority
        .validate_against_state(&artifacts.canonical_state)
        .expect_err("projection authority should fail")
        .to_string()
        .contains("non-authority"));
}
