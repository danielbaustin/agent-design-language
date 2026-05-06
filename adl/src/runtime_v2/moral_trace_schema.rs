//! Runtime-v2 moral trace schema.
//!
//! WP-04 adds the bounded trace contract that consumes the WP-02 moral event
//! record and extends it with trace-only outcome, attribution, visibility, and
//! review-reference surfaces. The contract is reviewable without requiring
//! public exposure of private state.

use super::*;

pub const MORAL_TRACE_SCHEMA_VERSION: &str = "moral_trace.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralTraceRecord {
    pub schema_version: String,
    pub trace_id: String,
    pub trace_sequence: u32,
    pub moral_event: MoralEventRecord,
    pub outcome: MoralTraceOutcome,
    pub attribution: MoralTraceAttribution,
    pub visibility: MoralTraceVisibility,
    pub review_refs: MoralTraceReviewRefs,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralTraceOutcome {
    pub outcome_kind: String,
    pub outcome_summary: String,
    pub outcome_evidence_refs: Vec<String>,
    pub downstream_effect_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralTraceAttribution {
    pub accountable_actor_ref: String,
    pub authority_ref: String,
    pub delegated_by_trace_ref: Option<String>,
    pub delegate_trace_ref: Option<String>,
    pub reviewer_chain_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralTraceVisibility {
    pub public_disclosure: String,
    pub public_summary: Option<String>,
    pub reviewer_evidence_refs: Vec<String>,
    pub governance_evidence_refs: Vec<String>,
    pub public_evidence_refs: Vec<String>,
    pub private_state_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralTraceReviewRefs {
    pub challenge_ref: Option<String>,
    pub review_packet_refs: Vec<String>,
    pub outcome_link_refs: Vec<String>,
    pub metric_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum MoralTraceExampleKind {
    OrdinaryAction,
    Refusal,
    Delegation,
    DeferredDecision,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralTraceExample {
    pub example_id: String,
    pub example_kind: MoralTraceExampleKind,
    pub summary: String,
    pub trace: MoralTraceRecord,
}

pub fn validate_moral_trace_record(record: &MoralTraceRecord) -> Result<()> {
    require_exact(
        &record.schema_version,
        MORAL_TRACE_SCHEMA_VERSION,
        "schema_version",
    )?;
    validate_nonempty_text(&record.trace_id, "moral_trace.trace_id")?;
    if record.trace_sequence == 0 {
        return Err(anyhow!("moral_trace.trace_sequence must be positive"));
    }

    validate_moral_event(&record.moral_event)?;
    validate_outcome(record)?;
    validate_attribution(record)?;
    validate_visibility(&record.visibility)?;
    validate_reviewability(record)?;

    Ok(())
}

pub fn moral_trace_required_examples() -> Vec<MoralTraceExample> {
    vec![
        MoralTraceExample {
            example_id: "moral-trace-ordinary-action-001".to_string(),
            example_kind: MoralTraceExampleKind::OrdinaryAction,
            summary: "Ordinary action preserves reviewer evidence and only exposes a public-safe summary.".to_string(),
            trace: ordinary_action_trace(),
        },
        MoralTraceExample {
            example_id: "moral-trace-refusal-001".to_string(),
            example_kind: MoralTraceExampleKind::Refusal,
            summary: "Refusal records the denied path, reviewer evidence, and public redaction boundary.".to_string(),
            trace: refusal_trace(),
        },
        MoralTraceExample {
            example_id: "moral-trace-delegation-001".to_string(),
            example_kind: MoralTraceExampleKind::Delegation,
            summary: "Delegation preserves parent accountability and links the delegate trace explicitly.".to_string(),
            trace: delegation_trace(),
        },
        MoralTraceExample {
            example_id: "moral-trace-deferred-decision-001".to_string(),
            example_kind: MoralTraceExampleKind::DeferredDecision,
            summary: "Deferred decisions stay reviewable under missing evidence without turning the deferral into a refusal.".to_string(),
            trace: deferred_decision_trace(),
        },
    ]
}

pub fn validate_moral_trace_examples(examples: &[MoralTraceExample]) -> Result<()> {
    if examples.len() != 4 {
        return Err(anyhow!(
            "moral trace examples must include exactly four required examples"
        ));
    }

    let mut kinds = std::collections::BTreeSet::new();
    for example in examples {
        validate_nonempty_text(&example.example_id, "moral_trace.example_id")?;
        validate_nonempty_text(&example.summary, "moral_trace.summary")?;
        validate_moral_trace_record(&example.trace)?;
        kinds.insert(example.example_kind.clone());
    }

    let required = [
        MoralTraceExampleKind::OrdinaryAction,
        MoralTraceExampleKind::Refusal,
        MoralTraceExampleKind::Delegation,
        MoralTraceExampleKind::DeferredDecision,
    ];
    for kind in required {
        if !kinds.contains(&kind) {
            return Err(anyhow!("missing required moral trace example kind"));
        }
    }
    Ok(())
}

pub fn canonical_moral_trace_json_bytes(record: &MoralTraceRecord) -> Result<Vec<u8>> {
    validate_moral_trace_record(record)?;
    let mut canonical = record.clone();
    canonicalize_moral_trace_record(&mut canonical);
    serde_json::to_vec_pretty(&canonical).context("serialize moral trace canonical json")
}

fn validate_outcome(record: &MoralTraceRecord) -> Result<()> {
    match record.outcome.outcome_kind.as_str() {
        "completed" | "refused" | "delegated" | "deferred" | "challenged" => {}
        _ => return Err(anyhow!("moral_trace.outcome.outcome_kind unsupported")),
    }
    validate_nonempty_text(
        &record.outcome.outcome_summary,
        "moral_trace.outcome.outcome_summary",
    )?;
    require_ref_list(
        &record.outcome.outcome_evidence_refs,
        "moral_trace.outcome.outcome_evidence_refs",
    )?;
    require_optional_ref_list(
        &record.outcome.downstream_effect_refs,
        "moral_trace.outcome.downstream_effect_refs",
    )?;

    match record.outcome.outcome_kind.as_str() {
        "refused"
            if !record.moral_event.refusal.refused || record.moral_event.event_kind != "denied" =>
        {
            return Err(anyhow!(
                "moral_trace.outcome refused must bind to a denied moral event"
            ));
        }
        "delegated" => {
            let delegated = record
                .moral_event
                .delegation_context
                .as_ref()
                .map(|ctx| ctx.delegated)
                .unwrap_or(false);
            if !delegated {
                return Err(anyhow!(
                    "moral_trace.outcome delegated must bind to an active delegation context"
                ));
            }
        }
        "deferred" if record.moral_event.event_kind != "deferred" => {
            return Err(anyhow!(
                "moral_trace.outcome deferred must bind to a deferred moral event"
            ));
        }
        "challenged" if record.moral_event.event_kind != "challenged" => {
            return Err(anyhow!(
                "moral_trace.outcome challenged must bind to a challenged moral event"
            ));
        }
        "completed" if record.moral_event.refusal.refused => {
            return Err(anyhow!(
                "moral_trace.outcome completed must not bind to a refusal"
            ));
        }
        _ => {}
    }
    Ok(())
}

fn validate_attribution(record: &MoralTraceRecord) -> Result<()> {
    validate_nonempty_text(
        &record.attribution.accountable_actor_ref,
        "moral_trace.attribution.accountable_actor_ref",
    )?;
    validate_nonempty_text(
        &record.attribution.authority_ref,
        "moral_trace.attribution.authority_ref",
    )?;
    require_optional_ref_list(
        &record.attribution.reviewer_chain_refs,
        "moral_trace.attribution.reviewer_chain_refs",
    )?;

    if record.attribution.accountable_actor_ref != record.moral_event.accountable_identity.actor_id
    {
        return Err(anyhow!(
            "moral_trace.attribution accountable_actor_ref must match moral_event accountable identity"
        ));
    }
    if record.attribution.authority_ref != record.moral_event.accountable_identity.authority_ref {
        return Err(anyhow!(
            "moral_trace.attribution authority_ref must match moral_event authority_ref"
        ));
    }

    let delegated = record
        .moral_event
        .delegation_context
        .as_ref()
        .map(|ctx| ctx.delegated)
        .unwrap_or(false);
    if delegated {
        let context = record
            .moral_event
            .delegation_context
            .as_ref()
            .expect("delegated context must exist");
        require_some_ref(
            record.attribution.delegated_by_trace_ref.as_deref(),
            "moral_trace.attribution.delegated_by_trace_ref",
        )?;
        require_some_ref(
            record.attribution.delegate_trace_ref.as_deref(),
            "moral_trace.attribution.delegate_trace_ref",
        )?;
        if record.attribution.delegate_trace_ref.as_deref()
            != Some(context.delegate_trace_ref.as_str())
        {
            return Err(anyhow!(
                "moral_trace.attribution delegate_trace_ref must match moral_event delegation_context"
            ));
        }
    } else if record.attribution.delegated_by_trace_ref.is_some()
        || record.attribution.delegate_trace_ref.is_some()
    {
        return Err(anyhow!(
            "moral_trace.attribution delegation refs require an active delegation context"
        ));
    }
    Ok(())
}

fn validate_visibility(visibility: &MoralTraceVisibility) -> Result<()> {
    match visibility.public_disclosure.as_str() {
        "none" | "summary_only" | "redacted" => {}
        _ => {
            return Err(anyhow!(
                "moral_trace.visibility.public_disclosure unsupported"
            ))
        }
    }

    if let Some(summary) = &visibility.public_summary {
        validate_nonempty_text(summary, "moral_trace.visibility.public_summary")?;
        reject_sensitive_public_text(summary, "moral_trace.visibility.public_summary")?;
    }

    require_optional_ref_list(
        &visibility.reviewer_evidence_refs,
        "moral_trace.visibility.reviewer_evidence_refs",
    )?;
    require_optional_ref_list(
        &visibility.governance_evidence_refs,
        "moral_trace.visibility.governance_evidence_refs",
    )?;
    require_optional_ref_list(
        &visibility.public_evidence_refs,
        "moral_trace.visibility.public_evidence_refs",
    )?;
    require_optional_ref_list(
        &visibility.private_state_refs,
        "moral_trace.visibility.private_state_refs",
    )?;

    for public_ref in &visibility.public_evidence_refs {
        reject_sensitive_public_text(public_ref, "moral_trace.visibility.public_evidence_refs")?;
    }
    if visibility.public_disclosure == "none"
        && (visibility.public_summary.is_some() || !visibility.public_evidence_refs.is_empty())
    {
        return Err(anyhow!(
            "moral_trace.visibility none must not expose public summary or public evidence refs"
        ));
    }
    Ok(())
}

fn validate_reviewability(record: &MoralTraceRecord) -> Result<()> {
    require_optional_ref_list(
        &record.review_refs.review_packet_refs,
        "moral_trace.review_refs.review_packet_refs",
    )?;
    require_optional_ref_list(
        &record.review_refs.outcome_link_refs,
        "moral_trace.review_refs.outcome_link_refs",
    )?;
    require_optional_ref_list(
        &record.review_refs.metric_refs,
        "moral_trace.review_refs.metric_refs",
    )?;
    if let Some(challenge_ref) = &record.review_refs.challenge_ref {
        validate_nonempty_text(challenge_ref, "moral_trace.review_refs.challenge_ref")?;
    }

    let has_reviewer_path = !record.visibility.reviewer_evidence_refs.is_empty()
        || !record.visibility.governance_evidence_refs.is_empty()
        || !record.review_refs.review_packet_refs.is_empty()
        || record.review_refs.challenge_ref.is_some();
    if !has_reviewer_path {
        return Err(anyhow!(
            "moral_trace must preserve a reviewer or governance review path"
        ));
    }

    if record.moral_event.event_kind == "challenged"
        && record
            .review_refs
            .challenge_ref
            .as_deref()
            .unwrap_or("")
            .is_empty()
    {
        return Err(anyhow!(
            "moral_trace challenged events require review_refs.challenge_ref"
        ));
    }
    Ok(())
}

fn canonicalize_moral_trace_record(record: &mut MoralTraceRecord) {
    record.moral_event.choice.decision_basis.sort();
    record
        .moral_event
        .alternatives
        .considered
        .sort_by(|left, right| left.action.cmp(&right.action));
    record
        .moral_event
        .alternatives
        .omitted_known_alternatives
        .sort_by(|left, right| left.action.cmp(&right.action));
    record.moral_event.affected_parties.direct.sort();
    record.moral_event.affected_parties.indirect.sort();
    record
        .moral_event
        .affected_parties
        .unknown_or_unrepresented
        .sort();
    record.moral_event.evidence.supporting_refs.sort();
    record.moral_event.evidence.missing_evidence.sort();
    record
        .moral_event
        .policy_context
        .governing_policy_refs
        .sort();
    record
        .moral_event
        .policy_context
        .safety_boundary_refs
        .sort();
    record.outcome.outcome_evidence_refs.sort();
    record.outcome.downstream_effect_refs.sort();
    record.attribution.reviewer_chain_refs.sort();
    record.visibility.reviewer_evidence_refs.sort();
    record.visibility.governance_evidence_refs.sort();
    record.visibility.public_evidence_refs.sort();
    record.visibility.private_state_refs.sort();
    record.review_refs.review_packet_refs.sort();
    record.review_refs.outcome_link_refs.sort();
    record.review_refs.metric_refs.sort();
}

fn require_exact(value: &str, expected: &str, field: &str) -> Result<()> {
    if value != expected {
        return Err(anyhow!("{field} unsupported"));
    }
    Ok(())
}

fn require_ref_list(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    require_optional_ref_list(values, field)
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
    if trimmed.starts_with('/') || trimmed.contains('\\') {
        return Err(anyhow!("{field} must not contain host paths"));
    }
    Ok(())
}

fn reject_sensitive_public_text(value: &str, field: &str) -> Result<()> {
    let lowered = value.to_ascii_lowercase();
    if lowered.contains("private_state")
        || lowered.contains("raw_private_state")
        || lowered.contains("private diagnostic")
        || lowered.contains("/users/")
        || lowered.contains("/home/")
        || contains_windows_host_path(value)
    {
        return Err(anyhow!(
            "{field} must not expose private-state markers publicly"
        ));
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

fn ordinary_action_trace() -> MoralTraceRecord {
    let mut event = base_event("allowed");
    event.choice.requested_action = "summarize a non-sensitive planning note".to_string();
    event.choice.selected_action =
        "provide bounded summary with no private diagnostics".to_string();
    event.choice.decision_basis = vec![
        "evidence:source-note-classified-internal".to_string(),
        "policy:bounded-summary".to_string(),
    ];
    event.alternatives.considered[0].action = "refuse the summary".to_string();
    event.alternatives.considered[0].reason_considered =
        "source could have contained private diagnostics".to_string();
    event.alternatives.considered[0].reason_rejected =
        "classification allowed reviewer-visible summary".to_string();
    event.refusal.refused = false;
    event.refusal.refusal_reason = "none".to_string();
    event.refusal.policy_ref = "policy:bounded-summary".to_string();
    event.uncertainty.level = "low".to_string();
    event.uncertainty.confidence_notes = "source classification was explicit".to_string();
    event.evidence.supporting_refs = vec!["artifact:source-note-classification".to_string()];
    event.policy_context.governing_policy_refs = vec!["policy:bounded-summary".to_string()];
    event.policy_context.safety_boundary_refs = vec!["boundary:no-private-diagnostics".to_string()];
    event.review.reviewer_visibility = "full".to_string();
    event.review.review_required = false;
    event.review.challenge_ref = "none".to_string();
    event.review.human_review_note = "safe summary path selected".to_string();

    MoralTraceRecord {
        schema_version: MORAL_TRACE_SCHEMA_VERSION.to_string(),
        trace_id: "trace_wp04_ordinary_action_001".to_string(),
        trace_sequence: 1,
        moral_event: event,
        outcome: MoralTraceOutcome {
            outcome_kind: "completed".to_string(),
            outcome_summary: "Safe summary produced under bounded reviewer-visible policy."
                .to_string(),
            outcome_evidence_refs: vec!["artifact:summary-output".to_string()],
            downstream_effect_refs: vec!["artifact:reviewable-summary".to_string()],
        },
        attribution: MoralTraceAttribution {
            accountable_actor_ref: "agent:demo-helper".to_string(),
            authority_ref: "policy:bounded-summary".to_string(),
            delegated_by_trace_ref: None,
            delegate_trace_ref: None,
            reviewer_chain_refs: vec!["review:operator-sanity-check".to_string()],
        },
        visibility: MoralTraceVisibility {
            public_disclosure: "summary_only".to_string(),
            public_summary: Some("A bounded non-sensitive summary was produced.".to_string()),
            reviewer_evidence_refs: vec!["artifact:source-note-classification".to_string()],
            governance_evidence_refs: vec!["policy:bounded-summary".to_string()],
            public_evidence_refs: vec!["artifact:public-safe-summary".to_string()],
            private_state_refs: Vec::new(),
        },
        review_refs: MoralTraceReviewRefs {
            challenge_ref: None,
            review_packet_refs: vec!["review:wp04-ordinary-action".to_string()],
            outcome_link_refs: vec!["outcome:summary-accepted".to_string()],
            metric_refs: vec!["metric:reviewability-preserved".to_string()],
        },
    }
}

fn refusal_trace() -> MoralTraceRecord {
    let mut event = base_event("denied");
    event.accountable_identity.authority_ref = "policy:private-diagnostic-boundary".to_string();
    event.choice.requested_action = "disclose private wellbeing diagnostics publicly".to_string();
    event.choice.selected_action =
        "refuse public disclosure and offer redacted review path".to_string();
    event.choice.decision_basis = vec![
        "policy:private-diagnostic-boundary".to_string(),
        "evidence:diagnostic-private-classification".to_string(),
    ];
    event.alternatives.considered = vec![
        MoralEventAlternative {
            action: "disclose full diagnostics".to_string(),
            reason_considered: "operator requested transparency".to_string(),
            reason_rejected: "public exposure of private diagnostics is out of scope".to_string(),
        },
        MoralEventAlternative {
            action: "provide redacted reviewer summary".to_string(),
            reason_considered: "preserves accountability".to_string(),
            reason_rejected: "not rejected; selected as safe alternative".to_string(),
        },
    ];
    event.refusal.refused = true;
    event.refusal.refusal_reason =
        "requested disclosure violates private diagnostic boundary".to_string();
    event.refusal.policy_ref = "policy:private-diagnostic-boundary".to_string();
    event.affected_parties.direct = vec!["citizen:diagnostic-subject".to_string()];
    event.affected_parties.indirect = vec!["reviewer:governance".to_string()];
    event.evidence.supporting_refs = vec!["artifact:diagnostic-classification".to_string()];
    event.evidence.privacy_classification = "private".to_string();
    event.policy_context.governing_policy_refs =
        vec!["policy:private-diagnostic-boundary".to_string()];
    event.policy_context.safety_boundary_refs = vec!["boundary:reviewer-redaction".to_string()];
    event.policy_context.exception_or_override = "rejected".to_string();
    event.review.reviewer_visibility = "redacted".to_string();
    event.review.review_required = true;
    event.review.challenge_ref = "none".to_string();
    event.review.human_review_note =
        "refusal preserves reviewer path without public leakage".to_string();

    MoralTraceRecord {
        schema_version: MORAL_TRACE_SCHEMA_VERSION.to_string(),
        trace_id: "trace_wp04_refusal_001".to_string(),
        trace_sequence: 2,
        moral_event: event,
        outcome: MoralTraceOutcome {
            outcome_kind: "refused".to_string(),
            outcome_summary: "Public disclosure refused; reviewer redaction path preserved."
                .to_string(),
            outcome_evidence_refs: vec!["artifact:redacted-review-summary".to_string()],
            downstream_effect_refs: vec!["outcome:no-public-private-disclosure".to_string()],
        },
        attribution: MoralTraceAttribution {
            accountable_actor_ref: "agent:demo-helper".to_string(),
            authority_ref: "policy:private-diagnostic-boundary".to_string(),
            delegated_by_trace_ref: None,
            delegate_trace_ref: None,
            reviewer_chain_refs: vec!["review:governance-redaction-check".to_string()],
        },
        visibility: MoralTraceVisibility {
            public_disclosure: "redacted".to_string(),
            public_summary: Some("A private disclosure request was refused.".to_string()),
            reviewer_evidence_refs: vec!["artifact:diagnostic-classification".to_string()],
            governance_evidence_refs: vec!["policy:private-diagnostic-boundary".to_string()],
            public_evidence_refs: vec!["artifact:public-refusal-summary".to_string()],
            private_state_refs: vec!["private_state:diagnostic-classification".to_string()],
        },
        review_refs: MoralTraceReviewRefs {
            challenge_ref: None,
            review_packet_refs: vec!["review:wp04-refusal".to_string()],
            outcome_link_refs: vec!["outcome:refusal-recorded".to_string()],
            metric_refs: vec!["metric:privacy-boundary-preserved".to_string()],
        },
    }
}

fn delegation_trace() -> MoralTraceRecord {
    let mut event = base_event("allowed");
    event.accountable_identity.authority_ref = "policy:delegated-safe-read".to_string();
    event.choice.requested_action = "delegate a bounded evidence lookup".to_string();
    event.choice.selected_action = "delegate through a governed reviewer-visible lane".to_string();
    event.choice.decision_basis = vec![
        "policy:delegated-safe-read".to_string(),
        "evidence:operator-approved-scope".to_string(),
    ];
    event.alternatives.considered[0].action = "perform lookup directly".to_string();
    event.alternatives.considered[0].reason_considered =
        "direct execution would be faster".to_string();
    event.alternatives.considered[0].reason_rejected =
        "delegated specialist lane preserves bounded authority".to_string();
    event.review.review_required = true;
    event.review.human_review_note =
        "delegation preserves parent accountability and review linkage".to_string();
    event.review.challenge_ref = "none".to_string();
    event.delegation_context = Some(MoralEventDelegationContext {
        delegated: true,
        delegate_actor_ref: "agent:delegate-helper".to_string(),
        delegate_authority_ref: "policy:delegated-safe-read".to_string(),
        delegate_trace_ref: "trace:delegate-safe-read-001".to_string(),
    });
    event.policy_context.governing_policy_refs = vec!["policy:delegated-safe-read".to_string()];
    event.policy_context.safety_boundary_refs =
        vec!["boundary:delegate-authority-bounded".to_string()];
    event.evidence.supporting_refs = vec!["artifact:delegation-scope".to_string()];

    MoralTraceRecord {
        schema_version: MORAL_TRACE_SCHEMA_VERSION.to_string(),
        trace_id: "trace_wp04_delegation_001".to_string(),
        trace_sequence: 3,
        moral_event: event,
        outcome: MoralTraceOutcome {
            outcome_kind: "delegated".to_string(),
            outcome_summary: "Delegated safe-read path selected with explicit child trace linkage."
                .to_string(),
            outcome_evidence_refs: vec!["artifact:delegation-scope".to_string()],
            downstream_effect_refs: vec!["trace:delegate-safe-read-001".to_string()],
        },
        attribution: MoralTraceAttribution {
            accountable_actor_ref: "agent:demo-helper".to_string(),
            authority_ref: "policy:delegated-safe-read".to_string(),
            delegated_by_trace_ref: Some("trace:parent-delegation-request".to_string()),
            delegate_trace_ref: Some("trace:delegate-safe-read-001".to_string()),
            reviewer_chain_refs: vec!["review:delegation-chain".to_string()],
        },
        visibility: MoralTraceVisibility {
            public_disclosure: "summary_only".to_string(),
            public_summary: Some("A bounded delegated lookup was used.".to_string()),
            reviewer_evidence_refs: vec!["artifact:delegation-scope".to_string()],
            governance_evidence_refs: vec!["policy:delegated-safe-read".to_string()],
            public_evidence_refs: vec!["artifact:delegation-public-summary".to_string()],
            private_state_refs: Vec::new(),
        },
        review_refs: MoralTraceReviewRefs {
            challenge_ref: None,
            review_packet_refs: vec!["review:wp04-delegation".to_string()],
            outcome_link_refs: vec!["outcome:delegate-safe-read-001".to_string()],
            metric_refs: vec!["metric:delegation-accountability-preserved".to_string()],
        },
    }
}

fn deferred_decision_trace() -> MoralTraceRecord {
    let mut event = base_event("deferred");
    event.accountable_identity.authority_ref = "policy:affected-party-review".to_string();
    event.choice.requested_action =
        "approve a delegated action affecting an unnamed party".to_string();
    event.choice.selected_action =
        "defer approval pending affected-party identification".to_string();
    event.choice.decision_basis = vec![
        "policy:affected-party-review".to_string(),
        "evidence:missing-party-context".to_string(),
    ];
    event.alternatives.considered = vec![
        MoralEventAlternative {
            action: "approve immediately".to_string(),
            reason_considered: "request claimed urgency".to_string(),
            reason_rejected: "affected party and harm context were missing".to_string(),
        },
        MoralEventAlternative {
            action: "deny permanently".to_string(),
            reason_considered: "incomplete request could be unsafe".to_string(),
            reason_rejected: "missing context may be repairable".to_string(),
        },
    ];
    event.refusal.refused = false;
    event.refusal.refusal_reason = "deferred rather than refused".to_string();
    event.refusal.policy_ref = "policy:affected-party-review".to_string();
    event.uncertainty.level = "high".to_string();
    event.uncertainty.unresolved_questions = vec![
        "who is affected by the delegated action".to_string(),
        "whether any party can consent or contest".to_string(),
    ];
    event.uncertainty.confidence_notes =
        "action cannot be morally reviewed without party context".to_string();
    event.affected_parties.direct = Vec::new();
    event.affected_parties.indirect = Vec::new();
    event.affected_parties.unknown_or_unrepresented = vec!["unnamed affected party".to_string()];
    event.evidence.supporting_refs = vec!["artifact:handoff-request".to_string()];
    event.evidence.missing_evidence = vec![
        "artifact:affected-party-identity".to_string(),
        "artifact:consent-or-challenge-route".to_string(),
    ];
    event.policy_context.governing_policy_refs = vec!["policy:affected-party-review".to_string()];
    event.policy_context.safety_boundary_refs =
        vec!["boundary:no-hidden-delegated-harm".to_string()];
    event.review.reviewer_visibility = "full".to_string();
    event.review.review_required = true;
    event.review.challenge_ref = "challenge:missing-affected-party".to_string();
    event.review.human_review_note =
        "deferral preserves reversibility until evidence exists".to_string();

    MoralTraceRecord {
        schema_version: MORAL_TRACE_SCHEMA_VERSION.to_string(),
        trace_id: "trace_wp04_deferred_001".to_string(),
        trace_sequence: 4,
        moral_event: event,
        outcome: MoralTraceOutcome {
            outcome_kind: "deferred".to_string(),
            outcome_summary: "Approval deferred pending affected-party and consent evidence."
                .to_string(),
            outcome_evidence_refs: vec!["artifact:handoff-request".to_string()],
            downstream_effect_refs: vec!["review:missing-affected-party".to_string()],
        },
        attribution: MoralTraceAttribution {
            accountable_actor_ref: "agent:demo-helper".to_string(),
            authority_ref: "policy:affected-party-review".to_string(),
            delegated_by_trace_ref: None,
            delegate_trace_ref: None,
            reviewer_chain_refs: vec!["review:affected-party-escalation".to_string()],
        },
        visibility: MoralTraceVisibility {
            public_disclosure: "none".to_string(),
            public_summary: None,
            reviewer_evidence_refs: vec!["artifact:handoff-request".to_string()],
            governance_evidence_refs: vec!["policy:affected-party-review".to_string()],
            public_evidence_refs: Vec::new(),
            private_state_refs: Vec::new(),
        },
        review_refs: MoralTraceReviewRefs {
            challenge_ref: Some("challenge:missing-affected-party".to_string()),
            review_packet_refs: vec!["review:wp04-deferred".to_string()],
            outcome_link_refs: vec!["outcome:deferred-pending-evidence".to_string()],
            metric_refs: vec!["metric:uncertainty-preserved".to_string()],
        },
    }
}

fn base_event(event_kind: &str) -> MoralEventRecord {
    MoralEventRecord {
        schema_version: MORAL_EVENT_SCHEMA_VERSION.to_string(),
        event_id: format!("evt_wp04_{event_kind}"),
        event_kind: event_kind.to_string(),
        occurred_at: "logical:wp04-fixture".to_string(),
        accountable_identity: MoralEventAccountableIdentity {
            actor_id: "agent:demo-helper".to_string(),
            actor_role: "agent".to_string(),
            authority_ref: "policy:bounded-summary".to_string(),
        },
        trace_context: MoralEventTraceContext {
            run_id: "run:wp04-fixture".to_string(),
            step_id: "gate:wp04-trace".to_string(),
            parent_trace_ref: "trace:wp04-parent".to_string(),
            visibility: "reviewer".to_string(),
        },
        choice: MoralEventChoice {
            requested_action: "placeholder".to_string(),
            selected_action: "placeholder".to_string(),
            decision_basis: vec!["policy:placeholder".to_string()],
        },
        alternatives: MoralEventAlternatives {
            considered: vec![MoralEventAlternative {
                action: "placeholder alternative".to_string(),
                reason_considered: "placeholder considered".to_string(),
                reason_rejected: "placeholder rejected".to_string(),
            }],
            omitted_known_alternatives: Vec::new(),
        },
        refusal: MoralEventRefusal {
            refused: false,
            refusal_reason: "none".to_string(),
            policy_ref: "policy:bounded-summary".to_string(),
        },
        uncertainty: MoralEventUncertainty {
            level: "low".to_string(),
            unresolved_questions: Vec::new(),
            confidence_notes: "placeholder confidence".to_string(),
        },
        affected_parties: MoralEventAffectedParties {
            direct: vec!["operator:requester".to_string()],
            indirect: Vec::new(),
            unknown_or_unrepresented: Vec::new(),
        },
        evidence: MoralEventEvidence {
            supporting_refs: vec!["artifact:placeholder".to_string()],
            missing_evidence: Vec::new(),
            privacy_classification: "internal".to_string(),
        },
        policy_context: MoralEventPolicyContext {
            governing_policy_refs: vec!["policy:bounded-summary".to_string()],
            safety_boundary_refs: vec!["boundary:placeholder".to_string()],
            exception_or_override: "none".to_string(),
            override_authority_ref: None,
        },
        review: MoralEventReview {
            reviewer_visibility: "full".to_string(),
            review_required: false,
            challenge_ref: "none".to_string(),
            human_review_note: "placeholder review note".to_string(),
        },
        delegation_context: None,
    }
}
