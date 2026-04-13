use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::artifacts;
use crate::freedom_gate;
use crate::obsmem_indexing::index_run_from_artifacts;
use crate::trace;

use super::StepExecutionRecord;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeControlState {
    pub signals: CognitiveSignalsState,
    pub arbitration: CognitiveArbitrationState,
    pub fast_slow: FastSlowPathState,
    pub agency: AgencySelectionState,
    pub bounded_execution: BoundedExecutionState,
    pub evaluation: EvaluationControlState,
    pub reframing: ReframingControlState,
    pub freedom_gate: FreedomGateState,
    pub memory: MemoryParticipationState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitiveSignalsState {
    pub dominant_instinct: String,
    pub completion_pressure: String,
    pub integrity_bias: String,
    pub curiosity_bias: String,
    pub candidate_selection_bias: String,
    pub urgency_level: String,
    pub salience_level: String,
    pub persistence_pressure: String,
    pub confidence_shift: String,
    pub downstream_influence: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitiveArbitrationState {
    pub route_selected: String,
    pub reasoning_mode: String,
    pub confidence: String,
    pub risk_class: String,
    pub applied_constraints: Vec<String>,
    pub cost_latency_assumption: String,
    pub route_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FastSlowPathState {
    pub selected_path: String,
    pub path_family: String,
    pub runtime_branch_taken: String,
    pub handoff_state: String,
    pub candidate_strategy: String,
    pub review_depth: String,
    pub execution_profile: String,
    pub termination_expectation: String,
    pub path_difference_summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgencySelectionState {
    pub candidate_generation_basis: String,
    pub selection_mode: String,
    pub candidate_set: Vec<AgencyCandidateRecord>,
    pub selected_candidate_id: String,
    pub selected_candidate_kind: String,
    pub selected_candidate_action: String,
    pub selected_candidate_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgencyCandidateRecord {
    pub candidate_id: String,
    pub candidate_kind: String,
    pub bounded_action: String,
    pub review_requirement: String,
    pub execution_priority: u32,
    pub rationale: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgencySelectionDecisionTemplate {
    pub selection_mode: &'static str,
    pub candidate_id: &'static str,
    pub candidate_kind: &'static str,
    pub candidate_action: &'static str,
    pub candidate_reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundedExecutionState {
    pub execution_status: String,
    pub continuation_state: String,
    pub provisional_termination_state: String,
    pub iterations: Vec<BoundedExecutionIteration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundedExecutionIteration {
    pub iteration_index: u32,
    pub stage: String,
    pub action: String,
    pub outcome: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvaluationControlState {
    pub progress_signal: String,
    pub contradiction_signal: String,
    pub failure_signal: String,
    pub termination_reason: String,
    pub behavior_effect: String,
    pub next_control_action: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReframingControlState {
    pub frame_adequacy_score: u32,
    pub reframing_trigger: String,
    pub reframing_reason: String,
    pub prior_frame: String,
    pub new_frame: String,
    pub reexecution_choice: String,
    pub post_reframe_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreedomGateState {
    pub input: FreedomGateInputState,
    pub gate_decision: String,
    pub reason_code: String,
    pub decision_reason: String,
    pub selected_action_or_none: Option<String>,
    pub commitment_blocked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreedomGateInputState {
    pub candidate_id: String,
    pub candidate_action: String,
    pub candidate_rationale: String,
    pub risk_class: String,
    pub policy_context: FreedomGatePolicyContextState,
    pub evaluation_signals: FreedomGateEvaluationSignalsState,
    pub frame_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreedomGatePolicyContextState {
    pub route_selected: String,
    pub selected_candidate_kind: String,
    pub requires_review: bool,
    pub policy_blocked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreedomGateEvaluationSignalsState {
    pub progress_signal: String,
    pub contradiction_signal: String,
    pub failure_signal: String,
    pub termination_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryParticipationState {
    pub read: MemoryReadState,
    pub write: MemoryWriteState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryReadState {
    pub query: MemoryQueryState,
    pub entries: Vec<MemoryReadEntry>,
    pub retrieval_order: String,
    pub influence_summary: String,
    pub influenced_stage: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryQueryState {
    pub workflow_id: String,
    pub status_filter: String,
    pub limit: u32,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryReadEntry {
    pub memory_entry_id: String,
    pub run_id: String,
    pub workflow_id: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryWriteState {
    pub entry_id: String,
    pub content: String,
    pub tags: Vec<String>,
    pub logical_timestamp: String,
    pub write_reason: String,
    pub source: String,
}

#[derive(Debug, Clone, Copy)]
struct RuntimeSignalEvidence {
    failure_count: usize,
    retry_count: usize,
    delegation_denied_count: usize,
    security_denied_count: usize,
    success_ratio_permille: usize,
    scheduler_max_parallel_observed: usize,
}

pub fn derive_runtime_control_state(
    overall_status: &str,
    records: &[StepExecutionRecord],
    tr: &trace::Trace,
) -> RuntimeControlState {
    let evidence = collect_runtime_signal_evidence(overall_status, records, tr);
    let signals = derive_cognitive_signals_state(overall_status, evidence);
    let arbitration = derive_cognitive_arbitration_state(overall_status, evidence, &signals);
    let fast_slow = derive_fast_slow_path_state(&arbitration);
    let agency = derive_agency_selection_state(&signals, &arbitration, &fast_slow);
    let bounded_execution = derive_bounded_execution_state(overall_status, &agency);
    let evaluation = derive_evaluation_control_state(overall_status, &bounded_execution);
    let reframing =
        derive_reframing_control_state(&fast_slow, &agency, &bounded_execution, &evaluation);
    let freedom_gate = derive_freedom_gate_state(&arbitration, &agency, &evaluation, &reframing);
    let memory = derive_memory_participation_state(
        &tr.run_id,
        &tr.workflow_id,
        overall_status,
        &arbitration,
        &agency,
        &evaluation,
    );

    RuntimeControlState {
        signals,
        arbitration,
        fast_slow,
        agency,
        bounded_execution,
        evaluation,
        reframing,
        freedom_gate,
        memory,
    }
}

fn collect_runtime_signal_evidence(
    overall_status: &str,
    records: &[StepExecutionRecord],
    tr: &trace::Trace,
) -> RuntimeSignalEvidence {
    let total_steps = records.len();
    let failure_count = records
        .iter()
        .filter(|record| record.status != "success")
        .count();
    let success_count = total_steps.saturating_sub(failure_count);
    let retry_count: usize = records
        .iter()
        .map(|record| record.attempts.saturating_sub(1) as usize)
        .sum();
    let delegation_denied_count = tr
        .events
        .iter()
        .filter(|event| matches!(event, trace::TraceEvent::DelegationDenied { .. }))
        .count();
    let security_denied_count = delegation_denied_count;
    let success_ratio_permille = if total_steps == 0 {
        if overall_status == "success" {
            1000
        } else {
            0
        }
    } else {
        (success_count * 1000) / total_steps
    };
    let scheduler_max_parallel_observed = compute_max_parallel_observed_from_trace(tr);

    RuntimeSignalEvidence {
        failure_count,
        retry_count,
        delegation_denied_count,
        security_denied_count,
        success_ratio_permille,
        scheduler_max_parallel_observed,
    }
}

fn compute_max_parallel_observed_from_trace(tr: &trace::Trace) -> usize {
    let mut active: HashSet<&str> = HashSet::new();
    let mut max_parallel = 0usize;
    for event in &tr.events {
        match event {
            trace::TraceEvent::StepStarted { step_id, .. } => {
                active.insert(step_id.as_str());
                max_parallel = max_parallel.max(active.len());
            }
            trace::TraceEvent::StepFinished { step_id, .. } => {
                active.remove(step_id.as_str());
            }
            _ => {}
        }
    }
    max_parallel.max(1)
}

fn derive_cognitive_signals_state(
    overall_status: &str,
    evidence: RuntimeSignalEvidence,
) -> CognitiveSignalsState {
    let completion_pressure = if evidence.failure_count > 0 || overall_status == "failure" {
        "elevated"
    } else if evidence.retry_count > 0 || overall_status == "paused" {
        "guarded"
    } else {
        "steady"
    };
    let integrity_bias = if evidence.security_denied_count > 0 {
        "high"
    } else {
        "bounded"
    };
    let curiosity_bias = if evidence.success_ratio_permille < 1000 && evidence.failure_count == 0 {
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
    let salience_level = if evidence.failure_count > 0 || evidence.delegation_denied_count > 0 {
        "high"
    } else if evidence.retry_count > 0 {
        "moderate"
    } else {
        "low"
    };
    let persistence_pressure = if evidence.failure_count > 0 {
        "retry_biased"
    } else if evidence.retry_count > 0 {
        "stabilize_then_retry"
    } else {
        "bounded_once"
    };
    let confidence_shift = if evidence.failure_count > 0 || evidence.delegation_denied_count > 0 {
        "reduced"
    } else {
        "stable"
    };

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
        downstream_influence: format!(
            "dominant_instinct={} failure_count={} retry_count={} delegation_denied_count={} max_parallel={}",
            dominant_instinct,
            evidence.failure_count,
            evidence.retry_count,
            evidence.delegation_denied_count,
            evidence.scheduler_max_parallel_observed
        ),
    }
}

fn derive_cognitive_arbitration_state(
    overall_status: &str,
    evidence: RuntimeSignalEvidence,
    signals: &CognitiveSignalsState,
) -> CognitiveArbitrationState {
    let (route_selected, reasoning_mode) = if evidence.security_denied_count > 0
        || evidence.failure_count > 0
        || signals.dominant_instinct == "integrity"
    {
        ("slow", "review_heavy")
    } else if evidence.retry_count > 0
        || overall_status == "paused"
        || signals.confidence_shift == "reduced"
    {
        ("hybrid", "bounded_recovery")
    } else {
        ("fast", "direct_execution")
    };
    let risk_class = if evidence.security_denied_count > 0 {
        "high"
    } else if evidence.failure_count > 0 || evidence.retry_count > 0 {
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
    if evidence.security_denied_count > 0 {
        applied_constraints.push("security_denial_present".to_string());
    }
    if evidence.failure_count > 0 {
        applied_constraints.push("failure_recovery_bias".to_string());
    }
    if evidence.retry_count > 0 {
        applied_constraints.push("retry_budget_pressure".to_string());
    }
    if overall_status == "paused" {
        applied_constraints.push("pause_boundary_present".to_string());
    }
    if applied_constraints.is_empty() {
        applied_constraints.push("bounded_default_path".to_string());
    }
    let cost_latency_assumption = match route_selected {
        "fast" => "prefer lower-cost low-latency execution when bounded evidence is stable",
        "hybrid" => "allow bounded extra review when retry or pause pressure is present",
        _ => "spend bounded additional cognition when failure or policy risk is present",
    };

    CognitiveArbitrationState {
        route_selected: route_selected.to_string(),
        reasoning_mode: reasoning_mode.to_string(),
        confidence: confidence.to_string(),
        risk_class: risk_class.to_string(),
        applied_constraints,
        cost_latency_assumption: cost_latency_assumption.to_string(),
        route_reason: format!(
            "route={} dominant_instinct={} overall_status={} failure_count={} retry_count={} delegation_denied_count={}",
            route_selected,
            signals.dominant_instinct,
            overall_status,
            evidence.failure_count,
            evidence.retry_count,
            evidence.delegation_denied_count
        ),
    }
}

fn derive_fast_slow_path_state(arbitration: &CognitiveArbitrationState) -> FastSlowPathState {
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

fn derive_agency_selection_state(
    signals: &CognitiveSignalsState,
    arbitration: &CognitiveArbitrationState,
    fast_slow: &FastSlowPathState,
) -> AgencySelectionState {
    let (
        selection_mode,
        candidate_set,
        selected_candidate_id,
        selected_candidate_kind,
        selected_candidate_action,
        selected_candidate_reason,
    ) = match fast_slow.selected_path.as_str() {
        "fast_path" => {
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
                        arbitration.route_selected, signals.dominant_instinct, arbitration.confidence
                    ),
                },
                AgencyCandidateRecord {
                    candidate_id: "cand-fast-verify".to_string(),
                    candidate_kind: "bounded_verification".to_string(),
                    bounded_action:
                        "perform one bounded verification pass before execution".to_string(),
                    review_requirement: "light".to_string(),
                    execution_priority: 2,
                    rationale:
                        "keep a bounded verification candidate available when instinct pressure favors uncertainty reduction or extra constraint checks"
                            .to_string(),
                },
            ];
            let decision = select_instinct_runtime_candidate(
                fast_slow.selected_path.as_str(),
                signals.dominant_instinct.as_str(),
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
                    bounded_action:
                        "review, refine, or veto the current candidate before execution"
                            .to_string(),
                    review_requirement: "verification_required".to_string(),
                    execution_priority: 1,
                    rationale: format!(
                        "route={} dominant_instinct={} risk_class={}",
                        arbitration.route_selected, signals.dominant_instinct, arbitration.risk_class
                    ),
                },
                AgencyCandidateRecord {
                    candidate_id: "cand-slow-direct".to_string(),
                    candidate_kind: "direct_execution".to_string(),
                    bounded_action:
                        "execute the current candidate without additional refinement".to_string(),
                    review_requirement: "minimal".to_string(),
                    execution_priority: 2,
                    rationale: "retain the direct-execution alternative when completion pressure can still justify a bounded finish-first move"
                        .to_string(),
                },
                AgencyCandidateRecord {
                    candidate_id: "cand-slow-defer".to_string(),
                    candidate_kind: "bounded_deferral".to_string(),
                    bounded_action:
                        "defer execution and surface the candidate set for later gate/review stages"
                            .to_string(),
                    review_requirement: "review_required".to_string(),
                    execution_priority: 3,
                    rationale: "preserve a bounded non-execution option when curiosity keeps uncertainty high or the system should pause before commitment"
                        .to_string(),
                },
            ];
            let decision = select_instinct_runtime_candidate(
                fast_slow.selected_path.as_str(),
                signals.dominant_instinct.as_str(),
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
            fast_slow.selected_path,
            fast_slow.runtime_branch_taken,
            arbitration.route_selected,
            signals.candidate_selection_bias
        ),
        selection_mode: selection_mode.to_string(),
        candidate_set,
        selected_candidate_id,
        selected_candidate_kind,
        selected_candidate_action,
        selected_candidate_reason,
    }
}

pub fn select_instinct_runtime_candidate(
    selected_path: &str,
    dominant_instinct: &str,
    risk_class: &str,
) -> AgencySelectionDecisionTemplate {
    match selected_path {
        "fast_path" => match dominant_instinct {
            "curiosity" | "integrity" => AgencySelectionDecisionTemplate {
                selection_mode: "fast_candidate_verification",
                candidate_id: "cand-fast-verify",
                candidate_kind: "bounded_verification",
                candidate_action: "perform one bounded verification pass before execution",
                candidate_reason:
                    "fast path stays bounded, but curiosity or integrity pressure upgrades the selected candidate to a single verification pass before execution",
            },
            _ => AgencySelectionDecisionTemplate {
                selection_mode: "fast_candidate_commitment",
                candidate_id: "cand-fast-execute",
                candidate_kind: "direct_execution",
                candidate_action: "execute selected candidate directly under bounded once semantics",
                candidate_reason:
                    "fast path prioritizes direct bounded execution when instinct pressure does not require extra verification",
            },
        },
        _ => {
            if risk_class == "high" || matches!(dominant_instinct, "integrity" | "coherence") {
                AgencySelectionDecisionTemplate {
                    selection_mode: "slow_candidate_review",
                    candidate_id: "cand-slow-review",
                    candidate_kind: "review_and_refine",
                    candidate_action:
                        "review, refine, or veto the current candidate before execution",
                    candidate_reason:
                        "slow path keeps review/refinement selected when risk stays high or instinct pressure favors constraint and coherence preservation",
                }
            } else if dominant_instinct == "curiosity" {
                AgencySelectionDecisionTemplate {
                    selection_mode: "slow_candidate_uncertainty_hold",
                    candidate_id: "cand-slow-defer",
                    candidate_kind: "bounded_deferral",
                    candidate_action:
                        "defer execution and surface the candidate set for later gate/review stages",
                    candidate_reason:
                        "slow path preserves a bounded defer option when curiosity keeps uncertainty reduction more important than immediate execution",
                }
            } else {
                AgencySelectionDecisionTemplate {
                    selection_mode: "slow_candidate_review",
                    candidate_id: "cand-slow-review",
                    candidate_kind: "review_and_refine",
                    candidate_action:
                        "review, refine, or veto the current candidate before execution",
                    candidate_reason:
                        "slow path keeps review/refinement selected unless curiosity introduces a bounded uncertainty hold",
                }
            }
        }
    }
}

fn derive_bounded_execution_state(
    overall_status: &str,
    agency: &AgencySelectionState,
) -> BoundedExecutionState {
    let (execution_status, continuation_state, provisional_termination_state, iterations) =
        match agency.selected_candidate_kind.as_str() {
            "direct_execution" => (
                "completed",
                "stop_after_one",
                "ready_for_evaluation",
                vec![BoundedExecutionIteration {
                    iteration_index: 1,
                    stage: "execute".to_string(),
                    action: agency.selected_candidate_action.clone(),
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
                        action: agency.selected_candidate_action.clone(),
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
                    action: agency.selected_candidate_action.clone(),
                    outcome: "bounded_deferral_recorded".to_string(),
                }],
            ),
        };

    let execution_status = if overall_status == "failure" {
        "completed_with_failure_signal"
    } else if overall_status == "paused" {
        "paused_for_review"
    } else {
        execution_status
    };
    let continuation_state = if overall_status == "failure" && iterations.len() > 1 {
        "bounded_review_complete_with_failure_signal"
    } else if overall_status == "paused" {
        "paused_before_termination"
    } else {
        continuation_state
    };

    BoundedExecutionState {
        execution_status: execution_status.to_string(),
        continuation_state: continuation_state.to_string(),
        provisional_termination_state: provisional_termination_state.to_string(),
        iterations,
    }
}

fn derive_evaluation_control_state(
    overall_status: &str,
    bounded_execution: &BoundedExecutionState,
) -> EvaluationControlState {
    let (
        progress_signal,
        contradiction_signal,
        failure_signal,
        termination_reason,
        behavior_effect,
        next_control_action,
    ) = if overall_status == "failure" {
        (
            "stalled_progress",
            "present",
            "bounded_failure_detected",
            if bounded_execution.iterations.len() > 1 {
                "bounded_failure"
            } else {
                "no_progress"
            },
            "emit bounded failure/termination signals for later reframing or policy handling",
            if bounded_execution.iterations.len() > 1 {
                "handoff_to_reframing"
            } else {
                "terminate_with_failure"
            },
        )
    } else if overall_status == "paused" {
        (
            "guarded_progress",
            "none",
            "none",
            "pause_boundary",
            "preserve bounded state for resume or explicit review handling",
            "await_resume",
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

fn derive_reframing_control_state(
    fast_slow: &FastSlowPathState,
    agency: &AgencySelectionState,
    bounded_execution: &BoundedExecutionState,
    evaluation: &EvaluationControlState,
) -> ReframingControlState {
    let prior_frame = match fast_slow.selected_path.as_str() {
        "fast_path" => "direct_execution_under_current_frame",
        _ => "review_and_refine_under_current_frame",
    };

    let (
        frame_adequacy_score,
        reframing_trigger,
        reframing_reason,
        new_frame,
        reexecution_choice,
        post_reframe_state,
    ) = match evaluation.next_control_action.as_str() {
        "handoff_to_reframing" => {
            let reason = if evaluation.contradiction_signal == "present" {
                "contradiction_detected_after_bounded_execution"
            } else if evaluation.failure_signal != "none" {
                "bounded_failure_after_execution"
            } else {
                "frame_inadequate_for_bounded_progress"
            };
            let score = if bounded_execution.iterations.len() > 1 {
                28
            } else {
                40
            };
            (
                score,
                "triggered",
                reason,
                "diagnose_and_restructure_before_retry",
                if agency.selected_candidate_kind == "review_and_refine" {
                    "bounded_reframe_and_retry"
                } else {
                    "bounded_reframe_then_review"
                },
                "ready_for_reframed_execution",
            )
        }
        "terminate_with_failure" => (
            36,
            "not_triggered",
            "failure_detected_but_retry_budget_not_justified",
            "retain_current_frame",
            "terminate_without_reframe",
            "terminate_with_failure",
        ),
        "await_resume" => (
            58,
            "not_triggered",
            "pause_boundary_preserves_current_frame_until_resume",
            "retain_current_frame",
            "await_resume",
            "await_resume",
        ),
        _ => (
            88,
            "not_triggered",
            "current_frame_adequate_for_bounded_progress",
            "retain_current_frame",
            "no_reframe_required",
            "complete_run",
        ),
    };

    ReframingControlState {
        frame_adequacy_score,
        reframing_trigger: reframing_trigger.to_string(),
        reframing_reason: reframing_reason.to_string(),
        prior_frame: prior_frame.to_string(),
        new_frame: new_frame.to_string(),
        reexecution_choice: reexecution_choice.to_string(),
        post_reframe_state: post_reframe_state.to_string(),
    }
}

fn derive_freedom_gate_state(
    arbitration: &CognitiveArbitrationState,
    agency: &AgencySelectionState,
    evaluation: &EvaluationControlState,
    reframing: &ReframingControlState,
) -> FreedomGateState {
    let input = freedom_gate::FreedomGateInput {
        candidate_id: agency.selected_candidate_id.clone(),
        candidate_action: agency.selected_candidate_action.clone(),
        candidate_rationale: agency.selected_candidate_reason.clone(),
        risk_class: arbitration.risk_class.clone(),
        policy_context: freedom_gate::FreedomGatePolicyContext {
            route_selected: arbitration.route_selected.clone(),
            selected_candidate_kind: agency.selected_candidate_kind.clone(),
            requires_review: agency.selected_candidate_kind == "bounded_deferral"
                || evaluation.next_control_action == "await_resume",
            policy_blocked: false,
        },
        evaluation_signals: freedom_gate::FreedomGateEvaluationSignals {
            progress_signal: evaluation.progress_signal.clone(),
            contradiction_signal: evaluation.contradiction_signal.clone(),
            failure_signal: evaluation.failure_signal.clone(),
            termination_reason: evaluation.termination_reason.clone(),
        },
        frame_state: reframing.post_reframe_state.clone(),
    };
    let decision = freedom_gate::evaluate_freedom_gate(&input);

    FreedomGateState {
        input: FreedomGateInputState {
            candidate_id: input.candidate_id,
            candidate_action: input.candidate_action,
            candidate_rationale: input.candidate_rationale,
            risk_class: input.risk_class,
            policy_context: FreedomGatePolicyContextState {
                route_selected: input.policy_context.route_selected,
                selected_candidate_kind: input.policy_context.selected_candidate_kind,
                requires_review: input.policy_context.requires_review,
                policy_blocked: input.policy_context.policy_blocked,
            },
            evaluation_signals: FreedomGateEvaluationSignalsState {
                progress_signal: input.evaluation_signals.progress_signal,
                contradiction_signal: input.evaluation_signals.contradiction_signal,
                failure_signal: input.evaluation_signals.failure_signal,
                termination_reason: input.evaluation_signals.termination_reason,
            },
            frame_state: input.frame_state,
        },
        gate_decision: decision.gate_decision,
        reason_code: decision.reason_code,
        decision_reason: decision.decision_reason,
        selected_action_or_none: decision.selected_action_or_none,
        commitment_blocked: decision.commitment_blocked,
    }
}

fn derive_memory_participation_state(
    run_id: &str,
    workflow_id: &str,
    overall_status: &str,
    arbitration: &CognitiveArbitrationState,
    agency: &AgencySelectionState,
    evaluation: &EvaluationControlState,
) -> MemoryParticipationState {
    let status_filter = if evaluation.next_control_action == "handoff_to_reframing" {
        "failed"
    } else {
        "succeeded"
    };
    let limit = 3u32;
    let entries = load_memory_read_entries(run_id, workflow_id, status_filter, limit as usize);
    let influence_summary = if entries.is_empty() {
        "no_prior_memory_hits_available_for_this_workflow".to_string()
    } else if evaluation.next_control_action == "handoff_to_reframing" {
        format!(
            "prior_failure_memory reinforces bounded reframing for route={} selected_candidate={}",
            arbitration.route_selected, agency.selected_candidate_id
        )
    } else {
        format!(
            "prior_success_memory reinforces retained bounded frame for route={} selected_candidate={}",
            arbitration.route_selected, agency.selected_candidate_id
        )
    };
    let influenced_stage = if evaluation.next_control_action == "handoff_to_reframing" {
        "reframing_decision".to_string()
    } else {
        "evaluation_termination".to_string()
    };

    let read = MemoryReadState {
        query: MemoryQueryState {
            workflow_id: workflow_id.to_string(),
            status_filter: status_filter.to_string(),
            limit,
            source: "repo_local_runs_root".to_string(),
        },
        entries,
        retrieval_order: "workflow_id_then_run_id_ascending".to_string(),
        influence_summary,
        influenced_stage,
    };

    let write_reason = if evaluation.next_control_action == "handoff_to_reframing" {
        "record_failure_for_future_reframing_context"
    } else if overall_status == "success" {
        "record_success_for_future_bounded_context"
    } else {
        "record_observed_outcome_for_future_context"
    };
    let logical_timestamp = format!("run:{run_id}");
    let mut tags = vec![
        format!("workflow:{workflow_id}"),
        format!("status:{overall_status}"),
        format!("route:{}", arbitration.route_selected),
        format!("candidate:{}", agency.selected_candidate_kind),
        format!("action:{}", evaluation.next_control_action),
    ];
    tags.sort();
    tags.dedup();
    let write = MemoryWriteState {
        entry_id: format!("mem-entry::{workflow_id}::{run_id}"),
        content: format!(
            "workflow={workflow_id} status={overall_status} next_control_action={} influence={}",
            evaluation.next_control_action, read.influence_summary
        ),
        tags,
        logical_timestamp,
        write_reason: write_reason.to_string(),
        source: "runtime_control_projection".to_string(),
    };

    MemoryParticipationState { read, write }
}

fn load_memory_read_entries(
    current_run_id: &str,
    workflow_id: &str,
    status_filter: &str,
    limit: usize,
) -> Vec<MemoryReadEntry> {
    let Ok(runs_root) = artifacts::runs_root() else {
        return Vec::new();
    };
    let Ok(read_dir) = std::fs::read_dir(&runs_root) else {
        return Vec::new();
    };

    let mut entries = Vec::new();
    for entry in read_dir.flatten() {
        let file_type_ok = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        if !file_type_ok {
            continue;
        }
        let candidate_run_id = entry.file_name().to_string_lossy().to_string();
        if candidate_run_id == current_run_id {
            continue;
        }
        let Ok(indexed) = index_run_from_artifacts(&runs_root, &candidate_run_id) else {
            continue;
        };
        if indexed.workflow_id != workflow_id {
            continue;
        }
        let status_matches = match status_filter {
            "failed" => indexed.status == "failed",
            "succeeded" => indexed.status == "succeeded",
            other => indexed.status == other,
        };
        if !status_matches {
            continue;
        }
        entries.push(MemoryReadEntry {
            memory_entry_id: format!("{}::{}", indexed.run_id, indexed.workflow_id),
            run_id: indexed.run_id,
            workflow_id: indexed.workflow_id,
            summary: indexed.summary,
            tags: indexed.tags,
            source: "indexed_run_artifacts".to_string(),
        });
    }

    entries.sort_by(|a, b| {
        a.workflow_id
            .cmp(&b.workflow_id)
            .then_with(|| a.run_id.cmp(&b.run_id))
            .then_with(|| a.memory_entry_id.cmp(&b.memory_entry_id))
    });
    entries.truncate(limit);
    entries
}

#[cfg(test)]
mod tests {
    use super::select_instinct_runtime_candidate;

    #[test]
    fn select_instinct_runtime_candidate_changes_fast_path_for_curiosity() {
        let decision = select_instinct_runtime_candidate("fast_path", "curiosity", "low");

        assert_eq!(decision.candidate_id, "cand-fast-verify");
        assert_eq!(decision.candidate_kind, "bounded_verification");
    }

    #[test]
    fn select_instinct_runtime_candidate_keeps_review_for_high_risk_slow_path() {
        let decision = select_instinct_runtime_candidate("slow_path", "completion", "high");

        assert_eq!(decision.candidate_id, "cand-slow-review");
        assert_eq!(decision.candidate_kind, "review_and_refine");
    }

    #[test]
    fn select_instinct_runtime_candidate_allows_curiosity_biased_slow_defer() {
        let decision = select_instinct_runtime_candidate("slow_path", "curiosity", "medium");

        assert_eq!(decision.candidate_id, "cand-slow-defer");
        assert_eq!(decision.candidate_kind, "bounded_deferral");
    }
}
