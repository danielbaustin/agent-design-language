use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use ::adl::{artifacts, godel};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct GodelRunCliSummary {
    run_id: String,
    workflow_id: String,
    stage_order: Vec<String>,
    experiment_record_path: String,
    obsmem_index_path: String,
}

pub(crate) fn real_godel(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|s| s.as_str()) else {
        return Err(anyhow::anyhow!(
            "godel subcommand required (supported: run)"
        ));
    };
    match cmd {
        "run" => real_godel_run(&args[1..]),
        other => Err(anyhow::anyhow!(
            "unknown godel subcommand '{other}' (supported: run)"
        )),
    }
}

pub(crate) fn real_godel_run(args: &[String]) -> Result<()> {
    let mut run_id: Option<String> = None;
    let mut workflow_id: Option<String> = None;
    let mut failure_code: Option<String> = None;
    let mut failure_summary: Option<String> = None;
    let mut evidence_refs: Vec<String> = Vec::new();
    let mut runs_dir: Option<PathBuf> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--run-id" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--run-id requires a value"));
                };
                run_id = Some(v.clone());
                i += 1;
            }
            "--workflow-id" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--workflow-id requires a value"));
                };
                workflow_id = Some(v.clone());
                i += 1;
            }
            "--failure-code" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--failure-code requires a value"));
                };
                failure_code = Some(v.clone());
                i += 1;
            }
            "--failure-summary" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--failure-summary requires a value"));
                };
                failure_summary = Some(v.clone());
                i += 1;
            }
            "--evidence-ref" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--evidence-ref requires a relative path"));
                };
                evidence_refs.push(v.clone());
                i += 1;
            }
            "--runs-dir" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--runs-dir requires a directory path"));
                };
                runs_dir = Some(PathBuf::from(v));
                i += 1;
            }
            other => {
                return Err(anyhow::anyhow!(
                    "unknown godel run arg '{other}' (supported: --run-id, --workflow-id, --failure-code, --failure-summary, --evidence-ref, --runs-dir)"
                ));
            }
        }
        i += 1;
    }

    let input = godel::StageLoopInput {
        run_id: run_id.ok_or_else(|| anyhow::anyhow!("godel run requires --run-id <id>"))?,
        workflow_id: workflow_id
            .ok_or_else(|| anyhow::anyhow!("godel run requires --workflow-id <id>"))?,
        failure_code: failure_code
            .ok_or_else(|| anyhow::anyhow!("godel run requires --failure-code <code>"))?,
        failure_summary: failure_summary
            .ok_or_else(|| anyhow::anyhow!("godel run requires --failure-summary <text>"))?,
        evidence_refs,
    };
    let runs_dir = runs_dir.unwrap_or_else(|| {
        artifacts::runs_root().unwrap_or_else(|_| PathBuf::from(".adl").join("runs"))
    });
    let result = godel::GodelStageLoopExecutor::new(godel::StageLoopConfig::default())
        .execute_and_persist(&input, &runs_dir)?;
    let summary = GodelRunCliSummary {
        run_id: result.run.record.run_id.clone(),
        workflow_id: result.run.record.workflow_id.clone(),
        stage_order: result
            .run
            .stage_order
            .iter()
            .map(|stage| stage.as_str().to_string())
            .collect(),
        experiment_record_path: result.experiment_record_rel_path.display().to_string(),
        obsmem_index_path: result.obsmem_index_rel_path.display().to_string(),
    };
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}
