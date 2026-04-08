use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};

use crate::resolve::AdlResolved;
use crate::trace::Trace;

mod runner;
mod state;
mod support;
#[cfg(test)]
mod tests;

pub use runner::{
    scheduler_policy_for_run, DELEGATION_POLICY_APPROVAL_REQUIRED_CODE, DELEGATION_POLICY_DENY_CODE,
};
pub use state::*;

use runner::{
    emit_delegation_lifecycle_finish, emit_delegation_lifecycle_start,
    enforce_delegation_policy_for_step_actions, execute_called_workflow,
    execute_concurrent_deterministic, execute_step_with_retry_core,
};
use state::DEFAULT_MAX_CONCURRENCY;
use support::{
    emit_resume_note, emit_step_output, missing_prompt_inputs, model_output_fingerprint,
    pause_reason_for_step, progress_step_done, progress_step_start, resolve_state_inputs,
    resume_disposition_for_step, validate_write_to, write_output, ResumeDisposition,
};

/// Execute the resolved run.
///
/// Behavior:
/// - sequential workflows execute step-by-step
/// - concurrent workflows execute via deterministic bounded batching
/// - retries and `on_error` policy are honored per step
///
/// Determinism:
/// - ready-step ordering is lexicographic by full step id
/// - bounded batches preserve deterministic output/record order
/// - effective max concurrency for concurrent workflow runs is deterministic and applied as:
///   1) `run.workflow.max_concurrency` (or `workflows.<id>.max_concurrency` via `workflow_ref`)
///   2) run.defaults.max_concurrency
///   3) DEFAULT_MAX_CONCURRENCY
pub fn execute_sequential(
    resolved: &AdlResolved,
    tr: &mut Trace,
    print_outputs: bool,
    emit_progress: bool,
    adl_base_dir: &Path,
    out_dir: &Path,
) -> Result<ExecutionResult> {
    execute_sequential_with_resume(
        resolved,
        tr,
        print_outputs,
        emit_progress,
        adl_base_dir,
        out_dir,
        None,
    )
}

pub fn execute_sequential_with_resume(
    resolved: &AdlResolved,
    tr: &mut Trace,
    print_outputs: bool,
    emit_progress: bool,
    adl_base_dir: &Path,
    out_dir: &Path,
    resume: Option<ResumeState>,
) -> Result<ExecutionResult> {
    let is_concurrent = matches!(
        resolved.execution_plan.workflow_kind,
        crate::adl::WorkflowKind::Concurrent
    );
    if is_concurrent {
        let doc_version = resolved.doc.version.trim();
        let pattern_run = resolved.doc.run.pattern_ref.is_some();
        let allow = doc_version == "0.3" || (doc_version == "0.5" && pattern_run);
        if !allow {
            tr.run_failed("concurrent workflows are not supported for this document shape/version");
            return Err(anyhow!(
                "feature 'concurrency' requires v0.3 workflows or v0.5 pattern runs; document version is {doc_version} (run.workflow.kind=concurrent)"
            ));
        }
    }
    tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Init);
    tr.execution_boundary_crossed(
        ExecutionBoundary::RuntimeInit,
        if resume.is_some() {
            "resume"
        } else {
            "fresh_start"
        },
    );
    if resume.is_some() {
        tr.execution_boundary_crossed(ExecutionBoundary::Resume, "entered");
    }
    tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Execute);
    if is_concurrent {
        return execute_concurrent_deterministic(
            resolved,
            tr,
            print_outputs,
            emit_progress,
            adl_base_dir,
            out_dir,
            resume.as_ref(),
        );
    }

    let mut outs = Vec::new();
    let mut artifacts = Vec::new();
    let mut records = Vec::new();
    let mut saved_state: HashMap<String, String> = resume
        .as_ref()
        .map(|r| r.saved_state.clone())
        .unwrap_or_default();
    let mut completed_outputs: HashMap<String, String> = resume
        .as_ref()
        .map(|r| r.completed_outputs.clone())
        .unwrap_or_default();
    let mut completed_step_ids: HashSet<String> = resume
        .as_ref()
        .map(|r| r.completed_step_ids.clone())
        .unwrap_or_default();
    let steering_history = resume
        .as_ref()
        .map(|r| r.steering_history.clone())
        .unwrap_or_default();

    if let Some(resume) = resume.as_ref() {
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
        completed_step_ids = validated_completed;
    }

    for step in &resolved.steps {
        let step_id = step.id.clone();
        if completed_step_ids.contains(&step_id) {
            continue;
        }

        let agent_id: &str = step.agent.as_deref().unwrap_or("<unresolved-agent>");
        let task_id: &str = step.task.as_deref().unwrap_or("<unresolved-task>");
        let provider_id: &str = step.provider.as_deref().unwrap_or("<unresolved-provider>");

        if let Some(write_to) = step.write_to.as_deref() {
            if step.save_as.is_none() {
                return Err(anyhow!(
                    "step '{}' uses write_to but is missing save_as",
                    step_id
                ));
            }
            validate_write_to(&step_id, write_to)?;
        }
        enforce_delegation_policy_for_step_actions(tr, step, &resolved.doc)?;

        emit_delegation_lifecycle_start(tr, step, &resolved.doc);
        tr.step_started(
            &step_id,
            agent_id,
            provider_id,
            task_id,
            step.delegation.as_ref(),
        );
        let step_started_elapsed = tr.current_elapsed_ms();
        progress_step_start(emit_progress, tr, &step_id, provider_id);

        if let Some(callee_workflow_id) = step.call.as_deref() {
            let namespace = step.as_ns.clone().unwrap_or_else(|| step_id.clone());
            tr.call_entered(&step_id, callee_workflow_id, &namespace);

            let call_result = execute_called_workflow(
                &step_id,
                &namespace,
                callee_workflow_id,
                &step.with,
                resolved,
                tr,
                print_outputs,
                emit_progress,
                adl_base_dir,
                out_dir,
                &saved_state,
            );

            match call_result {
                Ok((call_outs, call_artifacts, call_records, callee_final_state)) => {
                    tr.call_exited(&step_id, "success", &namespace);
                    emit_delegation_lifecycle_finish(tr, step, &resolved.doc, true, 0);
                    tr.step_finished(&step_id, true);
                    let duration_ms = tr.current_elapsed_ms().saturating_sub(step_started_elapsed);
                    progress_step_done(emit_progress, tr, &step_id, true, duration_ms);

                    for (k, v) in callee_final_state {
                        saved_state.insert(format!("{namespace}.{k}"), v);
                    }

                    outs.extend(call_outs);
                    artifacts.extend(call_artifacts);
                    records.extend(call_records);
                    records.push(StepExecutionRecord {
                        step_id: step_id.clone(),
                        provider_id: "<call>".to_string(),
                        status: "success".to_string(),
                        attempts: 1,
                        output_bytes: 0,
                    });
                    completed_step_ids.insert(step_id.clone());
                    if let Some(reason) = pause_reason_for_step(step) {
                        let mut completed_step_ids_vec: Vec<String> =
                            completed_step_ids.iter().cloned().collect();
                        completed_step_ids_vec.sort();
                        let remaining_step_ids = resolved
                            .steps
                            .iter()
                            .map(|s| s.id.clone())
                            .filter(|id| !completed_step_ids.contains(id))
                            .collect::<Vec<_>>();
                        return Ok(ExecutionResult {
                            runtime_control: derive_runtime_control_state("paused", &records, tr),
                            outputs: outs,
                            artifacts,
                            records,
                            pause: Some(PauseState {
                                paused_step_id: step_id,
                                reason,
                                completed_step_ids: completed_step_ids_vec,
                                remaining_step_ids,
                                saved_state,
                                completed_outputs,
                            }),
                            steering_history: steering_history.clone(),
                        });
                    }
                    continue;
                }
                Err(err) => {
                    tr.call_exited(&step_id, "failure", &namespace);
                    emit_delegation_lifecycle_finish(tr, step, &resolved.doc, false, 0);
                    tr.step_finished(&step_id, false);
                    let duration_ms = tr.current_elapsed_ms().saturating_sub(step_started_elapsed);
                    progress_step_done(emit_progress, tr, &step_id, false, duration_ms);
                    tr.run_failed(&err.to_string());
                    tr.execution_boundary_crossed(ExecutionBoundary::RunCompletion, "failure");
                    tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Teardown);
                    return Err(err);
                }
            }
        }

        let continue_on_error = matches!(step.on_error, Some(crate::adl::StepOnError::Continue));
        let max_attempts = step.retry.as_ref().map(|r| r.max_attempts).unwrap_or(1);
        match execute_step_with_retry_core(
            step,
            &resolved.doc,
            &resolved.run_id,
            &resolved.workflow_id,
            &saved_state,
            adl_base_dir,
            true,
            |prompt_hash| tr.prompt_assembled(&step_id, prompt_hash),
        ) {
            Ok(success) => {
                emit_delegation_lifecycle_finish(
                    tr,
                    step,
                    &resolved.doc,
                    true,
                    success.out.model_output.len(),
                );
                tr.step_finished(&step_id, true);
                let duration_ms = tr.current_elapsed_ms().saturating_sub(step_started_elapsed);
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
                    provider_id: provider_id.to_string(),
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
                completed_step_ids.insert(step_id.clone());
                outs.push(success.out);

                if let Some(reason) = pause_reason_for_step(step) {
                    let mut completed_vec: Vec<String> =
                        completed_step_ids.iter().cloned().collect();
                    completed_vec.sort();
                    let remaining_step_ids = resolved
                        .steps
                        .iter()
                        .map(|s| s.id.clone())
                        .filter(|id| !completed_step_ids.contains(id))
                        .collect::<Vec<_>>();
                    tr.execution_boundary_crossed(ExecutionBoundary::Pause, "entered");
                    tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Teardown);
                    return Ok(ExecutionResult {
                        runtime_control: derive_runtime_control_state("paused", &records, tr),
                        outputs: outs,
                        artifacts,
                        records,
                        pause: Some(PauseState {
                            paused_step_id: step_id,
                            reason,
                            completed_step_ids: completed_vec,
                            remaining_step_ids,
                            saved_state,
                            completed_outputs,
                        }),
                        steering_history: steering_history.clone(),
                    });
                }
            }
            Err(failure) => {
                if print_outputs && !failure.stream_chunks.is_empty() {
                    // Preserve already-produced stream output as observational trace data even
                    // when the step ultimately fails.
                    emit_step_output(&step_id, "", &failure.stream_chunks, tr);
                }
                emit_delegation_lifecycle_finish(tr, step, &resolved.doc, false, 0);
                tr.step_finished(&step_id, false);
                let duration_ms = tr.current_elapsed_ms().saturating_sub(step_started_elapsed);
                progress_step_done(emit_progress, tr, &step_id, false, duration_ms);
                records.push(StepExecutionRecord {
                    step_id: step_id.clone(),
                    provider_id: provider_id.to_string(),
                    status: "failure".to_string(),
                    attempts: failure.attempts,
                    output_bytes: 0,
                });
                if continue_on_error {
                    continue;
                }
                tr.run_failed(&failure.err.to_string());
                tr.execution_boundary_crossed(ExecutionBoundary::RunCompletion, "failure");
                tr.lifecycle_phase_entered(RuntimeLifecyclePhase::Teardown);
                return Err(failure.err.context(format!(
                    "step '{}' failed (attempt {}/{}, max_attempts={})",
                    step_id, failure.attempts, max_attempts, max_attempts
                )));
            }
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
