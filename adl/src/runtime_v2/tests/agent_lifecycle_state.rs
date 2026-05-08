use super::common::unique_temp_path;
use super::*;
use std::fs;

#[test]
fn runtime_v2_agent_lifecycle_state_contract_covers_required_states_and_policies() {
    let artifacts = runtime_v2_agent_lifecycle_state_contract().expect("lifecycle");

    let states = artifacts
        .state_contract
        .states
        .iter()
        .map(|state| state.state.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        states,
        vec![
            "ACTIVE",
            "QUIESCENT",
            "SUSPENDED",
            "DORMANT",
            "SIMULATION",
            "IN_TRANSIT",
            "BOOTSTRAP",
            "SHUTDOWN",
            "FORCED_SUSPENSION",
            "QUARANTINED",
            "REJECTED",
            "ORPHANED",
        ]
    );

    let active = artifacts
        .state_contract
        .states
        .iter()
        .find(|state| state.state == "ACTIVE")
        .expect("active");
    assert_eq!(active.runtime_binding_state, "active");
    assert!(active.capabilities.freedom_gate_agency_available);
    assert!(active.capabilities.aee_execution_available);
    assert_eq!(
        active.capabilities.acip_invocation_policy,
        "allowed_via_active_agency_path"
    );

    let simulation = artifacts
        .state_contract
        .states
        .iter()
        .find(|state| state.state == "SIMULATION")
        .expect("simulation");
    assert_eq!(simulation.runtime_binding_state, "overlay_only");
    assert!(!simulation.capabilities.external_commitment_allowed);
    assert_eq!(
        simulation.capabilities.acip_receipt_policy,
        "sealed_internal_replay_only"
    );

    let in_transit = artifacts
        .state_contract
        .states
        .iter()
        .find(|state| state.state == "IN_TRANSIT")
        .expect("in_transit");
    assert_eq!(in_transit.runtime_binding_state, "snapshotting");
    assert_eq!(
        in_transit.capabilities.chronosense_continuity,
        "sealed_continuity_only"
    );
    assert_eq!(in_transit.capabilities.acip_invocation_policy, "forbidden");
}

#[test]
fn runtime_v2_agent_lifecycle_transition_matrix_and_fixtures_cover_failure_and_custody_paths() {
    let artifacts = runtime_v2_agent_lifecycle_state_contract().expect("lifecycle");

    assert!(artifacts
        .transition_matrix
        .transitions
        .iter()
        .any(|transition| transition.from_state == "SUSPENDED"
            && transition.to_state == "ACTIVE"
            && transition.transition_kind == "normal"
            && transition.allowed));
    assert!(artifacts
        .transition_matrix
        .transitions
        .iter()
        .any(|transition| transition.from_state == "ACTIVE"
            && transition.to_state == "FORCED_SUSPENSION"
            && transition.transition_kind == "failure"
            && transition.allowed));
    assert!(artifacts
        .transition_matrix
        .transitions
        .iter()
        .any(|transition| transition.from_state == "FORCED_SUSPENSION"
            && transition.to_state == "QUARANTINED"
            && transition.transition_kind == "quarantine"
            && transition.allowed));
    assert!(artifacts
        .transition_matrix
        .transitions
        .iter()
        .any(|transition| transition.from_state == "SIMULATION"
            && transition.to_state == "ACTIVE"
            && !transition.allowed));

    let fixture_kinds = artifacts
        .proof_fixtures
        .fixtures
        .iter()
        .map(|fixture| fixture.fixture_kind.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        fixture_kinds,
        vec![
            "active_governed_invocation",
            "quiescent_queue_or_wake",
            "simulation_no_external_action",
            "dormant_continuity_without_active_cognition",
            "suspended_wake_only",
            "in_transit_no_agency",
            "forced_suspension_failure_mode",
            "quarantined_recovery_only",
            "rejected_no_operational_receipt",
            "orphaned_custody_recovery_only",
        ]
    );
}

#[test]
fn runtime_v2_agent_lifecycle_artifacts_write_without_path_leakage() {
    let temp_root = unique_temp_path("agent_lifecycle");
    let artifacts = runtime_v2_agent_lifecycle_state_contract().expect("lifecycle");

    artifacts
        .write_to_root(&temp_root)
        .expect("write lifecycle artifacts");

    let contract_path = temp_root.join(&artifacts.state_contract.artifact_path);
    let matrix_path = temp_root.join(&artifacts.transition_matrix.artifact_path);
    let fixtures_path = temp_root.join(&artifacts.proof_fixtures.artifact_path);
    assert!(contract_path.is_file());
    assert!(matrix_path.is_file());
    assert!(fixtures_path.is_file());

    let contract = fs::read_to_string(contract_path).expect("contract text");
    let fixtures = fs::read_to_string(fixtures_path).expect("fixtures text");
    let temp_root_text = temp_root.to_string_lossy();
    assert!(!contract.contains(temp_root_text.as_ref()));
    assert!(!fixtures.contains(temp_root_text.as_ref()));
    assert!(contract.contains("\"ACTIVE\""));
    assert!(fixtures.contains("\"simulation_no_external_action\""));

    fs::remove_dir_all(temp_root).ok();
}

#[test]
fn runtime_v2_agent_lifecycle_validation_rejects_state_or_fixture_drift() {
    let mut artifacts = runtime_v2_agent_lifecycle_state_contract().expect("lifecycle");
    artifacts.state_contract.states[0]
        .capabilities
        .acip_invocation_policy = "forbidden".to_string();
    assert!(artifacts
        .validate()
        .expect_err("active invocation policy drift should fail")
        .to_string()
        .contains("reviewed state order and capability matrix"));

    let mut artifacts = runtime_v2_agent_lifecycle_state_contract().expect("lifecycle");
    artifacts.proof_fixtures.fixtures.pop();
    assert!(artifacts
        .validate()
        .expect_err("fixture coverage drift should fail")
        .to_string()
        .contains("reviewed lifecycle proof cases"));
}

#[test]
fn runtime_v2_agent_lifecycle_state_contract_registry_accessors_cover_runtime_v2_contracts_module() {
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
    runtime_v2_standing_contract().expect("standing");
    runtime_v2_access_control_contract().expect("access control");
    runtime_v2_agent_lifecycle_state_contract().expect("agent lifecycle");
    runtime_v2_continuity_challenge_contract().expect("continuity challenge");
    runtime_v2_observatory_flagship_contract().expect("observatory flagship");
    runtime_v2_cognitive_being_flagship_demo_contract().expect("cognitive being flagship");
    runtime_v2_contract_market_demo_contract().expect("contract market demo");
}
