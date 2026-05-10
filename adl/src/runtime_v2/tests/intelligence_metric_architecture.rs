use super::*;

#[test]
fn runtime_v2_intelligence_metric_architecture_contract_is_stable() {
    let packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    packet.validate().expect("valid intelligence packet");

    assert_eq!(
        packet.schema_version,
        RUNTIME_V2_INTELLIGENCE_METRIC_ARCHITECTURE_SCHEMA
    );
    assert_eq!(packet.milestone, "v0.91.1");
    assert_eq!(packet.wp, "WP-10");
    assert_eq!(packet.evidence_surfaces.len(), 5);
    assert_eq!(packet.metric_dimensions.len(), 3);
    assert_eq!(packet.fixture_reports.len(), 2);
    assert!(packet
        .non_claims
        .iter()
        .any(|claim| claim.contains("public leaderboard")));
}

#[test]
fn runtime_v2_intelligence_metric_architecture_contract_registry_smoke_covers_accessors() {
    runtime_v2_memory_identity_architecture_contract().expect("memory identity architecture");
    runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation");
    runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture");
}

#[test]
fn runtime_v2_intelligence_metric_architecture_contract_accessors_cover_shared_registry() {
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
    runtime_v2_standing_contract().expect("standing");
    runtime_v2_access_control_contract().expect("access control");
    runtime_v2_agent_lifecycle_state_contract().expect("agent lifecycle");
    runtime_v2_continuity_challenge_contract().expect("continuity challenge");
    runtime_v2_observatory_flagship_contract().expect("observatory flagship");
    runtime_v2_cognitive_being_flagship_demo_contract().expect("cognitive being flagship");
    runtime_v2_contract_market_demo_contract().expect("contract market demo");
}

#[test]
fn runtime_v2_intelligence_metric_architecture_matches_golden_fixture() {
    let packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    let json = String::from_utf8(packet.pretty_json_bytes().expect("intelligence json"))
        .expect("utf8 intelligence json");
    let expected = include_str!(
        "../../../tests/fixtures/runtime_v2/intelligence/intelligence_metric_architecture.json"
    )
    .trim_end()
    .to_string();
    assert_eq!(json, expected);
}

#[test]
fn runtime_v2_intelligence_metric_architecture_validation_rejects_shape_drift() {
    let mut packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    packet.capability_dependency_ref = "docs/milestones/v0.91.1/review/drifted".to_string();
    assert!(packet
        .validate()
        .expect_err("drifted capability dependency should fail")
        .to_string()
        .contains("landed capability harness artifact root"));

    let mut packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    packet.metric_dimensions[2].prohibited_uses.clear();
    assert!(packet
        .validate()
        .expect_err("missing prohibited uses should fail")
        .to_string()
        .contains("must declare prohibited uses"));

    let mut packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    packet.cognitive_compression_cost.cost_axes = vec!["latency".to_string()];
    assert!(packet
        .validate()
        .expect_err("ccc axes drift should fail")
        .to_string()
        .contains("bounded cost axes"));
}

#[test]
fn runtime_v2_intelligence_metric_architecture_validate_against_rejects_dependency_drift() {
    let tom = runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation");

    let mut packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    packet.evidence_surfaces[2].artifact_ref =
        "runtime_v2/theory_of_mind/drifted_foundation.json".to_string();
    assert!(packet
        .validate_against(&tom)
        .expect_err("drifted tom surface should fail")
        .to_string()
        .contains("evidence surfaces"));

    let mut packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    packet.metric_dimensions[0].evidence_refs =
        vec!["docs/milestones/v0.91.1/review/unknown.json".to_string()];
    assert!(packet
        .validate_against(&tom)
        .expect_err("unknown dimension evidence should fail")
        .to_string()
        .contains("references unknown evidence"));
}

#[test]
fn runtime_v2_intelligence_metric_architecture_proof_route_paths_exist() {
    let packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");
    let mut proof_paths = vec![
        packet.source_feature_doc.clone(),
        "adl/src/runtime_v2/intelligence_metric_architecture.rs".to_string(),
        "adl/src/runtime_v2/tests/intelligence_metric_architecture.rs".to_string(),
        "adl/tests/fixtures/runtime_v2/intelligence/intelligence_metric_architecture.json"
            .to_string(),
    ];
    proof_paths.extend(
        packet
            .fixture_reports
            .iter()
            .map(|report| report.artifact_ref.clone()),
    );

    for proof_path in proof_paths {
        assert!(
            repo_root.join(&proof_path).exists(),
            "expected proof-route path to exist: {proof_path}"
        );
    }
}

#[test]
fn runtime_v2_intelligence_metric_architecture_write_to_path_materializes_fixture() {
    let packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    let output_path = common::unique_temp_path("intelligence-metric-write")
        .join("runtime_v2/intelligence/intelligence_metric_architecture.json");

    packet
        .write_to_path(&output_path)
        .expect("write intelligence packet to explicit path");

    let written = std::fs::read_to_string(&output_path).expect("read written intelligence json");
    assert_eq!(
        written.trim_end(),
        String::from_utf8(packet.pretty_json_bytes().expect("intelligence json"))
            .expect("utf8")
            .trim_end()
    );
}

#[test]
fn runtime_v2_intelligence_metric_architecture_write_to_root_materializes_fixture() {
    let packet = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture packet");
    let root = common::unique_temp_path("intelligence-metric-root");

    packet
        .write_to_root(&root)
        .expect("write intelligence packet to temp root");

    let written =
        std::fs::read_to_string(root.join(RUNTIME_V2_INTELLIGENCE_METRIC_ARCHITECTURE_PATH))
            .expect("read rooted intelligence json");
    assert_eq!(
        written.trim_end(),
        String::from_utf8(packet.pretty_json_bytes().expect("intelligence json"))
            .expect("utf8")
            .trim_end()
    );
}

#[test]
fn runtime_v2_intelligence_metric_fixture_bundle_matches_tracked_artifacts() {
    let bundle = build_intelligence_metric_fixture_bundle();
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");
    let tracked_root = repo_root.join(RUNTIME_V2_INTELLIGENCE_METRIC_REPORT_ROOT);

    assert_eq!(
        std::fs::read_to_string(tracked_root.join("scorecard.json"))
            .expect("read tracked scorecard")
            .trim_end(),
        bundle.scorecard_json.trim_end()
    );
    assert_eq!(
        std::fs::read_to_string(tracked_root.join("final_report.md"))
            .expect("read tracked report")
            .trim_end(),
        bundle.final_report_md.trim_end()
    );
}
