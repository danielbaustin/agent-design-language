use super::*;

#[test]
fn runtime_v2_cultivating_intelligence_packet_validates() {
    let packet = cultivating_intelligence_review_packet().expect("packet");
    validate_cultivating_intelligence_review_packet(&packet).expect("packet");
    assert_eq!(packet.dimensions.len(), 5);
    assert_eq!(packet.fixtures.len(), 3);
    assert_eq!(packet.boundary_refs.len(), 2);
}

#[test]
fn runtime_v2_cultivating_intelligence_json_materialization_is_stable() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.dimensions.reverse();
    packet.review_criteria.reverse();
    packet.boundary_refs.reverse();
    packet.fixtures.reverse();
    let first = cultivating_intelligence_review_packet_json_bytes(&packet).expect("first");
    let second = cultivating_intelligence_review_packet_json_bytes(&packet).expect("second");
    assert_eq!(first, second);
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_boundary_drift() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.interpretation_boundary = "This proves final intelligence.".to_string();
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("boundary")
        .to_string();
    assert!(err.contains("v0.91.1 intelligence absorption"));
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_duplicate_fixture_kinds() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.fixtures[1].fixture_kind = packet.fixtures[0].fixture_kind.clone();
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("fixture kind")
        .to_string();
    assert!(err.contains("canonical cultivation fixture kinds"));
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_unknown_boundary_doc_path() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.boundary_refs[0].doc_path =
        "docs/milestones/v0.91/features/CULTIVATING_INTELLIGENCE.md".to_string();
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("boundary path")
        .to_string();
    assert!(err.contains("must cite doc_path"));
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_absorbed_v0911_claims() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.boundary_refs[1].deferred_work =
        "This issue implements intelligence architecture and Theory of Mind now.".to_string();
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("absorbed claims")
        .to_string();
    assert!(err.contains("must explicitly defer adjacent work"));
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_assessment_refs_outside_fixture_support() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.fixtures[0].dimension_assessments[0].evidence_refs =
        vec!["resource-claim:resource-claim-conflict-care".to_string()];
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("assessment refs")
        .to_string();
    assert!(err.contains("must be a subset of the fixture supporting refs"));
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_finding_dimension_drift() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.review_findings[0].covered_dimension_ids = vec!["imaginary".to_string()];
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("finding dimensions")
        .to_string();
    assert!(err.contains("must exist on the same fixture"));
}
