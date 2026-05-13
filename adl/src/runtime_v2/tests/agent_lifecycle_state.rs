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
