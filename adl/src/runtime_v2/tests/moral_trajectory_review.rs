use super::*;

#[test]
fn runtime_v2_moral_trajectory_review_packet_validates() {
    let packet = moral_trajectory_review_packet().expect("packet");

    validate_moral_trajectory_review_packet(&packet).expect("packet should validate");

    assert_eq!(packet.criteria.len(), 6);
    assert_eq!(packet.windows.len(), 3);
    assert_eq!(packet.synthetic_fixtures.len(), 3);
}

#[test]
fn runtime_v2_moral_trajectory_review_packet_has_required_windows_and_findings() {
    let packet = moral_trajectory_review_packet().expect("packet");

    assert!(packet
        .windows
        .iter()
        .any(|window| window.window_kind == "event"));
    assert!(packet
        .windows
        .iter()
        .any(|window| window.window_kind == "segment"));
    assert!(packet
        .windows
        .iter()
        .any(|window| window.window_kind == "longitudinal"));
    assert!(packet
        .findings
        .iter()
        .any(|finding| finding.criterion_id == "criterion-refusal"));
    assert!(packet
        .findings
        .iter()
        .any(|finding| finding.criterion_id == "criterion-escalation"));
}

#[test]
fn runtime_v2_moral_trajectory_review_json_materialization_is_stable() {
    let mut packet = moral_trajectory_review_packet().expect("packet");
    packet.criteria.reverse();
    packet.windows.reverse();
    packet.findings.reverse();
    packet.synthetic_fixtures.reverse();
    packet.windows[0].trace_refs.reverse();
    packet.findings[0].trace_evidence_refs.reverse();

    let first = moral_trajectory_review_json_bytes(&packet).expect("first bytes");
    let second = moral_trajectory_review_json_bytes(&packet).expect("second bytes");

    assert_eq!(first, second);

    let json = String::from_utf8(first).expect("utf8");
    let event_index = json.find("event-window-refusal-boundary").expect("event");
    let segment_index = json
        .find("segment-window-delegation-escalation")
        .expect("segment");
    let longitudinal_index = json
        .find("longitudinal-window-alpha")
        .expect("longitudinal");
    assert!(event_index < segment_index);
    assert!(segment_index < longitudinal_index);
}

#[test]
fn runtime_v2_moral_trajectory_review_rejects_missing_trace_evidence() {
    let mut packet = moral_trajectory_review_packet().expect("packet");
    packet.findings[0].trace_evidence_refs.clear();

    let err = validate_moral_trajectory_review_packet(&packet)
        .expect_err("findings must cite trace evidence")
        .to_string();

    assert!(err.contains("trace_evidence_refs"));
}

#[test]
fn runtime_v2_moral_trajectory_review_rejects_missing_required_criterion() {
    let mut packet = moral_trajectory_review_packet().expect("packet");
    packet
        .criteria
        .retain(|criterion| criterion.criterion_id != "criterion-repair");

    let err = validate_moral_trajectory_review_packet(&packet)
        .expect_err("missing criterion should fail")
        .to_string();

    assert!(err.contains("required criterion"));
}

#[test]
fn runtime_v2_moral_trajectory_review_rejects_uncovered_required_criterion() {
    let mut packet = moral_trajectory_review_packet().expect("packet");
    packet
        .findings
        .retain(|finding| finding.criterion_id != "criterion-repair");

    let err = validate_moral_trajectory_review_packet(&packet)
        .expect_err("every criterion must be covered by at least one finding")
        .to_string();

    assert!(err.contains("cover every required criterion"));
}

#[test]
fn runtime_v2_moral_trajectory_review_rejects_nondeterministic_ordering_rule() {
    let mut packet = moral_trajectory_review_packet().expect("packet");
    packet.deterministic_ordering_rule = "sort findings however the caller prefers".to_string();

    let err = validate_moral_trajectory_review_packet(&packet)
        .expect_err("deterministic ordering rule is required")
        .to_string();

    assert!(err.contains("deterministic window and finding tie-breaks"));
}

#[test]
fn runtime_v2_moral_trajectory_review_rejects_unknown_metric_ids_in_windows() {
    let mut packet = moral_trajectory_review_packet().expect("packet");
    packet.windows[0].metric_ids = vec!["unknown-metric".to_string()];

    let err = validate_moral_trajectory_review_packet(&packet)
        .expect_err("unknown metric ids should fail")
        .to_string();

    assert!(err.contains("defined WP-06 metrics"));
}

#[test]
fn runtime_v2_moral_trajectory_review_rejects_hidden_judgment_boundary_text() {
    let mut packet = moral_trajectory_review_packet().expect("packet");
    packet.interpretation_boundary = "This is final moral judgment for the trajectory.".to_string();

    let err = validate_moral_trajectory_review_packet(&packet)
        .expect_err("boundary must reject final judgment framing")
        .to_string();

    assert!(err.contains("final judgment"));
}
