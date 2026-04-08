use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::artifacts;

use super::shared::{
    read_json, resolve_export_ids, stable_fingerprint_hex, validate_bundle_rel_path,
    validate_export_payload_safety,
};
use super::TRACE_BUNDLE_VERSION;

#[derive(Debug, Serialize, Deserialize)]
struct TraceBundleManifestV2 {
    trace_bundle_version: u32,
    run_count: usize,
    runs: Vec<String>,
    files: Vec<TraceBundleFileEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TraceBundleFileEntry {
    path: String,
    hash: String,
    size_bytes: usize,
}

#[derive(Debug, Serialize)]
struct TraceBundleRunMetadataV2 {
    trace_bundle_run_version: u32,
    run_id: String,
    workflow_id: String,
    adl_version: String,
    swarm_version: String,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_kind: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ImportedTraceBundleV2 {
    pub bundle_root: PathBuf,
    pub run_id: String,
    pub activation_log_path: PathBuf,
}

/// Export selected runs as trace bundle v2 under `trace_bundle_v2/`.
///
/// Required run files:
/// - `run.json`
/// - `steps.json`
/// - `run_summary.json`
/// - `run_status.json`
/// - `logs/activation_log.json`
pub fn export_trace_bundle_v2(
    runs_root: &Path,
    run_ids: &[String],
    out_dir: &Path,
) -> Result<usize> {
    let ids = resolve_export_ids(runs_root, run_ids)?;
    let bundle_root = out_dir.join("trace_bundle_v2");
    let runs_root_out = bundle_root.join("runs");

    if bundle_root.exists() {
        std::fs::remove_dir_all(&bundle_root)
            .with_context(|| format!("remove existing bundle root '{}'", bundle_root.display()))?;
    }
    std::fs::create_dir_all(&runs_root_out)
        .with_context(|| format!("create bundle runs root '{}'", runs_root_out.display()))?;

    let mut file_entries = Vec::new();
    for run_id in &ids {
        let safe_run_id = artifacts::validate_run_id_path_segment(run_id)?;
        let src_run_dir = runs_root.join(&safe_run_id);
        let out_run_dir = runs_root_out.join(&safe_run_id);
        std::fs::create_dir_all(&out_run_dir)
            .with_context(|| format!("create bundle run dir '{}'", out_run_dir.display()))?;

        let required = [
            "run.json",
            "steps.json",
            "run_summary.json",
            "run_status.json",
            "logs/activation_log.json",
        ];
        for rel in required {
            copy_trace_bundle_file(
                &bundle_root,
                &src_run_dir.join(rel),
                &out_run_dir.join(rel),
                &format!("runs/{safe_run_id}/{rel}"),
                &mut file_entries,
            )?;
        }

        for rel in ["learning/scores.json", "learning/suggestions.json"] {
            let src = src_run_dir.join(rel);
            if src.is_file() {
                copy_trace_bundle_file(
                    &bundle_root,
                    &src,
                    &out_run_dir.join(rel),
                    &format!("runs/{safe_run_id}/{rel}"),
                    &mut file_entries,
                )?;
            }
        }

        let summary: JsonValue = read_json(&src_run_dir.join("run_summary.json"))?;
        let status: JsonValue = read_json(&src_run_dir.join("run_status.json"))?;
        let metadata = TraceBundleRunMetadataV2 {
            trace_bundle_run_version: TRACE_BUNDLE_VERSION,
            run_id: safe_run_id.clone(),
            workflow_id: summary
                .get("workflow_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            adl_version: summary
                .get("adl_version")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            swarm_version: summary
                .get("swarm_version")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            status: summary
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            failure_kind: status
                .get("failure_kind")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        };
        write_trace_bundle_json(
            &bundle_root,
            &out_run_dir.join("metadata.json"),
            &format!("runs/{safe_run_id}/metadata.json"),
            &metadata,
            &mut file_entries,
        )?;
    }

    file_entries.sort_by(|a, b| a.path.cmp(&b.path));
    let manifest = TraceBundleManifestV2 {
        trace_bundle_version: TRACE_BUNDLE_VERSION,
        run_count: ids.len(),
        runs: ids,
        files: file_entries,
    };
    let manifest_bytes =
        serde_json::to_vec_pretty(&manifest).context("serialize trace bundle manifest")?;
    artifacts::atomic_write(&bundle_root.join("manifest.json"), &manifest_bytes)?;
    Ok(manifest.run_count)
}

/// Validate/import Trace Bundle v2 and return replay-relevant paths.
pub fn import_trace_bundle_v2(bundle_dir: &Path, run_id: &str) -> Result<ImportedTraceBundleV2> {
    let bundle_root = if bundle_dir.join("manifest.json").is_file() {
        bundle_dir.to_path_buf()
    } else if bundle_dir
        .join("trace_bundle_v2")
        .join("manifest.json")
        .is_file()
    {
        bundle_dir.join("trace_bundle_v2")
    } else {
        return Err(anyhow!(
            "trace bundle v2 manifest not found in '{}' (expected manifest.json or trace_bundle_v2/manifest.json)",
            bundle_dir.display()
        ));
    };

    let manifest_path = bundle_root.join("manifest.json");
    let manifest_raw = std::fs::read_to_string(&manifest_path)
        .with_context(|| format!("read trace bundle manifest '{}'", manifest_path.display()))?;
    let manifest: TraceBundleManifestV2 = serde_json::from_str(&manifest_raw)
        .with_context(|| format!("parse trace bundle manifest '{}'", manifest_path.display()))?;

    if manifest.trace_bundle_version != TRACE_BUNDLE_VERSION {
        return Err(anyhow!(
            "unsupported trace_bundle_version {} in '{}' (expected {})",
            manifest.trace_bundle_version,
            manifest_path.display(),
            TRACE_BUNDLE_VERSION
        ));
    }
    if !manifest.runs.iter().any(|r| r == run_id) {
        return Err(anyhow!("trace bundle does not contain run_id '{}'", run_id));
    }
    let mut sorted_runs = manifest.runs.clone();
    sorted_runs.sort();
    if sorted_runs != manifest.runs {
        return Err(anyhow!(
            "trace bundle runs list is not canonically sorted in '{}'",
            manifest_path.display()
        ));
    }

    let mut entries = manifest.files.clone();
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    if entries.iter().map(|e| &e.path).collect::<Vec<_>>()
        != manifest.files.iter().map(|e| &e.path).collect::<Vec<_>>()
    {
        return Err(anyhow!(
            "trace bundle file inventory is not canonically sorted in '{}'",
            manifest_path.display()
        ));
    }

    let by_path: BTreeMap<String, TraceBundleFileEntry> = manifest
        .files
        .iter()
        .cloned()
        .map(|entry| (entry.path.clone(), entry))
        .collect();

    for rel in required_trace_bundle_files(run_id) {
        if !by_path.contains_key(&rel) {
            return Err(anyhow!(
                "trace bundle v2 missing required file '{}' for run '{}'",
                rel,
                run_id
            ));
        }
    }

    for entry in &manifest.files {
        validate_bundle_rel_path(&entry.path)?;
        let full = bundle_root.join(&entry.path);
        if !full.is_file() {
            return Err(anyhow!(
                "trace bundle file '{}' is missing on disk",
                entry.path
            ));
        }
        let bytes = std::fs::read(&full)
            .with_context(|| format!("read bundle file '{}'", full.display()))?;
        if bytes.len() != entry.size_bytes {
            return Err(anyhow!(
                "trace bundle file '{}' size mismatch: manifest={} actual={}",
                entry.path,
                entry.size_bytes,
                bytes.len()
            ));
        }
        let actual_hash = stable_fingerprint_hex(&bytes);
        if actual_hash != entry.hash {
            return Err(anyhow!(
                "trace bundle file '{}' hash mismatch: manifest={} actual={}",
                entry.path,
                entry.hash,
                actual_hash
            ));
        }
        validate_export_payload_safety(&bundle_root, &entry.path, &bytes)?;
    }

    let activation_log_rel = format!("runs/{run_id}/logs/activation_log.json");
    Ok(ImportedTraceBundleV2 {
        bundle_root: bundle_root.clone(),
        run_id: run_id.to_string(),
        activation_log_path: bundle_root.join(activation_log_rel),
    })
}

fn write_trace_bundle_json<T: Serialize>(
    bundle_root: &Path,
    path: &Path,
    rel_path: &str,
    payload: &T,
    file_entries: &mut Vec<TraceBundleFileEntry>,
) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(payload)
        .with_context(|| format!("serialize trace bundle payload '{}'", rel_path))?;
    artifacts::atomic_write(path, &bytes)?;
    validate_export_payload_safety(bundle_root, rel_path, &bytes)?;
    file_entries.push(TraceBundleFileEntry {
        path: rel_path.to_string(),
        hash: stable_fingerprint_hex(&bytes),
        size_bytes: bytes.len(),
    });
    Ok(())
}

fn copy_trace_bundle_file(
    bundle_root: &Path,
    src: &Path,
    dest: &Path,
    rel_path: &str,
    file_entries: &mut Vec<TraceBundleFileEntry>,
) -> Result<()> {
    if !src.is_file() {
        return Err(anyhow!(
            "trace bundle v2 missing required file '{}'",
            rel_path
        ));
    }
    let bytes = std::fs::read(src)
        .with_context(|| format!("read required trace bundle file '{}'", src.display()))?;
    artifacts::atomic_write(dest, &bytes)?;
    validate_export_payload_safety(bundle_root, rel_path, &bytes)?;
    file_entries.push(TraceBundleFileEntry {
        path: rel_path.to_string(),
        hash: stable_fingerprint_hex(&bytes),
        size_bytes: bytes.len(),
    });
    Ok(())
}

fn required_trace_bundle_files(run_id: &str) -> Vec<String> {
    [
        "metadata.json",
        "run.json",
        "steps.json",
        "run_summary.json",
        "run_status.json",
        "logs/activation_log.json",
    ]
    .iter()
    .map(|rel| format!("runs/{run_id}/{rel}"))
    .collect()
}
