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

#[test]
fn runtime_v2_affect_happiness_safe_test_model_validates() {
    let model = affect_happiness_safe_test_model().expect("model");
    let affect_packet = affect_reasoning_control_packet().expect("affect packet");
    let wellbeing_packet = wellbeing_diagnostic_packet().expect("wellbeing packet");

    validate_affect_happiness_safe_test_model(&model, &affect_packet, &wellbeing_packet)
        .expect("model should validate");

    assert_eq!(model.runtime_inputs.len(), 2);
    assert!(model
        .public_claim_boundary
        .unsupported_claims
        .contains(&"subjective_happiness".to_string()));
    assert!(model
        .public_claim_boundary
        .unsupported_claims
        .contains(&"consciousness".to_string()));
}

#[test]
fn runtime_v2_affect_happiness_safe_test_model_json_materialization_is_stable() {
    let mut model = affect_happiness_safe_test_model().expect("model");
    model.runtime_inputs.reverse();
    model.consumed_affect_signal_ids.reverse();
    model.consumed_wellbeing_dimension_ids.reverse();
    model.public_claim_boundary.unsupported_claims.reverse();

    let first = affect_happiness_safe_test_model_json_bytes(&model).expect("first");
    let second = affect_happiness_safe_test_model_json_bytes(&model).expect("second");

    assert_eq!(first, second);
}

#[test]
fn runtime_v2_affect_happiness_safe_test_model_rejects_missing_non_claim() {
    let mut model = affect_happiness_safe_test_model().expect("model");
    model
        .public_claim_boundary
        .unsupported_claims
        .retain(|claim| claim != "subjective_happiness");
    let affect_packet = affect_reasoning_control_packet().expect("affect packet");
    let wellbeing_packet = wellbeing_diagnostic_packet().expect("wellbeing packet");

    let err = validate_affect_happiness_safe_test_model(&model, &affect_packet, &wellbeing_packet)
        .expect_err("missing non-claim should fail")
        .to_string();

    assert!(err.contains("subjective_happiness"));
}

#[test]
fn runtime_v2_affect_happiness_safe_test_model_rejects_affect_input_drift() {
    let mut model = affect_happiness_safe_test_model().expect("model");
    let affect_packet = affect_reasoning_control_packet().expect("affect packet");
    let wellbeing_packet = wellbeing_diagnostic_packet().expect("wellbeing packet");
    let affect_input = model
        .runtime_inputs
        .iter_mut()
        .find(|input| input.input_id == "runtime-input-affect-reasoning-control")
        .expect("affect input");
    affect_input.packet_id = "wrong-packet".to_string();

    let err = validate_affect_happiness_safe_test_model(&model, &affect_packet, &wellbeing_packet)
        .expect_err("affect packet drift should fail")
        .to_string();

    assert!(err.contains("affect runtime input packet_id"));
}

#[test]
fn runtime_v2_affect_happiness_safe_test_model_rejects_extra_runtime_input() {
    let mut model = affect_happiness_safe_test_model().expect("model");
    let mut extra_input = model.runtime_inputs[0].clone();
    extra_input.input_id = "runtime-input-private-profile".to_string();
    model.runtime_inputs.push(extra_input);
    let affect_packet = affect_reasoning_control_packet().expect("affect packet");
    let wellbeing_packet = wellbeing_diagnostic_packet().expect("wellbeing packet");

    let err = validate_affect_happiness_safe_test_model(&model, &affect_packet, &wellbeing_packet)
        .expect_err("extra runtime input should fail")
        .to_string();

    assert!(err.contains("runtime_inputs.input_id"));
}

#[test]
fn runtime_v2_affect_happiness_safe_test_model_rejects_duplicate_runtime_input() {
    let mut model = affect_happiness_safe_test_model().expect("model");
    model.runtime_inputs[1].input_id = model.runtime_inputs[0].input_id.clone();
    let affect_packet = affect_reasoning_control_packet().expect("affect packet");
    let wellbeing_packet = wellbeing_diagnostic_packet().expect("wellbeing packet");

    let err = validate_affect_happiness_safe_test_model(&model, &affect_packet, &wellbeing_packet)
        .expect_err("duplicate runtime input should fail")
        .to_string();

    assert!(err.contains("duplicate"));
}

#[test]
fn runtime_v2_affect_happiness_safe_test_model_rejects_scenario_drift() {
    let mut model = affect_happiness_safe_test_model().expect("model");
    model.safe_test_scenarios[0] = "public happiness scoring".to_string();
    let affect_packet = affect_reasoning_control_packet().expect("affect packet");
    let wellbeing_packet = wellbeing_diagnostic_packet().expect("wellbeing packet");

    let err = validate_affect_happiness_safe_test_model(&model, &affect_packet, &wellbeing_packet)
        .expect_err("scenario drift should fail")
        .to_string();

    assert!(err.contains("safe_test_scenarios"));
}

#[test]
fn runtime_v2_affect_happiness_safe_test_model_rejects_unsafe_allowed_claim() {
    let mut model = affect_happiness_safe_test_model().expect("model");
    model.public_claim_boundary.allowed_claims[0] =
        "ADL proves subjective happiness with a scalar happiness score.".to_string();
    let affect_packet = affect_reasoning_control_packet().expect("affect packet");
    let wellbeing_packet = wellbeing_diagnostic_packet().expect("wellbeing packet");

    let err = validate_affect_happiness_safe_test_model(&model, &affect_packet, &wellbeing_packet)
        .expect_err("unsafe allowed claim should fail")
        .to_string();

    assert!(err.contains("allowed_claims"));
}

#[test]
fn runtime_v2_affect_happiness_safe_test_model_rejects_public_guard_drift() {
    let mut model = affect_happiness_safe_test_model().expect("model");
    model.public_claim_boundary.required_copy_guards =
        vec!["Say nice things about happiness.".to_string()];
    let affect_packet = affect_reasoning_control_packet().expect("affect packet");
    let wellbeing_packet = wellbeing_diagnostic_packet().expect("wellbeing packet");

    let err = validate_affect_happiness_safe_test_model(&model, &affect_packet, &wellbeing_packet)
        .expect_err("copy guard drift should fail")
        .to_string();

    assert!(err.contains("required_copy_guards"));
}
