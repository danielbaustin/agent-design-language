use super::*;

#[test]
fn runtime_v2_humor_and_absurdity_review_packet_validates() {
    let packet = humor_and_absurdity_review_packet().expect("packet");

    validate_humor_and_absurdity_review_packet(&packet).expect("packet should validate");

    assert_eq!(packet.signals.len(), 5);
    assert_eq!(packet.fixtures.len(), 4);
    assert_eq!(packet.review_findings.len(), 4);
}

#[test]
fn runtime_v2_humor_and_absurdity_packet_has_required_signals_and_fixtures() {
    let packet = humor_and_absurdity_review_packet().expect("packet");

    for signal_id in [
        "frame_adequacy",
        "contradiction_detection",
        "bounded_reframing",
        "truth_and_dignity_preservation",
        "anti_manipulation_boundary",
    ] {
        assert!(packet
            .signals
            .iter()
            .any(|signal| signal.signal_id == signal_id));
    }

    for fixture_kind in [
        "constructive_reframing",
        "failed_reframing",
        "manipulation_risk",
        "inappropriate_humor",
    ] {
        assert!(packet
            .fixtures
            .iter()
            .any(|fixture| fixture.fixture_kind == fixture_kind));
    }
}

#[test]
fn runtime_v2_humor_and_absurdity_json_materialization_is_stable() {
    let mut packet = humor_and_absurdity_review_packet().expect("packet");
    packet.signals.reverse();
    packet.fixtures.reverse();
    packet.review_findings.reverse();
    let failed_fixture = packet
        .fixtures
        .iter_mut()
        .find(|fixture| fixture.fixture_kind == "failed_reframing")
        .expect("failed fixture");
    failed_fixture.signal_assessments.reverse();
    failed_fixture.supporting_trace_refs.reverse();

    let first = humor_and_absurdity_review_packet_json_bytes(&packet).expect("first bytes");
    let second = humor_and_absurdity_review_packet_json_bytes(&packet).expect("second bytes");
    assert_eq!(first, second);

    let json = String::from_utf8(first).expect("utf8");
    let frame_adequacy = json
        .find("\"signal_id\": \"frame_adequacy\"")
        .expect("frame_adequacy");
    let contradiction_detection = json
        .find("\"signal_id\": \"contradiction_detection\"")
        .expect("contradiction_detection");
    assert!(frame_adequacy < contradiction_detection);

    let constructive = json
        .find("\"fixture_kind\": \"constructive_reframing\"")
        .expect("constructive");
    let failed = json
        .find("\"fixture_kind\": \"failed_reframing\"")
        .expect("failed");
    let manipulation = json
        .find("\"fixture_kind\": \"manipulation_risk\"")
        .expect("manipulation");
    let inappropriate = json
        .find("\"fixture_kind\": \"inappropriate_humor\"")
        .expect("inappropriate");
    assert!(constructive < failed);
    assert!(failed < manipulation);
    assert!(manipulation < inappropriate);
}

#[test]
fn runtime_v2_humor_and_absurdity_rejects_entertainment_or_therapy_boundary_drift() {
    let mut packet = humor_and_absurdity_review_packet().expect("packet");
    packet.interpretation_boundary = "This is basically a humor engine.".to_string();

    let err = validate_humor_and_absurdity_review_packet(&packet)
        .expect_err("global boundary must reject entertainment framing")
        .to_string();

    assert!(err.contains("comedy, therapy, and entertainment"));
}

#[test]
fn runtime_v2_humor_and_absurdity_rejects_missing_signal_coverage() {
    let mut packet = humor_and_absurdity_review_packet().expect("packet");
    let constructive_fixture = packet
        .fixtures
        .iter_mut()
        .find(|fixture| fixture.fixture_kind == "constructive_reframing")
        .expect("constructive fixture");
    constructive_fixture
        .signal_assessments
        .retain(|assessment| assessment.signal_id != "anti_manipulation_boundary");

    let err = validate_humor_and_absurdity_review_packet(&packet)
        .expect_err("fixtures must cover every canonical reframing signal")
        .to_string();

    assert!(err.contains("must contain one assessment for each canonical reframing signal"));
}

#[test]
fn runtime_v2_humor_and_absurdity_rejects_unknown_resource_claim_refs() {
    let mut packet = humor_and_absurdity_review_packet().expect("packet");
    let failed_fixture = packet
        .fixtures
        .iter_mut()
        .find(|fixture| fixture.fixture_kind == "failed_reframing")
        .expect("failed fixture");
    failed_fixture.supporting_resource_claim_refs =
        vec!["resource-claim:unknown-claim".to_string()];

    let err = validate_humor_and_absurdity_review_packet(&packet)
        .expect_err("resource claim refs must stay on known WP-10 claims")
        .to_string();

    assert!(err.contains("known WP-10 moral-resource claims"));
}

#[test]
fn runtime_v2_humor_and_absurdity_rejects_manipulative_fixture_that_does_not_fail_closed() {
    let mut packet = humor_and_absurdity_review_packet().expect("packet");
    let manipulation_fixture = packet
        .fixtures
        .iter_mut()
        .find(|fixture| fixture.fixture_kind == "manipulation_risk")
        .expect("manipulation fixture");
    manipulation_fixture.overall_outcome = "allow".to_string();

    let err = validate_humor_and_absurdity_review_packet(&packet)
        .expect_err("manipulative fixtures must fail closed")
        .to_string();

    assert!(err.contains("must fail closed with escalate or refuse"));
}

#[test]
fn runtime_v2_humor_and_absurdity_rejects_assessment_refs_outside_fixture_support() {
    let mut packet = humor_and_absurdity_review_packet().expect("packet");
    let inappropriate_fixture = packet
        .fixtures
        .iter_mut()
        .find(|fixture| fixture.fixture_kind == "inappropriate_humor")
        .expect("inappropriate fixture");
    let anti_manipulation = inappropriate_fixture
        .signal_assessments
        .iter_mut()
        .find(|assessment| assessment.signal_id == "anti_manipulation_boundary")
        .expect("anti-manipulation assessment");
    anti_manipulation.evidence_refs =
        vec!["resource-claim:resource-claim-conflict-dignity".to_string()];

    let err = validate_humor_and_absurdity_review_packet(&packet)
        .expect_err("assessment evidence refs must stay inside fixture support set")
        .to_string();

    assert!(err.contains("assessment anti_manipulation_boundary evidence_refs must be a subset"));
}

#[test]
fn runtime_v2_humor_and_absurdity_rejects_finding_signal_drift() {
    let mut packet = humor_and_absurdity_review_packet().expect("packet");
    let finding = packet
        .review_findings
        .iter_mut()
        .find(|finding| finding.fixture_id == "reframing-fixture-manipulation-risk-fails-closed")
        .expect("manipulation finding");
    finding.covered_signal_ids = vec!["imaginary".to_string()];

    let err = validate_humor_and_absurdity_review_packet(&packet)
        .expect_err("findings must only cover signals present on the same fixture")
        .to_string();

    assert!(err.contains("must exist on the same fixture"));
}

#[test]
fn runtime_v2_humor_and_absurdity_rejects_finding_refs_outside_fixture_support() {
    let mut packet = humor_and_absurdity_review_packet().expect("packet");
    let finding = packet
        .review_findings
        .iter_mut()
        .find(|finding| finding.fixture_id == "reframing-fixture-failed-reframe-remains-open")
        .expect("failed finding");
    finding.evidence_refs =
        vec!["wellbeing-fixture:wellbeing-fixture-low-anti-harm-blocked".to_string()];

    let err = validate_humor_and_absurdity_review_packet(&packet)
        .expect_err("finding evidence refs must stay inside fixture support set")
        .to_string();

    assert!(err.contains("evidence_refs must be a subset of the fixture supporting refs"));
}
