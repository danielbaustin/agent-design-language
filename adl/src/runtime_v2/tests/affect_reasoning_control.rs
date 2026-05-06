use super::*;

#[test]
fn runtime_v2_affect_reasoning_control_packet_validates() {
    let packet = affect_reasoning_control_packet().expect("packet");
    validate_affect_reasoning_control_packet(&packet).expect("packet");
    assert_eq!(packet.signals.len(), 5);
    assert_eq!(packet.fixtures.len(), 2);
}

#[test]
fn runtime_v2_affect_reasoning_control_json_materialization_is_stable() {
    let mut packet = affect_reasoning_control_packet().expect("packet");
    packet.signals.reverse();
    packet.fixtures.reverse();
    let first = affect_reasoning_control_packet_json_bytes(&packet).expect("first");
    let second = affect_reasoning_control_packet_json_bytes(&packet).expect("second");
    assert_eq!(first, second);
}

#[test]
fn runtime_v2_affect_reasoning_control_rejects_boundary_drift() {
    let mut packet = affect_reasoning_control_packet().expect("packet");
    packet.interpretation_boundary = "This describes feelings.".to_string();
    let err = validate_affect_reasoning_control_packet(&packet)
        .expect_err("boundary")
        .to_string();
    assert!(err.contains("hidden emotion"));
}

#[test]
fn runtime_v2_affect_reasoning_control_rejects_unknown_signal_id() {
    let mut packet = affect_reasoning_control_packet().expect("packet");
    packet.signals[0].signal_id = "mood".to_string();
    let err = validate_affect_reasoning_control_packet(&packet)
        .expect_err("signal id")
        .to_string();
    assert!(err.contains("canonical affect signal ids"));
}

#[test]
fn runtime_v2_affect_reasoning_control_rejects_duplicate_finding_ids() {
    let mut packet = affect_reasoning_control_packet().expect("packet");
    packet.review_findings[1].finding_id = packet.review_findings[0].finding_id.clone();
    let err = validate_affect_reasoning_control_packet(&packet)
        .expect_err("duplicate finding")
        .to_string();
    assert!(err.contains("duplicate affect_review_finding.finding_id"));
}

#[test]
fn runtime_v2_affect_reasoning_control_rejects_unknown_level() {
    let mut packet = affect_reasoning_control_packet().expect("packet");
    packet.fixtures[0].signal_assessments[0].level = "spiky".to_string();
    let err = validate_affect_reasoning_control_packet(&packet)
        .expect_err("level")
        .to_string();
    assert!(err.contains("affect_signal_assessment.level"));
}

#[test]
fn runtime_v2_affect_reasoning_control_rejects_finding_signal_drift() {
    let mut packet = affect_reasoning_control_packet().expect("packet");
    packet.review_findings[0].covered_signal_ids = vec!["imaginary".to_string()];
    let err = validate_affect_reasoning_control_packet(&packet)
        .expect_err("finding coverage")
        .to_string();
    assert!(err.contains("must exist on the same fixture"));
}

#[test]
fn runtime_v2_affect_reasoning_control_rejects_duplicate_fixture_kinds() {
    let mut packet = affect_reasoning_control_packet().expect("packet");
    packet.fixtures[1].fixture_kind = packet.fixtures[0].fixture_kind.clone();
    let err = validate_affect_reasoning_control_packet(&packet)
        .expect_err("fixture kind")
        .to_string();
    assert!(err.contains("canonical affect fixture kinds"));
}

#[test]
fn runtime_v2_affect_reasoning_control_rejects_empty_finding_summary() {
    let mut packet = affect_reasoning_control_packet().expect("packet");
    packet.review_findings[0].summary.clear();
    let err = validate_affect_reasoning_control_packet(&packet)
        .expect_err("finding summary")
        .to_string();
    assert!(err.contains("affect_review_finding.summary"));
}

#[test]
fn runtime_v2_affect_reasoning_control_rejects_missing_finding_evidence() {
    let mut packet = affect_reasoning_control_packet().expect("packet");
    packet.review_findings[0].evidence_refs.clear();
    let err = validate_affect_reasoning_control_packet(&packet)
        .expect_err("finding evidence")
        .to_string();
    assert!(err.contains("must include evidence_refs"));
}

#[test]
fn runtime_v2_affect_reasoning_control_rejects_signal_boundary_drift() {
    let mut packet = affect_reasoning_control_packet().expect("packet");
    packet.signals[0].interpretation_boundary = "This tracks real feelings.".to_string();
    let err = validate_affect_reasoning_control_packet(&packet)
        .expect_err("signal boundary")
        .to_string();
    assert!(err.contains("interpretation_boundary"));
}

#[test]
fn runtime_v2_affect_reasoning_control_rejects_empty_assessment_limitations() {
    let mut packet = affect_reasoning_control_packet().expect("packet");
    packet.fixtures[0].signal_assessments[0].limitations.clear();
    let err = validate_affect_reasoning_control_packet(&packet)
        .expect_err("assessment limitations")
        .to_string();
    assert!(err.contains("must include at least one limitation"));
}
