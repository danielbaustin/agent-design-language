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
fn runtime_v2_acip_hardening_contract_registry_smoke_covers_accessors() {
    runtime_v2_contract_schema_contract().expect("contract schema");
    runtime_v2_manifold_contract().expect("manifold");
    runtime_v2_kernel_loop_contract().expect("kernel loop");
    runtime_v2_citizen_lifecycle_contract().expect("citizen lifecycle");
    runtime_v2_snapshot_rehydration_contract().expect("snapshot rehydration");
    runtime_v2_invariant_violation_contract().expect("invariant violation");
    runtime_v2_invariant_and_violation_contract().expect("invariant and violation");
    runtime_v2_operator_control_report_contract().expect("operator control report");
    runtime_v2_security_boundary_proof_contract().expect("security boundary proof");
    runtime_v2_csm_run_packet_contract().expect("csm run packet");
    runtime_v2_csm_boot_admission_contract().expect("boot admission");
    runtime_v2_csm_governed_episode_contract().expect("governed episode");
    runtime_v2_csm_freedom_gate_mediation_contract().expect("freedom gate mediation");
    runtime_v2_csm_invalid_action_rejection_contract().expect("invalid action rejection");
    runtime_v2_csm_wake_continuity_contract().expect("wake continuity");
    runtime_v2_csm_observatory_contract().expect("csm observatory");
    runtime_v2_csm_recovery_eligibility_contract().expect("recovery eligibility");
    runtime_v2_csm_quarantine_contract().expect("quarantine");
    runtime_v2_csm_hardening_contract().expect("hardening");
    runtime_v2_csm_integrated_run_contract().expect("integrated run");
    runtime_v2_feature_proof_coverage_contract().expect("feature proof coverage");
    runtime_v2_foundation_demo_contract().expect("foundation demo");
    runtime_v2_governed_tools_flagship_demo_contract().expect("governed tools flagship");
    runtime_v2_private_state_contract().expect("private state");
    runtime_v2_private_state_envelope_contract().expect("private state envelope");
    runtime_v2_private_state_sealing_contract().expect("private state sealing");
    runtime_v2_private_state_lineage_contract().expect("private state lineage");
    runtime_v2_private_state_witness_contract().expect("private state witness");
    runtime_v2_private_state_anti_equivocation_contract().expect("anti equivocation");
    runtime_v2_private_state_sanctuary_contract().expect("sanctuary");
    runtime_v2_private_state_observatory_contract().expect("private state observatory");
    runtime_v2_citizen_state_substrate_contract().expect("citizen state substrate");
    runtime_v2_memory_identity_architecture_contract().expect("memory identity architecture");
    runtime_v2_theory_of_mind_foundation_contract().expect("theory of mind foundation");
    runtime_v2_intelligence_metric_architecture_contract().expect("intelligence metric architecture");
    runtime_v2_governed_learning_substrate_contract().expect("governed learning substrate");
    runtime_v2_standing_contract().expect("standing");
    runtime_v2_access_control_contract().expect("access control");
    runtime_v2_agent_lifecycle_state_contract().expect("agent lifecycle");
    runtime_v2_continuity_challenge_contract().expect("continuity challenge");
    runtime_v2_observatory_flagship_contract().expect("observatory flagship");
    runtime_v2_cognitive_being_flagship_demo_contract().expect("cognitive being flagship");
    runtime_v2_contract_market_demo_contract().expect("contract market demo");
    runtime_v2_citizen_state_substrate_contract().expect("citizen state substrate");
    runtime_v2_private_state_observatory_contract().expect("private state observatory");
    runtime_v2_acip_hardening_contract().expect("ACIP hardening");
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
