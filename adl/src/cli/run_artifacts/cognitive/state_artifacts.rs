use super::super::*;

mod agency_execution;
mod decision_artifacts;

pub(crate) use agency_execution::*;
pub(crate) use decision_artifacts::*;

pub(crate) fn build_affect_state_artifact(
    run_summary: &RunSummaryArtifact,
    suggestions: &SuggestionsArtifact,
    scores: Option<&ScoresArtifact>,
) -> AffectStateArtifact {
    let selected = suggestions
        .suggestions
        .first()
        .cloned()
        .unwrap_or_else(|| SuggestionItem {
            id: "sug-000".to_string(),
            category: "stability".to_string(),
            severity: "info".to_string(),
            rationale: "No bounded adaptation signals fired; keep current policy state."
                .to_string(),
            evidence: SuggestionEvidence {
                failure_count: 0,
                retry_count: 0,
                delegation_denied_count: 0,
                security_denied_count: 0,
                success_ratio: 1.0,
                scheduler_max_parallel_observed: 1,
            },
            proposed_change: SuggestedChangeIntent {
                intent: "maintain_current_policy".to_string(),
                target: "workflow-runtime".to_string(),
            },
        });

    let recovery_bias = if selected.evidence.failure_count > 0
        || selected.proposed_change.intent == "increase_step_retry_budget"
    {
        2
    } else if selected.evidence.retry_count > 0 {
        1
    } else {
        0
    };

    let (affect_mode, urgency_level, frustration_level, confidence_shift, downstream_priority) =
        if recovery_bias >= 2 {
            (
                "recovery_focus",
                "elevated",
                "high",
                "reduced",
                "prefer bounded recovery before broader runtime review",
            )
        } else if recovery_bias == 1 {
            (
                "watchful_adjustment",
                "guarded",
                "moderate",
                "stable",
                "stabilize retries before expanding scope",
            )
        } else {
            (
                "steady_state",
                "low",
                "low",
                "stable",
                "maintain current bounded runtime policy",
            )
        };

    let update_reason = format!(
        "failure_count={} retry_count={} success_ratio={} selected_intent={}",
        selected.evidence.failure_count,
        selected.evidence.retry_count,
        selected.evidence.success_ratio,
        selected.proposed_change.intent
    );

    AffectStateArtifact {
        affect_state_version: 1,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: suggestions.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        affect: AffectStateRecord {
            affect_state_id: "affect-001".to_string(),
            affect_mode: affect_mode.to_string(),
            urgency_level: urgency_level.to_string(),
            frustration_level: frustration_level.to_string(),
            confidence_shift: confidence_shift.to_string(),
            recovery_bias,
            downstream_priority: downstream_priority.to_string(),
            update_reason,
            deterministic_update_rule:
                "derive affect mode and recovery bias from the first stable suggestion plus bounded failure and retry evidence"
                    .to_string(),
        },
    }
}

#[cfg(test)]
pub(crate) fn build_cognitive_signals_state(
    run_summary: &RunSummaryArtifact,
    suggestions: &SuggestionsArtifact,
    _scores: Option<&ScoresArtifact>,
) -> CognitiveSignalsState {
    let selected = suggestions
        .suggestions
        .first()
        .cloned()
        .unwrap_or_else(|| SuggestionItem {
            id: "sug-000".to_string(),
            category: "stability".to_string(),
            severity: "info".to_string(),
            rationale: "No bounded adaptation signals fired; keep current policy state."
                .to_string(),
            evidence: SuggestionEvidence {
                failure_count: 0,
                retry_count: 0,
                delegation_denied_count: 0,
                security_denied_count: 0,
                success_ratio: 1.0,
                scheduler_max_parallel_observed: 1,
            },
            proposed_change: SuggestedChangeIntent {
                intent: "maintain_current_policy".to_string(),
                target: "workflow-runtime".to_string(),
            },
        });

    let completion_pressure =
        if selected.evidence.failure_count > 0 || run_summary.status == "failure" {
            "elevated"
        } else if selected.evidence.retry_count > 0 {
            "guarded"
        } else {
            "steady"
        };
    let integrity_bias = if selected.evidence.security_denied_count > 0 {
        "high"
    } else {
        "bounded"
    };
    let curiosity_bias = if selected.evidence.success_ratio < 1.0 {
        "active"
    } else {
        "low"
    };
    let dominant_instinct = if integrity_bias == "high" {
        execute::DominantInstinct::Integrity
    } else if completion_pressure == "elevated" {
        execute::DominantInstinct::Completion
    } else if curiosity_bias == "active" {
        execute::DominantInstinct::Curiosity
    } else {
        execute::DominantInstinct::Coherence
    };
    let candidate_selection_bias = match dominant_instinct {
        execute::DominantInstinct::Integrity => "prefer lower-risk constrained candidates",
        execute::DominantInstinct::Completion => {
            "prefer candidates that reduce unfinished work quickly"
        }
        execute::DominantInstinct::Curiosity => "prefer candidates that reduce uncertainty",
        _ => "prefer candidates that preserve bounded coherence",
    };
    let salience_level = if selected.severity == "high" || selected.evidence.failure_count > 0 {
        "high"
    } else if selected.evidence.retry_count > 0 {
        "moderate"
    } else {
        "low"
    };
    let persistence_pressure = if completion_pressure == "elevated" {
        "retry_biased"
    } else if selected.evidence.retry_count > 0 {
        "stabilize_then_retry"
    } else {
        "bounded_once"
    };
    let confidence_shift = if selected.evidence.failure_count > 0 {
        "reduced"
    } else {
        "stable"
    };
    let downstream_influence = format!(
        "dominant_instinct={} selected_intent={} failure_count={} retry_count={}",
        dominant_instinct,
        selected.proposed_change.intent,
        selected.evidence.failure_count,
        selected.evidence.retry_count
    );

    CognitiveSignalsState {
        dominant_instinct,
        completion_pressure: completion_pressure.to_string(),
        integrity_bias: integrity_bias.to_string(),
        curiosity_bias: curiosity_bias.to_string(),
        candidate_selection_bias: candidate_selection_bias.to_string(),
        urgency_level: completion_pressure.to_string(),
        salience_level: salience_level.to_string(),
        persistence_pressure: persistence_pressure.to_string(),
        confidence_shift: confidence_shift.to_string(),
        downstream_influence,
    }
}

#[cfg(test)]
pub(crate) fn build_cognitive_signals_artifact(
    run_summary: &RunSummaryArtifact,
    suggestions: &SuggestionsArtifact,
    scores: Option<&ScoresArtifact>,
) -> CognitiveSignalsArtifact {
    let state = build_cognitive_signals_state(run_summary, suggestions, scores);
    build_cognitive_signals_artifact_from_state(run_summary, &state, suggestions, scores)
}

pub(crate) fn build_cognitive_signals_artifact_from_state(
    run_summary: &RunSummaryArtifact,
    state: &execute::CognitiveSignalsState,
    suggestions: &SuggestionsArtifact,
    scores: Option<&ScoresArtifact>,
) -> CognitiveSignalsArtifact {
    CognitiveSignalsArtifact {
        cognitive_signals_version: COGNITIVE_SIGNALS_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: suggestions.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        instinct: CognitiveInstinctRecord {
            instinct_profile_id: "instinct-001".to_string(),
            dominant_instinct: state.dominant_instinct.as_str().to_string(),
            completion_pressure: state.completion_pressure.clone(),
            integrity_bias: state.integrity_bias.clone(),
            curiosity_bias: state.curiosity_bias.clone(),
            candidate_selection_bias: state.candidate_selection_bias.clone(),
            deterministic_update_rule:
                "derive bounded instinct profile from stable failure, retry, security, and success evidence ordering"
                    .to_string(),
        },
        affect: CognitiveAffectSignalRecord {
            affect_state_id: "signal-affect-001".to_string(),
            urgency_level: state.urgency_level.clone(),
            salience_level: state.salience_level.clone(),
            persistence_pressure: state.persistence_pressure.clone(),
            confidence_shift: state.confidence_shift.clone(),
            downstream_influence: state.downstream_influence.clone(),
            deterministic_update_rule:
                "derive bounded affect signals from the first stable suggestion plus bounded run summary evidence"
                    .to_string(),
        },
    }
}

#[cfg(test)]
pub(crate) fn build_cognitive_arbitration_state(
    _run_summary: &RunSummaryArtifact,
    suggestions: &SuggestionsArtifact,
    signals: &CognitiveSignalsArtifact,
    affect_state: &AffectStateArtifact,
) -> CognitiveArbitrationState {
    let selected = suggestions
        .suggestions
        .first()
        .cloned()
        .unwrap_or_else(|| SuggestionItem {
            id: "sug-000".to_string(),
            category: "stability".to_string(),
            severity: "info".to_string(),
            rationale: "No bounded adaptation signals fired; keep current policy state."
                .to_string(),
            evidence: SuggestionEvidence {
                failure_count: 0,
                retry_count: 0,
                delegation_denied_count: 0,
                security_denied_count: 0,
                success_ratio: 1.0,
                scheduler_max_parallel_observed: 1,
            },
            proposed_change: SuggestedChangeIntent {
                intent: "maintain_current_policy".to_string(),
                target: "workflow-runtime".to_string(),
            },
        });

    let (route_selected, reasoning_mode) = if selected.evidence.security_denied_count > 0
        || selected.evidence.failure_count > 0
        || signals.instinct.integrity_bias == "reinforced"
    {
        (execute::Route::Slow, "review_heavy")
    } else if affect_state.affect.recovery_bias >= 2
        || selected.evidence.retry_count > 0
        || signals.affect.confidence_shift == "reduced"
        || signals.affect.persistence_pressure == "sustained"
    {
        (execute::Route::Hybrid, "bounded_recovery")
    } else {
        (execute::Route::Fast, "direct_execution")
    };
    let risk_class = if selected.evidence.security_denied_count > 0 {
        "high"
    } else if selected.evidence.failure_count > 0 || affect_state.affect.recovery_bias >= 2 {
        "medium"
    } else {
        "low"
    };
    let confidence = if route_selected == execute::Route::Fast {
        "high"
    } else if route_selected == execute::Route::Hybrid {
        "guarded"
    } else {
        "review_required"
    };
    let mut applied_constraints = Vec::new();
    if selected.evidence.security_denied_count > 0 {
        applied_constraints.push("security_denial_present".to_string());
    }
    if selected.evidence.failure_count > 0 {
        applied_constraints.push("failure_recovery_bias".to_string());
    }
    if selected.evidence.retry_count > 0 {
        applied_constraints.push("retry_budget_pressure".to_string());
    }
    if applied_constraints.is_empty() {
        applied_constraints.push("bounded_default_path".to_string());
    }

    let cost_latency_assumption = match route_selected {
        execute::Route::Fast => {
            "prefer lower-cost low-latency execution when bounded evidence is stable"
        }
        execute::Route::Hybrid => {
            "allow bounded extra review when retry or recovery pressure is present"
        }
        _ => "spend bounded additional cognition when failure or policy risk is present",
    };
    let route_reason = format!(
        "route={} dominant_instinct={} confidence_shift={} affect_mode={} failure_count={} retry_count={} security_denied_count={} selected_intent={}",
        route_selected.as_str(),
        signals.instinct.dominant_instinct,
        signals.affect.confidence_shift,
        affect_state.affect.affect_mode,
        selected.evidence.failure_count,
        selected.evidence.retry_count,
        selected.evidence.security_denied_count,
        selected.proposed_change.intent
    );

    CognitiveArbitrationState {
        route_selected,
        reasoning_mode: reasoning_mode.to_string(),
        confidence: confidence.to_string(),
        risk_class: risk_class.to_string(),
        applied_constraints,
        cost_latency_assumption: cost_latency_assumption.to_string(),
        route_reason,
    }
}

#[cfg(test)]
pub(crate) fn build_cognitive_arbitration_artifact(
    run_summary: &RunSummaryArtifact,
    suggestions: &SuggestionsArtifact,
    signals: &CognitiveSignalsArtifact,
    affect_state: &AffectStateArtifact,
    scores: Option<&ScoresArtifact>,
) -> CognitiveArbitrationArtifact {
    let state = build_cognitive_arbitration_state(run_summary, suggestions, signals, affect_state);
    build_cognitive_arbitration_artifact_from_state(run_summary, suggestions, &state, scores)
}

pub(crate) fn build_cognitive_arbitration_artifact_from_state(
    run_summary: &RunSummaryArtifact,
    suggestions: &SuggestionsArtifact,
    state: &execute::CognitiveArbitrationState,
    scores: Option<&ScoresArtifact>,
) -> CognitiveArbitrationArtifact {
    CognitiveArbitrationArtifact {
        cognitive_arbitration_version: COGNITIVE_ARBITRATION_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: suggestions.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        route_selected: state.route_selected.as_str().to_string(),
        reasoning_mode: state.reasoning_mode.clone(),
        confidence: state.confidence.clone(),
        risk_class: state.risk_class.clone(),
        applied_constraints: state.applied_constraints.clone(),
        cost_latency_assumption: state.cost_latency_assumption.clone(),
        route_reason: state.route_reason.clone(),
        deterministic_selection_rule:
            "derive route from runtime signal state, bounded affect recovery bias, and stable failure/security/retry evidence ordering"
                .to_string(),
    }
}

#[cfg(test)]
pub(crate) fn build_fast_slow_path_state(
    arbitration: &CognitiveArbitrationArtifact,
) -> FastSlowPathState {
    let route_selected = match arbitration.route_selected.as_str() {
        "fast" => execute::Route::Fast,
        "hybrid" => execute::Route::Hybrid,
        "slow" => execute::Route::Slow,
        other => panic!("unexpected arbitration route_selected value: {other}"),
    };
    let (
        selected_path,
        path_family,
        runtime_branch_taken,
        handoff_state,
        candidate_strategy,
        review_depth,
        execution_profile,
        termination_expectation,
    ) = match route_selected {
        execute::Route::Fast => (
            execute::SelectedPath::FastPath,
            "fast",
            "fast_direct_execution_branch",
            "direct_handoff",
            "accept first bounded candidate",
            "minimal",
            "single_pass_direct_execution",
            "terminate_on_first_bounded_success_or_policy_block",
        ),
        execute::Route::Hybrid => (
            execute::SelectedPath::SlowPath,
            "slow",
            "slow_bounded_recovery_branch",
            "bounded_recovery_handoff",
            "compare current candidate against one bounded refinement",
            "bounded_recovery_review",
            "review_then_execute_once",
            "terminate_after_bounded_review_cycle_or_policy_block",
        ),
        _ => (
            execute::SelectedPath::SlowPath,
            "slow",
            "slow_review_refine_branch",
            "review_handoff",
            "validate, refine, or veto the current bounded candidate",
            "verification_required",
            "review_and_refine_before_execution",
            "terminate_after_bounded_review_cycle_or_policy_block",
        ),
    };
    let path_difference_summary = match selected_path {
        execute::SelectedPath::FastPath => {
            "fast_path favors direct execution with minimal review and a single bounded candidate handoff"
        }
        execute::SelectedPath::SlowPath => {
            "slow_path requires bounded review/refinement before execution and can revise or veto the current candidate"
        }
    };

    FastSlowPathState {
        selected_path,
        path_family: path_family.to_string(),
        runtime_branch_taken: runtime_branch_taken.to_string(),
        handoff_state: handoff_state.to_string(),
        candidate_strategy: candidate_strategy.to_string(),
        review_depth: review_depth.to_string(),
        execution_profile: execution_profile.to_string(),
        termination_expectation: termination_expectation.to_string(),
        path_difference_summary: path_difference_summary.to_string(),
    }
}

pub(crate) fn build_fast_slow_path_artifact(
    run_summary: &RunSummaryArtifact,
    arbitration: &CognitiveArbitrationArtifact,
    state: &execute::FastSlowPathState,
    scores: Option<&ScoresArtifact>,
) -> FastSlowPathArtifact {
    FastSlowPathArtifact {
        fast_slow_path_version: FAST_SLOW_PATH_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: arbitration.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        arbitration_route: arbitration.route_selected.clone(),
        selected_path: state.selected_path.as_str().to_string(),
        path_family: state.path_family.clone(),
        runtime_branch_taken: state.runtime_branch_taken.clone(),
        handoff_state: state.handoff_state.clone(),
        candidate_strategy: state.candidate_strategy.clone(),
        review_depth: state.review_depth.clone(),
        execution_profile: state.execution_profile.clone(),
        termination_expectation: state.termination_expectation.clone(),
        path_difference_summary: state.path_difference_summary.clone(),
        deterministic_handoff_rule:
            "derive fast/slow path handoff and branch selection directly from bounded arbitration route selection before downstream candidate generation"
                .to_string(),
    }
}

pub(crate) fn build_reframing_artifact(
    run_summary: &RunSummaryArtifact,
    fast_slow_path: &FastSlowPathArtifact,
    agency_selection: &AgencySelectionArtifact,
    state: &ReframingControlState,
    scores: Option<&ScoresArtifact>,
) -> ReframingArtifact {
    ReframingArtifact {
        reframing_version: REFRAMING_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: agency_selection.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        selected_candidate_id: agency_selection.selected_candidate_id.clone(),
        selected_path: fast_slow_path.selected_path.clone(),
        frame_adequacy_score: state.frame_adequacy_score,
        reframing_trigger: state.reframing_trigger.clone(),
        reframing_reason: state.reframing_reason.clone(),
        prior_frame: state.prior_frame.clone(),
        new_frame: state.new_frame.clone(),
        reexecution_choice: state.reexecution_choice.clone(),
        post_reframe_state: state.post_reframe_state.clone(),
        deterministic_reframing_rule:
            "derive bounded frame adequacy, reframing trigger, and re-execution choice from execute-owned evaluation and bounded execution state without hidden retry loops"
                .to_string(),
    }
}

pub(crate) fn build_freedom_gate_artifact(
    run_summary: &RunSummaryArtifact,
    evaluation_signals: &EvaluationSignalsArtifact,
    state: &FreedomGateState,
    scores: Option<&ScoresArtifact>,
) -> FreedomGateArtifact {
    FreedomGateArtifact {
        freedom_gate_version: FREEDOM_GATE_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: evaluation_signals.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        input: state.input.clone(),
        gate_decision: state.gate_decision.clone(),
        reason_code: state.reason_code.clone(),
        decision_reason: state.decision_reason.clone(),
        selected_action_or_none: state.selected_action_or_none.clone(),
        commitment_blocked: state.commitment_blocked,
        judgment_boundary: state.judgment_boundary.clone(),
        required_follow_up: state.required_follow_up.clone(),
        decision_record_kind: state.decision_record_kind.clone(),
        deterministic_gate_rule:
            "derive allow/defer/refuse/escalate judgment decisions from execute-owned freedom-gate input state before action commitment and without hidden bypass paths"
                .to_string(),
    }
}

pub(crate) fn build_aee_convergence_artifact(
    run_summary: &RunSummaryArtifact,
    execution: &BoundedExecutionArtifact,
    evaluation: &EvaluationSignalsArtifact,
    reframing: &ReframingArtifact,
    freedom_gate: &FreedomGateArtifact,
    scores: Option<&ScoresArtifact>,
) -> AeeConvergenceArtifact {
    let convergence_state = if freedom_gate.commitment_blocked
        || matches!(
            freedom_gate.gate_decision.as_str(),
            "defer" | "refuse" | "escalate"
        ) {
        "policy_stop"
    } else if evaluation.termination_reason == "success" {
        "converged"
    } else if evaluation.next_control_action == "await_resume" {
        "handoff"
    } else if evaluation.next_control_action == "handoff_to_reframing" {
        "stalled"
    } else {
        "bounded_out"
    };

    let stop_condition_family = if freedom_gate.commitment_blocked
        || matches!(
            freedom_gate.gate_decision.as_str(),
            "defer" | "refuse" | "escalate"
        ) {
        "policy_boundary"
    } else {
        match evaluation.termination_reason.as_str() {
            "success" => "acceptance_satisfied",
            "pause_boundary" => "handoff_or_missing_input",
            "no_progress" => "no_meaningful_improvement",
            "bounded_failure" => "bounded_failure_cluster",
            _ => "bounded_runtime_stop",
        }
    };

    let stage_shift_count = execution
        .iterations
        .windows(2)
        .filter(|window| window[0].stage != window[1].stage)
        .count() as u32;
    let strategy_change_count =
        stage_shift_count + u32::from(reframing.reframing_trigger == "triggered");
    let strategy_change_visible = strategy_change_count > 0;

    let reviewer_summary = format!(
        "AEE convergence ended as '{}' after {} bounded iteration(s); progress signal '{}' led to stop family '{}' with next control action '{}' and gate decision '{}'.",
        convergence_state,
        execution.iteration_count,
        evaluation.progress_signal,
        stop_condition_family,
        evaluation.next_control_action,
        freedom_gate.gate_decision
    );

    AeeConvergenceArtifact {
        aee_convergence_version: AEE_CONVERGENCE_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: evaluation.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        selected_candidate_id: execution.selected_candidate_id.clone(),
        selected_path: execution.selected_path.clone(),
        convergence_state: convergence_state.to_string(),
        progress_signal: evaluation.progress_signal.clone(),
        stop_condition_family: stop_condition_family.to_string(),
        termination_reason: evaluation.termination_reason.clone(),
        next_control_action: evaluation.next_control_action.clone(),
        gate_decision: freedom_gate.gate_decision.clone(),
        iteration_count: execution.iteration_count,
        strategy_change_count,
        strategy_change_visible,
        reframing_trigger: reframing.reframing_trigger.clone(),
        reviewer_summary,
        deterministic_convergence_rule:
            "derive convergence, stall, bounded-out, policy-stop, or handoff from bounded execution, evaluation, reframing, and freedom-gate evidence without hidden retry state"
                .to_string(),
    }
}

pub(crate) fn build_memory_read_artifact(
    run_summary: &RunSummaryArtifact,
    evaluation_signals: &EvaluationSignalsArtifact,
    state: &MemoryReadState,
    scores: Option<&ScoresArtifact>,
) -> MemoryReadArtifact {
    MemoryReadArtifact {
        memory_read_version: MEMORY_READ_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: evaluation_signals.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        query: state.query.clone(),
        read_count: state.entries.len() as u32,
        entries: state.entries.clone(),
        retrieval_order: state.retrieval_order.clone(),
        influence_summary: state.influence_summary.clone(),
        influenced_stage: state.influenced_stage.clone(),
        deterministic_read_rule:
            "derive bounded memory reads from execute-owned runtime state and stable indexed run artifacts without hidden retrieval side effects"
                .to_string(),
    }
}

pub(crate) fn build_memory_write_artifact(
    run_summary: &RunSummaryArtifact,
    evaluation_signals: &EvaluationSignalsArtifact,
    state: &MemoryWriteState,
    scores: Option<&ScoresArtifact>,
) -> MemoryWriteArtifact {
    MemoryWriteArtifact {
        memory_write_version: MEMORY_WRITE_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: evaluation_signals.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        entry_id: state.entry_id.clone(),
        content: state.content.clone(),
        tags: state.tags.clone(),
        logical_timestamp: state.logical_timestamp.clone(),
        write_reason: state.write_reason.clone(),
        source: state.source.clone(),
        deterministic_write_rule:
            "derive bounded memory write state from execute-owned runtime control without hidden persistence side effects"
                .to_string(),
    }
}

pub(crate) fn build_control_path_memory_artifact(
    run_summary: &RunSummaryArtifact,
    read: &MemoryReadArtifact,
    write: &MemoryWriteArtifact,
) -> ControlPathMemoryArtifact {
    ControlPathMemoryArtifact {
        control_path_memory_version: CONTROL_PATH_MEMORY_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: read.generated_from.clone(),
        read: read.clone(),
        write: write.clone(),
    }
}
