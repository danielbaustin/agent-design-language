use super::*;

#[test]
fn runtime_v2_runtime_inhabitant_integration_contract_is_stable() {
    let artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts
        .packet
        .validate()
        .expect("valid runtime inhabitant integration packet");

    assert_eq!(
        artifacts.packet.schema_version,
        RUNTIME_V2_RUNTIME_INHABITANT_INTEGRATION_PACKET_SCHEMA
    );
    assert_eq!(artifacts.packet.milestone, "v0.91.1");
    assert_eq!(artifacts.packet.wp, "WP-15");
    assert_eq!(artifacts.packet.integration_stage_refs.len(), 11);
    assert_eq!(artifacts.packet.trace_refs.len(), 4);
    assert!(artifacts
        .packet
        .non_claims
        .iter()
        .any(|claim| claim.contains("WP-16")));
}

#[test]
fn runtime_v2_runtime_inhabitant_integration_contract_registry_smoke_covers_accessors() {
    runtime_v2_csm_integrated_run_contract().expect("integrated run");
    runtime_v2_standing_contract().expect("standing");
    runtime_v2_citizen_state_substrate_contract().expect("citizen state");
    runtime_v2_agent_lifecycle_state_contract().expect("lifecycle");
    runtime_v2_memory_identity_architecture_contract().expect("memory");
    runtime_v2_theory_of_mind_foundation_contract().expect("theory of mind");
    runtime_v2_intelligence_metric_architecture_contract().expect("intelligence");
    runtime_v2_governed_learning_substrate_contract().expect("learning");
    runtime_v2_access_control_contract().expect("access control");
    runtime_v2_acip_hardening_contract().expect("ACIP hardening");
    runtime_v2_a2a_adapter_boundary_contract().expect("A2A boundary");
    runtime_v2_runtime_inhabitant_integration_contract().expect("runtime inhabitant integration");
}

#[test]
fn runtime_v2_runtime_inhabitant_integration_contract_accessors_cover_shared_registry() {
    runtime_v2_contract_schema_contract().expect("contract schema");
    runtime_v2_manifold_contract().expect("manifold");
    runtime_v2_kernel_loop_contract().expect("kernel loop");
    runtime_v2_citizen_lifecycle_contract().expect("citizen lifecycle");
    runtime_v2_snapshot_rehydration_contract().expect("snapshot rehydration");
    runtime_v2_invariant_violation_contract().expect("invariant violation");
    runtime_v2_invariant_and_violation_contract().expect("invariant + violation");
    runtime_v2_operator_control_report_contract().expect("operator control report");
    runtime_v2_security_boundary_proof_contract().expect("security boundary");
    runtime_v2_csm_run_packet_contract().expect("csm run packet");
    runtime_v2_csm_boot_admission_contract().expect("boot admission");
    runtime_v2_csm_governed_episode_contract().expect("governed episode");
    runtime_v2_csm_freedom_gate_mediation_contract().expect("freedom gate");
    runtime_v2_csm_invalid_action_rejection_contract().expect("invalid action rejection");
    runtime_v2_csm_wake_continuity_contract().expect("wake continuity");
    runtime_v2_csm_observatory_contract().expect("observatory");
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
    runtime_v2_private_state_anti_equivocation_contract().expect("anti-equivocation");
    runtime_v2_private_state_sanctuary_contract().expect("private state sanctuary");
    runtime_v2_private_state_observatory_contract().expect("private state observatory");
    runtime_v2_citizen_state_substrate_contract().expect("citizen state substrate");
    runtime_v2_memory_identity_architecture_contract().expect("memory identity architecture");
    runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation");
    runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture");
    runtime_v2_governed_learning_substrate_contract().expect("governed learning substrate");
    runtime_v2_standing_contract().expect("standing");
    runtime_v2_access_control_contract().expect("access control");
    runtime_v2_acip_hardening_contract().expect("ACIP hardening");
    runtime_v2_a2a_adapter_boundary_contract().expect("A2A boundary");
    runtime_v2_runtime_inhabitant_integration_contract().expect("runtime inhabitant integration");
    runtime_v2_agent_lifecycle_state_contract().expect("agent lifecycle");
    runtime_v2_continuity_challenge_contract().expect("continuity challenge");
    runtime_v2_observatory_flagship_contract().expect("observatory flagship");
    runtime_v2_cognitive_being_flagship_demo_contract().expect("cognitive being flagship");
    runtime_v2_contract_market_demo_contract().expect("contract market demo");
}

#[test]
fn runtime_v2_runtime_inhabitant_integration_matches_golden_fixture_and_report() {
    let artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    let rendered = serde_json::to_value(&artifacts.packet).expect("serialize packet");
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/runtime_v2/inhabitant/runtime_inhabitant_integration.json"
    ))
    .expect("parse fixture");
    assert_eq!(rendered, fixture);

    let expected_report = include_str!(
        "../../../tests/fixtures/runtime_v2/inhabitant/runtime_inhabitant_operator_report.md"
    );
    assert_eq!(
        artifacts.operator_report_markdown.trim_end(),
        expected_report.trim_end()
    );
}

#[test]
fn runtime_v2_runtime_inhabitant_integration_validation_rejects_metadata_drift() {
    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.schema_version =
        "runtime_v2.runtime_inhabitant_integration_packet.v0".to_string();
    assert!(artifacts
        .packet
        .validate()
        .expect_err("schema drift should fail")
        .to_string()
        .contains("unsupported runtime inhabitant integration schema"));

    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.validation_commands.pop();
    assert!(artifacts
        .packet
        .validate()
        .expect_err("validation command drift should fail")
        .to_string()
        .contains("validation command set"));

    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.claim_boundary = "bounded integration".to_string();
    assert!(artifacts
        .packet
        .validate()
        .expect_err("claim boundary drift should fail")
        .to_string()
        .contains("birthday/autonomy claim boundary"));
}

#[test]
fn runtime_v2_runtime_inhabitant_integration_validation_rejects_stage_and_trace_gaps() {
    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.trace_refs.clear();
    assert!(artifacts
        .packet
        .validate()
        .expect_err("missing trace refs should fail")
        .to_string()
        .contains("exactly four reviewed trace refs"));

    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.integration_stage_refs[0].stage_id.clear();
    assert!(artifacts
        .packet
        .validate()
        .expect_err("bad stage id should fail")
        .to_string()
        .contains("stage_id"));

    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.integration_stage_refs[3].sequence = 99;
    assert!(artifacts
        .packet
        .validate()
        .expect_err("sequence drift should fail")
        .to_string()
        .contains("reviewed sequence order"));
}

#[test]
fn runtime_v2_runtime_inhabitant_integration_validate_against_rejects_dependency_drift() {
    let integrated_run = runtime_v2_csm_integrated_run_contract().expect("integrated run");
    let standing = runtime_v2_standing_contract().expect("standing");
    let citizen_state = runtime_v2_citizen_state_substrate_contract().expect("state");
    let lifecycle = runtime_v2_agent_lifecycle_state_contract().expect("lifecycle");
    let memory = runtime_v2_memory_identity_architecture_contract().expect("memory");
    let tom = runtime_v2_theory_of_mind_foundation_contract().expect("tom");
    let intelligence =
        runtime_v2_intelligence_metric_architecture_contract().expect("intelligence");
    let learning = runtime_v2_governed_learning_substrate_contract().expect("learning");
    let access = runtime_v2_access_control_contract().expect("access");
    let acip = runtime_v2_acip_hardening_contract().expect("acip");
    let a2a = runtime_v2_a2a_adapter_boundary_contract().expect("a2a");

    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.integrated_run_ref = "runtime_v2/csm_run/drifted.json".to_string();
    assert!(artifacts
        .packet
        .validate_against(
            &integrated_run,
            &standing,
            &citizen_state,
            &lifecycle,
            &memory,
            &tom,
            &intelligence,
            &learning,
            &access,
            &acip,
            &a2a,
        )
        .expect_err("integrated-run drift should fail")
        .to_string()
        .contains("integrated CSM run proof packet"));

    let mut artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    artifacts.packet.trace_refs[1] = "runtime_v2/trace/drifted.jsonl".to_string();
    assert!(artifacts
        .packet
        .validate_against(
            &integrated_run,
            &standing,
            &citizen_state,
            &lifecycle,
            &memory,
            &tom,
            &intelligence,
            &learning,
            &access,
            &acip,
            &a2a,
        )
        .expect_err("trace drift should fail")
        .to_string()
        .contains("trace refs drifted"));
}

#[test]
fn runtime_v2_runtime_inhabitant_integration_proof_route_paths_exist() {
    let artifacts = runtime_v2_runtime_inhabitant_integration_contract()
        .expect("runtime inhabitant integration artifacts");
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");

    let proof_paths = vec![
        artifacts.packet.source_feature_doc.clone(),
        "adl/src/runtime_v2/runtime_inhabitant_integration.rs".to_string(),
        "adl/src/runtime_v2/tests/runtime_inhabitant_integration.rs".to_string(),
        "adl/tests/fixtures/runtime_v2/inhabitant/runtime_inhabitant_integration.json".to_string(),
        "adl/tests/fixtures/runtime_v2/inhabitant/runtime_inhabitant_operator_report.md"
            .to_string(),
    ];
    for proof_path in proof_paths {
        assert!(
            repo_root.join(&proof_path).exists(),
            "expected proof-route path to exist: {proof_path}"
        );
    }

    let temp_root = common::unique_temp_path("runtime-inhabitant-integration-write");
    artifacts
        .write_to_root(&temp_root)
        .expect("write runtime inhabitant integration artifacts");
    let rendered = std::fs::read_to_string(
        temp_root.join(RUNTIME_V2_RUNTIME_INHABITANT_INTEGRATION_PACKET_PATH),
    )
    .expect("read written packet");
    assert!(rendered.contains("\"wp\": \"WP-15\""));

    let report =
        std::fs::read_to_string(temp_root.join(RUNTIME_V2_RUNTIME_INHABITANT_OPERATOR_REPORT_PATH))
            .expect("read written report");
    assert!(report.contains("Runtime Inhabitant Integration Operator Report"));
}
