use super::*;

#[test]
fn runtime_v2_a2a_adapter_boundary_contract_is_stable() {
    let packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary packet");
    packet
        .validate()
        .expect("valid A2A adapter boundary packet");

    assert_eq!(
        packet.schema_version,
        RUNTIME_V2_A2A_ADAPTER_BOUNDARY_PACKET_SCHEMA
    );
    assert_eq!(packet.milestone, "v0.91.1");
    assert_eq!(packet.wp, "WP-14");
    assert_eq!(packet.adapter_lanes.len(), 3);
    assert_eq!(packet.negative_cases.len(), 4);
    assert!(packet
        .non_claims
        .iter()
        .any(|claim| claim.contains("second canonical internal communication model")));
}

#[test]
fn runtime_v2_a2a_adapter_boundary_matches_golden_fixture() {
    let packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary packet");
    let rendered = serde_json::to_value(&packet).expect("serialize packet");
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/runtime_v2/comms/a2a_adapter_boundary.json"
    ))
    .expect("parse fixture");
    assert_eq!(rendered, fixture);
}

#[test]
fn runtime_v2_a2a_adapter_boundary_validation_rejects_shape_drift() {
    let mut packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary");
    packet.adapter_lanes[0].required_entrypoint = "direct.execute".to_string();
    assert!(packet
        .validate()
        .expect_err("wrong entrypoint should fail")
        .to_string()
        .contains("agent.invoke"));

    let mut packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary");
    packet.negative_cases.pop();
    assert!(packet
        .validate()
        .expect_err("missing negative case should fail")
        .to_string()
        .contains("exactly four reviewed negative cases"));

    let mut packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary");
    packet.adapter_lanes[2].trust_classification = "guest".to_string();
    assert!(packet
        .validate()
        .expect_err("trust order drift should fail")
        .to_string()
        .contains("reviewed trust-class order"));
}

#[test]
fn runtime_v2_a2a_adapter_boundary_validate_against_rejects_dependency_drift() {
    let acip = runtime_v2_acip_hardening_contract().expect("ACIP hardening");
    let fixtures = crate::agent_comms::acip_a2a_fixture_set_v1();

    let mut packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary");
    packet.acip_hardening_ref = "runtime_v2/acip/drifted_packet.json".to_string();
    assert!(packet
        .validate_against(&acip, &fixtures)
        .expect_err("ACIP hardening drift should fail")
        .to_string()
        .contains("ACIP hardening"));

    let mut packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary");
    packet.adapter_lanes[0].refusal_mode = "warn".to_string();
    assert!(packet
        .validate_against(&acip, &fixtures)
        .expect_err("refusal-mode drift should fail")
        .to_string()
        .contains("fail closed"));

    let mut packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary");
    packet.negative_cases[0].expected_error_substring = "drifted".to_string();
    assert!(packet
        .validate_against(&acip, &fixtures)
        .expect_err("negative-case drift should fail")
        .to_string()
        .contains("negative_cases"));
}

#[test]
fn runtime_v2_a2a_adapter_boundary_validation_rejects_packet_metadata_drift() {
    let mut packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary");
    packet.schema_version = "runtime_v2.a2a_adapter_boundary_packet.v0".to_string();
    assert!(packet
        .validate()
        .expect_err("schema drift should fail")
        .to_string()
        .contains("unsupported A2A adapter boundary schema"));

    let mut packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary");
    packet.source_feature_doc =
        "docs/milestones/v0.91/features/A2A_EXTERNAL_AGENT_ADAPTER.md".to_string();
    assert!(packet
        .validate()
        .expect_err("feature-doc drift should fail")
        .to_string()
        .contains("v0.91.1 feature doc"));

    let mut packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary");
    packet.validation_commands.pop();
    assert!(packet
        .validate()
        .expect_err("validation command drift should fail")
        .to_string()
        .contains("validation command set"));

    let mut packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary");
    packet.claim_boundary = "bounded adapter proof".to_string();
    assert!(packet
        .validate()
        .expect_err("claim boundary drift should fail")
        .to_string()
        .contains("external-federation non-claim boundary"));

    let mut packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary");
    packet.non_claims = vec!["does not prove external federation".to_string()];
    assert!(packet
        .validate()
        .expect_err("non-claim drift should fail")
        .to_string()
        .contains("one-communication-model non-claim"));
}

#[test]
fn runtime_v2_a2a_adapter_boundary_validation_rejects_lane_and_negative_case_gaps() {
    let mut packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary");
    packet.adapter_lanes[0].trace_evidence_refs.clear();
    assert!(packet
        .validate()
        .expect_err("missing trace evidence should fail")
        .to_string()
        .contains("trace evidence references"));

    let mut packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary");
    packet.adapter_lanes[1].non_claims = vec!["single".to_string()];
    assert!(packet
        .validate()
        .expect_err("missing lane non-claims should fail")
        .to_string()
        .contains("non-claim coverage"));

    let mut packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary");
    packet.negative_cases[0].expected_error_substring.clear();
    assert!(packet
        .validate()
        .expect_err("missing negative-case error substring should fail")
        .to_string()
        .contains("expected_error_substring"));

    let mut packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary");
    packet.negative_cases[1].proves.clear();
    assert!(packet
        .validate()
        .expect_err("missing negative-case proves text should fail")
        .to_string()
        .contains("negative_cases[].proves"));
}

#[test]
fn runtime_v2_a2a_adapter_boundary_proof_route_paths_exist() {
    let packet = runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary packet");
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");

    let proof_paths = vec![
        packet.source_feature_doc.clone(),
        "adl/src/runtime_v2/a2a_adapter_boundary.rs".to_string(),
        "adl/src/runtime_v2/tests/a2a_adapter_boundary.rs".to_string(),
        "adl/src/agent_comms/a2a.inc".to_string(),
        "adl/tests/fixtures/runtime_v2/comms/a2a_adapter_boundary.json".to_string(),
    ];
    for proof_path in proof_paths {
        assert!(
            repo_root.join(&proof_path).exists(),
            "expected proof-route path to exist: {proof_path}"
        );
    }

    let temp_root = common::unique_temp_path("a2a-adapter-boundary-write");
    packet
        .write_to_root(&temp_root)
        .expect("write A2A adapter boundary packet");
    let rendered =
        std::fs::read_to_string(temp_root.join(RUNTIME_V2_A2A_ADAPTER_BOUNDARY_PACKET_PATH))
            .expect("read written packet");
    assert!(rendered.contains("\"wp\": \"WP-14\""));
}
