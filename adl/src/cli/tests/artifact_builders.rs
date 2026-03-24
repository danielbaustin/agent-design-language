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
