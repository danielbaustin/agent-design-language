use super::*;

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
        runner::enforce_delegation_policy_for_step_actions(tr, &resolved_step, &resolved.doc)?;

        tr.step_started(
            &full_id,
            agent_id,
            provider_id,
            task_id,
            resolved_step.delegation.as_ref(),
        );
        let step_started_elapsed = tr.current_elapsed_ms();
        progress_step_start(emit_progress, tr, &full_id, provider_id);

        match runner::execute_step_with_retry(
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
