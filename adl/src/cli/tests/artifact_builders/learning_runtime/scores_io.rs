use super::*;

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
