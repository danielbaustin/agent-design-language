use super::super::cognitive::{
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
};
use super::super::summary::{
    build_cluster_groundwork_artifact, build_scores_artifact, build_suggestions_artifact,
};
use super::super::ControlPathSummaryContext;
use super::super::*;
use super::trace_envelope::build_trace_v1_envelope;
use ::adl::runtime_environment::RuntimeEnvironment;
use serde::Serialize;
use serde_json::{json, Value as JsonValue};
use std::collections::BTreeMap;

fn governed_trace_artifacts_for_run(tr: &trace::Trace) -> (Option<JsonValue>, Option<JsonValue>) {
    let mut proposal_redaction_summaries = BTreeMap::new();
    let mut proposal_redaction_details = BTreeMap::new();
    for event in &tr.events {
        match event {
            trace::TraceEvent::GovernedFreedomGateDecided {
                proposal_id,
                redaction_summary,
                ..
            } => {
                proposal_redaction_summaries
                    .entry(proposal_id.clone())
                    .or_insert_with(|| redaction_summary.clone());
            }
            trace::TraceEvent::GovernedRedactionDecisionRecorded {
                proposal_id,
                detail,
                ..
            } => {
                proposal_redaction_details
                    .entry(proposal_id.clone())
                    .or_insert_with(|| detail.clone());
            }
            _ => {}
        }
    }

    let mut proposal_entries = Vec::new();
    let mut result_entries = Vec::new();

    for event in &tr.events {
        match event {
            trace::TraceEvent::GovernedProposalObserved {
                proposal_id,
                tool_name,
                redacted_arguments_ref,
                ..
            } => {
                proposal_entries.push(json!({
                    "proposal_id": proposal_id,
                    "tool_name": tool_name,
                    "redacted_arguments_ref": redacted_arguments_ref,
                    "redaction": {
                        "status": "redacted",
                        "detail": proposal_redaction_details
                            .get(proposal_id)
                            .cloned()
                            .flatten(),
                        "summary": proposal_redaction_summaries.get(proposal_id).cloned(),
                    }
                }));
            }
            trace::TraceEvent::GovernedExecutionResultRecorded {
                proposal_id,
                action_id,
                adapter_id,
                result_ref,
                evidence_refs,
                ..
            } => {
                result_entries.push(json!({
                    "proposal_id": proposal_id,
                    "action_id": action_id,
                    "adapter_id": adapter_id,
                    "result_ref": result_ref,
                    "result_status": "redacted",
                    "evidence_refs": evidence_refs,
                }));
            }
            _ => {}
        }
    }

    let proposal_artifact = (!proposal_entries.is_empty()).then(|| {
        json!({
            "schema_version": "governed_trace_arguments.v1",
            "run_id": tr.run_id.clone(),
            "entries": proposal_entries,
        })
    });
    let result_artifact = (!result_entries.is_empty()).then(|| {
        json!({
            "schema_version": "governed_trace_results.v1",
            "run_id": tr.run_id.clone(),
            "entries": result_entries,
        })
    });

    (proposal_artifact, result_artifact)
}

pub(crate) fn write_governed_trace_artifacts_for_run_paths(
    run_paths: &artifacts::RunArtifactPaths,
    tr: &trace::Trace,
) -> Result<()> {
    let governed_dir = run_paths.run_dir().join("governed");
    std::fs::create_dir_all(&governed_dir).context("create governed artifact dir")?;

    let (proposal_artifact, result_artifact) = governed_trace_artifacts_for_run(tr);
    if let Some(proposal_artifact) = proposal_artifact {
        artifacts::atomic_write(
            &governed_dir.join("proposal_arguments.redacted.json"),
            &serde_json::to_vec_pretty(&proposal_artifact)
                .context("serialize governed proposal arguments artifact")?,
        )?;
    }
    if let Some(result_artifact) = result_artifact {
        artifacts::atomic_write(
            &governed_dir.join("result.redacted.json"),
            &serde_json::to_vec_pretty(&result_artifact)
                .context("serialize governed result artifact")?,
        )?;
    }
    Ok(())
}

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
    write_governed_trace_artifacts_for_run_paths(&run_paths, tr)?;
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
