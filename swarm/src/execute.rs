use std::collections::{HashMap, HashSet};
use std::path::{Component, Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

use crate::bounded_executor;
use crate::delegation_policy::{self, DelegationDecision};
use crate::prompt;
use crate::provider;
use crate::remote_exec;
use crate::resolve::AdlResolved;
use crate::sandbox;
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
        let path_for_stat = if candidate.is_absolute() {
            candidate.clone()
        } else {
            base_dir.join(&candidate)
        };

        let meta = std::fs::metadata(&path_for_stat).with_context(|| {
            format!(
                "failed to stat input file for '{k}': '{}' (base_dir='{}')",
                path_for_stat.display(),
                base_dir.display()
            )
        })?;
        if !meta.is_file() {
            return Err(anyhow!(
                "input '{k}' references a non-file path: '{}'",
                path_for_stat.display()
            ));
        }
        if meta.len() > MATERIALIZE_INPUT_MAX_FILE_BYTES {
            return Err(anyhow!(
                "input '{k}' file is too large ({} bytes > {} bytes): '{}'",
                meta.len(),
                MATERIALIZE_INPUT_MAX_FILE_BYTES,
                path_for_stat.display()
            ));
        }

        let canon =
            sandbox::resolve_existing_path_within_root(base_dir, &candidate).map_err(|err| {
                let requested = err.requested_path().unwrap_or("sandbox:/<unknown>");
                let resolved = err
                    .resolved_path()
                    .map(|value| format!(" resolved_path={value}"))
                    .unwrap_or_default();
                anyhow!(
                    "input '{k}' file rejected by sandbox resolver: code={} message={} requested_path={}{}",
                    err.code(),
                    err.message(),
                    requested,
                    resolved
                )
            })?;

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

/// Default concurrency cap for concurrent workflow runs when no override is provided.
const DEFAULT_MAX_CONCURRENCY: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerPolicySource {
    WorkflowOverride,
    RunDefault,
    EngineDefault,
}

impl SchedulerPolicySource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WorkflowOverride => "workflow_override",
            Self::RunDefault => "run_default",
            Self::EngineDefault => "engine_default",
        }
    }
}

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
    pub pause: Option<PauseState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PauseState {
    pub paused_step_id: String,
    pub reason: Option<String>,
    pub completed_step_ids: Vec<String>,
    pub remaining_step_ids: Vec<String>,
    pub saved_state: HashMap<String, String>,
    pub completed_outputs: HashMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct ResumeState {
    pub completed_step_ids: HashSet<String>,
    pub saved_state: HashMap<String, String>,
    pub completed_outputs: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Stable policy rejection kinds emitted by execution-time delegation checks.
pub enum ExecutionPolicyErrorKind {
    /// Action was denied by policy and execution must fail/stop that step.
    Denied,
    /// Action requires approval and cannot proceed automatically.
    ApprovalRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionPolicyError {
    pub kind: ExecutionPolicyErrorKind,
    pub step_id: String,
    pub action_kind: String,
    pub target_id: String,
    pub rule_id: Option<String>,
}

impl ExecutionPolicyError {
    pub fn code(&self) -> &'static str {
        match self.kind {
            ExecutionPolicyErrorKind::Denied => DELEGATION_POLICY_DENY_CODE,
            ExecutionPolicyErrorKind::ApprovalRequired => "DELEGATION_POLICY_APPROVAL_REQUIRED",
        }
    }
}

impl std::fmt::Display for ExecutionPolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix = self
            .rule_id
            .as_ref()
            .map(|id| format!(" (rule_id={id})"))
            .unwrap_or_default();
        match self.kind {
            ExecutionPolicyErrorKind::Denied => write!(
                f,
                "{}: step '{}' action '{}' target '{}' denied{}",
                self.code(),
                self.step_id,
                self.action_kind,
                self.target_id,
                suffix
            ),
            ExecutionPolicyErrorKind::ApprovalRequired => write!(
                f,
                "{}: step '{}' action '{}' target '{}' requires approval{}",
                self.code(),
                self.step_id,
                self.action_kind,
                self.target_id,
                suffix
            ),
        }
    }
}

impl std::error::Error for ExecutionPolicyError {}

pub fn stable_failure_kind(err: &anyhow::Error) -> Option<&'static str> {
    for cause in err.chain() {
        if cause.downcast_ref::<ExecutionPolicyError>().is_some() {
            return Some("policy_denied");
        }
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResumeDisposition {
    Skip(&'static str),
    Rerun(&'static str),
}

fn pause_reason_for_step(step: &crate::resolve::ResolvedStep) -> Option<Option<String>> {
    for guard in &step.guards {
        if guard.kind.trim().eq_ignore_ascii_case("pause") {
            let reason = guard
                .config
                .get("reason")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            return Some(reason);
        }
    }
    None
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

fn emit_step_output(step_id: &str, model_output: &str, stream_chunks: &[String], tr: &mut Trace) {
    println!("--- step: {} ---", step_id);
    if stream_chunks.is_empty() {
        println!("{}", model_output.trim_end());
    } else {
        for chunk in stream_chunks {
            if !chunk.is_empty() {
                print!("{chunk}");
            }
        }
        if !model_output.ends_with('\n') {
            println!();
        }
        tr.step_output_chunk(step_id, model_output.len());
    }
    println!();
}

fn stable_fingerprint_hex(bytes: &[u8]) -> String {
    // FNV-1a 64-bit (deterministic, dependency-free fingerprint for persisted metadata).
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in bytes {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn model_output_fingerprint(output: &str) -> String {
    stable_fingerprint_hex(output.as_bytes())
}

fn resume_disposition_for_step(
    step: &crate::resolve::ResolvedStep,
    out_dir: &Path,
    completed_outputs: &HashMap<String, String>,
) -> Result<ResumeDisposition> {
    let Some(write_to) = step.write_to.as_deref() else {
        return Ok(ResumeDisposition::Skip("completed_no_artifact_expected"));
    };

    validate_write_to(&step.id, write_to)?;
    let artifact_path = out_dir.join(write_to);
    if !artifact_path.is_file() {
        return Ok(ResumeDisposition::Rerun("missing_expected_artifact"));
    }

    let Some(expected_fingerprint) = completed_outputs.get(&step.id) else {
        return Ok(ResumeDisposition::Rerun("missing_output_fingerprint"));
    };
    let actual = std::fs::read_to_string(&artifact_path).with_context(|| {
        format!(
            "failed to read expected resume artifact for step '{}' at '{}'",
            step.id,
            artifact_path.display()
        )
    })?;
    let actual_fingerprint = model_output_fingerprint(&actual);
    if &actual_fingerprint != expected_fingerprint {
        return Ok(ResumeDisposition::Rerun("invalid_expected_artifact"));
    }

    Ok(ResumeDisposition::Skip("completed_artifact_verified"))
}

fn emit_resume_note(enabled: bool, step_id: &str, action: &str, reason: &str) {
    if !enabled {
        return;
    }
    eprintln!(
        "RESUME step={} action={} reason={}",
        step_id, action, reason
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
/// - effective max concurrency for concurrent workflow runs is deterministic and applied as:
///   1) run.workflow.max_concurrency (or `workflows.<id>.max_concurrency` via workflow_ref)
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
                    return Ok(ExecutionResult {
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
                return Err(failure.err.context(format!(
                    "step '{}' failed (attempt {}/{}, max_attempts={})",
                    step_id, failure.attempts, max_attempts, max_attempts
                )));
            }
        }
    }

    Ok(ExecutionResult {
        outputs: outs,
        artifacts,
        records,
        pause: None,
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

#[derive(Debug)]
struct StepRunSuccess {
    out: StepOutput,
    attempts: u32,
    prompt_hash: String,
    stream_chunks: Vec<String>,
}

#[derive(Debug)]
struct StepRunFailure {
    err: anyhow::Error,
    attempts: u32,
    stream_chunks: Vec<String>,
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

fn effective_step_placement(
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

fn emit_delegation_lifecycle_start(
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

fn emit_delegation_lifecycle_finish(
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

fn effective_max_concurrency_with_source(
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
    // v0.7 scope: scheduler policy surface is emitted for concurrent workflows only.
    // Sequential workflows intentionally return None to avoid implying a concurrency
    // policy where no scheduler fan-out is active.
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

fn enforce_delegation_policy_for_step_actions(
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

fn execute_step_with_retry(
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
fn execute_step_with_retry_core<F>(
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
                    let prov =
                        provider::build_provider(spec, model_override).with_context(|| {
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
                        anyhow!("REMOTE_SCHEMA_VIOLATION: run.remote.endpoint is required when placement=remote")
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

fn execute_concurrent_deterministic(
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
                    return Err(anyhow!("step '{}' failed: {:#}", step_id, err));
                }
            }
        }
        if let Some((paused_step_id, reason)) = batch_pause {
            let mut completed_step_ids: Vec<String> = completed.iter().cloned().collect();
            completed_step_ids.sort();
            let mut remaining_step_ids: Vec<String> = pending.iter().cloned().collect();
            remaining_step_ids.sort();
            return Ok(ExecutionResult {
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
            });
        }
    }

    Ok(ExecutionResult {
        outputs: outs,
        artifacts,
        records,
        pause: None,
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
    std::fs::create_dir_all(out_dir).with_context(|| {
        format!(
            "failed to create sandbox output root for step '{}': '{}'",
            step_id,
            out_dir.display()
        )
    })?;
    let rel = PathBuf::from(write_to);
    let path =
        sandbox::resolve_relative_path_for_write_within_root(out_dir, &rel).map_err(|err| {
            let requested = err.requested_path().unwrap_or("sandbox:/<unknown>");
            let resolved = err
                .resolved_path()
                .map(|value| format!(" resolved_path={value}"))
                .unwrap_or_default();
            anyhow!(
                "step {} write_to rejected by sandbox resolver: code={} message={} requested_path={}{}",
                step_id,
                err.code(),
                err.message(),
                requested,
                resolved
            )
        })?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adl::{AdlDoc, PromptSpec, RunDefaults, RunSpec, WorkflowKind, WorkflowSpec};
    use crate::resolve::AdlResolved;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn minimal_resolved() -> AdlResolved {
        AdlResolved {
            run_id: "run".to_string(),
            workflow_id: "wf".to_string(),
            doc: AdlDoc {
                version: "0.3".to_string(),
                providers: HashMap::new(),
                tools: HashMap::new(),
                agents: HashMap::new(),
                tasks: HashMap::new(),
                workflows: HashMap::new(),
                patterns: vec![],
                signature: None,
                run: RunSpec {
                    id: None,
                    name: Some("run".to_string()),
                    created_at: None,
                    defaults: RunDefaults::default(),
                    workflow_ref: None,
                    workflow: Some(WorkflowSpec {
                        id: None,
                        kind: WorkflowKind::Concurrent,
                        max_concurrency: None,
                        steps: vec![],
                    }),
                    pattern_ref: None,
                    inputs: HashMap::new(),
                    placement: None,
                    remote: None,
                    delegation_policy: None,
                },
            },
            steps: vec![],
            execution_plan: crate::execution_plan::ExecutionPlan {
                workflow_kind: WorkflowKind::Concurrent,
                nodes: vec![],
            },
        }
    }

    #[test]
    fn resolve_state_inputs_resolves_and_validates_state_bindings() {
        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), "@state:key1".to_string());
        inputs.insert("b".to_string(), "literal".to_string());
        let mut state = HashMap::new();
        state.insert("key1".to_string(), "value1".to_string());

        let merged = resolve_state_inputs("s1", &inputs, &state).expect("resolve");
        assert_eq!(merged.get("a").map(String::as_str), Some("value1"));
        assert_eq!(merged.get("b").map(String::as_str), Some("literal"));

        inputs.insert("a".to_string(), "@state:".to_string());
        let empty_err = resolve_state_inputs("s1", &inputs, &state).expect_err("empty key fails");
        assert!(empty_err
            .to_string()
            .contains("uses @state: with an empty key"));

        inputs.insert("a".to_string(), "@state:missing".to_string());
        let missing_err =
            resolve_state_inputs("s1", &inputs, &state).expect_err("missing key fails");
        assert!(missing_err
            .to_string()
            .contains("references missing saved state"));
    }

    #[test]
    fn missing_prompt_inputs_dedupes_and_sorts_missing_keys() {
        let prompt = PromptSpec {
            system: Some("{{ b }}".to_string()),
            user: Some("{{a}} and {{b}} and {{ a }}".to_string()),
            context: Some("{{c}}".to_string()),
            ..Default::default()
        };
        let mut inputs = HashMap::new();
        inputs.insert("c".to_string(), "ok".to_string());
        let missing = missing_prompt_inputs(&prompt, &inputs);
        assert_eq!(missing, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn validate_write_to_rejects_invalid_paths() {
        let empty_err = validate_write_to("s1", "   ").expect_err("empty should fail");
        assert!(empty_err.to_string().contains("empty write_to"));

        let abs_err = validate_write_to("s1", "/tmp/out.txt").expect_err("absolute should fail");
        assert!(abs_err.to_string().contains("must be a relative path"));

        let traversal_err =
            validate_write_to("s1", "../escape.txt").expect_err("traversal should fail");
        assert!(traversal_err.to_string().contains("without '..'"));
    }

    #[test]
    fn write_output_creates_parent_directories() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let out_dir =
            std::env::temp_dir().join(format!("swarm-write-output-{now}-{}", std::process::id()));
        let path =
            write_output("s1", &out_dir, "nested/result.txt", "hello").expect("write output");
        let written = std::fs::read_to_string(&path).expect("read output");
        assert_eq!(written, "hello");
        let _ = std::fs::remove_dir_all(out_dir);
    }

    #[cfg(unix)]
    #[test]
    fn write_output_rejects_symlink_escape() {
        use std::os::unix::fs as unix_fs;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let base = std::env::temp_dir().join(format!(
            "swarm-write-output-symlink-{now}-{}",
            std::process::id()
        ));
        let out_dir = base.join("root");
        let outside = base.join("outside");
        std::fs::create_dir_all(&out_dir).expect("create out dir");
        std::fs::create_dir_all(&outside).expect("create outside dir");
        unix_fs::symlink(&outside, out_dir.join("link")).expect("create symlink");

        let err = write_output("s1", &out_dir, "link/escape.txt", "data")
            .expect_err("symlink escape must be rejected");
        assert!(
            err.to_string().contains("sandbox resolver"),
            "unexpected error: {err}"
        );

        let _ = std::fs::remove_dir_all(base);
    }

    #[test]
    fn effective_max_concurrency_precedence_and_validation() {
        let mut resolved = minimal_resolved();
        assert_eq!(
            effective_max_concurrency_with_source(&resolved)
                .expect("default")
                .0,
            4
        );

        resolved.doc.run.defaults.max_concurrency = Some(3);
        assert_eq!(
            effective_max_concurrency_with_source(&resolved)
                .expect("run default")
                .0,
            3
        );

        resolved
            .doc
            .run
            .workflow
            .as_mut()
            .expect("workflow")
            .max_concurrency = Some(2);
        assert_eq!(
            effective_max_concurrency_with_source(&resolved)
                .expect("workflow override")
                .0,
            2
        );

        resolved
            .doc
            .run
            .workflow
            .as_mut()
            .expect("workflow")
            .max_concurrency = Some(0);
        let err = effective_max_concurrency_with_source(&resolved).expect_err("zero should fail");
        assert!(err.to_string().contains("must be >= 1"));
    }

    #[test]
    fn effective_max_concurrency_supports_workflow_ref_overrides() {
        let mut resolved = minimal_resolved();
        resolved.doc.run.workflow = None;
        resolved.doc.run.workflow_ref = Some("wf_ref".to_string());
        resolved.doc.workflows.insert(
            "wf_ref".to_string(),
            WorkflowSpec {
                id: Some("wf_ref".to_string()),
                kind: WorkflowKind::Concurrent,
                max_concurrency: Some(5),
                steps: vec![],
            },
        );
        assert_eq!(
            effective_max_concurrency_with_source(&resolved)
                .expect("workflow ref override")
                .0,
            5
        );
    }

    #[test]
    fn pause_reason_for_step_detects_pause_guard_and_optional_reason() {
        let mut step = crate::resolve::ResolvedStep {
            id: "s1".to_string(),
            agent: None,
            provider: None,
            placement: None,
            task: None,
            call: None,
            with: HashMap::new(),
            as_ns: None,
            delegation: None,
            prompt: None,
            inputs: HashMap::new(),
            guards: vec![],
            save_as: None,
            write_to: None,
            on_error: None,
            retry: None,
        };
        assert_eq!(pause_reason_for_step(&step), None);

        step.guards.push(crate::adl::GuardSpec {
            kind: "pause".to_string(),
            config: HashMap::new(),
        });
        assert_eq!(pause_reason_for_step(&step), Some(None));

        step.guards[0].config.insert(
            "reason".to_string(),
            serde_json::Value::String("needs review".to_string()),
        );
        assert_eq!(
            pause_reason_for_step(&step),
            Some(Some("needs review".to_string()))
        );
    }

    #[test]
    fn effective_step_placement_prefers_step_then_run_then_local_default() {
        let mut doc = minimal_resolved().doc;
        let step = crate::resolve::ResolvedStep {
            id: "s1".to_string(),
            agent: None,
            provider: None,
            placement: None,
            task: None,
            call: None,
            with: HashMap::new(),
            as_ns: None,
            delegation: None,
            prompt: None,
            inputs: HashMap::new(),
            guards: vec![],
            save_as: None,
            write_to: None,
            on_error: None,
            retry: None,
        };
        assert_eq!(
            effective_step_placement(&step, &doc),
            crate::adl::PlacementMode::Local
        );

        doc.run.placement = Some(crate::adl::RunPlacementSpec::Mode(
            crate::adl::PlacementMode::Remote,
        ));
        assert_eq!(
            effective_step_placement(&step, &doc),
            crate::adl::PlacementMode::Remote
        );

        let mut step_override = step.clone();
        step_override.placement = Some(crate::adl::PlacementMode::Local);
        assert_eq!(
            effective_step_placement(&step_override, &doc),
            crate::adl::PlacementMode::Local
        );
    }

    #[test]
    fn resolve_call_binding_requires_state_prefix_and_known_key() {
        let mut state = HashMap::new();
        state.insert("inputs.topic".to_string(), "ADL".to_string());

        let resolved =
            resolve_call_binding("@state:inputs.topic", &state).expect("state binding should work");
        assert_eq!(resolved, "ADL");
        let templated = resolve_call_binding("{{ state.inputs.topic }}", &state)
            .expect("templated state binding should work");
        assert_eq!(templated, "ADL");

        let missing = resolve_call_binding("@state:inputs.missing", &state)
            .expect_err("missing state key should fail");
        assert!(missing.to_string().contains("missing state key"));

        let passthrough = resolve_call_binding("literal", &state).expect("literal passthrough");
        assert_eq!(passthrough, "literal");
    }

    #[test]
    fn stable_failure_kind_detects_only_policy_errors() {
        let policy_err = anyhow::Error::new(ExecutionPolicyError {
            kind: ExecutionPolicyErrorKind::Denied,
            step_id: "s1".to_string(),
            action_kind: "tool".to_string(),
            target_id: "fs.write".to_string(),
            rule_id: Some("rule-1".to_string()),
        });
        assert_eq!(stable_failure_kind(&policy_err), Some("policy_denied"));

        let generic = anyhow::anyhow!("not policy related");
        assert_eq!(stable_failure_kind(&generic), None);
    }
}
