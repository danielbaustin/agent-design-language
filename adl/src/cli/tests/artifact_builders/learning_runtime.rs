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
            frame_state: "retain_current_frame".to_string(),
        },
        gate_decision: "allow".to_string(),
        reason_code: "within_policy".to_string(),
        decision_reason: "allowed".to_string(),
        selected_action_or_none: Some("execute".to_string()),
        commitment_blocked: false,
        deterministic_gate_rule: "rule".to_string(),
    };
    let defer_gate = run_artifacts::FreedomGateArtifact {
        gate_decision: "defer".to_string(),
        reason_code: "frame_inadequate".to_string(),
        decision_reason: "defer".to_string(),
        selected_action_or_none: None,
        commitment_blocked: true,
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
            frame_state: "ready_for_reframed_execution".to_string(),
        },
        gate_decision: "defer".to_string(),
        reason_code: "frame_inadequate".to_string(),
        decision_reason: "frame state requires bounded reframing before commitment can be allowed"
            .to_string(),
        selected_action_or_none: None,
        commitment_blocked: true,
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
    assert_eq!(left.gate_decision, "defer");
    assert_eq!(left.reason_code, "frame_inadequate");
    assert!(left.commitment_blocked);
    assert!(left.selected_action_or_none.is_none());
}

#[test]
fn build_reasoning_graph_artifact_changes_selected_path_with_affect() {
    let summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "reasoning-graph-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.85".to_string(),
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
    let failure_scores = ScoresArtifact {
        scores_version: 1,
        run_id: "reasoning-graph-run".to_string(),
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
    let failure_suggestions = build_suggestions_artifact(&summary, Some(&failure_scores));
    let failure_affect = run_artifacts::build_affect_state_artifact(
        &summary,
        &failure_suggestions,
        Some(&failure_scores),
    );
    let failure_decision = build_aee_decision_artifact(
        &summary,
        &failure_suggestions,
        &failure_affect,
        Some(&failure_scores),
    );
    let failure_graph = run_artifacts::build_reasoning_graph_artifact(
        &summary,
        &failure_affect,
        &failure_decision,
        Some(&failure_scores),
    );

    assert_eq!(failure_graph.graph.dominant_affect_mode, "recovery_focus");
    assert_eq!(
        failure_graph.graph.selected_path.selected_node_id,
        "action.retry_budget"
    );
    assert_eq!(
        failure_graph.graph.selected_path.selected_intent,
        "increase_step_retry_budget"
    );

    let success_summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "reasoning-graph-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.85".to_string(),
        swarm_version: "test".to_string(),
        status: "success".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 1,
            completed_steps: 1,
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
        summary: ScoresSummary {
            success_ratio: 1.0,
            failure_count: 0,
            retry_count: 0,
            delegation_denied_count: 0,
            security_denied_count: 0,
        },
        ..failure_scores
    };
    let success_suggestions = build_suggestions_artifact(&success_summary, Some(&success_scores));
    let success_affect = run_artifacts::build_affect_state_artifact(
        &success_summary,
        &success_suggestions,
        Some(&success_scores),
    );
    let success_decision = build_aee_decision_artifact(
        &success_summary,
        &success_suggestions,
        &success_affect,
        Some(&success_scores),
    );
    let success_graph = run_artifacts::build_reasoning_graph_artifact(
        &success_summary,
        &success_affect,
        &success_decision,
        Some(&success_scores),
    );

    assert_eq!(success_graph.graph.dominant_affect_mode, "steady_state");
    assert_eq!(
        success_graph.graph.selected_path.selected_node_id,
        "action.maintain_policy"
    );
    assert_eq!(
        success_graph.graph.nodes[2].node_id,
        "action.maintain_policy"
    );
    assert_eq!(success_graph.graph.nodes[2].rank, 1);
}

#[test]
fn build_run_status_tracks_attempts_and_resume_completed_steps() {
    let resolved = minimal_resolved_for_artifacts("status-run".to_string());
    let mut tr = trace::Trace::new(
        "status-run".to_string(),
        "wf".to_string(),
        "0.5".to_string(),
    );
    tr.step_started("s1", "a1", "p1", "t1", None);
    tr.step_finished("s1", true);
    tr.step_started("s2", "a1", "p1", "t1", None);
    tr.step_started("s2", "a1", "p1", "t1", None);
    tr.step_finished("s2", false);

    let steps = vec![
        StepStateArtifact {
            step_id: "s1".to_string(),
            agent_id: "a1".to_string(),
            provider_id: "p1".to_string(),
            conversation: None,
            status: "success".to_string(),
            output_artifact_path: Some("out/s1.txt".to_string()),
        },
        StepStateArtifact {
            step_id: "s2".to_string(),
            agent_id: "a1".to_string(),
            provider_id: "p1".to_string(),
            conversation: None,
            status: "failure".to_string(),
            output_artifact_path: None,
        },
        StepStateArtifact {
            step_id: "s3".to_string(),
            agent_id: "a1".to_string(),
            provider_id: "p1".to_string(),
            conversation: None,
            status: "not_run".to_string(),
            output_artifact_path: None,
        },
    ];
    let resume_completed = BTreeSet::from(["s0".to_string()]);
    let status = build_run_status(
        &resolved,
        &tr,
        "failed",
        &steps,
        None,
        None,
        &resume_completed,
    );

    assert_eq!(
        status.completed_steps,
        vec!["s0".to_string(), "s1".to_string()]
    );
    assert_eq!(
        status.pending_steps,
        vec!["s2".to_string(), "s3".to_string()]
    );
    assert_eq!(status.failed_step_id.as_deref(), Some("s2"));
    assert_eq!(status.attempt_counts_by_step.get("s0"), Some(&0));
    assert_eq!(status.attempt_counts_by_step.get("s1"), Some(&1));
    assert_eq!(status.attempt_counts_by_step.get("s2"), Some(&2));
    assert_eq!(status.started_steps.as_ref().map(|v| v.len()), Some(2));
    assert_eq!(status.resilience_classification.as_deref(), Some("crash"));
    assert_eq!(
        status.continuity_status.as_deref(),
        Some("continuity_unverified")
    );
    assert_eq!(
        status.preservation_status.as_deref(),
        Some("preserved_for_review")
    );
    assert_eq!(
        status.shepherd_decision.as_deref(),
        Some("operator_review_required")
    );
    assert_eq!(status.persistence_mode, "review_preserved_state");
    assert_eq!(status.cleanup_disposition, "retain_for_review");
    assert_eq!(status.resume_guard, "resume_not_permitted");
    assert_eq!(
        status.state_artifacts,
        vec![
            "run.json".to_string(),
            "steps.json".to_string(),
            "run_status.json".to_string(),
            "logs/trace_v1.json".to_string(),
        ]
    );
    assert!(
        status.effective_max_concurrency.is_none() || status.effective_max_concurrency == Some(4)
    );
}

#[test]
fn build_run_status_marks_paused_runs_as_resumable_interruption() {
    let resolved = minimal_resolved_for_artifacts("paused-run".to_string());
    let tr = trace::Trace::new(
        "paused-run".to_string(),
        "wf".to_string(),
        "0.87.1".to_string(),
    );
    let steps = vec![StepStateArtifact {
        step_id: "s1".to_string(),
        agent_id: "a1".to_string(),
        provider_id: "p1".to_string(),
        conversation: None,
        status: "success".to_string(),
        output_artifact_path: Some("out/s1.txt".to_string()),
    }];
    let pause = execute::PauseState {
        paused_step_id: "s2".to_string(),
        reason: Some("review".to_string()),
        completed_step_ids: vec!["s1".to_string()],
        remaining_step_ids: vec!["s2".to_string()],
        saved_state: HashMap::new(),
        completed_outputs: HashMap::new(),
    };

    let status = build_run_status(
        &resolved,
        &tr,
        "running",
        &steps,
        None,
        Some(&pause),
        &BTreeSet::new(),
    );

    assert_eq!(
        status.resilience_classification.as_deref(),
        Some("interruption")
    );
    assert_eq!(status.continuity_status.as_deref(), Some("resume_ready"));
    assert_eq!(
        status.preservation_status.as_deref(),
        Some("pause_state_preserved")
    );
    assert_eq!(
        status.shepherd_decision.as_deref(),
        Some("preserve_and_resume")
    );
    assert_eq!(status.persistence_mode, "checkpoint_resume_state");
    assert_eq!(status.cleanup_disposition, "retain_pause_state");
    assert_eq!(status.resume_guard, "execution_plan_hash_match_required");
    assert_eq!(
        status.state_artifacts,
        vec![
            "run.json".to_string(),
            "steps.json".to_string(),
            "run_status.json".to_string(),
            "logs/trace_v1.json".to_string(),
            "pause_state.json".to_string(),
        ]
    );
}

#[test]
fn build_run_status_refuses_resume_for_replay_invariant_corruption() {
    let resolved = minimal_resolved_for_artifacts("corrupt-run".to_string());
    let tr = trace::Trace::new(
        "corrupt-run".to_string(),
        "wf".to_string(),
        "0.87.1".to_string(),
    );
    let steps = vec![StepStateArtifact {
        step_id: "s1".to_string(),
        agent_id: "a1".to_string(),
        provider_id: "p1".to_string(),
        conversation: None,
        status: "failure".to_string(),
        output_artifact_path: None,
    }];
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let bad_trace_path = std::env::temp_dir().join(format!(
        "adl-replay-invariant-{now}-{}.json",
        std::process::id()
    ));
    std::fs::write(
        &bad_trace_path,
        "{\"activation_log_version\":1,\"ordering\":\"bad\",\"stable_ids\":{\"step_id\":\"x\",\"delegation_id\":\"x\",\"run_id\":\"x\"},\"events\":[]}",
    )
    .expect("write bad replay file");
    let replay_err =
        instrumentation::load_trace_artifact(&bad_trace_path).expect_err("ordering mismatch");

    let status = build_run_status(
        &resolved,
        &tr,
        "failed",
        &steps,
        Some(&replay_err),
        None,
        &BTreeSet::new(),
    );

    assert_eq!(
        status.resilience_classification.as_deref(),
        Some("corruption")
    );
    assert_eq!(
        status.continuity_status.as_deref(),
        Some("continuity_refused")
    );
    assert_eq!(
        status.preservation_status.as_deref(),
        Some("inspection_only")
    );
    assert_eq!(status.shepherd_decision.as_deref(), Some("refuse_resume"));
    assert_eq!(status.persistence_mode, "review_preserved_state");
    assert_eq!(status.cleanup_disposition, "retain_for_review");
    assert_eq!(status.resume_guard, "resume_not_permitted");

    let _ = std::fs::remove_file(&bad_trace_path);
}

#[test]
fn build_scores_and_suggestions_artifacts_are_deterministic() {
    let run_summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "run-demo".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.5".to_string(),
        swarm_version: env!("CARGO_PKG_VERSION").to_string(),
        status: "failure".to_string(),
        error_kind: Some("sandbox_denied".to_string()),
        counts: RunSummaryCounts {
            total_steps: 4,
            completed_steps: 3,
            failed_steps: 1,
            provider_call_count: 4,
            delegation_steps: 1,
            delegation_requires_verification_steps: 1,
        },
        policy: RunSummaryPolicy {
            security_envelope_enabled: true,
            signing_required: true,
            key_id_required: true,
            verify_allowed_algs: vec!["ed25519".to_string()],
            verify_allowed_key_sources: vec!["embedded".to_string()],
            sandbox_policy: "centralized_path_resolver_v1".to_string(),
            security_denials_by_code: BTreeMap::from([
                ("DELEGATION_DENIED".to_string(), 2usize),
                ("sandbox_denied".to_string(), 1usize),
            ]),
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
    let mut tr = trace::Trace::new("run-demo".to_string(), "wf".to_string(), "0.5".to_string());
    tr.step_started("a", "a1", "p1", "t1", None);
    tr.step_started("b", "a1", "p1", "t1", None);
    tr.step_finished("a", true);
    tr.step_started("b", "a1", "p1", "t1", None);
    tr.step_finished("b", false);

    let scores = build_scores_artifact(&run_summary, &tr);
    assert_eq!(scores.summary.failure_count, 1);
    assert_eq!(scores.summary.retry_count, 1);
    assert_eq!(scores.summary.delegation_denied_count, 2);
    assert_eq!(scores.summary.security_denied_count, 3);
    assert_eq!(scores.summary.success_ratio, 0.5);
    assert_eq!(scores.metrics.scheduler_max_parallel_observed, 2);

    let with_scores = build_suggestions_artifact(&run_summary, Some(&scores));
    let without_scores = build_suggestions_artifact(&run_summary, None);
    assert_eq!(with_scores.suggestions_version, 1);
    assert_eq!(
        with_scores.suggestions.first().map(|s| s.id.as_str()),
        Some("sug-001")
    );
    assert!(with_scores
        .suggestions
        .windows(2)
        .all(|pair| pair[0].id < pair[1].id));
    assert_eq!(with_scores.generated_from.scores_version, Some(1));
    assert_eq!(without_scores.generated_from.scores_version, None);
}

#[test]
fn read_scores_if_present_handles_valid_and_invalid_json() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("scores-read-{now}-{}", std::process::id());
    let runs_root = unique_temp_dir("adl-main-runs-scores");
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", &runs_root.to_string_lossy());
    let run_paths = artifacts::RunArtifactPaths::for_run(&run_id).expect("paths");
    run_paths.ensure_layout().expect("layout");

    artifacts::atomic_write(&run_paths.scores_json(), b"{not-json").expect("write invalid");
    assert!(read_scores_if_present(&run_paths).is_none());

    let valid = serde_json::to_vec_pretty(&ScoresArtifact {
        scores_version: 1,
        run_id: run_id.clone(),
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
    })
    .expect("serialize");
    artifacts::atomic_write(&run_paths.scores_json(), &valid).expect("write valid");
    let parsed = read_scores_if_present(&run_paths).expect("should parse valid score file");
    assert_eq!(parsed.run_id, run_id);

    let _ = std::fs::remove_dir_all(run_paths.run_dir());
}
