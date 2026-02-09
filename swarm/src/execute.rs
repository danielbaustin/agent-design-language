use std::collections::{HashMap, HashSet};
use std::path::{Component, Path, PathBuf};

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
        if meta.len() > MAX_FILE_BYTES {
            return Err(anyhow!(
                "input '{k}' file is too large ({} bytes > {} bytes): '{}'",
                meta.len(),
                MAX_FILE_BYTES,
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

/// Result of executing one step.
#[allow(dead_code)] // v0.1: returned for callers / future use; not all fields are read yet
#[derive(Debug, Clone)]
pub struct StepOutput {
    pub step_id: String,
    pub provider_id: String,
    pub model_output: String,
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub outputs: Vec<StepOutput>,
    pub artifacts: Vec<PathBuf>,
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
    out_dir: &Path,
) -> Result<ExecutionResult> {
    // Gate concurrent early (per our decision).
    if !matches!(
        resolved.doc.run.workflow.kind,
        crate::adl::WorkflowKind::Sequential
    ) {
        tr.run_failed("concurrent workflows are not supported in v0.1");
        return Err(anyhow!(
            "v0.1 does not support concurrent workflows. Use a single workflow with sequential steps, \
or upgrade once concurrency lands (planned v0.3)."
        ));
    }

    let mut outs = Vec::new();
    let mut artifacts = Vec::new();

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

        let result = (|| -> Result<StepOutput> {
            let p = step
                .effective_prompt_with_defaults(resolved)
                .ok_or_else(|| {
                    anyhow!(
                        "step '{}' has no effective prompt (step.prompt or task.prompt required)",
                        step_id
                    )
                })?;

            let missing = missing_prompt_inputs(&p, &step.inputs);
            if !missing.is_empty() {
                return Err(anyhow!(
                    "step '{}' missing input bindings for: {} (provide inputs or prior state)",
                    step_id,
                    missing.join(", ")
                ));
            }

            // v0.1: step-level inputs only (run.defaults currently has only `system`)
            let merged_inputs: HashMap<String, String> = step.inputs.clone();

            // Allow inputs to reference files via "@file:<path>".
            let inputs = materialize_inputs(merged_inputs, adl_base_dir)
                .with_context(|| format!("failed to materialize inputs for step '{}'", step_id))?;

            // Assemble a single text blob suitable for basic model consumption.
            let prompt_text = prompt::trace_prompt_assembly(&p, &inputs);
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

            Ok(StepOutput {
                step_id: step_id.clone(),
                provider_id: provider_id.to_string(),
                model_output,
            })
        })();

        match result {
            Ok(out) => {
                tr.step_finished(&step_id, true);

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

                outs.push(out);
            }
            Err(err) => {
                tr.step_finished(&step_id, false);
                tr.run_failed(&err.to_string());
                return Err(err);
            }
        }
    }

    Ok(ExecutionResult {
        outputs: outs,
        artifacts,
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
