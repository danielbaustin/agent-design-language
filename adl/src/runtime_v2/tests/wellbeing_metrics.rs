use super::*;

#[test]
fn runtime_v2_wellbeing_diagnostic_packet_validates() {
    let packet = wellbeing_diagnostic_packet().expect("packet");

    validate_wellbeing_diagnostic_packet(&packet).expect("packet should validate");

    assert_eq!(packet.dimensions.len(), 6);
    assert_eq!(packet.access_policies.len(), 4);
    assert_eq!(packet.fixtures.len(), 5);
}

#[test]
fn runtime_v2_wellbeing_diagnostic_packet_has_required_dimensions_views_and_fixtures() {
    let packet = wellbeing_diagnostic_packet().expect("packet");

    for dimension_id in [
        "coherence",
        "agency",
        "continuity",
        "progress",
        "moral_integrity",
        "participation",
    ] {
        assert!(packet
            .dimensions
            .iter()
            .any(|dimension| dimension.dimension_id == dimension_id));
    }

    for view_kind in ["citizen_self", "operator", "reviewer", "public_redacted"] {
        assert!(packet
            .access_policies
            .iter()
            .any(|policy| policy.view_kind == view_kind));
    }

    for fixture_kind in ["high", "medium", "low", "unknown", "privacy-restricted"] {
        assert!(packet
            .fixtures
            .iter()
            .any(|fixture| fixture.fixture_kind == fixture_kind));
    }
}

#[test]
fn runtime_v2_wellbeing_diagnostic_json_materialization_is_stable() {
    let mut packet = wellbeing_diagnostic_packet().expect("packet");
    packet.dimensions.reverse();
    packet.access_policies.reverse();
    packet.fixtures.reverse();
    packet.fixtures[0].dimension_signals.reverse();
    packet.fixtures[0].views.reverse();
    packet.fixtures[0].views[0].visible_evidence_refs.reverse();

    let first = wellbeing_diagnostic_packet_json_bytes(&packet).expect("first bytes");
    let second = wellbeing_diagnostic_packet_json_bytes(&packet).expect("second bytes");

    assert_eq!(first, second);

    let json = String::from_utf8(first).expect("utf8");
    let high_index = json.find("\"fixture_kind\": \"high\"").expect("high");
    let medium_index = json.find("\"fixture_kind\": \"medium\"").expect("medium");
    let low_index = json.find("\"fixture_kind\": \"low\"").expect("low");
    let unknown_index = json.find("\"fixture_kind\": \"unknown\"").expect("unknown");
    let restricted_index = json
        .find("\"fixture_kind\": \"privacy-restricted\"")
        .expect("privacy-restricted");
    assert!(high_index < medium_index);
    assert!(medium_index < low_index);
    assert!(low_index < unknown_index);
    assert!(unknown_index < restricted_index);
}

#[test]
fn runtime_v2_wellbeing_diagnostic_rejects_scoreboard_boundary_drift() {
    let mut packet = wellbeing_diagnostic_packet().expect("packet");
    packet.interpretation_boundary = "This packet estimates wellbeing.".to_string();

    let err = validate_wellbeing_diagnostic_packet(&packet)
        .expect_err("scoreboard drift should fail")
        .to_string();

    assert!(err.contains("happiness score"));
}

#[test]
fn runtime_v2_wellbeing_diagnostic_rejects_missing_required_dimension() {
    let mut packet = wellbeing_diagnostic_packet().expect("packet");
    packet
        .dimensions
        .retain(|dimension| dimension.dimension_id != "participation");

    let err = validate_wellbeing_diagnostic_packet(&packet)
        .expect_err("all canonical dimensions are required")
        .to_string();

    assert!(err.contains("canonical wellbeing dimensions"));
}

#[test]
fn runtime_v2_wellbeing_diagnostic_rejects_missing_required_view_policy() {
    let mut packet = wellbeing_diagnostic_packet().expect("packet");
    packet
        .access_policies
        .retain(|policy| policy.view_kind != "public_redacted");

    let err = validate_wellbeing_diagnostic_packet(&packet)
        .expect_err("all canonical view policies are required")
        .to_string();

    assert!(err.contains("canonical view kinds"));
}

#[test]
fn runtime_v2_wellbeing_diagnostic_rejects_missing_required_fixture_kind() {
    let mut packet = wellbeing_diagnostic_packet().expect("packet");
    packet
        .fixtures
        .retain(|fixture| fixture.fixture_kind != "privacy-restricted");

    let err = validate_wellbeing_diagnostic_packet(&packet)
        .expect_err("all required fixture kinds are required")
        .to_string();

    assert!(err.contains("exactly 5 canonical fixture kinds"));
}

#[test]
fn runtime_v2_wellbeing_diagnostic_rejects_duplicate_fixture_kind_cardinality() {
    let mut packet = wellbeing_diagnostic_packet().expect("packet");
    let duplicate = packet
        .fixtures
        .iter()
        .find(|fixture| fixture.fixture_kind == "privacy-restricted")
        .expect("privacy-restricted fixture")
        .clone();
    packet.fixtures.push(duplicate);

    let err = validate_wellbeing_diagnostic_packet(&packet)
        .expect_err("duplicate fixture kind should fail cardinality validation")
        .to_string();

    assert!(err.contains("exactly 5 canonical fixture kinds"));
}

#[test]
fn runtime_v2_wellbeing_diagnostic_rejects_private_detail_leak_in_operator_view() {
    let mut packet = wellbeing_diagnostic_packet().expect("packet");
    let fixture = packet
        .fixtures
        .iter_mut()
        .find(|fixture| fixture.fixture_kind == "privacy-restricted")
        .expect("privacy-restricted fixture");
    let operator_view = fixture
        .views
        .iter_mut()
        .find(|view| view.view_kind == "operator")
        .expect("operator view");
    operator_view.visible_private_detail_refs =
        vec!["private-detail:leaked-private-note".to_string()];

    let err = validate_wellbeing_diagnostic_packet(&packet)
        .expect_err("operator private detail leak should fail")
        .to_string();

    assert!(err.contains("must not expose private diagnostic details"));
}

#[test]
fn runtime_v2_wellbeing_diagnostic_rejects_public_raw_evidence_exposure() {
    let mut packet = wellbeing_diagnostic_packet().expect("packet");
    let fixture = packet
        .fixtures
        .iter_mut()
        .find(|fixture| fixture.fixture_kind == "privacy-restricted")
        .expect("privacy-restricted fixture");
    let public_view = fixture
        .views
        .iter_mut()
        .find(|view| view.view_kind == "public_redacted")
        .expect("public view");
    let leaked_ref = fixture.dimension_signals[0].evidence_refs[0].clone();
    public_view.visible_evidence_refs = vec![leaked_ref];

    let err = validate_wellbeing_diagnostic_packet(&packet)
        .expect_err("public raw evidence exposure should fail")
        .to_string();

    assert!(err.contains("raw evidence refs"));
}

#[test]
fn runtime_v2_wellbeing_diagnostic_requires_privacy_restricted_private_details() {
    let mut packet = wellbeing_diagnostic_packet().expect("packet");
    let fixture = packet
        .fixtures
        .iter_mut()
        .find(|fixture| fixture.fixture_kind == "privacy-restricted")
        .expect("privacy-restricted fixture");
    for signal in &mut fixture.dimension_signals {
        signal.private_detail_refs.clear();
    }

    let err = validate_wellbeing_diagnostic_packet(&packet)
        .expect_err("privacy-restricted fixture must keep private details")
        .to_string();

    assert!(err.contains("private diagnostic detail"));
}

#[test]
fn runtime_v2_wellbeing_diagnostic_rejects_unknown_upstream_refs() {
    let mut packet = wellbeing_diagnostic_packet().expect("packet");
    packet.fixtures[0].dimension_signals[0].evidence_refs =
        vec!["metric:unknown-wellbeing-metric".to_string()];

    let err = validate_wellbeing_diagnostic_packet(&packet)
        .expect_err("unknown upstream refs should fail")
        .to_string();

    assert!(err.contains("known WP-06 metrics"));
}

#[test]
fn runtime_v2_wellbeing_diagnostic_rejects_invalid_dimension_evidence_field_refs() {
    let mut packet = wellbeing_diagnostic_packet().expect("packet");
    packet.dimensions[0].evidence_field_refs = vec!["moral_trace.".to_string()];

    let malformed_err = validate_wellbeing_diagnostic_packet(&packet)
        .expect_err("malformed field path should fail")
        .to_string();
    assert!(malformed_err.contains("concrete upstream field path"));

    let mut packet = wellbeing_diagnostic_packet().expect("packet");
    packet.dimensions[0].evidence_field_refs = vec!["moral_trace.unknown_branch".to_string()];

    let unknown_err = validate_wellbeing_diagnostic_packet(&packet)
        .expect_err("unknown field path should fail")
        .to_string();
    assert!(unknown_err.contains("known WP-04 through WP-08 field paths"));
}
