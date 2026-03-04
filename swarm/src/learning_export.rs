use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use crate::artifacts;

pub const DATASET_VERSION: u32 = 1;
pub const BUNDLE_VERSION: u32 = 1;
pub const TRACE_BUNDLE_VERSION: u32 = 2;

/// Deterministic learning export row (JSONL format).
#[derive(Debug, Serialize)]
pub struct DatasetRowV1 {
    pub dataset_version: u32,
    pub run_id: String,
    pub workflow_id: String,
    pub adl_version: String,
    pub swarm_version: String,
    pub status: String,
    pub feedback_present: bool,
    pub pointers: BTreeMap<String, String>,
    pub step_records: Vec<StepRecord>,
    pub scores_summary: Option<BTreeMap<String, JsonValue>>,
    pub suggestions_summary: SuggestionsSummary,
}

/// Stable per-step record embedded in dataset exports.
#[derive(Debug, Serialize)]
pub struct StepRecord {
    pub step_id: String,
    pub provider_id: String,
    pub provider_profile: Option<String>,
    pub status: String,
    pub output_pointer_hash: Option<String>,
}

/// Compact deterministic suggestions summary.
#[derive(Debug, Default, Serialize)]
pub struct SuggestionsSummary {
    pub ids: Vec<String>,
    pub categories: Vec<String>,
}

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

/// Export selected runs as deterministic JSONL rows.
///
/// # Examples
///
/// ```text
/// adl learn export --format jsonl --runs-dir .adl/runs --out /tmp/learning.jsonl
/// ```
pub fn export_jsonl(runs_root: &Path, run_ids: &[String], out_file: &Path) -> Result<usize> {
    let mut ids = resolve_export_ids(runs_root, run_ids)?;

    let mut lines = Vec::new();
    for run_id in ids.drain(..) {
        let row = load_dataset_row(runs_root, &run_id)?;
        lines.push(serde_json::to_string(&row).context("serialize dataset row")?);
    }

    let mut out = lines.join("\n");
    if !out.is_empty() {
        out.push('\n');
    }
    artifacts::atomic_write(out_file, out.as_bytes())?;
    Ok(lines.len())
}

/// Export selected runs as bundle v1 under `learning_export_v1/`.
///
/// # Examples
///
/// ```text
/// adl learn export --format bundle --runs-dir .adl/runs --out /tmp/learning-bundle
/// ```
pub fn export_bundle_v1(runs_root: &Path, run_ids: &[String], out_dir: &Path) -> Result<usize> {
    let ids = resolve_export_ids(runs_root, run_ids)?;
    let bundle_root = out_dir.join("learning_export_v1");
    let runs_root_out = bundle_root.join("runs");

    if bundle_root.exists() {
        std::fs::remove_dir_all(&bundle_root)
            .with_context(|| format!("remove existing bundle root '{}'", bundle_root.display()))?;
    }
    std::fs::create_dir_all(&runs_root_out)
        .with_context(|| format!("create bundle runs root '{}'", runs_root_out.display()))?;

    let mut file_entries = Vec::new();
    for run_id in &ids {
        let row = load_dataset_row(runs_root, run_id)?;
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
        run_count: ids.len(),
        runs: ids,
        files: file_entries,
    };
    let manifest_path = bundle_root.join("manifest.json");
    let manifest_bytes =
        serde_json::to_vec_pretty(&manifest).context("serialize bundle manifest")?;
    artifacts::atomic_write(&manifest_path, &manifest_bytes)?;

    Ok(manifest.run_count)
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
        let src_run_dir = runs_root.join(run_id);
        let out_run_dir = runs_root_out.join(run_id);
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
                &format!("runs/{run_id}/{rel}"),
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
                    &format!("runs/{run_id}/{rel}"),
                    &mut file_entries,
                )?;
            }
        }

        let summary: JsonValue = read_json(&src_run_dir.join("run_summary.json"))?;
        let status: JsonValue = read_json(&src_run_dir.join("run_status.json"))?;
        let metadata = TraceBundleRunMetadataV2 {
            trace_bundle_run_version: TRACE_BUNDLE_VERSION,
            run_id: run_id.to_string(),
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
            &format!("runs/{run_id}/metadata.json"),
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

fn validate_export_payload_safety(bundle_root: &Path, rel_path: &str, bytes: &[u8]) -> Result<()> {
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

fn validate_bundle_rel_path(path: &str) -> Result<()> {
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

fn resolve_export_ids(runs_root: &Path, run_ids: &[String]) -> Result<Vec<String>> {
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

fn discover_run_ids(runs_root: &Path) -> Result<Vec<String>> {
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

fn load_dataset_row(runs_root: &Path, run_id: &str) -> Result<DatasetRowV1> {
    let run_dir = runs_root.join(run_id);
    let run_summary_path = run_dir.join("run_summary.json");
    let steps_path = run_dir.join("steps.json");
    let scores_path = run_dir.join("learning").join("scores.json");
    let suggestions_path = run_dir.join("learning").join("suggestions.json");
    let feedback_path = run_dir.join("feedback.json");

    let run_summary: JsonValue = read_json(&run_summary_path)?;
    let steps: JsonValue = read_json(&steps_path)?;
    let scores: Option<JsonValue> = read_json_opt(&scores_path)?;
    let suggestions: Option<JsonValue> = read_json_opt(&suggestions_path)?;

    let workflow_id = run_summary
        .get("workflow_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let adl_version = run_summary
        .get("adl_version")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let swarm_version = run_summary
        .get("swarm_version")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let status = run_summary
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let mut pointers = BTreeMap::new();
    pointers.insert(
        "run_summary_hash".to_string(),
        stable_hash_json(&run_summary)?,
    );
    pointers.insert("steps_hash".to_string(), stable_hash_json(&steps)?);
    if let Some(scores) = scores.as_ref() {
        pointers.insert("scores_hash".to_string(), stable_hash_json(scores)?);
    }
    if let Some(suggestions) = suggestions.as_ref() {
        pointers.insert(
            "suggestions_hash".to_string(),
            stable_hash_json(suggestions)?,
        );
    }

    let mut step_records = Vec::new();
    if let Some(step_arr) = steps.as_array() {
        for s in step_arr {
            let step_id = s
                .get("step_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let provider_id = s
                .get("provider_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let status = s
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let output_pointer_hash = s
                .get("output_artifact_path")
                .and_then(|v| v.as_str())
                .map(|p| stable_fingerprint_hex(p.as_bytes()));
            step_records.push(StepRecord {
                step_id,
                provider_id,
                provider_profile: None,
                status,
                output_pointer_hash,
            });
        }
        step_records.sort_by(|a, b| a.step_id.cmp(&b.step_id));
    }

    let scores_summary = scores
        .as_ref()
        .and_then(|v| v.get("summary"))
        .and_then(|v| v.as_object())
        .map(|obj| {
            obj.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<BTreeMap<String, JsonValue>>()
        });

    let suggestions_summary = build_suggestions_summary(suggestions.as_ref())?;

    Ok(DatasetRowV1 {
        dataset_version: DATASET_VERSION,
        run_id: run_id.to_string(),
        workflow_id,
        adl_version,
        swarm_version,
        status,
        feedback_present: feedback_path.is_file(),
        pointers,
        step_records,
        scores_summary,
        suggestions_summary,
    })
}

fn build_suggestions_summary(suggestions: Option<&JsonValue>) -> Result<SuggestionsSummary> {
    let Some(suggestions) = suggestions else {
        return Ok(SuggestionsSummary::default());
    };
    let mut ids = Vec::new();
    let mut categories = Vec::new();
    let arr = suggestions
        .get("suggestions")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("suggestions.json missing 'suggestions' array"))?;
    for item in arr {
        if let Some(id) = item.get("id").and_then(|v| v.as_str()) {
            ids.push(id.to_string());
        }
        if let Some(category) = item.get("category").and_then(|v| v.as_str()) {
            categories.push(category.to_string());
        }
    }
    ids.sort();
    categories.sort();
    categories.dedup();
    Ok(SuggestionsSummary { ids, categories })
}

fn read_json(path: &Path) -> Result<JsonValue> {
    let raw =
        std::fs::read_to_string(path).with_context(|| format!("read json '{}'", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("parse json '{}'", path.display()))
}

fn read_json_opt(path: &Path) -> Result<Option<JsonValue>> {
    if !path.is_file() {
        return Ok(None);
    }
    Ok(Some(read_json(path)?))
}

fn stable_hash_json(v: &JsonValue) -> Result<String> {
    let bytes = serde_json::to_vec(v).context("serialize json for hashing")?;
    Ok(stable_fingerprint_hex(&bytes))
}

fn stable_fingerprint_hex(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in bytes {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_jsonl_deterministic_for_fixture_runs() {
        let base = std::env::temp_dir().join(format!("learn-export-{}", std::process::id()));
        let runs_root = base.join("runs");
        let run_dir = runs_root.join("r1");
        std::fs::create_dir_all(run_dir.join("learning")).unwrap();
        std::fs::write(
            run_dir.join("run_summary.json"),
            r#"{"workflow_id":"wf","adl_version":"0.7","swarm_version":"0.6.0","status":"success"}"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("steps.json"),
            r#"[{"step_id":"s1","provider_id":"p1","status":"success","output_artifact_path":"/tmp/out.txt"}]"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("learning").join("scores.json"),
            r#"{"summary":{"success_ratio":1.0,"retry_count":0}}"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("learning").join("suggestions.json"),
            r#"{"suggestions":[{"id":"sug-002","category":"b"},{"id":"sug-001","category":"a"}]}"#,
        )
        .unwrap();

        let out1 = base.join("one.jsonl");
        let out2 = base.join("two.jsonl");
        export_jsonl(&runs_root, &[], &out1).unwrap();
        export_jsonl(&runs_root, &[], &out2).unwrap();
        let a = std::fs::read(&out1).unwrap();
        let b = std::fs::read(&out2).unwrap();
        assert_eq!(a, b, "export jsonl must be byte-stable");
    }

    #[test]
    fn export_bundle_v1_is_deterministic_and_path_safe() {
        let base = std::env::temp_dir().join(format!("learn-bundle-{}", std::process::id()));
        let runs_root = base.join("runs");
        let run_dir = runs_root.join("r1");
        std::fs::create_dir_all(run_dir.join("learning")).unwrap();
        std::fs::write(
            run_dir.join("run_summary.json"),
            r#"{"workflow_id":"wf","adl_version":"0.7","swarm_version":"0.6.0","status":"success"}"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("steps.json"),
            r#"[{"step_id":"s1","provider_id":"p1","status":"success","output_artifact_path":"/Users/redacted/path.txt"}]"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("learning").join("suggestions.json"),
            r#"{"suggestions":[{"id":"sug-002","category":"security"},{"id":"sug-001","category":"retry"}]}"#,
        )
        .unwrap();

        let out1 = base.join("bundle-a");
        let out2 = base.join("bundle-b");
        export_bundle_v1(&runs_root, &[], &out1).unwrap();
        export_bundle_v1(&runs_root, &[], &out2).unwrap();

        let manifest_a =
            std::fs::read(out1.join("learning_export_v1").join("manifest.json")).unwrap();
        let manifest_b =
            std::fs::read(out2.join("learning_export_v1").join("manifest.json")).unwrap();
        assert_eq!(
            manifest_a, manifest_b,
            "bundle manifest must be byte-stable"
        );

        let manifest_json: serde_json::Value = serde_json::from_slice(&manifest_a).unwrap();
        for entry in manifest_json
            .get("files")
            .and_then(|v| v.as_array())
            .unwrap()
        {
            let rel = entry.get("path").and_then(|v| v.as_str()).unwrap();
            let expected_hash = entry.get("hash").and_then(|v| v.as_str()).unwrap();
            let bytes = std::fs::read(out1.join("learning_export_v1").join(rel)).unwrap();
            assert_eq!(
                expected_hash,
                stable_fingerprint_hex(&bytes),
                "manifest hash must match file content for {rel}"
            );
        }

        let steps = std::fs::read_to_string(
            out1.join("learning_export_v1")
                .join("runs")
                .join("r1")
                .join("step_records.json"),
        )
        .unwrap();
        assert!(
            !steps.contains("/Users/") && !steps.contains("/home/"),
            "bundle must not leak host paths: {steps}"
        );
        assert!(
            !steps.contains("gho_"),
            "bundle must not leak token-like secrets: {steps}"
        );
    }

    #[test]
    fn export_trace_bundle_v2_requires_activation_log() {
        let base =
            std::env::temp_dir().join(format!("trace-bundle-missing-{}", std::process::id()));
        let runs_root = base.join("runs");
        let run_dir = runs_root.join("r1");
        std::fs::create_dir_all(run_dir.join("logs")).unwrap();
        std::fs::create_dir_all(run_dir.join("learning")).unwrap();
        std::fs::write(
            run_dir.join("run.json"),
            r#"{"schema_version":"run_state.v1","run_id":"r1","workflow_id":"wf","version":"0.75","status":"success","error_message":null,"start_time_ms":1,"end_time_ms":2,"duration_ms":1,"execution_plan_hash":"abc","scheduler_max_concurrency":null,"scheduler_policy_source":null,"pause":null}"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("steps.json"),
            r#"[{"step_id":"s1","agent_id":"a","provider_id":"p","status":"success","output_artifact_path":"outputs/s1.txt"}]"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("run_summary.json"),
            r#"{"run_summary_version":1,"artifact_model_version":1,"run_id":"r1","workflow_id":"wf","adl_version":"0.75","swarm_version":"0.7.0","status":"success","counts":{"total_steps":1,"completed_steps":1,"failed_steps":0,"provider_call_count":1,"delegation_steps":0,"delegation_requires_verification_steps":0},"policy":{"security_envelope_enabled":false,"signing_required":false,"key_id_required":false,"verify_allowed_algs":[],"verify_allowed_key_sources":[],"sandbox_policy":"centralized_path_resolver_v1","security_denials_by_code":{}},"links":{"run_json":"run.json","steps_json":"steps.json","outputs_dir":"outputs","logs_dir":"logs","learning_dir":"learning","overlays_dir":"learning/overlays"}}"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("run_status.json"),
            r#"{"run_status_version":1,"run_id":"r1","workflow_id":"wf","overall_status":"succeeded","completed_steps":["s1"],"pending_steps":[],"attempt_counts_by_step":{"s1":1}}"#,
        )
        .unwrap();

        let out = base.join("out");
        let err = export_trace_bundle_v2(&runs_root, &[], &out)
            .expect_err("missing activation log should fail");
        assert!(
            err.to_string().contains("missing required file"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn export_trace_bundle_v2_is_deterministic_and_manifest_hashes_match() {
        let base = std::env::temp_dir().join(format!("trace-bundle-{}", std::process::id()));
        let runs_root = base.join("runs");
        let run_dir = runs_root.join("r1");
        std::fs::create_dir_all(run_dir.join("logs")).unwrap();
        std::fs::create_dir_all(run_dir.join("learning")).unwrap();
        std::fs::write(
            run_dir.join("run.json"),
            r#"{"schema_version":"run_state.v1","run_id":"r1","workflow_id":"wf","version":"0.75","status":"success","error_message":null,"start_time_ms":1,"end_time_ms":2,"duration_ms":1,"execution_plan_hash":"abc","scheduler_max_concurrency":null,"scheduler_policy_source":null,"pause":null}"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("steps.json"),
            r#"[{"step_id":"s1","agent_id":"a","provider_id":"p","status":"success","output_artifact_path":"outputs/s1.txt"}]"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("run_summary.json"),
            r#"{"run_summary_version":1,"artifact_model_version":1,"run_id":"r1","workflow_id":"wf","adl_version":"0.75","swarm_version":"0.7.0","status":"success","counts":{"total_steps":1,"completed_steps":1,"failed_steps":0,"provider_call_count":1,"delegation_steps":0,"delegation_requires_verification_steps":0},"policy":{"security_envelope_enabled":false,"signing_required":false,"key_id_required":false,"verify_allowed_algs":[],"verify_allowed_key_sources":[],"sandbox_policy":"centralized_path_resolver_v1","security_denials_by_code":{}},"links":{"run_json":"run.json","steps_json":"steps.json","outputs_dir":"outputs","logs_dir":"logs","learning_dir":"learning","overlays_dir":"learning/overlays"}}"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("run_status.json"),
            r#"{"run_status_version":1,"run_id":"r1","workflow_id":"wf","overall_status":"succeeded","failure_kind":null,"completed_steps":["s1"],"pending_steps":[],"attempt_counts_by_step":{"s1":1}}"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("logs").join("activation_log.json"),
            r#"{"activation_log_version":1,"ordering":"append_only_emission_order","stable_ids":{"step_id":"replay_stable_with_same_plan","delegation_id":"replay_stable_with_same_activation_log","run_id":"run_scoped_not_cross_run_stable"},"events":[{"RunStarted":{"ts":"2026-03-01T00:00:00.000Z","run_id":"r1","workflow_id":"wf","version":"0.75"}}]}"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("learning").join("scores.json"),
            r#"{"scores_version":1,"run_id":"r1","generated_from":{"artifact_model_version":1,"run_summary_version":1},"summary":{"success_ratio":1.0,"failure_count":0,"retry_count":0,"delegation_denied_count":0,"security_denied_count":0},"metrics":{"scheduler_max_parallel_observed":1}}"#,
        )
        .unwrap();

        let out1 = base.join("bundle-a");
        let out2 = base.join("bundle-b");
        export_trace_bundle_v2(&runs_root, &[], &out1).unwrap();
        export_trace_bundle_v2(&runs_root, &[], &out2).unwrap();

        let manifest_a = std::fs::read(out1.join("trace_bundle_v2").join("manifest.json")).unwrap();
        let manifest_b = std::fs::read(out2.join("trace_bundle_v2").join("manifest.json")).unwrap();
        assert_eq!(
            manifest_a, manifest_b,
            "trace bundle manifest must be byte-stable"
        );

        let manifest_json: serde_json::Value = serde_json::from_slice(&manifest_a).unwrap();
        for entry in manifest_json
            .get("files")
            .and_then(|v| v.as_array())
            .unwrap()
        {
            let rel = entry.get("path").and_then(|v| v.as_str()).unwrap();
            let expected_hash = entry.get("hash").and_then(|v| v.as_str()).unwrap();
            let bytes = std::fs::read(out1.join("trace_bundle_v2").join(rel)).unwrap();
            assert_eq!(
                expected_hash,
                stable_fingerprint_hex(&bytes),
                "manifest hash must match file content for {rel}"
            );
        }
    }

    #[test]
    fn import_trace_bundle_v2_accepts_valid_bundle_and_returns_activation_log_path() {
        let base =
            std::env::temp_dir().join(format!("trace-bundle-import-ok-{}", std::process::id()));
        let runs_root = base.join("runs");
        let run_dir = runs_root.join("r1");
        std::fs::create_dir_all(run_dir.join("logs")).unwrap();
        std::fs::create_dir_all(run_dir.join("learning")).unwrap();
        std::fs::write(
            run_dir.join("run.json"),
            r#"{"schema_version":"run_state.v1","run_id":"r1","workflow_id":"wf","version":"0.75","status":"success","error_message":null,"start_time_ms":1,"end_time_ms":2,"duration_ms":1,"execution_plan_hash":"abc","scheduler_max_concurrency":null,"scheduler_policy_source":null,"pause":null}"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("steps.json"),
            r#"[{"step_id":"s1","agent_id":"a","provider_id":"p","status":"success","output_artifact_path":"outputs/s1.txt"}]"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("run_summary.json"),
            r#"{"run_summary_version":1,"artifact_model_version":1,"run_id":"r1","workflow_id":"wf","adl_version":"0.75","swarm_version":"0.7.0","status":"success","counts":{"total_steps":1,"completed_steps":1,"failed_steps":0,"provider_call_count":1,"delegation_steps":0,"delegation_requires_verification_steps":0},"policy":{"security_envelope_enabled":false,"signing_required":false,"key_id_required":false,"verify_allowed_algs":[],"verify_allowed_key_sources":[],"sandbox_policy":"centralized_path_resolver_v1","security_denials_by_code":{}},"links":{"run_json":"run.json","steps_json":"steps.json","outputs_dir":"outputs","logs_dir":"logs","learning_dir":"learning","overlays_dir":"learning/overlays"}}"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("run_status.json"),
            r#"{"run_status_version":1,"run_id":"r1","workflow_id":"wf","overall_status":"succeeded","failure_kind":null,"completed_steps":["s1"],"pending_steps":[],"attempt_counts_by_step":{"s1":1}}"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("logs").join("activation_log.json"),
            r#"{"activation_log_version":1,"ordering":"append_only_emission_order","stable_ids":{"step_id":"replay_stable_with_same_plan","delegation_id":"replay_stable_with_same_activation_log","run_id":"run_scoped_not_cross_run_stable"},"events":[{"RunStarted":{"ts":"2026-03-01T00:00:00.000Z","run_id":"r1","workflow_id":"wf","version":"0.75"}}]}"#,
        )
        .unwrap();
        let out = base.join("bundle");
        export_trace_bundle_v2(&runs_root, &[], &out).unwrap();

        let imported = import_trace_bundle_v2(&out.join("trace_bundle_v2"), "r1").unwrap();
        assert_eq!(imported.run_id, "r1");
        assert!(imported.activation_log_path.is_file());
        assert!(imported
            .activation_log_path
            .ends_with("trace_bundle_v2/runs/r1/logs/activation_log.json"));
    }

    #[test]
    fn import_trace_bundle_v2_rejects_manifest_hash_mismatch() {
        let base = std::env::temp_dir().join(format!(
            "trace-bundle-import-bad-hash-{}",
            std::process::id()
        ));
        let runs_root = base.join("runs");
        let run_dir = runs_root.join("r1");
        std::fs::create_dir_all(run_dir.join("logs")).unwrap();
        std::fs::write(
            run_dir.join("run.json"),
            r#"{"schema_version":"run_state.v1","run_id":"r1","workflow_id":"wf","version":"0.75","status":"success","error_message":null,"start_time_ms":1,"end_time_ms":2,"duration_ms":1,"execution_plan_hash":"abc","scheduler_max_concurrency":null,"scheduler_policy_source":null,"pause":null}"#,
        )
        .unwrap();
        std::fs::write(run_dir.join("steps.json"), r#"[]"#).unwrap();
        std::fs::write(
            run_dir.join("run_summary.json"),
            r#"{"run_summary_version":1,"artifact_model_version":1,"run_id":"r1","workflow_id":"wf","adl_version":"0.75","swarm_version":"0.7.0","status":"success","counts":{"total_steps":0,"completed_steps":0,"failed_steps":0,"provider_call_count":0,"delegation_steps":0,"delegation_requires_verification_steps":0},"policy":{"security_envelope_enabled":false,"signing_required":false,"key_id_required":false,"verify_allowed_algs":[],"verify_allowed_key_sources":[],"sandbox_policy":"centralized_path_resolver_v1","security_denials_by_code":{}},"links":{"run_json":"run.json","steps_json":"steps.json","outputs_dir":"outputs","logs_dir":"logs","learning_dir":"learning","overlays_dir":"learning/overlays"}}"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("run_status.json"),
            r#"{"run_status_version":1,"run_id":"r1","workflow_id":"wf","overall_status":"succeeded","failure_kind":null,"completed_steps":[],"pending_steps":[],"attempt_counts_by_step":{}}"#,
        )
        .unwrap();
        std::fs::write(
            run_dir.join("logs").join("activation_log.json"),
            r#"{"activation_log_version":1,"ordering":"append_only_emission_order","stable_ids":{"step_id":"x","delegation_id":"x","run_id":"x"},"events":[]}"#,
        )
        .unwrap();
        let out = base.join("bundle");
        export_trace_bundle_v2(&runs_root, &[], &out).unwrap();
        let activation = out
            .join("trace_bundle_v2")
            .join("runs")
            .join("r1")
            .join("logs")
            .join("activation_log.json");
        std::fs::write(&activation, b"{\"tampered\":true}").unwrap();

        let err = import_trace_bundle_v2(&out.join("trace_bundle_v2"), "r1")
            .expect_err("tampered bundle should fail hash check");
        assert!(
            err.to_string().contains("hash mismatch") || err.to_string().contains("size mismatch"),
            "unexpected: {err}"
        );
    }
}
