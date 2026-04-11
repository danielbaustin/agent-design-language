use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::{execute, freedom_gate};

use super::{write_file, DEMO_G_V086_CONTROL_PATH};

fn custom_v086_control_path_runtime() -> execute::RuntimeControlState {
    execute::RuntimeControlState {
        signals: execute::CognitiveSignalsState {
            dominant_instinct: "integrity".to_string(),
            completion_pressure: "guarded".to_string(),
            integrity_bias: "high".to_string(),
            curiosity_bias: "bounded".to_string(),
            candidate_selection_bias: "prefer lower-risk constrained candidates".to_string(),
            urgency_level: "moderate".to_string(),
            salience_level: "high".to_string(),
            persistence_pressure: "stabilize_then_retry".to_string(),
            confidence_shift: "reduced".to_string(),
            downstream_influence: "integrated control path demo".to_string(),
        },
        arbitration: execute::CognitiveArbitrationState {
            route_selected: "slow".to_string(),
            reasoning_mode: "review_heavy".to_string(),
            confidence: "guarded".to_string(),
            risk_class: "high".to_string(),
            applied_constraints: vec![
                "security_denial_present".to_string(),
                "failure_recovery_bias".to_string(),
            ],
            cost_latency_assumption:
                "spend bounded additional cognition when failure or policy risk is present"
                    .to_string(),
            route_reason:
                "integrated path favors slow route when contradiction and policy risk are present"
                    .to_string(),
        },
        fast_slow: execute::FastSlowPathState {
            selected_path: "slow_path".to_string(),
            path_family: "slow".to_string(),
            runtime_branch_taken: "slow_review_refine_branch".to_string(),
            handoff_state: "review_handoff".to_string(),
            candidate_strategy: "validate, refine, or veto the current bounded candidate"
                .to_string(),
            review_depth: "verification_required".to_string(),
            execution_profile: "review_and_refine_before_execution".to_string(),
            termination_expectation: "terminate_after_bounded_review_cycle_or_policy_block"
                .to_string(),
            path_difference_summary:
                "slow path keeps the run review-first and routes contradiction into reframing"
                    .to_string(),
        },
        agency: execute::AgencySelectionState {
            candidate_generation_basis: "integrated-control-path-scenario".to_string(),
            selection_mode: "slow_candidate_comparison".to_string(),
            candidate_set: vec![execute::AgencyCandidateRecord {
                candidate_id: "cand-custom-review".to_string(),
                candidate_kind: "review_and_refine".to_string(),
                bounded_action: "review and refine the candidate".to_string(),
                review_requirement: "verification_required".to_string(),
                execution_priority: 1,
                rationale: "selected for bounded review-heavy remediation".to_string(),
            }],
            selected_candidate_id: "cand-custom-review".to_string(),
            selected_candidate_kind: "review_and_refine".to_string(),
            selected_candidate_action: "review and refine the candidate".to_string(),
            selected_candidate_reason: "candidate preserves bounded progress under contradiction"
                .to_string(),
        },
        bounded_execution: execute::BoundedExecutionState {
            execution_status: "completed".to_string(),
            continuation_state: "bounded_review_complete".to_string(),
            provisional_termination_state: "ready_for_evaluation".to_string(),
            iterations: vec![
                execute::BoundedExecutionIteration {
                    iteration_index: 1,
                    stage: "review".to_string(),
                    action: "review the candidate".to_string(),
                    outcome: "bounded_review_pass_complete".to_string(),
                },
                execute::BoundedExecutionIteration {
                    iteration_index: 2,
                    stage: "execute".to_string(),
                    action: "execute the reviewed candidate".to_string(),
                    outcome: "bounded_reviewed_execution_complete".to_string(),
                },
            ],
        },
        evaluation: execute::EvaluationControlState {
            progress_signal: "steady_progress".to_string(),
            contradiction_signal: "present".to_string(),
            failure_signal: "none".to_string(),
            termination_reason: "contradiction_detected".to_string(),
            behavior_effect: "surface contradiction for bounded follow-up".to_string(),
            next_control_action: "handoff_to_reframing".to_string(),
        },
        reframing: execute::ReframingControlState {
            frame_adequacy_score: 24,
            reframing_trigger: "triggered".to_string(),
            reframing_reason: "contradiction_detected_after_bounded_execution".to_string(),
            prior_frame: "review_and_refine_under_current_frame".to_string(),
            new_frame: "diagnose_and_restructure_before_retry".to_string(),
            reexecution_choice: "bounded_reframe_and_retry".to_string(),
            post_reframe_state: "ready_for_reframed_execution".to_string(),
        },
        freedom_gate: execute::FreedomGateState {
            input: execute::FreedomGateInputState {
                candidate_id: "cand-custom-review".to_string(),
                candidate_action: "review and refine the candidate".to_string(),
                candidate_rationale: "candidate preserves bounded progress under contradiction"
                    .to_string(),
                risk_class: "high".to_string(),
                policy_context: execute::FreedomGatePolicyContextState {
                    route_selected: "slow".to_string(),
                    selected_candidate_kind: "review_and_refine".to_string(),
                    requires_review: false,
                    policy_blocked: false,
                },
                evaluation_signals: execute::FreedomGateEvaluationSignalsState {
                    progress_signal: "steady_progress".to_string(),
                    contradiction_signal: "present".to_string(),
                    failure_signal: "none".to_string(),
                    termination_reason: "contradiction_detected".to_string(),
                },
                frame_state: "ready_for_reframed_execution".to_string(),
            },
            gate_decision: "defer".to_string(),
            reason_code: "frame_inadequate".to_string(),
            decision_reason:
                "frame state requires bounded reframing before commitment can be allowed"
                    .to_string(),
            selected_action_or_none: None,
            commitment_blocked: true,
        },
        memory: execute::MemoryParticipationState {
            read: execute::MemoryReadState {
                query: execute::MemoryQueryState {
                    workflow_id: "wf".to_string(),
                    status_filter: "failed".to_string(),
                    limit: 3,
                    source: "repo_local_runs_root".to_string(),
                },
                entries: vec![execute::MemoryReadEntry {
                    memory_entry_id: "prev-run::wf".to_string(),
                    run_id: "prev-run".to_string(),
                    workflow_id: "wf".to_string(),
                    summary: "prior failure memory".to_string(),
                    tags: vec!["status:failed".to_string(), "workflow:wf".to_string()],
                    source: "indexed_run_artifacts".to_string(),
                }],
                retrieval_order: "workflow_id_then_run_id_ascending".to_string(),
                influence_summary:
                    "prior_failure_memory reinforces bounded reframing for route=slow selected_candidate=cand-custom-review"
                        .to_string(),
                influenced_stage: "reframing_decision".to_string(),
            },
            write: execute::MemoryWriteState {
                entry_id: "mem-entry::wf::runtime-control".to_string(),
                content: "workflow=wf status=failure next_control_action=handoff_to_reframing influence=prior_failure_memory reinforces bounded reframing for route=slow selected_candidate=cand-custom-review".to_string(),
                tags: vec![
                    "action:handoff_to_reframing".to_string(),
                    "candidate:review_and_refine".to_string(),
                    "status:failure".to_string(),
                    "workflow:wf".to_string(),
                ],
                logical_timestamp: "run:runtime-control".to_string(),
                write_reason: "record_failure_for_future_reframing_context".to_string(),
                source: "runtime_control_projection".to_string(),
            },
        },
    }
}

pub fn write_v086_control_path_demo(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let runtime = custom_v086_control_path_runtime();
    let mut artifacts = Vec::new();
    let generated_from = serde_json::json!({
        "artifact_model_version": 1,
        "run_summary_version": 1,
        "suggestions_version": 1,
        "scores_version": 1
    });
    let signals = serde_json::json!({
        "cognitive_signals_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "generated_from": generated_from.clone(),
        "instinct": {
            "instinct_profile_id": "instinct-001",
            "dominant_instinct": runtime.signals.dominant_instinct,
            "completion_pressure": runtime.signals.completion_pressure,
            "integrity_bias": runtime.signals.integrity_bias,
            "curiosity_bias": runtime.signals.curiosity_bias,
            "candidate_selection_bias": runtime.signals.candidate_selection_bias,
            "deterministic_update_rule": "demo deterministic signal projection"
        },
        "affect": {
            "affect_state_id": "signal-affect-001",
            "urgency_level": runtime.signals.urgency_level,
            "salience_level": runtime.signals.salience_level,
            "persistence_pressure": runtime.signals.persistence_pressure,
            "confidence_shift": runtime.signals.confidence_shift,
            "downstream_influence": runtime.signals.downstream_influence,
            "deterministic_update_rule": "demo deterministic affect projection"
        }
    });
    let candidate_selection = serde_json::json!({
        "agency_selection_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "generated_from": generated_from.clone(),
        "candidate_generation_basis": runtime.agency.candidate_generation_basis,
        "selection_mode": runtime.agency.selection_mode,
        "candidate_set": runtime.agency.candidate_set,
        "selected_candidate_id": runtime.agency.selected_candidate_id,
        "selected_candidate_reason": runtime.agency.selected_candidate_reason,
        "deterministic_selection_rule": "demo deterministic candidate selection"
    });
    let arbitration = serde_json::json!({
        "cognitive_arbitration_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "generated_from": generated_from.clone(),
        "route_selected": runtime.arbitration.route_selected,
        "reasoning_mode": runtime.arbitration.reasoning_mode,
        "confidence": runtime.arbitration.confidence,
        "risk_class": runtime.arbitration.risk_class,
        "applied_constraints": runtime.arbitration.applied_constraints,
        "cost_latency_assumption": runtime.arbitration.cost_latency_assumption,
        "route_reason": runtime.arbitration.route_reason,
        "deterministic_selection_rule": "demo deterministic arbitration"
    });
    let execution_iterations = serde_json::json!({
        "bounded_execution_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "generated_from": generated_from.clone(),
        "selected_candidate_id": runtime.agency.selected_candidate_id,
        "selected_path": runtime.fast_slow.selected_path,
        "execution_status": runtime.bounded_execution.execution_status,
        "continuation_state": runtime.bounded_execution.continuation_state,
        "provisional_termination_state": runtime.bounded_execution.provisional_termination_state,
        "iteration_count": runtime.bounded_execution.iterations.len(),
        "iterations": runtime.bounded_execution.iterations,
        "deterministic_execution_rule": "demo deterministic bounded execution"
    });
    let evaluation = serde_json::json!({
        "evaluation_signals_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "generated_from": generated_from.clone(),
        "selected_candidate_id": runtime.agency.selected_candidate_id,
        "selected_path": runtime.fast_slow.selected_path,
        "progress_signal": runtime.evaluation.progress_signal,
        "contradiction_signal": runtime.evaluation.contradiction_signal,
        "failure_signal": runtime.evaluation.failure_signal,
        "termination_reason": runtime.evaluation.termination_reason,
        "behavior_effect": runtime.evaluation.behavior_effect,
        "next_control_action": runtime.evaluation.next_control_action,
        "deterministic_evaluation_rule": "demo deterministic evaluation"
    });
    let reframing = serde_json::json!({
        "reframing_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "generated_from": generated_from.clone(),
        "selected_candidate_id": runtime.agency.selected_candidate_id,
        "selected_path": runtime.fast_slow.selected_path,
        "frame_adequacy_score": runtime.reframing.frame_adequacy_score,
        "reframing_trigger": runtime.reframing.reframing_trigger,
        "reframing_reason": runtime.reframing.reframing_reason,
        "prior_frame": runtime.reframing.prior_frame,
        "new_frame": runtime.reframing.new_frame,
        "reexecution_choice": runtime.reframing.reexecution_choice,
        "post_reframe_state": runtime.reframing.post_reframe_state,
        "deterministic_reframing_rule": "demo deterministic reframing"
    });
    let memory = serde_json::json!({
        "control_path_memory_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "generated_from": generated_from.clone(),
        "read": {
            "memory_read_version": 1,
            "run_id": DEMO_G_V086_CONTROL_PATH,
            "generated_from": generated_from.clone(),
            "query": runtime.memory.read.query,
            "read_count": runtime.memory.read.entries.len(),
            "entries": runtime.memory.read.entries,
            "retrieval_order": runtime.memory.read.retrieval_order,
            "influence_summary": runtime.memory.read.influence_summary,
            "influenced_stage": runtime.memory.read.influenced_stage,
            "deterministic_read_rule": "demo deterministic memory read"
        },
        "write": {
            "memory_write_version": 1,
            "run_id": DEMO_G_V086_CONTROL_PATH,
            "generated_from": generated_from.clone(),
            "entry_id": runtime.memory.write.entry_id,
            "content": runtime.memory.write.content,
            "tags": runtime.memory.write.tags,
            "logical_timestamp": runtime.memory.write.logical_timestamp,
            "write_reason": runtime.memory.write.write_reason,
            "source": runtime.memory.write.source,
            "deterministic_write_rule": "demo deterministic memory write"
        }
    });
    let freedom_gate = serde_json::json!({
        "freedom_gate_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "generated_from": generated_from.clone(),
        "input": runtime.freedom_gate.input,
        "gate_decision": runtime.freedom_gate.gate_decision,
        "reason_code": runtime.freedom_gate.reason_code,
        "decision_reason": runtime.freedom_gate.decision_reason,
        "selected_action_or_none": runtime.freedom_gate.selected_action_or_none,
        "commitment_blocked": runtime.freedom_gate.commitment_blocked,
        "deterministic_gate_rule": "demo deterministic freedom gate"
    });
    let final_result = serde_json::json!({
        "control_path_final_result_version": 1,
        "run_id": DEMO_G_V086_CONTROL_PATH,
        "route_selected": "slow",
        "selected_candidate": "cand-custom-review",
        "termination_reason": "contradiction_detected",
        "gate_decision": "defer",
        "final_result": "defer",
        "commitment_blocked": true,
        "next_control_action": "handoff_to_reframing",
        "stage_order": [
            "signals",
            "candidate_selection",
            "arbitration",
            "execution",
            "evaluation",
            "reframing",
            "memory",
            "freedom_gate",
            "final_result"
        ]
    });
    let summary = [
        "v0.86 canonical bounded cognitive path summary",
        "run_id: demo-g-v086-control-path",
        "stage_order: signals -> candidate_selection -> arbitration -> execution -> evaluation -> reframing -> memory -> freedom_gate -> final_result",
        "signals: instinct=integrity completion_pressure=guarded",
        "candidate_selection: candidate_id=cand-custom-review rationale=custom selected candidate reason",
        "arbitration: route=slow reasoning_mode=review_heavy",
        "execution: status=completed iterations=2",
        "evaluation: termination_reason=contradiction_detected next_control_action=handoff_to_reframing",
        "reframing: trigger=triggered choice=bounded_reframe_and_retry",
        "memory: read_count=1 influenced_stage=reframing write_reason=record_failure_for_future_reframing_context",
        "freedom_gate: decision=defer reason_code=frame_inadequate commitment_blocked=true",
        "final_result: defer",
    ]
    .join("\n");

    artifacts.push(write_file(
        out_dir,
        "signals.json",
        &serde_json::to_string_pretty(&signals)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "candidate_selection.json",
        &serde_json::to_string_pretty(&candidate_selection)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "arbitration.json",
        &serde_json::to_string_pretty(&arbitration)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "execution_iterations.json",
        &serde_json::to_string_pretty(&execution_iterations)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "evaluation.json",
        &serde_json::to_string_pretty(&evaluation)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "reframing.json",
        &serde_json::to_string_pretty(&reframing)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "memory.json",
        &serde_json::to_string_pretty(&memory)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "freedom_gate.json",
        &serde_json::to_string_pretty(&freedom_gate)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "final_result.json",
        &serde_json::to_string_pretty(&final_result)?,
    )?);
    artifacts.push(write_file(out_dir, "summary.txt", &summary)?);

    Ok(artifacts)
}

pub fn write_v086_fast_slow_demo(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut artifacts = Vec::new();
    let simple_arbitration = execute::CognitiveArbitrationState {
        route_selected: "fast".to_string(),
        reasoning_mode: "quick_commit".to_string(),
        confidence: "high".to_string(),
        risk_class: "low".to_string(),
        applied_constraints: vec!["bounded_latency_budget".to_string()],
        cost_latency_assumption: "prefer minimal deliberation for low-risk local tasks".to_string(),
        route_reason: "simple bounded summary task remains on the fast path".to_string(),
    };
    let simple_path = execute::FastSlowPathState {
        selected_path: "fast_path".to_string(),
        path_family: "fast".to_string(),
        runtime_branch_taken: "fast_direct_execution_branch".to_string(),
        handoff_state: "direct_commit".to_string(),
        candidate_strategy: "select highest-confidence direct execution candidate".to_string(),
        review_depth: "minimal".to_string(),
        execution_profile: "single_pass_bounded_execution".to_string(),
        termination_expectation: "terminate_after_direct_execution".to_string(),
        path_difference_summary: "fast path favors immediate bounded execution with minimal review"
            .to_string(),
    };
    let complex_arbitration = execute::CognitiveArbitrationState {
        route_selected: "slow".to_string(),
        reasoning_mode: "review_heavy".to_string(),
        confidence: "guarded".to_string(),
        risk_class: "high".to_string(),
        applied_constraints: vec![
            "ambiguity_requires_review".to_string(),
            "contradiction_risk_present".to_string(),
        ],
        cost_latency_assumption:
            "spend bounded additional cognition when ambiguity and contradiction risk are present"
                .to_string(),
        route_reason: "complex bounded planning task requires review-first slow routing"
            .to_string(),
    };
    let complex_path = execute::FastSlowPathState {
        selected_path: "slow_path".to_string(),
        path_family: "slow".to_string(),
        runtime_branch_taken: "slow_review_refine_branch".to_string(),
        handoff_state: "review_handoff".to_string(),
        candidate_strategy: "compare multiple candidates before commitment".to_string(),
        review_depth: "verification_required".to_string(),
        execution_profile: "review_and_refine_before_execution".to_string(),
        termination_expectation: "terminate_after_review_cycle_or_gate_decision".to_string(),
        path_difference_summary:
            "slow path introduces explicit review and refinement before execution".to_string(),
    };

    let simple_case = serde_json::json!({
        "scenario": "simple_case",
        "task": "Summarize one bounded local artifact update",
        "arbitration": simple_arbitration,
        "fast_slow_path": simple_path,
    });
    let complex_case = serde_json::json!({
        "scenario": "complex_case",
        "task": "Diagnose contradiction in a high-risk bounded planning request",
        "arbitration": complex_arbitration,
        "fast_slow_path": complex_path,
    });
    let comparison = [
        "Fast vs Slow routing demo comparison",
        "simple_case: route=fast selected_path=fast_path branch=fast_direct_execution_branch",
        "complex_case: route=slow selected_path=slow_path branch=slow_review_refine_branch",
        "difference: the complex case introduces explicit review depth and refinement before commitment",
    ]
    .join("\n");

    artifacts.push(write_file(
        out_dir,
        "simple_case.json",
        &serde_json::to_string_pretty(&simple_case)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "complex_case.json",
        &serde_json::to_string_pretty(&complex_case)?,
    )?);
    artifacts.push(write_file(out_dir, "comparison.txt", &comparison)?);

    Ok(artifacts)
}

pub fn write_v086_candidate_selection_demo(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut artifacts = Vec::new();
    let candidates = vec![
        execute::AgencyCandidateRecord {
            candidate_id: "cand-direct-execute".to_string(),
            candidate_kind: "direct_execution".to_string(),
            bounded_action: "Apply the bounded update immediately".to_string(),
            review_requirement: "none".to_string(),
            execution_priority: 2,
            rationale: "Fastest path, but lower resilience under contradiction".to_string(),
        },
        execute::AgencyCandidateRecord {
            candidate_id: "cand-review-refine".to_string(),
            candidate_kind: "review_and_refine".to_string(),
            bounded_action: "Review and refine before execution".to_string(),
            review_requirement: "verification_required".to_string(),
            execution_priority: 1,
            rationale: "Preferred bounded candidate when contradiction risk is present".to_string(),
        },
        execute::AgencyCandidateRecord {
            candidate_id: "cand-defer".to_string(),
            candidate_kind: "defer".to_string(),
            bounded_action: "Defer and request clarification".to_string(),
            review_requirement: "none".to_string(),
            execution_priority: 3,
            rationale: "Safest fallback if the frame remains inadequate".to_string(),
        },
    ];
    let candidates_artifact = serde_json::json!({
        "agency_selection_version": 1,
        "run_id": "demo-v086-candidate-selection",
        "candidate_generation_basis": "bounded local planning scenario with contradiction risk",
        "selection_mode": "slow_candidate_comparison",
        "candidate_set": candidates,
    });
    let selection_artifact = serde_json::json!({
        "run_id": "demo-v086-candidate-selection",
        "selected_candidate_id": "cand-review-refine",
        "selected_candidate_kind": "review_and_refine",
        "selected_candidate_action": "Review and refine before execution",
        "selected_candidate_reason": "candidate preserves bounded progress while handling contradiction explicitly",
        "deterministic_selection_rule": "prefer lower-risk constrained candidates when contradiction is present",
    });
    let summary = [
        "Candidate selection demo summary",
        "candidate_count: 3",
        "selected_candidate_id: cand-review-refine",
        "selected_candidate_kind: review_and_refine",
        "selection_reason: candidate preserves bounded progress while handling contradiction explicitly",
    ]
    .join("\n");

    artifacts.push(write_file(
        out_dir,
        "candidates.json",
        &serde_json::to_string_pretty(&candidates_artifact)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "selection.json",
        &serde_json::to_string_pretty(&selection_artifact)?,
    )?);
    artifacts.push(write_file(out_dir, "summary.txt", &summary)?);

    Ok(artifacts)
}

pub fn write_v086_freedom_gate_demo(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut artifacts = Vec::new();
    let allowed = freedom_gate::FreedomGateInput {
        candidate_id: "cand-allow-execute".to_string(),
        candidate_action: "execute bounded documentation update".to_string(),
        candidate_rationale: "low-risk bounded action with sufficient context".to_string(),
        risk_class: "low".to_string(),
        policy_context: freedom_gate::FreedomGatePolicyContext {
            route_selected: "fast".to_string(),
            selected_candidate_kind: "direct_execution".to_string(),
            requires_review: false,
            policy_blocked: false,
        },
        evaluation_signals: freedom_gate::FreedomGateEvaluationSignals {
            progress_signal: "steady_progress".to_string(),
            contradiction_signal: "none".to_string(),
            failure_signal: "none".to_string(),
            termination_reason: "success".to_string(),
        },
        frame_state: "complete_run".to_string(),
    };
    let blocked = freedom_gate::FreedomGateInput {
        candidate_id: "cand-block-secret-write".to_string(),
        candidate_action: "exfiltrate secrets to external host".to_string(),
        candidate_rationale: "high-energy action intentionally blocked for demo review".to_string(),
        risk_class: "high".to_string(),
        policy_context: freedom_gate::FreedomGatePolicyContext {
            route_selected: "slow".to_string(),
            selected_candidate_kind: "direct_execution".to_string(),
            requires_review: false,
            policy_blocked: true,
        },
        evaluation_signals: freedom_gate::FreedomGateEvaluationSignals {
            progress_signal: "guarded_progress".to_string(),
            contradiction_signal: "present".to_string(),
            failure_signal: "none".to_string(),
            termination_reason: "contradiction_detected".to_string(),
        },
        frame_state: "complete_run".to_string(),
    };
    let allowed_case = serde_json::json!({
        "scenario": "allowed_case",
        "input": allowed,
        "decision": freedom_gate::evaluate_freedom_gate(&allowed),
    });
    let blocked_case = serde_json::json!({
        "scenario": "blocked_case",
        "input": blocked,
        "decision": freedom_gate::evaluate_freedom_gate(&blocked),
    });
    let summary = [
        "Freedom Gate demo summary",
        "allowed_case: allow / policy_allowed / commitment_blocked=false",
        "blocked_case: refuse / policy_blocked / commitment_blocked=true",
    ]
    .join("\n");

    artifacts.push(write_file(
        out_dir,
        "allowed_case.json",
        &serde_json::to_string_pretty(&allowed_case)?,
    )?);
    artifacts.push(write_file(
        out_dir,
        "blocked_case.json",
        &serde_json::to_string_pretty(&blocked_case)?,
    )?);
    artifacts.push(write_file(out_dir, "summary.txt", &summary)?);

    Ok(artifacts)
}

pub fn write_v086_review_surface_demo(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut artifacts = Vec::new();
    let d1_dir = out_dir.join("d1_control_path");
    let d2_dir = out_dir.join("d2_fast_slow");
    let d3_dir = out_dir.join("d3_candidate_selection");
    let d4_dir = out_dir.join("d4_freedom_gate");

    artifacts.extend(write_v086_control_path_demo(&d1_dir)?);
    artifacts.extend(write_v086_fast_slow_demo(&d2_dir)?);
    artifacts.extend(write_v086_candidate_selection_demo(&d3_dir)?);
    artifacts.extend(write_v086_freedom_gate_demo(&d4_dir)?);

    let manifest = serde_json::json!({
        "schema_version": "v086_demo_manifest.v1",
        "review_entry_demo": "D1",
        "demos": [
            {
                "demo_id": "D1",
                "title": "Canonical Bounded Cognitive Path",
                "command": "./adl/tools/demo_v086_control_path.sh",
                "artifact_root": "d1_control_path",
                "primary_proof_surface": "d1_control_path/summary.txt",
            },
            {
                "demo_id": "D2",
                "title": "Fast vs Slow Routing",
                "command": "./adl/tools/demo_v086_fast_slow.sh",
                "artifact_root": "d2_fast_slow",
                "primary_proof_surface": "d2_fast_slow/comparison.txt",
            },
            {
                "demo_id": "D3",
                "title": "Agency / Candidate Selection",
                "command": "./adl/tools/demo_v086_candidate_selection.sh",
                "artifact_root": "d3_candidate_selection",
                "primary_proof_surface": "d3_candidate_selection/selection.json",
            },
            {
                "demo_id": "D4",
                "title": "Freedom Gate Enforcement",
                "command": "./adl/tools/demo_v086_freedom_gate.sh",
                "artifact_root": "d4_freedom_gate",
                "primary_proof_surface": "d4_freedom_gate/blocked_case.json",
            }
        ]
    });
    let readme = [
        "v0.86 Review Surface Walkthrough",
        "",
        "Run D1 first if you only inspect one proof surface.",
        "Primary entry point: d1_control_path/summary.txt",
        "",
        "Other primary proof surfaces:",
        "- D2: d2_fast_slow/comparison.txt",
        "- D3: d3_candidate_selection/selection.json",
        "- D4: d4_freedom_gate/blocked_case.json",
        "",
        "Use docs/milestones/v0.86/DEMO_MATRIX_v0.86.md for the full review contract.",
    ]
    .join("\n");
    let index = [
        "D1 -> d1_control_path/summary.txt",
        "D2 -> d2_fast_slow/comparison.txt",
        "D3 -> d3_candidate_selection/selection.json",
        "D4 -> d4_freedom_gate/blocked_case.json",
    ]
    .join("\n");

    artifacts.push(write_file(
        out_dir,
        "demo_manifest.json",
        &serde_json::to_string_pretty(&manifest)?,
    )?);
    artifacts.push(write_file(out_dir, "README.txt", &readme)?);
    artifacts.push(write_file(out_dir, "index.txt", &index)?);

    Ok(artifacts)
}
