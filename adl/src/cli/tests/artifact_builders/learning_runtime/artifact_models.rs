use super::*;

#[test]
fn build_aee_convergence_artifact_distinguishes_core_outcome_classes() {
    let generated_from = AeeDecisionGeneratedFrom {
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_summary_version: 1,
        suggestions_version: 1,
        scores_version: Some(1),
    };
    let run_summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "conv-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.89".to_string(),
        swarm_version: "test".to_string(),
        status: "success".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 2,
            completed_steps: 2,
            failed_steps: 0,
            provider_call_count: 1,
            delegation_steps: 0,
            delegation_requires_verification_steps: 0,
        },
        policy: RunSummaryPolicy {
            security_envelope_enabled: false,
            signing_required: false,
            key_id_required: false,
            verify_allowed_algs: Vec::new(),
            verify_allowed_key_sources: Vec::new(),
            sandbox_policy: "centralized_path_resolver_v1".to_string(),
            security_denials_by_code: BTreeMap::new(),
        },
        links: RunSummaryLinks {
            run_json: "run.json".to_string(),
            steps_json: "steps.json".to_string(),
            pause_state_json: None,
            outputs_dir: "outputs".to_string(),
            logs_dir: "logs".to_string(),
            learning_dir: "learning".to_string(),
            scores_json: None,
            suggestions_json: None,
            aee_decision_json: None,
            cognitive_signals_json: None,
            fast_slow_path_json: None,
            agency_selection_json: None,
            bounded_execution_json: None,
            evaluation_signals_json: None,
            cognitive_arbitration_json: None,
            affect_state_json: None,
            reasoning_graph_json: None,
            overlays_dir: "learning/overlays".to_string(),
            cluster_groundwork_json: None,
            trace_json: None,
        },
    };
    let bounded_execution = run_artifacts::BoundedExecutionArtifact {
        bounded_execution_version: 1,
        run_id: "conv-run".to_string(),
        generated_from: generated_from.clone(),
        selected_candidate_id: "cand-a".to_string(),
        selected_path: "slow_path".to_string(),
        execution_status: "completed".to_string(),
        continuation_state: "bounded_review_complete".to_string(),
        provisional_termination_state: "ready_for_evaluation".to_string(),
        iteration_count: 2,
        iterations: vec![
            execute::BoundedExecutionIteration {
                iteration_index: 1,
                stage: "review".to_string(),
                action: "review".to_string(),
                outcome: "improve".to_string(),
            },
            execute::BoundedExecutionIteration {
                iteration_index: 2,
                stage: "execute".to_string(),
                action: "execute".to_string(),
                outcome: "complete".to_string(),
            },
        ],
        deterministic_execution_rule: "rule".to_string(),
    };
    let converged_evaluation = run_artifacts::EvaluationSignalsArtifact {
        evaluation_signals_version: 1,
        run_id: "conv-run".to_string(),
        generated_from: generated_from.clone(),
        selected_candidate_id: "cand-a".to_string(),
        selected_path: "slow_path".to_string(),
        progress_signal: "steady_progress".to_string(),
        contradiction_signal: "none".to_string(),
        failure_signal: "none".to_string(),
        termination_reason: "success".to_string(),
        behavior_effect: "complete".to_string(),
        next_control_action: "complete_run".to_string(),
        deterministic_evaluation_rule: "rule".to_string(),
    };
    let stalled_evaluation = run_artifacts::EvaluationSignalsArtifact {
        termination_reason: "bounded_failure".to_string(),
        progress_signal: "stalled_progress".to_string(),
        failure_signal: "bounded_failure_detected".to_string(),
        next_control_action: "handoff_to_reframing".to_string(),
        ..converged_evaluation.clone()
    };
    let bounded_out_evaluation = run_artifacts::EvaluationSignalsArtifact {
        termination_reason: "no_progress".to_string(),
        progress_signal: "stalled_progress".to_string(),
        failure_signal: "bounded_failure_detected".to_string(),
        next_control_action: "terminate_with_failure".to_string(),
        ..converged_evaluation.clone()
    };
    let handoff_evaluation = run_artifacts::EvaluationSignalsArtifact {
        termination_reason: "pause_boundary".to_string(),
        progress_signal: "guarded_progress".to_string(),
        failure_signal: "none".to_string(),
        next_control_action: "await_resume".to_string(),
        ..converged_evaluation.clone()
    };
    let steady_reframing = run_artifacts::ReframingArtifact {
        reframing_version: 1,
        run_id: "conv-run".to_string(),
        generated_from: generated_from.clone(),
        selected_candidate_id: "cand-a".to_string(),
        selected_path: "slow_path".to_string(),
        frame_adequacy_score: 88,
        reframing_trigger: "not_triggered".to_string(),
        reframing_reason: "adequate".to_string(),
        prior_frame: "direct".to_string(),
        new_frame: "retain".to_string(),
        reexecution_choice: "no_reframe_required".to_string(),
        post_reframe_state: "complete_run".to_string(),
        deterministic_reframing_rule: "rule".to_string(),
    };
    let triggered_reframing = run_artifacts::ReframingArtifact {
        reframing_trigger: "triggered".to_string(),
        reexecution_choice: "bounded_reframe_and_retry".to_string(),
        ..steady_reframing.clone()
    };
    let allow_gate = run_artifacts::FreedomGateArtifact {
        freedom_gate_version: 1,
        run_id: "conv-run".to_string(),
        generated_from: generated_from.clone(),
        input: execute::FreedomGateInputState {
            candidate_id: "cand-a".to_string(),
            candidate_action: "execute".to_string(),
            candidate_rationale: "execute the reviewed bounded candidate".to_string(),
            risk_class: "bounded".to_string(),
            policy_context: execute::FreedomGatePolicyContextState {
                route_selected: "slow".to_string(),
                selected_candidate_kind: "review_and_refine".to_string(),
                requires_review: true,
                policy_blocked: false,
            },
            evaluation_signals: execute::FreedomGateEvaluationSignalsState {
                progress_signal: "steady_progress".to_string(),
                contradiction_signal: "none".to_string(),
                failure_signal: "none".to_string(),
                termination_reason: "success".to_string(),
            },
            consequence_context: execute::FreedomGateConsequenceContextState {
                impact_scope: "local_bounded".to_string(),
                recovery_cost: "low".to_string(),
                operator_visibility: "routine".to_string(),
                escalation_available: false,
            },
            frame_state: "retain_current_frame".to_string(),
        },
        gate_decision: "allow".to_string(),
        reason_code: "within_policy".to_string(),
        decision_reason: "allowed".to_string(),
        selected_action_or_none: Some("execute".to_string()),
        commitment_blocked: false,
        judgment_boundary: "commitment_boundary".to_string(),
        required_follow_up: "commit_selected_action".to_string(),
        decision_record_kind: "gate_allow_record".to_string(),
        deterministic_gate_rule: "rule".to_string(),
    };
    let defer_gate = run_artifacts::FreedomGateArtifact {
        gate_decision: "defer".to_string(),
        reason_code: "frame_inadequate".to_string(),
        decision_reason: "defer".to_string(),
        selected_action_or_none: None,
        commitment_blocked: true,
        judgment_boundary: "frame_boundary".to_string(),
        required_follow_up: "reframe_before_commitment".to_string(),
        decision_record_kind: "gate_defer_record".to_string(),
        ..allow_gate.clone()
    };
    let scores = ScoresArtifact {
        scores_version: 1,
        run_id: "conv-run".to_string(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
        },
        summary: ScoresSummary {
            success_ratio: 1.0,
            failure_count: 0,
            retry_count: 0,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        },
    };

    let converged = run_artifacts::build_aee_convergence_artifact(
        &run_summary,
        &bounded_execution,
        &converged_evaluation,
        &steady_reframing,
        &allow_gate,
        Some(&scores),
    );
    assert_eq!(converged.convergence_state, "converged");
    assert_eq!(converged.stop_condition_family, "acceptance_satisfied");
    assert!(converged.strategy_change_visible);

    let stalled = run_artifacts::build_aee_convergence_artifact(
        &run_summary,
        &bounded_execution,
        &stalled_evaluation,
        &triggered_reframing,
        &allow_gate,
        Some(&scores),
    );
    assert_eq!(stalled.convergence_state, "stalled");
    assert_eq!(stalled.stop_condition_family, "bounded_failure_cluster");

    let bounded_out = run_artifacts::build_aee_convergence_artifact(
        &run_summary,
        &bounded_execution,
        &bounded_out_evaluation,
        &steady_reframing,
        &allow_gate,
        Some(&scores),
    );
    assert_eq!(bounded_out.convergence_state, "bounded_out");
    assert_eq!(
        bounded_out.stop_condition_family,
        "no_meaningful_improvement"
    );

    let handoff = run_artifacts::build_aee_convergence_artifact(
        &run_summary,
        &bounded_execution,
        &handoff_evaluation,
        &steady_reframing,
        &allow_gate,
        Some(&scores),
    );
    assert_eq!(handoff.convergence_state, "handoff");
    assert_eq!(handoff.stop_condition_family, "handoff_or_missing_input");

    let policy_stop = run_artifacts::build_aee_convergence_artifact(
        &run_summary,
        &bounded_execution,
        &stalled_evaluation,
        &triggered_reframing,
        &defer_gate,
        Some(&scores),
    );
    assert_eq!(policy_stop.convergence_state, "policy_stop");
    assert_eq!(policy_stop.stop_condition_family, "policy_boundary");
    assert!(policy_stop.reviewer_summary.contains("policy_stop"));
}

#[test]
fn build_reframing_artifact_is_deterministic_and_distinguishes_triggered_paths() {
    let success_summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "reframing-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.86".to_string(),
        swarm_version: "test".to_string(),
        status: "success".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 2,
            completed_steps: 2,
            failed_steps: 0,
            provider_call_count: 1,
            delegation_steps: 0,
            delegation_requires_verification_steps: 0,
        },
        policy: RunSummaryPolicy {
            security_envelope_enabled: false,
            signing_required: false,
            key_id_required: false,
            verify_allowed_algs: Vec::new(),
            verify_allowed_key_sources: Vec::new(),
            sandbox_policy: "centralized_path_resolver_v1".to_string(),
            security_denials_by_code: BTreeMap::new(),
        },
        links: RunSummaryLinks {
            run_json: "run.json".to_string(),
            steps_json: "steps.json".to_string(),
            pause_state_json: None,
            outputs_dir: "outputs".to_string(),
            logs_dir: "logs".to_string(),
            learning_dir: "learning".to_string(),
            scores_json: None,
            suggestions_json: None,
            aee_decision_json: None,
            cognitive_signals_json: None,
            fast_slow_path_json: None,
            agency_selection_json: None,
            bounded_execution_json: None,
            evaluation_signals_json: None,
            cognitive_arbitration_json: None,
            affect_state_json: None,
            reasoning_graph_json: None,
            overlays_dir: "learning/overlays".to_string(),
            cluster_groundwork_json: None,
            trace_json: None,
        },
    };
    let success_scores = ScoresArtifact {
        scores_version: 1,
        run_id: "reframing-run".to_string(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
        },
        summary: ScoresSummary {
            success_ratio: 1.0,
            failure_count: 0,
            retry_count: 0,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        },
    };
    let success_suggestions = build_suggestions_artifact(&success_summary, Some(&success_scores));
    let success_signals = run_artifacts::build_cognitive_signals_artifact(
        &success_summary,
        &success_suggestions,
        Some(&success_scores),
    );
    let success_affect = run_artifacts::build_affect_state_artifact(
        &success_summary,
        &success_suggestions,
        Some(&success_scores),
    );
    let success_arbitration = run_artifacts::build_cognitive_arbitration_artifact(
        &success_summary,
        &success_suggestions,
        &success_signals,
        &success_affect,
        Some(&success_scores),
    );
    let success_path_state = run_artifacts::build_fast_slow_path_state(&success_arbitration);
    let success_path = run_artifacts::build_fast_slow_path_artifact(
        &success_summary,
        &success_arbitration,
        &success_path_state,
        Some(&success_scores),
    );
    let success_agency_state = run_artifacts::build_agency_selection_state(
        &success_signals,
        &success_arbitration,
        &success_path_state,
        &success_path,
    );
    let success_agency = run_artifacts::build_agency_selection_artifact(
        &success_summary,
        &success_arbitration,
        &success_agency_state,
        Some(&success_scores),
    );

    let success_reframing = execute::ReframingControlState {
        frame_adequacy_score: 88,
        reframing_trigger: "not_triggered".to_string(),
        reframing_reason: "current_frame_adequate_for_bounded_progress".to_string(),
        prior_frame: "direct_execution_under_current_frame".to_string(),
        new_frame: "retain_current_frame".to_string(),
        reexecution_choice: "no_reframe_required".to_string(),
        post_reframe_state: "complete_run".to_string(),
    };
    let success_left = run_artifacts::build_reframing_artifact(
        &success_summary,
        &success_path,
        &success_agency,
        &success_reframing,
        Some(&success_scores),
    );
    let success_right = run_artifacts::build_reframing_artifact(
        &success_summary,
        &success_path,
        &success_agency,
        &success_reframing,
        Some(&success_scores),
    );
    assert_eq!(
        serde_json::to_value(&success_left).expect("success reframing value"),
        serde_json::to_value(&success_right).expect("success reframing value")
    );
    assert_eq!(success_left.reframing_trigger, "not_triggered");
    assert_eq!(success_left.frame_adequacy_score, 88);

    let failure_summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "reframing-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.86".to_string(),
        swarm_version: "test".to_string(),
        status: "failure".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 2,
            completed_steps: 1,
            failed_steps: 1,
            provider_call_count: 1,
            delegation_steps: 0,
            delegation_requires_verification_steps: 0,
        },
        policy: RunSummaryPolicy {
            security_envelope_enabled: false,
            signing_required: false,
            key_id_required: false,
            verify_allowed_algs: Vec::new(),
            verify_allowed_key_sources: Vec::new(),
            sandbox_policy: "centralized_path_resolver_v1".to_string(),
            security_denials_by_code: BTreeMap::new(),
        },
        links: RunSummaryLinks {
            run_json: "run.json".to_string(),
            steps_json: "steps.json".to_string(),
            pause_state_json: None,
            outputs_dir: "outputs".to_string(),
            logs_dir: "logs".to_string(),
            learning_dir: "learning".to_string(),
            scores_json: None,
            suggestions_json: None,
            aee_decision_json: None,
            cognitive_signals_json: None,
            fast_slow_path_json: None,
            agency_selection_json: None,
            bounded_execution_json: None,
            evaluation_signals_json: None,
            cognitive_arbitration_json: None,
            affect_state_json: None,
            reasoning_graph_json: None,
            overlays_dir: "learning/overlays".to_string(),
            cluster_groundwork_json: None,
            trace_json: None,
        },
    };
    let failure_scores = ScoresArtifact {
        scores_version: 1,
        run_id: "reframing-run".to_string(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
        },
        summary: ScoresSummary {
            success_ratio: 0.0,
            failure_count: 1,
            retry_count: 1,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        },
    };
    let failure_suggestions = build_suggestions_artifact(&failure_summary, Some(&failure_scores));
    let failure_signals = run_artifacts::build_cognitive_signals_artifact(
        &failure_summary,
        &failure_suggestions,
        Some(&failure_scores),
    );
    let failure_affect = run_artifacts::build_affect_state_artifact(
        &failure_summary,
        &failure_suggestions,
        Some(&failure_scores),
    );
    let failure_arbitration = run_artifacts::build_cognitive_arbitration_artifact(
        &failure_summary,
        &failure_suggestions,
        &failure_signals,
        &failure_affect,
        Some(&failure_scores),
    );
    let failure_path_state = run_artifacts::build_fast_slow_path_state(&failure_arbitration);
    let failure_path = run_artifacts::build_fast_slow_path_artifact(
        &failure_summary,
        &failure_arbitration,
        &failure_path_state,
        Some(&failure_scores),
    );
    let failure_agency_state = run_artifacts::build_agency_selection_state(
        &failure_signals,
        &failure_arbitration,
        &failure_path_state,
        &failure_path,
    );
    let failure_agency = run_artifacts::build_agency_selection_artifact(
        &failure_summary,
        &failure_arbitration,
        &failure_agency_state,
        Some(&failure_scores),
    );
    let failure_reframing = execute::ReframingControlState {
        frame_adequacy_score: 28,
        reframing_trigger: "triggered".to_string(),
        reframing_reason: "contradiction_detected_after_bounded_execution".to_string(),
        prior_frame: "review_and_refine_under_current_frame".to_string(),
        new_frame: "diagnose_and_restructure_before_retry".to_string(),
        reexecution_choice: "bounded_reframe_and_retry".to_string(),
        post_reframe_state: "ready_for_reframed_execution".to_string(),
    };
    let failure_artifact = run_artifacts::build_reframing_artifact(
        &failure_summary,
        &failure_path,
        &failure_agency,
        &failure_reframing,
        Some(&failure_scores),
    );
    assert_eq!(failure_artifact.reframing_trigger, "triggered");
    assert_eq!(
        failure_artifact.reexecution_choice,
        "bounded_reframe_and_retry"
    );
    assert_ne!(
        success_left.frame_adequacy_score,
        failure_artifact.frame_adequacy_score
    );
}

#[test]
fn build_memory_artifacts_are_deterministic_and_preserve_read_write_semantics() {
    let run_summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "memory-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.86".to_string(),
        swarm_version: "test".to_string(),
        status: "failure".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 1,
            completed_steps: 1,
            failed_steps: 1,
            provider_call_count: 1,
            delegation_steps: 0,
            delegation_requires_verification_steps: 0,
        },
        policy: RunSummaryPolicy {
            security_envelope_enabled: false,
            signing_required: false,
            key_id_required: false,
            verify_allowed_algs: Vec::new(),
            verify_allowed_key_sources: Vec::new(),
            sandbox_policy: "centralized_path_resolver_v1".to_string(),
            security_denials_by_code: BTreeMap::new(),
        },
        links: RunSummaryLinks {
            run_json: "run.json".to_string(),
            steps_json: "steps.json".to_string(),
            pause_state_json: None,
            outputs_dir: "outputs".to_string(),
            logs_dir: "logs".to_string(),
            learning_dir: "learning".to_string(),
            scores_json: None,
            suggestions_json: None,
            aee_decision_json: None,
            cognitive_signals_json: None,
            fast_slow_path_json: None,
            agency_selection_json: None,
            bounded_execution_json: None,
            evaluation_signals_json: None,
            cognitive_arbitration_json: None,
            affect_state_json: None,
            reasoning_graph_json: None,
            overlays_dir: "learning/overlays".to_string(),
            cluster_groundwork_json: None,
            trace_json: None,
        },
    };
    let evaluation = run_artifacts::EvaluationSignalsArtifact {
        evaluation_signals_version: 1,
        run_id: "memory-run".to_string(),
        generated_from: AeeDecisionGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
            suggestions_version: 1,
            scores_version: Some(1),
        },
        selected_candidate_id: "cand-slow-review".to_string(),
        selected_path: "slow_path".to_string(),
        progress_signal: "guarded_progress".to_string(),
        contradiction_signal: "present".to_string(),
        failure_signal: "none".to_string(),
        termination_reason: "contradiction_detected".to_string(),
        behavior_effect: "surface contradiction for bounded follow-up".to_string(),
        next_control_action: "handoff_to_reframing".to_string(),
        deterministic_evaluation_rule: "deterministic".to_string(),
    };
    let scores = ScoresArtifact {
        scores_version: 1,
        run_id: "memory-run".to_string(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
        },
        summary: ScoresSummary {
            success_ratio: 0.0,
            failure_count: 1,
            retry_count: 0,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        },
    };
    let read_state = execute::MemoryReadState {
        query: execute::MemoryQueryState {
            workflow_id: "wf".to_string(),
            status_filter: "failed".to_string(),
            limit: 3,
            source: "repo_local_runs_root".to_string(),
        },
        entries: vec![execute::MemoryReadEntry {
            memory_entry_id: "prior-run::wf".to_string(),
            run_id: "prior-run".to_string(),
            workflow_id: "wf".to_string(),
            summary: "prior failed run".to_string(),
            tags: vec!["status:failed".to_string(), "workflow:wf".to_string()],
            source: "indexed_run_artifacts".to_string(),
        }],
        retrieval_order: "workflow_id_then_run_id_ascending".to_string(),
        influence_summary:
            "prior_failure_memory reinforces bounded reframing for route=slow selected_candidate=cand-slow-review"
                .to_string(),
        influenced_stage: "reframing_decision".to_string(),
    };
    let write_state = execute::MemoryWriteState {
        entry_id: "mem-entry::wf::memory-run".to_string(),
        content:
            "workflow=wf status=failure next_control_action=handoff_to_reframing influence=prior_failure_memory"
                .to_string(),
        tags: vec![
            "action:handoff_to_reframing".to_string(),
            "candidate:review_and_refine".to_string(),
            "status:failure".to_string(),
            "workflow:wf".to_string(),
        ],
        logical_timestamp: "run:memory-run".to_string(),
        write_reason: "record_failure_for_future_reframing_context".to_string(),
        source: "runtime_control_projection".to_string(),
    };

    let read_left = run_artifacts::build_memory_read_artifact(
        &run_summary,
        &evaluation,
        &read_state,
        Some(&scores),
    );
    let read_right = run_artifacts::build_memory_read_artifact(
        &run_summary,
        &evaluation,
        &read_state,
        Some(&scores),
    );
    assert_eq!(
        serde_json::to_value(&read_left).expect("memory read left"),
        serde_json::to_value(&read_right).expect("memory read right")
    );
    assert_eq!(read_left.read_count, 1);
    assert_eq!(read_left.query.status_filter, "failed");
    assert_eq!(read_left.influenced_stage, "reframing_decision");

    let write_left = run_artifacts::build_memory_write_artifact(
        &run_summary,
        &evaluation,
        &write_state,
        Some(&scores),
    );
    let write_right = run_artifacts::build_memory_write_artifact(
        &run_summary,
        &evaluation,
        &write_state,
        Some(&scores),
    );
    assert_eq!(
        serde_json::to_value(&write_left).expect("memory write left"),
        serde_json::to_value(&write_right).expect("memory write right")
    );
    assert_eq!(
        write_left.write_reason,
        "record_failure_for_future_reframing_context"
    );
    assert_eq!(write_left.logical_timestamp, "run:memory-run");
}

#[test]
fn build_freedom_gate_artifact_is_deterministic_and_blocks_commitment_when_not_allowed() {
    let run_summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "freedom-gate-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.86".to_string(),
        swarm_version: "test".to_string(),
        status: "failure".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 1,
            completed_steps: 1,
            failed_steps: 1,
            provider_call_count: 1,
            delegation_steps: 0,
            delegation_requires_verification_steps: 0,
        },
        policy: RunSummaryPolicy {
            security_envelope_enabled: false,
            signing_required: false,
            key_id_required: false,
            verify_allowed_algs: Vec::new(),
            verify_allowed_key_sources: Vec::new(),
            sandbox_policy: "centralized_path_resolver_v1".to_string(),
            security_denials_by_code: BTreeMap::new(),
        },
        links: RunSummaryLinks {
            run_json: "run.json".to_string(),
            steps_json: "steps.json".to_string(),
            pause_state_json: None,
            outputs_dir: "outputs".to_string(),
            logs_dir: "logs".to_string(),
            learning_dir: "learning".to_string(),
            scores_json: None,
            suggestions_json: None,
            aee_decision_json: None,
            cognitive_signals_json: None,
            fast_slow_path_json: None,
            agency_selection_json: None,
            bounded_execution_json: None,
            evaluation_signals_json: None,
            cognitive_arbitration_json: None,
            affect_state_json: None,
            reasoning_graph_json: None,
            overlays_dir: "learning/overlays".to_string(),
            cluster_groundwork_json: None,
            trace_json: None,
        },
    };
    let evaluation = run_artifacts::EvaluationSignalsArtifact {
        evaluation_signals_version: 1,
        run_id: "freedom-gate-run".to_string(),
        generated_from: run_artifacts::AeeDecisionGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
            suggestions_version: 1,
            scores_version: Some(1),
        },
        selected_candidate_id: "cand-slow-review".to_string(),
        selected_path: "slow_path".to_string(),
        progress_signal: "guarded_progress".to_string(),
        contradiction_signal: "present".to_string(),
        failure_signal: "none".to_string(),
        termination_reason: "contradiction_detected".to_string(),
        behavior_effect: "surface contradiction for bounded follow-up".to_string(),
        next_control_action: "handoff_to_reframing".to_string(),
        deterministic_evaluation_rule: "deterministic".to_string(),
    };
    let scores = ScoresArtifact {
        scores_version: 1,
        run_id: "freedom-gate-run".to_string(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
            run_summary_version: 1,
        },
        summary: ScoresSummary {
            success_ratio: 0.0,
            failure_count: 1,
            retry_count: 0,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        },
    };
    let gate_state = execute::FreedomGateState {
        input: execute::FreedomGateInputState {
            candidate_id: "cand-slow-review".to_string(),
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
                progress_signal: "guarded_progress".to_string(),
                contradiction_signal: "present".to_string(),
                failure_signal: "none".to_string(),
                termination_reason: "contradiction_detected".to_string(),
            },
            consequence_context: execute::FreedomGateConsequenceContextState {
                impact_scope: "cross_surface".to_string(),
                recovery_cost: "requires_reframing".to_string(),
                operator_visibility: "review_required".to_string(),
                escalation_available: true,
            },
            frame_state: "ready_for_reframed_execution".to_string(),
        },
        gate_decision: "escalate".to_string(),
        reason_code: "frame_escalation_required".to_string(),
        decision_reason:
            "frame state and consequence context require explicit escalation before commitment can proceed"
                .to_string(),
        selected_action_or_none: None,
        commitment_blocked: true,
        judgment_boundary: "judgment_boundary".to_string(),
        required_follow_up: "escalate_for_judgment_review".to_string(),
        decision_record_kind: "gate_escalation_record".to_string(),
    };

    let left = run_artifacts::build_freedom_gate_artifact(
        &run_summary,
        &evaluation,
        &gate_state,
        Some(&scores),
    );
    let right = run_artifacts::build_freedom_gate_artifact(
        &run_summary,
        &evaluation,
        &gate_state,
        Some(&scores),
    );
    assert_eq!(
        serde_json::to_value(&left).expect("freedom gate left"),
        serde_json::to_value(&right).expect("freedom gate right")
    );
    assert_eq!(left.gate_decision, "escalate");
    assert_eq!(left.reason_code, "frame_escalation_required");
    assert!(left.commitment_blocked);
    assert!(left.selected_action_or_none.is_none());
    assert_eq!(left.decision_record_kind, "gate_escalation_record");
}
