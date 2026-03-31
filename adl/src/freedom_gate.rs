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
pub struct FreedomGateDecision {
    pub gate_decision: String,
    pub reason_code: String,
    pub decision_reason: String,
    pub selected_action_or_none: Option<String>,
    pub commitment_blocked: bool,
}

pub fn evaluate_freedom_gate(input: &FreedomGateInput) -> FreedomGateDecision {
    let candidate_action = input.candidate_action.trim();

    let (gate_decision, reason_code, decision_reason, selected_action_or_none, commitment_blocked) =
        if input.policy_context.policy_blocked {
            (
                "refuse",
                "policy_blocked",
                "policy context explicitly blocks commitment for this bounded candidate",
                None,
                true,
            )
        } else if candidate_action.is_empty() {
            (
                "defer",
                "insufficient_context",
                "candidate action is empty, so commitment must remain blocked until context is restored",
                None,
                true,
            )
        } else if input.frame_state == "ready_for_reframed_execution" {
            (
                "defer",
                "frame_inadequate",
                "frame state requires bounded reframing before commitment can be allowed",
                None,
                true,
            )
        } else if input.policy_context.requires_review {
            (
                "defer",
                "requires_review",
                "policy context requires bounded review before commitment can be allowed",
                None,
                true,
            )
        } else if input.risk_class == "high" {
            (
                "refuse",
                "risk_too_high",
                "risk class remains too high for bounded commitment in v0.86",
                None,
                true,
            )
        } else {
            (
                "allow",
                "policy_allowed",
                "bounded policy context allows commitment for the selected candidate",
                Some(candidate_action.to_string()),
                false,
            )
        };

    FreedomGateDecision {
        gate_decision: gate_decision.to_string(),
        reason_code: reason_code.to_string(),
        decision_reason: decision_reason.to_string(),
        selected_action_or_none,
        commitment_blocked,
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
    }

    #[test]
    fn freedom_gate_defers_when_reframing_is_required() {
        let mut input = base_input();
        input.frame_state = "ready_for_reframed_execution".to_string();
        let decision = evaluate_freedom_gate(&input);
        assert_eq!(decision.gate_decision, "defer");
        assert_eq!(decision.reason_code, "frame_inadequate");
        assert!(decision.commitment_blocked);
    }

    #[test]
    fn freedom_gate_refuses_when_policy_blocks() {
        let mut input = base_input();
        input.policy_context.policy_blocked = true;
        let decision = evaluate_freedom_gate(&input);
        assert_eq!(decision.gate_decision, "refuse");
        assert_eq!(decision.reason_code, "policy_blocked");
        assert!(decision.commitment_blocked);
    }
}
