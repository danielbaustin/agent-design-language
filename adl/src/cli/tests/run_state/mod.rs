use super::*;
use crate::cli::run_artifacts;
use crate::cli::run_artifacts::{
    AgencySelectionArtifact, BoundedExecutionArtifact, CognitiveArbitrationArtifact,
    CognitiveSignalsArtifact, ControlPathFinalResultArtifact, ControlPathMemoryArtifact,
    EvaluationSignalsArtifact, FastSlowPathArtifact, FreedomGateArtifact, MemoryReadArtifact,
    MemoryWriteArtifact,
};

pub(super) fn minimal_resolved_for_artifacts(run_id: String) -> resolve::AdlResolved {
    resolve::AdlResolved {
        run_id,
        workflow_id: "wf".to_string(),
        steps: vec![resolve::ResolvedStep {
            id: "s1".to_string(),
            agent: Some("a1".to_string()),
            provider: Some("p1".to_string()),
            placement: None,
            task: Some("t1".to_string()),
            call: None,
            with: HashMap::new(),
            as_ns: None,
            delegation: None,
            conversation: None,
            prompt: Some(adl::PromptSpec {
                user: Some("u".to_string()),
                ..Default::default()
            }),
            inputs: HashMap::new(),
            guards: vec![],
            save_as: Some("s1_out".to_string()),
            write_to: Some("out/s1.txt".to_string()),
            on_error: None,
            retry: None,
        }],
        execution_plan: ::adl::execution_plan::ExecutionPlan {
            workflow_kind: adl::WorkflowKind::Sequential,
            nodes: vec![::adl::execution_plan::ExecutionNode {
                step_id: "s1".to_string(),
                depends_on: vec![],
                save_as: Some("s1_out".to_string()),
                delegation: None,
            }],
        },
        doc: adl::AdlDoc {
            version: "0.5".to_string(),
            providers: HashMap::new(),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: vec![],
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: Some("run".to_string()),
                created_at: None,
                defaults: adl::RunDefaults::default(),
                workflow_ref: None,
                workflow: Some(adl::WorkflowSpec {
                    id: Some("wf".to_string()),
                    kind: adl::WorkflowKind::Sequential,
                    max_concurrency: None,
                    steps: vec![],
                }),
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        },
    }
}

fn runtime_control_for(status: &str, tr: &trace::Trace) -> execute::RuntimeControlState {
    execute::derive_runtime_control_state(status, &[], tr)
}

fn custom_runtime_control() -> execute::RuntimeControlState {
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
            downstream_influence: "custom downstream influence".to_string(),
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
            route_reason: "custom arbitration reason".to_string(),
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
            path_difference_summary: "custom path difference summary".to_string(),
        },
        agency: execute::AgencySelectionState {
            candidate_generation_basis: "custom generation basis".to_string(),
            selection_mode: "slow_candidate_comparison".to_string(),
            candidate_set: vec![execute::AgencyCandidateRecord {
                candidate_id: "cand-custom-review".to_string(),
                candidate_kind: "review_and_refine".to_string(),
                bounded_action: "review and refine the candidate".to_string(),
                review_requirement: "verification_required".to_string(),
                execution_priority: 1,
                rationale: "custom rationale".to_string(),
            }],
            selected_candidate_id: "cand-custom-review".to_string(),
            selected_candidate_kind: "review_and_refine".to_string(),
            selected_candidate_action: "review and refine the candidate".to_string(),
            selected_candidate_reason: "custom selected candidate reason".to_string(),
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
                candidate_rationale: "custom selected candidate reason".to_string(),
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
                    tags: vec![
                        "status:failed".to_string(),
                        "workflow:wf".to_string(),
                    ],
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

mod basics;
mod failure_taxonomy;
mod persistence;
mod runtime_control;
