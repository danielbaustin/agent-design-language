use super::*;

#[test]
fn runtime_v2_citizen_state_substrate_contract_is_stable() {
    let packet =
        runtime_v2_citizen_state_substrate_contract().expect("citizen-state substrate packet");
    packet.validate().expect("valid citizen-state substrate");

    assert_eq!(
        packet.schema_version,
        RUNTIME_V2_CITIZEN_STATE_SUBSTRATE_SCHEMA
    );
    assert_eq!(packet.demo_id, "D10");
    assert_eq!(packet.milestone, "v0.91.1");
    assert_eq!(packet.wp, "WP-06");
    assert_eq!(packet.inherited_private_state_milestone, "v0.90.3");
    assert_eq!(packet.audience_views.len(), 4);
    assert_eq!(packet.fixture_matrix.len(), 4);
    assert!(packet
        .audience_views
        .iter()
        .any(|view| view.audience == "runtime"
            && view.artifact_ref == RUNTIME_V2_PRIVATE_STATE_PROJECTION_PATH
            && view.authority_status == "projection_not_authority"));
    assert!(packet
        .fixture_matrix
        .iter()
        .any(|fixture| fixture.fixture_kind == "stale_state"
            && fixture
                .artifact_ref
                .contains("private_state/lineage_negative_cases.json")));
    assert!(packet
        .fixture_matrix
        .iter()
        .any(|fixture| fixture.fixture_kind == "overexposed_projection"
            && fixture
                .artifact_ref
                .contains("observatory/private_state_projection_negative_cases.json")));
}

#[test]
fn runtime_v2_citizen_state_substrate_contract_matches_golden_fixture() {
    let packet =
        runtime_v2_citizen_state_substrate_contract().expect("citizen-state substrate packet");
    let json = String::from_utf8(packet.pretty_json_bytes().expect("substrate json"))
        .expect("utf8 substrate json");

    assert_eq!(
        json,
        include_str!(
            "../../../tests/fixtures/runtime_v2/citizen_state/citizen_state_substrate.json"
        )
        .trim_end()
    );
}

#[test]
fn runtime_v2_citizen_state_substrate_validation_rejects_drift() {
    let mut packet =
        runtime_v2_citizen_state_substrate_contract().expect("citizen-state substrate packet");
    packet.inherited_private_state_milestone = "v0.91.1".to_string();
    assert!(packet
        .validate()
        .expect_err("rewriting inherited milestone should fail")
        .to_string()
        .contains("v0.90.3"));

    let mut packet =
        runtime_v2_citizen_state_substrate_contract().expect("citizen-state substrate packet");
    packet
        .audience_views
        .retain(|view| view.audience != "public");
    assert!(packet
        .validate()
        .expect_err("missing public audience should fail")
        .to_string()
        .contains("runtime/operator/reviewer/public"));

    let mut packet =
        runtime_v2_citizen_state_substrate_contract().expect("citizen-state substrate packet");
    let operator = packet
        .audience_views
        .iter_mut()
        .find(|view| view.audience == "operator")
        .expect("operator audience");
    operator.raw_private_state_allowed = true;
    assert!(packet
        .validate()
        .expect_err("raw private-state allowance should fail")
        .to_string()
        .contains("never allow raw private state"));

    let mut packet =
        runtime_v2_citizen_state_substrate_contract().expect("citizen-state substrate packet");
    packet
        .fixture_matrix
        .retain(|fixture| fixture.fixture_kind != "stale_state");
    assert!(packet
        .validate()
        .expect_err("missing stale fixture should fail")
        .to_string()
        .contains("valid/malformed/stale/overexposed"));

    let mut packet =
        runtime_v2_citizen_state_substrate_contract().expect("citizen-state substrate packet");
    packet.fixture_matrix[0].artifact_ref = "/tmp/leak.json".to_string();
    assert!(packet
        .validate()
        .expect_err("absolute fixture path should fail")
        .to_string()
        .contains("repository-relative"));
}

#[cfg(feature = "slow-proof-tests")]
#[test]
fn runtime_v2_citizen_state_substrate_write_to_root_materializes_fixture() {
    let packet =
        runtime_v2_citizen_state_substrate_contract().expect("citizen-state substrate packet");
    let root = common::unique_temp_path("citizen-state-substrate-write");

    packet.write_to_root(&root).expect("write substrate packet");

    let json = std::fs::read_to_string(root.join(RUNTIME_V2_CITIZEN_STATE_SUBSTRATE_PATH))
        .expect("substrate packet");
    assert!(json.contains(RUNTIME_V2_CITIZEN_STATE_SUBSTRATE_SCHEMA));
    assert!(json.contains("\"milestone\": \"v0.91.1\""));
    assert!(!json.contains(root.to_string_lossy().as_ref()));

    std::fs::remove_dir_all(root).expect("cleanup citizen-state substrate temp root");
}
