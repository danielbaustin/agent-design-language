use super::*;

#[test]
fn runtime_v2_memory_identity_architecture_contract_is_stable() {
    let packet = runtime_v2_memory_identity_architecture_contract()
        .expect("memory/identity architecture packet");
    packet.validate().expect("valid memory/identity packet");

    assert_eq!(
        packet.schema_version,
        RUNTIME_V2_MEMORY_IDENTITY_ARCHITECTURE_SCHEMA
    );
    assert_eq!(packet.demo_id, "memory_tom_evidence_demo");
    assert_eq!(packet.milestone, "v0.91.1");
    assert_eq!(packet.wp, "WP-07");
    assert_eq!(packet.memory_surfaces.len(), 7);
    assert_eq!(packet.fixture_matrix.len(), 4);
    assert!(packet
        .memory_surfaces
        .iter()
        .any(
            |surface| surface.surface_kind == "private_state_lineage_ledger"
                && surface.authority_status == "authoritative_continuity_record"
        ));
    assert!(packet
        .memory_surfaces
        .iter()
        .any(|surface| surface.surface_kind == "obsmem_write_contract"
            && surface.artifact_ref == "adl/src/obsmem_contract/models.rs"));
    assert!(packet
        .non_claims
        .iter()
        .any(|claim| claim.contains("does not prove full identity continuity")));
}

#[test]
fn runtime_v2_memory_identity_architecture_contract_registry_smoke_covers_accessors() {
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
    runtime_v2_standing_contract().expect("standing");
    runtime_v2_access_control_contract().expect("access control");
    runtime_v2_agent_lifecycle_state_contract().expect("agent lifecycle");
    runtime_v2_continuity_challenge_contract().expect("continuity challenge");
    runtime_v2_observatory_flagship_contract().expect("observatory flagship");
    runtime_v2_cognitive_being_flagship_demo_contract().expect("cognitive being flagship");
    runtime_v2_contract_market_demo_contract().expect("contract market demo");
}

#[test]
fn runtime_v2_memory_identity_architecture_matches_golden_fixture() {
    let packet = runtime_v2_memory_identity_architecture_contract()
        .expect("memory/identity architecture packet");
    let json = String::from_utf8(packet.pretty_json_bytes().expect("memory/identity json"))
        .expect("utf8 memory/identity json");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/memory_identity/memory_identity_architecture.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_memory_identity_architecture_validation_rejects_shape_drift() {
    let mut packet = runtime_v2_memory_identity_architecture_contract()
        .expect("memory/identity architecture packet");
    packet.demo_id = "D99".to_string();
    assert!(packet
        .validate()
        .expect_err("changing demo route should fail")
        .to_string()
        .contains("memory/ToM evidence demo route"));

    let mut packet = runtime_v2_memory_identity_architecture_contract()
        .expect("memory/identity architecture packet");
    packet
        .memory_surfaces
        .retain(|surface| surface.surface_kind != "indexed_memory_entry");
    assert!(packet
        .validate()
        .expect_err("missing indexed-memory surface should fail")
        .to_string()
        .contains(
            "must cover roster, lineage, witness, receipt, observatory, and ObsMem surfaces"
        ));

    let mut packet = runtime_v2_memory_identity_architecture_contract()
        .expect("memory/identity architecture packet");
    packet.memory_write_example.citations[0].path = "/tmp/leak.json".to_string();
    assert!(packet
        .validate()
        .expect_err("absolute citation path should fail")
        .to_string()
        .contains("relative and must not escape"));

    let mut packet = runtime_v2_memory_identity_architecture_contract()
        .expect("memory/identity architecture packet");
    packet
        .fixture_matrix
        .retain(|fixture| fixture.fixture_kind != "observatory_projection_evidence");
    assert!(packet
        .validate()
        .expect_err("missing observatory evidence fixture should fail")
        .to_string()
        .contains("must cover roster, witness, receipt, and observatory evidence"));
}

#[test]
fn runtime_v2_memory_identity_architecture_validate_against_rejects_dependency_drift() {
    let citizen_state =
        runtime_v2_citizen_state_substrate_contract().expect("citizen-state substrate");
    let boot = runtime_v2_csm_boot_admission_contract().expect("boot admission");
    let lineage = runtime_v2_private_state_lineage_contract().expect("lineage artifacts");
    let witness = runtime_v2_private_state_witness_contract().expect("witness artifacts");
    let observatory =
        runtime_v2_private_state_observatory_contract().expect("observatory artifacts");

    let mut packet = runtime_v2_memory_identity_architecture_contract()
        .expect("memory/identity architecture packet");
    packet.identity_evidence_refs[0] = "runtime_v2/csm_run/drifted_roster.json".to_string();
    assert!(packet
        .validate_against(&citizen_state, &boot, &lineage, &witness, &observatory)
        .expect_err("identity evidence drift should fail")
        .to_string()
        .contains("identity_evidence_refs"));

    let mut packet = runtime_v2_memory_identity_architecture_contract()
        .expect("memory/identity architecture packet");
    let roster_surface = packet
        .memory_surfaces
        .iter_mut()
        .find(|surface| surface.surface_kind == "citizen_roster_memory_roots")
        .expect("citizen roster surface");
    roster_surface.artifact_ref = "runtime_v2/citizens/drifted_roster.json".to_string();
    assert!(packet
        .validate_against(&citizen_state, &boot, &lineage, &witness, &observatory)
        .expect_err("roster drift should fail")
        .to_string()
        .contains("citizen-roster surface"));

    let mut packet = runtime_v2_memory_identity_architecture_contract()
        .expect("memory/identity architecture packet");
    packet.fixture_matrix[0].artifact_ref =
        "adl/tests/fixtures/runtime_v2/csm_run/drifted_citizen_roster.json".to_string();
    assert!(packet
        .validate_against(&citizen_state, &boot, &lineage, &witness, &observatory)
        .expect_err("fixture drift should fail")
        .to_string()
        .contains("fixture_matrix"));

    let mut packet = runtime_v2_memory_identity_architecture_contract()
        .expect("memory/identity architecture packet");
    packet.memory_write_example.citations[1].path =
        "runtime_v2/private_state/drifted_witnesses.json".to_string();
    packet.memory_write_example.normalize();
    assert!(packet
        .validate_against(&citizen_state, &boot, &lineage, &witness, &observatory)
        .expect_err("witness citation drift should fail")
        .to_string()
        .contains("explicit evidence"));

    let mut packet = runtime_v2_memory_identity_architecture_contract()
        .expect("memory/identity architecture packet");
    let observatory_surface = packet
        .memory_surfaces
        .iter_mut()
        .find(|surface| surface.surface_kind == "observatory_projection_packet")
        .expect("observatory surface");
    observatory_surface.authority_status = "authoritative_projection".to_string();
    assert!(packet
        .validate_against(&citizen_state, &boot, &lineage, &witness, &observatory)
        .expect_err("authoritative projection drift should fail")
        .to_string()
        .contains("read-only and non-authoritative"));
}

#[test]
fn runtime_v2_memory_identity_architecture_proof_route_paths_exist() {
    let packet = runtime_v2_memory_identity_architecture_contract()
        .expect("memory/identity architecture packet");
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");
    let boot = runtime_v2_csm_boot_admission_contract().expect("boot admission");
    let lineage = runtime_v2_private_state_lineage_contract().expect("lineage artifacts");
    let witness = runtime_v2_private_state_witness_contract().expect("witness artifacts");
    let observatory =
        runtime_v2_private_state_observatory_contract().expect("observatory artifacts");

    assert_eq!(
        packet.identity_evidence_refs,
        vec![
            boot.citizen_roster.artifact_path,
            lineage.ledger.artifact_path,
            witness.witness_set.artifact_path,
            witness.receipt_set.artifact_path,
            observatory.projection_packet.artifact_path,
        ]
    );

    let mut proof_paths = vec![
        packet.source_feature_doc.clone(),
        "adl/src/runtime_v2/memory_identity_architecture.rs".to_string(),
        "adl/src/runtime_v2/tests/memory_identity_architecture.rs".to_string(),
        "adl/src/runtime_v2/boot_admission.rs".to_string(),
        "adl/src/runtime_v2/private_state_lineage.rs".to_string(),
        "adl/src/runtime_v2/private_state_witness.rs".to_string(),
        "adl/src/runtime_v2/private_state_observatory.rs".to_string(),
        "adl/src/obsmem_contract/models.rs".to_string(),
        "adl/src/obsmem_indexing.rs".to_string(),
    ];
    proof_paths.extend(
        packet
            .fixture_matrix
            .iter()
            .map(|fixture| fixture.artifact_ref.clone()),
    );

    for proof_path in proof_paths {
        assert!(
            repo_root.join(&proof_path).exists(),
            "expected proof-route path to exist: {proof_path}"
        );
    }

    let temp_root = common::unique_temp_path("memory-identity-proof-route");
    packet
        .write_to_root(&temp_root)
        .expect("materialize memory/identity packet");
    assert!(
        temp_root
            .join(RUNTIME_V2_MEMORY_IDENTITY_ARCHITECTURE_PATH)
            .exists(),
        "expected materialized packet path to exist"
    );
    std::fs::remove_dir_all(temp_root).expect("cleanup memory-identity proof-route temp root");
}

#[cfg(feature = "slow-proof-tests")]
#[test]
fn runtime_v2_memory_identity_architecture_write_to_root_materializes_fixture() {
    let packet = runtime_v2_memory_identity_architecture_contract()
        .expect("memory/identity architecture packet");
    let root = common::unique_temp_path("memory-identity-architecture-write");

    packet
        .write_to_root(&root)
        .expect("write memory/identity packet");

    let json = std::fs::read_to_string(root.join(RUNTIME_V2_MEMORY_IDENTITY_ARCHITECTURE_PATH))
        .expect("memory/identity packet");
    assert!(json.contains(RUNTIME_V2_MEMORY_IDENTITY_ARCHITECTURE_SCHEMA));
    assert!(json.contains("memory_tom_evidence_demo"));

    std::fs::remove_dir_all(root).expect("cleanup memory/identity temp root");
}
