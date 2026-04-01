use super::*;

#[test]
fn build_affect_state_artifact_covers_watchful_and_steady_modes() {
    let summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "affect-state-run".to_string(),
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
    let generated_from = run_artifacts::SuggestionsGeneratedFrom {
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_summary_version: 1,
        scores_version: Some(1),
    };

    let watchful = run_artifacts::build_affect_state_artifact(
        &summary,
        &run_artifacts::SuggestionsArtifact {
            suggestions_version: 1,
            run_id: "affect-state-run".to_string(),
            generated_from: generated_from.clone(),
            suggestions: vec![run_artifacts::SuggestionItem {
                id: "sug-watchful".to_string(),
                category: "stability".to_string(),
                severity: "warn".to_string(),
                rationale: "One retry was needed; keep a guarded posture.".to_string(),
                evidence: run_artifacts::SuggestionEvidence {
                    failure_count: 0,
                    retry_count: 1,
                    delegation_denied_count: 0,
                    security_denied_count: 0,
                    success_ratio: 1.0,
                    scheduler_max_parallel_observed: 1,
                },
                proposed_change: run_artifacts::SuggestedChangeIntent {
                    intent: "maintain_current_policy".to_string(),
                    target: "workflow-runtime".to_string(),
                },
            }],
        },
        None,
    );
    assert_eq!(watchful.affect.affect_mode, "watchful_adjustment");
    assert_eq!(watchful.affect.recovery_bias, 1);

    let steady = run_artifacts::build_affect_state_artifact(
        &summary,
        &run_artifacts::SuggestionsArtifact {
            suggestions_version: 1,
            run_id: "affect-state-run".to_string(),
            generated_from,
            suggestions: vec![run_artifacts::SuggestionItem {
                id: "sug-steady".to_string(),
                category: "stability".to_string(),
                severity: "info".to_string(),
                rationale: "No adaptation needed.".to_string(),
                evidence: run_artifacts::SuggestionEvidence {
                    failure_count: 0,
                    retry_count: 0,
                    delegation_denied_count: 0,
                    security_denied_count: 0,
                    success_ratio: 1.0,
                    scheduler_max_parallel_observed: 1,
                },
                proposed_change: run_artifacts::SuggestedChangeIntent {
                    intent: "maintain_current_policy".to_string(),
                    target: "workflow-runtime".to_string(),
                },
            }],
        },
        None,
    );
    assert_eq!(steady.affect.affect_mode, "steady_state");
    assert_eq!(steady.affect.recovery_bias, 0);
}

#[test]
fn build_cognitive_signals_artifact_is_deterministic_and_bounded() {
    let summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "cognitive-signals-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.86".to_string(),
        swarm_version: "test".to_string(),
        status: "failure".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 2,
            completed_steps: 2,
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
    let scores = ScoresArtifact {
        scores_version: 1,
        run_id: "cognitive-signals-run".to_string(),
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
    let suggestions = build_suggestions_artifact(&summary, Some(&scores));

    let left =
        run_artifacts::build_cognitive_signals_artifact(&summary, &suggestions, Some(&scores));
    let right =
        run_artifacts::build_cognitive_signals_artifact(&summary, &suggestions, Some(&scores));

    assert_eq!(
        serde_json::to_value(&left).expect("left value"),
        serde_json::to_value(&right).expect("right value")
    );
    assert_eq!(left.cognitive_signals_version, 1);
    assert_eq!(left.instinct.dominant_instinct, "completion");
    assert_eq!(left.instinct.completion_pressure, "elevated");
    assert_eq!(left.affect.salience_level, "high");
    assert_eq!(left.affect.confidence_shift, "reduced");
    assert!(left
        .affect
        .downstream_influence
        .contains("selected_intent=increase_step_retry_budget"));
}

#[test]
fn build_cognitive_signals_state_is_deterministic_and_runtime_usable() {
    let summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "cognitive-signals-state-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.86".to_string(),
        swarm_version: "test".to_string(),
        status: "failure".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 2,
            completed_steps: 2,
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
    let scores = ScoresArtifact {
        scores_version: 1,
        run_id: "cognitive-signals-state-run".to_string(),
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
    let suggestions = build_suggestions_artifact(&summary, Some(&scores));

    let left = run_artifacts::build_cognitive_signals_state(&summary, &suggestions, Some(&scores));
    let right = run_artifacts::build_cognitive_signals_state(&summary, &suggestions, Some(&scores));

    assert_eq!(left.dominant_instinct, right.dominant_instinct);
    assert_eq!(left.confidence_shift, right.confidence_shift);
    assert_eq!(left.persistence_pressure, right.persistence_pressure);
    assert_eq!(left.dominant_instinct, "completion");
    assert_eq!(left.confidence_shift, "reduced");
    assert_eq!(left.persistence_pressure, "retry_biased");
}

#[test]
fn build_cognitive_arbitration_artifact_is_deterministic_and_routes_boundedly() {
    let summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "cognitive-arbitration-run".to_string(),
        workflow_id: "wf".to_string(),
        adl_version: "0.86".to_string(),
        swarm_version: "test".to_string(),
        status: "failure".to_string(),
        error_kind: None,
        counts: RunSummaryCounts {
            total_steps: 2,
            completed_steps: 2,
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
    let scores = ScoresArtifact {
        scores_version: 1,
        run_id: "cognitive-arbitration-run".to_string(),
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
    let suggestions = build_suggestions_artifact(&summary, Some(&scores));
    let signals =
        run_artifacts::build_cognitive_signals_artifact(&summary, &suggestions, Some(&scores));
    let affect_state =
        run_artifacts::build_affect_state_artifact(&summary, &suggestions, Some(&scores));

    let left = run_artifacts::build_cognitive_arbitration_artifact(
        &summary,
        &suggestions,
        &signals,
        &affect_state,
        Some(&scores),
    );
    let right = run_artifacts::build_cognitive_arbitration_artifact(
        &summary,
        &suggestions,
        &signals,
        &affect_state,
        Some(&scores),
    );

    assert_eq!(
        serde_json::to_value(&left).expect("left value"),
        serde_json::to_value(&right).expect("right value")
    );
    assert_eq!(left.cognitive_arbitration_version, 1);
    assert_eq!(left.route_selected, "slow");
    assert_eq!(left.reasoning_mode, "review_heavy");
    assert_eq!(left.risk_class, "medium");
    assert!(left
        .applied_constraints
        .contains(&"failure_recovery_bias".to_string()));
    assert!(left
        .route_reason
        .contains("selected_intent=increase_step_retry_budget"));
    assert!(left.route_reason.contains("dominant_instinct=completion"));
}

#[test]
fn build_cognitive_arbitration_artifact_consumes_signal_state() {
    let summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "cognitive-arbitration-signals-run".to_string(),
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
    let scores = ScoresArtifact {
        scores_version: 1,
        run_id: "cognitive-arbitration-signals-run".to_string(),
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
    let suggestions = build_suggestions_artifact(&summary, Some(&scores));
    let affect_state =
        run_artifacts::build_affect_state_artifact(&summary, &suggestions, Some(&scores));
    let baseline_signals =
        run_artifacts::build_cognitive_signals_artifact(&summary, &suggestions, Some(&scores));
    let mut reduced_confidence = baseline_signals.clone();
    reduced_confidence.affect.confidence_shift = "reduced".to_string();
    reduced_confidence.affect.persistence_pressure = "sustained".to_string();

    let fast = run_artifacts::build_cognitive_arbitration_artifact(
        &summary,
        &suggestions,
        &baseline_signals,
        &affect_state,
        Some(&scores),
    );
    let hybrid = run_artifacts::build_cognitive_arbitration_artifact(
        &summary,
        &suggestions,
        &reduced_confidence,
        &affect_state,
        Some(&scores),
    );

    assert_eq!(fast.route_selected, "fast");
    assert_eq!(hybrid.route_selected, "hybrid");
}

#[test]
fn build_fast_slow_path_artifact_is_deterministic_and_distinguishes_modes() {
    let mut summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "fast-slow-path-run".to_string(),
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
        run_id: "fast-slow-path-run".to_string(),
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
    let success_suggestions = build_suggestions_artifact(&summary, Some(&success_scores));
    let success_affect = run_artifacts::build_affect_state_artifact(
        &summary,
        &success_suggestions,
        Some(&success_scores),
    );
    let success_signals = run_artifacts::build_cognitive_signals_artifact(
        &summary,
        &success_suggestions,
        Some(&success_scores),
    );
    let success_arbitration = run_artifacts::build_cognitive_arbitration_artifact(
        &summary,
        &success_suggestions,
        &success_signals,
        &success_affect,
        Some(&success_scores),
    );
    let success_state = run_artifacts::build_fast_slow_path_state(&success_arbitration);
    let fast_left = run_artifacts::build_fast_slow_path_artifact(
        &summary,
        &success_arbitration,
        &success_state,
        Some(&success_scores),
    );
    let fast_right = run_artifacts::build_fast_slow_path_artifact(
        &summary,
        &success_arbitration,
        &success_state,
        Some(&success_scores),
    );
    assert_eq!(
        serde_json::to_value(&fast_left).expect("fast left value"),
        serde_json::to_value(&fast_right).expect("fast right value")
    );
    assert_eq!(fast_left.selected_path, "fast_path");
    assert_eq!(
        fast_left.runtime_branch_taken,
        "fast_direct_execution_branch"
    );
    assert_eq!(fast_left.review_depth, "minimal");
    assert_eq!(fast_left.execution_profile, "single_pass_direct_execution");

    summary.status = "failure".to_string();
    summary.counts.failed_steps = 1;
    let failure_scores = ScoresArtifact {
        scores_version: 1,
        run_id: "fast-slow-path-run".to_string(),
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
    let failure_suggestions = build_suggestions_artifact(&summary, Some(&failure_scores));
    let failure_affect = run_artifacts::build_affect_state_artifact(
        &summary,
        &failure_suggestions,
        Some(&failure_scores),
    );
    let failure_signals = run_artifacts::build_cognitive_signals_artifact(
        &summary,
        &failure_suggestions,
        Some(&failure_scores),
    );
    let failure_arbitration = run_artifacts::build_cognitive_arbitration_artifact(
        &summary,
        &failure_suggestions,
        &failure_signals,
        &failure_affect,
        Some(&failure_scores),
    );
    let failure_state = run_artifacts::build_fast_slow_path_state(&failure_arbitration);
    let slow = run_artifacts::build_fast_slow_path_artifact(
        &summary,
        &failure_arbitration,
        &failure_state,
        Some(&failure_scores),
    );
    assert_eq!(slow.fast_slow_path_version, 1);
    assert_eq!(slow.selected_path, "slow_path");
    assert_eq!(slow.runtime_branch_taken, "slow_review_refine_branch");
    assert_eq!(slow.review_depth, "verification_required");
    assert_eq!(slow.execution_profile, "review_and_refine_before_execution");
    assert_ne!(
        fast_left.path_difference_summary,
        slow.path_difference_summary
    );
}
