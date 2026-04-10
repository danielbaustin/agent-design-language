use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::Path;

use crate::artifacts;

use super::dataset::load_dataset_row_from_dir;
use super::shared::{resolve_export_runs, stable_fingerprint_hex, validate_export_payload_safety};
use super::BUNDLE_VERSION;

#[derive(Debug, Serialize)]
struct BundleRunMetadataV1 {
    bundle_run_version: u32,
    run_id: String,
    workflow_id: String,
    adl_version: String,
    swarm_version: String,
    status: String,
    feedback_present: bool,
    pointers: BTreeMap<String, String>,
}

#[derive(Debug, Serialize)]
struct BundleManifestV1 {
    bundle_version: u32,
    run_count: usize,
    runs: Vec<String>,
    files: Vec<BundleFileEntry>,
}

#[derive(Debug, Serialize)]
struct BundleFileEntry {
    path: String,
    hash: String,
}

/// Export selected runs as bundle v1 under `learning_export_v1/`.
///
/// # Examples
///
/// ```text
/// adl learn export --format bundle --runs-dir .adl/runs --out /tmp/learning-bundle
/// ```
pub fn export_bundle_v1(runs_root: &Path, run_ids: &[String], out_dir: &Path) -> Result<usize> {
    let runs = resolve_export_runs(runs_root, run_ids)?;
    let bundle_root = out_dir.join("learning_export_v1");
    let runs_root_out = bundle_root.join("runs");

    if bundle_root.exists() {
        std::fs::remove_dir_all(&bundle_root)
            .with_context(|| format!("remove existing bundle root '{}'", bundle_root.display()))?;
    }
    std::fs::create_dir_all(&runs_root_out)
        .with_context(|| format!("create bundle runs root '{}'", runs_root_out.display()))?;

    let mut file_entries = Vec::new();
    let mut exported_ids = Vec::new();
    for run in &runs {
        let row = load_dataset_row_from_dir(&run.run_dir, &run.run_id)?;
        let run_id = &run.run_id;
        exported_ids.push(run_id.clone());
        let run_dir = runs_root_out.join(run_id);
        std::fs::create_dir_all(&run_dir)
            .with_context(|| format!("create bundle run dir '{}'", run_dir.display()))?;

        let metadata = BundleRunMetadataV1 {
            bundle_run_version: BUNDLE_VERSION,
            run_id: row.run_id.clone(),
            workflow_id: row.workflow_id.clone(),
            adl_version: row.adl_version.clone(),
            swarm_version: row.swarm_version.clone(),
            status: row.status.clone(),
            feedback_present: row.feedback_present,
            pointers: row.pointers.clone(),
        };
        write_bundle_json(
            &bundle_root,
            &run_dir.join("metadata.json"),
            &format!("runs/{run_id}/metadata.json"),
            &metadata,
            &mut file_entries,
        )?;

        write_bundle_json(
            &bundle_root,
            &run_dir.join("step_records.json"),
            &format!("runs/{run_id}/step_records.json"),
            &row.step_records,
            &mut file_entries,
        )?;

        if let Some(scores_summary) = row.scores_summary.as_ref() {
            write_bundle_json(
                &bundle_root,
                &run_dir.join("scores_summary.json"),
                &format!("runs/{run_id}/scores_summary.json"),
                scores_summary,
                &mut file_entries,
            )?;
        }

        write_bundle_json(
            &bundle_root,
            &run_dir.join("suggestions_summary.json"),
            &format!("runs/{run_id}/suggestions_summary.json"),
            &row.suggestions_summary,
            &mut file_entries,
        )?;
    }

    file_entries.sort_by(|a, b| a.path.cmp(&b.path));
    let manifest = BundleManifestV1 {
        bundle_version: BUNDLE_VERSION,
        run_count: exported_ids.len(),
        runs: exported_ids,
        files: file_entries,
    };
    let manifest_path = bundle_root.join("manifest.json");
    let manifest_bytes =
        serde_json::to_vec_pretty(&manifest).context("serialize bundle manifest")?;
    artifacts::atomic_write(&manifest_path, &manifest_bytes)?;

    Ok(manifest.run_count)
}

fn write_bundle_json<T: Serialize>(
    bundle_root: &Path,
    path: &Path,
    rel_path: &str,
    payload: &T,
    file_entries: &mut Vec<BundleFileEntry>,
) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(payload)
        .with_context(|| format!("serialize bundle payload '{}'", rel_path))?;
    artifacts::atomic_write(path, &bytes)?;
    validate_export_payload_safety(bundle_root, rel_path, &bytes)?;
    file_entries.push(BundleFileEntry {
        path: rel_path.to_string(),
        hash: stable_fingerprint_hex(&bytes),
    });
    Ok(())
}
