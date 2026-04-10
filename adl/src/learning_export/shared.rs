use anyhow::{anyhow, Context, Result};
use serde_json::Value as JsonValue;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone)]
pub(super) struct RunExportRef {
    pub run_id: String,
    pub run_dir: PathBuf,
}

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

#[cfg(test)]
pub(super) fn resolve_export_ids(runs_root: &Path, run_ids: &[String]) -> Result<Vec<String>> {
    Ok(resolve_export_runs(runs_root, run_ids)?
        .into_iter()
        .map(|run| run.run_id)
        .collect())
}

pub(super) fn resolve_export_runs(
    runs_root: &Path,
    run_ids: &[String],
) -> Result<Vec<RunExportRef>> {
    let discovered = discover_run_refs(runs_root)?;
    let mut by_id: BTreeMap<String, Vec<RunExportRef>> = BTreeMap::new();
    for run in discovered {
        by_id.entry(run.run_id.clone()).or_default().push(run);
    }

    let selected_ids = if run_ids.is_empty() {
        by_id.keys().cloned().collect::<Vec<_>>()
    } else {
        let mut ids = run_ids
            .iter()
            .map(|run_id| crate::artifacts::validate_run_id_path_segment(run_id))
            .collect::<Result<Vec<_>>>()?;
        ids.sort();
        ids.dedup();
        ids
    };

    let mut selected = Vec::new();
    for run_id in selected_ids {
        let matches = by_id.get(&run_id).ok_or_else(|| {
            anyhow!(
                "run_id '{}' was not found under '{}' or its milestone archive",
                run_id,
                runs_root.display()
            )
        })?;
        if matches.len() > 1 {
            let paths = matches
                .iter()
                .map(|run| run.run_dir.display().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(anyhow!(
                "run_id '{}' is ambiguous across run archive locations: {}",
                run_id,
                paths
            ));
        }
        selected.push(matches[0].clone());
    }

    Ok(selected)
}

#[cfg(test)]
pub(super) fn discover_run_ids(runs_root: &Path) -> Result<Vec<String>> {
    Ok(discover_run_refs(runs_root)?
        .into_iter()
        .map(|run| run.run_id)
        .collect())
}

pub(super) fn discover_run_refs(runs_root: &Path) -> Result<Vec<RunExportRef>> {
    let mut refs = Vec::new();
    let mut seen = BTreeSet::new();
    collect_flat_run_refs(runs_root, &mut refs, &mut seen)?;

    let archive_milestones = runs_root.join("milestones");
    if archive_milestones.is_dir() {
        for milestone in std::fs::read_dir(&archive_milestones).with_context(|| {
            format!("read archive milestones '{}'", archive_milestones.display())
        })? {
            let milestone = milestone?;
            if !milestone.file_type()?.is_dir() {
                continue;
            }
            let archived_runs = milestone.path().join("runs");
            if archived_runs.is_dir() {
                collect_flat_run_refs(&archived_runs, &mut refs, &mut seen)?;
            }
        }
    }

    refs.sort_by(|a, b| {
        a.run_id
            .cmp(&b.run_id)
            .then_with(|| a.run_dir.cmp(&b.run_dir))
    });
    Ok(refs)
}

fn collect_flat_run_refs(
    runs_root: &Path,
    refs: &mut Vec<RunExportRef>,
    seen_paths: &mut BTreeSet<PathBuf>,
) -> Result<()> {
    for entry in std::fs::read_dir(runs_root)
        .with_context(|| format!("read runs root '{}'", runs_root.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let run_id = entry.file_name().to_string_lossy().to_string();
        let run_dir = entry.path();
        if run_dir.join("run_summary.json").is_file() && seen_paths.insert(run_dir.clone()) {
            refs.push(RunExportRef { run_id, run_dir });
        }
    }
    Ok(())
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
