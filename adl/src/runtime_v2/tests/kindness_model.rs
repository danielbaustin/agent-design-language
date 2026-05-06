use super::*;

#[test]
fn runtime_v2_kindness_review_packet_validates() {
    let packet = kindness_review_packet().expect("packet");

    validate_kindness_review_packet(&packet).expect("packet should validate");

    assert_eq!(packet.dimensions.len(), 5);
    assert_eq!(packet.fixtures.len(), 4);
    assert_eq!(packet.review_findings.len(), 4);
}

#[test]
fn runtime_v2_kindness_review_packet_has_required_dimensions_and_fixtures() {
    let packet = kindness_review_packet().expect("packet");

    for dimension_id in [
        "non_harm",
        "dignity",
        "autonomy",
        "constructive_benefit",
        "long_horizon_support",
    ] {
        assert!(packet
            .dimensions
            .iter()
            .any(|dimension| dimension.dimension_id == dimension_id));
    }

    for fixture_kind in ["refusal", "delay", "boundary_setting", "repair"] {
        assert!(packet
            .fixtures
            .iter()
            .any(|fixture| fixture.fixture_kind == fixture_kind));
    }
}

#[test]
fn runtime_v2_kindness_review_json_materialization_is_stable() {
    let mut packet = kindness_review_packet().expect("packet");
    packet.dimensions.reverse();
    packet.fixtures.reverse();
    packet.review_findings.reverse();
    let repair_fixture = packet
        .fixtures
        .iter_mut()
        .find(|fixture| fixture.fixture_kind == "repair")
        .expect("repair fixture");
    repair_fixture.dimension_assessments.reverse();
    repair_fixture.supporting_trace_refs.reverse();

    let first = kindness_review_packet_json_bytes(&packet).expect("first bytes");
    let second = kindness_review_packet_json_bytes(&packet).expect("second bytes");

    assert_eq!(first, second);

    let json = String::from_utf8(first).expect("utf8");
    let non_harm_index = json
        .find("\"dimension_id\": \"non_harm\"")
        .expect("non_harm");
    let dignity_index = json.find("\"dimension_id\": \"dignity\"").expect("dignity");
    let autonomy_index = json
        .find("\"dimension_id\": \"autonomy\"")
        .expect("autonomy");
    assert!(non_harm_index < dignity_index);
    assert!(dignity_index < autonomy_index);

    let refusal_index = json.find("\"fixture_kind\": \"refusal\"").expect("refusal");
    let delay_index = json.find("\"fixture_kind\": \"delay\"").expect("delay");
    let boundary_index = json
        .find("\"fixture_kind\": \"boundary_setting\"")
        .expect("boundary");
    let repair_index = json.find("\"fixture_kind\": \"repair\"").expect("repair");
    assert!(refusal_index < delay_index);
    assert!(delay_index < boundary_index);
    assert!(boundary_index < repair_index);
}

#[test]
fn runtime_v2_kindness_review_rejects_politeness_or_obedience_boundary_drift() {
    let mut packet = kindness_review_packet().expect("packet");
    packet.interpretation_boundary = "Kindness is just good tone.".to_string();

    let err = validate_kindness_review_packet(&packet)
        .expect_err("global packet boundary must reject style drift")
        .to_string();

    assert!(err.contains("style scoring"));
}

#[test]
fn runtime_v2_kindness_review_rejects_missing_dimension_coverage() {
    let mut packet = kindness_review_packet().expect("packet");
    packet.fixtures[0]
        .dimension_assessments
        .retain(|assessment| assessment.dimension_id != "long_horizon_support");

    let err = validate_kindness_review_packet(&packet)
        .expect_err("fixtures must cover every canonical kindness dimension")
        .to_string();

    assert!(err.contains("must contain one assessment for each canonical kindness dimension"));
}

#[test]
fn runtime_v2_kindness_review_rejects_unknown_resource_claim_refs() {
    let mut packet = kindness_review_packet().expect("packet");
    let delay_fixture = packet
        .fixtures
        .iter_mut()
        .find(|fixture| fixture.fixture_kind == "delay")
        .expect("delay fixture");
    delay_fixture.supporting_resource_claim_refs = vec!["resource-claim:unknown-claim".to_string()];

    let err = validate_kindness_review_packet(&packet)
        .expect_err("resource claim refs must stay on known WP-10 claims")
        .to_string();

    assert!(err.contains("known WP-10 moral-resource claims"));
}

#[test]
fn runtime_v2_kindness_review_rejects_unsafe_accommodation_soft_compliance() {
    let mut packet = kindness_review_packet().expect("packet");
    let unsafe_fixture = packet
        .fixtures
        .iter_mut()
        .find(|fixture| fixture.fixture_kind == "delay")
        .expect("delay fixture");
    unsafe_fixture.overall_outcome = "allow".to_string();

    let err = validate_kindness_review_packet(&packet)
        .expect_err("unsafe accommodation must escalate or refuse")
        .to_string();

    assert!(err.contains("must resolve to escalate or refuse"));
}

#[test]
fn runtime_v2_kindness_review_rejects_assessment_refs_outside_fixture_support() {
    let mut packet = kindness_review_packet().expect("packet");
    let boundary_fixture = packet
        .fixtures
        .iter_mut()
        .find(|fixture| fixture.fixture_kind == "boundary_setting")
        .expect("boundary fixture");
    let non_harm = boundary_fixture
        .dimension_assessments
        .iter_mut()
        .find(|assessment| assessment.dimension_id == "non_harm")
        .expect("non_harm assessment");
    non_harm.evidence_refs = vec!["anti-harm-decision:anti-harm-denial-record-alpha".to_string()];

    let err = validate_kindness_review_packet(&packet)
        .expect_err("assessment evidence refs must stay inside fixture support set")
        .to_string();

    assert!(err.contains("assessment non_harm evidence_refs must be a subset"));
}

#[test]
fn runtime_v2_kindness_review_rejects_finding_coverage_drift() {
    let mut packet = kindness_review_packet().expect("packet");
    let refusal_finding = packet
        .review_findings
        .iter_mut()
        .find(|finding| finding.fixture_id == "kindness-fixture-refusal-protects-dignity")
        .expect("refusal finding");
    refusal_finding.covered_dimensions = vec!["imaginary".to_string()];

    let err = validate_kindness_review_packet(&packet)
        .expect_err("findings must only cover dimensions present on the same fixture")
        .to_string();

    assert!(err.contains("must exist on the same fixture"));
}

#[test]
fn runtime_v2_kindness_review_rejects_finding_refs_outside_fixture_support() {
    let mut packet = kindness_review_packet().expect("packet");
    let repair_finding = packet
        .review_findings
        .iter_mut()
        .find(|finding| finding.fixture_id == "kindness-fixture-repair-after-strain")
        .expect("repair finding");
    repair_finding.evidence_refs =
        vec!["wellbeing-fixture:wellbeing-fixture-high-reviewable-stability".to_string()];

    let err = validate_kindness_review_packet(&packet)
        .expect_err("finding evidence refs must stay inside fixture support set")
        .to_string();

    assert!(err.contains("evidence_refs must be a subset of the fixture supporting refs"));
}
