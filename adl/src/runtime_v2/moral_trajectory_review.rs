//! Runtime-v2 moral trajectory review contract.
//!
//! WP-07 consumes the trace, outcome-linkage, and moral-metric surfaces and
//! packages them into a deterministic reviewer packet over event, segment, and
//! longitudinal windows. The packet must cite evidence directly and preserve
//! uncertainty rather than substituting hidden judgment.

use super::*;

pub const MORAL_TRAJECTORY_REVIEW_PACKET_SCHEMA_VERSION: &str = "moral_trajectory_review_packet.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralTrajectoryReviewCriterion {
    pub criterion_id: String,
    pub focus_kind: String,
    pub question: String,
    pub evidence_requirements: Vec<String>,
    pub tie_break_rule: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralTrajectoryReviewWindow {
    pub window_id: String,
    pub window_kind: String,
    pub summary: String,
    pub trace_refs: Vec<String>,
    pub outcome_linkage_refs: Vec<String>,
    pub metric_ids: Vec<String>,
    pub first_trace_sequence: u32,
    pub last_trace_sequence: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralTrajectoryReviewFinding {
    pub finding_id: String,
    pub window_id: String,
    pub criterion_id: String,
    pub review_status: String,
    pub signal_kind: String,
    pub summary: String,
    pub trace_evidence_refs: Vec<String>,
    pub outcome_linkage_refs: Vec<String>,
    pub metric_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralTrajectorySyntheticFixture {
    pub fixture_id: String,
    pub window_id: String,
    pub summary: String,
    pub expected_criterion_ids: Vec<String>,
    pub claim_boundary: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralTrajectoryReviewPacket {
    pub schema_version: String,
    pub review_id: String,
    pub summary: String,
    pub interpretation_boundary: String,
    pub deterministic_ordering_rule: String,
    pub criteria: Vec<MoralTrajectoryReviewCriterion>,
    pub windows: Vec<MoralTrajectoryReviewWindow>,
    pub findings: Vec<MoralTrajectoryReviewFinding>,
    pub synthetic_fixtures: Vec<MoralTrajectorySyntheticFixture>,
}

pub fn moral_trajectory_review_packet() -> Result<MoralTrajectoryReviewPacket> {
    let trace_examples = moral_trace_required_examples();
    let outcome_examples = outcome_linkage_required_examples();
    let metric_report = moral_metric_fixture_report()?;

    let ordinary_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::OrdinaryAction)
        .ok_or_else(|| anyhow!("WP-07 requires the ordinary-action moral trace example"))?
        .trace
        .clone();
    let refusal_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::Refusal)
        .ok_or_else(|| anyhow!("WP-07 requires the refusal moral trace example"))?
        .trace
        .clone();
    let delegation_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::Delegation)
        .ok_or_else(|| anyhow!("WP-07 requires the delegation moral trace example"))?
        .trace
        .clone();
    let deferred_trace = trace_examples
        .iter()
        .find(|example| example.example_kind == MoralTraceExampleKind::DeferredDecision)
        .ok_or_else(|| anyhow!("WP-07 requires the deferred-decision moral trace example"))?
        .trace
        .clone();

    let known_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Known)
        .ok_or_else(|| anyhow!("WP-07 requires the known outcome-linkage example"))?
        .record
        .clone();
    let unknown_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Unknown)
        .ok_or_else(|| anyhow!("WP-07 requires the unknown outcome-linkage example"))?
        .record
        .clone();
    let partial_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Partial)
        .ok_or_else(|| anyhow!("WP-07 requires the partial outcome-linkage example"))?
        .record
        .clone();
    let delayed_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Delayed)
        .ok_or_else(|| anyhow!("WP-07 requires the delayed outcome-linkage example"))?
        .record
        .clone();
    let contested_outcome = outcome_examples
        .iter()
        .find(|example| example.example_kind == OutcomeLinkageExampleKind::Contested)
        .ok_or_else(|| anyhow!("WP-07 requires the contested outcome-linkage example"))?
        .record
        .clone();

    let metric_ids = metric_report
        .definitions
        .iter()
        .map(|definition| definition.metric_id.clone())
        .collect::<Vec<_>>();

    let all_trace_refs = ordered_trace_refs(&[
        ordinary_trace.clone(),
        refusal_trace.clone(),
        delegation_trace.clone(),
        deferred_trace.clone(),
    ]);
    let all_outcome_refs = ordered_outcome_refs(&[
        known_outcome.clone(),
        unknown_outcome.clone(),
        partial_outcome.clone(),
        delayed_outcome.clone(),
        contested_outcome.clone(),
    ]);

    let criteria = trajectory_review_criteria();
    let windows = vec![
        MoralTrajectoryReviewWindow {
            window_id: "event-window-refusal-boundary".to_string(),
            window_kind: "event".to_string(),
            summary:
                "Single-event window showing that refusal stays reviewable without public private-state leakage."
                    .to_string(),
            trace_refs: vec![format!("trace:{}", refusal_trace.trace_id)],
            outcome_linkage_refs: vec![],
            metric_ids: vec!["trace-review-path-coverage".to_string()],
            first_trace_sequence: refusal_trace.trace_sequence,
            last_trace_sequence: refusal_trace.trace_sequence,
        },
        MoralTrajectoryReviewWindow {
            window_id: "segment-window-delegation-escalation".to_string(),
            window_kind: "segment".to_string(),
            summary:
                "Segment window joining delegated, delayed, and contested evidence so reviewers can inspect escalation and unresolved uncertainty."
                    .to_string(),
            trace_refs: ordered_trace_refs(&[delegation_trace.clone(), deferred_trace.clone()]),
            outcome_linkage_refs: ordered_outcome_refs(&[
                delayed_outcome.clone(),
                contested_outcome.clone(),
            ]),
            metric_ids: vec![
                "delegation-lineage-retention".to_string(),
                "unresolved-outcome-attention-count".to_string(),
            ],
            first_trace_sequence: delegation_trace.trace_sequence.min(deferred_trace.trace_sequence),
            last_trace_sequence: delegation_trace.trace_sequence.max(deferred_trace.trace_sequence),
        },
        MoralTrajectoryReviewWindow {
            window_id: "longitudinal-window-alpha".to_string(),
            window_kind: "longitudinal".to_string(),
            summary:
                "Longitudinal fixture window combining all required WP-04 through WP-06 examples for drift, repetition, repair, refusal, escalation, and uncertainty review."
                    .to_string(),
            trace_refs: all_trace_refs.clone(),
            outcome_linkage_refs: all_outcome_refs.clone(),
            metric_ids,
            first_trace_sequence: ordinary_trace
                .trace_sequence
                .min(refusal_trace.trace_sequence)
                .min(delegation_trace.trace_sequence)
                .min(deferred_trace.trace_sequence),
            last_trace_sequence: ordinary_trace
                .trace_sequence
                .max(refusal_trace.trace_sequence)
                .max(delegation_trace.trace_sequence)
                .max(deferred_trace.trace_sequence),
        },
    ];

    let findings = vec![
        MoralTrajectoryReviewFinding {
            finding_id: "trajectory-finding-refusal-preserved".to_string(),
            window_id: "event-window-refusal-boundary".to_string(),
            criterion_id: "criterion-refusal".to_string(),
            review_status: "observed".to_string(),
            signal_kind: "refusal_preserved".to_string(),
            summary:
                "The refusal event preserved reviewer evidence and challenge visibility while blocking public private-state disclosure."
                    .to_string(),
            trace_evidence_refs: vec![format!("trace:{}", refusal_trace.trace_id)],
            outcome_linkage_refs: vec![],
            metric_ids: vec!["trace-review-path-coverage".to_string()],
        },
        MoralTrajectoryReviewFinding {
            finding_id: "trajectory-finding-escalation-active".to_string(),
            window_id: "segment-window-delegation-escalation".to_string(),
            criterion_id: "criterion-escalation".to_string(),
            review_status: "review_needed".to_string(),
            signal_kind: "escalation_required".to_string(),
            summary:
                "The deferred trace and delayed outcome keep the escalation path active rather than pretending the review burden is already closed."
                    .to_string(),
            trace_evidence_refs: vec![format!("trace:{}", deferred_trace.trace_id)],
            outcome_linkage_refs: vec![format!(
                "outcome-linkage:{}",
                delayed_outcome.linkage_id
            )],
            metric_ids: vec!["unresolved-outcome-attention-count".to_string()],
        },
        MoralTrajectoryReviewFinding {
            finding_id: "trajectory-finding-uncertainty-open".to_string(),
            window_id: "segment-window-delegation-escalation".to_string(),
            criterion_id: "criterion-unresolved-uncertainty".to_string(),
            review_status: "observed".to_string(),
            signal_kind: "uncertainty_active".to_string(),
            summary:
                "Unknown, partial, delayed, and contested outcomes remain explicitly open for review instead of being collapsed into false certainty."
                    .to_string(),
            trace_evidence_refs: vec![
                format!("trace:{}", delegation_trace.trace_id),
                format!("trace:{}", deferred_trace.trace_id),
            ],
            outcome_linkage_refs: ordered_outcome_refs(&[
                unknown_outcome.clone(),
                partial_outcome.clone(),
                delayed_outcome.clone(),
                contested_outcome.clone(),
            ]),
            metric_ids: vec!["unresolved-outcome-attention-count".to_string()],
        },
        MoralTrajectoryReviewFinding {
            finding_id: "trajectory-finding-drift-stable".to_string(),
            window_id: "longitudinal-window-alpha".to_string(),
            criterion_id: "criterion-drift".to_string(),
            review_status: "observed".to_string(),
            signal_kind: "stable".to_string(),
            summary:
                "This bounded longitudinal window shows no downward drift in review-path or delegation-lineage evidence, though the sample is too small for production claims."
                    .to_string(),
            trace_evidence_refs: all_trace_refs.clone(),
            outcome_linkage_refs: vec![format!("outcome-linkage:{}", contested_outcome.linkage_id)],
            metric_ids: vec![
                "trace-review-path-coverage".to_string(),
                "delegation-lineage-retention".to_string(),
            ],
        },
        MoralTrajectoryReviewFinding {
            finding_id: "trajectory-finding-repetition-bounded".to_string(),
            window_id: "longitudinal-window-alpha".to_string(),
            criterion_id: "criterion-repetition".to_string(),
            review_status: "observed".to_string(),
            signal_kind: "stable".to_string(),
            summary:
                "The fixture set does not contain a repetitive refusal or delegation loop, so repetition remains bounded rather than silently compounding."
                    .to_string(),
            trace_evidence_refs: all_trace_refs.clone(),
            outcome_linkage_refs: vec![],
            metric_ids: vec!["trace-review-path-coverage".to_string()],
        },
        MoralTrajectoryReviewFinding {
            finding_id: "trajectory-finding-repair-watch".to_string(),
            window_id: "longitudinal-window-alpha".to_string(),
            criterion_id: "criterion-repair".to_string(),
            review_status: "watch".to_string(),
            signal_kind: "repair_watch".to_string(),
            summary:
                "The later completed action shows bounded forward movement after refusal, but the fixture window does not prove full repair closure and should stay under review."
                    .to_string(),
            trace_evidence_refs: vec![
                format!("trace:{}", refusal_trace.trace_id),
                format!("trace:{}", ordinary_trace.trace_id),
            ],
            outcome_linkage_refs: vec![format!("outcome-linkage:{}", known_outcome.linkage_id)],
            metric_ids: vec![
                "trace-review-path-coverage".to_string(),
                "unresolved-outcome-attention-count".to_string(),
            ],
        },
    ];

    let synthetic_fixtures = vec![
        MoralTrajectorySyntheticFixture {
            fixture_id: "trajectory-fixture-refusal-boundary".to_string(),
            window_id: "event-window-refusal-boundary".to_string(),
            summary:
                "Synthetic single-event refusal fixture used to prove reviewer-visible refusal reasoning without public leakage."
                    .to_string(),
            expected_criterion_ids: vec!["criterion-refusal".to_string()],
            claim_boundary:
                "Proves a bounded refusal review window only; it does not prove full repair, anti-harm closure, or production moral judgment."
                    .to_string(),
            limitations: vec![
                "A single-event fixture cannot establish longitudinal trend behavior."
                    .to_string(),
            ],
        },
        MoralTrajectorySyntheticFixture {
            fixture_id: "trajectory-fixture-delegation-escalation".to_string(),
            window_id: "segment-window-delegation-escalation".to_string(),
            summary:
                "Synthetic segment fixture for delegated, delayed, and contested evidence with explicit escalation and uncertainty review."
                    .to_string(),
            expected_criterion_ids: vec![
                "criterion-escalation".to_string(),
                "criterion-unresolved-uncertainty".to_string(),
            ],
            claim_boundary:
                "Proves bounded segment-level reviewability only; it does not prove anti-harm denial semantics from WP-08."
                    .to_string(),
            limitations: vec![
                "The fixture is reviewer-oriented and intentionally does not simulate live harmful execution."
                    .to_string(),
            ],
        },
        MoralTrajectorySyntheticFixture {
            fixture_id: "trajectory-fixture-longitudinal-alpha".to_string(),
            window_id: "longitudinal-window-alpha".to_string(),
            summary:
                "Synthetic longitudinal window combining all required upstream examples into one deterministic review packet."
                    .to_string(),
            expected_criterion_ids: vec![
                "criterion-drift".to_string(),
                "criterion-repetition".to_string(),
                "criterion-repair".to_string(),
            ],
            claim_boundary:
                "Proves deterministic longitudinal review packet generation only; it does not imply production-scale behavioral generalization."
                    .to_string(),
            limitations: vec![
                "The longitudinal fixture is intentionally small and should not be read as a complete population sample."
                    .to_string(),
            ],
        },
    ];

    let packet = MoralTrajectoryReviewPacket {
        schema_version: MORAL_TRAJECTORY_REVIEW_PACKET_SCHEMA_VERSION.to_string(),
        review_id: "moral-trajectory-review-alpha-001".to_string(),
        summary:
            "WP-07 trajectory review packet lets reviewers inspect bounded event, segment, and longitudinal moral evidence without reconstructing hidden state manually."
                .to_string(),
        interpretation_boundary:
            "Interpret this packet as reviewer evidence only. It is not final moral judgment, not a scalar score, and not a replacement for later anti-harm review."
                .to_string(),
        deterministic_ordering_rule:
            "Sort windows by window_kind rank (event, segment, longitudinal), then first_trace_sequence, then window_id. Sort findings by window_id, criterion_id, then finding_id."
                .to_string(),
        criteria,
        windows,
        findings,
        synthetic_fixtures,
    };

    validate_moral_trajectory_review_packet(&packet)?;
    Ok(packet)
}

pub fn moral_trajectory_review_json_bytes(packet: &MoralTrajectoryReviewPacket) -> Result<Vec<u8>> {
    validate_moral_trajectory_review_packet(packet)?;
    let mut canonical = packet.clone();
    canonicalize_moral_trajectory_review_packet(&mut canonical);
    serde_json::to_vec_pretty(&canonical).context("serialize moral trajectory review packet json")
}

pub fn validate_moral_trajectory_review_packet(packet: &MoralTrajectoryReviewPacket) -> Result<()> {
    require_exact(
        &packet.schema_version,
        MORAL_TRAJECTORY_REVIEW_PACKET_SCHEMA_VERSION,
        "moral_trajectory_review_packet.schema_version",
    )?;
    normalize_id(
        packet.review_id.clone(),
        "moral_trajectory_review_packet.review_id",
    )?;
    validate_nonempty_text(&packet.summary, "moral_trajectory_review_packet.summary")?;
    validate_nonempty_text(
        &packet.interpretation_boundary,
        "moral_trajectory_review_packet.interpretation_boundary",
    )?;
    validate_nonempty_text(
        &packet.deterministic_ordering_rule,
        "moral_trajectory_review_packet.deterministic_ordering_rule",
    )?;
    require_non_judgment_boundary(&packet.interpretation_boundary)?;
    require_deterministic_ordering_rule(&packet.deterministic_ordering_rule)?;

    let required_criteria = [
        "criterion-drift",
        "criterion-repetition",
        "criterion-repair",
        "criterion-refusal",
        "criterion-escalation",
        "criterion-unresolved-uncertainty",
    ];
    if packet.criteria.len() != required_criteria.len() {
        return Err(anyhow!(
            "moral_trajectory_review_packet.criteria must include each required criterion exactly once"
        ));
    }
    let mut criterion_ids = std::collections::BTreeSet::new();
    for criterion in &packet.criteria {
        validate_trajectory_criterion(criterion)?;
        if !criterion_ids.insert(criterion.criterion_id.clone()) {
            return Err(anyhow!(
                "moral_trajectory_review_packet.criteria contains duplicate criterion_id"
            ));
        }
    }
    for criterion_id in required_criteria {
        if !criterion_ids.contains(criterion_id) {
            return Err(anyhow!(
                "moral_trajectory_review_packet.criteria missing required criterion"
            ));
        }
    }

    let required_window_kinds = ["event", "segment", "longitudinal"];
    if packet.windows.len() != required_window_kinds.len() {
        return Err(anyhow!(
            "moral_trajectory_review_packet.windows must include event, segment, and longitudinal windows"
        ));
    }
    let mut window_ids = std::collections::BTreeSet::new();
    let mut window_kinds = std::collections::BTreeSet::new();
    let mut known_metric_ids = std::collections::BTreeSet::new();
    for definition in moral_metric_definitions() {
        known_metric_ids.insert(definition.metric_id);
    }
    for window in &packet.windows {
        validate_trajectory_window(window, &known_metric_ids)?;
        if !window_ids.insert(window.window_id.clone()) {
            return Err(anyhow!(
                "moral_trajectory_review_packet.windows contains duplicate window_id"
            ));
        }
        if !window_kinds.insert(window.window_kind.clone()) {
            return Err(anyhow!(
                "moral_trajectory_review_packet.windows contains duplicate window_kind"
            ));
        }
    }
    for window_kind in required_window_kinds {
        if !window_kinds.contains(window_kind) {
            return Err(anyhow!(
                "moral_trajectory_review_packet.windows missing required window kind"
            ));
        }
    }

    if packet.findings.is_empty() {
        return Err(anyhow!(
            "moral_trajectory_review_packet.findings must not be empty"
        ));
    }
    let mut finding_ids = std::collections::BTreeSet::new();
    let mut covered_criterion_ids = std::collections::BTreeSet::new();
    for finding in &packet.findings {
        validate_trajectory_finding(finding, &window_ids, &criterion_ids, &known_metric_ids)?;
        if !finding_ids.insert(finding.finding_id.clone()) {
            return Err(anyhow!(
                "moral_trajectory_review_packet.findings contains duplicate finding_id"
            ));
        }
        covered_criterion_ids.insert(finding.criterion_id.clone());
    }
    for criterion_id in &criterion_ids {
        if !covered_criterion_ids.contains(criterion_id) {
            return Err(anyhow!(
                "moral_trajectory_review_packet.findings must cover every required criterion at least once"
            ));
        }
    }

    if packet.synthetic_fixtures.is_empty() {
        return Err(anyhow!(
            "moral_trajectory_review_packet.synthetic_fixtures must not be empty"
        ));
    }
    let mut fixture_ids = std::collections::BTreeSet::new();
    for fixture in &packet.synthetic_fixtures {
        validate_synthetic_fixture(fixture, &window_ids, &criterion_ids)?;
        if !fixture_ids.insert(fixture.fixture_id.clone()) {
            return Err(anyhow!(
                "moral_trajectory_review_packet.synthetic_fixtures contains duplicate fixture_id"
            ));
        }
    }
    Ok(())
}

fn trajectory_review_criteria() -> Vec<MoralTrajectoryReviewCriterion> {
    vec![
        criterion(
            "criterion-drift",
            "drift",
            "Are reviewability, accountability, or disclosure patterns degrading over time?",
            &[
                "moral_trace.review_refs.review_packet_refs",
                "moral_trace.visibility.reviewer_evidence_refs",
                "outcome_linkage.attribution.delegated_by_trace_ref",
            ],
            "If multiple windows show the same drift signal, prefer the earliest first_trace_sequence and then criterion_id.",
            &[
                "Drift signals in this bounded fixture set are trend hints rather than production-rate estimates.",
            ],
        ),
        criterion(
            "criterion-repetition",
            "repetition",
            "Is a refusal, delegation, or unresolved-review pattern repeating instead of resolving?",
            &[
                "moral_trace.outcome.outcome_kind",
                "outcome_linkage.linked_outcomes.outcome_status",
            ],
            "Break ties by window_id, then criterion_id, then finding_id.",
            &[
                "Bounded examples can show repeated structure without proving pathological loop behavior.",
            ],
        ),
        criterion(
            "criterion-repair",
            "repair",
            "Does later evidence show bounded corrective movement after refusal, contestation, or uncertainty?",
            &[
                "moral_trace.outcome.outcome_summary",
                "outcome_linkage.linked_outcomes.effect_summary",
            ],
            "Prefer direct trace evidence before outcome-linkage inferences when repair signals conflict.",
            &[
                "Absence of repair evidence is not the same as proof that repair is impossible.",
            ],
        ),
        criterion(
            "criterion-refusal",
            "refusal",
            "Was refusal preserved as reviewable moral action rather than flattened into silent failure?",
            &[
                "moral_trace.moral_event.refusal.refused",
                "moral_trace.review_refs.challenge_ref",
            ],
            "When multiple refusal traces exist, review lower trace_sequence first and then criterion_id.",
            &[
                "A preserved refusal path does not by itself prove every later consequence was benign.",
            ],
        ),
        criterion(
            "criterion-escalation",
            "escalation",
            "Did delayed or contested evidence keep an explicit escalation path open?",
            &[
                "moral_trace.outcome.outcome_kind",
                "outcome_linkage.linked_outcomes.uncertainty_refs",
            ],
            "Prefer deferred traces over inferred escalation notes when both exist in the same window.",
            &[
                "Escalation review remains bounded to available evidence and should not infer hidden operator action.",
            ],
        ),
        criterion(
            "criterion-unresolved-uncertainty",
            "unresolved_uncertainty",
            "What uncertainty remains open across the trajectory and how is it kept visible?",
            &[
                "moral_trace.moral_event.uncertainty.level",
                "outcome_linkage.linked_outcomes.uncertainty_refs",
                "outcome_linkage.linked_outcomes.rebuttal_refs",
            ],
            "Prefer windows with more direct uncertainty refs; break remaining ties by criterion_id.",
            &[
                "A high uncertainty signal can indicate honest disclosure rather than worse underlying behavior.",
            ],
        ),
    ]
}

fn criterion(
    criterion_id: &str,
    focus_kind: &str,
    question: &str,
    evidence_requirements: &[&str],
    tie_break_rule: &str,
    limitations: &[&str],
) -> MoralTrajectoryReviewCriterion {
    MoralTrajectoryReviewCriterion {
        criterion_id: criterion_id.to_string(),
        focus_kind: focus_kind.to_string(),
        question: question.to_string(),
        evidence_requirements: evidence_requirements
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        tie_break_rule: tie_break_rule.to_string(),
        limitations: limitations
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
    }
}

fn validate_trajectory_criterion(criterion: &MoralTrajectoryReviewCriterion) -> Result<()> {
    normalize_id(
        criterion.criterion_id.clone(),
        "moral_trajectory_review_criterion.criterion_id",
    )?;
    match criterion.focus_kind.as_str() {
        "drift" | "repetition" | "repair" | "refusal" | "escalation" | "unresolved_uncertainty" => {
        }
        _ => {
            return Err(anyhow!(
                "moral_trajectory_review_criterion.focus_kind unsupported"
            ))
        }
    }
    validate_nonempty_text(
        &criterion.question,
        "moral_trajectory_review_criterion.question",
    )?;
    validate_nonempty_text(
        &criterion.tie_break_rule,
        "moral_trajectory_review_criterion.tie_break_rule",
    )?;
    if criterion.evidence_requirements.is_empty() {
        return Err(anyhow!(
            "moral_trajectory_review_criterion.evidence_requirements must not be empty"
        ));
    }
    for evidence_field in &criterion.evidence_requirements {
        validate_evidence_field_ref(
            evidence_field,
            "moral_trajectory_review_criterion.evidence_requirements",
        )?;
    }
    if criterion.limitations.is_empty() {
        return Err(anyhow!(
            "moral_trajectory_review_criterion.limitations must not be empty"
        ));
    }
    for limitation in &criterion.limitations {
        validate_nonempty_text(limitation, "moral_trajectory_review_criterion.limitations")?;
    }
    Ok(())
}

fn validate_trajectory_window(
    window: &MoralTrajectoryReviewWindow,
    known_metric_ids: &std::collections::BTreeSet<String>,
) -> Result<()> {
    normalize_id(
        window.window_id.clone(),
        "moral_trajectory_review_window.window_id",
    )?;
    match window.window_kind.as_str() {
        "event" | "segment" | "longitudinal" => {}
        _ => {
            return Err(anyhow!(
                "moral_trajectory_review_window.window_kind unsupported"
            ))
        }
    }
    validate_nonempty_text(&window.summary, "moral_trajectory_review_window.summary")?;
    if window.trace_refs.is_empty() {
        return Err(anyhow!(
            "moral_trajectory_review_window.trace_refs must not be empty"
        ));
    }
    for trace_ref in &window.trace_refs {
        validate_prefixed_ref(
            trace_ref,
            "trace:",
            "moral_trajectory_review_window.trace_refs",
        )?;
    }
    for outcome_ref in &window.outcome_linkage_refs {
        validate_prefixed_ref(
            outcome_ref,
            "outcome-linkage:",
            "moral_trajectory_review_window.outcome_linkage_refs",
        )?;
    }
    for metric_id in &window.metric_ids {
        let normalized_metric_id = normalize_id(
            metric_id.clone(),
            "moral_trajectory_review_window.metric_ids",
        )?;
        if !known_metric_ids.contains(&normalized_metric_id) {
            return Err(anyhow!(
                "moral_trajectory_review_window.metric_ids must refer to defined WP-06 metrics"
            ));
        }
    }
    if window.first_trace_sequence == 0 || window.last_trace_sequence == 0 {
        return Err(anyhow!(
            "moral_trajectory_review_window trace sequences must be positive"
        ));
    }
    if window.first_trace_sequence > window.last_trace_sequence {
        return Err(anyhow!(
            "moral_trajectory_review_window first_trace_sequence must not exceed last_trace_sequence"
        ));
    }
    Ok(())
}

fn validate_trajectory_finding(
    finding: &MoralTrajectoryReviewFinding,
    window_ids: &std::collections::BTreeSet<String>,
    criterion_ids: &std::collections::BTreeSet<String>,
    known_metric_ids: &std::collections::BTreeSet<String>,
) -> Result<()> {
    normalize_id(
        finding.finding_id.clone(),
        "moral_trajectory_review_finding.finding_id",
    )?;
    if !window_ids.contains(&finding.window_id) {
        return Err(anyhow!(
            "moral_trajectory_review_finding.window_id must refer to a known window"
        ));
    }
    if !criterion_ids.contains(&finding.criterion_id) {
        return Err(anyhow!(
            "moral_trajectory_review_finding.criterion_id must refer to a known criterion"
        ));
    }
    match finding.review_status.as_str() {
        "observed" | "watch" | "review_needed" => {}
        _ => {
            return Err(anyhow!(
                "moral_trajectory_review_finding.review_status unsupported"
            ))
        }
    }
    match finding.signal_kind.as_str() {
        "stable"
        | "repair_watch"
        | "refusal_preserved"
        | "escalation_required"
        | "uncertainty_active" => {}
        _ => {
            return Err(anyhow!(
                "moral_trajectory_review_finding.signal_kind unsupported"
            ))
        }
    }
    validate_nonempty_text(&finding.summary, "moral_trajectory_review_finding.summary")?;
    require_non_judgment_text(&finding.summary, "moral_trajectory_review_finding.summary")?;
    if finding.trace_evidence_refs.is_empty() {
        return Err(anyhow!(
            "moral_trajectory_review_finding.trace_evidence_refs must cite trace evidence directly"
        ));
    }
    for trace_ref in &finding.trace_evidence_refs {
        validate_prefixed_ref(
            trace_ref,
            "trace:",
            "moral_trajectory_review_finding.trace_evidence_refs",
        )?;
    }
    for outcome_ref in &finding.outcome_linkage_refs {
        validate_prefixed_ref(
            outcome_ref,
            "outcome-linkage:",
            "moral_trajectory_review_finding.outcome_linkage_refs",
        )?;
    }
    for metric_id in &finding.metric_ids {
        let normalized_metric_id = normalize_id(
            metric_id.clone(),
            "moral_trajectory_review_finding.metric_ids",
        )?;
        if !known_metric_ids.contains(&normalized_metric_id) {
            return Err(anyhow!(
                "moral_trajectory_review_finding.metric_ids must refer to defined WP-06 metrics"
            ));
        }
    }
    Ok(())
}

fn validate_synthetic_fixture(
    fixture: &MoralTrajectorySyntheticFixture,
    window_ids: &std::collections::BTreeSet<String>,
    criterion_ids: &std::collections::BTreeSet<String>,
) -> Result<()> {
    normalize_id(
        fixture.fixture_id.clone(),
        "moral_trajectory_synthetic_fixture.fixture_id",
    )?;
    if !window_ids.contains(&fixture.window_id) {
        return Err(anyhow!(
            "moral_trajectory_synthetic_fixture.window_id must refer to a known window"
        ));
    }
    validate_nonempty_text(
        &fixture.summary,
        "moral_trajectory_synthetic_fixture.summary",
    )?;
    validate_nonempty_text(
        &fixture.claim_boundary,
        "moral_trajectory_synthetic_fixture.claim_boundary",
    )?;
    require_non_judgment_text(
        &fixture.claim_boundary,
        "moral_trajectory_synthetic_fixture.claim_boundary",
    )?;
    if fixture.expected_criterion_ids.is_empty() {
        return Err(anyhow!(
            "moral_trajectory_synthetic_fixture.expected_criterion_ids must not be empty"
        ));
    }
    for criterion_id in &fixture.expected_criterion_ids {
        if !criterion_ids.contains(criterion_id) {
            return Err(anyhow!(
                "moral_trajectory_synthetic_fixture.expected_criterion_ids must refer to known criteria"
            ));
        }
    }
    if fixture.limitations.is_empty() {
        return Err(anyhow!(
            "moral_trajectory_synthetic_fixture.limitations must not be empty"
        ));
    }
    for limitation in &fixture.limitations {
        validate_nonempty_text(limitation, "moral_trajectory_synthetic_fixture.limitations")?;
    }
    Ok(())
}

fn ordered_trace_refs(traces: &[MoralTraceRecord]) -> Vec<String> {
    let mut refs = traces
        .iter()
        .map(|trace| (trace.trace_sequence, format!("trace:{}", trace.trace_id)))
        .collect::<Vec<_>>();
    refs.sort();
    refs.into_iter().map(|(_, value)| value).collect()
}

fn ordered_outcome_refs(outcomes: &[OutcomeLinkageRecord]) -> Vec<String> {
    let mut refs = outcomes
        .iter()
        .map(|outcome| format!("outcome-linkage:{}", outcome.linkage_id))
        .collect::<Vec<_>>();
    refs.sort();
    refs
}

fn require_non_judgment_boundary(value: &str) -> Result<()> {
    let normalized = value.to_ascii_lowercase();
    if normalized.contains("not final moral judgment")
        && normalized.contains("not a scalar score")
        && normalized.contains("not a replacement for later anti-harm review")
    {
        return Ok(());
    }
    Err(anyhow!(
        "moral_trajectory_review_packet.interpretation_boundary must explicitly reject final judgment, scalar scoring, and anti-harm replacement framing"
    ))
}

fn require_non_judgment_text(value: &str, field: &str) -> Result<()> {
    let normalized = value.to_ascii_lowercase();
    for token in [
        "hidden judgment",
        "final moral judgment",
        "scoreboard",
        "reputation score",
    ] {
        if normalized.contains(token) {
            return Err(anyhow!(
                "{field} must avoid hidden-judgment or scoreboard framing"
            ));
        }
    }
    Ok(())
}

fn require_deterministic_ordering_rule(value: &str) -> Result<()> {
    let normalized = value.to_ascii_lowercase();
    if normalized.contains("window_kind")
        && normalized.contains("first_trace_sequence")
        && normalized.contains("window_id")
        && normalized.contains("criterion_id")
        && normalized.contains("finding_id")
    {
        return Ok(());
    }
    Err(anyhow!(
        "moral_trajectory_review_packet.deterministic_ordering_rule must declare deterministic window and finding tie-breaks"
    ))
}

fn canonicalize_moral_trajectory_review_packet(packet: &mut MoralTrajectoryReviewPacket) {
    packet
        .criteria
        .sort_by(|left, right| left.criterion_id.cmp(&right.criterion_id));
    for criterion in &mut packet.criteria {
        criterion.evidence_requirements.sort();
        criterion.limitations.sort();
    }
    packet.windows.sort_by(|left, right| {
        window_kind_rank(&left.window_kind)
            .cmp(&window_kind_rank(&right.window_kind))
            .then(left.first_trace_sequence.cmp(&right.first_trace_sequence))
            .then(left.window_id.cmp(&right.window_id))
    });
    for window in &mut packet.windows {
        window.trace_refs.sort();
        window.outcome_linkage_refs.sort();
        window.metric_ids.sort();
    }
    packet.findings.sort_by(|left, right| {
        left.window_id
            .cmp(&right.window_id)
            .then(left.criterion_id.cmp(&right.criterion_id))
            .then(left.finding_id.cmp(&right.finding_id))
    });
    for finding in &mut packet.findings {
        finding.trace_evidence_refs.sort();
        finding.outcome_linkage_refs.sort();
        finding.metric_ids.sort();
    }
    packet
        .synthetic_fixtures
        .sort_by(|left, right| left.fixture_id.cmp(&right.fixture_id));
    for fixture in &mut packet.synthetic_fixtures {
        fixture.expected_criterion_ids.sort();
        fixture.limitations.sort();
    }
}

fn window_kind_rank(kind: &str) -> u8 {
    match kind {
        "event" => 0,
        "segment" => 1,
        "longitudinal" => 2,
        _ => 255,
    }
}

fn validate_evidence_field_ref(value: &str, field: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let suffix = if let Some(remainder) = trimmed.strip_prefix("moral_trace.") {
        remainder
    } else if let Some(remainder) = trimmed.strip_prefix("outcome_linkage.") {
        remainder
    } else {
        return Err(anyhow!(
            "{field} must derive from explicit moral_trace or outcome_linkage fields"
        ));
    };
    if suffix.is_empty() {
        return Err(anyhow!("{field} must include a concrete field path"));
    }
    if trimmed.contains('/') || trimmed.contains('\\') {
        return Err(anyhow!("{field} must not contain host-path separators"));
    }
    for segment in suffix.split('.') {
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
    let suffix = &trimmed[prefix.len()..];
    if suffix.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
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

fn require_exact(value: &str, expected: &str, field: &str) -> Result<()> {
    if value == expected {
        Ok(())
    } else {
        Err(anyhow!("{field} must equal {expected}"))
    }
}
