use super::*;

#[test]
fn runtime_v2_access_control_contract_is_stable() {
    let artifacts = runtime_v2_access_control_contract().expect("access artifacts");
    artifacts.validate().expect("valid access artifacts");

    assert_eq!(
        artifacts.authority_matrix.schema_version,
        RUNTIME_V2_ACCESS_AUTHORITY_MATRIX_SCHEMA
    );
    assert_eq!(
        artifacts.event_packet.schema_version,
        RUNTIME_V2_ACCESS_EVENT_PACKET_SCHEMA
    );
    assert_eq!(artifacts.authority_matrix.demo_id, "D10");
    assert_eq!(artifacts.authority_matrix.rules.len(), 9);
    assert_eq!(artifacts.event_packet.events.len(), 9);
    assert_eq!(artifacts.denial_fixtures.required_denials.len(), 6);
}

#[test]
fn runtime_v2_access_control_serializes_and_matches_golden_fixtures() {
    let artifacts = runtime_v2_access_control_contract().expect("access artifacts");
    let matrix_json = String::from_utf8(
        artifacts
            .authority_matrix
            .pretty_json_bytes()
            .expect("matrix json"),
    )
    .expect("utf8 matrix");
    assert_eq!(
        matrix_json,
        include_str!("../../../tests/fixtures/runtime_v2/access_control/authority_matrix.json")
            .trim_end()
    );
    let events_json = String::from_utf8(
        artifacts
            .event_packet
            .pretty_json_bytes()
            .expect("events json"),
    )
    .expect("utf8 events");
    assert_eq!(
        events_json,
        include_str!("../../../tests/fixtures/runtime_v2/access_control/access_events.json")
            .trim_end()
    );
    let denials_json = String::from_utf8(
        artifacts
            .denial_fixtures
            .pretty_json_bytes()
            .expect("denials json"),
    )
    .expect("utf8 denials");
    assert_eq!(
        denials_json,
        include_str!("../../../tests/fixtures/runtime_v2/access_control/denial_fixtures.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_access_control_rejects_unsafe_event_mutations() {
    let artifacts = runtime_v2_access_control_contract().expect("access artifacts");
    let mut missing_event = artifacts.event_packet.clone();
    missing_event
        .events
        .retain(|event| event.access_path != "inspection");
    missing_event.packet_hash = missing_event.computed_hash();
    assert!(missing_event
        .validate_against(
            &artifacts.authority_matrix,
            &artifacts.observatory_artifacts
        )
        .expect_err("missing sensitive event should fail")
        .to_string()
        .contains("every sensitive access path emits an auditable event"));

    let mut denied_inspection = artifacts.event_packet.clone();
    let inspection = denied_inspection
        .events
        .iter_mut()
        .find(|event| event.access_path == "inspection")
        .expect("inspection event");
    inspection.raw_private_state_disclosed = true;
    denied_inspection.packet_hash = denied_inspection.computed_hash();
    assert!(denied_inspection
        .validate_against(
            &artifacts.authority_matrix,
            &artifacts.observatory_artifacts
        )
        .expect_err("raw leakage should fail")
        .to_string()
        .contains("denied access must not leak raw private state"));

    let mut denied_decryption = artifacts.event_packet.clone();
    let decryption = denied_decryption
        .events
        .iter_mut()
        .find(|event| event.access_path == "decryption")
        .expect("decryption event");
    decryption.raw_private_state_disclosed = true;
    decryption
        .granted_authority
        .push("decrypted_payload".to_string());
    denied_decryption.packet_hash = denied_decryption.computed_hash();
    assert!(denied_decryption
        .validate_against(
            &artifacts.authority_matrix,
            &artifacts.observatory_artifacts
        )
        .expect_err("cleartext denial should fail")
        .to_string()
        .contains("denied access must not leak raw private state"));

    let mut denied_migration = artifacts.event_packet.clone();
    let migration = denied_migration
        .events
        .iter_mut()
        .find(|event| event.access_path == "migration")
        .expect("migration event");
    migration.continuity_mutated = true;
    migration.continuity_sequence_after += 1;
    denied_migration.packet_hash = denied_migration.computed_hash();
    assert!(denied_migration
        .validate_against(
            &artifacts.authority_matrix,
            &artifacts.observatory_artifacts
        )
        .expect_err("continuity mutation should fail")
        .to_string()
        .contains("denied access must not mutate citizen continuity"));

    let mut projection = artifacts.event_packet.clone();
    let projection_event = projection
        .events
        .iter_mut()
        .find(|event| event.access_path == "projection")
        .expect("projection event");
    projection_event.raw_private_state_disclosed = true;
    projection_event
        .granted_authority
        .push("inspect_raw_private_state".to_string());
    projection.packet_hash = projection.computed_hash();
    assert!(projection
        .validate_against(
            &artifacts.authority_matrix,
            &artifacts.observatory_artifacts
        )
        .expect_err("raw projection should fail")
        .to_string()
        .contains("access events must not disclose raw private state"));

    let mut denied_release = artifacts.event_packet.clone();
    let release = denied_release
        .events
        .iter_mut()
        .find(|event| event.access_path == "release")
        .expect("release event");
    release
        .granted_authority
        .push("release_from_quarantine".to_string());
    denied_release.packet_hash = denied_release.computed_hash();
    assert!(denied_release
        .validate_against(
            &artifacts.authority_matrix,
            &artifacts.observatory_artifacts
        )
        .expect_err("denied grant should fail")
        .to_string()
        .contains("denied access decision cannot grant authority"));
}

#[test]
fn runtime_v2_access_control_write_to_root_materializes_fixtures() {
    let artifacts = runtime_v2_access_control_contract().expect("access artifacts");
    let root = common::unique_temp_path("access-control-write");

    artifacts
        .write_to_root(&root)
        .expect("write access artifacts");

    for rel_path in [
        RUNTIME_V2_ACCESS_AUTHORITY_MATRIX_PATH,
        RUNTIME_V2_ACCESS_EVENT_PACKET_PATH,
        RUNTIME_V2_ACCESS_DENIAL_FIXTURES_PATH,
    ] {
        let text = std::fs::read_to_string(root.join(rel_path)).expect("artifact text");
        assert!(text.contains("D10"));
        assert!(text.contains("WP-12") || text.contains("access"));
        assert!(!text.contains(root.to_string_lossy().as_ref()));
    }

    std::fs::remove_dir_all(root).expect("cleanup access-control temp root");
}
