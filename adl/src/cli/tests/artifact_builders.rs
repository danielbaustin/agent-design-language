use super::run_state::minimal_resolved_for_artifacts;
use super::*;
use crate::cli::run_artifacts;

#[test]
fn build_run_summary_sorts_remote_policy_and_tracks_denials() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let run_id = format!("summary-{now}-{}", std::process::id());
    let mut resolved = minimal_resolved_for_artifacts(run_id);
    let runs_root = unique_temp_dir("adl-main-runs-summary");
    let _runs_guard = EnvGuard::set("ADL_RUNS_ROOT", &runs_root.to_string_lossy());
    resolved.steps.push(resolve::ResolvedStep {
        id: "s2".to_string(),
        agent: Some("a1".to_string()),
        provider: Some("p1".to_string()),
        placement: None,
        task: Some("t1".to_string()),
        call: None,
        with: HashMap::new(),
        as_ns: None,
        delegation: Some(adl::DelegationSpec {
            role: Some("reviewer".to_string()),
            requires_verification: Some(true),
            escalation_target: None,
            tags: vec!["b".to_string(), "a".to_string()],
        }),
        prompt: Some(adl::PromptSpec {
            user: Some("u".to_string()),
            ..Default::default()
        }),
        inputs: HashMap::new(),
        guards: vec![],
        save_as: Some("s2_out".to_string()),
        write_to: Some("out/s2.txt".to_string()),
        on_error: None,
        retry: None,
    });
    resolved.doc.run.remote = Some(adl::RunRemoteSpec {
        endpoint: "http://127.0.0.1:8787".to_string(),
        timeout_ms: Some(30_000),
        require_signed_requests: true,
        require_key_id: true,
        verify_allowed_algs: vec!["rsa".to_string(), "ed25519".to_string(), "rsa".to_string()],
        verify_allowed_key_sources: vec![
            "embedded".to_string(),
            "kms".to_string(),
            "embedded".to_string(),
        ],
    });

    let run_paths = artifacts::RunArtifactPaths::for_run(&resolved.run_id).expect("paths");
    run_paths.ensure_layout().expect("layout");
    run_paths.write_model_marker().expect("marker");
    artifacts::atomic_write(&run_paths.scores_json(), b"{}").expect("scores");
    artifacts::atomic_write(&run_paths.suggestions_json(), b"{}").expect("suggestions");

    let steps = vec![
        StepStateArtifact {
            step_id: "s1".to_string(),
            agent_id: "a1".to_string(),
            provider_id: "p1".to_string(),
            status: "success".to_string(),
            output_artifact_path: Some("out/s1.txt".to_string()),
        },
        StepStateArtifact {
            step_id: "s2".to_string(),
            agent_id: "a1".to_string(),
            provider_id: "p1".to_string(),
            status: "failure".to_string(),
            output_artifact_path: None,
        },
        StepStateArtifact {
            step_id: "s3".to_string(),
            agent_id: "a1".to_string(),
            provider_id: "p1".to_string(),
            status: "not_run".to_string(),
            output_artifact_path: None,
        },
    ];
    let failure = anyhow::Error::new(::adl::sandbox::SandboxPathError::PathDenied {
        requested_path: "sandbox:/bad".to_string(),
        reason: "parent_traversal",
    });
    let summary = build_run_summary(
        &resolved,
        "failure",
        None,
        &steps,
        2,
        Some(&failure),
        &run_paths,
    );

    assert!(summary.policy.security_envelope_enabled);
    assert_eq!(summary.policy.verify_allowed_algs, vec!["ed25519", "rsa"]);
    assert_eq!(
        summary.policy.verify_allowed_key_sources,
        vec!["embedded", "kms"]
    );
    assert_eq!(
        summary
            .policy
            .security_denials_by_code
            .get("sandbox_denied"),
        Some(&1)
    );
    assert_eq!(summary.counts.total_steps, 2);
    assert_eq!(summary.counts.completed_steps, 2);
    assert_eq!(summary.counts.failed_steps, 1);
    assert_eq!(summary.counts.delegation_steps, 1);
    assert_eq!(summary.counts.delegation_requires_verification_steps, 1);
    assert_eq!(
        summary.links.scores_json.as_deref(),
        Some("learning/scores.json")
    );
    assert_eq!(
        summary.links.suggestions_json.as_deref(),
        Some("learning/suggestions.json")
    );
    assert_eq!(
        summary.links.aee_decision_json.as_deref(),
        Some("learning/aee_decision.json")
    );
    assert_eq!(
        summary.links.cognitive_signals_json.as_deref(),
        Some("learning/cognitive_signals.v1.json")
    );
    assert_eq!(
        summary.links.fast_slow_path_json.as_deref(),
        Some("learning/fast_slow_path.v1.json")
    );
    assert_eq!(
        summary.links.agency_selection_json.as_deref(),
        Some("learning/agency_selection.v1.json")
    );
    assert_eq!(
        summary.links.bounded_execution_json.as_deref(),
        Some("learning/bounded_execution.v1.json")
    );
    assert_eq!(
        summary.links.evaluation_signals_json.as_deref(),
        Some("learning/evaluation_signals.v1.json")
    );
    assert_eq!(
        summary.links.cognitive_arbitration_json.as_deref(),
        Some("learning/cognitive_arbitration.v1.json")
    );
    assert_eq!(
        summary.links.affect_state_json.as_deref(),
        Some("learning/affect_state.v1.json")
    );
    assert_eq!(
        summary.links.reasoning_graph_json.as_deref(),
        Some("learning/reasoning_graph.v1.json")
    );

    let _ = std::fs::remove_dir_all(run_paths.run_dir());
}

#[test]
fn build_aee_decision_artifact_selects_retry_recovery_for_failures() {
    let summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "aee-decision-run".to_string(),
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
    let scores = ScoresArtifact {
        scores_version: 1,
        run_id: "aee-decision-run".to_string(),
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
    let suggestions = build_suggestions_artifact(&summary, Some(&scores));
    let affect_state =
        run_artifacts::build_affect_state_artifact(&summary, &suggestions, Some(&scores));
    let aee_decision =
        build_aee_decision_artifact(&summary, &suggestions, &affect_state, Some(&scores));

    assert_eq!(aee_decision.aee_decision_version, AEE_DECISION_VERSION);
    assert_eq!(affect_state.affect.affect_mode, "recovery_focus");
    assert_eq!(affect_state.affect.recovery_bias, 2);
    assert_eq!(aee_decision.decision.decision_id, "aee-001");
    assert_eq!(aee_decision.decision.intent, "increase_step_retry_budget");
    assert_eq!(
        aee_decision.decision.decision_kind,
        "bounded_retry_recovery"
    );
    assert_eq!(aee_decision.decision.target, "failed-step-set");
    assert_eq!(aee_decision.affect_state.affect_state_id, "affect-001");
    assert_eq!(aee_decision.affect_state.affect_mode, "recovery_focus");
    assert_eq!(aee_decision.decision.recommended_retry_budget, Some(2));
    assert!(aee_decision
        .decision
        .expected_downstream_effect
        .contains("retry budget"));
    assert!(aee_decision
        .decision
        .expected_downstream_effect
        .contains("affect-guided recovery bias"));
}

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

#[test]
fn build_agency_selection_artifact_is_deterministic_and_emits_multiple_candidates() {
    let mut summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "agency-selection-run".to_string(),
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
        run_id: "agency-selection-run".to_string(),
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
    let success_signals = run_artifacts::build_cognitive_signals_artifact(
        &summary,
        &success_suggestions,
        Some(&success_scores),
    );
    let success_affect = run_artifacts::build_affect_state_artifact(
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
    let success_path = run_artifacts::build_fast_slow_path_artifact(
        &summary,
        &success_arbitration,
        &success_state,
        Some(&success_scores),
    );
    let success_agency_state = run_artifacts::build_agency_selection_state(
        &success_signals,
        &success_arbitration,
        &success_state,
        &success_path,
    );
    let fast_left = run_artifacts::build_agency_selection_artifact(
        &summary,
        &success_arbitration,
        &success_agency_state,
        Some(&success_scores),
    );
    let fast_right = run_artifacts::build_agency_selection_artifact(
        &summary,
        &success_arbitration,
        &success_agency_state,
        Some(&success_scores),
    );
    assert_eq!(
        serde_json::to_value(&fast_left).expect("fast left value"),
        serde_json::to_value(&fast_right).expect("fast right value")
    );
    assert_eq!(fast_left.agency_selection_version, 1);
    assert_eq!(fast_left.candidate_set.len(), 2);
    assert_eq!(fast_left.selected_candidate_id, "cand-fast-execute");
    assert_eq!(
        success_agency_state.selected_candidate_id,
        "cand-fast-execute"
    );
    assert_eq!(
        success_agency_state.selected_candidate_kind,
        "direct_execution"
    );
    assert!(fast_left
        .candidate_generation_basis
        .contains("runtime_branch=fast_direct_execution_branch"));

    summary.status = "failure".to_string();
    summary.counts.failed_steps = 1;
    let failure_scores = ScoresArtifact {
        scores_version: 1,
        run_id: "agency-selection-run".to_string(),
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
    let failure_signals = run_artifacts::build_cognitive_signals_artifact(
        &summary,
        &failure_suggestions,
        Some(&failure_scores),
    );
    let failure_affect = run_artifacts::build_affect_state_artifact(
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
    let failure_path = run_artifacts::build_fast_slow_path_artifact(
        &summary,
        &failure_arbitration,
        &failure_state,
        Some(&failure_scores),
    );
    let failure_agency_state = run_artifacts::build_agency_selection_state(
        &failure_signals,
        &failure_arbitration,
        &failure_state,
        &failure_path,
    );
    let slow = run_artifacts::build_agency_selection_artifact(
        &summary,
        &failure_arbitration,
        &failure_agency_state,
        Some(&failure_scores),
    );
    assert_eq!(slow.selected_candidate_id, "cand-slow-review");
    assert!(slow.candidate_set.len() >= 3);
    assert_eq!(
        failure_agency_state.selected_candidate_kind,
        "review_and_refine"
    );
    assert_ne!(fast_left.selection_mode, slow.selection_mode);
    assert_ne!(
        fast_left.selected_candidate_reason,
        slow.selected_candidate_reason
    );
}

#[test]
fn build_bounded_execution_artifact_is_deterministic_and_shows_iteration_shape() {
    let mut summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "bounded-execution-run".to_string(),
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
        run_id: "bounded-execution-run".to_string(),
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
    let success_signals = run_artifacts::build_cognitive_signals_artifact(
        &summary,
        &success_suggestions,
        Some(&success_scores),
    );
    let success_affect = run_artifacts::build_affect_state_artifact(
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
    let success_path = run_artifacts::build_fast_slow_path_artifact(
        &summary,
        &success_arbitration,
        &success_state,
        Some(&success_scores),
    );
    let success_agency_state = run_artifacts::build_agency_selection_state(
        &success_signals,
        &success_arbitration,
        &success_state,
        &success_path,
    );
    let success_agency = run_artifacts::build_agency_selection_artifact(
        &summary,
        &success_arbitration,
        &success_agency_state,
        Some(&success_scores),
    );
    let success_state = run_artifacts::build_bounded_execution_state(
        &summary,
        &success_path,
        &success_agency,
        &success_agency_state,
    );
    let fast_left = run_artifacts::build_bounded_execution_artifact(
        &summary,
        &success_path,
        &success_agency,
        &success_state,
        Some(&success_scores),
    );
    let fast_right = run_artifacts::build_bounded_execution_artifact(
        &summary,
        &success_path,
        &success_agency,
        &success_state,
        Some(&success_scores),
    );
    assert_eq!(
        serde_json::to_value(&fast_left).expect("fast left value"),
        serde_json::to_value(&fast_right).expect("fast right value")
    );
    assert_eq!(fast_left.iteration_count, 1);
    assert_eq!(
        fast_left.provisional_termination_state,
        "ready_for_evaluation"
    );
    assert_eq!(success_state.continuation_state, "stop_after_one");

    summary.status = "failure".to_string();
    summary.counts.failed_steps = 1;
    let failure_scores = ScoresArtifact {
        scores_version: 1,
        run_id: "bounded-execution-run".to_string(),
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
    let failure_signals = run_artifacts::build_cognitive_signals_artifact(
        &summary,
        &failure_suggestions,
        Some(&failure_scores),
    );
    let failure_affect = run_artifacts::build_affect_state_artifact(
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
    let failure_path = run_artifacts::build_fast_slow_path_artifact(
        &summary,
        &failure_arbitration,
        &failure_state,
        Some(&failure_scores),
    );
    let failure_agency_state = run_artifacts::build_agency_selection_state(
        &failure_signals,
        &failure_arbitration,
        &failure_state,
        &failure_path,
    );
    let failure_agency = run_artifacts::build_agency_selection_artifact(
        &summary,
        &failure_arbitration,
        &failure_agency_state,
        Some(&failure_scores),
    );
    let failure_state = run_artifacts::build_bounded_execution_state(
        &summary,
        &failure_path,
        &failure_agency,
        &failure_agency_state,
    );
    let slow = run_artifacts::build_bounded_execution_artifact(
        &summary,
        &failure_path,
        &failure_agency,
        &failure_state,
        Some(&failure_scores),
    );
    assert_eq!(slow.bounded_execution_version, 1);
    assert_eq!(slow.iteration_count, 2);
    assert_eq!(slow.iterations[0].stage, "review");
    assert_eq!(
        slow.provisional_termination_state,
        "ready_for_runtime_evaluation"
    );
    assert_ne!(fast_left.iteration_count, slow.iteration_count);
}

#[test]
fn build_evaluation_signals_artifact_is_deterministic_and_emits_termination_reasons() {
    let mut summary = RunSummaryArtifact {
        run_summary_version: 1,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: "evaluation-signals-run".to_string(),
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
        run_id: "evaluation-signals-run".to_string(),
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
    let success_signals = run_artifacts::build_cognitive_signals_artifact(
        &summary,
        &success_suggestions,
        Some(&success_scores),
    );
    let success_affect = run_artifacts::build_affect_state_artifact(
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
    let success_path = run_artifacts::build_fast_slow_path_artifact(
        &summary,
        &success_arbitration,
        &success_state,
        Some(&success_scores),
    );
    let success_agency_state = run_artifacts::build_agency_selection_state(
        &success_signals,
        &success_arbitration,
        &success_state,
        &success_path,
    );
    let success_agency = run_artifacts::build_agency_selection_artifact(
        &summary,
        &success_arbitration,
        &success_agency_state,
        Some(&success_scores),
    );
    let success_execution_state = run_artifacts::build_bounded_execution_state(
        &summary,
        &success_path,
        &success_agency,
        &success_agency_state,
    );
    let success_execution = run_artifacts::build_bounded_execution_artifact(
        &summary,
        &success_path,
        &success_agency,
        &success_execution_state,
        Some(&success_scores),
    );
    let success_eval_state =
        run_artifacts::build_evaluation_control_state(&summary, &success_execution);
    let success_left = run_artifacts::build_evaluation_signals_artifact(
        &summary,
        &success_path,
        &success_agency,
        &success_eval_state,
        Some(&success_scores),
    );
    let success_right = run_artifacts::build_evaluation_signals_artifact(
        &summary,
        &success_path,
        &success_agency,
        &success_eval_state,
        Some(&success_scores),
    );
    assert_eq!(
        serde_json::to_value(&success_left).expect("success left value"),
        serde_json::to_value(&success_right).expect("success right value")
    );
    assert_eq!(success_left.termination_reason, "success");
    assert_eq!(success_left.failure_signal, "none");
    assert_eq!(success_left.next_control_action, "complete_run");

    summary.status = "failure".to_string();
    summary.counts.failed_steps = 1;
    let failure_scores = ScoresArtifact {
        scores_version: 1,
        run_id: "evaluation-signals-run".to_string(),
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
    let failure_signals = run_artifacts::build_cognitive_signals_artifact(
        &summary,
        &failure_suggestions,
        Some(&failure_scores),
    );
    let failure_affect = run_artifacts::build_affect_state_artifact(
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
    let failure_path = run_artifacts::build_fast_slow_path_artifact(
        &summary,
        &failure_arbitration,
        &failure_state,
        Some(&failure_scores),
    );
    let failure_agency_state = run_artifacts::build_agency_selection_state(
        &failure_signals,
        &failure_arbitration,
        &failure_state,
        &failure_path,
    );
    let failure_agency = run_artifacts::build_agency_selection_artifact(
        &summary,
        &failure_arbitration,
        &failure_agency_state,
        Some(&failure_scores),
    );
    let failure_execution_state = run_artifacts::build_bounded_execution_state(
        &summary,
        &failure_path,
        &failure_agency,
        &failure_agency_state,
    );
    let failure_execution = run_artifacts::build_bounded_execution_artifact(
        &summary,
        &failure_path,
        &failure_agency,
        &failure_execution_state,
        Some(&failure_scores),
    );
    let failure_eval_state =
        run_artifacts::build_evaluation_control_state(&summary, &failure_execution);
    let failure_eval = run_artifacts::build_evaluation_signals_artifact(
        &summary,
        &failure_path,
        &failure_agency,
        &failure_eval_state,
        Some(&failure_scores),
    );
    assert_eq!(failure_eval.evaluation_signals_version, 1);
    assert_eq!(failure_eval.termination_reason, "bounded_failure");
    assert_eq!(failure_eval.contradiction_signal, "present");
    assert_eq!(failure_eval.next_control_action, "handoff_to_reframing");
    assert_ne!(
        success_left.termination_reason,
        failure_eval.termination_reason
    );
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
            status: "success".to_string(),
            output_artifact_path: Some("out/s1.txt".to_string()),
        },
        StepStateArtifact {
            step_id: "s2".to_string(),
            agent_id: "a1".to_string(),
            provider_id: "p1".to_string(),
            status: "failure".to_string(),
            output_artifact_path: None,
        },
        StepStateArtifact {
            step_id: "s3".to_string(),
            agent_id: "a1".to_string(),
            provider_id: "p1".to_string(),
            status: "not_run".to_string(),
            output_artifact_path: None,
        },
    ];
    let resume_completed = BTreeSet::from(["s0".to_string()]);
    let status = build_run_status(&resolved, &tr, "failed", &steps, None, &resume_completed);

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
    assert!(
        status.effective_max_concurrency.is_none() || status.effective_max_concurrency == Some(4)
    );
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
