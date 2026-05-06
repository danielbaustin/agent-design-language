use super::*;

#[test]
fn runtime_v2_moral_metrics_fixture_report_validates() {
    let report = moral_metric_fixture_report().expect("fixture report");

    validate_moral_metric_fixture_report(&report).expect("fixture report should validate");

    assert_eq!(report.definitions.len(), 3);
    assert_eq!(report.fixtures.len(), 1);
}

#[test]
fn runtime_v2_moral_metrics_fixture_report_has_expected_outputs() {
    let report = moral_metric_fixture_report().expect("fixture report");
    let fixture = report.fixtures.first().expect("fixture");

    let coverage = fixture
        .observations
        .iter()
        .find(|observation| observation.metric_id == "trace-review-path-coverage")
        .expect("coverage metric");
    assert_eq!(coverage.numerator, 4);
    assert_eq!(coverage.denominator, Some(4));

    let delegation = fixture
        .observations
        .iter()
        .find(|observation| observation.metric_id == "delegation-lineage-retention")
        .expect("delegation metric");
    assert_eq!(delegation.numerator, 1);
    assert_eq!(delegation.denominator, Some(1));

    let unresolved = fixture
        .observations
        .iter()
        .find(|observation| observation.metric_id == "unresolved-outcome-attention-count")
        .expect("unresolved metric");
    assert_eq!(unresolved.numerator, 4);
    assert_eq!(unresolved.denominator, None);
}

#[test]
fn runtime_v2_moral_metrics_json_materialization_is_stable() {
    let mut report = moral_metric_fixture_report().expect("fixture report");
    report.definitions.reverse();
    report.fixtures[0].input_trace_refs.reverse();
    report.fixtures[0].observations.reverse();
    report.fixtures[0].observations[0].evidence_refs = vec![
        "outcome_linkage.linked_outcomes.uncertainty_refs".to_string(),
        "outcome_linkage.linked_outcomes.outcome_status".to_string(),
    ];

    let first = moral_metric_fixture_report_json_bytes(&report).expect("first bytes");
    let second = moral_metric_fixture_report_json_bytes(&report).expect("second bytes");

    assert_eq!(first, second);

    let json = String::from_utf8(first).expect("utf8");
    let lineage_index = json
        .find("delegation-lineage-retention")
        .expect("lineage metric");
    let review_index = json
        .find("trace-review-path-coverage")
        .expect("review metric");
    let unresolved_index = json
        .find("unresolved-outcome-attention-count")
        .expect("unresolved metric");
    assert!(lineage_index < review_index);
    assert!(review_index < unresolved_index);
}

#[test]
fn runtime_v2_moral_metrics_rejects_scoreboard_framing() {
    let mut report = moral_metric_fixture_report().expect("fixture report");
    report.definitions[0].display_name = "Moral karma score".to_string();

    let err = validate_moral_metric_fixture_report(&report)
        .expect_err("scoreboard framing should fail")
        .to_string();

    assert!(err.contains("display_name"));
}

#[test]
fn runtime_v2_moral_metrics_rejects_scoreboard_framing_in_definition_boundary() {
    let mut report = moral_metric_fixture_report().expect("fixture report");
    report.definitions[0].interpretation_boundary =
        "Interpret as a happiness score for public reputation.".to_string();

    let err = validate_moral_metric_fixture_report(&report)
        .expect_err("definition boundary framing should fail")
        .to_string();

    assert!(err.contains("interpretation_boundary"));
}

#[test]
fn runtime_v2_moral_metrics_rejects_non_trace_derived_evidence_fields() {
    let mut report = moral_metric_fixture_report().expect("fixture report");
    report.definitions[0].evidence_field_refs = vec!["external_dashboard.value".to_string()];

    let err = validate_moral_metric_fixture_report(&report)
        .expect_err("non trace evidence should fail")
        .to_string();

    assert!(err.contains("explicit moral_trace or outcome_linkage evidence fields"));
}

#[test]
fn runtime_v2_moral_metrics_requires_non_scoreboard_boundary_language() {
    let mut report = moral_metric_fixture_report().expect("fixture report");
    report.interpretation_boundary = "These metrics are useful signals.".to_string();

    let err = validate_moral_metric_fixture_report(&report)
        .expect_err("boundary must reject scoreboard framing")
        .to_string();

    assert!(err.contains("scalar karma"));
}

#[test]
fn runtime_v2_moral_metrics_rejects_duplicate_fixture_observations() {
    let mut report = moral_metric_fixture_report().expect("fixture report");
    let duplicate = report.fixtures[0].observations[0].clone();
    report.fixtures[0].observations.push(duplicate);

    let err = validate_moral_metric_fixture_report(&report)
        .expect_err("duplicate metric observations should fail")
        .to_string();

    assert!(err.contains("duplicate metric_id"));
}

#[test]
fn runtime_v2_moral_metrics_rejects_weak_prefixed_refs_and_field_paths() {
    let mut report = moral_metric_fixture_report().expect("fixture report");
    report.fixtures[0].input_trace_refs[0] = "trace:bad:id".to_string();

    let trace_err = validate_moral_metric_fixture_report(&report)
        .expect_err("prefixed refs should reject nested separators")
        .to_string();
    assert!(trace_err.contains("must not contain path or nested prefix separators"));

    let mut report = moral_metric_fixture_report().expect("fixture report");
    report.definitions[0].evidence_field_refs = vec!["moral_trace.".to_string()];

    let field_err = validate_moral_metric_fixture_report(&report)
        .expect_err("field refs should require concrete field paths")
        .to_string();
    assert!(field_err.contains("concrete field path"));
}

#[test]
fn runtime_v2_moral_metrics_ties_ratio_and_count_observations_to_measurement_kind() {
    let mut report = moral_metric_fixture_report().expect("fixture report");
    let ratio = report.fixtures[0]
        .observations
        .iter_mut()
        .find(|observation| observation.metric_id == "trace-review-path-coverage")
        .expect("ratio observation");
    ratio.denominator = None;

    let ratio_err = validate_moral_metric_fixture_report(&report)
        .expect_err("ratio observations require denominators")
        .to_string();
    assert!(ratio_err.contains("must include a denominator"));

    let mut report = moral_metric_fixture_report().expect("fixture report");
    let count = report.fixtures[0]
        .observations
        .iter_mut()
        .find(|observation| observation.metric_id == "unresolved-outcome-attention-count")
        .expect("count observation");
    count.denominator = Some(4);

    let count_err = validate_moral_metric_fixture_report(&report)
        .expect_err("count observations should reject denominators")
        .to_string();
    assert!(count_err.contains("must not include a denominator"));
}
