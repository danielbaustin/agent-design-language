use super::*;

#[test]
fn runtime_v2_governed_learning_substrate_contract_is_stable() {
    let packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.validate().expect("valid governed learning packet");

    assert_eq!(
        packet.schema_version,
        RUNTIME_V2_GOVERNED_LEARNING_SUBSTRATE_SCHEMA
    );
    assert_eq!(packet.milestone, "v0.91.1");
    assert_eq!(packet.wp, "WP-11");
    assert_eq!(packet.fixture_matrix.len(), 3);
    assert_eq!(packet.rollback_policy.required_audit_artifacts.len(), 2);
    assert!(packet
        .non_claims
        .iter()
        .any(|claim| claim.contains("autonomous retraining")));
}

#[test]
fn runtime_v2_governed_learning_substrate_contract_registry_smoke_covers_accessors() {
    runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture");
    runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation");
    runtime_v2_governed_learning_substrate_contract().expect("governed learning substrate");
}

#[test]
fn runtime_v2_governed_learning_substrate_contract_accessors_cover_shared_registry() {
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
    runtime_v2_agent_lifecycle_state_contract().expect("agent lifecycle");
    runtime_v2_continuity_challenge_contract().expect("continuity challenge");
    runtime_v2_observatory_flagship_contract().expect("observatory flagship");
    runtime_v2_cognitive_being_flagship_demo_contract().expect("cognitive being flagship");
    runtime_v2_contract_market_demo_contract().expect("contract market demo");
}

#[test]
fn runtime_v2_governed_learning_substrate_matches_golden_fixture() {
    let packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    let json = String::from_utf8(packet.pretty_json_bytes().expect("governed learning json"))
        .expect("utf8 governed learning json");
    let expected = include_str!(
        "../../../tests/fixtures/runtime_v2/learning/governed_learning_substrate.json"
    )
    .trim_end()
    .to_string();
    assert_eq!(json, expected);
}

#[test]
fn runtime_v2_governed_learning_substrate_validation_rejects_shape_drift() {
    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.capability_dependency_ref = "runtime_v2/drifted/capability.json".to_string();
    assert!(packet
        .validate()
        .expect_err("drifted capability dependency should fail")
        .to_string()
        .contains("landed capability artifact root"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.rollback_policy.required_audit_artifacts =
        vec!["learning/overlays/only.json".to_string()];
    assert!(packet
        .validate()
        .expect_err("missing rollback audit artifact should fail")
        .to_string()
        .contains("rollback linkage"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[0].rollback_ref = None;
    assert!(packet
        .validate()
        .expect_err("accepted fixture without rollback should fail")
        .to_string()
        .contains("rollback reference"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[0].rollback_ref = Some("/tmp/rollback.json".to_string());
    assert!(packet
        .validate()
        .expect_err("absolute rollback path should fail")
        .to_string()
        .contains("governed_learning.fixture_matrix.rollback_ref"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[2].denial_reason = Some("unsafe update".to_string());
    assert!(packet
        .validate()
        .expect_err("unsafe fixture denial reason drift should fail")
        .to_string()
        .contains("hidden self-modification"));
}

#[test]
fn runtime_v2_governed_learning_substrate_validate_against_rejects_dependency_drift() {
    let intelligence = runtime_v2_intelligence_metric_architecture_contract()
        .expect("intelligence metric architecture");
    let tom = runtime_v2_theory_of_mind_foundation_contract().expect("theory-of-mind foundation");

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.intelligence_dependency_ref = "runtime_v2/intelligence/drifted.json".to_string();
    assert!(packet
        .validate_against(&intelligence, &tom)
        .expect_err("drifted intelligence dependency should fail")
        .to_string()
        .contains("intelligence dependency drifted"));

    let mut packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    packet.fixture_matrix[0].evidence_refs =
        vec!["docs/milestones/v0.91.1/review/unknown.json".to_string()];
    assert!(packet
        .validate_against(&intelligence, &tom)
        .expect_err("fixture matrix drift should fail")
        .to_string()
        .contains("fixture matrix"));
}

#[test]
fn runtime_v2_governed_learning_substrate_proof_route_paths_exist() {
    let packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");
    let mut proof_paths = vec![
        packet.source_feature_doc.clone(),
        packet.overlay_guardrails_source_ref.clone(),
        packet.overlay_runtime_source_ref.clone(),
        "adl/src/runtime_v2/governed_learning_substrate.rs".to_string(),
        "adl/src/runtime_v2/tests/governed_learning_substrate.rs".to_string(),
        "adl/tests/fixtures/runtime_v2/learning/governed_learning_substrate.json".to_string(),
    ];
    proof_paths.extend(
        packet
            .fixture_matrix
            .iter()
            .map(|fixture| fixture.artifact_ref.clone()),
    );
    proof_paths.push(RUNTIME_V2_GOVERNED_LEARNING_ROLLBACK_FIXTURE_PATH.to_string());

    for proof_path in proof_paths {
        assert!(
            repo_root.join(&proof_path).exists(),
            "expected proof-route path to exist: {proof_path}"
        );
    }
}

#[test]
fn runtime_v2_governed_learning_substrate_write_to_path_materializes_fixture() {
    let packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    let output_path = common::unique_temp_path("governed-learning-write")
        .join("runtime_v2/learning/governed_learning_substrate.json");

    packet
        .write_to_path(&output_path)
        .expect("write governed learning packet to explicit path");

    let written =
        std::fs::read_to_string(&output_path).expect("read written governed learning json");
    assert_eq!(
        written.trim_end(),
        String::from_utf8(packet.pretty_json_bytes().expect("governed learning json"))
            .expect("utf8")
            .trim_end()
    );
}

#[test]
fn runtime_v2_governed_learning_substrate_write_to_root_materializes_fixture() {
    let packet =
        runtime_v2_governed_learning_substrate_contract().expect("governed learning packet");
    let root = common::unique_temp_path("governed-learning-root");

    packet
        .write_to_root(&root)
        .expect("write governed learning packet to temp root");

    let written = std::fs::read_to_string(root.join(RUNTIME_V2_GOVERNED_LEARNING_SUBSTRATE_PATH))
        .expect("read rooted governed learning json");
    assert_eq!(
        written.trim_end(),
        String::from_utf8(packet.pretty_json_bytes().expect("governed learning json"))
            .expect("utf8")
            .trim_end()
    );
}

#[test]
fn runtime_v2_governed_learning_review_bundle_matches_tracked_artifacts() {
    let bundle = build_governed_learning_review_bundle();
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");
    let tracked_root = repo_root.join(RUNTIME_V2_GOVERNED_LEARNING_REVIEW_ROOT);

    assert_eq!(
        std::fs::read_to_string(tracked_root.join("accepted_feedback_update.json"))
            .expect("read accepted feedback update")
            .trim_end(),
        bundle.accepted_feedback_update_json.trim_end()
    );
    assert_eq!(
        std::fs::read_to_string(tracked_root.join("accepted_feedback_rollback.json"))
            .expect("read accepted feedback rollback")
            .trim_end(),
        bundle.accepted_feedback_rollback_json.trim_end()
    );
    assert_eq!(
        std::fs::read_to_string(tracked_root.join("rejected_feedback_claim.json"))
            .expect("read rejected feedback claim")
            .trim_end(),
        bundle.rejected_feedback_claim_json.trim_end()
    );
    assert_eq!(
        std::fs::read_to_string(tracked_root.join("unsafe_hidden_update_claim.json"))
            .expect("read unsafe hidden update claim")
            .trim_end(),
        bundle.unsafe_hidden_update_claim_json.trim_end()
    );
}
