use super::*;

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
