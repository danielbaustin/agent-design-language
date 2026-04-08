use anyhow::{anyhow, Context, Result};
use serde_json::Value as JsonValue;
use std::path::{Component, Path};

pub(super) fn validate_export_payload_safety(
    bundle_root: &Path,
    rel_path: &str,
    bytes: &[u8],
) -> Result<()> {
    let rendered = String::from_utf8_lossy(bytes);
    if rendered.contains("/Users/")
        || rendered.contains("/home/")
        || rendered.contains("gho_")
        || rendered.contains("sk-")
    {
        return Err(anyhow!(
            "bundle payload '{}' contains disallowed host path or token-like secret",
            rel_path
        ));
    }
    let absolute_bundle_root = bundle_root.display().to_string();
    if rendered.contains(&absolute_bundle_root) {
        return Err(anyhow!(
            "bundle payload '{}' leaked absolute bundle root path",
            rel_path
        ));
    }
    Ok(())
}

pub(super) fn validate_bundle_rel_path(path: &str) -> Result<()> {
    let p = Path::new(path);
    if p.is_absolute() {
        return Err(anyhow!("trace bundle contains absolute path '{}'", path));
    }
    if path.contains('\\') {
        return Err(anyhow!(
            "trace bundle contains non-canonical path separator in '{}'",
            path
        ));
    }
    for component in p.components() {
        if matches!(component, Component::ParentDir | Component::Prefix(_)) {
            return Err(anyhow!(
                "trace bundle contains traversal or prefix component in '{}'",
                path
            ));
        }
    }
    Ok(())
}

pub(super) fn resolve_export_ids(runs_root: &Path, run_ids: &[String]) -> Result<Vec<String>> {
    let mut ids = if run_ids.is_empty() {
        discover_run_ids(runs_root)?
    } else {
        let mut v = run_ids.to_vec();
        v.sort();
        v
    };
    ids.dedup();
    Ok(ids)
}

pub(super) fn discover_run_ids(runs_root: &Path) -> Result<Vec<String>> {
    let mut ids = Vec::new();
    for entry in std::fs::read_dir(runs_root)
        .with_context(|| format!("read runs root '{}'", runs_root.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let run_id = entry.file_name().to_string_lossy().to_string();
        if entry.path().join("run_summary.json").is_file() {
            ids.push(run_id);
        }
    }
    ids.sort();
    Ok(ids)
}

pub(super) fn read_json(path: &Path) -> Result<JsonValue> {
    let raw =
        std::fs::read_to_string(path).with_context(|| format!("read json '{}'", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("parse json '{}'", path.display()))
}

pub(super) fn read_json_opt(path: &Path) -> Result<Option<JsonValue>> {
    if !path.is_file() {
        return Ok(None);
    }
    Ok(Some(read_json(path)?))
}

pub(super) fn stable_hash_json(v: &JsonValue) -> Result<String> {
    let bytes = serde_json::to_vec(v).context("serialize json for hashing")?;
    Ok(stable_fingerprint_hex(&bytes))
}

pub(super) fn stable_fingerprint_hex(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in bytes {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}
