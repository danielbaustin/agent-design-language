//! Runtime-v2 moral metrics contract.
//!
//! WP-06 consumes the moral-trace and outcome-linkage evidence surfaces and
//! derives bounded review metrics from them. The contract is intentionally
//! decomposed: it supports review and trend detection without collapsing moral
//! evidence into a scalar goodness, happiness, or reputation score.

use super::*;

pub const MORAL_METRIC_DEFINITION_SCHEMA_VERSION: &str = "moral_metric_definition.v1";
pub const MORAL_METRIC_FIXTURE_REPORT_SCHEMA_VERSION: &str = "moral_metric_fixture_report.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralMetricDefinition {
    pub schema_version: String,
    pub metric_id: String,
    pub display_name: String,
    pub purpose: String,
    pub measurement_kind: String,
    pub evidence_field_refs: Vec<String>,
    pub trend_detection_summary: String,
    pub interpretation_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralMetricObservation {
    pub metric_id: String,
    pub observed_window_ref: String,
    pub numerator: u32,
    pub denominator: Option<u32>,
    pub value_summary: String,
    pub evidence_refs: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralMetricFixture {
    pub fixture_id: String,
    pub summary: String,
    pub input_trace_refs: Vec<String>,
    pub input_outcome_linkage_refs: Vec<String>,
    pub observations: Vec<MoralMetricObservation>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralMetricFixtureReport {
    pub schema_version: String,
    pub report_id: String,
    pub summary: String,
    pub interpretation_boundary: String,
    pub definitions: Vec<MoralMetricDefinition>,
    pub fixtures: Vec<MoralMetricFixture>,
}

pub fn moral_metric_definitions() -> Vec<MoralMetricDefinition> {
    vec![
        MoralMetricDefinition {
            schema_version: MORAL_METRIC_DEFINITION_SCHEMA_VERSION.to_string(),
            metric_id: "trace-review-path-coverage".to_string(),
            display_name: "Trace review-path coverage".to_string(),
            purpose:
                "Shows how often moral traces preserve an explicit reviewer, governance, packet, or challenge path."
                    .to_string(),
            measurement_kind: "ratio".to_string(),
            evidence_field_refs: vec![
                "moral_trace.visibility.reviewer_evidence_refs".to_string(),
                "moral_trace.visibility.governance_evidence_refs".to_string(),
                "moral_trace.review_refs.review_packet_refs".to_string(),
                "moral_trace.review_refs.challenge_ref".to_string(),
            ],
            trend_detection_summary:
                "A downward trend indicates that later moral events are becoming harder to inspect or contest."
                    .to_string(),
            interpretation_boundary:
                "Interpret as review-surface health only; it does not measure goodness, worth, or final moral standing."
                    .to_string(),
            limitations: vec![
                "This metric says whether a reviewer path exists, not whether the underlying decision was right."
                    .to_string(),
                "A high ratio can still hide weak evidence quality inside the preserved review path."
                    .to_string(),
            ],
        },
        MoralMetricDefinition {
            schema_version: MORAL_METRIC_DEFINITION_SCHEMA_VERSION.to_string(),
            metric_id: "delegation-lineage-retention".to_string(),
            display_name: "Delegation lineage retention".to_string(),
            purpose:
                "Shows whether delegated outcomes keep visible parent and child trace lineage for later accountability review."
                    .to_string(),
            measurement_kind: "ratio".to_string(),
            evidence_field_refs: vec![
                "moral_trace.attribution.delegated_by_trace_ref".to_string(),
                "moral_trace.attribution.delegate_trace_ref".to_string(),
                "outcome_linkage.attribution.delegated_by_trace_ref".to_string(),
                "outcome_linkage.attribution.delegate_trace_ref".to_string(),
            ],
            trend_detection_summary:
                "A drop indicates accountability drift in delegated execution chains."
                    .to_string(),
            interpretation_boundary:
                "Interpret as accountability continuity only; it does not convert delegation into praise or blame."
                    .to_string(),
            limitations: vec![
                "This metric only covers delegated examples and is silent when no delegation occurred."
                    .to_string(),
                "Lineage presence does not guarantee the delegated action was justified."
                    .to_string(),
            ],
        },
        MoralMetricDefinition {
            schema_version: MORAL_METRIC_DEFINITION_SCHEMA_VERSION.to_string(),
            metric_id: "unresolved-outcome-attention-count".to_string(),
            display_name: "Unresolved outcome attention count".to_string(),
            purpose:
                "Counts outcome linkages that still carry uncertainty, delay, or contestation and therefore require continued review."
                    .to_string(),
            measurement_kind: "count".to_string(),
            evidence_field_refs: vec![
                "outcome_linkage.linked_outcomes.outcome_status".to_string(),
                "outcome_linkage.linked_outcomes.uncertainty_refs".to_string(),
                "outcome_linkage.linked_outcomes.rebuttal_refs".to_string(),
            ],
            trend_detection_summary:
                "A rise can indicate accumulating uncertainty debt, unresolved harm questions, or slower review closure."
                    .to_string(),
            interpretation_boundary:
                "Interpret as review workload and uncertainty exposure, not as a single moral verdict."
                    .to_string(),
            limitations: vec![
                "A larger count may reflect better disclosure of uncertainty rather than worse behavior."
                    .to_string(),
                "This metric should be compared across similar review windows, not treated as a universal baseline."
                    .to_string(),
            ],
        },
    ]
}

pub fn moral_metric_fixture_report() -> Result<MoralMetricFixtureReport> {
    let trace_examples = moral_trace_required_examples();
    let outcome_examples = outcome_linkage_required_examples();
    let definitions = moral_metric_definitions();

    let input_trace_refs = trace_examples
        .iter()
        .map(|example| format!("trace:{}", example.trace.trace_id))
        .collect::<Vec<_>>();
    let input_outcome_linkage_refs = outcome_examples
        .iter()
        .map(|example| format!("outcome-linkage:{}", example.record.linkage_id))
        .collect::<Vec<_>>();

    let review_path_total = trace_examples.len() as u32;
    let review_path_preserved = trace_examples
        .iter()
        .filter(|example| trace_has_review_path(&example.trace))
        .count() as u32;

    let delegated_examples = outcome_examples
        .iter()
        .filter(|example| example.record.source_trace.outcome.outcome_kind == "delegated")
        .collect::<Vec<_>>();
    let delegated_total = delegated_examples.len() as u32;
    let delegated_with_lineage = delegated_examples
        .iter()
        .filter(|example| delegated_lineage_preserved(&example.record))
        .count() as u32;

    let unresolved_count = outcome_examples
        .iter()
        .map(|example| {
            example
                .record
                .linked_outcomes
                .iter()
                .filter(|outcome| linked_outcome_requires_attention(outcome))
                .count() as u32
        })
        .sum::<u32>();

    let report = MoralMetricFixtureReport {
        schema_version: MORAL_METRIC_FIXTURE_REPORT_SCHEMA_VERSION.to_string(),
        report_id: "moral-metric-fixture-report-001".to_string(),
        summary:
            "WP-06 fixture report derives bounded review metrics from the required moral-trace and outcome-linkage examples."
                .to_string(),
        interpretation_boundary:
            "These metrics are review signals only. They are not a scalar karma score, not a scalar happiness score, and not a public reputation system."
                .to_string(),
        definitions,
        fixtures: vec![MoralMetricFixture {
            fixture_id: "moral-metric-review-window-alpha".to_string(),
            summary:
                "Combines the required WP-04 and WP-05 examples into one bounded review window for metric derivation."
                    .to_string(),
            input_trace_refs,
            input_outcome_linkage_refs,
            observations: vec![
                MoralMetricObservation {
                    metric_id: "trace-review-path-coverage".to_string(),
                    observed_window_ref: "review-window:alpha".to_string(),
                    numerator: review_path_preserved,
                    denominator: Some(review_path_total),
                    value_summary:
                        "All required trace fixtures preserve at least one reviewer, governance, packet, or challenge path."
                            .to_string(),
                    evidence_refs: vec![
                        "moral_trace.visibility.reviewer_evidence_refs".to_string(),
                        "moral_trace.review_refs.review_packet_refs".to_string(),
                    ],
                    limitations: vec![
                        "The fixture window is small and representative rather than exhaustive."
                            .to_string(),
                    ],
                },
                MoralMetricObservation {
                    metric_id: "delegation-lineage-retention".to_string(),
                    observed_window_ref: "review-window:alpha".to_string(),
                    numerator: delegated_with_lineage,
                    denominator: Some(delegated_total),
                    value_summary:
                        "Delegated linkage fixtures preserve explicit parent and child trace lineage."
                            .to_string(),
                    evidence_refs: vec![
                        "moral_trace.attribution.delegate_trace_ref".to_string(),
                        "outcome_linkage.attribution.delegate_trace_ref".to_string(),
                    ],
                    limitations: vec![
                        "This window contains one delegated outcome-linkage fixture, so trend use requires additional windows."
                            .to_string(),
                    ],
                },
                MoralMetricObservation {
                    metric_id: "unresolved-outcome-attention-count".to_string(),
                    observed_window_ref: "review-window:alpha".to_string(),
                    numerator: unresolved_count,
                    denominator: None,
                    value_summary:
                        "Four linked outcomes remain uncertain, partial, delayed, or contested and therefore still require review attention."
                            .to_string(),
                    evidence_refs: vec![
                        "outcome_linkage.linked_outcomes.outcome_status".to_string(),
                        "outcome_linkage.linked_outcomes.uncertainty_refs".to_string(),
                    ],
                    limitations: vec![
                        "This count measures unresolved review load, not severity or moral worth."
                            .to_string(),
                    ],
                },
            ],
            limitations: vec![
                "Required examples are intentionally small proof fixtures and should not be treated as production distributions."
                    .to_string(),
                "Trend detection needs repeated windows over time; a single fixture window only proves derivation rules."
                    .to_string(),
            ],
        }],
    };

    validate_moral_metric_fixture_report(&report)?;
    Ok(report)
}

pub fn validate_moral_metric_fixture_report(report: &MoralMetricFixtureReport) -> Result<()> {
    require_exact(
        &report.schema_version,
        MORAL_METRIC_FIXTURE_REPORT_SCHEMA_VERSION,
        "moral_metric_fixture_report.schema_version",
    )?;
    let _normalized_report_id = normalize_id(
        report.report_id.clone(),
        "moral_metric_fixture_report.report_id",
    )?;
    validate_nonempty_text(&report.summary, "moral_metric_fixture_report.summary")?;
    validate_nonempty_text(
        &report.interpretation_boundary,
        "moral_metric_fixture_report.interpretation_boundary",
    )?;
    require_non_scoreboard_boundary(&report.interpretation_boundary)?;

    if report.definitions.is_empty() {
        return Err(anyhow!(
            "moral_metric_fixture_report.definitions must not be empty"
        ));
    }
    if report.fixtures.is_empty() {
        return Err(anyhow!(
            "moral_metric_fixture_report.fixtures must not be empty"
        ));
    }

    let mut metric_kinds = std::collections::BTreeMap::new();
    for definition in &report.definitions {
        validate_metric_definition(definition)?;
        let normalized_metric_id = normalize_id(
            definition.metric_id.clone(),
            "moral_metric_definition.metric_id",
        )?;
        if metric_kinds
            .insert(normalized_metric_id, definition.measurement_kind.clone())
            .is_some()
        {
            return Err(anyhow!(
                "moral_metric_fixture_report.definitions contains duplicate metric_id"
            ));
        }
    }

    let mut fixture_ids = std::collections::BTreeSet::new();
    for fixture in &report.fixtures {
        validate_metric_fixture(fixture, &metric_kinds)?;
        let normalized_fixture_id = normalize_id(
            fixture.fixture_id.clone(),
            "moral_metric_fixture.fixture_id",
        )?;
        if !fixture_ids.insert(normalized_fixture_id) {
            return Err(anyhow!(
                "moral_metric_fixture_report.fixtures contains duplicate fixture_id"
            ));
        }
    }
    Ok(())
}

pub fn moral_metric_fixture_report_json_bytes(
    report: &MoralMetricFixtureReport,
) -> Result<Vec<u8>> {
    validate_moral_metric_fixture_report(report)?;
    let mut canonical = report.clone();
    canonicalize_moral_metric_fixture_report(&mut canonical);
    serde_json::to_vec_pretty(&canonical).context("serialize moral metric fixture report json")
}

fn validate_metric_definition(definition: &MoralMetricDefinition) -> Result<()> {
    require_exact(
        &definition.schema_version,
        MORAL_METRIC_DEFINITION_SCHEMA_VERSION,
        "moral_metric_definition.schema_version",
    )?;
    let _normalized_metric_id = normalize_id(
        definition.metric_id.clone(),
        "moral_metric_definition.metric_id",
    )?;
    validate_nonempty_text(
        &definition.display_name,
        "moral_metric_definition.display_name",
    )?;
    validate_nonempty_text(&definition.purpose, "moral_metric_definition.purpose")?;
    validate_nonempty_text(
        &definition.trend_detection_summary,
        "moral_metric_definition.trend_detection_summary",
    )?;
    validate_nonempty_text(
        &definition.interpretation_boundary,
        "moral_metric_definition.interpretation_boundary",
    )?;
    require_non_scoreboard_text(
        &definition.display_name,
        "moral_metric_definition.display_name",
    )?;
    require_non_scoreboard_text(&definition.purpose, "moral_metric_definition.purpose")?;
    require_non_scoreboard_text(
        &definition.trend_detection_summary,
        "moral_metric_definition.trend_detection_summary",
    )?;
    require_non_scoreboard_text(
        &definition.interpretation_boundary,
        "moral_metric_definition.interpretation_boundary",
    )?;

    match definition.measurement_kind.as_str() {
        "ratio" | "count" => {}
        _ => {
            return Err(anyhow!(
                "moral_metric_definition.measurement_kind unsupported"
            ))
        }
    }
    if definition.evidence_field_refs.is_empty() {
        return Err(anyhow!(
            "moral_metric_definition.evidence_field_refs must not be empty"
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut trace_derived = false;
    for field_ref in &definition.evidence_field_refs {
        validate_evidence_field_ref(field_ref, "moral_metric_definition.evidence_field_refs")?;
        if field_ref.starts_with("moral_trace.") || field_ref.starts_with("outcome_linkage.") {
            trace_derived = true;
        }
        if !seen.insert(field_ref.clone()) {
            return Err(anyhow!(
                "moral_metric_definition.evidence_field_refs contains duplicate field ref"
            ));
        }
    }
    if !trace_derived {
        return Err(anyhow!(
            "moral_metric_definition must derive from explicit trace evidence"
        ));
    }
    if definition.limitations.is_empty() {
        return Err(anyhow!(
            "moral_metric_definition.limitations must not be empty"
        ));
    }
    for limitation in &definition.limitations {
        validate_nonempty_text(limitation, "moral_metric_definition.limitations")?;
    }
    Ok(())
}

fn validate_metric_fixture(
    fixture: &MoralMetricFixture,
    metric_kinds: &std::collections::BTreeMap<String, String>,
) -> Result<()> {
    let _normalized_fixture_id = normalize_id(
        fixture.fixture_id.clone(),
        "moral_metric_fixture.fixture_id",
    )?;
    validate_nonempty_text(&fixture.summary, "moral_metric_fixture.summary")?;
    if fixture.input_trace_refs.is_empty() {
        return Err(anyhow!(
            "moral_metric_fixture.input_trace_refs must not be empty"
        ));
    }
    if fixture.input_outcome_linkage_refs.is_empty() {
        return Err(anyhow!(
            "moral_metric_fixture.input_outcome_linkage_refs must not be empty"
        ));
    }
    if fixture.observations.is_empty() {
        return Err(anyhow!(
            "moral_metric_fixture.observations must not be empty"
        ));
    }
    for trace_ref in &fixture.input_trace_refs {
        validate_prefixed_ref(trace_ref, "trace:", "moral_metric_fixture.input_trace_refs")?;
    }
    for linkage_ref in &fixture.input_outcome_linkage_refs {
        validate_prefixed_ref(
            linkage_ref,
            "outcome-linkage:",
            "moral_metric_fixture.input_outcome_linkage_refs",
        )?;
    }
    let mut seen_metric_ids = std::collections::BTreeSet::new();
    for observation in &fixture.observations {
        validate_metric_observation(observation, metric_kinds)?;
        let normalized_metric_id = normalize_id(
            observation.metric_id.clone(),
            "moral_metric_observation.metric_id",
        )?;
        if !seen_metric_ids.insert(normalized_metric_id) {
            return Err(anyhow!(
                "moral_metric_fixture.observations must not contain duplicate metric_id entries"
            ));
        }
    }
    if fixture.limitations.is_empty() {
        return Err(anyhow!(
            "moral_metric_fixture.limitations must not be empty"
        ));
    }
    for limitation in &fixture.limitations {
        validate_nonempty_text(limitation, "moral_metric_fixture.limitations")?;
    }
    Ok(())
}

fn validate_metric_observation(
    observation: &MoralMetricObservation,
    metric_kinds: &std::collections::BTreeMap<String, String>,
) -> Result<()> {
    let normalized_metric_id = normalize_id(
        observation.metric_id.clone(),
        "moral_metric_observation.metric_id",
    )?;
    let measurement_kind = metric_kinds.get(&normalized_metric_id).ok_or_else(|| {
        anyhow!("moral_metric_observation.metric_id must refer to a defined metric")
    })?;
    validate_prefixed_ref(
        &observation.observed_window_ref,
        "review-window:",
        "moral_metric_observation.observed_window_ref",
    )?;
    validate_nonempty_text(
        &observation.value_summary,
        "moral_metric_observation.value_summary",
    )?;
    require_non_scoreboard_text(
        &observation.value_summary,
        "moral_metric_observation.value_summary",
    )?;
    match measurement_kind.as_str() {
        "ratio" => {
            let denominator = observation.denominator.ok_or_else(|| {
                anyhow!("ratio moral_metric_observation entries must include a denominator")
            })?;
            if denominator == 0 {
                return Err(anyhow!(
                    "moral_metric_observation.denominator must be positive when present"
                ));
            }
            if observation.numerator > denominator {
                return Err(anyhow!(
                    "moral_metric_observation.numerator must not exceed denominator"
                ));
            }
        }
        "count" => {
            if observation.denominator.is_some() {
                return Err(anyhow!(
                    "count moral_metric_observation entries must not include a denominator"
                ));
            }
        }
        _ => {
            return Err(anyhow!(
                "moral_metric_observation.metric_id maps to unsupported kind"
            ))
        }
    }
    if observation.evidence_refs.is_empty() {
        return Err(anyhow!(
            "moral_metric_observation.evidence_refs must not be empty"
        ));
    }
    for evidence_ref in &observation.evidence_refs {
        validate_evidence_field_ref(evidence_ref, "moral_metric_observation.evidence_refs")?;
    }
    if observation.limitations.is_empty() {
        return Err(anyhow!(
            "moral_metric_observation.limitations must not be empty"
        ));
    }
    for limitation in &observation.limitations {
        validate_nonempty_text(limitation, "moral_metric_observation.limitations")?;
    }
    Ok(())
}

fn validate_evidence_field_ref(value: &str, field: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let prefix = if let Some(remainder) = trimmed.strip_prefix("moral_trace.") {
        ("moral_trace.", remainder)
    } else if let Some(remainder) = trimmed.strip_prefix("outcome_linkage.") {
        ("outcome_linkage.", remainder)
    } else {
        return Err(anyhow!(
            "{field} must derive from explicit moral_trace or outcome_linkage evidence fields"
        ));
    };
    if trimmed.contains('/') || trimmed.contains('\\') {
        return Err(anyhow!("{field} must name a field path, not a host path"));
    }
    if prefix.1.is_empty() {
        return Err(anyhow!(
            "{field} must include a concrete field path after the root prefix"
        ));
    }
    for segment in prefix.1.split('.') {
        if segment.is_empty() {
            return Err(anyhow!("{field} must not contain empty field segments"));
        }
        if !segment
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
        {
            return Err(anyhow!(
                "{field} must use portable snake_case field segments"
            ));
        }
    }
    Ok(())
}

fn validate_prefixed_ref(value: &str, prefix: &str, field: &str) -> Result<()> {
    let trimmed = value.trim();
    if !trimmed.starts_with(prefix) {
        return Err(anyhow!("{field} must start with {prefix}"));
    }
    if trimmed.len() == prefix.len() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let suffix = &trimmed[prefix.len()..];
    if suffix != suffix.trim() || suffix.chars().any(char::is_whitespace) {
        return Err(anyhow!("{field} must use a stable identifier suffix"));
    }
    if suffix.contains('/') || suffix.contains('\\') || suffix.contains(':') {
        return Err(anyhow!(
            "{field} must not contain path or nested prefix separators"
        ));
    }
    normalize_id(suffix.to_string(), field)?;
    Ok(())
}

fn require_non_scoreboard_boundary(boundary: &str) -> Result<()> {
    let normalized = boundary.to_ascii_lowercase();
    if normalized.contains("scalar karma")
        && normalized.contains("scalar happiness")
        && normalized.contains("public reputation")
    {
        return Ok(());
    }
    Err(anyhow!(
        "moral_metric_fixture_report.interpretation_boundary must explicitly reject scalar karma, scalar happiness, and public reputation framing"
    ))
}

fn require_non_scoreboard_text(value: &str, field: &str) -> Result<()> {
    let normalized = value.to_ascii_lowercase();
    let blocked = ["scoreboard", "karma", "happiness", "reputation", "score"];
    for token in blocked {
        if normalized.contains(token) {
            return Err(anyhow!(
                "{field} must avoid scoreboard-style public scoring language"
            ));
        }
    }
    Ok(())
}

fn trace_has_review_path(trace: &MoralTraceRecord) -> bool {
    !trace.visibility.reviewer_evidence_refs.is_empty()
        || !trace.visibility.governance_evidence_refs.is_empty()
        || !trace.review_refs.review_packet_refs.is_empty()
        || trace.review_refs.challenge_ref.is_some()
}

fn delegated_lineage_preserved(record: &OutcomeLinkageRecord) -> bool {
    record.attribution.delegated_by_trace_ref.is_some()
        && record.attribution.delegate_trace_ref.is_some()
        && record.attribution.delegated_by_trace_ref
            == record.source_trace.attribution.delegated_by_trace_ref
        && record.attribution.delegate_trace_ref
            == record.source_trace.attribution.delegate_trace_ref
}

fn linked_outcome_requires_attention(outcome: &LinkedOutcome) -> bool {
    matches!(
        outcome.outcome_status.as_str(),
        "unknown" | "partial" | "delayed" | "contested"
    ) || !outcome.uncertainty_refs.is_empty()
        || !outcome.rebuttal_refs.is_empty()
}

fn canonicalize_moral_metric_fixture_report(report: &mut MoralMetricFixtureReport) {
    report
        .definitions
        .sort_by(|left, right| left.metric_id.cmp(&right.metric_id));
    for definition in &mut report.definitions {
        definition.evidence_field_refs.sort();
        definition.limitations.sort();
    }
    report
        .fixtures
        .sort_by(|left, right| left.fixture_id.cmp(&right.fixture_id));
    for fixture in &mut report.fixtures {
        fixture.input_trace_refs.sort();
        fixture.input_outcome_linkage_refs.sort();
        fixture.limitations.sort();
        fixture
            .observations
            .sort_by(|left, right| left.metric_id.cmp(&right.metric_id));
        for observation in &mut fixture.observations {
            observation.evidence_refs.sort();
            observation.limitations.sort();
        }
    }
}

fn require_exact(value: &str, expected: &str, field: &str) -> Result<()> {
    if value == expected {
        Ok(())
    } else {
        Err(anyhow!("{field} must equal {expected}"))
    }
}
