use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const GODEL_RUNTIME_STATUS_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GodelRuntimeSurfaceStatus {
    pub status_version: u32,
    pub stage_order: Vec<String>,
    pub loaded_artifacts: Vec<String>,
    pub checks: Vec<String>,
}

pub fn load_v08_surface_status(repo_root: &Path) -> Result<GodelRuntimeSurfaceStatus> {
    let docs_root = repo_root.join("docs").join("milestones").join("v0.8");

    let workflow_template =
        read_json(&docs_root.join("godel_experiment_workflow.template.v1.json"))
            .context("load workflow template")?;
    let evidence_view = read_json(&docs_root.join("canonical_evidence_view.v1.example.json"))
        .context("load canonical evidence example")?;
    let mutation =
        read_json(&docs_root.join("mutation.v1.example.json")).context("load mutation example")?;
    let evaluation_plan = read_json(&docs_root.join("evaluation_plan.v1.example.json"))
        .context("load evaluation plan example")?;
    let experiment_record = read_json(&docs_root.join("experiment_record.v1.example.json"))
        .context("load experiment record example")?;
    let run_summary = read_json(&docs_root.join("run_summary.v1.example.json"))
        .context("load run summary example")?;
    let experiment_index = read_json(&docs_root.join("experiment_index_entry.v1.example.json"))
        .context("load experiment index example")?;

    let stage_order = read_stage_order(&workflow_template)?;
    validate_stage_order(&stage_order)?;
    validate_cross_links(
        &evidence_view,
        &mutation,
        &evaluation_plan,
        &experiment_record,
    )?;
    validate_index_and_summary(&run_summary, &experiment_index)?;

    Ok(GodelRuntimeSurfaceStatus {
        status_version: GODEL_RUNTIME_STATUS_VERSION,
        stage_order,
        loaded_artifacts: vec![
            "docs/milestones/v0.8/godel_experiment_workflow.template.v1.json".to_string(),
            "docs/milestones/v0.8/canonical_evidence_view.v1.example.json".to_string(),
            "docs/milestones/v0.8/mutation.v1.example.json".to_string(),
            "docs/milestones/v0.8/evaluation_plan.v1.example.json".to_string(),
            "docs/milestones/v0.8/experiment_record.v1.example.json".to_string(),
            "docs/milestones/v0.8/run_summary.v1.example.json".to_string(),
            "docs/milestones/v0.8/experiment_index_entry.v1.example.json".to_string(),
        ],
        checks: vec![
            "workflow stage order matches deterministic scientific loop".to_string(),
            "mutation/evaluation_plan/experiment_record references are consistent".to_string(),
            "run_summary and experiment_index include deterministic recovery fields".to_string(),
        ],
    })
}

pub fn repo_root_from_manifest() -> Result<PathBuf> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let Some(repo_root) = manifest.parent() else {
        bail!("unable to derive repository root from CARGO_MANIFEST_DIR");
    };
    Ok(repo_root.to_path_buf())
}

fn read_json(path: &Path) -> Result<Value> {
    let raw =
        fs::read_to_string(path).with_context(|| format!("failed to read '{}'", path.display()))?;
    let parsed: Value = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse '{}' as JSON", path.display()))?;
    Ok(parsed)
}

fn read_stage_order(template: &Value) -> Result<Vec<String>> {
    let Some(arr) = template.get("stage_order").and_then(Value::as_array) else {
        bail!("workflow template missing stage_order array");
    };
    let mut out = Vec::with_capacity(arr.len());
    for v in arr {
        let Some(s) = v.as_str() else {
            bail!("workflow stage_order must contain strings only");
        };
        out.push(s.to_string());
    }
    Ok(out)
}

fn validate_stage_order(stage_order: &[String]) -> Result<()> {
    let expected = [
        "failure",
        "hypothesis",
        "mutation",
        "experiment",
        "evaluation",
        "record",
    ];
    if stage_order != expected {
        bail!(
            "unexpected stage order: expected {:?}, got {:?}",
            expected,
            stage_order
        );
    }
    Ok(())
}

fn validate_cross_links(
    evidence_view: &Value,
    mutation: &Value,
    evaluation_plan: &Value,
    experiment_record: &Value,
) -> Result<()> {
    if evidence_view
        .get("schema_name")
        .and_then(Value::as_str)
        .unwrap_or_default()
        != "canonical_evidence_view"
    {
        bail!("canonical evidence example has unexpected schema_name");
    }

    let mutation_id = mutation
        .get("mutation_id")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("mutation example missing mutation_id"))?;
    if mutation_id.trim().is_empty() {
        bail!("mutation example mutation_id is empty");
    }
    let plan_id = evaluation_plan
        .get("plan_id")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("evaluation plan example missing plan_id"))?;
    if plan_id.trim().is_empty() {
        bail!("evaluation plan example plan_id is empty");
    }

    let exp_mutation_id = experiment_record
        .get("mutation")
        .and_then(|v| v.get("mutation_id"))
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("experiment record missing mutation.mutation_id"))?;
    if exp_mutation_id.trim().is_empty() {
        bail!("experiment record mutation.mutation_id is empty");
    }
    let exp_plan_id = experiment_record
        .get("evaluation_plan")
        .and_then(|v| v.get("evaluation_plan_id"))
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("experiment record missing evaluation_plan_id"))?;
    if exp_plan_id.trim().is_empty() {
        bail!("experiment record evaluation_plan_id is empty");
    }

    let mutation_plan_schema_name = mutation
        .get("evaluation_plan_ref")
        .and_then(|v| v.get("schema_name"))
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("mutation missing evaluation_plan_ref.schema_name"))?;
    if mutation_plan_schema_name != "evaluation_plan" {
        bail!("mutation evaluation_plan_ref.schema_name must be evaluation_plan");
    }

    let eval_schema_name = evaluation_plan
        .get("schema_name")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("evaluation plan example missing schema_name"))?;
    if eval_schema_name != "evaluation_plan" {
        bail!("evaluation plan example schema_name must be evaluation_plan");
    }

    Ok(())
}

fn validate_index_and_summary(run_summary: &Value, experiment_index: &Value) -> Result<()> {
    if run_summary
        .get("schema_version")
        .and_then(Value::as_str)
        .unwrap_or_default()
        != "run_summary.v1"
    {
        bail!("run summary example missing schema_version=run_summary.v1");
    }
    if experiment_index
        .get("schema_version")
        .and_then(Value::as_str)
        .unwrap_or_default()
        != "experiment_index_entry.v1"
    {
        bail!("experiment index example missing schema_version=experiment_index_entry.v1");
    }
    if experiment_index.get("improvement_delta").is_none() {
        bail!("experiment index example missing improvement_delta");
    }
    if experiment_index.get("experiment_seed").is_none() {
        bail!("experiment index example missing experiment_seed");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_and_validates_v08_surfaces() {
        let repo_root = repo_root_from_manifest().expect("repo root");
        let status = load_v08_surface_status(&repo_root).expect("status should load");
        assert_eq!(status.status_version, GODEL_RUNTIME_STATUS_VERSION);
        assert_eq!(
            status.stage_order,
            vec![
                "failure".to_string(),
                "hypothesis".to_string(),
                "mutation".to_string(),
                "experiment".to_string(),
                "evaluation".to_string(),
                "record".to_string(),
            ]
        );
        assert!(status
            .loaded_artifacts
            .iter()
            .all(|p| p.starts_with("docs/milestones/v0.8/")));
    }
}
