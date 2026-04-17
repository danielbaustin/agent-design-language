use super::*;

#[cfg(test)]
pub(crate) fn build_agency_selection_state(
    signals: &CognitiveSignalsArtifact,
    arbitration: &CognitiveArbitrationArtifact,
    fast_slow_state: &FastSlowPathState,
    fast_slow_path: &FastSlowPathArtifact,
) -> AgencySelectionState {
    let dominant_instinct = match signals.instinct.dominant_instinct.as_str() {
        "integrity" => execute::DominantInstinct::Integrity,
        "completion" => execute::DominantInstinct::Completion,
        "curiosity" => execute::DominantInstinct::Curiosity,
        "coherence" => execute::DominantInstinct::Coherence,
        other => panic!("unexpected dominant_instinct value: {other}"),
    };
    let selected_path = match fast_slow_path.selected_path.as_str() {
        "fast_path" => execute::SelectedPath::FastPath,
        "slow_path" => execute::SelectedPath::SlowPath,
        other => panic!("unexpected selected_path value: {other}"),
    };
    let (
        selection_mode,
        candidate_set,
        selected_candidate_id,
        selected_candidate_kind,
        selected_candidate_action,
        selected_candidate_reason,
    ) = match selected_path {
        execute::SelectedPath::FastPath => {
            let candidate_set = vec![
                AgencyCandidateRecord {
                    candidate_id: "cand-fast-execute".to_string(),
                    candidate_kind: "direct_execution".to_string(),
                    bounded_action:
                        "execute selected candidate directly under bounded once semantics"
                            .to_string(),
                    review_requirement: "minimal".to_string(),
                    execution_priority: 1,
                    rationale: format!(
                        "route={} dominant_instinct={} confidence={}",
                        arbitration.route_selected, dominant_instinct, arbitration.confidence
                    ),
                },
                AgencyCandidateRecord {
                    candidate_id: "cand-fast-verify".to_string(),
                    candidate_kind: "bounded_verification".to_string(),
                    bounded_action: "perform one bounded verification pass before execution"
                        .to_string(),
                    review_requirement: "light".to_string(),
                    execution_priority: 2,
                    rationale: "keep a bounded verification candidate available when instinct pressure favors uncertainty reduction or extra constraint checks".to_string(),
                },
            ];
            let decision = execute::select_instinct_runtime_candidate(
                selected_path,
                dominant_instinct,
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
        execute::SelectedPath::SlowPath => {
            let candidate_set = vec![
                AgencyCandidateRecord {
                    candidate_id: "cand-slow-review".to_string(),
                    candidate_kind: "review_and_refine".to_string(),
                    bounded_action:
                        "review, refine, or veto the current candidate before execution"
                            .to_string(),
                    review_requirement: "verification_required".to_string(),
                    execution_priority: 1,
                    rationale: format!(
                        "route={} dominant_instinct={} risk_class={}",
                        arbitration.route_selected, dominant_instinct, arbitration.risk_class
                    ),
                },
                AgencyCandidateRecord {
                    candidate_id: "cand-slow-direct".to_string(),
                    candidate_kind: "direct_execution".to_string(),
                    bounded_action:
                        "execute the current candidate without additional refinement".to_string(),
                    review_requirement: "minimal".to_string(),
                    execution_priority: 2,
                    rationale: "retain the direct-execution alternative when completion pressure can still justify a bounded finish-first move".to_string(),
                },
                AgencyCandidateRecord {
                    candidate_id: "cand-slow-defer".to_string(),
                    candidate_kind: "bounded_deferral".to_string(),
                    bounded_action:
                        "defer execution and surface the candidate set for later gate/review stages"
                            .to_string(),
                    review_requirement: "review_required".to_string(),
                    execution_priority: 3,
                    rationale: "preserve a bounded non-execution option when curiosity keeps uncertainty high or the system should pause before commitment".to_string(),
                },
            ];
            let decision = execute::select_instinct_runtime_candidate(
                selected_path,
                dominant_instinct,
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
            selected_path,
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
