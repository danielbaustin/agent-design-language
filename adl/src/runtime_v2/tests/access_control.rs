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
fn runtime_v2_access_control_matrix_matches_golden_fixture() {
    let artifacts = runtime_v2_access_control_contract().expect("access artifacts");
    let json = String::from_utf8(
        artifacts
            .authority_matrix
            .pretty_json_bytes()
            .expect("matrix json"),
    )
    .expect("utf8 matrix");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/access_control/authority_matrix.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_access_control_events_match_golden_fixture() {
    let artifacts = runtime_v2_access_control_contract().expect("access artifacts");
    let json = String::from_utf8(
        artifacts
            .event_packet
            .pretty_json_bytes()
            .expect("events json"),
    )
    .expect("utf8 events");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/access_control/access_events.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_access_control_denials_match_golden_fixture() {
    let artifacts = runtime_v2_access_control_contract().expect("access artifacts");
    let json = String::from_utf8(
        artifacts
            .denial_fixtures
            .pretty_json_bytes()
            .expect("denials json"),
    )
    .expect("utf8 denials");

    assert_eq!(
        json,
        include_str!("../../../tests/fixtures/runtime_v2/access_control/denial_fixtures.json")
            .trim_end()
    );
}

#[test]
fn runtime_v2_access_control_requires_event_for_every_sensitive_path() {
    let artifacts = runtime_v2_access_control_contract().expect("access artifacts");
    let mut packet = artifacts.event_packet.clone();
    packet
        .events
        .retain(|event| event.access_path != "inspection");
    packet.packet_hash = packet.computed_hash();

    assert!(packet
        .validate_against(
            &artifacts.authority_matrix,
            &artifacts.observatory_artifacts
        )
        .expect_err("missing sensitive event should fail")
        .to_string()
        .contains("every sensitive access path emits an auditable event"));
}

#[test]
fn runtime_v2_access_control_denied_access_cannot_leak_raw_private_state() {
    let artifacts = runtime_v2_access_control_contract().expect("access artifacts");
    let mut packet = artifacts.event_packet.clone();
    let inspection = packet
        .events
        .iter_mut()
        .find(|event| event.access_path == "inspection")
        .expect("inspection event");
    inspection.raw_private_state_disclosed = true;
    packet.packet_hash = packet.computed_hash();

    assert!(packet
        .validate_against(
            &artifacts.authority_matrix,
            &artifacts.observatory_artifacts
        )
        .expect_err("raw leakage should fail")
        .to_string()
        .contains("denied access must not leak raw private state"));
}

#[test]
fn runtime_v2_access_control_denied_decryption_cannot_return_cleartext() {
    let artifacts = runtime_v2_access_control_contract().expect("access artifacts");
    let mut packet = artifacts.event_packet.clone();
    let decryption = packet
        .events
        .iter_mut()
        .find(|event| event.access_path == "decryption")
        .expect("decryption event");
    decryption.raw_private_state_disclosed = true;
    decryption
        .granted_authority
        .push("decrypted_payload".to_string());
    packet.packet_hash = packet.computed_hash();

    assert!(packet
        .validate_against(
            &artifacts.authority_matrix,
            &artifacts.observatory_artifacts
        )
        .expect_err("cleartext denial should fail")
        .to_string()
        .contains("denied access must not leak raw private state"));
}

#[test]
fn runtime_v2_access_control_denied_access_cannot_mutate_continuity() {
    let artifacts = runtime_v2_access_control_contract().expect("access artifacts");
    let mut packet = artifacts.event_packet.clone();
    let migration = packet
        .events
        .iter_mut()
        .find(|event| event.access_path == "migration")
        .expect("migration event");
    migration.continuity_mutated = true;
    migration.continuity_sequence_after += 1;
    packet.packet_hash = packet.computed_hash();

    assert!(packet
        .validate_against(
            &artifacts.authority_matrix,
            &artifacts.observatory_artifacts
        )
        .expect_err("continuity mutation should fail")
        .to_string()
        .contains("denied access must not mutate citizen continuity"));
}

#[test]
fn runtime_v2_access_control_allowed_projection_cannot_become_raw_inspection() {
    let artifacts = runtime_v2_access_control_contract().expect("access artifacts");
    let mut packet = artifacts.event_packet.clone();
    let projection = packet
        .events
        .iter_mut()
        .find(|event| event.access_path == "projection")
        .expect("projection event");
    projection.raw_private_state_disclosed = true;
    projection
        .granted_authority
        .push("inspect_raw_private_state".to_string());
    packet.packet_hash = packet.computed_hash();

    assert!(packet
        .validate_against(
            &artifacts.authority_matrix,
            &artifacts.observatory_artifacts
        )
        .expect_err("raw projection should fail")
        .to_string()
        .contains("access events must not disclose raw private state"));
}

#[test]
fn runtime_v2_access_control_denied_release_cannot_grant_authority() {
    let artifacts = runtime_v2_access_control_contract().expect("access artifacts");
    let mut packet = artifacts.event_packet.clone();
    let release = packet
        .events
        .iter_mut()
        .find(|event| event.access_path == "release")
        .expect("release event");
    release
        .granted_authority
        .push("release_from_quarantine".to_string());
    packet.packet_hash = packet.computed_hash();

    assert!(packet
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
