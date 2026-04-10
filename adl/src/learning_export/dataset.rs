use anyhow::{anyhow, Result};
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::path::Path;

use crate::artifacts;

use super::shared::{read_json, read_json_opt, stable_fingerprint_hex, stable_hash_json};
use super::DATASET_VERSION;

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

pub(super) fn load_dataset_row_from_dir(run_dir: &Path, run_id: &str) -> Result<DatasetRowV1> {
    let safe_run_id = artifacts::validate_run_id_path_segment(run_id)?;
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
        run_id: safe_run_id,
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
