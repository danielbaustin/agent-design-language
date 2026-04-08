use super::*;

use crate::bounded_executor;
use crate::delegation_policy::{self, DelegationDecision};
use crate::prompt;
use crate::provider;
use crate::remote_exec;

#[derive(Debug)]
pub(super) struct StepRunSuccess {
    pub(super) out: StepOutput,
    pub(super) attempts: u32,
    pub(super) prompt_hash: String,
    pub(super) stream_chunks: Vec<String>,
}

#[derive(Debug)]
pub(super) struct StepRunFailure {
    pub(super) err: anyhow::Error,
    pub(super) attempts: u32,
    pub(super) stream_chunks: Vec<String>,
}

type StepJob = Box<dyn FnOnce() -> (String, Result<StepRunSuccess>) + Send>;

pub const DELEGATION_POLICY_DENY_CODE: &str = "DELEGATION_POLICY_DENY";
pub const DELEGATION_POLICY_APPROVAL_REQUIRED_CODE: &str = "DELEGATION_POLICY_APPROVAL_REQUIRED";

fn effective_prompt_with_defaults_from_doc(
    step: &crate::resolve::ResolvedStep,
    doc: &crate::adl::AdlDoc,
) -> Option<crate::adl::PromptSpec> {
    let mut p = if let Some(p) = step.prompt.as_ref() {
        p.clone()
    } else if let Some(task_key) = step.task.as_ref() {
        doc.tasks.get(task_key).map(|t| t.prompt.clone())?
    } else if let Some(agent_key) = step.agent.as_ref() {
        doc.agents.get(agent_key).and_then(|a| a.prompt.clone())?
    } else {
        return None;
    };
    if p.system.is_none() {
        if let Some(default_system) = doc.run.defaults.system.as_ref() {
            p.system = Some(default_system.clone());
        }
    }
    Some(p)
}

pub(super) fn effective_step_placement(
    step: &crate::resolve::ResolvedStep,
    doc: &crate::adl::AdlDoc,
) -> crate::adl::PlacementMode {
    step.placement
        .clone()
        .or_else(|| doc.run.placement.as_ref().and_then(|p| p.mode()))
        .unwrap_or(crate::adl::PlacementMode::Local)
}

fn delegation_trace_target(
    step: &crate::resolve::ResolvedStep,
    doc: &crate::adl::AdlDoc,
) -> Option<(&'static str, String)> {
    step.delegation.as_ref()?;
    let provider_id = step
        .provider
        .clone()
        .unwrap_or_else(|| "<unresolved-provider>".to_string());
    let action_kind = match effective_step_placement(step, doc) {
        crate::adl::PlacementMode::Local => "provider_call",
        crate::adl::PlacementMode::Remote => "remote_exec",
    };
    Some((action_kind, provider_id))
}

pub(super) fn emit_delegation_lifecycle_start(
    tr: &mut Trace,
    step: &crate::resolve::ResolvedStep,
    doc: &crate::adl::AdlDoc,
) {
    if let Some((action_kind, target_id)) = delegation_trace_target(step, doc) {
        tr.delegation_requested(&step.id, action_kind, &target_id);
        tr.delegation_policy_evaluated(&step.id, action_kind, &target_id, "allowed", None);
        tr.delegation_dispatched(&step.id, action_kind, &target_id);
    }
}

pub(super) fn emit_delegation_lifecycle_finish(
    tr: &mut Trace,
    step: &crate::resolve::ResolvedStep,
    doc: &crate::adl::AdlDoc,
    success: bool,
    output_bytes: usize,
) {
    if delegation_trace_target(step, doc).is_some() {
        tr.delegation_result_received(&step.id, success, output_bytes);
        tr.delegation_completed(&step.id, if success { "success" } else { "failure" });
    }
}

pub(super) fn effective_max_concurrency_with_source(
    resolved: &AdlResolved,
) -> Result<(usize, SchedulerPolicySource)> {
    let workflow_override = if resolved.doc.run.pattern_ref.is_some() {
        None
    } else {
        Some(
            resolved
                .doc
                .run
                .resolve_workflow(&resolved.doc)
                .context("resolve workflow for max_concurrency precedence")?
                .max_concurrency,
        )
        .flatten()
    };

    let (max_parallel, source) = if let Some(v) = workflow_override {
        (v, SchedulerPolicySource::WorkflowOverride)
    } else if let Some(v) = resolved.doc.run.defaults.max_concurrency {
        (v, SchedulerPolicySource::RunDefault)
    } else {
        (
            DEFAULT_MAX_CONCURRENCY,
            SchedulerPolicySource::EngineDefault,
        )
    };

    if max_parallel == 0 {
        return Err(anyhow!(
            "effective max_concurrency must be >= 1 for concurrent runs"
        ));
    }

    Ok((max_parallel, source))
}

pub fn scheduler_policy_for_run(
    resolved: &AdlResolved,
) -> Result<Option<(usize, SchedulerPolicySource)>> {
    let is_concurrent = matches!(
        resolved.execution_plan.workflow_kind,
        crate::adl::WorkflowKind::Concurrent
    );
    if !is_concurrent {
        return Ok(None);
    }
    let (max_parallel, source) = effective_max_concurrency_with_source(resolved)?;
    Ok(Some((max_parallel, source)))
}

fn enforce_delegation_policy(
    tr: &mut Trace,
    step: &crate::resolve::ResolvedStep,
    doc: &crate::adl::AdlDoc,
    action: crate::adl::DelegationActionKind,
    target_id: &str,
) -> Result<()> {
    let outcome = delegation_policy::evaluate(
        doc.run.delegation_policy.as_ref(),
        step.delegation.as_ref(),
        action.clone(),
        target_id,
    );

    match outcome.decision {
        DelegationDecision::Allowed => Ok(()),
        DelegationDecision::NeedsApproval => {
            tr.delegation_policy_evaluated(
                &step.id,
                action.as_str(),
                target_id,
                outcome.decision.as_str(),
                outcome.rule_id.as_deref(),
            );
            Err(ExecutionPolicyError {
                kind: ExecutionPolicyErrorKind::ApprovalRequired,
                step_id: step.id.clone(),
                action_kind: action.as_str().to_string(),
                target_id: target_id.to_string(),
                rule_id: outcome.rule_id,
            }
            .into())
        }
        DelegationDecision::Denied => {
            tr.delegation_policy_evaluated(
                &step.id,
                action.as_str(),
                target_id,
                outcome.decision.as_str(),
                outcome.rule_id.as_deref(),
            );
            tr.delegation_denied(
                &step.id,
                action.as_str(),
                target_id,
                outcome.rule_id.as_deref(),
            );
            Err(ExecutionPolicyError {
                kind: ExecutionPolicyErrorKind::Denied,
                step_id: step.id.clone(),
                action_kind: action.as_str().to_string(),
                target_id: target_id.to_string(),
                rule_id: outcome.rule_id,
            }
            .into())
        }
    }
}

pub(super) fn enforce_delegation_policy_for_step_actions(
    tr: &mut Trace,
    step: &crate::resolve::ResolvedStep,
    doc: &crate::adl::AdlDoc,
) -> Result<()> {
    let provider_id: &str = step.provider.as_deref().unwrap_or("<unresolved-provider>");
    match effective_step_placement(step, doc) {
        crate::adl::PlacementMode::Local => enforce_delegation_policy(
            tr,
            step,
            doc,
            crate::adl::DelegationActionKind::ProviderCall,
            provider_id,
        )?,
        crate::adl::PlacementMode::Remote => enforce_delegation_policy(
            tr,
            step,
            doc,
            crate::adl::DelegationActionKind::RemoteExec,
            "run.remote.endpoint",
        )?,
    }

    if step.write_to.is_some() {
        let target = step.write_to.as_deref().unwrap_or("<none>");
        enforce_delegation_policy(
            tr,
            step,
            doc,
            crate::adl::DelegationActionKind::FilesystemWrite,
            target,
        )?;
    }

    if step
        .inputs
        .values()
        .any(|v| v.trim_start().starts_with("@file:"))
    {
        enforce_delegation_policy(
            tr,
            step,
            doc,
            crate::adl::DelegationActionKind::FilesystemRead,
            "@file",
        )?;
    }

    Ok(())
}

pub(super) fn execute_step_with_retry(
    step: &crate::resolve::ResolvedStep,
    doc: &crate::adl::AdlDoc,
    run_id: &str,
    workflow_id: &str,
    saved_state: &HashMap<String, String>,
    adl_base_dir: &Path,
    capture_stream_chunks: bool,
) -> Result<StepRunSuccess> {
    match execute_step_with_retry_core(
        step,
        doc,
        run_id,
        workflow_id,
        saved_state,
        adl_base_dir,
        capture_stream_chunks,
        |_| {},
    ) {
        Ok(success) => Ok(success),
        Err(failure) => Err(failure.err),
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn execute_step_with_retry_core<F>(
    step: &crate::resolve::ResolvedStep,
    doc: &crate::adl::AdlDoc,
    run_id: &str,
    workflow_id: &str,
    saved_state: &HashMap<String, String>,
    adl_base_dir: &Path,
    capture_stream_chunks: bool,
    mut on_prompt_hash: F,
) -> std::result::Result<StepRunSuccess, StepRunFailure>
where
    F: FnMut(&str),
{
    let step_id = step.id.clone();
    let provider_id: &str = step.provider.as_deref().unwrap_or("<unresolved-provider>");
    let max_attempts = step.retry.as_ref().map(|r| r.max_attempts).unwrap_or(1);
    let mut attempt: u32 = 0;
    let mut last_err: Option<anyhow::Error> = None;
    let mut last_stream_chunks: Vec<String> = Vec::new();

    while attempt < max_attempts {
        attempt += 1;
        let mut attempt_stream_chunks: Vec<String> = Vec::new();
        let result = (|| -> Result<StepRunSuccess> {
            let p = effective_prompt_with_defaults_from_doc(step, doc).ok_or_else(|| {
                anyhow!(
                    "step '{}' has no effective prompt (step.prompt or task.prompt required)",
                    step_id
                )
            })?;

            let merged_inputs = resolve_state_inputs(&step.id, &step.inputs, saved_state)
                .with_context(|| format!("failed to resolve inputs for step '{}'", step_id))?;
            let missing = missing_prompt_inputs(&p, &merged_inputs);
            if !missing.is_empty() {
                return Err(anyhow!(
                    "step '{}' missing input bindings for: {} (provide inputs or prior state)",
                    step_id,
                    missing.join(", ")
                ));
            }

            let inputs = materialize_inputs(merged_inputs, adl_base_dir)
                .with_context(|| format!("failed to materialize inputs for step '{}'", step_id))?;

            let prompt_text = prompt::trace_prompt_assembly(&p, &inputs);
            let prompt_hash = prompt::hash_prompt(&prompt_text);
            on_prompt_hash(&prompt_hash);

            let spec = doc.providers.get(provider_id).with_context(|| {
                format!(
                    "step '{}' references unknown provider '{}'",
                    step_id, provider_id
                )
            })?;
            let model_override = step
                .agent
                .as_ref()
                .and_then(|agent_id| doc.agents.get(agent_id))
                .map(|agent| agent.model.as_str());

            let placement = effective_step_placement(step, doc);

            let model_output = match placement {
                crate::adl::PlacementMode::Local => {
                    let prov = provider::build_provider_for_id(provider_id, spec, model_override)
                        .with_context(|| {
                        format!(
                            "failed to build provider '{}' for step '{}'",
                            provider_id, step_id
                        )
                    })?;
                    let mut on_chunk = |chunk: &str| {
                        if capture_stream_chunks && !chunk.is_empty() {
                            attempt_stream_chunks.push(chunk.to_string());
                        }
                    };
                    prov.complete_stream(&prompt_text, &mut on_chunk).with_context(|| {
                        format!(
                            "provider '{}' complete() failed for step '{}' (attempt {attempt}/{max_attempts})",
                            provider_id, step_id
                        )
                    })?
                }
                crate::adl::PlacementMode::Remote => {
                    let remote = doc.run.remote.as_ref().ok_or_else(|| {
                        crate::remote_exec::RemoteExecuteClientError::new(
                            crate::remote_exec::RemoteExecuteClientErrorKind::SchemaViolation,
                            "REMOTE_SCHEMA_VIOLATION",
                            "run.remote.endpoint is required when placement=remote",
                        )
                    })?;
                    let timeout_ms = remote.timeout_ms.unwrap_or(30_000);
                    let mut req = remote_exec::ExecuteRequest {
                        protocol_version: remote_exec::PROTOCOL_VERSION.to_string(),
                        run_id: run_id.to_string(),
                        workflow_id: workflow_id.to_string(),
                        step_id: step_id.clone(),
                        step: remote_exec::ExecuteStepPayload {
                            kind: "task".to_string(),
                            provider: provider_id.to_string(),
                            prompt: prompt_text.clone(),
                            tools: Vec::new(),
                            provider_spec: spec.clone(),
                            model_override: model_override.map(|v| v.to_string()),
                        },
                        inputs: remote_exec::ExecuteInputsPayload {
                            inputs: inputs.clone(),
                            state: saved_state.clone(),
                        },
                        timeout_ms,
                        security: Some(remote_exec::ExecuteSecurityEnvelope {
                            require_signature: remote.require_signed_requests,
                            require_key_id: remote.require_key_id,
                            signed: doc.signature.is_some(),
                            key_id: doc.signature.as_ref().map(|s| s.key_id.clone()),
                            signature_alg: doc.signature.as_ref().map(|s| s.alg.clone()),
                            key_source: doc.signature.as_ref().and_then(|s| {
                                s.public_key_b64.as_ref().map(|_| "embedded".to_string())
                            }),
                            request_signature: None,
                            allowed_algs: remote.verify_allowed_algs.clone(),
                            allowed_key_sources: remote.verify_allowed_key_sources.clone(),
                            sandbox_root: Some(adl_base_dir.display().to_string()),
                            requested_paths: step
                                .write_to
                                .as_ref()
                                .map(|w| vec![w.clone()])
                                .unwrap_or_default(),
                        }),
                    };
                    remote_exec::maybe_attach_request_signature_from_env(&mut req).with_context(
                        || {
                            format!(
                                "failed to attach remote request signature for step '{}'",
                                step_id
                            )
                        },
                    )?;
                    remote_exec::execute_remote(&remote.endpoint, timeout_ms, &req)
                        .with_context(|| format!("remote step '{}' execution failed", step_id))?
                }
            };

            Ok(StepRunSuccess {
                out: StepOutput {
                    step_id: step_id.clone(),
                    provider_id: provider_id.to_string(),
                    model_output,
                },
                attempts: attempt,
                prompt_hash,
                stream_chunks: attempt_stream_chunks.clone(),
            })
        })();

        match result {
            Ok(success) => return std::result::Result::Ok(success),
            Err(err) => {
                last_stream_chunks = attempt_stream_chunks;
                let retryable = provider::is_retryable_error(&err);
                last_err = Some(err);
                if !retryable || attempt >= max_attempts {
                    break;
                }
            }
        }
    }

    std::result::Result::Err(StepRunFailure {
        err: last_err.unwrap_or_else(|| anyhow!("step '{}' failed", step_id)),
        attempts: attempt.max(1),
        stream_chunks: last_stream_chunks,
    })
}

pub(super) fn execute_concurrent_deterministic(
    resolved: &AdlResolved,
    tr: &mut Trace,
    print_outputs: bool,
    emit_progress: bool,
    adl_base_dir: &Path,
    out_dir: &Path,
    resume: Option<&ResumeState>,
) -> Result<ExecutionResult> {
    let (max_parallel, scheduler_source) = effective_max_concurrency_with_source(resolved)?;
    tr.scheduler_policy(max_parallel, scheduler_source.as_str());

    let mut outs = Vec::new();
    let mut artifacts = Vec::new();
    let mut records = Vec::new();
    let mut progress_started_ms: HashMap<String, u128> = HashMap::new();
    let mut saved_state: HashMap<String, String> =
        resume.map(|r| r.saved_state.clone()).unwrap_or_default();
    let mut completed_outputs: HashMap<String, String> = resume
        .map(|r| r.completed_outputs.clone())
        .unwrap_or_default();
    let mut completed: HashSet<String> = resume
        .map(|r| r.completed_step_ids.clone())
        .unwrap_or_default();
    let steering_history = resume
        .map(|r| r.steering_history.clone())
        .unwrap_or_default();
    if let Some(resume) = resume {
        let mut validated_completed = HashSet::new();
        for step in &resolved.steps {
            if !resume.completed_step_ids.contains(&step.id) {
                continue;
            }
            match resume_disposition_for_step(step, out_dir, &completed_outputs)? {
                ResumeDisposition::Skip(reason) => {
                    emit_resume_note(emit_progress, &step.id, "skip", reason);
                    validated_completed.insert(step.id.clone());
                }
                ResumeDisposition::Rerun(reason) => {
                    emit_resume_note(emit_progress, &step.id, "rerun", reason);
                    if let Some(save_as) = step.save_as.as_ref() {
                        saved_state.remove(save_as);
                    }
                    completed_outputs.remove(&step.id);
                }
            }
        }
        completed = validated_completed;
    }
    let mut pending: HashSet<String> = resolved
        .execution_plan
        .nodes
        .iter()
        .map(|n| n.step_id.clone())
        .collect();
    for done in &completed {
        pending.remove(done);
    }
    let by_id: HashMap<String, crate::resolve::ResolvedStep> = resolved
        .steps
        .iter()
        .cloned()
        .map(|s| (s.id.clone(), s))
        .collect();

    while !pending.is_empty() {
        let mut ready_ids: Vec<String> = resolved
            .execution_plan
            .nodes
            .iter()
            .filter(|node| pending.contains(&node.step_id))
            .filter(|node| node.depends_on.iter().all(|dep| completed.contains(dep)))
            .map(|node| node.step_id.clone())
            .collect();
        ready_ids.sort();

        if ready_ids.is_empty() {
            let mut unresolved: Vec<String> = pending.iter().cloned().collect();
            unresolved.sort();
            tr.execution_boundary_crossed(ExecutionBoundary::RunCompletion, "failure");
            tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Teardown);
            return Err(anyhow!(
                "no dependency-ready steps remain (possible unsatisfied join/state deps): {}",
                unresolved.join(", ")
            ));
        }

        let state_snapshot = saved_state.clone();
        let doc_snapshot = resolved.doc.clone();
        let base_snapshot = adl_base_dir.to_path_buf();
        let run_id_snapshot = resolved.run_id.clone();
        let workflow_id_snapshot = resolved.workflow_id.clone();

        let batch_ids: Vec<String> = ready_ids.into_iter().take(max_parallel).collect();

        for step_id in &batch_ids {
            let step = by_id
                .get(step_id)
                .ok_or_else(|| anyhow!("execution plan references unknown step '{}'", step_id))?;
            let agent_id = step.agent.as_deref().unwrap_or("<unresolved-agent>");
            let task_id = step.task.as_deref().unwrap_or("<unresolved-task>");
            let provider_id = step.provider.as_deref().unwrap_or("<unresolved-provider>");
            enforce_delegation_policy_for_step_actions(tr, step, &resolved.doc)?;
            emit_delegation_lifecycle_start(tr, step, &resolved.doc);
            tr.step_started(
                step_id,
                agent_id,
                provider_id,
                task_id,
                step.delegation.as_ref(),
            );
            progress_started_ms.insert(step_id.clone(), tr.current_elapsed_ms());
            progress_step_start(emit_progress, tr, step_id, provider_id);
        }

        let mut jobs: Vec<StepJob> = Vec::new();
        for step_id in &batch_ids {
            let step_id_owned = step_id.clone();
            let step = by_id
                .get(step_id)
                .ok_or_else(|| anyhow!("execution plan references unknown step '{}'", step_id))?
                .clone();
            let state_snapshot = state_snapshot.clone();
            let doc_snapshot = doc_snapshot.clone();
            let base_snapshot = base_snapshot.clone();
            let run_id_snapshot = run_id_snapshot.clone();
            let workflow_id_snapshot = workflow_id_snapshot.clone();
            jobs.push(Box::new(move || {
                let run = execute_step_with_retry(
                    &step,
                    &doc_snapshot,
                    &run_id_snapshot,
                    &workflow_id_snapshot,
                    &state_snapshot,
                    &base_snapshot,
                    true,
                );
                (step_id_owned, run)
            }));
        }

        let results = bounded_executor::run_bounded(max_parallel, jobs)?;
        let mut batch_pause: Option<(String, Option<String>)> = None;
        for (step_id, run_result) in results {
            let step = by_id
                .get(&step_id)
                .ok_or_else(|| anyhow!("execution result references unknown step '{}'", step_id))?;
            pending.remove(&step_id);

            match run_result {
                Ok(success) => {
                    tr.prompt_assembled(&step_id, &success.prompt_hash);
                    emit_delegation_lifecycle_finish(
                        tr,
                        step,
                        &resolved.doc,
                        true,
                        success.out.model_output.len(),
                    );
                    tr.step_finished(&step_id, true);
                    let duration_ms = tr.current_elapsed_ms().saturating_sub(
                        progress_started_ms
                            .remove(&step_id)
                            .unwrap_or_else(|| tr.current_elapsed_ms()),
                    );
                    progress_step_done(emit_progress, tr, &step_id, true, duration_ms);

                    if let Some(write_to) = step.write_to.as_deref() {
                        let path =
                            write_output(&step_id, out_dir, write_to, &success.out.model_output)?;
                        println!(
                            "ARTIFACT step={} path={} bytes={}",
                            step_id,
                            path.display(),
                            success.out.model_output.len()
                        );
                        artifacts.push(path);
                    }

                    if print_outputs {
                        emit_step_output(
                            &step_id,
                            &success.out.model_output,
                            &success.stream_chunks,
                            tr,
                        );
                    }
                    records.push(StepExecutionRecord {
                        step_id: step_id.clone(),
                        provider_id: success.out.provider_id.clone(),
                        status: "success".to_string(),
                        attempts: success.attempts,
                        output_bytes: success.out.model_output.len(),
                    });
                    if let Some(save_as) = step.save_as.as_ref() {
                        saved_state.insert(save_as.clone(), success.out.model_output.clone());
                    }
                    completed_outputs.insert(
                        step_id.clone(),
                        model_output_fingerprint(&success.out.model_output),
                    );
                    outs.push(success.out);
                    completed.insert(step_id.clone());
                    if let Some(reason) = pause_reason_for_step(step) {
                        batch_pause = Some((step_id.clone(), reason));
                    }
                }
                Err(err) => {
                    tr.step_finished(&step_id, false);
                    let duration_ms = tr.current_elapsed_ms().saturating_sub(
                        progress_started_ms
                            .remove(&step_id)
                            .unwrap_or_else(|| tr.current_elapsed_ms()),
                    );
                    progress_step_done(emit_progress, tr, &step_id, false, duration_ms);
                    let provider_id = step
                        .provider
                        .clone()
                        .unwrap_or_else(|| "<unresolved-provider>".to_string());
                    records.push(StepExecutionRecord {
                        step_id: step_id.clone(),
                        provider_id,
                        status: "failure".to_string(),
                        attempts: step.retry.as_ref().map(|r| r.max_attempts).unwrap_or(1),
                        output_bytes: 0,
                    });
                    let continue_on_error =
                        matches!(step.on_error, Some(crate::adl::StepOnError::Continue));
                    if continue_on_error {
                        completed.insert(step_id);
                        continue;
                    }
                    tr.run_failed(&err.to_string());
                    tr.execution_boundary_crossed(ExecutionBoundary::RunCompletion, "failure");
                    tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Teardown);
                    return Err(anyhow!("step '{}' failed: {:#}", step_id, err));
                }
            }
        }
        if let Some((paused_step_id, reason)) = batch_pause {
            let mut completed_step_ids: Vec<String> = completed.iter().cloned().collect();
            completed_step_ids.sort();
            let mut remaining_step_ids: Vec<String> = pending.iter().cloned().collect();
            remaining_step_ids.sort();
            tr.execution_boundary_crossed(ExecutionBoundary::Pause, "entered");
            tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Teardown);
            return Ok(ExecutionResult {
                runtime_control: derive_runtime_control_state("paused", &records, tr),
                outputs: outs,
                artifacts,
                records,
                pause: Some(PauseState {
                    paused_step_id,
                    reason,
                    completed_step_ids,
                    remaining_step_ids,
                    saved_state,
                    completed_outputs,
                }),
                steering_history: steering_history.clone(),
            });
        }
    }

    tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Complete);
    tr.execution_boundary_crossed(ExecutionBoundary::RunCompletion, "success");
    tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Teardown);
    Ok(ExecutionResult {
        runtime_control: derive_runtime_control_state("success", &records, tr),
        outputs: outs,
        artifacts,
        records,
        pause: None,
        steering_history,
    })
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(super) fn execute_called_workflow(
    caller_step_id: &str,
    namespace: &str,
    callee_workflow_id: &str,
    call_with: &HashMap<String, String>,
    resolved: &AdlResolved,
    tr: &mut Trace,
    print_outputs: bool,
    emit_progress: bool,
    adl_base_dir: &Path,
    out_dir: &Path,
    caller_state: &HashMap<String, String>,
) -> Result<(
    Vec<StepOutput>,
    Vec<PathBuf>,
    Vec<StepExecutionRecord>,
    HashMap<String, String>,
)> {
    let callee = resolved
        .doc
        .workflows
        .get(callee_workflow_id)
        .ok_or_else(|| anyhow!("call references unknown workflow '{}'", callee_workflow_id))?;

    let mut child_state: HashMap<String, String> = HashMap::new();
    for (k, v) in call_with {
        let bound = resolve_call_binding(v, caller_state).with_context(|| {
            format!(
                "failed to resolve call.with binding '{}' for caller step '{}'",
                k, caller_step_id
            )
        })?;
        child_state.insert(format!("inputs.{k}"), bound);
    }

    let mut outs = Vec::new();
    let mut artifacts = Vec::new();
    let mut records = Vec::new();

    for (idx, step) in callee.steps.iter().enumerate() {
        let local_id = step.id.clone().unwrap_or_else(|| format!("step-{idx}"));
        let full_id = format!("{caller_step_id}::{local_id}");

        if let Some(nested) = step.call.as_deref() {
            let nested_namespace = step.as_ns.clone().unwrap_or_else(|| local_id.clone());
            tr.call_entered(&full_id, nested, &nested_namespace);

            let nested_result = execute_called_workflow(
                &full_id,
                &nested_namespace,
                nested,
                &step.with,
                resolved,
                tr,
                print_outputs,
                emit_progress,
                adl_base_dir,
                out_dir,
                &child_state,
            );

            let (sub_outs, sub_artifacts, sub_records, sub_state) = match nested_result {
                Ok(v) => {
                    tr.call_exited(&full_id, "success", &nested_namespace);
                    v
                }
                Err(err) => {
                    tr.call_exited(&full_id, "failure", &nested_namespace);
                    return Err(err);
                }
            };
            for (k, v) in sub_state {
                child_state.insert(format!("{nested_namespace}.{k}"), v);
            }
            outs.extend(sub_outs);
            artifacts.extend(sub_artifacts);
            records.extend(sub_records);
            records.push(StepExecutionRecord {
                step_id: full_id,
                provider_id: "<call>".to_string(),
                status: "success".to_string(),
                attempts: 1,
                output_bytes: 0,
            });
            continue;
        }

        let mut step_for_exec = step.clone();
        for key in child_state.keys() {
            step_for_exec
                .inputs
                .entry(key.clone())
                .or_insert_with(|| format!("@state:{key}"));
        }
        let resolved_step = resolved_step_from_raw_step(&full_id, &step_for_exec, &resolved.doc);
        let provider_id: &str = resolved_step
            .provider
            .as_deref()
            .unwrap_or("<unresolved-provider>");
        let agent_id: &str = resolved_step
            .agent
            .as_deref()
            .unwrap_or("<unresolved-agent>");
        let task_id: &str = resolved_step.task.as_deref().unwrap_or("<unresolved-task>");

        if let Some(write_to) = resolved_step.write_to.as_deref() {
            if resolved_step.save_as.is_none() {
                return Err(anyhow!(
                    "step '{}' uses write_to but is missing save_as",
                    full_id
                ));
            }
            validate_write_to(&full_id, write_to)?;
        }
        enforce_delegation_policy_for_step_actions(tr, &resolved_step, &resolved.doc)?;

        tr.step_started(
            &full_id,
            agent_id,
            provider_id,
            task_id,
            resolved_step.delegation.as_ref(),
        );
        let step_started_elapsed = tr.current_elapsed_ms();
        progress_step_start(emit_progress, tr, &full_id, provider_id);

        match execute_step_with_retry(
            &resolved_step,
            &resolved.doc,
            &resolved.run_id,
            &resolved.workflow_id,
            &child_state,
            adl_base_dir,
            true,
        ) {
            Ok(success) => {
                tr.prompt_assembled(&full_id, &success.prompt_hash);
                tr.step_finished(&full_id, true);
                let duration_ms = tr.current_elapsed_ms().saturating_sub(step_started_elapsed);
                progress_step_done(emit_progress, tr, &full_id, true, duration_ms);

                if let Some(write_to) = resolved_step.write_to.as_deref() {
                    let path =
                        write_output(&full_id, out_dir, write_to, &success.out.model_output)?;
                    println!(
                        "ARTIFACT step={} path={} bytes={}",
                        full_id,
                        path.display(),
                        success.out.model_output.len()
                    );
                    artifacts.push(path);
                }

                if print_outputs {
                    emit_step_output(
                        &full_id,
                        &success.out.model_output,
                        &success.stream_chunks,
                        tr,
                    );
                }

                if let Some(save_as) = resolved_step.save_as.as_ref() {
                    child_state.insert(save_as.clone(), success.out.model_output.clone());
                }

                records.push(StepExecutionRecord {
                    step_id: full_id.clone(),
                    provider_id: success.out.provider_id.clone(),
                    status: "success".to_string(),
                    attempts: success.attempts,
                    output_bytes: success.out.model_output.len(),
                });
                outs.push(StepOutput {
                    step_id: full_id,
                    provider_id: success.out.provider_id,
                    model_output: success.out.model_output,
                });
            }
            Err(err) => {
                tr.step_finished(&full_id, false);
                let duration_ms = tr.current_elapsed_ms().saturating_sub(step_started_elapsed);
                progress_step_done(emit_progress, tr, &full_id, false, duration_ms);
                tr.run_failed(&err.to_string());
                return Err(anyhow!(
                    "called workflow '{}' step '{}' failed: {:#}",
                    callee_workflow_id,
                    full_id,
                    err
                ));
            }
        }
    }

    let _ = namespace;
    Ok((outs, artifacts, records, child_state))
}

pub(super) fn resolve_call_binding(
    value: &str,
    caller_state: &HashMap<String, String>,
) -> Result<String> {
    let trimmed = value.trim();
    if let Some(state_key) = trimmed.strip_prefix("@state:") {
        let key = state_key.trim();
        return caller_state
            .get(key)
            .cloned()
            .ok_or_else(|| anyhow!("missing state key '{}' for @state binding", key));
    }

    if trimmed.starts_with("{{") && trimmed.ends_with("}}") {
        let inner = trimmed[2..trimmed.len() - 2].trim();
        if let Some(rest) = inner.strip_prefix("state.") {
            let key = rest.trim();
            return caller_state.get(key).cloned().ok_or_else(|| {
                anyhow!(
                    "missing state key '{}' for '{{{{ state.* }}}}' binding",
                    key
                )
            });
        }
    }

    Ok(value.to_string())
}

fn resolved_step_from_raw_step(
    full_id: &str,
    step: &crate::adl::StepSpec,
    doc: &crate::adl::AdlDoc,
) -> crate::resolve::ResolvedStep {
    let agent = step.agent.clone().or_else(|| {
        step.task
            .as_ref()
            .and_then(|t| doc.tasks.get(t))
            .and_then(|task| task.agent_ref.clone())
    });

    let provider = agent
        .as_ref()
        .and_then(|a| doc.agents.get(a))
        .map(|a| a.provider.clone())
        .or_else(|| {
            if doc.providers.len() == 1 {
                doc.providers.keys().next().cloned()
            } else {
                None
            }
        });

    crate::resolve::ResolvedStep {
        id: full_id.to_string(),
        agent,
        provider,
        task: step.task.clone(),
        placement: step.placement.clone(),
        call: step.call.clone(),
        with: step.with.clone(),
        as_ns: step.as_ns.clone(),
        delegation: step.delegation.clone(),
        prompt: step.prompt.clone(),
        inputs: step.inputs.clone(),
        guards: step.guards.clone(),
        save_as: step.save_as.clone(),
        write_to: step.write_to.clone(),
        on_error: step.on_error.clone(),
        retry: step.retry.clone(),
    }
}
