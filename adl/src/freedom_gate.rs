use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreedomGateInput {
    pub candidate_id: String,
    pub candidate_action: String,
    pub candidate_rationale: String,
    pub risk_class: String,
    pub policy_context: FreedomGatePolicyContext,
    pub evaluation_signals: FreedomGateEvaluationSignals,
    pub consequence_context: FreedomGateConsequenceContext,
    pub frame_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreedomGatePolicyContext {
    pub route_selected: String,
    pub selected_candidate_kind: String,
    pub requires_review: bool,
    pub policy_blocked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreedomGateEvaluationSignals {
    pub progress_signal: String,
    pub contradiction_signal: String,
    pub failure_signal: String,
    pub termination_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreedomGateConsequenceContext {
    pub impact_scope: String,
    pub recovery_cost: String,
    pub operator_visibility: String,
    pub escalation_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreedomGateDecision {
    pub gate_decision: String,
    pub reason_code: String,
    pub decision_reason: String,
    pub selected_action_or_none: Option<String>,
    pub commitment_blocked: bool,
    pub judgment_boundary: String,
    pub required_follow_up: String,
    pub decision_record_kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FreedomGateToolDecisionV1 {
    Allowed,
    Denied,
    Deferred,
    Challenged,
    Escalated,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FreedomGateToolBoundaryV1 {
    Policy,
    Privacy,
    OperatorReview,
    CitizenAction,
    Escalation,
    Execution,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FreedomGateToolCandidateV1 {
    pub candidate_id: String,
    pub proposal_id: String,
    pub normalized_proposal_ref: String,
    pub acc_contract_id: String,
    pub policy_evidence_ref: String,
    pub action_kind: String,
    pub risk_class: String,
    pub operator_actor_id: String,
    pub citizen_boundary_ref: String,
    pub private_argument_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FreedomGateToolGateContextV1 {
    pub policy_decision: String,
    pub requires_operator_review: bool,
    pub requires_human_challenge: bool,
    pub escalation_available: bool,
    pub citizen_action_boundary_intact: bool,
    pub operator_action_boundary_intact: bool,
    pub private_arguments_redacted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FreedomGateToolDecisionEventV1 {
    pub decision: FreedomGateToolDecisionV1,
    pub reason_code: String,
    pub stopped_before_executor: bool,
    pub executor_invocation_ref: Option<String>,
    pub boundary: FreedomGateToolBoundaryV1,
    pub trace_links: Vec<String>,
    pub redaction_summary: String,
}

fn gate_token_like(value: &str) -> bool {
    !value.trim().is_empty()
        && value.chars().all(|ch| {
            ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '-' | '_' | '.')
        })
}

fn contains_private_payload_shape(value: &str) -> bool {
    value.contains('{')
        || value.contains('}')
        || value.contains('/')
        || value.contains('\\')
        || value.to_ascii_lowercase().contains("secret")
}

fn valid_private_argument_digest(value: &str) -> bool {
    value
        .strip_prefix("sha256:")
        .is_some_and(|digest| digest.len() == 64 && digest.chars().all(|ch| ch.is_ascii_hexdigit()))
}

fn valid_tool_risk_class(value: &str) -> bool {
    matches!(value, "low" | "medium" | "high")
}

fn decision_event(
    decision: FreedomGateToolDecisionV1,
    reason_code: impl Into<String>,
    boundary: FreedomGateToolBoundaryV1,
    candidate: &FreedomGateToolCandidateV1,
) -> FreedomGateToolDecisionEventV1 {
    let stopped_before_executor = !matches!(decision, FreedomGateToolDecisionV1::Allowed);
    let executor_invocation_ref = if stopped_before_executor {
        None
    } else {
        Some(format!("executor.{}", candidate.candidate_id))
    };

    FreedomGateToolDecisionEventV1 {
        decision,
        reason_code: reason_code.into(),
        stopped_before_executor,
        executor_invocation_ref,
        boundary,
        trace_links: vec![
            format!("proposal:{}", candidate.proposal_id),
            format!("normalized_proposal:{}", candidate.normalized_proposal_ref),
            format!("acc:{}", candidate.acc_contract_id),
            format!("policy:{}", candidate.policy_evidence_ref),
            format!("action:{}", candidate.action_kind),
            format!("gate:{}", candidate.candidate_id),
        ],
        redaction_summary: format!(
            "private_arguments_redacted digest={}",
            candidate.private_argument_digest
        ),
    }
}

fn invalid_candidate_decision_event() -> FreedomGateToolDecisionEventV1 {
    FreedomGateToolDecisionEventV1 {
        decision: FreedomGateToolDecisionV1::Denied,
        reason_code: "invalid_gate_trace_context".to_string(),
        stopped_before_executor: true,
        executor_invocation_ref: None,
        boundary: FreedomGateToolBoundaryV1::Privacy,
        trace_links: vec![
            "proposal:invalid".to_string(),
            "normalized_proposal:invalid".to_string(),
            "acc:invalid".to_string(),
            "policy:invalid".to_string(),
            "action:invalid".to_string(),
            "gate:invalid".to_string(),
        ],
        redaction_summary: "private_arguments_redacted digest=invalid".to_string(),
    }
}

pub fn evaluate_tool_candidate_freedom_gate_v1(
    candidate: &FreedomGateToolCandidateV1,
    context: &FreedomGateToolGateContextV1,
) -> FreedomGateToolDecisionEventV1 {
    if !gate_token_like(&candidate.candidate_id)
        || !gate_token_like(&candidate.proposal_id)
        || !gate_token_like(&candidate.normalized_proposal_ref)
        || !gate_token_like(&candidate.acc_contract_id)
        || !gate_token_like(&candidate.policy_evidence_ref)
        || !gate_token_like(&candidate.action_kind)
        || !valid_tool_risk_class(&candidate.risk_class)
        || !gate_token_like(&candidate.operator_actor_id)
        || !gate_token_like(&candidate.citizen_boundary_ref)
        || !valid_private_argument_digest(&candidate.private_argument_digest)
        || contains_private_payload_shape(&candidate.private_argument_digest)
    {
        return invalid_candidate_decision_event();
    }

    if !context.private_arguments_redacted {
        return decision_event(
            FreedomGateToolDecisionV1::Denied,
            "private_arguments_not_redacted",
            FreedomGateToolBoundaryV1::Privacy,
            candidate,
        );
    }

    if !context.citizen_action_boundary_intact {
        return decision_event(
            FreedomGateToolDecisionV1::Denied,
            "citizen_action_boundary_broken",
            FreedomGateToolBoundaryV1::CitizenAction,
            candidate,
        );
    }

    if !context.operator_action_boundary_intact {
        return decision_event(
            FreedomGateToolDecisionV1::Denied,
            "operator_action_boundary_broken",
            FreedomGateToolBoundaryV1::OperatorReview,
            candidate,
        );
    }

    match context.policy_decision.as_str() {
        "denied" | "revoked" => {
            return decision_event(
                FreedomGateToolDecisionV1::Denied,
                "policy_denied",
                FreedomGateToolBoundaryV1::Policy,
                candidate,
            );
        }
        "deferred" => {
            return decision_event(
                FreedomGateToolDecisionV1::Deferred,
                "policy_deferred",
                FreedomGateToolBoundaryV1::OperatorReview,
                candidate,
            );
        }
        "challenged" => {
            return decision_event(
                FreedomGateToolDecisionV1::Challenged,
                "policy_challenged",
                FreedomGateToolBoundaryV1::OperatorReview,
                candidate,
            );
        }
        "allowed" => {}
        _ => {
            return decision_event(
                FreedomGateToolDecisionV1::Denied,
                "unknown_policy_decision",
                FreedomGateToolBoundaryV1::Policy,
                candidate,
            );
        }
    }

    if context.requires_human_challenge {
        return decision_event(
            FreedomGateToolDecisionV1::Challenged,
            "human_challenge_required",
            FreedomGateToolBoundaryV1::OperatorReview,
            candidate,
        );
    }

    if context.requires_operator_review {
        return decision_event(
            FreedomGateToolDecisionV1::Deferred,
            "operator_review_required",
            FreedomGateToolBoundaryV1::OperatorReview,
            candidate,
        );
    }

    if candidate.risk_class == "high" && context.escalation_available {
        return decision_event(
            FreedomGateToolDecisionV1::Escalated,
            "high_risk_escalation_required",
            FreedomGateToolBoundaryV1::Escalation,
            candidate,
        );
    }

    if candidate.risk_class == "high" {
        return decision_event(
            FreedomGateToolDecisionV1::Denied,
            "high_risk_without_escalation",
            FreedomGateToolBoundaryV1::Policy,
            candidate,
        );
    }

    decision_event(
        FreedomGateToolDecisionV1::Allowed,
        "gate_allowed",
        FreedomGateToolBoundaryV1::Execution,
        candidate,
    )
}

pub fn evaluate_freedom_gate(input: &FreedomGateInput) -> FreedomGateDecision {
    let candidate_action = input.candidate_action.trim();

    let escalation_candidate = input.consequence_context.escalation_available
        && (input.risk_class == "high"
            || input.evaluation_signals.contradiction_signal == "present"
            || input.evaluation_signals.failure_signal != "none"
            || input.frame_state == "ready_for_reframed_execution");

    let (
        gate_decision,
        reason_code,
        decision_reason,
        selected_action_or_none,
        commitment_blocked,
        judgment_boundary,
        required_follow_up,
        decision_record_kind,
    ) = if input.policy_context.policy_blocked {
        (
            "refuse",
            "policy_blocked",
            "policy context explicitly blocks commitment for this bounded candidate",
            None,
            true,
            "policy_boundary",
            "record_refusal_and_stop",
            "gate_refusal_record",
        )
    } else if candidate_action.is_empty() {
        (
            "defer",
            "insufficient_context",
            "candidate action is empty, so commitment must remain blocked until context is restored",
            None,
            true,
            "context_boundary",
            "restore_candidate_context",
            "gate_defer_record",
        )
    } else if escalation_candidate && input.frame_state == "ready_for_reframed_execution" {
        (
            "escalate",
            "frame_escalation_required",
            "frame state and consequence context require explicit escalation before commitment can proceed",
            None,
            true,
            "judgment_boundary",
            "escalate_for_judgment_review",
            "gate_escalation_record",
        )
    } else if escalation_candidate && input.policy_context.requires_review {
        (
            "escalate",
            "review_escalation_required",
            "policy review and consequence context together require escalation instead of a silent defer",
            None,
            true,
            "judgment_boundary",
            "escalate_for_review_board",
            "gate_escalation_record",
        )
    } else if input.frame_state == "ready_for_reframed_execution" {
        (
            "defer",
            "frame_inadequate",
            "frame state requires bounded reframing before commitment can be allowed",
            None,
            true,
            "frame_boundary",
            "reframe_before_commitment",
            "gate_defer_record",
        )
    } else if input.policy_context.requires_review {
        (
            "defer",
            "requires_review",
            "policy context requires bounded review before commitment can be allowed",
            None,
            true,
            "review_boundary",
            "complete_bounded_review",
            "gate_defer_record",
        )
    } else if input.risk_class == "high" && input.consequence_context.escalation_available {
        (
            "escalate",
            "high_risk_requires_escalation",
            "risk remains too high for bounded commitment and must be escalated through an explicit judgment path",
            None,
            true,
            "judgment_boundary",
            "escalate_for_operator_decision",
            "gate_escalation_record",
        )
    } else if input.risk_class == "high" {
        (
            "refuse",
            "risk_too_high",
            "risk class remains too high for bounded commitment without an available escalation path",
            None,
            true,
            "risk_boundary",
            "record_refusal_and_stop",
            "gate_refusal_record",
        )
    } else {
        (
            "allow",
            "policy_allowed",
            "bounded policy context allows commitment for the selected candidate",
            Some(candidate_action.to_string()),
            false,
            "commitment_boundary",
            "commit_selected_action",
            "gate_allow_record",
        )
    };

    FreedomGateDecision {
        gate_decision: gate_decision.to_string(),
        reason_code: reason_code.to_string(),
        decision_reason: decision_reason.to_string(),
        selected_action_or_none,
        commitment_blocked,
        judgment_boundary: judgment_boundary.to_string(),
        required_follow_up: required_follow_up.to_string(),
        decision_record_kind: decision_record_kind.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_input() -> FreedomGateInput {
        FreedomGateInput {
            candidate_id: "cand-001".to_string(),
            candidate_action: "execute bounded candidate".to_string(),
            candidate_rationale: "bounded rationale".to_string(),
            risk_class: "low".to_string(),
            policy_context: FreedomGatePolicyContext {
                route_selected: "fast".to_string(),
                selected_candidate_kind: "direct_execution".to_string(),
                requires_review: false,
                policy_blocked: false,
            },
            evaluation_signals: FreedomGateEvaluationSignals {
                progress_signal: "steady_progress".to_string(),
                contradiction_signal: "none".to_string(),
                failure_signal: "none".to_string(),
                termination_reason: "success".to_string(),
            },
            consequence_context: FreedomGateConsequenceContext {
                impact_scope: "local_bounded".to_string(),
                recovery_cost: "low".to_string(),
                operator_visibility: "routine".to_string(),
                escalation_available: false,
            },
            frame_state: "complete_run".to_string(),
        }
    }

    fn tool_candidate() -> FreedomGateToolCandidateV1 {
        FreedomGateToolCandidateV1 {
            candidate_id: "candidate.tool.safe-read".to_string(),
            proposal_id: "proposal.fixture.safe-read".to_string(),
            normalized_proposal_ref: "normalized.proposal.fixture.safe-read".to_string(),
            acc_contract_id: "acc.compiler.proposal.fixture.safe-read".to_string(),
            policy_evidence_ref: "policy.wp11.fixture".to_string(),
            action_kind: "fixture_read".to_string(),
            risk_class: "low".to_string(),
            operator_actor_id: "actor.operator.alice".to_string(),
            citizen_boundary_ref: "citizen.boundary.fixture".to_string(),
            private_argument_digest: format!("sha256:{}", "1".repeat(64)),
        }
    }

    fn tool_gate_context(policy_decision: &str) -> FreedomGateToolGateContextV1 {
        FreedomGateToolGateContextV1 {
            policy_decision: policy_decision.to_string(),
            requires_operator_review: false,
            requires_human_challenge: false,
            escalation_available: false,
            citizen_action_boundary_intact: true,
            operator_action_boundary_intact: true,
            private_arguments_redacted: true,
        }
    }

    #[test]
    fn tool_freedom_gate_allows_only_when_executor_invocation_is_safe() {
        let event = evaluate_tool_candidate_freedom_gate_v1(
            &tool_candidate(),
            &tool_gate_context("allowed"),
        );

        assert_eq!(event.decision, FreedomGateToolDecisionV1::Allowed);
        assert_eq!(event.reason_code, "gate_allowed");
        assert!(!event.stopped_before_executor);
        assert_eq!(
            event.executor_invocation_ref.as_deref(),
            Some("executor.candidate.tool.safe-read")
        );
        assert_eq!(event.boundary, FreedomGateToolBoundaryV1::Execution);
        assert!(event
            .trace_links
            .contains(&"proposal:proposal.fixture.safe-read".to_string()));
        assert!(event
            .trace_links
            .contains(&"acc:acc.compiler.proposal.fixture.safe-read".to_string()));
        assert!(event
            .trace_links
            .contains(&"action:fixture_read".to_string()));
    }

    #[test]
    fn tool_freedom_gate_stops_denied_deferred_challenged_and_escalated_actions() {
        let candidate = tool_candidate();
        let denied =
            evaluate_tool_candidate_freedom_gate_v1(&candidate, &tool_gate_context("denied"));
        let deferred =
            evaluate_tool_candidate_freedom_gate_v1(&candidate, &tool_gate_context("deferred"));
        let challenged =
            evaluate_tool_candidate_freedom_gate_v1(&candidate, &tool_gate_context("challenged"));
        let mut high_risk = candidate.clone();
        high_risk.risk_class = "high".to_string();
        let mut escalation_context = tool_gate_context("allowed");
        escalation_context.escalation_available = true;
        let escalated = evaluate_tool_candidate_freedom_gate_v1(&high_risk, &escalation_context);

        for event in [denied, deferred, challenged, escalated] {
            assert!(event.stopped_before_executor);
            assert_eq!(event.executor_invocation_ref, None);
            assert_ne!(event.decision, FreedomGateToolDecisionV1::Allowed);
        }
    }

    #[test]
    fn tool_freedom_gate_records_decisions_without_private_argument_leakage() {
        let event = evaluate_tool_candidate_freedom_gate_v1(
            &tool_candidate(),
            &tool_gate_context("allowed"),
        );
        let serialized = serde_json::to_string(&event).expect("serialize gate event");

        assert!(serialized.contains("private_arguments_redacted"));
        assert!(serialized
            .contains("sha256:1111111111111111111111111111111111111111111111111111111111111111"));
        assert!(!serialized.contains("fixture_id"));
        assert!(!serialized.contains("secret"));
    }

    #[test]
    fn tool_freedom_gate_denies_unredacted_private_arguments_before_executor() {
        let mut context = tool_gate_context("allowed");
        context.private_arguments_redacted = false;

        let event = evaluate_tool_candidate_freedom_gate_v1(&tool_candidate(), &context);

        assert_eq!(event.decision, FreedomGateToolDecisionV1::Denied);
        assert_eq!(event.reason_code, "private_arguments_not_redacted");
        assert_eq!(event.boundary, FreedomGateToolBoundaryV1::Privacy);
        assert!(event.stopped_before_executor);
        assert_eq!(event.executor_invocation_ref, None);
    }

    #[test]
    fn tool_freedom_gate_denies_broken_citizen_boundaries_before_executor() {
        let mut context = tool_gate_context("allowed");
        context.citizen_action_boundary_intact = false;

        let event = evaluate_tool_candidate_freedom_gate_v1(&tool_candidate(), &context);

        assert_eq!(event.decision, FreedomGateToolDecisionV1::Denied);
        assert_eq!(event.reason_code, "citizen_action_boundary_broken");
        assert_eq!(event.boundary, FreedomGateToolBoundaryV1::CitizenAction);
        assert!(event.stopped_before_executor);
        assert_eq!(event.executor_invocation_ref, None);
    }

    #[test]
    fn tool_freedom_gate_denies_broken_operator_boundaries_before_executor() {
        let mut context = tool_gate_context("allowed");
        context.operator_action_boundary_intact = false;

        let event = evaluate_tool_candidate_freedom_gate_v1(&tool_candidate(), &context);

        assert_eq!(event.decision, FreedomGateToolDecisionV1::Denied);
        assert_eq!(event.reason_code, "operator_action_boundary_broken");
        assert_eq!(event.boundary, FreedomGateToolBoundaryV1::OperatorReview);
        assert!(event.stopped_before_executor);
        assert_eq!(event.executor_invocation_ref, None);
    }

    #[test]
    fn tool_freedom_gate_rejects_unsafe_trace_and_digest_shapes() {
        let mut candidate = tool_candidate();
        candidate.action_kind = "../fixture_read".to_string();
        let event =
            evaluate_tool_candidate_freedom_gate_v1(&candidate, &tool_gate_context("allowed"));
        assert_eq!(event.decision, FreedomGateToolDecisionV1::Denied);
        assert_eq!(event.reason_code, "invalid_gate_trace_context");
        assert!(event.stopped_before_executor);
        let serialized = serde_json::to_string(&event).expect("serialize invalid action event");
        assert!(!serialized.contains("../fixture_read"));
        assert!(serialized.contains("action:invalid"));

        let mut candidate = tool_candidate();
        candidate.private_argument_digest = "secret-token-value".to_string();
        let event =
            evaluate_tool_candidate_freedom_gate_v1(&candidate, &tool_gate_context("allowed"));
        assert_eq!(event.decision, FreedomGateToolDecisionV1::Denied);
        assert_eq!(event.reason_code, "invalid_gate_trace_context");
        assert!(event.stopped_before_executor);
        let serialized = serde_json::to_string(&event).expect("serialize invalid digest event");
        assert!(!serialized.contains("secret-token-value"));
        assert!(serialized.contains("digest=invalid"));
    }

    #[test]
    fn freedom_gate_allows_bounded_low_risk_commitment() {
        let decision = evaluate_freedom_gate(&base_input());
        assert_eq!(decision.gate_decision, "allow");
        assert_eq!(decision.reason_code, "policy_allowed");
        assert_eq!(
            decision.selected_action_or_none.as_deref(),
            Some("execute bounded candidate")
        );
        assert!(!decision.commitment_blocked);
        assert_eq!(decision.judgment_boundary, "commitment_boundary");
        assert_eq!(decision.required_follow_up, "commit_selected_action");
    }

    #[test]
    fn freedom_gate_defers_when_reframing_is_required() {
        let mut input = base_input();
        input.frame_state = "ready_for_reframed_execution".to_string();
        let decision = evaluate_freedom_gate(&input);
        assert_eq!(decision.gate_decision, "defer");
        assert_eq!(decision.reason_code, "frame_inadequate");
        assert!(decision.commitment_blocked);
        assert_eq!(decision.decision_record_kind, "gate_defer_record");
    }

    #[test]
    fn freedom_gate_refuses_when_policy_blocks() {
        let mut input = base_input();
        input.policy_context.policy_blocked = true;
        let decision = evaluate_freedom_gate(&input);
        assert_eq!(decision.gate_decision, "refuse");
        assert_eq!(decision.reason_code, "policy_blocked");
        assert!(decision.commitment_blocked);
        assert_eq!(decision.decision_record_kind, "gate_refusal_record");
    }

    #[test]
    fn freedom_gate_escalates_high_risk_reframing_cases_when_escalation_is_available() {
        let mut input = base_input();
        input.risk_class = "high".to_string();
        input.frame_state = "ready_for_reframed_execution".to_string();
        input.evaluation_signals.contradiction_signal = "present".to_string();
        input.consequence_context.escalation_available = true;
        input.consequence_context.operator_visibility = "review_required".to_string();
        let decision = evaluate_freedom_gate(&input);
        assert_eq!(decision.gate_decision, "escalate");
        assert_eq!(decision.reason_code, "frame_escalation_required");
        assert_eq!(decision.judgment_boundary, "judgment_boundary");
        assert_eq!(decision.required_follow_up, "escalate_for_judgment_review");
        assert_eq!(decision.decision_record_kind, "gate_escalation_record");
    }

    #[test]
    fn freedom_gate_defers_when_candidate_action_is_empty() {
        let mut input = base_input();
        input.candidate_action = "   ".to_string();
        let decision = evaluate_freedom_gate(&input);
        assert_eq!(decision.gate_decision, "defer");
        assert_eq!(decision.reason_code, "insufficient_context");
        assert_eq!(decision.judgment_boundary, "context_boundary");
        assert_eq!(decision.required_follow_up, "restore_candidate_context");
        assert!(decision.commitment_blocked);
        assert_eq!(decision.decision_record_kind, "gate_defer_record");
    }

    #[test]
    fn freedom_gate_defers_when_review_is_required_without_escalation() {
        let mut input = base_input();
        input.policy_context.requires_review = true;
        let decision = evaluate_freedom_gate(&input);
        assert_eq!(decision.gate_decision, "defer");
        assert_eq!(decision.reason_code, "requires_review");
        assert_eq!(decision.judgment_boundary, "review_boundary");
        assert_eq!(decision.required_follow_up, "complete_bounded_review");
        assert!(decision.commitment_blocked);
    }

    #[test]
    fn freedom_gate_escalates_when_review_and_consequence_context_require_it() {
        let mut input = base_input();
        input.policy_context.requires_review = true;
        input.consequence_context.escalation_available = true;
        input.evaluation_signals.failure_signal = "tool_failure".to_string();
        let decision = evaluate_freedom_gate(&input);
        assert_eq!(decision.gate_decision, "escalate");
        assert_eq!(decision.reason_code, "review_escalation_required");
        assert_eq!(decision.judgment_boundary, "judgment_boundary");
        assert_eq!(decision.required_follow_up, "escalate_for_review_board");
        assert_eq!(decision.decision_record_kind, "gate_escalation_record");
    }

    #[test]
    fn freedom_gate_escalates_high_risk_cases_when_operator_escalation_exists() {
        let mut input = base_input();
        input.risk_class = "high".to_string();
        input.consequence_context.escalation_available = true;
        let decision = evaluate_freedom_gate(&input);
        assert_eq!(decision.gate_decision, "escalate");
        assert_eq!(decision.reason_code, "high_risk_requires_escalation");
        assert_eq!(
            decision.required_follow_up,
            "escalate_for_operator_decision"
        );
        assert_eq!(decision.decision_record_kind, "gate_escalation_record");
        assert!(decision.commitment_blocked);
    }

    #[test]
    fn freedom_gate_refuses_high_risk_cases_without_escalation_path() {
        let mut input = base_input();
        input.risk_class = "high".to_string();
        let decision = evaluate_freedom_gate(&input);
        assert_eq!(decision.gate_decision, "refuse");
        assert_eq!(decision.reason_code, "risk_too_high");
        assert_eq!(decision.judgment_boundary, "risk_boundary");
        assert_eq!(decision.required_follow_up, "record_refusal_and_stop");
        assert_eq!(decision.decision_record_kind, "gate_refusal_record");
        assert!(decision.commitment_blocked);
    }
}
