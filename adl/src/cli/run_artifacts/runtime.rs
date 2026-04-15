use super::cognitive::{
    build_aee_decision_artifact, build_affect_state_artifact, build_agency_selection_artifact,
    build_bounded_execution_artifact, build_cognitive_arbitration_artifact_from_state,
    build_cognitive_signals_artifact_from_state, build_control_path_action_mediation_artifact,
    build_control_path_action_proposals_artifact, build_control_path_decisions_artifact,
    build_control_path_final_result_artifact, build_control_path_memory_artifact,
    build_control_path_security_review_artifact,
    build_control_path_skill_execution_protocol_artifact, build_control_path_skill_model_artifact,
    build_control_path_summary, build_evaluation_signals_artifact, build_fast_slow_path_artifact,
    build_freedom_gate_artifact, build_memory_read_artifact, build_memory_write_artifact,
    build_reasoning_graph_artifact, build_reframing_artifact,
    control_path_security_attacker_pressure, control_path_security_boundaries,
    control_path_security_posture, control_path_security_reduced_trust_surfaces,
    control_path_security_required_mitigations, control_path_security_review_surfaces,
    control_path_security_threat_classes, control_path_security_trust_state,
};
use super::summary::{
    build_cluster_groundwork_artifact, build_scores_artifact, build_suggestions_artifact,
};
use super::ControlPathSummaryContext;
use super::*;
use ::adl::runtime_environment::RuntimeEnvironment;
use ::adl::trace_schema_v1::{
    validate_trace_event_envelope_v1, ContractValidationResultV1, TraceActorTypeV1, TraceActorV1,
    TraceContractValidationV1, TraceDecisionContextV1, TraceErrorV1, TraceEventEnvelopeV1,
    TraceEventTypeV1, TraceEventV1, TraceScopeLevelV1, TraceScopeV1,
};
use serde::Serialize;
use serde_json::json;

#[allow(clippy::too_many_arguments)]
pub(crate) fn write_run_state_artifacts(
    resolved: &resolve::AdlResolved,
    tr: &trace::Trace,
    adl_path: &Path,
    _out_dir: &Path,
    start_ms: u128,
    end_ms: u128,
    status: &str,
    pause: Option<&execute::PauseState>,
    steering_history: &[execute::SteeringRecord],
    runtime_control: &execute::RuntimeControlState,
    resume_completed_step_ids: Option<&HashSet<String>>,
    failure: Option<&anyhow::Error>,
) -> Result<PathBuf> {
    let run_paths = artifacts::RunArtifactPaths::for_run(&resolved.run_id)?;
    run_paths.ensure_layout()?;
    run_paths.write_model_marker()?;
    let run_dir = run_paths.run_dir();
    let resume_completed: BTreeSet<String> = resume_completed_step_ids
        .map(|ids| ids.iter().cloned().collect())
        .unwrap_or_default();

    let mut status_by_step: HashMap<String, String> = HashMap::new();
    for ev in &tr.events {
        if let trace::TraceEvent::StepFinished {
            step_id, success, ..
        } = ev
        {
            let status = if *success { "success" } else { "failure" };
            status_by_step.insert(step_id.clone(), status.to_string());
        }
    }

    let mut steps = Vec::with_capacity(resolved.steps.len());
    for step in &resolved.steps {
        let status = status_by_step
            .get(&step.id)
            .cloned()
            .or_else(|| {
                resume_completed
                    .contains(&step.id)
                    .then(|| "success".to_string())
            })
            .unwrap_or_else(|| "not_run".to_string());
        let output_artifact_path = match (status.as_str(), step.write_to.as_deref()) {
            ("success", Some(write_to)) => Some(write_to.to_string()),
            _ => None,
        };

        let agent_id = step
            .agent
            .as_deref()
            .unwrap_or("<unresolved-agent>")
            .to_string();
        let provider_id = step
            .provider
            .as_deref()
            .unwrap_or("<unresolved-provider>")
            .to_string();

        steps.push(StepStateArtifact {
            step_id: step.id.clone(),
            agent_id,
            provider_id,
            conversation: step
                .conversation
                .as_ref()
                .map(|turn| ConversationTurnArtifact {
                    id: turn.id.clone(),
                    speaker: turn.speaker.clone(),
                    sequence: turn.sequence,
                    thread_id: turn.thread_id.clone(),
                    responds_to: turn.responds_to.clone(),
                }),
            status,
            output_artifact_path,
        });
    }

    let scheduler_policy = execute::scheduler_policy_for_run(resolved)?;
    let error_message = tr.events.iter().rev().find_map(|ev| match ev {
        trace::TraceEvent::RunFailed { message, .. } => Some(message.clone()),
        _ => None,
    });
    let run_artifact = RunStateArtifact {
        schema_version: RUN_STATE_SCHEMA_VERSION.to_string(),
        run_id: resolved.run_id.clone(),
        workflow_id: resolved.workflow_id.clone(),
        version: resolved.doc.version.clone(),
        status: status.to_string(),
        error_message: error_message.clone(),
        start_time_ms: start_ms,
        end_time_ms: end_ms,
        duration_ms: end_ms.saturating_sub(start_ms),
        execution_plan_hash: execution_plan_hash(&resolved.execution_plan)?,
        scheduler_max_concurrency: scheduler_policy.map(|(v, _)| v),
        scheduler_policy_source: scheduler_policy.map(|(_, source)| source.as_str().to_string()),
        steering_history: steering_history.to_vec(),
        pause: pause.cloned(),
    };

    let run_json = serde_json::to_vec_pretty(&run_artifact).context("serialize run.json")?;
    let steps_json = serde_json::to_vec_pretty(&steps).context("serialize steps.json")?;
    let activation_log_path = run_paths.activation_log_json();
    instrumentation::write_trace_artifact(&activation_log_path, &tr.events)?;
    let trace_v1 =
        build_trace_v1_envelope(resolved, tr, &steps, start_ms, end_ms, status, failure)?;
    let trace_v1_json = serde_json::to_vec_pretty(&trace_v1).context("serialize trace_v1.json")?;
    let cluster_groundwork = build_cluster_groundwork_artifact(resolved, &steps, tr);
    let cluster_groundwork_json = serde_json::to_vec_pretty(&cluster_groundwork)
        .context("serialize cluster_groundwork.json")?;
    artifacts::atomic_write(
        &run_paths.cluster_groundwork_json(),
        &cluster_groundwork_json,
    )?;
    let run_summary = build_run_summary(
        resolved,
        status,
        pause,
        &steps,
        tr.events
            .iter()
            .filter(|ev| matches!(ev, trace::TraceEvent::StepFinished { .. }))
            .count(),
        failure,
        &run_paths,
    );
    let overall_status = match status {
        "success" => "succeeded",
        "failure" => "failed",
        "paused" => "running",
        other => other,
    };
    let run_status = build_run_status(
        resolved,
        tr,
        overall_status,
        &steps,
        failure,
        pause,
        &resume_completed,
    );
    let run_summary_json =
        serde_json::to_vec_pretty(&run_summary).context("serialize run_summary.json")?;
    let run_status_json =
        serde_json::to_vec_pretty(&run_status).context("serialize run_status.json")?;
    let scores = build_scores_artifact(&run_summary, tr);
    let scores_json = serde_json::to_vec_pretty(&scores).context("serialize scores.json")?;
    let scores_for_suggestions = read_scores_if_present(&run_paths).unwrap_or(scores.clone());
    let suggestions = build_suggestions_artifact(&run_summary, Some(&scores_for_suggestions));
    let suggestions_json =
        serde_json::to_vec_pretty(&suggestions).context("serialize suggestions.json")?;
    let cognitive_signals = build_cognitive_signals_artifact_from_state(
        &run_summary,
        &runtime_control.signals,
        &suggestions,
        Some(&scores_for_suggestions),
    );
    let cognitive_signals_json = serde_json::to_vec_pretty(&cognitive_signals)
        .context("serialize cognitive_signals.v1.json")?;
    let affect_state =
        build_affect_state_artifact(&run_summary, &suggestions, Some(&scores_for_suggestions));
    let affect_state_json =
        serde_json::to_vec_pretty(&affect_state).context("serialize affect_state.v1.json")?;
    let cognitive_arbitration = build_cognitive_arbitration_artifact_from_state(
        &run_summary,
        &suggestions,
        &runtime_control.arbitration,
        Some(&scores_for_suggestions),
    );
    let fast_slow_path = build_fast_slow_path_artifact(
        &run_summary,
        &cognitive_arbitration,
        &runtime_control.fast_slow,
        Some(&scores_for_suggestions),
    );
    let agency_selection = build_agency_selection_artifact(
        &run_summary,
        &cognitive_arbitration,
        &runtime_control.agency,
        Some(&scores_for_suggestions),
    );
    let bounded_execution = build_bounded_execution_artifact(
        &run_summary,
        &fast_slow_path,
        &agency_selection,
        &runtime_control.bounded_execution,
        Some(&scores_for_suggestions),
    );
    let evaluation_signals = build_evaluation_signals_artifact(
        &run_summary,
        &fast_slow_path,
        &agency_selection,
        &runtime_control.evaluation,
        Some(&scores_for_suggestions),
    );
    let reframing = build_reframing_artifact(
        &run_summary,
        &fast_slow_path,
        &agency_selection,
        &runtime_control.reframing,
        Some(&scores_for_suggestions),
    );
    let freedom_gate = build_freedom_gate_artifact(
        &run_summary,
        &evaluation_signals,
        &runtime_control.freedom_gate,
        Some(&scores_for_suggestions),
    );
    let convergence = build_aee_convergence_artifact(
        &run_summary,
        &bounded_execution,
        &evaluation_signals,
        &reframing,
        &freedom_gate,
        Some(&scores_for_suggestions),
    );
    let memory_read = build_memory_read_artifact(
        &run_summary,
        &evaluation_signals,
        &runtime_control.memory.read,
        Some(&scores_for_suggestions),
    );
    let memory_write = build_memory_write_artifact(
        &run_summary,
        &evaluation_signals,
        &runtime_control.memory.write,
        Some(&scores_for_suggestions),
    );
    let control_path_memory =
        build_control_path_memory_artifact(&run_summary, &memory_read, &memory_write);
    let control_path_action_proposals = build_control_path_action_proposals_artifact(
        &run_summary,
        &cognitive_arbitration,
        &agency_selection,
        &freedom_gate,
        Some(&scores_for_suggestions),
    );
    let control_path_decisions = build_control_path_decisions_artifact(
        &run_summary,
        &cognitive_arbitration,
        &agency_selection,
        &evaluation_signals,
        &reframing,
        &freedom_gate,
        Some(&scores_for_suggestions),
    );
    let control_path_action_mediation = build_control_path_action_mediation_artifact(
        &run_summary,
        &control_path_action_proposals,
        &freedom_gate,
        &control_path_decisions,
        Some(&scores_for_suggestions),
    );
    let control_path_skill_model = build_control_path_skill_model_artifact(
        &run_summary,
        &control_path_action_proposals,
        &control_path_action_mediation,
        Some(&scores_for_suggestions),
    );
    let control_path_skill_execution_protocol =
        build_control_path_skill_execution_protocol_artifact(
            &run_summary,
            &control_path_action_proposals,
            &control_path_skill_model,
            &control_path_action_mediation,
            Some(&scores_for_suggestions),
        );
    let control_path_final_result = build_control_path_final_result_artifact(
        &run_summary,
        &cognitive_arbitration,
        &agency_selection,
        &evaluation_signals,
        &freedom_gate,
    );
    let control_path_security_review = build_control_path_security_review_artifact(
        &run_summary,
        &cognitive_arbitration,
        &control_path_action_proposals,
        &control_path_action_mediation,
        &freedom_gate,
        &control_path_memory,
        &control_path_final_result,
        Some(&scores_for_suggestions),
    );
    let control_path_summary = build_control_path_summary(&ControlPathSummaryContext {
        signals: &cognitive_signals,
        agency: &agency_selection,
        arbitration: &cognitive_arbitration,
        execution: &bounded_execution,
        evaluation: &evaluation_signals,
        reframing: &reframing,
        convergence: &convergence,
        memory: &control_path_memory,
        action_proposals: &control_path_action_proposals,
        skill_model: &control_path_skill_model,
        skill_execution_protocol: &control_path_skill_execution_protocol,
        mediation: &control_path_action_mediation,
        freedom_gate: &freedom_gate,
        final_result: &control_path_final_result,
        security_review: &control_path_security_review,
    });
    let cognitive_arbitration_json = serde_json::to_vec_pretty(&cognitive_arbitration)
        .context("serialize cognitive_arbitration.v1.json")?;
    let fast_slow_path_json =
        serde_json::to_vec_pretty(&fast_slow_path).context("serialize fast_slow_path.v1.json")?;
    let agency_selection_json = serde_json::to_vec_pretty(&agency_selection)
        .context("serialize agency_selection.v1.json")?;
    let bounded_execution_json = serde_json::to_vec_pretty(&bounded_execution)
        .context("serialize bounded_execution.v1.json")?;
    let evaluation_signals_json = serde_json::to_vec_pretty(&evaluation_signals)
        .context("serialize evaluation_signals.v1.json")?;
    let reframing_json =
        serde_json::to_vec_pretty(&reframing).context("serialize reframing.v1.json")?;
    let freedom_gate_json =
        serde_json::to_vec_pretty(&freedom_gate).context("serialize freedom_gate.v1.json")?;
    let convergence_json =
        serde_json::to_vec_pretty(&convergence).context("serialize convergence.json")?;
    let memory_read_json =
        serde_json::to_vec_pretty(&memory_read).context("serialize memory_read.v1.json")?;
    let memory_write_json =
        serde_json::to_vec_pretty(&memory_write).context("serialize memory_write.v1.json")?;
    let control_path_memory_json = serde_json::to_vec_pretty(&control_path_memory)
        .context("serialize control_path memory.json")?;
    let control_path_action_proposals_json =
        serde_json::to_vec_pretty(&control_path_action_proposals)
            .context("serialize control_path action_proposals.json")?;
    let control_path_decisions_json = serde_json::to_vec_pretty(&control_path_decisions)
        .context("serialize control_path decisions.json")?;
    let control_path_action_mediation_json =
        serde_json::to_vec_pretty(&control_path_action_mediation)
            .context("serialize control_path mediation.json")?;
    let control_path_skill_model_json = serde_json::to_vec_pretty(&control_path_skill_model)
        .context("serialize control_path skill_model.json")?;
    let control_path_skill_execution_protocol_json =
        serde_json::to_vec_pretty(&control_path_skill_execution_protocol)
            .context("serialize control_path skill_execution_protocol.json")?;
    let control_path_final_result_json = serde_json::to_vec_pretty(&control_path_final_result)
        .context("serialize control_path final_result.json")?;
    let control_path_security_review_json =
        serde_json::to_vec_pretty(&control_path_security_review)
            .context("serialize control_path security_review.json")?;
    let aee_decision = build_aee_decision_artifact(
        &run_summary,
        &suggestions,
        &affect_state,
        Some(&scores_for_suggestions),
    );
    let reasoning_graph = build_reasoning_graph_artifact(
        &run_summary,
        &affect_state,
        &aee_decision,
        Some(&scores_for_suggestions),
    );
    let aee_decision_json =
        serde_json::to_vec_pretty(&aee_decision).context("serialize aee_decision.json")?;
    let reasoning_graph_json =
        serde_json::to_vec_pretty(&reasoning_graph).context("serialize reasoning_graph.v1.json")?;

    artifacts::atomic_write(&run_paths.run_json(), &run_json)?;
    artifacts::atomic_write(&run_paths.steps_json(), &steps_json)?;
    artifacts::atomic_write(&run_paths.trace_v1_json(), &trace_v1_json)?;
    artifacts::atomic_write(&run_paths.run_status_json(), &run_status_json)?;
    artifacts::atomic_write(&run_paths.run_summary_json(), &run_summary_json)?;
    let run_manifest = build_run_manifest(resolved, status, &run_paths);
    let run_manifest_json =
        serde_json::to_vec_pretty(&run_manifest).context("serialize run_manifest.json")?;
    artifacts::atomic_write(&run_paths.run_manifest_json(), &run_manifest_json)?;
    artifacts::atomic_write(&run_paths.scores_json(), &scores_json)?;
    artifacts::atomic_write(&run_paths.suggestions_json(), &suggestions_json)?;
    artifacts::atomic_write(&run_paths.cognitive_signals_json(), &cognitive_signals_json)?;
    artifacts::atomic_write(
        &run_paths.cognitive_arbitration_json(),
        &cognitive_arbitration_json,
    )?;
    artifacts::atomic_write(&run_paths.fast_slow_path_json(), &fast_slow_path_json)?;
    artifacts::atomic_write(&run_paths.agency_selection_json(), &agency_selection_json)?;
    artifacts::atomic_write(&run_paths.bounded_execution_json(), &bounded_execution_json)?;
    artifacts::atomic_write(
        &run_paths.evaluation_signals_json(),
        &evaluation_signals_json,
    )?;
    artifacts::atomic_write(&run_paths.reframing_json(), &reframing_json)?;
    artifacts::atomic_write(&run_paths.freedom_gate_json(), &freedom_gate_json)?;
    artifacts::atomic_write(&run_paths.memory_read_json(), &memory_read_json)?;
    artifacts::atomic_write(&run_paths.memory_write_json(), &memory_write_json)?;
    artifacts::atomic_write(
        &run_paths.control_path_signals_json(),
        &cognitive_signals_json,
    )?;
    artifacts::atomic_write(
        &run_paths.control_path_candidate_selection_json(),
        &agency_selection_json,
    )?;
    artifacts::atomic_write(
        &run_paths.control_path_arbitration_json(),
        &cognitive_arbitration_json,
    )?;
    artifacts::atomic_write(
        &run_paths.control_path_execution_iterations_json(),
        &bounded_execution_json,
    )?;
    artifacts::atomic_write(
        &run_paths.control_path_evaluation_json(),
        &evaluation_signals_json,
    )?;
    artifacts::atomic_write(&run_paths.control_path_reframing_json(), &reframing_json)?;
    artifacts::atomic_write(
        &run_paths.control_path_memory_json(),
        &control_path_memory_json,
    )?;
    artifacts::atomic_write(
        &run_paths.control_path_action_proposals_json(),
        &control_path_action_proposals_json,
    )?;
    artifacts::atomic_write(
        &run_paths.control_path_decisions_json(),
        &control_path_decisions_json,
    )?;
    artifacts::atomic_write(
        &run_paths.control_path_action_mediation_json(),
        &control_path_action_mediation_json,
    )?;
    artifacts::atomic_write(
        &run_paths.control_path_skill_model_json(),
        &control_path_skill_model_json,
    )?;
    artifacts::atomic_write(
        &run_paths.control_path_skill_execution_protocol_json(),
        &control_path_skill_execution_protocol_json,
    )?;
    artifacts::atomic_write(
        &run_paths.control_path_freedom_gate_json(),
        &freedom_gate_json,
    )?;
    artifacts::atomic_write(
        &run_paths.control_path_convergence_json(),
        &convergence_json,
    )?;
    artifacts::atomic_write(
        &run_paths.control_path_final_result_json(),
        &control_path_final_result_json,
    )?;
    artifacts::atomic_write(
        &run_paths.control_path_security_review_json(),
        &control_path_security_review_json,
    )?;
    artifacts::atomic_write(
        &run_paths.control_path_summary_txt(),
        control_path_summary.as_bytes(),
    )?;
    artifacts::atomic_write(&run_paths.cognitive_signals_json(), &cognitive_signals_json)?;
    artifacts::atomic_write(
        &run_paths.cognitive_arbitration_json(),
        &cognitive_arbitration_json,
    )?;
    artifacts::atomic_write(&run_paths.fast_slow_path_json(), &fast_slow_path_json)?;
    artifacts::atomic_write(&run_paths.agency_selection_json(), &agency_selection_json)?;
    artifacts::atomic_write(&run_paths.bounded_execution_json(), &bounded_execution_json)?;
    artifacts::atomic_write(
        &run_paths.evaluation_signals_json(),
        &evaluation_signals_json,
    )?;
    artifacts::atomic_write(&run_paths.reframing_json(), &reframing_json)?;
    artifacts::atomic_write(&run_paths.freedom_gate_json(), &freedom_gate_json)?;
    artifacts::atomic_write(&run_paths.memory_read_json(), &memory_read_json)?;
    artifacts::atomic_write(&run_paths.memory_write_json(), &memory_write_json)?;
    artifacts::atomic_write(&run_paths.affect_state_json(), &affect_state_json)?;
    artifacts::atomic_write(&run_paths.aee_decision_json(), &aee_decision_json)?;
    artifacts::atomic_write(&run_paths.reasoning_graph_json(), &reasoning_graph_json)?;
    if let Some(pause_payload) = pause {
        let pause_artifact = PauseStateArtifact {
            schema_version: PAUSE_STATE_SCHEMA_VERSION.to_string(),
            run_id: resolved.run_id.clone(),
            workflow_id: resolved.workflow_id.clone(),
            version: resolved.doc.version.clone(),
            status: "paused".to_string(),
            adl_path: sanitize_pause_adl_path(adl_path),
            execution_plan_hash: execution_plan_hash(&resolved.execution_plan)?,
            steering_history: steering_history.to_vec(),
            pause: pause_payload.clone(),
        };
        let pause_json =
            serde_json::to_vec_pretty(&pause_artifact).context("serialize pause_state.json")?;
        artifacts::atomic_write(&run_paths.pause_state_json(), &pause_json)?;
    }

    Ok(run_dir)
}

#[derive(Debug, Serialize)]
struct RunManifestV1 {
    schema_version: &'static str,
    run_id: String,
    workflow_id: String,
    adl_version: String,
    status: String,
    milestone: String,
    issue: Option<String>,
    pr: Option<String>,
    demo: Option<String>,
    provider_ids: Vec<String>,
    runtime_root_source: String,
    runs_root_source: String,
    runs_root: String,
    trace_status: String,
    generated_artifacts: Vec<String>,
}

fn build_run_manifest(
    resolved: &resolve::AdlResolved,
    status: &str,
    run_paths: &artifacts::RunArtifactPaths,
) -> RunManifestV1 {
    let mut provider_ids: Vec<String> = resolved
        .steps
        .iter()
        .filter_map(|step| step.provider.as_ref())
        .cloned()
        .collect();
    provider_ids.sort();
    provider_ids.dedup();

    let (runtime_root_source, runs_root_source, runs_root) = RuntimeEnvironment::current()
        .ok()
        .map(|env| {
            let runs_root = if env.runs_root() == run_paths.runs_root() {
                if let Ok(relative) = env.runs_root().strip_prefix(env.repo_root()) {
                    relative.display().to_string()
                } else {
                    "external_runs_root".to_string()
                }
            } else {
                "explicit_runs_root".to_string()
            };
            (
                env.runtime_root_source().as_str().to_string(),
                env.runs_root_source().as_str().to_string(),
                runs_root,
            )
        })
        .unwrap_or_else(|| {
            (
                "unknown".to_string(),
                "unknown".to_string(),
                "unknown_runs_root".to_string(),
            )
        });

    RunManifestV1 {
        schema_version: "trace_run_manifest.v1",
        run_id: resolved.run_id.clone(),
        workflow_id: resolved.workflow_id.clone(),
        adl_version: resolved.doc.version.clone(),
        status: status.to_string(),
        milestone: trimmed_env("ADL_MILESTONE")
            .unwrap_or_else(|| version_to_milestone(&resolved.doc.version)),
        issue: trimmed_env("ADL_ISSUE_ID"),
        pr: trimmed_env("ADL_PR_ID"),
        demo: trimmed_env("ADL_DEMO_NAME"),
        provider_ids,
        runtime_root_source,
        runs_root_source,
        runs_root,
        trace_status: "captured".to_string(),
        generated_artifacts: vec![
            "run.json".to_string(),
            "steps.json".to_string(),
            "run_status.json".to_string(),
            "run_summary.json".to_string(),
            "run_manifest.json".to_string(),
            "logs/activation_log.json".to_string(),
            "logs/trace_v1.json".to_string(),
        ],
    }
}

fn version_to_milestone(version: &str) -> String {
    let trimmed = version.trim();
    if trimmed.is_empty() {
        "unclassified".to_string()
    } else if trimmed.starts_with('v') {
        trimmed.to_string()
    } else {
        format!("v{trimmed}")
    }
}

fn trimmed_env(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn build_trace_v1_envelope(
    resolved: &resolve::AdlResolved,
    tr: &trace::Trace,
    steps: &[StepStateArtifact],
    start_ms: u128,
    end_ms: u128,
    status: &str,
    failure: Option<&anyhow::Error>,
) -> Result<TraceEventEnvelopeV1> {
    let mut events = Vec::new();
    let mut next_id: u64 = 1;
    let trace_id = resolved.run_id.clone();
    let root_span_id = format!("run:{}", resolved.run_id);
    let run_ref = artifact_ref(&resolved.run_id, "run.json");
    let steps_ref = artifact_ref(&resolved.run_id, "steps.json");
    let activation_log_ref = artifact_ref(&resolved.run_id, "logs/activation_log.json");

    push_trace_v1_event(
        &mut events,
        &mut next_id,
        TraceEventV1 {
            event_id: String::new(),
            timestamp: trace::format_iso_utc_ms(start_ms),
            event_type: TraceEventTypeV1::RunStart,
            trace_id: trace_id.clone(),
            run_id: resolved.run_id.clone(),
            span_id: root_span_id.clone(),
            parent_span_id: None,
            actor: TraceActorV1 {
                r#type: TraceActorTypeV1::Agent,
                id: resolved.workflow_id.clone(),
            },
            scope: TraceScopeV1 {
                level: TraceScopeLevelV1::Run,
                name: resolved.workflow_id.clone(),
            },
            inputs_ref: Some(run_ref.clone()),
            outputs_ref: Some(activation_log_ref.clone()),
            artifact_ref: Some(run_ref.clone()),
            decision_context: None,
            provider: None,
            error: None,
            contract_validation: None,
        },
    );

    for event in &tr.events {
        match event {
            trace::TraceEvent::LifecyclePhaseEntered { ts_ms, phase, .. } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::LifecyclePhase,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("run:{}:phase:{}", resolved.run_id, phase.as_str()),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "runtime".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Run,
                        name: phase.as_str().to_string(),
                    },
                    inputs_ref: Some(run_ref.clone()),
                    outputs_ref: Some(activation_log_ref.clone()),
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: Some(TraceDecisionContextV1 {
                        context: "runtime lifecycle phase".to_string(),
                        outcome: phase.as_str().to_string(),
                        rationale: None,
                    }),
                    provider: None,
                    error: None,
                    contract_validation: None,
                },
            ),
            trace::TraceEvent::ExecutionBoundaryCrossed {
                ts_ms,
                boundary,
                state,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::ExecutionBoundary,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!(
                        "run:{}:boundary:{}:{}",
                        resolved.run_id,
                        boundary.as_str(),
                        state
                    ),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "runtime".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Run,
                        name: boundary.as_str().to_string(),
                    },
                    inputs_ref: Some(run_ref.clone()),
                    outputs_ref: Some(activation_log_ref.clone()),
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: Some(TraceDecisionContextV1 {
                        context: "execution boundary".to_string(),
                        outcome: state.clone(),
                        rationale: None,
                    }),
                    provider: None,
                    error: None,
                    contract_validation: None,
                },
            ),
            trace::TraceEvent::StepStarted {
                ts_ms,
                step_id,
                agent_id,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::StepStart,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("step:{step_id}"),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::Agent,
                        id: agent_id.clone(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Step,
                        name: step_id.clone(),
                    },
                    inputs_ref: Some(steps_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: None,
                    provider: None,
                    error: None,
                    contract_validation: None,
                },
            ),
            trace::TraceEvent::StepFinished {
                ts_ms,
                step_id,
                success,
                ..
            } => {
                let step_output_ref = step_artifact_ref(&resolved.run_id, steps, step_id);
                push_trace_v1_event(
                    &mut events,
                    &mut next_id,
                    TraceEventV1 {
                        event_id: String::new(),
                        timestamp: trace::format_iso_utc_ms(*ts_ms),
                        event_type: TraceEventTypeV1::StepEnd,
                        trace_id: trace_id.clone(),
                        run_id: resolved.run_id.clone(),
                        span_id: format!("step:{step_id}"),
                        parent_span_id: Some(root_span_id.clone()),
                        actor: TraceActorV1 {
                            r#type: TraceActorTypeV1::Agent,
                            id: resolved.workflow_id.clone(),
                        },
                        scope: TraceScopeV1 {
                            level: TraceScopeLevelV1::Step,
                            name: step_id.clone(),
                        },
                        inputs_ref: Some(steps_ref.clone()),
                        outputs_ref: step_output_ref.clone().or(Some(steps_ref.clone())),
                        artifact_ref: step_output_ref.or(Some(activation_log_ref.clone())),
                        decision_context: None,
                        provider: None,
                        error: None,
                        contract_validation: None,
                    },
                );
                if !success {
                    push_trace_v1_event(
                        &mut events,
                        &mut next_id,
                        TraceEventV1 {
                            event_id: String::new(),
                            timestamp: trace::format_iso_utc_ms(*ts_ms),
                            event_type: TraceEventTypeV1::Error,
                            trace_id: trace_id.clone(),
                            run_id: resolved.run_id.clone(),
                            span_id: format!("step:{step_id}:error"),
                            parent_span_id: Some(format!("step:{step_id}")),
                            actor: TraceActorV1 {
                                r#type: TraceActorTypeV1::System,
                                id: "runtime".to_string(),
                            },
                            scope: TraceScopeV1 {
                                level: TraceScopeLevelV1::Step,
                                name: step_id.clone(),
                            },
                            inputs_ref: Some(steps_ref.clone()),
                            outputs_ref: None,
                            artifact_ref: Some(activation_log_ref.clone()),
                            decision_context: None,
                            provider: None,
                            error: Some(TraceErrorV1 {
                                code: "STEP_FAILURE".to_string(),
                                message: format!("step '{step_id}' finished unsuccessfully"),
                                details: None,
                            }),
                            contract_validation: None,
                        },
                    );
                }
            }
            trace::TraceEvent::DelegationPolicyEvaluated {
                ts_ms,
                step_id,
                action_kind,
                target_id,
                decision,
                rule_id,
                ..
            } => {
                let result = if decision.eq_ignore_ascii_case("allowed")
                    || decision.eq_ignore_ascii_case("approved")
                    || decision.eq_ignore_ascii_case("pass")
                {
                    ContractValidationResultV1::Pass
                } else {
                    ContractValidationResultV1::Fail
                };
                push_trace_v1_event(
                    &mut events,
                    &mut next_id,
                    TraceEventV1 {
                        event_id: String::new(),
                        timestamp: trace::format_iso_utc_ms(*ts_ms),
                        event_type: TraceEventTypeV1::ContractValidation,
                        trace_id: trace_id.clone(),
                        run_id: resolved.run_id.clone(),
                        span_id: format!("step:{step_id}:policy"),
                        parent_span_id: Some(format!("step:{step_id}")),
                        actor: TraceActorV1 {
                            r#type: TraceActorTypeV1::System,
                            id: "policy-engine".to_string(),
                        },
                        scope: TraceScopeV1 {
                            level: TraceScopeLevelV1::Step,
                            name: step_id.clone(),
                        },
                        inputs_ref: Some(steps_ref.clone()),
                        outputs_ref: None,
                        artifact_ref: Some(activation_log_ref.clone()),
                        decision_context: None,
                        provider: None,
                        error: None,
                        contract_validation: Some(TraceContractValidationV1 {
                            contract_id: "adl.delegation_policy".to_string(),
                            result,
                            details: Some(json!({
                                "step_id": step_id,
                                "action_kind": action_kind,
                                "target_id": target_id,
                                "decision": decision,
                                "rule_id": rule_id,
                            })),
                        }),
                    },
                );
            }
            trace::TraceEvent::DelegationApproved { ts_ms, step_id, .. } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::Approval,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("step:{step_id}:approval"),
                    parent_span_id: Some(format!("step:{step_id}")),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "policy-engine".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Step,
                        name: step_id.clone(),
                    },
                    inputs_ref: Some(steps_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: Some(TraceDecisionContextV1 {
                        context: "delegation policy".to_string(),
                        outcome: "approved".to_string(),
                        rationale: None,
                    }),
                    provider: None,
                    error: None,
                    contract_validation: None,
                },
            ),
            trace::TraceEvent::DelegationDenied {
                ts_ms,
                step_id,
                action_kind,
                target_id,
                rule_id,
                ..
            } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::Rejection,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("step:{step_id}:rejection"),
                    parent_span_id: Some(format!("step:{step_id}")),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "policy-engine".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Step,
                        name: step_id.clone(),
                    },
                    inputs_ref: Some(steps_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: Some(TraceDecisionContextV1 {
                        context: format!("delegation policy {action_kind} -> {target_id}"),
                        outcome: "denied".to_string(),
                        rationale: rule_id.clone(),
                    }),
                    provider: None,
                    error: None,
                    contract_validation: None,
                },
            ),
            trace::TraceEvent::RunFailed { ts_ms, message, .. } => push_trace_v1_event(
                &mut events,
                &mut next_id,
                TraceEventV1 {
                    event_id: String::new(),
                    timestamp: trace::format_iso_utc_ms(*ts_ms),
                    event_type: TraceEventTypeV1::Error,
                    trace_id: trace_id.clone(),
                    run_id: resolved.run_id.clone(),
                    span_id: format!("run:{}:error", resolved.run_id),
                    parent_span_id: Some(root_span_id.clone()),
                    actor: TraceActorV1 {
                        r#type: TraceActorTypeV1::System,
                        id: "runtime".to_string(),
                    },
                    scope: TraceScopeV1 {
                        level: TraceScopeLevelV1::Run,
                        name: resolved.workflow_id.clone(),
                    },
                    inputs_ref: Some(run_ref.clone()),
                    outputs_ref: None,
                    artifact_ref: Some(activation_log_ref.clone()),
                    decision_context: None,
                    provider: None,
                    error: Some(TraceErrorV1 {
                        code: "RUN_FAILURE".to_string(),
                        message: message.clone(),
                        details: None,
                    }),
                    contract_validation: None,
                },
            ),
            _ => {}
        }
    }

    let run_end_outcome = if status == "success" {
        "success".to_string()
    } else if status == "paused" {
        "paused".to_string()
    } else {
        "failure".to_string()
    };
    push_trace_v1_event(
        &mut events,
        &mut next_id,
        TraceEventV1 {
            event_id: String::new(),
            timestamp: trace::format_iso_utc_ms(end_ms),
            event_type: TraceEventTypeV1::RunEnd,
            trace_id,
            run_id: resolved.run_id.clone(),
            span_id: root_span_id,
            parent_span_id: None,
            actor: TraceActorV1 {
                r#type: TraceActorTypeV1::Agent,
                id: resolved.workflow_id.clone(),
            },
            scope: TraceScopeV1 {
                level: TraceScopeLevelV1::Run,
                name: resolved.workflow_id.clone(),
            },
            inputs_ref: Some(run_ref),
            outputs_ref: Some(steps_ref),
            artifact_ref: Some(activation_log_ref),
            decision_context: Some(TraceDecisionContextV1 {
                context: "run completion".to_string(),
                outcome: run_end_outcome,
                rationale: failure.map(|err| err.to_string()),
            }),
            provider: None,
            error: None,
            contract_validation: None,
        },
    );

    let envelope = TraceEventEnvelopeV1 {
        schema_version: "trace.v1".to_string(),
        events,
    };
    validate_trace_event_envelope_v1(&envelope)?;
    Ok(envelope)
}

fn push_trace_v1_event(events: &mut Vec<TraceEventV1>, next_id: &mut u64, mut event: TraceEventV1) {
    event.event_id = format!("trace-v1-{:04}", *next_id);
    *next_id = next_id.saturating_add(1);
    events.push(event);
}

fn artifact_ref(run_id: &str, relative_path: &str) -> String {
    format!("artifacts/{run_id}/{relative_path}")
}

fn step_artifact_ref(run_id: &str, steps: &[StepStateArtifact], step_id: &str) -> Option<String> {
    let rel = steps
        .iter()
        .find(|step| step.step_id == step_id)
        .and_then(|step| step.output_artifact_path.as_deref())?;
    Some(artifact_ref(run_id, rel))
}

fn read_required_json_artifact<T>(control_path_dir: &Path, file_name: &str) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let path = control_path_dir.join(file_name);
    let raw = std::fs::read_to_string(&path).with_context(|| {
        format!(
            "missing required control-path artifact '{}'",
            path.display()
        )
    })?;
    serde_json::from_str(&raw)
        .with_context(|| format!("invalid control-path artifact '{}'", path.display()))
}

fn read_run_summary_near_control_path(control_path_dir: &Path) -> Result<RunSummaryArtifact> {
    let candidate_paths = [
        control_path_dir.join("run_summary.json"),
        control_path_dir
            .parent()
            .map(|parent| parent.join("run_summary.json"))
            .unwrap_or_else(|| control_path_dir.join("run_summary.json")),
    ];
    for path in candidate_paths {
        if !path.exists() {
            continue;
        }
        let raw = std::fs::read_to_string(&path).with_context(|| {
            format!(
                "failed to read control-path sibling artifact '{}'",
                path.display()
            )
        })?;
        return serde_json::from_str(&raw).with_context(|| {
            format!("invalid control-path sibling artifact '{}'", path.display())
        });
    }
    Err(anyhow!(
        "missing required control-path sibling artifact 'run_summary.json' near '{}'",
        control_path_dir.display()
    ))
}

pub(crate) fn validate_control_path_artifact_set(control_path_dir: &Path) -> Result<()> {
    if !control_path_dir.exists() {
        return Err(anyhow!(
            "control-path artifact root does not exist: {}",
            control_path_dir.display()
        ));
    }

    let signals: CognitiveSignalsArtifact =
        read_required_json_artifact(control_path_dir, "signals.json")?;
    let agency: AgencySelectionArtifact =
        read_required_json_artifact(control_path_dir, "candidate_selection.json")?;
    let arbitration: CognitiveArbitrationArtifact =
        read_required_json_artifact(control_path_dir, "arbitration.json")?;
    let execution: BoundedExecutionArtifact =
        read_required_json_artifact(control_path_dir, "execution_iterations.json")?;
    let evaluation: EvaluationSignalsArtifact =
        read_required_json_artifact(control_path_dir, "evaluation.json")?;
    let reframing: ReframingArtifact =
        read_required_json_artifact(control_path_dir, "reframing.json")?;
    let memory: ControlPathMemoryArtifact =
        read_required_json_artifact(control_path_dir, "memory.json")?;
    let action_proposals: ControlPathActionProposalsArtifact =
        read_required_json_artifact(control_path_dir, "action_proposals.json")?;
    let decisions: ControlPathDecisionsArtifact =
        read_required_json_artifact(control_path_dir, "decisions.json")?;
    let mediation: ControlPathActionMediationArtifact =
        read_required_json_artifact(control_path_dir, "mediation.json")?;
    let skill_model: ControlPathSkillModelArtifact =
        read_required_json_artifact(control_path_dir, "skill_model.json")?;
    let skill_execution_protocol: ControlPathSkillExecutionProtocolArtifact =
        read_required_json_artifact(control_path_dir, "skill_execution_protocol.json")?;
    let freedom_gate: FreedomGateArtifact =
        read_required_json_artifact(control_path_dir, "freedom_gate.json")?;
    let convergence: AeeConvergenceArtifact =
        read_required_json_artifact(control_path_dir, "convergence.json")?;
    let final_result: ControlPathFinalResultArtifact =
        read_required_json_artifact(control_path_dir, "final_result.json")?;
    let security_review: ControlPathSecurityReviewArtifact =
        read_required_json_artifact(control_path_dir, "security_review.json")?;
    let run_summary = read_run_summary_near_control_path(control_path_dir)?;

    let summary_path = control_path_dir.join("summary.txt");
    let summary = std::fs::read_to_string(&summary_path).with_context(|| {
        format!(
            "missing required control-path artifact '{}'",
            summary_path.display()
        )
    })?;
    if summary.trim().is_empty() {
        return Err(anyhow!(
            "control-path summary is empty at '{}'",
            summary_path.display()
        ));
    }

    let expected_stage_order = vec![
        "signals".to_string(),
        "candidate_selection".to_string(),
        "arbitration".to_string(),
        "execution".to_string(),
        "evaluation".to_string(),
        "reframing".to_string(),
        "memory".to_string(),
        "freedom_gate".to_string(),
        "final_result".to_string(),
    ];
    if final_result.stage_order != expected_stage_order {
        return Err(anyhow!(
            "control-path final_result stage_order mismatch: expected {:?}, found {:?}",
            expected_stage_order,
            final_result.stage_order
        ));
    }

    let run_ids = [
        signals.run_id.as_str(),
        agency.run_id.as_str(),
        arbitration.run_id.as_str(),
        execution.run_id.as_str(),
        evaluation.run_id.as_str(),
        reframing.run_id.as_str(),
        memory.run_id.as_str(),
        action_proposals.run_id.as_str(),
        decisions.run_id.as_str(),
        mediation.run_id.as_str(),
        skill_model.run_id.as_str(),
        skill_execution_protocol.run_id.as_str(),
        freedom_gate.run_id.as_str(),
        convergence.run_id.as_str(),
        final_result.run_id.as_str(),
        security_review.run_id.as_str(),
    ];
    let canonical_run_id = final_result.run_id.as_str();
    if run_ids.iter().any(|run_id| *run_id != canonical_run_id) {
        return Err(anyhow!(
            "control-path artifact run_id mismatch under '{}'",
            control_path_dir.display()
        ));
    }

    if final_result.route_selected != arbitration.route_selected {
        return Err(anyhow!(
            "control-path final_result route '{}' does not match arbitration route '{}'",
            final_result.route_selected,
            arbitration.route_selected
        ));
    }
    if final_result.selected_candidate != agency.selected_candidate_id {
        return Err(anyhow!(
            "control-path final_result selected_candidate '{}' does not match candidate_selection '{}'",
            final_result.selected_candidate,
            agency.selected_candidate_id
        ));
    }
    if final_result.termination_reason != evaluation.termination_reason {
        return Err(anyhow!(
            "control-path final_result termination_reason '{}' does not match evaluation '{}'",
            final_result.termination_reason,
            evaluation.termination_reason
        ));
    }
    if final_result.gate_decision != freedom_gate.gate_decision {
        return Err(anyhow!(
            "control-path final_result gate_decision '{}' does not match freedom_gate '{}'",
            final_result.gate_decision,
            freedom_gate.gate_decision
        ));
    }
    if final_result.next_control_action != evaluation.next_control_action {
        return Err(anyhow!(
            "control-path final_result next_control_action '{}' does not match evaluation '{}'",
            final_result.next_control_action,
            evaluation.next_control_action
        ));
    }
    if convergence.selected_candidate_id != agency.selected_candidate_id {
        return Err(anyhow!(
            "control-path convergence selected_candidate_id '{}' does not match candidate_selection '{}'",
            convergence.selected_candidate_id,
            agency.selected_candidate_id
        ));
    }
    if convergence.termination_reason != evaluation.termination_reason {
        return Err(anyhow!(
            "control-path convergence termination_reason '{}' does not match evaluation '{}'",
            convergence.termination_reason,
            evaluation.termination_reason
        ));
    }
    if convergence.gate_decision != freedom_gate.gate_decision {
        return Err(anyhow!(
            "control-path convergence gate_decision '{}' does not match freedom_gate '{}'",
            convergence.gate_decision,
            freedom_gate.gate_decision
        ));
    }
    if convergence.next_control_action != evaluation.next_control_action {
        return Err(anyhow!(
            "control-path convergence next_control_action '{}' does not match evaluation '{}'",
            convergence.next_control_action,
            evaluation.next_control_action
        ));
    }
    if security_review.posture.declared_posture
        != control_path_security_posture(&run_summary, &freedom_gate)
    {
        return Err(anyhow!(
            "control-path security review posture '{}' does not match derived posture '{}'",
            security_review.posture.declared_posture,
            control_path_security_posture(&run_summary, &freedom_gate)
        ));
    }
    if security_review.threat_model.attacker_pressure
        != control_path_security_attacker_pressure(&run_summary, &arbitration)
    {
        return Err(anyhow!(
            "control-path security review attacker_pressure '{}' does not match derived '{}'",
            security_review.threat_model.attacker_pressure,
            control_path_security_attacker_pressure(&run_summary, &arbitration)
        ));
    }
    if security_review.posture.accepted_risk_level != freedom_gate.input.risk_class {
        return Err(anyhow!(
            "control-path security review accepted_risk_level '{}' does not match freedom_gate '{}'",
            security_review.posture.accepted_risk_level,
            freedom_gate.input.risk_class
        ));
    }
    if security_review.posture.commitment_policy != freedom_gate.gate_decision {
        return Err(anyhow!(
            "control-path security review commitment_policy '{}' does not match freedom_gate '{}'",
            security_review.posture.commitment_policy,
            freedom_gate.gate_decision
        ));
    }
    if security_review.posture.mitigation_authority != mediation.mediation.runtime_authority {
        return Err(anyhow!(
            "control-path security review mitigation_authority '{}' does not match mediation '{}'",
            security_review.posture.mitigation_authority,
            mediation.mediation.runtime_authority
        ));
    }
    if security_review.trust_under_adversary.trust_state
        != control_path_security_trust_state(&action_proposals, &mediation, &freedom_gate)
    {
        return Err(anyhow!(
            "control-path security review trust_state '{}' does not match derived '{}'",
            security_review.trust_under_adversary.trust_state,
            control_path_security_trust_state(&action_proposals, &mediation, &freedom_gate)
        ));
    }
    if security_review.threat_model.active_trust_boundaries
        != control_path_security_boundaries(&run_summary)
    {
        return Err(anyhow!(
            "control-path security review boundaries mismatch: expected {:?}, found {:?}",
            control_path_security_boundaries(&run_summary),
            security_review.threat_model.active_trust_boundaries
        ));
    }
    if security_review.threat_model.canonical_threat_classes
        != control_path_security_threat_classes(&run_summary)
    {
        return Err(anyhow!(
            "control-path security review threat classes mismatch: expected {:?}, found {:?}",
            control_path_security_threat_classes(&run_summary),
            security_review.threat_model.canonical_threat_classes
        ));
    }
    if security_review.threat_model.required_mitigations
        != control_path_security_required_mitigations(&run_summary)
    {
        return Err(anyhow!(
            "control-path security review mitigations mismatch: expected {:?}, found {:?}",
            control_path_security_required_mitigations(&run_summary),
            security_review.threat_model.required_mitigations
        ));
    }
    if security_review.threat_model.reviewer_visible_surfaces
        != control_path_security_review_surfaces()
    {
        return Err(anyhow!(
            "control-path security review proof surfaces mismatch: expected {:?}, found {:?}",
            control_path_security_review_surfaces(),
            security_review.threat_model.reviewer_visible_surfaces
        ));
    }
    if security_review.trust_under_adversary.trusted_surfaces
        != control_path_security_review_surfaces()
    {
        return Err(anyhow!(
            "control-path security review trusted surfaces mismatch: expected {:?}, found {:?}",
            control_path_security_review_surfaces(),
            security_review.trust_under_adversary.trusted_surfaces
        ));
    }
    if security_review.trust_under_adversary.reduced_trust_surfaces
        != control_path_security_reduced_trust_surfaces(&run_summary, &memory)
    {
        return Err(anyhow!(
            "control-path security review reduced trust surfaces mismatch: expected {:?}, found {:?}",
            control_path_security_reduced_trust_surfaces(&run_summary, &memory),
            security_review.trust_under_adversary.reduced_trust_surfaces
        ));
    }
    if security_review
        .trust_under_adversary
        .revalidation_requirements
        != mediation.mediation.validation_checks
    {
        return Err(anyhow!(
            "control-path security review revalidation requirements mismatch: expected {:?}, found {:?}",
            mediation.mediation.validation_checks,
            security_review.trust_under_adversary.revalidation_requirements
        ));
    }
    if security_review.trust_under_adversary.escalation_path != freedom_gate.required_follow_up {
        return Err(anyhow!(
            "control-path security review escalation_path '{}' does not match freedom_gate '{}'",
            security_review.trust_under_adversary.escalation_path,
            freedom_gate.required_follow_up
        ));
    }
    let expected_security_denials: usize =
        run_summary.policy.security_denials_by_code.values().sum();
    if security_review.evidence.route_selected != arbitration.route_selected {
        return Err(anyhow!(
            "control-path security review evidence route '{}' does not match arbitration '{}'",
            security_review.evidence.route_selected,
            arbitration.route_selected
        ));
    }
    if security_review.evidence.risk_class != freedom_gate.input.risk_class {
        return Err(anyhow!(
            "control-path security review evidence risk_class '{}' does not match freedom_gate '{}'",
            security_review.evidence.risk_class,
            freedom_gate.input.risk_class
        ));
    }
    if security_review.evidence.mediation_outcome != mediation.mediation.mediation_outcome {
        return Err(anyhow!(
            "control-path security review evidence mediation_outcome '{}' does not match mediation '{}'",
            security_review.evidence.mediation_outcome,
            mediation.mediation.mediation_outcome
        ));
    }
    if security_review.evidence.gate_decision != freedom_gate.gate_decision {
        return Err(anyhow!(
            "control-path security review evidence gate_decision '{}' does not match freedom_gate '{}'",
            security_review.evidence.gate_decision,
            freedom_gate.gate_decision
        ));
    }
    if security_review.evidence.final_result != final_result.final_result {
        return Err(anyhow!(
            "control-path security review evidence final_result '{}' does not match final_result '{}'",
            security_review.evidence.final_result,
            final_result.final_result
        ));
    }
    if security_review.evidence.security_denied_count != expected_security_denials {
        return Err(anyhow!(
            "control-path security review evidence security_denied_count '{}' does not match run_summary '{}'",
            security_review.evidence.security_denied_count,
            expected_security_denials
        ));
    }
    if security_review.evidence.security_envelope_enabled
        != run_summary.policy.security_envelope_enabled
    {
        return Err(anyhow!(
            "control-path security review evidence security_envelope_enabled '{}' does not match run_summary '{}'",
            security_review.evidence.security_envelope_enabled,
            run_summary.policy.security_envelope_enabled
        ));
    }
    if security_review.evidence.signing_required != run_summary.policy.signing_required {
        return Err(anyhow!(
            "control-path security review evidence signing_required '{}' does not match run_summary '{}'",
            security_review.evidence.signing_required,
            run_summary.policy.signing_required
        ));
    }
    if security_review.evidence.key_id_required != run_summary.policy.key_id_required {
        return Err(anyhow!(
            "control-path security review evidence key_id_required '{}' does not match run_summary '{}'",
            security_review.evidence.key_id_required,
            run_summary.policy.key_id_required
        ));
    }
    if security_review.evidence.verify_allowed_algs != run_summary.policy.verify_allowed_algs {
        return Err(anyhow!(
            "control-path security review evidence verify_allowed_algs mismatch: expected {:?}, found {:?}",
            run_summary.policy.verify_allowed_algs,
            security_review.evidence.verify_allowed_algs
        ));
    }
    if security_review.evidence.verify_allowed_key_sources
        != run_summary.policy.verify_allowed_key_sources
    {
        return Err(anyhow!(
            "control-path security review evidence verify_allowed_key_sources mismatch: expected {:?}, found {:?}",
            run_summary.policy.verify_allowed_key_sources,
            security_review.evidence.verify_allowed_key_sources
        ));
    }
    if security_review.evidence.sandbox_policy != run_summary.policy.sandbox_policy {
        return Err(anyhow!(
            "control-path security review evidence sandbox_policy '{}' does not match run_summary '{}'",
            security_review.evidence.sandbox_policy,
            run_summary.policy.sandbox_policy
        ));
    }
    if security_review.evidence.trace_visibility_expectation
        != mediation.mediation.trace_expectation
    {
        return Err(anyhow!(
            "control-path security review evidence trace_visibility_expectation '{}' does not match mediation '{}'",
            security_review.evidence.trace_visibility_expectation,
            mediation.mediation.trace_expectation
        ));
    }

    let expected_schema_fields = vec![
        "decision_id".to_string(),
        "surface_id".to_string(),
        "proposal_or_action".to_string(),
        "outcome_class".to_string(),
        "decision_maker".to_string(),
        "policy_bindings".to_string(),
        "rationale".to_string(),
        "downstream_consequence".to_string(),
        "temporal_anchor".to_string(),
    ];
    if decisions.decision_schema_fields != expected_schema_fields {
        return Err(anyhow!(
            "control-path decisions schema fields mismatch: expected {:?}, found {:?}",
            expected_schema_fields,
            decisions.decision_schema_fields
        ));
    }

    let expected_outcome_vocabulary = vec![
        "accept".to_string(),
        "reject".to_string(),
        "defer".to_string(),
        "escalate".to_string(),
        "reroute".to_string(),
    ];
    if decisions.outcome_class_vocabulary != expected_outcome_vocabulary {
        return Err(anyhow!(
            "control-path decisions outcome vocabulary mismatch: expected {:?}, found {:?}",
            expected_outcome_vocabulary,
            decisions.outcome_class_vocabulary
        ));
    }
    if decisions.surfaces.len() != 3 || decisions.decisions.len() != 3 {
        return Err(anyhow!(
            "control-path decisions artifact must contain exactly 3 surfaces and 3 records"
        ));
    }

    let expected_proposal_schema_fields = vec![
        "proposal_id".to_string(),
        "kind".to_string(),
        "target".to_string(),
        "arguments".to_string(),
        "intent".to_string(),
        "content".to_string(),
        "confidence".to_string(),
        "requires_approval".to_string(),
        "metadata".to_string(),
        "non_authoritative".to_string(),
        "temporal_anchor".to_string(),
    ];
    if action_proposals.proposal_schema_fields != expected_proposal_schema_fields {
        return Err(anyhow!(
            "control-path action proposal schema fields mismatch: expected {:?}, found {:?}",
            expected_proposal_schema_fields,
            action_proposals.proposal_schema_fields
        ));
    }

    let expected_proposal_kind_vocabulary = vec![
        "tool_call".to_string(),
        "skill_call".to_string(),
        "memory_read".to_string(),
        "memory_write".to_string(),
        "final_answer".to_string(),
        "refuse".to_string(),
        "defer".to_string(),
    ];
    if action_proposals.proposal_kind_vocabulary != expected_proposal_kind_vocabulary {
        return Err(anyhow!(
            "control-path action proposal vocabulary mismatch: expected {:?}, found {:?}",
            expected_proposal_kind_vocabulary,
            action_proposals.proposal_kind_vocabulary
        ));
    }
    if action_proposals.proposals.len() != 1 {
        return Err(anyhow!(
            "control-path action proposals artifact must contain exactly 1 bounded proposal"
        ));
    }
    let proposal = &action_proposals.proposals[0];
    if !proposal.non_authoritative {
        return Err(anyhow!(
            "control-path action proposal '{}' must remain non-authoritative",
            proposal.proposal_id
        ));
    }
    if !action_proposals
        .proposal_kind_vocabulary
        .contains(&proposal.kind)
    {
        return Err(anyhow!(
            "control-path action proposal kind '{}' is not in the declared vocabulary",
            proposal.kind
        ));
    }

    let expected_mediation_outcome_vocabulary = vec![
        "approved".to_string(),
        "rejected".to_string(),
        "deferred".to_string(),
        "escalated".to_string(),
    ];
    if mediation.mediation_outcome_vocabulary != expected_mediation_outcome_vocabulary {
        return Err(anyhow!(
            "control-path mediation outcome vocabulary mismatch: expected {:?}, found {:?}",
            expected_mediation_outcome_vocabulary,
            mediation.mediation_outcome_vocabulary
        ));
    }
    if mediation.authority_boundary != "models_propose_runtime_decides_executes" {
        return Err(anyhow!(
            "control-path mediation authority boundary mismatch: '{}'",
            mediation.authority_boundary
        ));
    }
    if mediation.mediation.proposal_id != proposal.proposal_id {
        return Err(anyhow!(
            "control-path mediation proposal '{}' does not match action proposal '{}'",
            mediation.mediation.proposal_id,
            proposal.proposal_id
        ));
    }
    if mediation.mediation.runtime_authority != "freedom_gate" {
        return Err(anyhow!(
            "control-path mediation runtime authority '{}' must be freedom_gate",
            mediation.mediation.runtime_authority
        ));
    }
    let expected_mediation_outcome = match freedom_gate.gate_decision.as_str() {
        "allow" => "approved",
        "refuse" => "rejected",
        "defer" => "deferred",
        "escalate" => "escalated",
        other => {
            return Err(anyhow!(
                "control-path mediation cannot classify unknown freedom-gate decision '{}'",
                other
            ))
        }
    };
    if mediation.mediation.mediation_outcome != expected_mediation_outcome {
        return Err(anyhow!(
            "control-path mediation outcome '{}' does not match freedom_gate '{}'",
            mediation.mediation.mediation_outcome,
            freedom_gate.gate_decision
        ));
    }
    if mediation.mediation.decision_id != "decision.commitment_gate" {
        return Err(anyhow!(
            "control-path mediation decision_id '{}' must reference decision.commitment_gate",
            mediation.mediation.decision_id
        ));
    }
    if mediation.mediation.temporal_anchor != "control_path/freedom_gate.json" {
        return Err(anyhow!(
            "control-path mediation temporal anchor '{}' must point at control_path/freedom_gate.json",
            mediation.mediation.temporal_anchor
        ));
    }
    if mediation.mediation.judgment_boundary != freedom_gate.judgment_boundary {
        return Err(anyhow!(
            "control-path mediation judgment_boundary '{}' does not match freedom_gate '{}'",
            mediation.mediation.judgment_boundary,
            freedom_gate.judgment_boundary
        ));
    }
    if mediation.mediation.required_follow_up != freedom_gate.required_follow_up {
        return Err(anyhow!(
            "control-path mediation required_follow_up '{}' does not match freedom_gate '{}'",
            mediation.mediation.required_follow_up,
            freedom_gate.required_follow_up
        ));
    }
    if expected_mediation_outcome == "approved" {
        if mediation.mediation.approved_action_or_none.is_none() {
            return Err(anyhow!(
                "control-path mediation must carry approved_action_or_none when outcome is approved"
            ));
        }
    } else if mediation.mediation.approved_action_or_none.is_some() {
        return Err(anyhow!(
            "control-path mediation must not carry approved_action_or_none when outcome is not approved"
        ));
    }

    let expected_skill_schema_fields = vec![
        "skill_id".to_string(),
        "selection_status".to_string(),
        "purpose".to_string(),
        "bounded_role".to_string(),
        "input_contract_fields".to_string(),
        "output_contract_surfaces".to_string(),
        "stop_condition".to_string(),
        "distinguished_from".to_string(),
        "temporal_anchor".to_string(),
    ];
    if skill_model.skill_schema_fields != expected_skill_schema_fields {
        return Err(anyhow!(
            "control-path skill model schema fields mismatch: expected {:?}, found {:?}",
            expected_skill_schema_fields,
            skill_model.skill_schema_fields
        ));
    }
    let expected_distinction_vocabulary = vec![
        "skill".to_string(),
        "provider_capability".to_string(),
        "raw_aptitude".to_string(),
        "tool_call".to_string(),
        "memory_operation".to_string(),
        "final_answer".to_string(),
    ];
    if skill_model.distinction_vocabulary != expected_distinction_vocabulary {
        return Err(anyhow!(
            "control-path skill model distinction vocabulary mismatch: expected {:?}, found {:?}",
            expected_distinction_vocabulary,
            skill_model.distinction_vocabulary
        ));
    }
    if skill_model.selected_execution_unit_kind != proposal.kind {
        return Err(anyhow!(
            "control-path skill model selected_execution_unit_kind '{}' does not match proposal '{}'",
            skill_model.selected_execution_unit_kind,
            proposal.kind
        ));
    }
    let expected_selection_status = if proposal.kind == "skill_call" {
        "selected"
    } else {
        "not_selected"
    };
    if skill_model.skill.selection_status != expected_selection_status {
        return Err(anyhow!(
            "control-path skill model selection_status '{}' does not match expected '{}'",
            skill_model.skill.selection_status,
            expected_selection_status
        ));
    }
    if skill_model.skill.temporal_anchor != "control_path/action_proposals.json" {
        return Err(anyhow!(
            "control-path skill model temporal anchor '{}' must point at control_path/action_proposals.json",
            skill_model.skill.temporal_anchor
        ));
    }
    let expected_skill_outputs = vec![
        "control_path/mediation.json".to_string(),
        "control_path/final_result.json".to_string(),
        "logs/trace_v1.json".to_string(),
    ];
    if skill_model.skill.output_contract_surfaces != expected_skill_outputs {
        return Err(anyhow!(
            "control-path skill model output surfaces mismatch: expected {:?}, found {:?}",
            expected_skill_outputs,
            skill_model.skill.output_contract_surfaces
        ));
    }
    let expected_input_contract_fields: Vec<String> = proposal.arguments.keys().cloned().collect();
    if skill_model.skill.input_contract_fields != expected_input_contract_fields {
        return Err(anyhow!(
            "control-path skill model input contract fields mismatch: expected {:?}, found {:?}",
            expected_input_contract_fields,
            skill_model.skill.input_contract_fields
        ));
    }

    let expected_protocol_stages = vec![
        "proposed".to_string(),
        "validated".to_string(),
        "authorized".to_string(),
        "trace_visible".to_string(),
        "ready_for_execution".to_string(),
    ];
    if skill_execution_protocol.lifecycle_stages != expected_protocol_stages {
        return Err(anyhow!(
            "control-path skill execution protocol stages mismatch: expected {:?}, found {:?}",
            expected_protocol_stages,
            skill_execution_protocol.lifecycle_stages
        ));
    }
    if skill_execution_protocol.invocation.proposal_id != proposal.proposal_id {
        return Err(anyhow!(
            "control-path skill execution protocol proposal '{}' does not match action proposal '{}'",
            skill_execution_protocol.invocation.proposal_id,
            proposal.proposal_id
        ));
    }
    if skill_execution_protocol.invocation.decision_id != mediation.mediation.decision_id {
        return Err(anyhow!(
            "control-path skill execution protocol decision '{}' does not match mediation '{}'",
            skill_execution_protocol.invocation.decision_id,
            mediation.mediation.decision_id
        ));
    }
    if skill_execution_protocol.invocation.invocation_kind != proposal.kind {
        return Err(anyhow!(
            "control-path skill execution protocol invocation_kind '{}' does not match proposal '{}'",
            skill_execution_protocol.invocation.invocation_kind,
            proposal.kind
        ));
    }
    if skill_execution_protocol.invocation.skill_id != skill_model.skill.skill_id {
        return Err(anyhow!(
            "control-path skill execution protocol skill_id '{}' does not match skill model '{}'",
            skill_execution_protocol.invocation.skill_id,
            skill_model.skill.skill_id
        ));
    }
    let expected_protocol_state = match mediation.mediation.mediation_outcome.as_str() {
        "approved" => "authorized_ready_for_execution",
        "rejected" => "rejected_before_execution",
        "deferred" => "deferred_before_execution",
        "escalated" => "escalated_before_execution",
        _ => "blocked_before_execution",
    };
    if skill_execution_protocol.invocation.lifecycle_state != expected_protocol_state {
        return Err(anyhow!(
            "control-path skill execution protocol lifecycle_state '{}' does not match expected '{}'",
            skill_execution_protocol.invocation.lifecycle_state,
            expected_protocol_state
        ));
    }
    if skill_execution_protocol.invocation.authorization_decision
        != mediation.mediation.mediation_outcome
    {
        return Err(anyhow!(
            "control-path skill execution protocol authorization_decision '{}' does not match mediation '{}'",
            skill_execution_protocol.invocation.authorization_decision,
            mediation.mediation.mediation_outcome
        ));
    }
    if skill_execution_protocol.invocation.output_contract_surfaces != expected_skill_outputs {
        return Err(anyhow!(
            "control-path skill execution protocol output surfaces mismatch: expected {:?}, found {:?}",
            expected_skill_outputs,
            skill_execution_protocol.invocation.output_contract_surfaces
        ));
    }
    let expected_error_outcomes = vec![
        "rejected".to_string(),
        "deferred".to_string(),
        "escalated".to_string(),
    ];
    if skill_execution_protocol.invocation.error_outcome_vocabulary != expected_error_outcomes {
        return Err(anyhow!(
            "control-path skill execution protocol error vocabulary mismatch: expected {:?}, found {:?}",
            expected_error_outcomes,
            skill_execution_protocol.invocation.error_outcome_vocabulary
        ));
    }
    if skill_execution_protocol.invocation.trace_expectation
        != mediation.mediation.trace_expectation
    {
        return Err(anyhow!(
            "control-path skill execution protocol trace expectation '{}' does not match mediation '{}'",
            skill_execution_protocol.invocation.trace_expectation,
            mediation.mediation.trace_expectation
        ));
    }
    if skill_execution_protocol.invocation.temporal_anchor != "control_path/mediation.json" {
        return Err(anyhow!(
            "control-path skill execution protocol temporal anchor '{}' must point at control_path/mediation.json",
            skill_execution_protocol.invocation.temporal_anchor
        ));
    }

    let expected_surface_ids = [
        "delegation_and_routing.route_selection",
        "recovery_continuity.reframing",
        "pre_execution_authorization.commitment_gate",
    ];
    for expected_surface_id in expected_surface_ids {
        let Some(surface) = decisions
            .surfaces
            .iter()
            .find(|surface| surface.surface_id == expected_surface_id)
        else {
            return Err(anyhow!(
                "control-path decisions artifact is missing surface '{}'",
                expected_surface_id
            ));
        };
        let Some(record) = decisions
            .decisions
            .iter()
            .find(|record| record.surface_id == expected_surface_id)
        else {
            return Err(anyhow!(
                "control-path decisions artifact is missing decision record for '{}'",
                expected_surface_id
            ));
        };
        if record.temporal_anchor != surface.temporal_anchor_ref {
            return Err(anyhow!(
                "control-path decision temporal anchor '{}' does not match surface anchor '{}'",
                record.temporal_anchor,
                surface.temporal_anchor_ref
            ));
        }
        if !decisions
            .outcome_class_vocabulary
            .contains(&record.outcome_class)
        {
            return Err(anyhow!(
                "control-path decision outcome '{}' is not in the declared vocabulary",
                record.outcome_class
            ));
        }
    }

    let required_summary_markers = [
        "stage_order: signals -> candidate_selection -> arbitration -> execution -> evaluation -> reframing -> memory -> freedom_gate -> final_result".to_string(),
        format!("candidate_selection: candidate_id={}", agency.selected_candidate_id),
        format!("arbitration: route={}", arbitration.route_selected),
        format!("evaluation: termination_reason={}", evaluation.termination_reason),
        format!("reframing: trigger={}", reframing.reframing_trigger),
        format!(
            "decisions: route_selection={} reframing={} commitment_gate={}",
            decisions.decisions[0].outcome_class,
            decisions.decisions[1].outcome_class,
            decisions.decisions[2].outcome_class
        ),
        format!(
            "action_proposal: kind={} target={} requires_approval={}",
            proposal.kind,
            proposal.target.as_deref().unwrap_or("<none>"),
            proposal.requires_approval
        ),
        format!(
            "action_mediation: outcome={} authority={} follow_up={}",
            mediation.mediation.mediation_outcome,
            mediation.mediation.runtime_authority,
            mediation.mediation.required_follow_up
        ),
        format!(
            "skill_model: selection_status={} skill_id={} invocation_kind={}",
            skill_model.skill.selection_status,
            skill_model.skill.skill_id,
            skill_model.selected_execution_unit_kind
        ),
        format!(
            "skill_execution_protocol: lifecycle_state={} authorization={} trace_expectation={}",
            skill_execution_protocol.invocation.lifecycle_state,
            skill_execution_protocol.invocation.authorization_decision,
            skill_execution_protocol.invocation.trace_expectation
        ),
        format!(
            "security_review: posture={} trust_state={} attacker_pressure={}",
            security_review.posture.declared_posture,
            security_review.trust_under_adversary.trust_state,
            security_review.threat_model.attacker_pressure
        ),
        format!("freedom_gate: decision={}", freedom_gate.gate_decision),
        format!(
            "convergence: state={} stop_condition_family={} progress_signal={}",
            convergence.convergence_state,
            convergence.stop_condition_family,
            convergence.progress_signal
        ),
        format!("final_result: {}", final_result.final_result),
    ];
    for marker in required_summary_markers {
        if !summary.contains(&marker) {
            return Err(anyhow!(
                "control-path summary '{}' is missing required marker '{}'",
                summary_path.display(),
                marker
            ));
        }
    }

    Ok(())
}

pub(crate) fn load_resume_state(
    path: &Path,
    resolved: &resolve::AdlResolved,
) -> Result<execute::ResumeState> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read resume state '{}'", path.display()))?;
    let artifact: RunStateArtifact = serde_json::from_str(&raw).with_context(|| {
        format!(
            "failed to parse resume state '{}' as run_state artifact",
            path.display()
        )
    })?;

    if artifact.schema_version != RUN_STATE_SCHEMA_VERSION {
        return Err(anyhow::anyhow!(
            "resume state schema_version mismatch: state='{}' expected='{}'",
            artifact.schema_version,
            RUN_STATE_SCHEMA_VERSION
        ));
    }

    if artifact.schema_version != RUN_STATE_SCHEMA_VERSION {
        return Err(anyhow::anyhow!(
            "resume state schema_version mismatch: state='{}' expected='{}'",
            artifact.schema_version,
            RUN_STATE_SCHEMA_VERSION
        ));
    }

    if artifact.status != "paused" {
        return Err(anyhow::anyhow!(
            "resume state must have status='paused' (found='{}' for run_id='{}' in '{}')",
            artifact.status,
            artifact.run_id,
            path.display()
        ));
    }
    if artifact.run_id != resolved.run_id {
        return Err(anyhow::anyhow!(
            "resume run_id mismatch: state='{}' current='{}'",
            artifact.run_id,
            resolved.run_id
        ));
    }
    if artifact.workflow_id != resolved.workflow_id {
        return Err(anyhow::anyhow!(
            "resume workflow_id mismatch for run_id='{}' in '{}': state='{}' current='{}'",
            artifact.run_id,
            path.display(),
            artifact.workflow_id,
            resolved.workflow_id
        ));
    }
    if artifact.version != resolved.doc.version {
        return Err(anyhow::anyhow!(
            "resume version mismatch for run_id='{}' in '{}': state='{}' current='{}'",
            artifact.run_id,
            path.display(),
            artifact.version,
            resolved.doc.version
        ));
    }
    let plan_hash = execution_plan_hash(&resolved.execution_plan)?;
    if artifact.execution_plan_hash != plan_hash {
        return Err(anyhow::anyhow!(
            "resume execution plan mismatch for run_id='{}' in '{}'; state plan != current plan (resume requires identical plan + ordering)",
            artifact.run_id,
            path.display()
        ));
    }
    let pause = artifact
        .pause
        .ok_or_else(|| anyhow::anyhow!("resume state missing pause payload"))?;

    let completed_step_ids = pause.completed_step_ids.into_iter().collect();
    Ok(execute::ResumeState {
        completed_step_ids,
        saved_state: pause.saved_state,
        completed_outputs: pause.completed_outputs,
        steering_history: artifact.steering_history,
    })
}

pub(crate) fn resume_state_path_for_run_id(run_id: &str) -> Result<PathBuf> {
    Ok(artifacts::RunArtifactPaths::for_run(run_id)?.pause_state_json())
}

pub(crate) fn load_pause_state_artifact(path: &Path) -> Result<PauseStateArtifact> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read pause state '{}'", path.display()))?;
    let artifact: PauseStateArtifact =
        serde_json::from_str(&raw).with_context(|| "failed to parse pause_state.json")?;
    Ok(artifact)
}

pub(crate) fn load_steering_patch(path: &Path) -> Result<(execute::SteeringPatch, String)> {
    let raw = std::fs::read(path)
        .with_context(|| format!("failed to read steering patch '{}'", path.display()))?;
    let fingerprint = stable_fingerprint_hex(&raw);
    let patch: execute::SteeringPatch =
        serde_json::from_slice(&raw).with_context(|| "failed to parse steering patch JSON")?;
    execute::validate_steering_patch(&patch)?;
    Ok((patch, fingerprint))
}

pub(crate) fn validate_pause_artifact_basic(
    artifact: &PauseStateArtifact,
    run_id: &str,
) -> Result<()> {
    if artifact.schema_version != PAUSE_STATE_SCHEMA_VERSION {
        return Err(anyhow::anyhow!(
            "pause state schema_version mismatch: state='{}' expected='{}'",
            artifact.schema_version,
            PAUSE_STATE_SCHEMA_VERSION
        ));
    }
    if artifact.status != "paused" {
        return Err(anyhow::anyhow!(
            "pause state must have status='paused' (found '{}')",
            artifact.status
        ));
    }
    if artifact.run_id != run_id {
        return Err(anyhow::anyhow!(
            "pause state run_id mismatch: state='{}' requested='{}'",
            artifact.run_id,
            run_id
        ));
    }
    Ok(())
}

pub(crate) fn validate_pause_artifact_for_resume(
    artifact: &PauseStateArtifact,
    run_id: &str,
    resolved: &resolve::AdlResolved,
) -> Result<()> {
    validate_pause_artifact_basic(artifact, run_id)?;
    if artifact.run_id != resolved.run_id {
        return Err(anyhow::anyhow!(
            "resume run_id mismatch: state='{}' current='{}'",
            artifact.run_id,
            resolved.run_id
        ));
    }
    if artifact.workflow_id != resolved.workflow_id {
        return Err(anyhow::anyhow!(
            "resume workflow_id mismatch: state='{}' current='{}'",
            artifact.workflow_id,
            resolved.workflow_id
        ));
    }
    if artifact.version != resolved.doc.version {
        return Err(anyhow::anyhow!(
            "resume version mismatch: state='{}' current='{}'",
            artifact.version,
            resolved.doc.version
        ));
    }
    let plan_hash = execution_plan_hash(&resolved.execution_plan)?;
    if artifact.execution_plan_hash != plan_hash {
        return Err(anyhow::anyhow!(
            "resume execution plan hash mismatch; resume requires identical plan and ordering"
        ));
    }
    Ok(())
}
