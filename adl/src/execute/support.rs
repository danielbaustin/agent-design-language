use std::collections::{HashMap, HashSet};
use std::path::{Component, Path, PathBuf};

use anyhow::{anyhow, Context, Result};

use crate::sandbox;
use crate::trace::Trace;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ResumeDisposition {
    Skip(&'static str),
    Rerun(&'static str),
}

pub(super) fn pause_reason_for_step(step: &crate::resolve::ResolvedStep) -> Option<Option<String>> {
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

pub(super) fn progress_step_start(enabled: bool, tr: &Trace, step_id: &str, provider_id: &str) {
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

pub(super) fn progress_step_done(
    enabled: bool,
    tr: &Trace,
    step_id: &str,
    ok: bool,
    duration_ms: u128,
) {
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

pub(super) fn emit_step_output(
    step_id: &str,
    model_output: &str,
    stream_chunks: &[String],
    tr: &mut Trace,
) {
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

pub(super) fn stable_fingerprint_hex(bytes: &[u8]) -> String {
    // FNV-1a 64-bit (deterministic, dependency-free fingerprint for persisted metadata).
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in bytes {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

pub(super) fn model_output_fingerprint(output: &str) -> String {
    stable_fingerprint_hex(output.as_bytes())
}

pub(super) fn resume_disposition_for_step(
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

pub(super) fn emit_resume_note(enabled: bool, step_id: &str, action: &str, reason: &str) {
    if !enabled {
        return;
    }
    eprintln!(
        "RESUME step={} action={} reason={}",
        step_id, action, reason
    );
}

pub(super) fn validate_write_to(step_id: &str, write_to: &str) -> Result<()> {
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

pub(super) fn write_output(
    step_id: &str,
    out_dir: &Path,
    write_to: &str,
    contents: &str,
) -> Result<PathBuf> {
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

pub(super) fn resolve_state_inputs(
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

pub(super) fn missing_prompt_inputs(
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
