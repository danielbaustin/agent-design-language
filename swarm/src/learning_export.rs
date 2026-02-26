use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::path::Path;

use crate::artifacts;

pub const DATASET_VERSION: u32 = 1;

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

#[derive(Debug, Serialize)]
pub struct StepRecord {
    pub step_id: String,
    pub provider_id: String,
    pub provider_profile: Option<String>,
    pub status: String,
    pub output_pointer_hash: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct SuggestionsSummary {
    pub ids: Vec<String>,
    pub categories: Vec<String>,
}

pub fn export_jsonl(runs_root: &Path, run_ids: &[String], out_file: &Path) -> Result<usize> {
    let mut ids = if run_ids.is_empty() {
        discover_run_ids(runs_root)?
    } else {
        let mut v = run_ids.to_vec();
        v.sort();
        v
    };
    ids.dedup();

    let mut lines = Vec::new();
    for run_id in ids {
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
}
