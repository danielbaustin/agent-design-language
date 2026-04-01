use super::*;

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
