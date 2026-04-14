use std::collections::BTreeMap;

use super::*;

fn aee_decision_kind_for_intent(intent: &str) -> (&'static str, &'static str) {
    match intent {
        "increase_step_retry_budget" => (
            "bounded_retry_recovery",
            "raise retry budget for the failed step set on the next bounded run",
        ),
        "evaluate_parallelizable_dependencies" => (
            "bounded_scheduler_review",
            "review whether the workflow can safely increase bounded parallelism",
        ),
        "review_delegation_policy_scope" => (
            "bounded_delegation_review",
            "review delegation boundaries before the next bounded run",
        ),
        "review_security_policy_expectations" => (
            "bounded_security_review",
            "review trust-policy expectations before the next bounded run",
        ),
        "review_failure_hotspots" => (
            "bounded_failure_review",
            "review failing dependency hotspots before the next bounded run",
        ),
        _ => (
            "bounded_runtime_review",
            "review bounded runtime signals before the next run",
        ),
    }
}

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
        "integrity"
    } else if completion_pressure == "elevated" {
        "completion"
    } else if curiosity_bias == "active" {
        "curiosity"
    } else {
        "coherence"
    };
    let candidate_selection_bias = match dominant_instinct {
        "integrity" => "prefer lower-risk constrained candidates",
        "completion" => "prefer candidates that reduce unfinished work quickly",
        "curiosity" => "prefer candidates that reduce uncertainty",
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
        dominant_instinct: dominant_instinct.to_string(),
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
            dominant_instinct: state.dominant_instinct.clone(),
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
        ("slow", "review_heavy")
    } else if affect_state.affect.recovery_bias >= 2
        || selected.evidence.retry_count > 0
        || signals.affect.confidence_shift == "reduced"
        || signals.affect.persistence_pressure == "sustained"
    {
        ("hybrid", "bounded_recovery")
    } else {
        ("fast", "direct_execution")
    };
    let risk_class = if selected.evidence.security_denied_count > 0 {
        "high"
    } else if selected.evidence.failure_count > 0 || affect_state.affect.recovery_bias >= 2 {
        "medium"
    } else {
        "low"
    };
    let confidence = if route_selected == "fast" {
        "high"
    } else if route_selected == "hybrid" {
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
        "fast" => "prefer lower-cost low-latency execution when bounded evidence is stable",
        "hybrid" => "allow bounded extra review when retry or recovery pressure is present",
        _ => "spend bounded additional cognition when failure or policy risk is present",
    };
    let route_reason = format!(
        "route={} dominant_instinct={} confidence_shift={} affect_mode={} failure_count={} retry_count={} security_denied_count={} selected_intent={}",
        route_selected,
        signals.instinct.dominant_instinct,
        signals.affect.confidence_shift,
        affect_state.affect.affect_mode,
        selected.evidence.failure_count,
        selected.evidence.retry_count,
        selected.evidence.security_denied_count,
        selected.proposed_change.intent
    );

    CognitiveArbitrationState {
        route_selected: route_selected.to_string(),
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
        route_selected: state.route_selected.clone(),
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
    let (
        selected_path,
        path_family,
        runtime_branch_taken,
        handoff_state,
        candidate_strategy,
        review_depth,
        execution_profile,
        termination_expectation,
    ) = match arbitration.route_selected.as_str() {
        "fast" => (
            "fast_path",
            "fast",
            "fast_direct_execution_branch",
            "direct_handoff",
            "accept first bounded candidate",
            "minimal",
            "single_pass_direct_execution",
            "terminate_on_first_bounded_success_or_policy_block",
        ),
        "hybrid" => (
            "slow_path",
            "slow",
            "slow_bounded_recovery_branch",
            "bounded_recovery_handoff",
            "compare current candidate against one bounded refinement",
            "bounded_recovery_review",
            "review_then_execute_once",
            "terminate_after_bounded_review_cycle_or_policy_block",
        ),
        _ => (
            "slow_path",
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
        "fast_path" => {
            "fast_path favors direct execution with minimal review and a single bounded candidate handoff"
        }
        _ => {
            "slow_path requires bounded review/refinement before execution and can revise or veto the current candidate"
        }
    };

    FastSlowPathState {
        selected_path: selected_path.to_string(),
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
        selected_path: state.selected_path.clone(),
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

#[cfg(test)]
pub(crate) fn build_agency_selection_state(
    signals: &CognitiveSignalsArtifact,
    arbitration: &CognitiveArbitrationArtifact,
    fast_slow_state: &FastSlowPathState,
    fast_slow_path: &FastSlowPathArtifact,
) -> AgencySelectionState {
    let (
        selection_mode,
        candidate_set,
        selected_candidate_id,
        selected_candidate_kind,
        selected_candidate_action,
        selected_candidate_reason,
    ) = match fast_slow_path.selected_path.as_str() {
        "fast_path" => {
            let candidate_set = vec![
                    AgencyCandidateRecord {
                        candidate_id: "cand-fast-execute".to_string(),
                        candidate_kind: "direct_execution".to_string(),
                        bounded_action: "execute selected candidate directly under bounded once semantics".to_string(),
                        review_requirement: "minimal".to_string(),
                        execution_priority: 1,
                        rationale: format!(
                            "route={} dominant_instinct={} confidence={}",
                            arbitration.route_selected, signals.instinct.dominant_instinct, arbitration.confidence
                        ),
                    },
                    AgencyCandidateRecord {
                        candidate_id: "cand-fast-verify".to_string(),
                        candidate_kind: "bounded_verification".to_string(),
                        bounded_action: "perform one bounded verification pass before execution".to_string(),
                        review_requirement: "light".to_string(),
                        execution_priority: 2,
                        rationale: "keep a bounded verification candidate available when instinct pressure favors uncertainty reduction or extra constraint checks".to_string(),
                    },
                ];
            let decision = execute::select_instinct_runtime_candidate(
                fast_slow_path.selected_path.as_str(),
                signals.instinct.dominant_instinct.as_str(),
                arbitration.risk_class.as_str(),
            );
            (
                decision.selection_mode,
                candidate_set,
                decision.candidate_id.to_string(),
                decision.candidate_kind.to_string(),
                decision.candidate_action.to_string(),
                decision.candidate_reason.to_string(),
            )
        }
        _ => {
            let candidate_set = vec![
                    AgencyCandidateRecord {
                        candidate_id: "cand-slow-review".to_string(),
                        candidate_kind: "review_and_refine".to_string(),
                        bounded_action: "review, refine, or veto the current candidate before execution".to_string(),
                        review_requirement: "verification_required".to_string(),
                        execution_priority: 1,
                        rationale: format!(
                            "route={} dominant_instinct={} risk_class={}",
                            arbitration.route_selected, signals.instinct.dominant_instinct, arbitration.risk_class
                        ),
                    },
                    AgencyCandidateRecord {
                        candidate_id: "cand-slow-direct".to_string(),
                        candidate_kind: "direct_execution".to_string(),
                        bounded_action: "execute the current candidate without additional refinement".to_string(),
                        review_requirement: "minimal".to_string(),
                        execution_priority: 2,
                        rationale: "retain the direct-execution alternative when completion pressure can still justify a bounded finish-first move".to_string(),
                    },
                    AgencyCandidateRecord {
                        candidate_id: "cand-slow-defer".to_string(),
                        candidate_kind: "bounded_deferral".to_string(),
                        bounded_action: "defer execution and surface the candidate set for later gate/review stages".to_string(),
                        review_requirement: "review_required".to_string(),
                        execution_priority: 3,
                        rationale: "preserve a bounded non-execution option when curiosity keeps uncertainty high or the system should pause before commitment".to_string(),
                    },
                ];
            let decision = execute::select_instinct_runtime_candidate(
                fast_slow_path.selected_path.as_str(),
                signals.instinct.dominant_instinct.as_str(),
                arbitration.risk_class.as_str(),
            );
            (
                decision.selection_mode,
                candidate_set,
                decision.candidate_id.to_string(),
                decision.candidate_kind.to_string(),
                decision.candidate_action.to_string(),
                decision.candidate_reason.to_string(),
            )
        }
    };

    AgencySelectionState {
        candidate_generation_basis: format!(
            "path={} runtime_branch={} route={} candidate_selection_bias={}",
            fast_slow_path.selected_path,
            fast_slow_state.runtime_branch_taken,
            arbitration.route_selected,
            signals.instinct.candidate_selection_bias
        ),
        selection_mode: selection_mode.to_string(),
        candidate_set,
        selected_candidate_id,
        selected_candidate_kind,
        selected_candidate_action,
        selected_candidate_reason,
    }
}

pub(crate) fn build_agency_selection_artifact(
    run_summary: &RunSummaryArtifact,
    arbitration: &CognitiveArbitrationArtifact,
    state: &execute::AgencySelectionState,
    scores: Option<&ScoresArtifact>,
) -> AgencySelectionArtifact {
    AgencySelectionArtifact {
        agency_selection_version: AGENCY_SELECTION_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: arbitration.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        candidate_generation_basis: state.candidate_generation_basis.clone(),
        selection_mode: state.selection_mode.clone(),
        candidate_set: state.candidate_set.clone(),
        selected_candidate_id: state.selected_candidate_id.clone(),
        selected_candidate_reason: state.selected_candidate_reason.clone(),
        deterministic_selection_rule:
            "derive the bounded candidate set and selected candidate from the already-selected fast/slow runtime branch, arbitration route, and instinct bias without hidden initiative state"
                .to_string(),
    }
}

#[cfg(test)]
pub(crate) fn build_bounded_execution_state(
    run_summary: &RunSummaryArtifact,
    _fast_slow_path: &FastSlowPathArtifact,
    _agency_selection: &AgencySelectionArtifact,
    agency_state: &AgencySelectionState,
) -> BoundedExecutionState {
    let (execution_status, continuation_state, provisional_termination_state, iterations) =
        match agency_state.selected_candidate_kind.as_str() {
            "direct_execution" => (
                "completed",
                "stop_after_one",
                "ready_for_evaluation",
                vec![BoundedExecutionIteration {
                    iteration_index: 1,
                    stage: "execute".to_string(),
                    action: agency_state.selected_candidate_action.clone(),
                    outcome: "bounded_direct_execution_complete".to_string(),
                }],
            ),
            "review_and_refine" => (
                "completed",
                "bounded_review_complete",
                "ready_for_evaluation",
                vec![
                    BoundedExecutionIteration {
                        iteration_index: 1,
                        stage: "review".to_string(),
                        action: agency_state.selected_candidate_action.clone(),
                        outcome: "bounded_review_pass_complete".to_string(),
                    },
                    BoundedExecutionIteration {
                        iteration_index: 2,
                        stage: "execute".to_string(),
                        action: "execute the reviewed bounded candidate".to_string(),
                        outcome: "bounded_reviewed_execution_complete".to_string(),
                    },
                ],
            ),
            _ => (
                "completed",
                "deferred",
                "ready_for_evaluation",
                vec![BoundedExecutionIteration {
                    iteration_index: 1,
                    stage: "defer".to_string(),
                    action: agency_state.selected_candidate_action.clone(),
                    outcome: "bounded_deferral_recorded".to_string(),
                }],
            ),
        };

    let execution_status = if run_summary.status == "failure" {
        "completed_with_failure_signal"
    } else {
        execution_status
    };
    let continuation_state = if run_summary.status == "failure" && iterations.len() > 1 {
        "bounded_review_complete_with_failure_signal"
    } else {
        continuation_state
    };
    let provisional_termination_state = if run_summary.status == "failure" {
        "ready_for_runtime_evaluation"
    } else {
        provisional_termination_state
    };

    BoundedExecutionState {
        execution_status: execution_status.to_string(),
        continuation_state: continuation_state.to_string(),
        provisional_termination_state: provisional_termination_state.to_string(),
        iterations,
    }
}

pub(crate) fn build_bounded_execution_artifact(
    run_summary: &RunSummaryArtifact,
    fast_slow_path: &FastSlowPathArtifact,
    agency_selection: &AgencySelectionArtifact,
    state: &BoundedExecutionState,
    scores: Option<&ScoresArtifact>,
) -> BoundedExecutionArtifact {
    BoundedExecutionArtifact {
        bounded_execution_version: BOUNDED_EXECUTION_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: agency_selection.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        selected_candidate_id: agency_selection.selected_candidate_id.clone(),
        selected_path: fast_slow_path.selected_path.clone(),
        execution_status: state.execution_status.clone(),
        continuation_state: state.continuation_state.clone(),
        provisional_termination_state: state.provisional_termination_state.clone(),
        iteration_count: state.iterations.len() as u32,
        iterations: state.iterations.clone(),
        deterministic_execution_rule:
            "derive bounded iteration shape, continuation state, and provisional termination from runtime loop state without hidden retry state"
                .to_string(),
    }
}

#[cfg(test)]
pub(crate) fn build_evaluation_control_state(
    run_summary: &RunSummaryArtifact,
    bounded_execution: &BoundedExecutionArtifact,
) -> EvaluationControlState {
    let (
        progress_signal,
        contradiction_signal,
        failure_signal,
        termination_reason,
        behavior_effect,
        next_control_action,
    ) = if run_summary.status == "failure" {
        (
            "stalled_progress",
            "present",
            "bounded_failure_detected",
            if bounded_execution.iteration_count > 1 {
                "bounded_failure"
            } else {
                "no_progress"
            },
            "emit bounded failure/termination signals for later reframing or policy handling",
            if bounded_execution.iteration_count > 1 {
                "handoff_to_reframing"
            } else {
                "terminate_with_failure"
            },
        )
    } else {
        (
            "steady_progress",
            "none",
            "none",
            "success",
            "allow bounded execution to terminate cleanly after evaluation confirms progress",
            "complete_run",
        )
    };

    EvaluationControlState {
        progress_signal: progress_signal.to_string(),
        contradiction_signal: contradiction_signal.to_string(),
        failure_signal: failure_signal.to_string(),
        termination_reason: termination_reason.to_string(),
        behavior_effect: behavior_effect.to_string(),
        next_control_action: next_control_action.to_string(),
    }
}

pub(crate) fn build_evaluation_signals_artifact(
    run_summary: &RunSummaryArtifact,
    fast_slow_path: &FastSlowPathArtifact,
    agency_selection: &AgencySelectionArtifact,
    state: &EvaluationControlState,
    scores: Option<&ScoresArtifact>,
) -> EvaluationSignalsArtifact {
    EvaluationSignalsArtifact {
        evaluation_signals_version: EVALUATION_SIGNALS_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: agency_selection.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        selected_candidate_id: agency_selection.selected_candidate_id.clone(),
        selected_path: fast_slow_path.selected_path.clone(),
        progress_signal: state.progress_signal.clone(),
        contradiction_signal: state.contradiction_signal.clone(),
        failure_signal: state.failure_signal.clone(),
        termination_reason: state.termination_reason.clone(),
        behavior_effect: state.behavior_effect.clone(),
        next_control_action: state.next_control_action.clone(),
        deterministic_evaluation_rule:
            "derive bounded evaluation, termination, and next control action from runtime execution evidence without hidden continuation state"
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

fn route_outcome_class(route_selected: &str) -> &'static str {
    if route_selected == "fast" {
        "accept"
    } else {
        "reroute"
    }
}

fn reframing_outcome_class(reframing_trigger: &str) -> &'static str {
    if reframing_trigger == "triggered" {
        "reroute"
    } else {
        "accept"
    }
}

fn gate_outcome_class(gate_decision: &str) -> &'static str {
    match gate_decision {
        "allow" => "accept",
        "refuse" => "reject",
        "defer" => "defer",
        "escalate" => "escalate",
        _ => "reject",
    }
}

fn proposal_kind_for_candidate_kind(candidate_kind: &str) -> &'static str {
    match candidate_kind {
        "bounded_deferral" => "defer",
        "refusal" => "refuse",
        "memory_read" => "memory_read",
        "memory_write" => "memory_write",
        "final_answer" => "final_answer",
        "tool_call" => "tool_call",
        _ => "skill_call",
    }
}

fn proposal_target_for_candidate_kind(candidate_kind: &str) -> Option<String> {
    match proposal_kind_for_candidate_kind(candidate_kind) {
        "defer" | "refuse" | "final_answer" => None,
        _ => Some(format!("candidate.{}", candidate_kind.trim())),
    }
}

fn proposal_confidence_for_arbitration(confidence: &str) -> Option<f64> {
    match confidence.trim() {
        "high" => Some(0.9),
        "medium" => Some(0.72),
        "guarded" => Some(0.58),
        "reduced" => Some(0.41),
        "low" => Some(0.28),
        _ => None,
    }
}

fn mediation_outcome_for_gate_decision(gate_decision: &str) -> &'static str {
    match gate_decision {
        "allow" => "approved",
        "refuse" => "rejected",
        "defer" => "deferred",
        "escalate" => "escalated",
        _ => "rejected",
    }
}

pub(crate) fn build_control_path_action_proposals_artifact(
    run_summary: &RunSummaryArtifact,
    arbitration: &CognitiveArbitrationArtifact,
    agency: &AgencySelectionArtifact,
    freedom_gate: &FreedomGateArtifact,
    scores: Option<&ScoresArtifact>,
) -> ControlPathActionProposalsArtifact {
    let selected_candidate = agency
        .candidate_set
        .iter()
        .find(|candidate| candidate.candidate_id == agency.selected_candidate_id)
        .cloned()
        .unwrap_or_else(|| AgencyCandidateRecord {
            candidate_id: agency.selected_candidate_id.clone(),
            candidate_kind: "review_and_refine".to_string(),
            bounded_action: freedom_gate.input.candidate_action.clone(),
            review_requirement: "review_required".to_string(),
            execution_priority: 1,
            rationale: agency.selected_candidate_reason.clone(),
        });
    let mut arguments = BTreeMap::new();
    arguments.insert(
        "candidate_id".to_string(),
        agency.selected_candidate_id.clone(),
    );
    arguments.insert(
        "candidate_kind".to_string(),
        selected_candidate.candidate_kind.clone(),
    );
    arguments.insert(
        "requested_action".to_string(),
        selected_candidate.bounded_action.clone(),
    );
    arguments.insert(
        "route_selected".to_string(),
        arbitration.route_selected.clone(),
    );

    let mut metadata = BTreeMap::new();
    metadata.insert(
        "surface_id".to_string(),
        "decision.commitment_gate".to_string(),
    );
    metadata.insert(
        "decision_record_kind".to_string(),
        freedom_gate.decision_record_kind.clone(),
    );
    metadata.insert(
        "risk_class".to_string(),
        freedom_gate.input.risk_class.clone(),
    );

    ControlPathActionProposalsArtifact {
        control_path_action_proposals_version: CONTROL_PATH_ACTION_PROPOSALS_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: arbitration.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        proposal_schema_name: "adl.runtime.action_proposal.v1".to_string(),
        proposal_schema_fields: vec![
            "proposal_id".to_string(),
            "kind".to_string(),
            "target".to_string(),
            "arguments".to_string(),
            "intent".to_string(),
            "content".to_string(),
            "confidence".to_string(),
            "requires_approval".to_string(),
            "metadata".to_string(),
            "non_authoritative".to_string(),
            "temporal_anchor".to_string(),
        ],
        proposal_kind_vocabulary: vec![
            "tool_call".to_string(),
            "skill_call".to_string(),
            "memory_read".to_string(),
            "memory_write".to_string(),
            "final_answer".to_string(),
            "refuse".to_string(),
            "defer".to_string(),
        ],
        proposals: vec![ActionProposalRecord {
            proposal_id: "proposal.selected_candidate".to_string(),
            kind: proposal_kind_for_candidate_kind(&selected_candidate.candidate_kind).to_string(),
            target: proposal_target_for_candidate_kind(&selected_candidate.candidate_kind),
            arguments,
            intent: agency.selected_candidate_reason.clone(),
            content: None,
            confidence: proposal_confidence_for_arbitration(&arbitration.confidence),
            requires_approval: freedom_gate.input.policy_context.requires_review
                || freedom_gate.input.consequence_context.escalation_available
                || freedom_gate.commitment_blocked,
            metadata,
            non_authoritative: true,
            temporal_anchor: "control_path/candidate_selection.json".to_string(),
        }],
    }
}

pub(crate) fn build_control_path_decisions_artifact(
    run_summary: &RunSummaryArtifact,
    arbitration: &CognitiveArbitrationArtifact,
    agency: &AgencySelectionArtifact,
    evaluation: &EvaluationSignalsArtifact,
    reframing: &ReframingArtifact,
    freedom_gate: &FreedomGateArtifact,
    scores: Option<&ScoresArtifact>,
) -> ControlPathDecisionsArtifact {
    let route_policy_bindings = if arbitration.applied_constraints.is_empty() {
        vec!["no_explicit_constraints".to_string()]
    } else {
        arbitration.applied_constraints.clone()
    };
    let reframing_policy_bindings = vec![
        format!("frame_adequacy_score={}", reframing.frame_adequacy_score),
        format!("termination_reason={}", evaluation.termination_reason),
        format!("progress_signal={}", evaluation.progress_signal),
    ];
    let gate_policy_bindings = vec![
        format!(
            "route_selected={}",
            freedom_gate.input.policy_context.route_selected
        ),
        format!(
            "selected_candidate_kind={}",
            freedom_gate.input.policy_context.selected_candidate_kind
        ),
        format!(
            "requires_review={}",
            freedom_gate.input.policy_context.requires_review
        ),
        format!(
            "policy_blocked={}",
            freedom_gate.input.policy_context.policy_blocked
        ),
        format!(
            "impact_scope={}",
            freedom_gate.input.consequence_context.impact_scope
        ),
        format!(
            "operator_visibility={}",
            freedom_gate.input.consequence_context.operator_visibility
        ),
        format!(
            "escalation_available={}",
            freedom_gate.input.consequence_context.escalation_available
        ),
    ];

    ControlPathDecisionsArtifact {
        control_path_decisions_version: CONTROL_PATH_DECISIONS_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: arbitration.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        decision_schema_name: "adl.runtime.decision.v1".to_string(),
        decision_schema_fields: vec![
            "decision_id".to_string(),
            "surface_id".to_string(),
            "proposal_or_action".to_string(),
            "outcome_class".to_string(),
            "decision_maker".to_string(),
            "policy_bindings".to_string(),
            "rationale".to_string(),
            "downstream_consequence".to_string(),
            "temporal_anchor".to_string(),
        ],
        outcome_class_vocabulary: vec![
            "accept".to_string(),
            "reject".to_string(),
            "defer".to_string(),
            "escalate".to_string(),
            "reroute".to_string(),
        ],
        surfaces: vec![
            DecisionSurfaceRecord {
                surface_id: "delegation_and_routing.route_selection".to_string(),
                surface_family: "delegation_and_routing".to_string(),
                bounded_role:
                    "select the bounded runtime path before commitment is attempted".to_string(),
                outcome_classes: vec!["accept".to_string(), "reroute".to_string()],
                temporal_anchor_ref: "control_path/arbitration.json".to_string(),
            },
            DecisionSurfaceRecord {
                surface_id: "recovery_continuity.reframing".to_string(),
                surface_family: "recovery_continuity".to_string(),
                bounded_role:
                    "decide whether the current frame should be retained or rerouted through reframing"
                        .to_string(),
                outcome_classes: vec!["accept".to_string(), "reroute".to_string()],
                temporal_anchor_ref: "control_path/reframing.json".to_string(),
            },
            DecisionSurfaceRecord {
                surface_id: "pre_execution_authorization.commitment_gate".to_string(),
                surface_family: "pre_execution_authorization".to_string(),
                bounded_role:
                    "decide whether commitment may proceed for the selected bounded candidate"
                        .to_string(),
                outcome_classes: vec![
                    "accept".to_string(),
                    "reject".to_string(),
                    "defer".to_string(),
                    "escalate".to_string(),
                ],
                temporal_anchor_ref: "control_path/freedom_gate.json".to_string(),
            },
        ],
        decisions: vec![
            DecisionRecord {
                decision_id: "decision.route_selection".to_string(),
                surface_id: "delegation_and_routing.route_selection".to_string(),
                proposal_or_action: format!(
                    "route candidate {} through the {} path",
                    agency.selected_candidate_id, arbitration.route_selected
                ),
                outcome_class: route_outcome_class(&arbitration.route_selected).to_string(),
                decision_maker: "cognitive_arbitration".to_string(),
                policy_bindings: route_policy_bindings,
                rationale: arbitration.route_reason.clone(),
                downstream_consequence: format!(
                    "selected_path={} reasoning_mode={}",
                    arbitration.route_selected, arbitration.reasoning_mode
                ),
                temporal_anchor: "control_path/arbitration.json".to_string(),
            },
            DecisionRecord {
                decision_id: "decision.reframing".to_string(),
                surface_id: "recovery_continuity.reframing".to_string(),
                proposal_or_action: format!(
                    "decide whether candidate {} should keep the current frame or reframe before re-execution",
                    agency.selected_candidate_id
                ),
                outcome_class: reframing_outcome_class(&reframing.reframing_trigger).to_string(),
                decision_maker: "reframing_control".to_string(),
                policy_bindings: reframing_policy_bindings,
                rationale: reframing.reframing_reason.clone(),
                downstream_consequence: reframing.reexecution_choice.clone(),
                temporal_anchor: "control_path/reframing.json".to_string(),
            },
            DecisionRecord {
                decision_id: "decision.commitment_gate".to_string(),
                surface_id: "pre_execution_authorization.commitment_gate".to_string(),
                proposal_or_action: freedom_gate
                    .selected_action_or_none
                    .clone()
                    .filter(|value| !value.trim().is_empty())
                    .unwrap_or_else(|| {
                        let candidate_action = freedom_gate.input.candidate_action.trim();
                        if candidate_action.is_empty() {
                            "withhold commitment until bounded context is restored".to_string()
                        } else {
                            candidate_action.to_string()
                        }
                    }),
                outcome_class: gate_outcome_class(&freedom_gate.gate_decision).to_string(),
                decision_maker: "freedom_gate".to_string(),
                policy_bindings: gate_policy_bindings,
                rationale: freedom_gate.decision_reason.clone(),
                downstream_consequence: freedom_gate
                    .selected_action_or_none
                    .clone()
                    .unwrap_or_else(|| freedom_gate.required_follow_up.clone()),
                temporal_anchor: "control_path/freedom_gate.json".to_string(),
            },
        ],
    }
}

pub(crate) fn build_control_path_action_mediation_artifact(
    run_summary: &RunSummaryArtifact,
    action_proposals: &ControlPathActionProposalsArtifact,
    freedom_gate: &FreedomGateArtifact,
    decisions: &ControlPathDecisionsArtifact,
    scores: Option<&ScoresArtifact>,
) -> ControlPathActionMediationArtifact {
    let proposal = action_proposals
        .proposals
        .first()
        .cloned()
        .unwrap_or_else(|| ActionProposalRecord {
            proposal_id: "proposal.none".to_string(),
            kind: "defer".to_string(),
            target: None,
            arguments: BTreeMap::new(),
            intent: "no bounded proposal available".to_string(),
            content: None,
            confidence: None,
            requires_approval: true,
            metadata: BTreeMap::new(),
            non_authoritative: true,
            temporal_anchor: "control_path/candidate_selection.json".to_string(),
        });
    let gate_decision = decisions
        .decisions
        .iter()
        .find(|record| record.decision_id == "decision.commitment_gate")
        .cloned()
        .unwrap_or_else(|| DecisionRecord {
            decision_id: "decision.commitment_gate".to_string(),
            surface_id: "pre_execution_authorization.commitment_gate".to_string(),
            proposal_or_action: proposal.intent.clone(),
            outcome_class: gate_outcome_class(&freedom_gate.gate_decision).to_string(),
            decision_maker: "freedom_gate".to_string(),
            policy_bindings: Vec::new(),
            rationale: freedom_gate.decision_reason.clone(),
            downstream_consequence: freedom_gate.required_follow_up.clone(),
            temporal_anchor: "control_path/freedom_gate.json".to_string(),
        });
    let approved_action_or_none = if freedom_gate.gate_decision == "allow" {
        freedom_gate
            .selected_action_or_none
            .clone()
            .or_else(|| Some(freedom_gate.input.candidate_action.clone()))
    } else {
        None
    };

    ControlPathActionMediationArtifact {
        control_path_action_mediation_version: CONTROL_PATH_ACTION_MEDIATION_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: action_proposals.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        authority_boundary: "models_propose_runtime_decides_executes".to_string(),
        mediation_outcome_vocabulary: vec![
            "approved".to_string(),
            "rejected".to_string(),
            "deferred".to_string(),
            "escalated".to_string(),
        ],
        mediation: ActionMediationRecord {
            mediation_id: "mediation.commitment_gate".to_string(),
            proposal_id: proposal.proposal_id,
            decision_id: gate_decision.decision_id,
            runtime_authority: "freedom_gate".to_string(),
            judgment_boundary: freedom_gate.judgment_boundary.clone(),
            mediation_outcome: mediation_outcome_for_gate_decision(&freedom_gate.gate_decision)
                .to_string(),
            approved_action_or_none,
            required_follow_up: freedom_gate.required_follow_up.clone(),
            validation_checks: vec![
                "proposal_non_authoritative".to_string(),
                "decision_surface_linked".to_string(),
                "policy_bindings_present".to_string(),
                "freedom_gate_authority_boundary".to_string(),
            ],
            policy_bindings: gate_decision.policy_bindings,
            rationale: freedom_gate.decision_reason.clone(),
            temporal_anchor: "control_path/freedom_gate.json".to_string(),
            trace_expectation:
                "approval, rejection, defer, or escalation remains trace-visible before privileged execution"
                    .to_string(),
        },
    }
}

pub(crate) fn build_control_path_final_result_artifact(
    run_summary: &RunSummaryArtifact,
    arbitration: &CognitiveArbitrationArtifact,
    agency: &AgencySelectionArtifact,
    evaluation: &EvaluationSignalsArtifact,
    freedom_gate: &FreedomGateArtifact,
) -> ControlPathFinalResultArtifact {
    let final_result = match freedom_gate.gate_decision.as_str() {
        "allow" => freedom_gate
            .selected_action_or_none
            .clone()
            .or_else(|| {
                agency
                    .candidate_set
                    .iter()
                    .find(|candidate| candidate.candidate_id == agency.selected_candidate_id)
                    .map(|candidate| candidate.bounded_action.clone())
            })
            .unwrap_or_else(|| agency.selected_candidate_reason.clone()),
        "defer" => "defer".to_string(),
        "refuse" => "refuse".to_string(),
        "escalate" => "escalate".to_string(),
        other => format!("unrecognized_gate_decision:{other}"),
    };

    ControlPathFinalResultArtifact {
        control_path_final_result_version: CONTROL_PATH_FINAL_RESULT_VERSION,
        run_id: run_summary.run_id.clone(),
        route_selected: arbitration.route_selected.clone(),
        selected_candidate: agency.selected_candidate_id.clone(),
        termination_reason: evaluation.termination_reason.clone(),
        gate_decision: freedom_gate.gate_decision.clone(),
        final_result,
        commitment_blocked: freedom_gate.commitment_blocked,
        next_control_action: evaluation.next_control_action.clone(),
        stage_order: vec![
            "signals".to_string(),
            "candidate_selection".to_string(),
            "arbitration".to_string(),
            "execution".to_string(),
            "evaluation".to_string(),
            "reframing".to_string(),
            "memory".to_string(),
            "freedom_gate".to_string(),
            "final_result".to_string(),
        ],
    }
}

pub(crate) fn build_control_path_summary(context: &ControlPathSummaryContext<'_>) -> String {
    let signals = context.signals;
    let agency = context.agency;
    let arbitration = context.arbitration;
    let execution = context.execution;
    let evaluation = context.evaluation;
    let reframing = context.reframing;
    let convergence = context.convergence;
    let memory = context.memory;
    let action_proposals = context.action_proposals;
    let mediation = context.mediation;
    let freedom_gate = context.freedom_gate;
    let final_result = context.final_result;
    let proposal = action_proposals
        .proposals
        .first()
        .expect("control_path summary requires one proposal");

    [
        "v0.86 canonical bounded cognitive path summary".to_string(),
        format!("run_id: {}", final_result.run_id),
        "stage_order: signals -> candidate_selection -> arbitration -> execution -> evaluation -> reframing -> memory -> freedom_gate -> final_result".to_string(),
        format!(
            "signals: instinct={} completion_pressure={}",
            signals.instinct.dominant_instinct, signals.instinct.completion_pressure
        ),
        format!(
            "candidate_selection: candidate_id={} rationale={}",
            agency.selected_candidate_id, agency.selected_candidate_reason
        ),
        format!(
            "arbitration: route={} reasoning_mode={}",
            arbitration.route_selected, arbitration.reasoning_mode
        ),
        format!(
            "execution: status={} iterations={}",
            execution.execution_status, execution.iteration_count
        ),
        format!(
            "evaluation: termination_reason={} next_control_action={}",
            evaluation.termination_reason, evaluation.next_control_action
        ),
        format!(
            "reframing: trigger={} choice={}",
            reframing.reframing_trigger, reframing.reexecution_choice
        ),
        format!(
            "convergence: state={} stop_condition_family={} progress_signal={}",
            convergence.convergence_state,
            convergence.stop_condition_family,
            convergence.progress_signal
        ),
        format!(
            "decisions: route_selection={} reframing={} commitment_gate={}",
            route_outcome_class(&arbitration.route_selected),
            reframing_outcome_class(&reframing.reframing_trigger),
            gate_outcome_class(&freedom_gate.gate_decision)
        ),
        format!(
            "action_proposal: kind={} target={} requires_approval={}",
            proposal.kind,
            proposal.target.clone().unwrap_or_else(|| "<none>".to_string()),
            proposal.requires_approval
        ),
        format!(
            "action_mediation: outcome={} authority={} follow_up={}",
            mediation.mediation.mediation_outcome,
            mediation.mediation.runtime_authority,
            mediation.mediation.required_follow_up
        ),
        format!(
            "memory: read_count={} influenced_stage={} write_reason={}",
            memory.read.read_count, memory.read.influenced_stage, memory.write.write_reason
        ),
        format!(
            "freedom_gate: decision={} reason_code={} follow_up={} commitment_blocked={}",
            freedom_gate.gate_decision,
            freedom_gate.reason_code,
            freedom_gate.required_follow_up,
            freedom_gate.commitment_blocked
        ),
        format!("final_result: {}", final_result.final_result),
    ]
    .join("\n")
}

pub(crate) fn build_aee_decision_artifact(
    run_summary: &RunSummaryArtifact,
    suggestions: &SuggestionsArtifact,
    affect_state: &AffectStateArtifact,
    scores: Option<&ScoresArtifact>,
) -> AeeDecisionArtifact {
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
    let (decision_kind, expected_downstream_effect) =
        aee_decision_kind_for_intent(&selected.proposed_change.intent);
    let recommended_retry_budget = (selected.proposed_change.intent
        == "increase_step_retry_budget"
        && affect_state.affect.recovery_bias >= 2)
        .then_some(2);
    let expected_downstream_effect = if let Some(budget) = recommended_retry_budget {
        format!(
            "{expected_downstream_effect}; affect-guided recovery bias recommends retry budget max_attempts={budget}"
        )
    } else {
        expected_downstream_effect.to_string()
    };

    AeeDecisionArtifact {
        aee_decision_version: AEE_DECISION_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: suggestions.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        affect_state: AffectStateRef {
            affect_state_id: affect_state.affect.affect_state_id.clone(),
            affect_mode: affect_state.affect.affect_mode.clone(),
            downstream_priority: affect_state.affect.downstream_priority.clone(),
            recovery_bias: affect_state.affect.recovery_bias,
        },
        decision: AeeDecisionRecord {
            decision_id: "aee-001".to_string(),
            decision_kind: decision_kind.to_string(),
            selected_suggestion_id: selected.id,
            category: selected.category,
            intent: selected.proposed_change.intent,
            target: selected.proposed_change.target,
            rationale: selected.rationale,
            expected_downstream_effect,
            deterministic_selection_rule:
                "select the first suggestion emitted by build_suggestions_artifact after stable category ordering, then apply the deterministic affect-state recovery bias"
                    .to_string(),
            recommended_retry_budget,
            evidence: selected.evidence,
        },
    }
}

pub(crate) fn build_reasoning_graph_artifact(
    run_summary: &RunSummaryArtifact,
    affect_state: &AffectStateArtifact,
    aee_decision: &AeeDecisionArtifact,
    scores: Option<&ScoresArtifact>,
) -> ReasoningGraphArtifact {
    let retry_score = if affect_state.affect.recovery_bias >= 2 {
        92
    } else if affect_state.affect.recovery_bias == 1 {
        68
    } else {
        22
    };
    let maintain_score = if affect_state.affect.recovery_bias == 0 {
        88
    } else {
        36
    };
    let failure_signal_score = if aee_decision.decision.evidence.failure_count > 0 {
        80
    } else {
        24
    };

    let mut action_nodes = [
        ReasoningGraphNode {
            node_id: "action.retry_budget".to_string(),
            node_kind: "action".to_string(),
            label: "increase retry budget".to_string(),
            rank: 0,
            priority_score: retry_score,
            affect_mode: Some(affect_state.affect.affect_mode.clone()),
            rationale: format!(
                "Affect-guided recovery bias {} favors bounded retry recovery.",
                affect_state.affect.recovery_bias
            ),
        },
        ReasoningGraphNode {
            node_id: "action.maintain_policy".to_string(),
            node_kind: "action".to_string(),
            label: "maintain current policy".to_string(),
            rank: 0,
            priority_score: maintain_score,
            affect_mode: Some(affect_state.affect.affect_mode.clone()),
            rationale: "Steady-state or low-bias runs preserve the current bounded runtime policy."
                .to_string(),
        },
    ];
    action_nodes.sort_by(|a, b| {
        b.priority_score
            .cmp(&a.priority_score)
            .then_with(|| a.node_id.cmp(&b.node_id))
    });
    for (idx, node) in action_nodes.iter_mut().enumerate() {
        node.rank = (idx + 1) as u32;
    }

    let selected_node = action_nodes
        .iter()
        .find(|node| {
            (aee_decision.decision.intent == "increase_step_retry_budget"
                && node.node_id == "action.retry_budget")
                || (aee_decision.decision.intent != "increase_step_retry_budget"
                    && node.node_id == "action.maintain_policy")
        })
        .cloned()
        .unwrap_or_else(|| action_nodes[0].clone());

    let nodes = vec![
        ReasoningGraphNode {
            node_id: "evidence.runtime".to_string(),
            node_kind: "evidence".to_string(),
            label: "bounded runtime evidence".to_string(),
            rank: 1,
            priority_score: failure_signal_score,
            affect_mode: None,
            rationale: format!(
                "failure_count={} retry_count={} success_ratio={}",
                aee_decision.decision.evidence.failure_count,
                aee_decision.decision.evidence.retry_count,
                aee_decision.decision.evidence.success_ratio
            ),
        },
        ReasoningGraphNode {
            node_id: "affect.current".to_string(),
            node_kind: "affect".to_string(),
            label: affect_state.affect.affect_mode.replace('_', " "),
            rank: 1,
            priority_score: 100,
            affect_mode: Some(affect_state.affect.affect_mode.clone()),
            rationale: affect_state.affect.downstream_priority.clone(),
        },
        action_nodes[0].clone(),
        action_nodes[1].clone(),
    ];

    let edges = vec![
        ReasoningGraphEdge {
            edge_id: "edge-001".to_string(),
            from: "evidence.runtime".to_string(),
            to: "affect.current".to_string(),
            relation: "updates".to_string(),
            rationale: affect_state.affect.update_reason.clone(),
        },
        ReasoningGraphEdge {
            edge_id: "edge-002".to_string(),
            from: "affect.current".to_string(),
            to: "action.retry_budget".to_string(),
            relation: "prioritizes".to_string(),
            rationale: "High recovery bias increases retry-budget priority.".to_string(),
        },
        ReasoningGraphEdge {
            edge_id: "edge-003".to_string(),
            from: "affect.current".to_string(),
            to: "action.maintain_policy".to_string(),
            relation: "prioritizes".to_string(),
            rationale: "Low recovery bias preserves the maintain-current-policy path.".to_string(),
        },
    ];

    ReasoningGraphArtifact {
        reasoning_graph_version: REASONING_GRAPH_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            suggestions_version: affect_state.generated_from.suggestions_version,
            scores_version: scores.map(|value| value.scores_version),
        },
        graph: ReasoningGraphRecord {
            graph_id: "reasoning-graph-001".to_string(),
            dominant_affect_mode: affect_state.affect.affect_mode.clone(),
            ranking_rule:
                "sort action nodes by descending priority_score, then lexicographic node_id"
                    .to_string(),
            selected_path: ReasoningGraphSelection {
                selected_node_id: selected_node.node_id,
                selected_intent: aee_decision.decision.intent.clone(),
                selected_target: aee_decision.decision.target.clone(),
                graph_derived_output: aee_decision.decision.expected_downstream_effect.clone(),
                affect_changed_ranking: affect_state.affect.recovery_bias > 0,
            },
            nodes,
            edges,
        },
    }
}
