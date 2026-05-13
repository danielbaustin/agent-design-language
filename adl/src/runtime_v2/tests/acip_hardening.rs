use super::*;

#[test]
fn runtime_v2_acip_hardening_contract_is_stable() {
    let packet = runtime_v2_acip_hardening_contract().expect("ACIP hardening packet");
    packet.validate().expect("valid ACIP hardening packet");

    assert_eq!(
        packet.schema_version,
        RUNTIME_V2_ACIP_HARDENING_PACKET_SCHEMA
    );
    assert_eq!(packet.milestone, "v0.91.1");
    assert_eq!(packet.wp, "WP-13");
    assert_eq!(packet.negative_cases.len(), 4);
    assert_eq!(packet.state_cases.len(), 12);
    assert_eq!(
        packet.envelope_profile.channel_scope,
        "local_intra_polis_only"
    );
    assert!(!packet.state_cases[0].blocked_message_kinds.is_empty());
    assert!(packet
        .non_claims
        .iter()
        .any(|claim| claim.contains("external transport")));
}

#[test]
fn runtime_v2_acip_hardening_validation_rejects_shape_drift() {
    let mut packet = runtime_v2_acip_hardening_contract().expect("ACIP hardening packet");
    packet.envelope_profile.external_transport_policy = "allowed_without_tls".to_string();
    assert!(packet
        .validate()
        .expect_err("weak external transport policy should fail")
        .to_string()
        .contains("TLS or mutual-TLS-equivalent protection"));

    let mut packet = runtime_v2_acip_hardening_contract().expect("ACIP hardening packet");
    packet.negative_cases.pop();
    assert!(packet
        .validate()
        .expect_err("missing negative case should fail")
        .to_string()
        .contains("exactly four required negative cases"));

    let mut packet = runtime_v2_acip_hardening_contract().expect("ACIP hardening packet");
    packet.state_cases[0].state = "WRONG".to_string();
    assert!(packet
        .validate()
        .expect_err("state order drift should fail")
        .to_string()
        .contains("reviewed lifecycle order"));
}

#[test]
fn runtime_v2_acip_hardening_validate_against_rejects_dependency_drift() {
    let lifecycle = runtime_v2_agent_lifecycle_state_contract().expect("agent lifecycle");
    let citizen_state =
        runtime_v2_citizen_state_substrate_contract().expect("citizen state substrate");
    let access = runtime_v2_access_control_contract().expect("access control");
    let observatory =
        runtime_v2_private_state_observatory_contract().expect("private state observatory");
    let report = crate::agent_comms::acip_conformance_report_v1();

    let mut packet = runtime_v2_acip_hardening_contract().expect("ACIP hardening packet");
    packet.lifecycle_state_contract_ref = "runtime_v2/agent_lifecycle/drifted.json".to_string();
    assert!(packet
        .validate_against(&lifecycle, &citizen_state, &access, &observatory, &report)
        .expect_err("lifecycle ref drift should fail")
        .to_string()
        .contains("lifecycle_state_contract_ref"));

    let mut packet = runtime_v2_acip_hardening_contract().expect("ACIP hardening packet");
    packet.envelope_profile.authority_gate_refs.pop();
    assert!(packet
        .validate_against(&lifecycle, &citizen_state, &access, &observatory, &report)
        .expect_err("authority gate drift should fail")
        .to_string()
        .contains("envelope_profile"));

    let mut packet = runtime_v2_acip_hardening_contract().expect("ACIP hardening packet");
    packet.negative_cases[2].expected_error_substring = "drifted".to_string();
    assert!(packet
        .validate_against(&lifecycle, &citizen_state, &access, &observatory, &report)
        .expect_err("negative case drift should fail")
        .to_string()
        .contains("negative_cases"));
}

#[test]
fn runtime_v2_acip_hardening_proof_route_paths_exist() {
    let packet = runtime_v2_acip_hardening_contract().expect("ACIP hardening packet");
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");
    let lifecycle = runtime_v2_agent_lifecycle_state_contract().expect("agent lifecycle");
    let citizen_state =
        runtime_v2_citizen_state_substrate_contract().expect("citizen state substrate");

    assert_eq!(
        packet.lifecycle_state_contract_ref,
        RUNTIME_V2_AGENT_LIFECYCLE_STATE_CONTRACT_PATH
    );
    assert_eq!(
        packet.lifecycle_transition_matrix_ref,
        RUNTIME_V2_AGENT_LIFECYCLE_TRANSITION_MATRIX_PATH
    );
    assert_eq!(packet.citizen_state_ref, citizen_state.artifact_path);

    let proof_paths = vec![
        packet.source_feature_doc.clone(),
        "adl/src/runtime_v2/acip_hardening.rs".to_string(),
        "adl/src/runtime_v2/tests/acip_hardening.rs".to_string(),
        "adl/src/agent_comms/orchestrate/conformance.inc".to_string(),
    ];
    for proof_path in proof_paths {
        assert!(
            repo_root.join(&proof_path).exists(),
            "expected proof-route path to exist: {proof_path}"
        );
    }

    let temp_root = common::unique_temp_path("acip-hardening-write");
    packet
        .write_to_root(&temp_root)
        .expect("write ACIP hardening packet");
    let rendered = std::fs::read_to_string(temp_root.join(RUNTIME_V2_ACIP_HARDENING_PACKET_PATH))
        .expect("read written packet");
    assert!(rendered.contains("\"wp\": \"WP-13\""));
    assert_eq!(
        lifecycle.state_contract.states.len(),
        packet.state_cases.len()
    );
}
