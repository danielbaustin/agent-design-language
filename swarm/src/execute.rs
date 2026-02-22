use std::collections::{HashMap, HashSet};
use std::path::{Component, Path, PathBuf};

use anyhow::{anyhow, Context, Result};

use crate::bounded_executor;
use crate::prompt;
use crate::provider;
use crate::remote_exec;
use crate::resolve::AdlResolved;
use crate::trace::Trace;

/// Replace any input values that start with `@file:<path>` with file contents.
///
/// Behavior (v0.1):
/// - Resolves relative paths against `base_dir` (typically the directory containing the ADL YAML).
/// - Rejects empty paths.
/// - Enforces a conservative max file size to avoid accidental huge prompts.
/// - Reads as UTF-8 (lossless); errors if bytes are not valid UTF-8.
/// - Normalizes Windows newlines (\r\n -> \n).
pub fn materialize_inputs(
    mut inputs: HashMap<String, String>,
    base_dir: &Path,
) -> Result<HashMap<String, String>> {
    // Canonical base dir once so we can enforce that @file: inputs cannot escape it.
    // This rejects both `../` traversal and absolute paths outside the base dir.
    let base_canon = base_dir
        .canonicalize()
        .with_context(|| format!("failed to canonicalize base_dir '{}'", base_dir.display()))?;

    for (k, v) in inputs.iter_mut() {
        let Some(raw) = v.strip_prefix("@file:") else {
            continue;
        };

        let mut path_str = raw.trim();
        if path_str.is_empty() {
            return Err(anyhow!("input '{k}' uses @file: with an empty path"));
        }

        // Allow simple quoting in YAML values: "@file:..." or '@file:...'
        if (path_str.starts_with('"') && path_str.ends_with('"'))
            || (path_str.starts_with('\'') && path_str.ends_with('\''))
        {
            path_str = &path_str[1..path_str.len() - 1];
            path_str = path_str.trim();
        }

        let candidate = PathBuf::from(path_str);
        let path = if candidate.is_absolute() {
            candidate
        } else {
            base_dir.join(candidate)
        };

        let meta = std::fs::metadata(&path).with_context(|| {
            format!(
                "failed to stat input file for '{k}': '{}' (base_dir='{}')",
                path.display(),
                base_dir.display()
            )
        })?;
        if !meta.is_file() {
            return Err(anyhow!(
                "input '{k}' references a non-file path: '{}'",
                path.display()
            ));
        }
        if meta.len() > MATERIALIZE_INPUT_MAX_FILE_BYTES {
            return Err(anyhow!(
                "input '{k}' file is too large ({} bytes > {} bytes): '{}'",
                meta.len(),
                MATERIALIZE_INPUT_MAX_FILE_BYTES,
                path.display()
            ));
        }

        // Enforce that the resolved path stays within the base directory.
        // Canonicalization also collapses any `..` segments.
        let canon = path.canonicalize().with_context(|| {
            format!(
                "failed to canonicalize input file for '{k}': '{}' (base_dir='{}')",
                path.display(),
                base_dir.display()
            )
        })?;

        if !canon.starts_with(&base_canon) {
            return Err(anyhow!(
                "input '{k}' file resolves outside base_dir: '{}' (base_dir='{}')",
                canon.display(),
                base_dir.display()
            ));
        }

        let bytes = std::fs::read(&canon).with_context(|| {
            format!("failed to read input file for '{k}': '{}'", canon.display())
        })?;
        let mut text = String::from_utf8(bytes).with_context(|| {
            format!("input '{k}' file is not valid UTF-8: '{}'", canon.display())
        })?;

        // Normalize newlines for stable hashing / traces.
        if text.contains("\r\n") {
            text = text.replace("\r\n", "\n");
        }

        *v = text;
    }

    Ok(inputs)
}

/// Maximum allowed bytes per `@file:` materialized input.
pub const MATERIALIZE_INPUT_MAX_FILE_BYTES: u64 = 512 * 1024;

/// Result of executing one step.
#[allow(dead_code)] // v0.1: returned for callers / future use; not all fields are read yet
#[derive(Debug, Clone)]
pub struct StepOutput {
    pub step_id: String,
    pub provider_id: String,
    pub model_output: String,
}

/// Stable execution telemetry record for one step.
///
/// Records are emitted in deterministic step completion order and are intended
/// for run summaries and machine-readable artifact generation.
#[derive(Debug, Clone)]
pub struct StepExecutionRecord {
    pub step_id: String,
    pub provider_id: String,
    pub status: String,
    pub attempts: u32,
    pub output_bytes: usize,
}

/// Aggregate result from executing a resolved workflow.
///
/// Contains step outputs, generated artifact paths, and per-step records.
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub outputs: Vec<StepOutput>,
    pub artifacts: Vec<PathBuf>,
    pub records: Vec<StepExecutionRecord>,
}

fn progress_step_start(enabled: bool, tr: &Trace, step_id: &str, provider_id: &str) {
    if !enabled {
        return;
    }
    eprintln!(
        "STEP start (+{}ms) {} provider={}",
        tr.current_elapsed_ms(),
        step_id,
        provider_id
    );
}

fn progress_step_done(enabled: bool, tr: &Trace, step_id: &str, ok: bool, duration_ms: u128) {
    if !enabled {
        return;
    }
    let status = if ok { "ok" } else { "fail" };
    eprintln!(
        "STEP done (+{}ms) {} {} duration_ms={}",
        tr.current_elapsed_ms(),
        step_id,
        status,
        duration_ms
    );
}

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
/// - effective max concurrency for concurrent workflow runs is deterministic:
///   run.workflow.max_concurrency/workflows.<id>.max_concurrency > run.defaults.max_concurrency > 4
pub fn execute_sequential(
    resolved: &AdlResolved,
    tr: &mut Trace,
    print_outputs: bool,
    emit_progress: bool,
    adl_base_dir: &Path,
    out_dir: &Path,
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
    if is_concurrent {
        return execute_concurrent_deterministic(
            resolved,
            tr,
            print_outputs,
            emit_progress,
            adl_base_dir,
            out_dir,
        );
    }

    let mut outs = Vec::new();
    let mut artifacts = Vec::new();
    let mut records = Vec::new();
    let mut saved_state: HashMap<String, String> = HashMap::new();

    for step in &resolved.steps {
        let step_id = step.id.clone();

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

        tr.step_started(&step_id, agent_id, provider_id, task_id);
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
                    continue;
                }
                Err(err) => {
                    tr.call_exited(&step_id, "failure", &namespace);
                    tr.step_finished(&step_id, false);
                    let duration_ms = tr.current_elapsed_ms().saturating_sub(step_started_elapsed);
                    progress_step_done(emit_progress, tr, &step_id, false, duration_ms);
                    tr.run_failed(&err.to_string());
                    return Err(err);
                }
            }
        }

        let max_attempts = step.retry.as_ref().map(|r| r.max_attempts).unwrap_or(1);
        let continue_on_error = matches!(step.on_error, Some(crate::adl::StepOnError::Continue));
        let mut attempt: u32 = 0;
        let mut last_err: Option<anyhow::Error> = None;
        let mut success_out: Option<StepOutput> = None;

        while attempt < max_attempts {
            attempt += 1;
            let result = (|| -> Result<StepOutput> {
                let p = step
                    .effective_prompt_with_defaults(resolved)
                    .ok_or_else(|| {
                        anyhow!(
                            "step '{}' has no effective prompt (step.prompt or task.prompt required)",
                            step_id
                        )
                    })?;

                let merged_inputs = resolve_state_inputs(&step.id, &step.inputs, &saved_state)
                    .with_context(|| format!("failed to resolve inputs for step '{}'", step_id))?;

                let missing = missing_prompt_inputs(&p, &merged_inputs);
                if !missing.is_empty() {
                    return Err(anyhow!(
                        "step '{}' missing input bindings for: {} (provide inputs or prior state)",
                        step_id,
                        missing.join(", ")
                    ));
                }

                // Allow inputs to reference files via "@file:<path>".
                let inputs =
                    materialize_inputs(merged_inputs, adl_base_dir).with_context(|| {
                        format!("failed to materialize inputs for step '{}'", step_id)
                    })?;

                // Assemble a single text blob suitable for basic model consumption.
                let prompt_text = prompt::trace_prompt_assembly(&p, &inputs);
                let prompt_hash = prompt::hash_prompt(&prompt_text);
                tr.prompt_assembled(&step_id, &prompt_hash);

                // Build provider from doc.providers[provider_id]
                let spec = resolved.doc.providers.get(provider_id).with_context(|| {
                    format!(
                        "step '{}' references unknown provider '{}'",
                        step_id, provider_id
                    )
                })?;
                let model_override = step
                    .agent
                    .as_ref()
                    .and_then(|agent_id| resolved.doc.agents.get(agent_id))
                    .map(|agent| agent.model.as_str());

                let placement = effective_step_placement(step, &resolved.doc);

                let model_output = match placement {
                    crate::adl::PlacementMode::Local => {
                        let prov =
                            provider::build_provider(spec, model_override).with_context(|| {
                                format!(
                                    "failed to build provider '{}' for step '{}'",
                                    provider_id, step_id
                                )
                            })?;
                        prov.complete(&prompt_text).with_context(|| {
                            format!(
                                "provider '{}' complete() failed for step '{}' (attempt {attempt}/{max_attempts})",
                                provider_id, step_id
                            )
                        })?
                    }
                    crate::adl::PlacementMode::Remote => {
                        let remote = resolved.doc.run.remote.as_ref().ok_or_else(|| {
                            anyhow!("REMOTE_SCHEMA_VIOLATION: run.remote.endpoint is required when placement=remote")
                        })?;
                        let timeout_ms = remote.timeout_ms.unwrap_or(30_000);
                        let req = remote_exec::ExecuteRequest {
                            protocol_version: remote_exec::PROTOCOL_VERSION.to_string(),
                            run_id: resolved.run_id.clone(),
                            workflow_id: resolved.workflow_id.clone(),
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
                        };
                        remote_exec::execute_remote(&remote.endpoint, timeout_ms, &req)
                            .with_context(|| {
                                format!("remote step '{}' execution failed", step_id)
                            })?
                    }
                };

                Ok(StepOutput {
                    step_id: step_id.clone(),
                    provider_id: provider_id.to_string(),
                    model_output,
                })
            })();

            match result {
                Ok(out) => {
                    success_out = Some(out);
                    break;
                }
                Err(err) => {
                    let retryable = provider::is_retryable_error(&err);
                    last_err = Some(err);
                    if !retryable {
                        break;
                    }
                    if attempt >= max_attempts {
                        break;
                    }
                }
            }
        }

        match success_out {
            Some(out) => {
                tr.step_finished(&step_id, true);
                let duration_ms = tr.current_elapsed_ms().saturating_sub(step_started_elapsed);
                progress_step_done(emit_progress, tr, &step_id, true, duration_ms);

                if let Some(write_to) = step.write_to.as_deref() {
                    let path = write_output(&step_id, out_dir, write_to, &out.model_output)?;
                    println!(
                        "ARTIFACT step={} path={} bytes={}",
                        step_id,
                        path.display(),
                        out.model_output.len()
                    );
                    artifacts.push(path);
                }

                if print_outputs {
                    println!("--- step: {} ---", step_id);
                    println!("{}", out.model_output.trim_end());
                    println!();
                }
                records.push(StepExecutionRecord {
                    step_id: step_id.clone(),
                    provider_id: provider_id.to_string(),
                    status: "success".to_string(),
                    attempts: attempt,
                    output_bytes: out.model_output.len(),
                });
                if let Some(save_as) = step.save_as.as_ref() {
                    saved_state.insert(save_as.clone(), out.model_output.clone());
                }
                outs.push(out);
            }
            None => {
                let err = last_err.unwrap_or_else(|| anyhow!("step '{}' failed", step_id));
                tr.step_finished(&step_id, false);
                let duration_ms = tr.current_elapsed_ms().saturating_sub(step_started_elapsed);
                progress_step_done(emit_progress, tr, &step_id, false, duration_ms);
                records.push(StepExecutionRecord {
                    step_id: step_id.clone(),
                    provider_id: provider_id.to_string(),
                    status: "failure".to_string(),
                    attempts: attempt.max(1),
                    output_bytes: 0,
                });
                if continue_on_error {
                    continue;
                }
                tr.run_failed(&err.to_string());
                return Err(anyhow!(
                    "step '{}' failed (attempt {}/{}, max_attempts={}): {:#}",
                    step_id,
                    attempt.max(1),
                    max_attempts,
                    max_attempts,
                    err
                ));
            }
        }
    }

    Ok(ExecutionResult {
        outputs: outs,
        artifacts,
        records,
    })
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn execute_called_workflow(
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

        tr.step_started(&full_id, agent_id, provider_id, task_id);
        let step_started_elapsed = tr.current_elapsed_ms();
        progress_step_start(emit_progress, tr, &full_id, provider_id);

        match execute_step_with_retry(
            &resolved_step,
            &resolved.doc,
            &resolved.run_id,
            &resolved.workflow_id,
            &child_state,
            adl_base_dir,
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
                    println!("--- step: {} ---", full_id);
                    println!("{}", success.out.model_output.trim_end());
                    println!();
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

fn resolve_call_binding(value: &str, caller_state: &HashMap<String, String>) -> Result<String> {
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
        prompt: step.prompt.clone(),
        inputs: step.inputs.clone(),
        save_as: step.save_as.clone(),
        write_to: step.write_to.clone(),
        on_error: step.on_error.clone(),
        retry: step.retry.clone(),
    }
}

#[derive(Debug)]
struct StepRunSuccess {
    out: StepOutput,
    attempts: u32,
    prompt_hash: String,
}

type StepJob = Box<dyn FnOnce() -> (String, Result<StepRunSuccess>) + Send>;

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

fn effective_step_placement(
    step: &crate::resolve::ResolvedStep,
    doc: &crate::adl::AdlDoc,
) -> crate::adl::PlacementMode {
    step.placement
        .clone()
        .or_else(|| doc.run.placement.as_ref().and_then(|p| p.mode()))
        .unwrap_or(crate::adl::PlacementMode::Local)
}

fn effective_max_concurrency(resolved: &AdlResolved) -> Result<usize> {
    const DEFAULT_MAX_PARALLEL: usize = 4;

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

    let max_parallel = workflow_override
        .or(resolved.doc.run.defaults.max_concurrency)
        .unwrap_or(DEFAULT_MAX_PARALLEL);

    if max_parallel == 0 {
        return Err(anyhow!(
            "effective max_concurrency must be >= 1 for concurrent runs"
        ));
    }

    Ok(max_parallel)
}

fn execute_step_with_retry(
    step: &crate::resolve::ResolvedStep,
    doc: &crate::adl::AdlDoc,
    run_id: &str,
    workflow_id: &str,
    saved_state: &HashMap<String, String>,
    adl_base_dir: &Path,
) -> Result<StepRunSuccess> {
    let step_id = step.id.clone();
    let provider_id: &str = step.provider.as_deref().unwrap_or("<unresolved-provider>");
    let max_attempts = step.retry.as_ref().map(|r| r.max_attempts).unwrap_or(1);
    let mut attempt: u32 = 0;
    let mut last_err: Option<anyhow::Error> = None;

    while attempt < max_attempts {
        attempt += 1;
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
                    let prov =
                        provider::build_provider(spec, model_override).with_context(|| {
                            format!(
                                "failed to build provider '{}' for step '{}'",
                                provider_id, step_id
                            )
                        })?;
                    prov.complete(&prompt_text).with_context(|| {
                        format!(
                            "provider '{}' complete() failed for step '{}' (attempt {attempt}/{max_attempts})",
                            provider_id, step_id
                        )
                    })?
                }
                crate::adl::PlacementMode::Remote => {
                    let remote = doc.run.remote.as_ref().ok_or_else(|| {
                        anyhow!("REMOTE_SCHEMA_VIOLATION: run.remote.endpoint is required when placement=remote")
                    })?;
                    let timeout_ms = remote.timeout_ms.unwrap_or(30_000);
                    let req = remote_exec::ExecuteRequest {
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
                    };
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
            })
        })();

        match result {
            Ok(success) => return Ok(success),
            Err(err) => {
                let retryable = provider::is_retryable_error(&err);
                last_err = Some(err);
                if !retryable || attempt >= max_attempts {
                    break;
                }
            }
        }
    }

    let err = last_err.unwrap_or_else(|| anyhow!("step '{}' failed", step_id));
    Err(anyhow!(
        "step '{}' failed (attempt {}/{}, max_attempts={}): {:#}",
        step_id,
        attempt.max(1),
        max_attempts,
        max_attempts,
        err
    ))
}

fn execute_concurrent_deterministic(
    resolved: &AdlResolved,
    tr: &mut Trace,
    print_outputs: bool,
    emit_progress: bool,
    adl_base_dir: &Path,
    out_dir: &Path,
) -> Result<ExecutionResult> {
    let max_parallel = effective_max_concurrency(resolved)?;

    let mut outs = Vec::new();
    let mut artifacts = Vec::new();
    let mut records = Vec::new();
    let mut progress_started_ms: HashMap<String, u128> = HashMap::new();
    let mut saved_state: HashMap<String, String> = HashMap::new();
    let mut completed: HashSet<String> = HashSet::new();
    let mut pending: HashSet<String> = resolved
        .execution_plan
        .nodes
        .iter()
        .map(|n| n.step_id.clone())
        .collect();
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
        // Keep branch execution deterministic even when YAML declaration order varies.
        ready_ids.sort();

        if ready_ids.is_empty() {
            let mut unresolved: Vec<String> = pending.iter().cloned().collect();
            unresolved.sort();
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
            tr.step_started(step_id, agent_id, provider_id, task_id);
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
                );
                (step_id_owned, run)
            }));
        }

        let results = bounded_executor::run_bounded(max_parallel, jobs)?;
        for (step_id, run_result) in results {
            let step = by_id
                .get(&step_id)
                .ok_or_else(|| anyhow!("execution result references unknown step '{}'", step_id))?;
            pending.remove(&step_id);

            match run_result {
                Ok(success) => {
                    tr.prompt_assembled(&step_id, &success.prompt_hash);
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
                        println!("--- step: {} ---", step_id);
                        println!("{}", success.out.model_output.trim_end());
                        println!();
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
                    outs.push(success.out);
                    completed.insert(step_id);
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
                    return Err(anyhow!("step '{}' failed: {:#}", step_id, err));
                }
            }
        }
    }

    Ok(ExecutionResult {
        outputs: outs,
        artifacts,
        records,
    })
}

fn validate_write_to(step_id: &str, write_to: &str) -> Result<()> {
    if write_to.trim().is_empty() {
        return Err(anyhow!("step '{}' has empty write_to path", step_id));
    }
    let path = Path::new(write_to);
    if path.is_absolute() || path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(anyhow!(
            "step '{}' write_to must be a relative path without '..'",
            step_id
        ));
    }
    Ok(())
}

fn write_output(step_id: &str, out_dir: &Path, write_to: &str, contents: &str) -> Result<PathBuf> {
    validate_write_to(step_id, write_to)?;
    let rel = PathBuf::from(write_to);
    let path = out_dir.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create output directory for step '{}': '{}'",
                step_id,
                parent.display()
            )
        })?;
    }
    std::fs::write(&path, contents.as_bytes()).with_context(|| {
        format!(
            "failed to write output for step '{}' to '{}'",
            step_id,
            path.display()
        )
    })?;
    Ok(path)
}

fn resolve_state_inputs(
    step_id: &str,
    inputs: &HashMap<String, String>,
    saved_state: &HashMap<String, String>,
) -> Result<HashMap<String, String>> {
    let mut merged = HashMap::new();
    for (key, value) in inputs.iter() {
        if let Some(raw_state_key) = value.strip_prefix("@state:") {
            let state_key = raw_state_key.trim();
            if state_key.is_empty() {
                return Err(anyhow!(
                    "step '{}' uses @state: with an empty key for input '{}'",
                    step_id,
                    key
                ));
            }
            let state_value = saved_state.get(state_key).ok_or_else(|| {
                anyhow!(
                    "step '{}' references missing saved state '{}' for input '{}'",
                    step_id,
                    state_key,
                    key
                )
            })?;
            merged.insert(key.clone(), state_value.clone());
            continue;
        }
        merged.insert(key.clone(), value.clone());
    }
    Ok(merged)
}

fn missing_prompt_inputs(
    p: &crate::adl::PromptSpec,
    inputs: &HashMap<String, String>,
) -> Vec<String> {
    let mut missing = HashSet::new();
    let mut check = |s: &str| {
        let mut i = 0;
        while let Some(start) = s[i..].find("{{") {
            let start_idx = i + start + 2;
            if let Some(end) = s[start_idx..].find("}}") {
                let end_idx = start_idx + end;
                let key = s[start_idx..end_idx].trim();
                if !key.is_empty() && !inputs.contains_key(key) {
                    missing.insert(key.to_string());
                }
                i = end_idx + 2;
            } else {
                break;
            }
        }
    };

    if let Some(v) = p.system.as_deref() {
        check(v);
    }
    if let Some(v) = p.developer.as_deref() {
        check(v);
    }
    if let Some(v) = p.user.as_deref() {
        check(v);
    }
    if let Some(v) = p.context.as_deref() {
        check(v);
    }
    if let Some(v) = p.output.as_deref() {
        check(v);
    }

    let mut out: Vec<String> = missing.into_iter().collect();
    out.sort();
    out
}
