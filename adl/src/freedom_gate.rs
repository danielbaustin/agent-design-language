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
}
