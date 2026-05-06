//! Runtime-v2 outcome linkage and attribution contract.
//!
//! WP-05 consumes WP-04 moral traces and adds a bounded review surface for
//! downstream consequences. The contract preserves uncertainty explicitly so
//! later review, metric, and anti-harm work can build on evidence rather than
//! pretending every consequence is fully known.

use super::*;

pub const OUTCOME_LINKAGE_SCHEMA_VERSION: &str = "outcome_linkage.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutcomeLinkageRecord {
    pub schema_version: String,
    pub linkage_id: String,
    pub source_trace: MoralTraceRecord,
    pub attribution: OutcomeLinkageAttribution,
    pub linked_outcomes: Vec<LinkedOutcome>,
    pub review_refs: OutcomeLinkageReviewRefs,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutcomeLinkageAttribution {
    pub accountable_actor_ref: String,
    pub authority_ref: String,
    pub delegated_by_trace_ref: Option<String>,
    pub delegate_trace_ref: Option<String>,
    pub policy_contribution_refs: Vec<String>,
    pub tool_contribution_refs: Vec<String>,
    pub environment_contribution_refs: Vec<String>,
    pub reviewer_chain_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkedOutcome {
    pub outcome_ref: String,
    pub outcome_status: String,
    pub effect_summary: String,
    pub causal_posture: String,
    pub evidence_refs: Vec<String>,
    pub uncertainty_refs: Vec<String>,
    pub rebuttal_refs: Vec<String>,
    pub downstream_actor_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutcomeLinkageReviewRefs {
    pub review_packet_refs: Vec<String>,
    pub trajectory_review_refs: Vec<String>,
    pub metric_refs: Vec<String>,
    pub challenge_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeLinkageExampleKind {
    Known,
    Unknown,
    Partial,
    Delayed,
    Contested,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutcomeLinkageExample {
    pub example_id: String,
    pub example_kind: OutcomeLinkageExampleKind,
    pub summary: String,
    pub record: OutcomeLinkageRecord,
}

pub fn validate_outcome_linkage_record(record: &OutcomeLinkageRecord) -> Result<()> {
    require_exact(
        &record.schema_version,
        OUTCOME_LINKAGE_SCHEMA_VERSION,
        "outcome_linkage.schema_version",
    )?;
    validate_nonempty_text(&record.linkage_id, "outcome_linkage.linkage_id")?;
    validate_moral_trace_record(&record.source_trace)?;
    validate_linkage_attribution(record)?;
    validate_linked_outcomes(&record.linked_outcomes)?;
    validate_review_refs(&record.review_refs)?;
    Ok(())
}

pub fn outcome_linkage_required_examples() -> Vec<OutcomeLinkageExample> {
    vec![
        OutcomeLinkageExample {
            example_id: "outcome-linkage-known-001".to_string(),
            example_kind: OutcomeLinkageExampleKind::Known,
            summary: "Known outcomes keep direct evidence without widening beyond what the trace actually proves.".to_string(),
            record: known_outcome_record(),
        },
        OutcomeLinkageExample {
            example_id: "outcome-linkage-unknown-001".to_string(),
            example_kind: OutcomeLinkageExampleKind::Unknown,
            summary: "Unknown outcomes preserve reviewability by recording open consequence questions rather than guessing.".to_string(),
            record: unknown_outcome_record(),
        },
        OutcomeLinkageExample {
            example_id: "outcome-linkage-partial-001".to_string(),
            example_kind: OutcomeLinkageExampleKind::Partial,
            summary: "Partial outcomes can cite observed effects while keeping unresolved causal boundaries explicit.".to_string(),
            record: partial_outcome_record(),
        },
        OutcomeLinkageExample {
            example_id: "outcome-linkage-delayed-001".to_string(),
            example_kind: OutcomeLinkageExampleKind::Delayed,
            summary: "Delayed outcomes keep pending-review posture instead of manufacturing a present-tense result.".to_string(),
            record: delayed_outcome_record(),
        },
        OutcomeLinkageExample {
            example_id: "outcome-linkage-contested-001".to_string(),
            example_kind: OutcomeLinkageExampleKind::Contested,
            summary: "Contested delegated outcomes preserve rebuttal evidence and visible attribution lineage.".to_string(),
            record: contested_outcome_record(),
        },
    ]
}

pub fn validate_outcome_linkage_examples(examples: &[OutcomeLinkageExample]) -> Result<()> {
    if examples.len() != 5 {
        return Err(anyhow!(
            "outcome linkage examples must include exactly five required examples"
        ));
    }

    let mut kinds = std::collections::BTreeSet::new();
    for example in examples {
        validate_nonempty_text(&example.example_id, "outcome_linkage.example_id")?;
        validate_nonempty_text(&example.summary, "outcome_linkage.summary")?;
        validate_outcome_linkage_record(&example.record)?;
        kinds.insert(example.example_kind.clone());
    }

    let required = [
        OutcomeLinkageExampleKind::Known,
        OutcomeLinkageExampleKind::Unknown,
        OutcomeLinkageExampleKind::Partial,
        OutcomeLinkageExampleKind::Delayed,
        OutcomeLinkageExampleKind::Contested,
    ];
    for kind in required {
        if !kinds.contains(&kind) {
            return Err(anyhow!("missing required outcome linkage example kind"));
        }
    }
    Ok(())
}

pub fn canonical_outcome_linkage_json_bytes(record: &OutcomeLinkageRecord) -> Result<Vec<u8>> {
    validate_outcome_linkage_record(record)?;
    let mut canonical = record.clone();
    canonicalize_outcome_linkage_record(&mut canonical);
    serde_json::to_vec_pretty(&canonical).context("serialize outcome linkage canonical json")
}

fn validate_linkage_attribution(record: &OutcomeLinkageRecord) -> Result<()> {
    let attribution = &record.attribution;
    validate_nonempty_text(
        &attribution.accountable_actor_ref,
        "outcome_linkage.attribution.accountable_actor_ref",
    )?;
    validate_nonempty_text(
        &attribution.authority_ref,
        "outcome_linkage.attribution.authority_ref",
    )?;
    require_optional_ref_list(
        &attribution.policy_contribution_refs,
        "outcome_linkage.attribution.policy_contribution_refs",
    )?;
    require_optional_ref_list(
        &attribution.tool_contribution_refs,
        "outcome_linkage.attribution.tool_contribution_refs",
    )?;
    require_optional_ref_list(
        &attribution.environment_contribution_refs,
        "outcome_linkage.attribution.environment_contribution_refs",
    )?;
    require_optional_ref_list(
        &attribution.reviewer_chain_refs,
        "outcome_linkage.attribution.reviewer_chain_refs",
    )?;

    if attribution.accountable_actor_ref != record.source_trace.attribution.accountable_actor_ref {
        return Err(anyhow!(
            "outcome_linkage.attribution accountable_actor_ref must match source trace attribution"
        ));
    }
    if attribution.authority_ref != record.source_trace.attribution.authority_ref {
        return Err(anyhow!(
            "outcome_linkage.attribution authority_ref must match source trace attribution"
        ));
    }

    let trace_delegate_ref = record
        .source_trace
        .attribution
        .delegate_trace_ref
        .as_deref();
    let trace_parent_ref = record
        .source_trace
        .attribution
        .delegated_by_trace_ref
        .as_deref();
    match record.source_trace.outcome.outcome_kind.as_str() {
        "delegated" => {
            require_some_ref(
                attribution.delegated_by_trace_ref.as_deref(),
                "outcome_linkage.attribution.delegated_by_trace_ref",
            )?;
            require_some_ref(
                attribution.delegate_trace_ref.as_deref(),
                "outcome_linkage.attribution.delegate_trace_ref",
            )?;
            if attribution.delegate_trace_ref.as_deref() != trace_delegate_ref
                || attribution.delegated_by_trace_ref.as_deref() != trace_parent_ref
            {
                return Err(anyhow!(
                    "outcome_linkage.attribution delegated outcomes must retain the source trace delegation lineage"
                ));
            }
        }
        _ => {
            if attribution.delegated_by_trace_ref.is_some()
                || attribution.delegate_trace_ref.is_some()
            {
                return Err(anyhow!(
                    "outcome_linkage.attribution delegation refs require a delegated source trace"
                ));
            }
        }
    }
    Ok(())
}

fn validate_linked_outcomes(outcomes: &[LinkedOutcome]) -> Result<()> {
    if outcomes.is_empty() {
        return Err(anyhow!("outcome_linkage.linked_outcomes must not be empty"));
    }

    let mut seen = std::collections::BTreeSet::new();
    for outcome in outcomes {
        validate_ref_value(
            &outcome.outcome_ref,
            "outcome_linkage.linked_outcomes.outcome_ref",
        )?;
        validate_nonempty_text(
            &outcome.effect_summary,
            "outcome_linkage.linked_outcomes.effect_summary",
        )?;
        require_optional_ref_list(
            &outcome.evidence_refs,
            "outcome_linkage.linked_outcomes.evidence_refs",
        )?;
        require_optional_ref_list(
            &outcome.uncertainty_refs,
            "outcome_linkage.linked_outcomes.uncertainty_refs",
        )?;
        require_optional_ref_list(
            &outcome.rebuttal_refs,
            "outcome_linkage.linked_outcomes.rebuttal_refs",
        )?;
        require_optional_ref_list(
            &outcome.downstream_actor_refs,
            "outcome_linkage.linked_outcomes.downstream_actor_refs",
        )?;
        validate_outcome_status(outcome)?;
        if !seen.insert(outcome.outcome_ref.clone()) {
            return Err(anyhow!(
                "outcome_linkage.linked_outcomes must not contain duplicate outcome_ref"
            ));
        }
    }
    Ok(())
}

fn validate_outcome_status(outcome: &LinkedOutcome) -> Result<()> {
    match outcome.outcome_status.as_str() {
        "known" | "unknown" | "partial" | "delayed" | "contested" => {}
        _ => {
            return Err(anyhow!(
                "outcome_linkage.linked_outcomes.outcome_status unsupported"
            ))
        }
    }
    match outcome.causal_posture.as_str() {
        "evidenced" | "inferred" | "pending_review" | "contested" | "none" => {}
        _ => {
            return Err(anyhow!(
                "outcome_linkage.linked_outcomes.causal_posture unsupported"
            ))
        }
    }

    match outcome.outcome_status.as_str() {
        "known" => {
            if outcome.evidence_refs.is_empty() {
                return Err(anyhow!(
                    "outcome_linkage.linked_outcomes known outcomes require evidence_refs"
                ));
            }
            if outcome.causal_posture != "evidenced" && outcome.causal_posture != "inferred" {
                return Err(anyhow!(
                    "outcome_linkage.linked_outcomes known outcomes require evidenced or inferred posture"
                ));
            }
        }
        "unknown" => {
            if outcome.causal_posture != "none" {
                return Err(anyhow!(
                    "outcome_linkage.linked_outcomes unknown outcomes must not collapse uncertainty into a causal claim"
                ));
            }
            if outcome.uncertainty_refs.is_empty() {
                return Err(anyhow!(
                    "outcome_linkage.linked_outcomes unknown outcomes require uncertainty_refs"
                ));
            }
            if !outcome.evidence_refs.is_empty() {
                return Err(anyhow!(
                    "outcome_linkage.linked_outcomes unknown outcomes must not claim direct outcome evidence"
                ));
            }
        }
        "partial" => {
            if outcome.causal_posture != "inferred" && outcome.causal_posture != "pending_review" {
                return Err(anyhow!(
                    "outcome_linkage.linked_outcomes partial outcomes must preserve incomplete causality"
                ));
            }
            if outcome.evidence_refs.is_empty() || outcome.uncertainty_refs.is_empty() {
                return Err(anyhow!(
                    "outcome_linkage.linked_outcomes partial outcomes require both evidence_refs and uncertainty_refs"
                ));
            }
        }
        "delayed" => {
            if outcome.causal_posture != "pending_review" {
                return Err(anyhow!(
                    "outcome_linkage.linked_outcomes delayed outcomes require pending_review posture"
                ));
            }
            if outcome.uncertainty_refs.is_empty() {
                return Err(anyhow!(
                    "outcome_linkage.linked_outcomes delayed outcomes require uncertainty_refs"
                ));
            }
        }
        "contested" => {
            if outcome.causal_posture != "contested" {
                return Err(anyhow!(
                    "outcome_linkage.linked_outcomes contested outcomes require contested posture"
                ));
            }
            if outcome.rebuttal_refs.is_empty() || outcome.uncertainty_refs.is_empty() {
                return Err(anyhow!(
                    "outcome_linkage.linked_outcomes contested outcomes require rebuttal_refs and uncertainty_refs"
                ));
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn validate_review_refs(review_refs: &OutcomeLinkageReviewRefs) -> Result<()> {
    require_optional_ref_list(
        &review_refs.review_packet_refs,
        "outcome_linkage.review_refs.review_packet_refs",
    )?;
    require_optional_ref_list(
        &review_refs.trajectory_review_refs,
        "outcome_linkage.review_refs.trajectory_review_refs",
    )?;
    require_optional_ref_list(
        &review_refs.metric_refs,
        "outcome_linkage.review_refs.metric_refs",
    )?;
    if let Some(challenge_ref) = &review_refs.challenge_ref {
        validate_ref_value(challenge_ref, "outcome_linkage.review_refs.challenge_ref")?;
    }

    let has_review_path = !review_refs.review_packet_refs.is_empty()
        || !review_refs.trajectory_review_refs.is_empty()
        || !review_refs.metric_refs.is_empty()
        || review_refs.challenge_ref.is_some();
    if !has_review_path {
        return Err(anyhow!(
            "outcome_linkage must preserve a review packet, trajectory, metric, or challenge path"
        ));
    }
    Ok(())
}

fn canonicalize_outcome_linkage_record(record: &mut OutcomeLinkageRecord) {
    canonicalize_moral_trace_record(&mut record.source_trace);
    record.attribution.policy_contribution_refs.sort();
    record.attribution.tool_contribution_refs.sort();
    record.attribution.environment_contribution_refs.sort();
    record.attribution.reviewer_chain_refs.sort();
    for outcome in &mut record.linked_outcomes {
        outcome.evidence_refs.sort();
        outcome.uncertainty_refs.sort();
        outcome.rebuttal_refs.sort();
        outcome.downstream_actor_refs.sort();
    }
    record
        .linked_outcomes
        .sort_by(|left, right| left.outcome_ref.cmp(&right.outcome_ref));
    record.review_refs.review_packet_refs.sort();
    record.review_refs.trajectory_review_refs.sort();
    record.review_refs.metric_refs.sort();
}

fn known_outcome_record() -> OutcomeLinkageRecord {
    let source_trace = trace_example(MoralTraceExampleKind::OrdinaryAction);
    OutcomeLinkageRecord {
        schema_version: OUTCOME_LINKAGE_SCHEMA_VERSION.to_string(),
        linkage_id: "outcome_linkage_wp05_known_001".to_string(),
        attribution: attribution_from_trace(&source_trace),
        linked_outcomes: vec![LinkedOutcome {
            outcome_ref: "outcome:summary-accepted".to_string(),
            outcome_status: "known".to_string(),
            effect_summary: "The bounded summary was accepted as reviewer-safe output.".to_string(),
            causal_posture: "evidenced".to_string(),
            evidence_refs: vec![
                "artifact:summary-output".to_string(),
                "artifact:review-acceptance".to_string(),
            ],
            uncertainty_refs: Vec::new(),
            rebuttal_refs: Vec::new(),
            downstream_actor_refs: vec!["reviewer:operator".to_string()],
        }],
        review_refs: OutcomeLinkageReviewRefs {
            review_packet_refs: vec!["review:wp05-known".to_string()],
            trajectory_review_refs: vec!["trajectory:single-event-known".to_string()],
            metric_refs: vec!["metric:known-outcome-coverage".to_string()],
            challenge_ref: None,
        },
        source_trace,
    }
}

fn unknown_outcome_record() -> OutcomeLinkageRecord {
    let source_trace = trace_example(MoralTraceExampleKind::Refusal);
    OutcomeLinkageRecord {
        schema_version: OUTCOME_LINKAGE_SCHEMA_VERSION.to_string(),
        linkage_id: "outcome_linkage_wp05_unknown_001".to_string(),
        attribution: attribution_from_trace(&source_trace),
        linked_outcomes: vec![LinkedOutcome {
            outcome_ref: "outcome:operator-follow-on-unknown".to_string(),
            outcome_status: "unknown".to_string(),
            effect_summary: "The later operator response was intentionally left unknown pending separate review evidence.".to_string(),
            causal_posture: "none".to_string(),
            evidence_refs: Vec::new(),
            uncertainty_refs: vec![
                "question:operator-follow-on-not-observed".to_string(),
                "review:separate-review-pending".to_string(),
            ],
            rebuttal_refs: Vec::new(),
            downstream_actor_refs: vec!["operator:requester".to_string()],
        }],
        review_refs: OutcomeLinkageReviewRefs {
            review_packet_refs: vec!["review:wp05-unknown".to_string()],
            trajectory_review_refs: vec!["trajectory:refusal-follow-on-open".to_string()],
            metric_refs: vec!["metric:unknown-outcome-count".to_string()],
            challenge_ref: None,
        },
        source_trace,
    }
}

fn partial_outcome_record() -> OutcomeLinkageRecord {
    let source_trace = trace_example(MoralTraceExampleKind::OrdinaryAction);
    OutcomeLinkageRecord {
        schema_version: OUTCOME_LINKAGE_SCHEMA_VERSION.to_string(),
        linkage_id: "outcome_linkage_wp05_partial_001".to_string(),
        attribution: attribution_from_trace(&source_trace),
        linked_outcomes: vec![LinkedOutcome {
            outcome_ref: "outcome:summary-used-in-review".to_string(),
            outcome_status: "partial".to_string(),
            effect_summary: "One reviewer used the summary, but downstream circulation beyond that review lane is still unresolved.".to_string(),
            causal_posture: "inferred".to_string(),
            evidence_refs: vec!["artifact:review-acceptance".to_string()],
            uncertainty_refs: vec!["question:distribution-beyond-reviewer-lane".to_string()],
            rebuttal_refs: Vec::new(),
            downstream_actor_refs: vec![
                "reviewer:operator".to_string(),
                "reviewer:governance".to_string(),
            ],
        }],
        review_refs: OutcomeLinkageReviewRefs {
            review_packet_refs: vec!["review:wp05-partial".to_string()],
            trajectory_review_refs: vec!["trajectory:summary-partial-propagation".to_string()],
            metric_refs: vec!["metric:partial-outcome-count".to_string()],
            challenge_ref: None,
        },
        source_trace,
    }
}

fn delayed_outcome_record() -> OutcomeLinkageRecord {
    let source_trace = trace_example(MoralTraceExampleKind::DeferredDecision);
    OutcomeLinkageRecord {
        schema_version: OUTCOME_LINKAGE_SCHEMA_VERSION.to_string(),
        linkage_id: "outcome_linkage_wp05_delayed_001".to_string(),
        attribution: attribution_from_trace(&source_trace),
        linked_outcomes: vec![LinkedOutcome {
            outcome_ref: "outcome:affected-party-evidence-pending".to_string(),
            outcome_status: "delayed".to_string(),
            effect_summary: "The deferral should only resolve after affected-party evidence is gathered in a later review window.".to_string(),
            causal_posture: "pending_review".to_string(),
            evidence_refs: vec!["artifact:handoff-request".to_string()],
            uncertainty_refs: vec![
                "artifact:affected-party-identity".to_string(),
                "artifact:consent-or-challenge-route".to_string(),
            ],
            rebuttal_refs: Vec::new(),
            downstream_actor_refs: vec!["reviewer:governance".to_string()],
        }],
        review_refs: OutcomeLinkageReviewRefs {
            review_packet_refs: vec!["review:wp05-delayed".to_string()],
            trajectory_review_refs: vec!["trajectory:deferred-follow-on".to_string()],
            metric_refs: vec!["metric:delayed-outcome-count".to_string()],
            challenge_ref: Some("challenge:missing-affected-party".to_string()),
        },
        source_trace,
    }
}

fn contested_outcome_record() -> OutcomeLinkageRecord {
    let source_trace = trace_example(MoralTraceExampleKind::Delegation);
    OutcomeLinkageRecord {
        schema_version: OUTCOME_LINKAGE_SCHEMA_VERSION.to_string(),
        linkage_id: "outcome_linkage_wp05_contested_001".to_string(),
        attribution: attribution_from_trace(&source_trace),
        linked_outcomes: vec![LinkedOutcome {
            outcome_ref: "outcome:delegate-safe-read-contested".to_string(),
            outcome_status: "contested".to_string(),
            effect_summary: "The delegated lookup result is disputed until reviewer-visible rebuttal evidence is resolved.".to_string(),
            causal_posture: "contested".to_string(),
            evidence_refs: vec!["artifact:delegation-scope".to_string()],
            uncertainty_refs: vec!["review:delegate-result-under-contest".to_string()],
            rebuttal_refs: vec!["artifact:delegate-rebuttal-summary".to_string()],
            downstream_actor_refs: vec![
                "agent:delegate-helper".to_string(),
                "reviewer:operator".to_string(),
            ],
        }],
        review_refs: OutcomeLinkageReviewRefs {
            review_packet_refs: vec!["review:wp05-contested".to_string()],
            trajectory_review_refs: vec!["trajectory:delegation-contest".to_string()],
            metric_refs: vec!["metric:contested-outcome-count".to_string()],
            challenge_ref: Some("challenge:delegate-safe-read-review".to_string()),
        },
        source_trace,
    }
}

fn attribution_from_trace(trace: &MoralTraceRecord) -> OutcomeLinkageAttribution {
    OutcomeLinkageAttribution {
        accountable_actor_ref: trace.attribution.accountable_actor_ref.clone(),
        authority_ref: trace.attribution.authority_ref.clone(),
        delegated_by_trace_ref: trace.attribution.delegated_by_trace_ref.clone(),
        delegate_trace_ref: trace.attribution.delegate_trace_ref.clone(),
        policy_contribution_refs: trace
            .moral_event
            .policy_context
            .governing_policy_refs
            .clone(),
        tool_contribution_refs: vec!["tool:none".to_string()],
        environment_contribution_refs: vec!["environment:local-reviewed-lane".to_string()],
        reviewer_chain_refs: trace.attribution.reviewer_chain_refs.clone(),
    }
}

fn trace_example(kind: MoralTraceExampleKind) -> MoralTraceRecord {
    moral_trace_required_examples()
        .into_iter()
        .find(|example| example.example_kind == kind)
        .expect("required moral trace example")
        .trace
}

fn require_exact(value: &str, expected: &str, field: &str) -> Result<()> {
    if value != expected {
        return Err(anyhow!("{field} unsupported"));
    }
    Ok(())
}

fn require_optional_ref_list(values: &[String], field: &str) -> Result<()> {
    for value in values {
        validate_ref_value(value, field)?;
    }
    Ok(())
}

fn require_some_ref(value: Option<&str>, field: &str) -> Result<()> {
    let value = value.ok_or_else(|| anyhow!("{field} must not be empty"))?;
    validate_ref_value(value, field)
}

fn validate_ref_value(value: &str, field: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    if trimmed.starts_with('/')
        || trimmed.contains('\\')
        || trimmed.contains("file://")
        || contains_windows_host_path(trimmed)
    {
        return Err(anyhow!("{field} must not contain host paths"));
    }
    Ok(())
}

fn contains_windows_host_path(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && (bytes[2] == b'\\' || bytes[2] == b'/')
}
