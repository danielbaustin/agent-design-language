use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};

use crate::prompt;
use crate::provider;
use crate::resolve::AdlResolved;
use crate::trace::Trace;

/// Replace any input values that start with "@file:<path>" with the file contents.
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
    const MAX_FILE_BYTES: u64 = 512 * 1024; // 512 KiB per input file (v0.1 safety bound)

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
        let path: PathBuf = if candidate.is_absolute() || candidate.starts_with(base_dir) {
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
        if meta.len() > MAX_FILE_BYTES {
            return Err(anyhow!(
                "input '{k}' file is too large ({} bytes > {} bytes): '{}'",
                meta.len(),
                MAX_FILE_BYTES,
                path.display()
            ));
        }

        let bytes = std::fs::read(&path).with_context(|| {
            format!("failed to read input file for '{k}': '{}'", path.display())
        })?;
        let mut text = String::from_utf8(bytes).with_context(|| {
            format!("input '{k}' file is not valid UTF-8: '{}'", path.display())
        })?;

        // Normalize newlines for stable hashing / traces.
        if text.contains("\r\n") {
            text = text.replace("\r\n", "\n");
        }

        *v = text;
    }

    Ok(inputs)
}

/// Result of executing one step.
#[allow(dead_code)] // v0.1: returned for callers / future use; not all fields are read yet
#[derive(Debug, Clone)]
pub struct StepOutput {
    pub step_id: String,
    pub provider_id: String,
    pub model_output: String,
}

/// Execute the resolved run in **sequential** mode (v0.1).
///
/// v0.1 behavior:
/// - blocking provider calls
/// - prints outputs to stdout (caller can choose to print or not)
pub fn execute_sequential(
    resolved: &AdlResolved,
    tr: &mut Trace,
    print_outputs: bool,
    adl_base_dir: &Path,
) -> Result<Vec<StepOutput>> {
    // Gate concurrent early (per our decision).
    if !matches!(
        resolved.doc.run.workflow.kind,
        crate::adl::WorkflowKind::Sequential
    ) {
        return Err(anyhow!(
            "workflow kind {:?} is not implemented in v0.1 (only 'sequential')",
            resolved.doc.run.workflow.kind
        ));
    }

    let mut outs = Vec::new();

    for step in &resolved.steps {
        let step_id = step.id.clone();

        let agent_id: &str = step.agent.as_deref().unwrap_or("<unresolved-agent>");
        let task_id: &str = step.task.as_deref().unwrap_or("<unresolved-task>");
        let provider_id: &str = step.provider.as_deref().unwrap_or("<unresolved-provider>");

        tr.step_started(&step_id, agent_id, provider_id, task_id);

        let p = step.effective_prompt(resolved).ok_or_else(|| {
            anyhow!(
                "step '{}' has no effective prompt (step.prompt or task.prompt required)",
                step_id
            )
        })?;

        // v0.1: step-level inputs only (run.defaults currently has only `system`)
        let merged_inputs: HashMap<String, String> = step.inputs.clone();

        // Allow inputs to reference files via "@file:<path>".
        let inputs = materialize_inputs(merged_inputs, adl_base_dir)
            .with_context(|| format!("failed to materialize inputs for step '{}'", step_id))?;

        // If inputs aren't coming through, fail loudly (this matches the symptom you're seeing).
        if inputs.is_empty() {
            return Err(anyhow!(
                "step '{}' has no inputs after resolution/materialization; expected doc_1/doc_2/doc_3",
                step_id
            ));
        }

        // Assemble a single text blob suitable for basic model consumption.
        let prompt_text = prompt::trace_prompt_assembly(p, &inputs);
        let prompt_hash = prompt::hash_string(&prompt_text);
        tr.prompt_assembled(&step_id, &prompt_hash);

        // Build provider from doc.providers[provider_id]
        let spec = resolved.doc.providers.get(provider_id).with_context(|| {
            format!(
                "step '{}' references unknown provider '{}'",
                step_id, provider_id
            )
        })?;

        let prov = provider::build_provider(spec).with_context(|| {
            format!(
                "failed to build provider '{}' for step '{}'",
                provider_id, step_id
            )
        })?;

        let model_output = prov.complete(&prompt_text).with_context(|| {
            format!(
                "provider '{}' complete() failed for step '{}'",
                provider_id, step_id
            )
        })?;

        tr.step_finished(&step_id, true);

        if print_outputs {
            println!("--- step: {} ---", step_id);
            println!("{}", model_output.trim_end());
            println!();
        }

        outs.push(StepOutput {
            step_id,
            provider_id: provider_id.to_string(),
            model_output,
        });
    }

    Ok(outs)
}
