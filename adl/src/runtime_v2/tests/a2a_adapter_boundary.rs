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
fn runtime_v2_a2a_adapter_boundary_contract_registry_smoke_covers_accessors() {
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
    runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture");
    runtime_v2_governed_learning_substrate_contract().expect("governed learning substrate");
    runtime_v2_standing_contract().expect("standing");
    runtime_v2_access_control_contract().expect("access control");
    runtime_v2_agent_lifecycle_state_contract().expect("agent lifecycle");
    runtime_v2_continuity_challenge_contract().expect("continuity challenge");
    runtime_v2_observatory_flagship_contract().expect("observatory flagship");
    runtime_v2_cognitive_being_flagship_demo_contract().expect("cognitive being flagship");
    runtime_v2_contract_market_demo_contract().expect("contract market demo");
    runtime_v2_acip_hardening_contract().expect("ACIP hardening");
    runtime_v2_a2a_adapter_boundary_contract().expect("A2A adapter boundary");
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
    packet.source_feature_doc = "docs/milestones/v0.91/features/A2A_EXTERNAL_AGENT_ADAPTER.md".to_string();
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
