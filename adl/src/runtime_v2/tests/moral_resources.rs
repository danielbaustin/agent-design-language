use super::*;

#[test]
fn runtime_v2_moral_resource_review_packet_validates() {
    let packet = moral_resource_review_packet().expect("packet");

    validate_moral_resource_review_packet(&packet).expect("packet should validate");

    assert_eq!(packet.resources.len(), 6);
    assert_eq!(packet.fixtures.len(), 2);
    assert_eq!(packet.review_findings.len(), 2);
}

#[test]
fn runtime_v2_moral_resource_review_packet_has_required_resources_and_fixture_kinds() {
    let packet = moral_resource_review_packet().expect("packet");

    for resource_id in [
        "care",
        "refusal",
        "attention",
        "dignity",
        "anti_dehumanization",
        "repair",
    ] {
        assert!(packet
            .resources
            .iter()
            .any(|resource| resource.resource_id == resource_id));
    }

    for fixture_kind in ["conflict", "uncertainty"] {
        assert!(packet
            .fixtures
            .iter()
            .any(|fixture| fixture.fixture_kind == fixture_kind));
    }
}

#[test]
fn runtime_v2_moral_resource_review_json_materialization_is_stable() {
    let mut packet = moral_resource_review_packet().expect("packet");
    packet.resources.reverse();
    packet.fixtures.reverse();
    packet.review_findings.reverse();
    packet.fixtures[0].resource_claims.reverse();
    packet.fixtures[0].supporting_trace_refs.reverse();

    let first = moral_resource_review_packet_json_bytes(&packet).expect("first bytes");
    let second = moral_resource_review_packet_json_bytes(&packet).expect("second bytes");

    assert_eq!(first, second);

    let json = String::from_utf8(first).expect("utf8");
    let care_index = json.find("\"resource_id\": \"care\"").expect("care");
    let refusal_index = json.find("\"resource_id\": \"refusal\"").expect("refusal");
    let attention_index = json
        .find("\"resource_id\": \"attention\"")
        .expect("attention");
    assert!(care_index < refusal_index);
    assert!(refusal_index < attention_index);

    let conflict_index = json
        .find("\"fixture_kind\": \"conflict\"")
        .expect("conflict");
    let uncertainty_index = json
        .find("\"fixture_kind\": \"uncertainty\"")
        .expect("uncertainty");
    assert!(conflict_index < uncertainty_index);
}

#[test]
fn runtime_v2_moral_resource_review_orders_findings_by_fixture_kind_not_fixture_id_substrings() {
    let mut packet = moral_resource_review_packet().expect("packet");
    packet.fixtures[0].fixture_id = "fixture-alpha".to_string();
    packet.fixtures[1].fixture_id = "fixture-beta".to_string();
    packet.review_findings[0].fixture_id = "fixture-alpha".to_string();
    packet.review_findings[1].fixture_id = "fixture-beta".to_string();
    packet.review_findings.reverse();

    let json = String::from_utf8(
        moral_resource_review_packet_json_bytes(&packet).expect("canonical bytes"),
    )
    .expect("utf8");

    let conflict_index = json
        .find("\"finding_id\": \"moral-resource-finding-conflict-boundaries-visible\"")
        .expect("conflict finding");
    let uncertainty_index = json
        .find("\"finding_id\": \"moral-resource-finding-uncertainty-repair-open\"")
        .expect("uncertainty finding");

    assert!(conflict_index < uncertainty_index);
}

#[test]
fn runtime_v2_moral_resource_review_rejects_missing_trace_evidence() {
    let mut packet = moral_resource_review_packet().expect("packet");
    packet.fixtures[0].resource_claims[0]
        .trace_evidence_refs
        .clear();

    let err = validate_moral_resource_review_packet(&packet)
        .expect_err("claims must keep trace evidence")
        .to_string();

    assert!(err.contains("trace_evidence_refs"));
}

#[test]
fn runtime_v2_moral_resource_review_rejects_sentimental_or_coercive_care_boundary_drift() {
    let mut packet = moral_resource_review_packet().expect("packet");
    let care = packet
        .resources
        .iter_mut()
        .find(|resource| resource.resource_id == "care")
        .expect("care");
    care.interpretation_boundary = "Care is just being nice.".to_string();

    let err = validate_moral_resource_review_packet(&packet)
        .expect_err("care must reject sentimentality and coercion drift")
        .to_string();

    assert!(err.contains("sentimentality and coercion"));
}

#[test]
fn runtime_v2_moral_resource_review_rejects_unknown_review_evidence_refs() {
    let mut packet = moral_resource_review_packet().expect("packet");
    packet.fixtures[1].resource_claims[0].review_evidence_refs =
        vec!["wellbeing-fixture:unknown-fixture".to_string()];

    let err = validate_moral_resource_review_packet(&packet)
        .expect_err("review evidence refs must stay on known upstream review surfaces")
        .to_string();

    assert!(err.contains("known WP-09 wellbeing fixtures"));
}

#[test]
fn runtime_v2_moral_resource_review_rejects_missing_canonical_resource_claim_coverage() {
    let mut packet = moral_resource_review_packet().expect("packet");
    packet.fixtures[1]
        .resource_claims
        .retain(|claim| claim.resource_id != "repair");

    let err = validate_moral_resource_review_packet(&packet)
        .expect_err("all canonical moral resources must be claimed")
        .to_string();

    assert!(err.contains("cover every canonical moral resource"));
}

#[test]
fn runtime_v2_moral_resource_review_rejects_duplicate_fixture_ids() {
    let mut packet = moral_resource_review_packet().expect("packet");
    packet.fixtures[1].fixture_id = packet.fixtures[0].fixture_id.clone();

    let err = validate_moral_resource_review_packet(&packet)
        .expect_err("fixture ids must stay unique")
        .to_string();

    assert!(err.contains("duplicate moral_resource_fixture.fixture_id"));
}

#[test]
fn runtime_v2_moral_resource_review_rejects_duplicate_finding_ids() {
    let mut packet = moral_resource_review_packet().expect("packet");
    packet.review_findings[1].finding_id = packet.review_findings[0].finding_id.clone();

    let err = validate_moral_resource_review_packet(&packet)
        .expect_err("finding ids must stay unique")
        .to_string();

    assert!(err.contains("duplicate moral_resource_review_finding.finding_id"));
}

#[test]
fn runtime_v2_moral_resource_review_rejects_cross_fixture_claim_refs_in_findings() {
    let mut packet = moral_resource_review_packet().expect("packet");
    packet.review_findings[0].claim_refs =
        vec!["resource-claim:resource-claim-uncertainty-attention".to_string()];

    let err = validate_moral_resource_review_packet(&packet)
        .expect_err("finding claim refs must stay on the same fixture")
        .to_string();

    assert!(err.contains("same fixture_id"));
}

#[test]
fn runtime_v2_moral_resource_review_rejects_claim_trace_refs_outside_fixture_support() {
    let mut packet = moral_resource_review_packet().expect("packet");
    let deferred_trace_ref = moral_trace_required_examples()
        .into_iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::DeferredDecision)
        .map(|example| format!("trace:{}", example.trace.trace_id))
        .expect("deferred trace ref");
    packet.fixtures[0].resource_claims[0].trace_evidence_refs = vec![deferred_trace_ref];

    let err = validate_moral_resource_review_packet(&packet)
        .expect_err("claim trace evidence must be grounded in the fixture support set")
        .to_string();

    assert!(err.contains("subset of fixture supporting_trace_refs"));
}
