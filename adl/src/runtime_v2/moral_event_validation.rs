//! Runtime-v2 moral event validation.
//!
//! This module implements the v0.91 WP-03 fail-closed validator for the
//! `moral_event.v1` contract. Validation errors intentionally report only
//! stable field paths and codes, never record values or private diagnostics.

use super::*;

pub const MORAL_EVENT_SCHEMA_VERSION: &str = "moral_event.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralEventRecord {
    pub schema_version: String,
    pub event_id: String,
    pub event_kind: String,
    pub occurred_at: String,
    pub accountable_identity: MoralEventAccountableIdentity,
    pub trace_context: MoralEventTraceContext,
    pub choice: MoralEventChoice,
    pub alternatives: MoralEventAlternatives,
    pub refusal: MoralEventRefusal,
    pub uncertainty: MoralEventUncertainty,
    pub affected_parties: MoralEventAffectedParties,
    pub evidence: MoralEventEvidence,
    pub policy_context: MoralEventPolicyContext,
    pub review: MoralEventReview,
    #[serde(default)]
    pub delegation_context: Option<MoralEventDelegationContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralEventAccountableIdentity {
    pub actor_id: String,
    pub actor_role: String,
    pub authority_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralEventTraceContext {
    pub run_id: String,
    pub step_id: String,
    pub parent_trace_ref: String,
    pub visibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralEventChoice {
    pub requested_action: String,
    pub selected_action: String,
    pub decision_basis: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralEventAlternatives {
    pub considered: Vec<MoralEventAlternative>,
    pub omitted_known_alternatives: Vec<MoralEventOmittedAlternative>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralEventAlternative {
    pub action: String,
    pub reason_considered: String,
    pub reason_rejected: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralEventOmittedAlternative {
    pub action: String,
    pub omission_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralEventRefusal {
    pub refused: bool,
    pub refusal_reason: String,
    pub policy_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralEventUncertainty {
    pub level: String,
    pub unresolved_questions: Vec<String>,
    pub confidence_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralEventAffectedParties {
    pub direct: Vec<String>,
    pub indirect: Vec<String>,
    pub unknown_or_unrepresented: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralEventEvidence {
    pub supporting_refs: Vec<String>,
    pub missing_evidence: Vec<String>,
    pub privacy_classification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralEventPolicyContext {
    pub governing_policy_refs: Vec<String>,
    pub safety_boundary_refs: Vec<String>,
    pub exception_or_override: String,
    #[serde(default)]
    pub override_authority_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralEventReview {
    pub reviewer_visibility: String,
    pub review_required: bool,
    pub challenge_ref: String,
    pub human_review_note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoralEventDelegationContext {
    pub delegated: bool,
    pub delegate_actor_ref: String,
    pub delegate_authority_ref: String,
    pub delegate_trace_ref: String,
}

pub fn validate_moral_event(record: &MoralEventRecord) -> Result<()> {
    let mut errors = Vec::new();

    require_eq(
        &mut errors,
        &record.schema_version,
        MORAL_EVENT_SCHEMA_VERSION,
        "schema_version",
    );
    require_nonempty(&mut errors, &record.event_id, "event_id");
    require_one_of(
        &mut errors,
        &record.event_kind,
        &["allowed", "denied", "deferred", "challenged"],
        "event_kind",
    );
    require_nonempty(&mut errors, &record.occurred_at, "occurred_at");

    require_nonempty(
        &mut errors,
        &record.accountable_identity.actor_id,
        "accountable_identity.actor_id",
    );
    require_one_of(
        &mut errors,
        &record.accountable_identity.actor_role,
        &["citizen", "operator", "agent", "tool", "reviewer"],
        "accountable_identity.actor_role",
    );
    require_nonempty(
        &mut errors,
        &record.accountable_identity.authority_ref,
        "accountable_identity.authority_ref",
    );

    require_nonempty(
        &mut errors,
        &record.trace_context.run_id,
        "trace_context.run_id",
    );
    require_nonempty(
        &mut errors,
        &record.trace_context.step_id,
        "trace_context.step_id",
    );
    require_one_of(
        &mut errors,
        &record.trace_context.visibility,
        &["private", "reviewer", "governance", "public_redacted"],
        "trace_context.visibility",
    );

    require_nonempty(
        &mut errors,
        &record.choice.requested_action,
        "choice.requested_action",
    );
    require_nonempty(
        &mut errors,
        &record.choice.selected_action,
        "choice.selected_action",
    );
    require_nonempty_vec(
        &mut errors,
        &record.choice.decision_basis,
        "choice.decision_basis",
    );

    require_nonempty_vec(
        &mut errors,
        &record.alternatives.considered,
        "alternatives.considered",
    );
    for alternative in &record.alternatives.considered {
        require_nonempty(
            &mut errors,
            &alternative.action,
            "alternatives.considered.action",
        );
        require_nonempty(
            &mut errors,
            &alternative.reason_considered,
            "alternatives.considered.reason_considered",
        );
        require_nonempty(
            &mut errors,
            &alternative.reason_rejected,
            "alternatives.considered.reason_rejected",
        );
    }
    for omitted in &record.alternatives.omitted_known_alternatives {
        require_nonempty(
            &mut errors,
            &omitted.action,
            "alternatives.omitted_known_alternatives.action",
        );
        require_nonempty(
            &mut errors,
            &omitted.omission_reason,
            "alternatives.omitted_known_alternatives.omission_reason",
        );
    }

    validate_refusal_consistency(&mut errors, record);
    validate_uncertainty(&mut errors, record);
    validate_affected_parties(&mut errors, record);
    validate_evidence(&mut errors, record);
    validate_policy_context(&mut errors, record);
    validate_review(&mut errors, record);
    validate_delegation(&mut errors, record);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(anyhow!(
            "moral_event validation failed: {}",
            errors.join("; ")
        ))
    }
}

fn validate_refusal_consistency(errors: &mut Vec<String>, record: &MoralEventRecord) {
    match record.event_kind.as_str() {
        "denied" if !record.refusal.refused => {
            errors.push("contradictory:denied_without_refusal".to_string())
        }
        "allowed" if record.refusal.refused => {
            errors.push("contradictory:allowed_with_refusal".to_string())
        }
        _ => {}
    }

    if record.refusal.refused {
        require_nonempty(
            errors,
            &record.refusal.refusal_reason,
            "refusal.refusal_reason",
        );
        require_nonempty(errors, &record.refusal.policy_ref, "refusal.policy_ref");
    }
}

fn validate_uncertainty(errors: &mut Vec<String>, record: &MoralEventRecord) {
    require_one_of(
        errors,
        &record.uncertainty.level,
        &["low", "medium", "high", "unknown"],
        "uncertainty.level",
    );
    require_nonempty(
        errors,
        &record.uncertainty.confidence_notes,
        "uncertainty.confidence_notes",
    );

    if matches!(record.uncertainty.level.as_str(), "high" | "unknown")
        && record.uncertainty.unresolved_questions.is_empty()
    {
        errors.push("uncertainty.unresolved_questions required_for_high_or_unknown".to_string());
    }
    if record.uncertainty.level == "low" && !record.evidence.missing_evidence.is_empty() {
        errors.push("unsupported_certainty:low_with_missing_evidence".to_string());
    }
}

fn validate_affected_parties(errors: &mut Vec<String>, record: &MoralEventRecord) {
    if record.affected_parties.direct.is_empty()
        && record.affected_parties.indirect.is_empty()
        && record.affected_parties.unknown_or_unrepresented.is_empty()
    {
        errors.push(
            "affected_parties must identify direct, indirect, or unknown parties".to_string(),
        );
    }
}

fn validate_evidence(errors: &mut Vec<String>, record: &MoralEventRecord) {
    if record.evidence.supporting_refs.is_empty() && record.evidence.missing_evidence.is_empty() {
        errors.push("evidence requires supporting_refs or missing_evidence".to_string());
    }
    require_one_of(
        errors,
        &record.evidence.privacy_classification,
        &["public", "internal", "private", "secret_redacted"],
        "evidence.privacy_classification",
    );
}

fn validate_policy_context(errors: &mut Vec<String>, record: &MoralEventRecord) {
    require_nonempty_vec(
        errors,
        &record.policy_context.governing_policy_refs,
        "policy_context.governing_policy_refs",
    );
    require_nonempty_vec(
        errors,
        &record.policy_context.safety_boundary_refs,
        "policy_context.safety_boundary_refs",
    );
    require_one_of(
        errors,
        &record.policy_context.exception_or_override,
        &["none", "requested", "approved", "rejected"],
        "policy_context.exception_or_override",
    );
    if record.policy_context.exception_or_override == "approved"
        && record
            .policy_context
            .override_authority_ref
            .as_deref()
            .unwrap_or_default()
            .trim()
            .is_empty()
    {
        errors.push("policy_incoherent:approved_override_without_authority".to_string());
    }
}

fn validate_review(errors: &mut Vec<String>, record: &MoralEventRecord) {
    require_one_of(
        errors,
        &record.review.reviewer_visibility,
        &["full", "redacted", "summary_only"],
        "review.reviewer_visibility",
    );
    if record.review.review_required && record.review.human_review_note.trim().is_empty() {
        errors.push("unreviewable:review_required_without_human_review_note".to_string());
    }
    if record.event_kind == "challenged" && record.review.challenge_ref.trim().is_empty() {
        errors.push("unreviewable:challenged_event_without_challenge_ref".to_string());
    }
}

fn validate_delegation(errors: &mut Vec<String>, record: &MoralEventRecord) {
    let Some(delegation) = &record.delegation_context else {
        return;
    };
    if !delegation.delegated {
        return;
    }
    require_nonempty(
        errors,
        &delegation.delegate_actor_ref,
        "delegation_context.delegate_actor_ref",
    );
    require_nonempty(
        errors,
        &delegation.delegate_authority_ref,
        "delegation_context.delegate_authority_ref",
    );
    require_nonempty(
        errors,
        &delegation.delegate_trace_ref,
        "delegation_context.delegate_trace_ref",
    );
}

fn require_eq(errors: &mut Vec<String>, value: &str, expected: &str, field: &str) {
    if value != expected {
        errors.push(format!("{field} unsupported"));
    }
}

fn require_nonempty(errors: &mut Vec<String>, value: &str, field: &str) {
    if value.trim().is_empty() {
        errors.push(format!("{field} must not be empty"));
    }
}

fn require_nonempty_vec<T>(errors: &mut Vec<String>, value: &[T], field: &str) {
    if value.is_empty() {
        errors.push(format!("{field} must not be empty"));
    }
}

fn require_one_of(errors: &mut Vec<String>, value: &str, allowed: &[&str], field: &str) {
    if !allowed.contains(&value) {
        errors.push(format!("{field} unsupported"));
    }
}
