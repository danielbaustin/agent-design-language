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

#[test]
fn runtime_v2_cultivating_intelligence_rejects_dimension_without_evidence_fields() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.dimensions[0].evidence_field_refs.clear();
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("dimension evidence fields")
        .to_string();
    assert!(err.contains("must include evidence_field_refs"));
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_duplicate_criterion_ids() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.review_criteria[1].criterion_id = packet.review_criteria[0].criterion_id.clone();
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("duplicate criterion")
        .to_string();
    assert!(err.contains("duplicate cultivation_review_criterion.criterion_id"));
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_boundary_refs_without_v0911_citation() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.boundary_refs[0].summary = "Capability boundaries matter.".to_string();
    packet.boundary_refs[0].deferred_work = "Deferred capability work remains pending.".to_string();
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("missing v0.91.1 citation")
        .to_string();
    assert!(err.contains("must explicitly cite v0.91.1"));
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_capability_boundary_without_capability_language() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.boundary_refs[0].summary =
        "v0.91.1 keeps adjacent work deferred without naming the governing boundary.".to_string();
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("capability language")
        .to_string();
    assert!(err.contains("must describe capability or aptitude deferral"));
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_invalid_fixture_outcome() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.fixtures[0].overall_outcome = "excellent".to_string();
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("overall outcome")
        .to_string();
    assert!(err.contains("must be one of improving, stable, strained, or unclear"));
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_invalid_supporting_trace_ref_prefix() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.fixtures[0].supporting_trace_refs = vec!["bogus:trace-1".to_string()];
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("trace ref prefix")
        .to_string();
    assert!(err.contains("must start with trace:"));
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_invalid_review_status() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.review_findings[0].review_status = "resolved".to_string();
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("review status")
        .to_string();
    assert!(err.contains("must be one of supported, guarded, or contested"));
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_missing_fixture_finding_coverage() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.review_findings.pop();
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("missing fixture finding")
        .to_string();
    assert!(err.contains("must contain exactly one finding per cultivation fixture"));
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_unknown_supporting_trace_ref() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.fixtures[0].supporting_trace_refs = vec!["trace:not-a-known-example".to_string()];
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("unknown trace ref")
        .to_string();
    assert!(err.contains("must come from known WP-04 trace examples"));
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_invalid_assessment_level() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.fixtures[0].dimension_assessments[0].cultivation_level = "rising".to_string();
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("assessment level")
        .to_string();
    assert!(err.contains("must be one of high, medium, or low"));
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_unknown_fixture_kind() {
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.fixtures[0].fixture_kind = "invented-kind".to_string();
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("fixture kind")
        .to_string();
    assert!(err.contains("cultivation fixture kinds must be canonical"));
}

#[test]
fn runtime_v2_cultivating_intelligence_rejects_intelligence_boundary_without_intelligence_language()
{
    let mut packet = cultivating_intelligence_review_packet().expect("packet");
    packet.boundary_refs[1].summary =
        "v0.91.1 keeps adjacent architecture work deferred.".to_string();
    packet.boundary_refs[1].deferred_work =
        "Deferred work remains not implemented until the later milestone.".to_string();
    let err = validate_cultivating_intelligence_review_packet(&packet)
        .expect_err("intelligence language")
        .to_string();
    assert!(err.contains("must describe intelligence/memory/ToM deferral"));
}
