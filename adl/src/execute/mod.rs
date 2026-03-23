use std::collections::{HashMap, HashSet};
use std::path::{Component, Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

use crate::resolve::AdlResolved;
use crate::sandbox;
use crate::trace::Trace;

mod runner;

use runner::{
    emit_delegation_lifecycle_finish, emit_delegation_lifecycle_start,
    enforce_delegation_policy_for_step_actions, execute_called_workflow,
    execute_concurrent_deterministic, execute_step_with_retry_core,
};
pub use runner::{
    scheduler_policy_for_run, DELEGATION_POLICY_APPROVAL_REQUIRED_CODE, DELEGATION_POLICY_DENY_CODE,
};

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
    pub steering_history: Vec<SteeringRecord>,
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
    pub steering_history: Vec<SteeringRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SteeringPatch {
    pub schema_version: String,
    pub apply_at: String,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub set_state: HashMap<String, String>,
    #[serde(default)]
    pub remove_state: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SteeringRecord {
    pub sequence: u32,
    pub apply_at: String,
    #[serde(default)]
    pub reason: Option<String>,
    pub payload_fingerprint: String,
    #[serde(default)]
    pub set_state_keys: Vec<String>,
    #[serde(default)]
    pub removed_state_keys: Vec<String>,
}

pub const STEERING_PATCH_SCHEMA_VERSION: &str = "steering_patch.v1";
pub const STEERING_APPLY_AT_RESUME_BOUNDARY: &str = "resume_boundary";

pub fn validate_steering_patch(patch: &SteeringPatch) -> Result<()> {
    if patch.schema_version != STEERING_PATCH_SCHEMA_VERSION {
        return Err(anyhow!(
            "steering patch schema_version mismatch: patch='{}' expected='{}'",
            patch.schema_version,
            STEERING_PATCH_SCHEMA_VERSION
        ));
    }
    if patch.apply_at != STEERING_APPLY_AT_RESUME_BOUNDARY {
        return Err(anyhow!(
            "steering patch apply_at must be '{}' (found '{}')",
            STEERING_APPLY_AT_RESUME_BOUNDARY,
            patch.apply_at
        ));
    }

    let mut remove_set = HashSet::new();
    for key in &patch.remove_state {
        let trimmed = key.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("steering patch remove_state contains an empty key"));
        }
        if !remove_set.insert(trimmed.to_string()) {
            return Err(anyhow!(
                "steering patch remove_state contains duplicate key '{}'",
                trimmed
            ));
        }
    }

    for key in patch.set_state.keys() {
        let trimmed = key.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("steering patch set_state contains an empty key"));
        }
        if remove_set.contains(trimmed) {
            return Err(anyhow!(
                "steering patch key '{}' cannot appear in both set_state and remove_state",
                trimmed
            ));
        }
    }

    if patch.set_state.is_empty() && patch.remove_state.is_empty() {
        return Err(anyhow!(
            "steering patch must set or remove at least one saved-state key"
        ));
    }

    Ok(())
}

pub fn steering_record_from_patch(
    sequence: u32,
    payload_fingerprint: String,
    patch: &SteeringPatch,
) -> SteeringRecord {
    let mut set_state_keys: Vec<String> = patch.set_state.keys().cloned().collect();
    set_state_keys.sort();
    let mut removed_state_keys = patch.remove_state.clone();
    removed_state_keys.sort();
    removed_state_keys.dedup();

    SteeringRecord {
        sequence,
        apply_at: patch.apply_at.clone(),
        reason: patch.reason.clone(),
        payload_fingerprint,
        set_state_keys,
        removed_state_keys,
    }
}

pub fn apply_steering_patch(
    resume: &mut ResumeState,
    patch: &SteeringPatch,
    payload_fingerprint: String,
) -> Result<SteeringRecord> {
    validate_steering_patch(patch)?;

    for key in &patch.remove_state {
        resume.saved_state.remove(key.trim());
    }
    for (key, value) in &patch.set_state {
        resume
            .saved_state
            .insert(key.trim().to_string(), value.clone());
    }

    let sequence = u32::try_from(resume.steering_history.len())
        .unwrap_or(u32::MAX)
        .saturating_add(1);
    let record = steering_record_from_patch(sequence, payload_fingerprint, patch);
    resume.steering_history.push(record.clone());
    Ok(record)
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
        steering_history,
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
    use super::runner::{
        effective_max_concurrency_with_source, effective_step_placement, resolve_call_binding,
    };
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

    fn step_with_write_to(id: &str, write_to: Option<&str>) -> crate::resolve::ResolvedStep {
        crate::resolve::ResolvedStep {
            id: id.to_string(),
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
            save_as: Some("saved.output".to_string()),
            write_to: write_to.map(str::to_string),
            on_error: None,
            retry: None,
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
            std::env::temp_dir().join(format!("adl-write-output-{now}-{}", std::process::id()));
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
            "adl-write-output-symlink-{now}-{}",
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
    fn resume_disposition_validates_saved_artifacts_and_fingerprints() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let out_dir = std::env::temp_dir().join(format!(
            "adl-resume-disposition-{now}-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(out_dir.join("nested")).expect("create nested out dir");
        let artifact = out_dir.join("nested").join("result.txt");
        std::fs::write(&artifact, "hello").expect("write artifact");

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
            save_as: Some("saved".to_string()),
            write_to: Some("nested/result.txt".to_string()),
            on_error: None,
            retry: None,
        };

        let missing = resume_disposition_for_step(&step, &out_dir, &HashMap::new())
            .expect("missing fingerprint should rerun");
        assert_eq!(
            missing,
            ResumeDisposition::Rerun("missing_output_fingerprint")
        );

        let mut wrong = HashMap::new();
        wrong.insert("s1".to_string(), model_output_fingerprint("different"));
        let mismatch =
            resume_disposition_for_step(&step, &out_dir, &wrong).expect("mismatch should rerun");
        assert_eq!(
            mismatch,
            ResumeDisposition::Rerun("invalid_expected_artifact")
        );

        let mut exact = HashMap::new();
        exact.insert("s1".to_string(), model_output_fingerprint("hello"));
        let verified =
            resume_disposition_for_step(&step, &out_dir, &exact).expect("match should skip");
        assert_eq!(
            verified,
            ResumeDisposition::Skip("completed_artifact_verified")
        );

        std::fs::write(&artifact, vec![0xff, 0xfe, 0xfd]).expect("overwrite with invalid utf8");
        let err = resume_disposition_for_step(&step, &out_dir, &exact)
            .expect_err("invalid utf8 should surface artifact read failure");
        assert!(err
            .to_string()
            .contains("failed to read expected resume artifact"));

        let _ = std::fs::remove_dir_all(out_dir);
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

    #[test]
    fn scheduler_policy_source_as_str_matches_wire_values() {
        assert_eq!(
            SchedulerPolicySource::WorkflowOverride.as_str(),
            "workflow_override"
        );
        assert_eq!(SchedulerPolicySource::RunDefault.as_str(), "run_default");
        assert_eq!(
            SchedulerPolicySource::EngineDefault.as_str(),
            "engine_default"
        );
    }

    #[test]
    fn execution_policy_error_code_covers_all_kinds() {
        let denied = ExecutionPolicyError {
            kind: ExecutionPolicyErrorKind::Denied,
            step_id: "s1".to_string(),
            action_kind: "provider_call".to_string(),
            target_id: "default".to_string(),
            rule_id: None,
        };
        assert_eq!(denied.code(), DELEGATION_POLICY_DENY_CODE);

        let approval = ExecutionPolicyError {
            kind: ExecutionPolicyErrorKind::ApprovalRequired,
            step_id: "s1".to_string(),
            action_kind: "provider_call".to_string(),
            target_id: "default".to_string(),
            rule_id: None,
        };
        assert_eq!(approval.code(), "DELEGATION_POLICY_APPROVAL_REQUIRED");
    }

    #[test]
    fn execution_policy_error_display_includes_rule_id_for_denied() {
        let denied = ExecutionPolicyError {
            kind: ExecutionPolicyErrorKind::Denied,
            step_id: "step-a".to_string(),
            action_kind: "remote_exec".to_string(),
            target_id: "profile-x".to_string(),
            rule_id: Some("rule-7".to_string()),
        };
        let rendered = denied.to_string();
        assert!(rendered.contains("denied"));
        assert!(rendered.contains("rule_id=rule-7"));
    }

    #[test]
    fn execution_policy_error_display_handles_approval_without_rule_id() {
        let approval = ExecutionPolicyError {
            kind: ExecutionPolicyErrorKind::ApprovalRequired,
            step_id: "step-b".to_string(),
            action_kind: "provider_call".to_string(),
            target_id: "provider-1".to_string(),
            rule_id: None,
        };
        let rendered = approval.to_string();
        assert!(rendered.contains("requires approval"));
        assert!(!rendered.contains("rule_id="));
    }

    #[test]
    fn pause_reason_for_step_ignores_non_pause_guards() {
        let mut step = step_with_write_to("s1", None);
        step.guards.push(crate::adl::GuardSpec {
            kind: "retry".to_string(),
            config: HashMap::new(),
        });
        assert_eq!(pause_reason_for_step(&step), None);
    }

    #[test]
    fn scheduler_policy_for_run_returns_none_for_sequential_workflows() {
        let mut resolved = minimal_resolved();
        resolved.execution_plan.workflow_kind = WorkflowKind::Sequential;
        resolved.doc.run.workflow = Some(WorkflowSpec {
            id: None,
            kind: WorkflowKind::Sequential,
            max_concurrency: None,
            steps: vec![],
        });
        let policy = scheduler_policy_for_run(&resolved).expect("sequential policy");
        assert!(policy.is_none());
    }

    #[test]
    fn effective_max_concurrency_tracks_source_order() {
        let mut resolved = minimal_resolved();
        assert_eq!(
            effective_max_concurrency_with_source(&resolved).expect("engine default"),
            (
                DEFAULT_MAX_CONCURRENCY,
                SchedulerPolicySource::EngineDefault
            )
        );

        resolved.doc.run.defaults.max_concurrency = Some(6);
        assert_eq!(
            effective_max_concurrency_with_source(&resolved).expect("run default"),
            (6, SchedulerPolicySource::RunDefault)
        );

        resolved
            .doc
            .run
            .workflow
            .as_mut()
            .expect("workflow")
            .max_concurrency = Some(3);
        assert_eq!(
            effective_max_concurrency_with_source(&resolved).expect("workflow override"),
            (3, SchedulerPolicySource::WorkflowOverride)
        );
    }

    #[test]
    fn resume_disposition_for_step_skips_when_no_artifact_expected() {
        let step = step_with_write_to("s1", None);
        let out_dir = std::env::temp_dir();
        let completed_outputs = HashMap::new();
        let disposition =
            resume_disposition_for_step(&step, &out_dir, &completed_outputs).expect("disposition");
        assert_eq!(
            disposition,
            ResumeDisposition::Skip("completed_no_artifact_expected")
        );
    }

    #[test]
    fn resume_disposition_for_step_reruns_when_expected_artifact_missing() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let out_dir = std::env::temp_dir().join(format!("adl-resume-missing-{now}"));
        std::fs::create_dir_all(&out_dir).expect("create out_dir");
        let step = step_with_write_to("s1", Some("outputs/out.txt"));
        let completed_outputs = HashMap::new();
        let disposition =
            resume_disposition_for_step(&step, &out_dir, &completed_outputs).expect("disposition");
        assert_eq!(
            disposition,
            ResumeDisposition::Rerun("missing_expected_artifact")
        );
        let _ = std::fs::remove_dir_all(out_dir);
    }

    #[test]
    fn resume_disposition_for_step_reruns_when_fingerprint_missing() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let out_dir = std::env::temp_dir().join(format!("adl-resume-no-fp-{now}"));
        let artifact = out_dir.join("outputs/out.txt");
        std::fs::create_dir_all(artifact.parent().expect("parent")).expect("create parent");
        std::fs::write(&artifact, "ok").expect("write artifact");
        let step = step_with_write_to("s1", Some("outputs/out.txt"));
        let completed_outputs = HashMap::new();
        let disposition =
            resume_disposition_for_step(&step, &out_dir, &completed_outputs).expect("disposition");
        assert_eq!(
            disposition,
            ResumeDisposition::Rerun("missing_output_fingerprint")
        );
        let _ = std::fs::remove_dir_all(out_dir);
    }

    #[test]
    fn resume_disposition_for_step_reruns_on_fingerprint_mismatch() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let out_dir = std::env::temp_dir().join(format!("adl-resume-bad-fp-{now}"));
        let artifact = out_dir.join("outputs/out.txt");
        std::fs::create_dir_all(artifact.parent().expect("parent")).expect("create parent");
        std::fs::write(&artifact, "actual").expect("write artifact");
        let step = step_with_write_to("s1", Some("outputs/out.txt"));
        let mut completed_outputs = HashMap::new();
        completed_outputs.insert("s1".to_string(), model_output_fingerprint("expected"));
        let disposition =
            resume_disposition_for_step(&step, &out_dir, &completed_outputs).expect("disposition");
        assert_eq!(
            disposition,
            ResumeDisposition::Rerun("invalid_expected_artifact")
        );
        let _ = std::fs::remove_dir_all(out_dir);
    }

    #[test]
    fn resume_disposition_for_step_skips_when_artifact_and_fingerprint_match() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let out_dir = std::env::temp_dir().join(format!("adl-resume-ok-fp-{now}"));
        let artifact = out_dir.join("outputs/out.txt");
        std::fs::create_dir_all(artifact.parent().expect("parent")).expect("create parent");
        std::fs::write(&artifact, "stable output").expect("write artifact");
        let step = step_with_write_to("s1", Some("outputs/out.txt"));
        let mut completed_outputs = HashMap::new();
        completed_outputs.insert("s1".to_string(), model_output_fingerprint("stable output"));
        let disposition =
            resume_disposition_for_step(&step, &out_dir, &completed_outputs).expect("disposition");
        assert_eq!(
            disposition,
            ResumeDisposition::Skip("completed_artifact_verified")
        );
        let _ = std::fs::remove_dir_all(out_dir);
    }

    #[test]
    fn execution_policy_error_display_covers_approval_required_branch() {
        let err = ExecutionPolicyError {
            kind: ExecutionPolicyErrorKind::ApprovalRequired,
            step_id: "s-approve".to_string(),
            action_kind: "tool".to_string(),
            target_id: "fs.write".to_string(),
            rule_id: Some("rule-approval".to_string()),
        };

        assert_eq!(err.code(), "DELEGATION_POLICY_APPROVAL_REQUIRED");
        let rendered = err.to_string();
        assert!(rendered.contains("requires approval"));
        assert!(rendered.contains("rule_id=rule-approval"));
    }

    #[test]
    fn execute_called_workflow_rejects_unknown_workflow() {
        let resolved = minimal_resolved();
        let mut tr = crate::trace::Trace::new("run", "wf", "0.8");
        let caller_state = HashMap::new();
        let err = execute_called_workflow(
            "caller",
            "ns",
            "missing-workflow",
            &HashMap::new(),
            &resolved,
            &mut tr,
            false,
            false,
            Path::new("."),
            Path::new("."),
            &caller_state,
        )
        .expect_err("unknown called workflow should fail");
        assert!(err.to_string().contains("call references unknown workflow"));
    }

    #[test]
    fn execute_called_workflow_rejects_missing_state_binding_in_call_with() {
        let mut resolved = minimal_resolved();
        resolved.doc.workflows.insert(
            "callee".to_string(),
            WorkflowSpec {
                id: Some("callee".to_string()),
                kind: WorkflowKind::Sequential,
                max_concurrency: None,
                steps: vec![],
            },
        );

        let mut tr = crate::trace::Trace::new("run", "wf", "0.8");
        let mut call_with = HashMap::new();
        call_with.insert("topic".to_string(), "@state:inputs.missing".to_string());
        let caller_state = HashMap::new();

        let err = execute_called_workflow(
            "caller",
            "ns",
            "callee",
            &call_with,
            &resolved,
            &mut tr,
            false,
            false,
            Path::new("."),
            Path::new("."),
            &caller_state,
        )
        .expect_err("missing call.with binding should fail");
        let text = err.to_string();
        assert!(text.contains("failed to resolve call.with binding"));
        assert!(text.contains("caller step"));
    }

    #[test]
    fn execute_called_workflow_rejects_write_to_without_save_as() {
        let mut resolved = minimal_resolved();
        resolved.doc.workflows.insert(
            "callee".to_string(),
            WorkflowSpec {
                id: Some("callee".to_string()),
                kind: WorkflowKind::Sequential,
                max_concurrency: None,
                steps: vec![crate::adl::StepSpec {
                    id: Some("writer".to_string()),
                    write_to: Some("out/result.txt".to_string()),
                    // This intentionally triggers the write_to/save_as contract.
                    save_as: None,
                    ..crate::adl::StepSpec::default()
                }],
            },
        );

        let mut tr = crate::trace::Trace::new("run", "wf", "0.8");
        let caller_state = HashMap::new();
        let err = execute_called_workflow(
            "caller",
            "ns",
            "callee",
            &HashMap::new(),
            &resolved,
            &mut tr,
            false,
            false,
            Path::new("."),
            Path::new("."),
            &caller_state,
        )
        .expect_err("write_to without save_as should fail in called workflow");
        assert!(err
            .to_string()
            .contains("uses write_to but is missing save_as"));
    }

    #[test]
    fn execute_called_workflow_nested_call_failure_is_propagated() {
        let mut resolved = minimal_resolved();
        resolved.doc.workflows.insert(
            "parent".to_string(),
            WorkflowSpec {
                id: Some("parent".to_string()),
                kind: WorkflowKind::Sequential,
                max_concurrency: None,
                steps: vec![crate::adl::StepSpec {
                    id: Some("call-child".to_string()),
                    call: Some("missing-child".to_string()),
                    ..crate::adl::StepSpec::default()
                }],
            },
        );

        let mut tr = crate::trace::Trace::new("run", "wf", "0.8");
        let caller_state = HashMap::new();
        let err = execute_called_workflow(
            "caller",
            "ns",
            "parent",
            &HashMap::new(),
            &resolved,
            &mut tr,
            false,
            false,
            Path::new("."),
            Path::new("."),
            &caller_state,
        )
        .expect_err("nested call to missing workflow should fail");
        assert!(err.to_string().contains("unknown workflow"));
    }
}
