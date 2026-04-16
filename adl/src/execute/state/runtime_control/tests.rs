use super::*;
use crate::freedom_gate;

#[test]
fn select_instinct_runtime_candidate_changes_fast_path_for_curiosity() {
    let decision = select_instinct_runtime_candidate(
        SelectedPath::FastPath,
        DominantInstinct::Curiosity,
        "low",
    );

    assert_eq!(decision.candidate_id, "cand-fast-verify");
    assert_eq!(decision.candidate_kind, "bounded_verification");
}

#[test]
fn select_instinct_runtime_candidate_keeps_review_for_high_risk_slow_path() {
    let decision = select_instinct_runtime_candidate(
        SelectedPath::SlowPath,
        DominantInstinct::Completion,
        "high",
    );

    assert_eq!(decision.candidate_id, "cand-slow-review");
    assert_eq!(decision.candidate_kind, "review_and_refine");
}

#[test]
fn select_instinct_runtime_candidate_allows_curiosity_biased_slow_defer() {
    let decision = select_instinct_runtime_candidate(
        SelectedPath::SlowPath,
        DominantInstinct::Curiosity,
        "medium",
    );

    assert_eq!(decision.candidate_id, "cand-slow-defer");
    assert_eq!(decision.candidate_kind, "bounded_deferral");
}

#[test]
fn runtime_control_enums_serialize_to_existing_string_values() {
    let state = CognitiveSignalsState {
        dominant_instinct: DominantInstinct::Integrity,
        completion_pressure: "guarded".to_string(),
        integrity_bias: "high".to_string(),
        curiosity_bias: "bounded".to_string(),
        candidate_selection_bias: "prefer lower-risk constrained candidates".to_string(),
        urgency_level: "moderate".to_string(),
        salience_level: "high".to_string(),
        persistence_pressure: "stabilize_then_retry".to_string(),
        confidence_shift: "reduced".to_string(),
        downstream_influence: "demo".to_string(),
    };

    let json = serde_json::to_value(&state).expect("serialize state");
    assert_eq!(json["dominant_instinct"], "integrity");
}

#[test]
fn freedom_gate_input_state_conversion_preserves_nested_fields() {
    let input = freedom_gate::FreedomGateInput {
        candidate_id: "cand-007".to_string(),
        candidate_action: "review".to_string(),
        candidate_rationale: "bounded rationale".to_string(),
        risk_class: "medium".to_string(),
        policy_context: freedom_gate::FreedomGatePolicyContext {
            route_selected: "slow".to_string(),
            selected_candidate_kind: "review_and_refine".to_string(),
            requires_review: true,
            policy_blocked: false,
        },
        evaluation_signals: freedom_gate::FreedomGateEvaluationSignals {
            progress_signal: "guarded".to_string(),
            contradiction_signal: "present".to_string(),
            failure_signal: "none".to_string(),
            termination_reason: "paused".to_string(),
        },
        consequence_context: freedom_gate::FreedomGateConsequenceContext {
            impact_scope: "cross_surface".to_string(),
            recovery_cost: "bounded_review_replay".to_string(),
            operator_visibility: "review_required".to_string(),
            escalation_available: true,
        },
        frame_state: "ready_for_reframed_execution".to_string(),
    };

    let state = FreedomGateInputState::from(input);

    assert_eq!(state.candidate_id, "cand-007");
    assert_eq!(state.candidate_action, "review");
    assert_eq!(state.candidate_rationale, "bounded rationale");
    assert_eq!(state.risk_class, "medium");
    assert_eq!(state.policy_context.route_selected, Route::Slow);
    assert_eq!(
        state.policy_context.selected_candidate_kind,
        "review_and_refine"
    );
    assert!(state.policy_context.requires_review);
    assert!(!state.policy_context.policy_blocked);
    assert_eq!(state.evaluation_signals.progress_signal, "guarded");
    assert_eq!(state.evaluation_signals.contradiction_signal, "present");
    assert_eq!(state.evaluation_signals.failure_signal, "none");
    assert_eq!(state.evaluation_signals.termination_reason, "paused");
    assert_eq!(state.consequence_context.impact_scope, "cross_surface");
    assert_eq!(
        state.consequence_context.recovery_cost,
        "bounded_review_replay"
    );
    assert_eq!(
        state.consequence_context.operator_visibility,
        "review_required"
    );
    assert!(state.consequence_context.escalation_available);
    assert_eq!(state.frame_state, "ready_for_reframed_execution");
}
