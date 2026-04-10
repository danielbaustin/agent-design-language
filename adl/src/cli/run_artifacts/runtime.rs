use super::cognitive::{
    build_aee_decision_artifact, build_affect_state_artifact, build_agency_selection_artifact,
    build_bounded_execution_artifact, build_cognitive_arbitration_artifact_from_state,
    build_cognitive_signals_artifact_from_state, build_control_path_final_result_artifact,
    build_control_path_memory_artifact, build_control_path_summary,
    build_evaluation_signals_artifact, build_fast_slow_path_artifact, build_freedom_gate_artifact,
    build_memory_read_artifact, build_memory_write_artifact, build_reasoning_graph_artifact,
    build_reframing_artifact,
};
use super::summary::{
    build_cluster_groundwork_artifact, build_scores_artifact, build_suggestions_artifact,
};
use super::ControlPathSummaryContext;
use super::*;
use ::adl::trace_schema_v1::{
    validate_trace_event_envelope_v1, ContractValidationResultV1, TraceActorTypeV1, TraceActorV1,
    TraceContractValidationV1, TraceDecisionContextV1, TraceErrorV1, TraceEventEnvelopeV1,
    TraceEventTypeV1, TraceEventV1, TraceScopeLevelV1, TraceScopeV1,
};
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
    let control_path_final_result = build_control_path_final_result_artifact(
        &run_summary,
        &cognitive_arbitration,
        &agency_selection,
        &evaluation_signals,
        &freedom_gate,
    );
    let control_path_summary = build_control_path_summary(&ControlPathSummaryContext {
        signals: &cognitive_signals,
        agency: &agency_selection,
        arbitration: &cognitive_arbitration,
        execution: &bounded_execution,
        evaluation: &evaluation_signals,
        reframing: &reframing,
        memory: &control_path_memory,
        freedom_gate: &freedom_gate,
        final_result: &control_path_final_result,
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
    let memory_read_json =
        serde_json::to_vec_pretty(&memory_read).context("serialize memory_read.v1.json")?;
    let memory_write_json =
        serde_json::to_vec_pretty(&memory_write).context("serialize memory_write.v1.json")?;
    let control_path_memory_json = serde_json::to_vec_pretty(&control_path_memory)
        .context("serialize control_path memory.json")?;
    let control_path_final_result_json = serde_json::to_vec_pretty(&control_path_final_result)
        .context("serialize control_path final_result.json")?;
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
        &run_paths.control_path_freedom_gate_json(),
        &freedom_gate_json,
    )?;
    artifacts::atomic_write(
        &run_paths.control_path_final_result_json(),
        &control_path_final_result_json,
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
    let freedom_gate: FreedomGateArtifact =
        read_required_json_artifact(control_path_dir, "freedom_gate.json")?;
    let final_result: ControlPathFinalResultArtifact =
        read_required_json_artifact(control_path_dir, "final_result.json")?;

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
        freedom_gate.run_id.as_str(),
        final_result.run_id.as_str(),
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

    let required_summary_markers = [
        "stage_order: signals -> candidate_selection -> arbitration -> execution -> evaluation -> reframing -> memory -> freedom_gate -> final_result".to_string(),
        format!("candidate_selection: candidate_id={}", agency.selected_candidate_id),
        format!("arbitration: route={}", arbitration.route_selected),
        format!("evaluation: termination_reason={}", evaluation.termination_reason),
        format!("reframing: trigger={}", reframing.reframing_trigger),
        format!("freedom_gate: decision={}", freedom_gate.gate_decision),
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
