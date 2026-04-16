use super::*;

pub(crate) fn build_run_summary(
    resolved: &resolve::AdlResolved,
    status: &str,
    pause: Option<&execute::PauseState>,
    steps: &[StepStateArtifact],
    records: usize,
    failure: Option<&anyhow::Error>,
    run_paths: &artifacts::RunArtifactPaths,
) -> RunSummaryArtifact {
    let failed_steps = steps.iter().filter(|s| s.status == "failure").count();
    let completed_steps = steps
        .iter()
        .filter(|s| s.status == "success" || s.status == "failure")
        .count();
    let delegation_steps = resolved
        .steps
        .iter()
        .filter(|s| {
            s.delegation
                .as_ref()
                .map(|d| !d.is_effectively_empty())
                .unwrap_or(false)
        })
        .count();
    let delegation_requires_verification_steps = resolved
        .steps
        .iter()
        .filter(|s| {
            s.delegation
                .as_ref()
                .and_then(|d| d.requires_verification)
                .unwrap_or(false)
        })
        .count();
    let mut security_denials_by_code = BTreeMap::new();
    if let Some(code) = failure.and_then(classify_failure_kind) {
        *security_denials_by_code
            .entry(code.to_string())
            .or_insert(0) += 1;
    }

    let (
        security_envelope_enabled,
        signing_required,
        key_id_required,
        mut allowed_algs,
        mut allowed_key_sources,
    ) = if let Some(remote) = resolved.doc.run.remote.as_ref() {
        (
            true,
            remote.require_signed_requests,
            remote.require_key_id,
            remote.verify_allowed_algs.clone(),
            remote.verify_allowed_key_sources.clone(),
        )
    } else {
        (false, false, false, Vec::new(), Vec::new())
    };
    allowed_algs.sort();
    allowed_algs.dedup();
    allowed_key_sources.sort();
    allowed_key_sources.dedup();
    let scores_rel = run_paths
        .scores_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/scores.json".to_string());
    let suggestions_rel = run_paths
        .suggestions_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/suggestions.json".to_string());
    let aee_decision_rel = run_paths
        .aee_decision_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/aee_decision.json".to_string());
    let cognitive_signals_rel = run_paths
        .cognitive_signals_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/cognitive_signals.v1.json".to_string());
    let fast_slow_path_rel = run_paths
        .fast_slow_path_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/fast_slow_path.v1.json".to_string());
    let agency_selection_rel = run_paths
        .agency_selection_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/agency_selection.v1.json".to_string());
    let bounded_execution_rel = run_paths
        .bounded_execution_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/bounded_execution.v1.json".to_string());
    let evaluation_signals_rel = run_paths
        .evaluation_signals_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/evaluation_signals.v1.json".to_string());
    let cognitive_arbitration_rel = run_paths
        .cognitive_arbitration_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/cognitive_arbitration.v1.json".to_string());
    let affect_state_rel = run_paths
        .affect_state_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/affect_state.v1.json".to_string());
    let reasoning_graph_rel = run_paths
        .reasoning_graph_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "learning/reasoning_graph.v1.json".to_string());
    let cluster_groundwork_rel = run_paths
        .cluster_groundwork_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "meta/cluster_groundwork.json".to_string());
    let trace_rel = run_paths
        .trace_v1_json()
        .strip_prefix(run_paths.run_dir())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "logs/trace_v1.json".to_string());

    RunSummaryArtifact {
        run_summary_version: RUN_SUMMARY_VERSION,
        artifact_model_version: artifacts::ARTIFACT_MODEL_VERSION,
        run_id: resolved.run_id.clone(),
        workflow_id: resolved.workflow_id.clone(),
        adl_version: resolved.doc.version.clone(),
        swarm_version: env!("CARGO_PKG_VERSION").to_string(),
        status: status.to_string(),
        error_kind: failure.and_then(classify_failure_kind).map(str::to_string),
        counts: RunSummaryCounts {
            total_steps: resolved.steps.len(),
            completed_steps,
            failed_steps,
            provider_call_count: records,
            delegation_steps,
            delegation_requires_verification_steps,
        },
        policy: RunSummaryPolicy {
            security_envelope_enabled,
            signing_required,
            key_id_required,
            verify_allowed_algs: allowed_algs,
            verify_allowed_key_sources: allowed_key_sources,
            sandbox_policy: "centralized_path_resolver_v1".to_string(),
            security_denials_by_code,
        },
        links: RunSummaryLinks {
            run_json: "run.json".to_string(),
            steps_json: "steps.json".to_string(),
            pause_state_json: pause.map(|_| "pause_state.json".to_string()),
            outputs_dir: run_paths
                .outputs_dir()
                .strip_prefix(run_paths.run_dir())
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "outputs".to_string()),
            logs_dir: run_paths
                .logs_dir()
                .strip_prefix(run_paths.run_dir())
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "logs".to_string()),
            learning_dir: run_paths
                .learning_dir()
                .strip_prefix(run_paths.run_dir())
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "learning".to_string()),
            scores_json: Some(scores_rel),
            suggestions_json: Some(suggestions_rel),
            aee_decision_json: Some(aee_decision_rel),
            cognitive_signals_json: Some(cognitive_signals_rel),
            fast_slow_path_json: Some(fast_slow_path_rel),
            agency_selection_json: Some(agency_selection_rel),
            bounded_execution_json: Some(bounded_execution_rel),
            evaluation_signals_json: Some(evaluation_signals_rel),
            cognitive_arbitration_json: Some(cognitive_arbitration_rel),
            affect_state_json: Some(affect_state_rel),
            reasoning_graph_json: Some(reasoning_graph_rel),
            overlays_dir: run_paths
                .overlays_dir()
                .strip_prefix(run_paths.run_dir())
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "learning/overlays".to_string()),
            cluster_groundwork_json: run_paths
                .cluster_groundwork_json()
                .is_file()
                .then_some(cluster_groundwork_rel),
            trace_json: Some(trace_rel),
        },
    }
}

pub(crate) fn build_cluster_groundwork_artifact(
    resolved: &resolve::AdlResolved,
    steps: &[StepStateArtifact],
    tr: &trace::Trace,
) -> ClusterGroundworkArtifact {
    let mut remaining_deps: BTreeMap<String, BTreeSet<String>> = resolved
        .execution_plan
        .nodes
        .iter()
        .map(|node| {
            (
                node.step_id.clone(),
                node.depends_on.iter().cloned().collect::<BTreeSet<_>>(),
            )
        })
        .collect();
    let mut remaining_nodes: BTreeSet<String> =
        remaining_deps.keys().cloned().collect::<BTreeSet<_>>();
    let mut readiness_frontiers = Vec::new();
    while !remaining_nodes.is_empty() {
        let ready = remaining_nodes
            .iter()
            .filter(|step_id| {
                remaining_deps
                    .get(step_id.as_str())
                    .map(|deps| deps.is_empty())
                    .unwrap_or(false)
            })
            .cloned()
            .collect::<Vec<_>>();
        if ready.is_empty() {
            break;
        }
        readiness_frontiers.push(ClusterReadyFrontier {
            frontier_index: readiness_frontiers.len() as u32,
            ready_step_ids: ready.clone(),
        });
        for step_id in &ready {
            remaining_nodes.remove(step_id);
        }
        for deps in remaining_deps.values_mut() {
            for step_id in &ready {
                deps.remove(step_id);
            }
        }
    }

    let mut attempts_by_step: BTreeMap<String, u32> = BTreeMap::new();
    for event in &tr.events {
        if let trace::TraceEvent::StepStarted { step_id, .. } = event {
            *attempts_by_step.entry(step_id.clone()).or_insert(0) += 1;
        }
    }
    let status_by_step = steps
        .iter()
        .map(|step| (step.step_id.clone(), step.status.clone()))
        .collect::<BTreeMap<_, _>>();
    let depends_on_by_step = resolved
        .execution_plan
        .nodes
        .iter()
        .map(|node| (node.step_id.clone(), node.depends_on.clone()))
        .collect::<BTreeMap<_, _>>();

    let mut lease_records = Vec::new();
    let mut issued_sequence: u32 = 0;
    for frontier in &readiness_frontiers {
        for step_id in &frontier.ready_step_ids {
            issued_sequence = issued_sequence.saturating_add(1);
            let status = status_by_step
                .get(step_id)
                .map(|value| value.as_str())
                .unwrap_or("not_run");
            let lease_state = match status {
                "success" => "completed",
                "failure" => "failed",
                _ => "planned",
            };
            lease_records.push(ClusterLeaseRecord {
                issued_sequence,
                lease_id: format!("lease:{}:{}:1", resolved.run_id, step_id),
                step_id: step_id.clone(),
                depends_on: depends_on_by_step.get(step_id).cloned().unwrap_or_default(),
                observed_attempts: attempts_by_step.get(step_id).copied().unwrap_or(0),
                claim_owner: "adl-coordinator-local".to_string(),
                worker_id: "adl-worker-local".to_string(),
                lease_state: lease_state.to_string(),
            });
        }
    }

    ClusterGroundworkArtifact {
        cluster_groundwork_version: CLUSTER_GROUNDWORK_VERSION,
        run_id: resolved.run_id.clone(),
        workflow_id: resolved.workflow_id.clone(),
        coordinator_id: "adl-coordinator-local".to_string(),
        worker_id: "adl-worker-local".to_string(),
        canonical_ordering_key: "(run_id, step_id, attempt)".to_string(),
        frontier_ordering: "topological_frontier_then_step_id".to_string(),
        readiness_frontiers,
        lease_records,
    }
}

pub(crate) fn build_run_status(
    resolved: &resolve::AdlResolved,
    tr: &trace::Trace,
    overall_status: &str,
    steps: &[StepStateArtifact],
    failure: Option<&anyhow::Error>,
    pause: Option<&execute::PauseState>,
    resume_completed_step_ids: &BTreeSet<String>,
) -> RunStatusArtifact {
    let mut completed_steps: BTreeSet<String> = resume_completed_step_ids.clone();
    let mut pending_steps: BTreeSet<String> = BTreeSet::new();
    let mut failed_step_id: Option<String> = None;

    for step in steps {
        match step.status.as_str() {
            "success" => {
                completed_steps.insert(step.step_id.clone());
            }
            "failure" => {
                if failed_step_id.is_none() {
                    failed_step_id = Some(step.step_id.clone());
                }
                pending_steps.insert(step.step_id.clone());
            }
            _ => {
                pending_steps.insert(step.step_id.clone());
            }
        }
    }

    let mut attempts_by_step: BTreeMap<String, u32> = resume_completed_step_ids
        .iter()
        .map(|step_id| (step_id.clone(), 0))
        .collect();
    let mut started_set = BTreeSet::new();
    for event in &tr.events {
        if let trace::TraceEvent::StepStarted { step_id, .. } = event {
            started_set.insert(step_id.clone());
            *attempts_by_step.entry(step_id.clone()).or_insert(0) += 1;
        }
    }

    let scheduler_policy = execute::scheduler_policy_for_run(resolved).ok().flatten();
    let failure_kind = failure.and_then(classify_failure_kind);
    let (resilience_classification, continuity_status, preservation_status, shepherd_decision) =
        derive_resilience_status(overall_status, failure_kind, pause);
    let (persistence_mode, cleanup_disposition, resume_guard, state_artifacts) =
        derive_persistence_discipline(overall_status, pause);

    RunStatusArtifact {
        run_status_version: RUN_STATUS_VERSION,
        run_id: resolved.run_id.clone(),
        workflow_id: resolved.workflow_id.clone(),
        overall_status: overall_status.to_string(),
        failure_kind: failure_kind.map(str::to_string),
        failed_step_id,
        completed_steps: completed_steps.into_iter().collect(),
        pending_steps: pending_steps.into_iter().collect(),
        started_steps: if started_set.is_empty() {
            None
        } else {
            Some(started_set.into_iter().collect())
        },
        resilience_classification: Some(resilience_classification.to_string()),
        continuity_status: Some(continuity_status.to_string()),
        preservation_status: Some(preservation_status.to_string()),
        shepherd_decision: Some(shepherd_decision.to_string()),
        persistence_mode: persistence_mode.to_string(),
        cleanup_disposition: cleanup_disposition.to_string(),
        resume_guard: resume_guard.to_string(),
        state_artifacts,
        attempt_counts_by_step: attempts_by_step,
        effective_max_concurrency: scheduler_policy.map(|(value, _)| value),
        effective_max_concurrency_source: scheduler_policy
            .map(|(_, source)| source.as_str().to_string()),
    }
}

fn derive_persistence_discipline(
    overall_status: &str,
    pause: Option<&execute::PauseState>,
) -> (&'static str, &'static str, &'static str, Vec<String>) {
    let mut state_artifacts = vec![
        "run.json".to_string(),
        "steps.json".to_string(),
        "run_status.json".to_string(),
        "logs/trace_v1.json".to_string(),
    ];

    if pause.is_some() {
        state_artifacts.push("pause_state.json".to_string());
        return (
            "checkpoint_resume_state",
            "retain_pause_state",
            "execution_plan_hash_match_required",
            state_artifacts,
        );
    }

    if overall_status == "failed" {
        return (
            "review_preserved_state",
            "retain_for_review",
            "resume_not_permitted",
            state_artifacts,
        );
    }

    (
        "completed_run_record",
        "no_resume_state_retained",
        "not_applicable",
        state_artifacts,
    )
}

fn derive_resilience_status(
    overall_status: &str,
    failure_kind: Option<&str>,
    pause: Option<&execute::PauseState>,
) -> (&'static str, &'static str, &'static str, &'static str) {
    if pause.is_some() {
        return (
            "interruption",
            "resume_ready",
            "pause_state_preserved",
            "preserve_and_resume",
        );
    }

    if overall_status == "failed" {
        if matches!(failure_kind, Some("replay_invariant_violation")) {
            return (
                "corruption",
                "continuity_refused",
                "inspection_only",
                "refuse_resume",
            );
        }
        return (
            "crash",
            "continuity_unverified",
            "preserved_for_review",
            "operator_review_required",
        );
    }

    (
        "not_applicable",
        "continuous",
        "no_preservation_needed",
        "none",
    )
}

pub(crate) fn compute_retry_count(tr: &trace::Trace) -> usize {
    let mut started_by_step: BTreeMap<&str, usize> = BTreeMap::new();
    for event in &tr.events {
        if let trace::TraceEvent::StepStarted { step_id, .. } = event {
            *started_by_step.entry(step_id.as_str()).or_insert(0) += 1;
        }
    }
    started_by_step
        .values()
        .map(|count| count.saturating_sub(1))
        .sum()
}

pub(crate) fn compute_max_parallel_observed(tr: &trace::Trace) -> usize {
    let mut active: BTreeSet<&str> = BTreeSet::new();
    let mut max_parallel = 0usize;
    for event in &tr.events {
        match event {
            trace::TraceEvent::StepStarted { step_id, .. } => {
                active.insert(step_id.as_str());
                max_parallel = max_parallel.max(active.len());
            }
            trace::TraceEvent::StepFinished { step_id, .. } => {
                active.remove(step_id.as_str());
            }
            _ => {}
        }
    }
    max_parallel
}

pub(crate) fn build_scores_artifact(
    run_summary: &RunSummaryArtifact,
    tr: &trace::Trace,
) -> ScoresArtifact {
    let success_steps = run_summary
        .counts
        .completed_steps
        .saturating_sub(run_summary.counts.failed_steps);
    let success_ratio = success_steps
        .saturating_mul(1000)
        .checked_div(run_summary.counts.total_steps)
        .map(|permille| (permille as f64) / 1000.0)
        .unwrap_or(1.0);
    let security_denied_count: usize = run_summary.policy.security_denials_by_code.values().sum();
    let delegation_denied_count: usize = run_summary
        .policy
        .security_denials_by_code
        .iter()
        .filter_map(|(code, count)| {
            if code.starts_with("DELEGATION_") {
                Some(*count)
            } else {
                None
            }
        })
        .sum();

    ScoresArtifact {
        scores_version: SCORES_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: ScoresGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
        },
        summary: ScoresSummary {
            success_ratio,
            failure_count: run_summary.counts.failed_steps,
            retry_count: compute_retry_count(tr),
            delegation_denied_count,
            security_denied_count,
        },
        metrics: ScoresMetrics {
            scheduler_max_parallel_observed: compute_max_parallel_observed(tr),
        },
    }
}

pub(crate) fn read_scores_if_present(
    run_paths: &artifacts::RunArtifactPaths,
) -> Option<ScoresArtifact> {
    let path = run_paths.scores_json();
    let raw = std::fs::read_to_string(path).ok()?;
    serde_json::from_str::<ScoresArtifact>(&raw).ok()
}

pub(crate) fn build_suggestions_artifact(
    run_summary: &RunSummaryArtifact,
    scores: Option<&ScoresArtifact>,
) -> SuggestionsArtifact {
    let fallback_summary;
    let fallback_metrics;
    let (score_summary, score_metrics, score_version) = if let Some(scores) = scores {
        (
            &scores.summary,
            &scores.metrics,
            Some(scores.scores_version),
        )
    } else {
        let failed_steps = run_summary.counts.failed_steps;
        let success_steps = run_summary
            .counts
            .completed_steps
            .saturating_sub(failed_steps);
        let success_ratio = success_steps
            .saturating_mul(1000)
            .checked_div(run_summary.counts.total_steps)
            .map(|permille| (permille as f64) / 1000.0)
            .unwrap_or(1.0);
        let security_denied_count: usize =
            run_summary.policy.security_denials_by_code.values().sum();
        let delegation_denied_count: usize = run_summary
            .policy
            .security_denials_by_code
            .iter()
            .filter_map(|(code, count)| {
                if code.starts_with("DELEGATION_") {
                    Some(*count)
                } else {
                    None
                }
            })
            .sum();
        fallback_summary = ScoresSummary {
            success_ratio,
            failure_count: failed_steps,
            retry_count: 0,
            delegation_denied_count,
            security_denied_count,
        };
        fallback_metrics = ScoresMetrics {
            scheduler_max_parallel_observed: 1,
        };
        (&fallback_summary, &fallback_metrics, None)
    };

    let base_evidence = SuggestionEvidence {
        failure_count: score_summary.failure_count,
        retry_count: score_summary.retry_count,
        delegation_denied_count: score_summary.delegation_denied_count,
        security_denied_count: score_summary.security_denied_count,
        success_ratio: score_summary.success_ratio,
        scheduler_max_parallel_observed: score_metrics.scheduler_max_parallel_observed,
    };

    let mut suggestions = Vec::new();

    if score_summary.failure_count > 0 {
        suggestions.push(SuggestionItem {
            id: String::new(),
            category: "retry".to_string(),
            severity: "improvement".to_string(),
            rationale: "One or more steps failed; consider safer retry policy for transient paths."
                .to_string(),
            evidence: base_evidence.clone(),
            proposed_change: SuggestedChangeIntent {
                intent: "increase_step_retry_budget".to_string(),
                target: "failed-step-set".to_string(),
            },
        });
    }
    if score_summary.delegation_denied_count > 0 {
        suggestions.push(SuggestionItem {
            id: String::new(),
            category: "delegation".to_string(),
            severity: "warning".to_string(),
            rationale: "Delegation-denied signals detected; review delegation policy scope."
                .to_string(),
            evidence: base_evidence.clone(),
            proposed_change: SuggestedChangeIntent {
                intent: "review_delegation_policy_scope".to_string(),
                target: "delegation-boundary".to_string(),
            },
        });
    }
    if score_summary.security_denied_count > 0 {
        suggestions.push(SuggestionItem {
            id: String::new(),
            category: "security".to_string(),
            severity: "warning".to_string(),
            rationale: "Security denials observed; align expected capabilities with trust policy."
                .to_string(),
            evidence: base_evidence.clone(),
            proposed_change: SuggestedChangeIntent {
                intent: "review_security_policy_expectations".to_string(),
                target: "security-envelope".to_string(),
            },
        });
    }
    if score_summary.success_ratio < 1.0 {
        suggestions.push(SuggestionItem {
            id: String::new(),
            category: "general".to_string(),
            severity: "improvement".to_string(),
            rationale: "Success ratio is below 1.0; review failing steps and dependency shape."
                .to_string(),
            evidence: base_evidence.clone(),
            proposed_change: SuggestedChangeIntent {
                intent: "review_failure_hotspots".to_string(),
                target: "workflow-step-dependencies".to_string(),
            },
        });
    }
    if run_summary.counts.total_steps > 1 && score_metrics.scheduler_max_parallel_observed <= 1 {
        suggestions.push(SuggestionItem {
            id: String::new(),
            category: "scheduler".to_string(),
            severity: "info".to_string(),
            rationale: "Observed parallelism is low; evaluate opportunities for safe concurrency."
                .to_string(),
            evidence: base_evidence,
            proposed_change: SuggestedChangeIntent {
                intent: "evaluate_parallelizable_dependencies".to_string(),
                target: "workflow-structure".to_string(),
            },
        });
    }

    for (idx, suggestion) in suggestions.iter_mut().enumerate() {
        suggestion.id = format!("sug-{:03}", idx + 1);
    }

    SuggestionsArtifact {
        suggestions_version: SUGGESTIONS_VERSION,
        run_id: run_summary.run_id.clone(),
        generated_from: SuggestionsGeneratedFrom {
            artifact_model_version: run_summary.artifact_model_version,
            run_summary_version: run_summary.run_summary_version,
            scores_version: score_version,
        },
        suggestions,
    }
}
