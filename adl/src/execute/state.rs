use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

use super::DELEGATION_POLICY_DENY_CODE;
use crate::sandbox;

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
pub(crate) const DEFAULT_MAX_CONCURRENCY: usize = 4;

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
