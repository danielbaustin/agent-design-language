use anyhow::{Context, Result};
use std::path::Path;

use crate::artifacts;

mod bundle_v1;
mod dataset;
mod shared;
mod trace_bundle_v2;

pub const DATASET_VERSION: u32 = 1;
pub const BUNDLE_VERSION: u32 = 1;
pub const TRACE_BUNDLE_VERSION: u32 = 2;

pub use bundle_v1::export_bundle_v1;
pub use dataset::{DatasetRowV1, StepRecord, SuggestionsSummary};
pub use trace_bundle_v2::{export_trace_bundle_v2, import_trace_bundle_v2, ImportedTraceBundleV2};

/// Export selected runs as deterministic JSONL rows.
///
/// # Examples
///
/// ```text
/// adl learn export --format jsonl --runs-dir .adl/runs --out /tmp/learning.jsonl
/// ```
pub fn export_jsonl(runs_root: &Path, run_ids: &[String], out_file: &Path) -> Result<usize> {
    let mut ids = shared::resolve_export_ids(runs_root, run_ids)?;

    let mut lines = Vec::new();
    for run_id in ids.drain(..) {
        let row = dataset::load_dataset_row(runs_root, &run_id)?;
        lines.push(serde_json::to_string(&row).context("serialize dataset row")?);
    }

    let mut out = lines.join("\n");
    if !out.is_empty() {
        out.push('\n');
    }
    artifacts::atomic_write(out_file, out.as_bytes())?;
    Ok(lines.len())
}

#[cfg(test)]
mod tests;
