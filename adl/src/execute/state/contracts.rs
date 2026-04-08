use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};

use crate::sandbox;

use super::{PauseState, RuntimeControlState, SteeringRecord};

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
pub enum RuntimeLifecyclePhase {
    Init,
    Execute,
    Complete,
    Teardown,
}

impl RuntimeLifecyclePhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::Execute => "execute",
            Self::Complete => "complete",
            Self::Teardown => "teardown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionBoundary {
    RuntimeInit,
    WorkflowCall,
    Pause,
    Resume,
    RunCompletion,
}

impl ExecutionBoundary {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeInit => "runtime_init",
            Self::WorkflowCall => "workflow_call",
            Self::Pause => "pause",
            Self::Resume => "resume",
            Self::RunCompletion => "run_completion",
        }
    }
}

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
    pub runtime_control: RuntimeControlState,
}
