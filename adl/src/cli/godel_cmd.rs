use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use ::adl::{
    artifacts, godel,
    godel::affect_slice::{
        build_affect_godel_vertical_slice_artifact, persist_affect_godel_vertical_slice_artifact,
        AffectSliceInputs, PersistedAffectStateArtifact, PersistedReasoningGraphArtifact,
    },
    godel::cross_workflow::PersistedCrossWorkflowArtifact,
    godel::experiment_record::PersistedExperimentRecord,
    godel::obsmem_index::PersistedStageIndexEntry,
    godel::policy::{PersistedPolicyArtifact, PersistedPolicyComparisonArtifact},
    godel::prioritization::PersistedPrioritizationArtifact,
    godel::promotion::{PersistedEvalReportArtifact, PersistedPromotionDecisionArtifact},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct GodelRunCliSummary {
    run_id: String,
    workflow_id: String,
    stage_order: Vec<String>,
    hypothesis_path: String,
    policy_path: String,
    policy_comparison_path: String,
    prioritization_path: String,
    cross_workflow_path: String,
    eval_report_path: String,
    promotion_decision_path: String,
    experiment_record_path: String,
    obsmem_index_path: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct GodelInspectCliSummary {
    run_id: String,
    hypothesis_path: String,
    policy_path: String,
    policy_comparison_path: String,
    prioritization_path: String,
    cross_workflow_path: String,
    eval_report_path: String,
    promotion_decision_path: String,
    experiment_record_path: String,
    obsmem_index_path: String,
    failure_code: String,
    workflow_id: String,
    hypothesis_id: String,
    hypothesis_claim: String,
    policy_id: String,
    policy_mode_before: String,
    policy_mode_after: String,
    changed_policy_fields: Vec<String>,
    top_experiment_candidate_id: String,
    top_experiment_confidence: f64,
    prioritization_tie_break_rule: String,
    cross_workflow_learning_id: String,
    downstream_workflow_id: String,
    downstream_decision_id: String,
    downstream_decision_class: String,
    downstream_expected_behavior_change: String,
    evaluation_id: String,
    evaluation_score: u32,
    promotion_id: String,
    promotion_decision: String,
    promotion_reason: String,
    mutation_id: String,
    evaluation_decision: String,
    improvement_delta: i32,
    obsmem_index_key: String,
    experiment_outcome: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct GodelEvaluateCliSummary {
    failure_code: String,
    experiment_result: String,
    score_delta: i32,
    decision: String,
    rationale: String,
    evaluation_plan_example: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct GodelAffectSliceCliSummary {
    initial_run_id: String,
    adapted_run_id: String,
    godel_run_id: String,
    artifact_path: String,
    baseline_candidate_id: String,
    initial_selected_candidate_id: String,
    initial_selected_strategy: String,
    adapted_selected_candidate_id: String,
    adapted_selected_strategy: String,
    changed_output_surface: String,
    changed_downstream_decision: bool,
}

pub(crate) fn real_godel(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|s| s.as_str()) else {
        return Err(anyhow::anyhow!(
            "godel subcommand required (supported: run, evaluate, inspect, affect-slice)"
        ));
    };
    match cmd {
        "run" => real_godel_run(&args[1..]),
        "evaluate" => real_godel_evaluate(&args[1..]),
        "inspect" => real_godel_inspect(&args[1..]),
        "affect-slice" => real_godel_affect_slice(&args[1..]),
        other => Err(anyhow::anyhow!(
            "unknown godel subcommand '{other}' (supported: run, evaluate, inspect, affect-slice)"
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
        hypothesis_path: result.hypothesis_rel_path.display().to_string(),
        policy_path: result.policy_rel_path.display().to_string(),
        policy_comparison_path: result.policy_comparison_rel_path.display().to_string(),
        prioritization_path: result.prioritization_rel_path.display().to_string(),
        cross_workflow_path: result.cross_workflow_rel_path.display().to_string(),
        eval_report_path: result.eval_report_rel_path.display().to_string(),
        promotion_decision_path: result.promotion_decision_rel_path.display().to_string(),
        experiment_record_path: result.experiment_record_rel_path.display().to_string(),
        obsmem_index_path: result.obsmem_index_rel_path.display().to_string(),
    };
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

pub(crate) fn real_godel_inspect(args: &[String]) -> Result<()> {
    let mut run_id: Option<String> = None;
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
            "--runs-dir" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--runs-dir requires a directory path"));
                };
                runs_dir = Some(PathBuf::from(v));
                i += 1;
            }
            other => {
                return Err(anyhow::anyhow!(
                    "unknown godel inspect arg '{other}' (supported: --run-id, --runs-dir)"
                ));
            }
        }
        i += 1;
    }

    let run_id = run_id.ok_or_else(|| anyhow::anyhow!("godel inspect requires --run-id <id>"))?;
    let runs_dir = runs_dir.unwrap_or_else(|| {
        artifacts::runs_root().unwrap_or_else(|_| PathBuf::from(".adl").join("runs"))
    });

    let experiment_record_rel = PathBuf::from("runs")
        .join(&run_id)
        .join("godel")
        .join("experiment_record.runtime.v1.json");
    let hypothesis_rel = PathBuf::from("runs")
        .join(&run_id)
        .join("godel")
        .join("godel_hypothesis.v1.json");
    let policy_rel = PathBuf::from("runs")
        .join(&run_id)
        .join("godel")
        .join("godel_policy.v1.json");
    let policy_comparison_rel = PathBuf::from("runs")
        .join(&run_id)
        .join("godel")
        .join("godel_policy_comparison.v1.json");
    let prioritization_rel = PathBuf::from("runs")
        .join(&run_id)
        .join("godel")
        .join("godel_experiment_priority.v1.json");
    let cross_workflow_rel = PathBuf::from("runs")
        .join(&run_id)
        .join("godel")
        .join("godel_cross_workflow_learning.v1.json");
    let eval_report_rel = PathBuf::from("runs")
        .join(&run_id)
        .join("godel")
        .join("godel_eval_report.v1.json");
    let promotion_decision_rel = PathBuf::from("runs")
        .join(&run_id)
        .join("godel")
        .join("godel_promotion_decision.v1.json");
    let obsmem_index_rel = PathBuf::from("runs")
        .join(&run_id)
        .join("godel")
        .join("obsmem_index_entry.runtime.v1.json");
    let hypothesis_path = runs_dir
        .join(&run_id)
        .join("godel")
        .join("godel_hypothesis.v1.json");
    let policy_path = runs_dir
        .join(&run_id)
        .join("godel")
        .join("godel_policy.v1.json");
    let policy_comparison_path = runs_dir
        .join(&run_id)
        .join("godel")
        .join("godel_policy_comparison.v1.json");
    let prioritization_path = runs_dir
        .join(&run_id)
        .join("godel")
        .join("godel_experiment_priority.v1.json");
    let cross_workflow_path = runs_dir
        .join(&run_id)
        .join("godel")
        .join("godel_cross_workflow_learning.v1.json");
    let eval_report_path = runs_dir
        .join(&run_id)
        .join("godel")
        .join("godel_eval_report.v1.json");
    let promotion_decision_path = runs_dir
        .join(&run_id)
        .join("godel")
        .join("godel_promotion_decision.v1.json");
    let experiment_record_path = runs_dir
        .join(&run_id)
        .join("godel")
        .join("experiment_record.runtime.v1.json");
    let obsmem_index_path = runs_dir
        .join(&run_id)
        .join("godel")
        .join("obsmem_index_entry.runtime.v1.json");

    let hypothesis: godel::hypothesis::PersistedHypothesisArtifact =
        serde_json::from_str(&fs::read_to_string(&hypothesis_path).map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_IO: failed to read {}: {err}",
                hypothesis_rel.display()
            )
        })?)
        .map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_INVALID: failed to parse {}: {err}",
                hypothesis_rel.display()
            )
        })?;

    let record: PersistedExperimentRecord =
        serde_json::from_str(&fs::read_to_string(&experiment_record_path).map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_IO: failed to read {}: {err}",
                experiment_record_rel.display()
            )
        })?)
        .map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_INVALID: failed to parse {}: {err}",
                experiment_record_rel.display()
            )
        })?;

    let policy: PersistedPolicyArtifact =
        serde_json::from_str(&fs::read_to_string(&policy_path).map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_IO: failed to read {}: {err}",
                policy_rel.display()
            )
        })?)
        .map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_INVALID: failed to parse {}: {err}",
                policy_rel.display()
            )
        })?;

    let comparison: PersistedPolicyComparisonArtifact =
        serde_json::from_str(&fs::read_to_string(&policy_comparison_path).map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_IO: failed to read {}: {err}",
                policy_comparison_rel.display()
            )
        })?)
        .map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_INVALID: failed to parse {}: {err}",
                policy_comparison_rel.display()
            )
        })?;

    let prioritization: PersistedPrioritizationArtifact =
        serde_json::from_str(&fs::read_to_string(&prioritization_path).map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_IO: failed to read {}: {err}",
                prioritization_rel.display()
            )
        })?)
        .map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_INVALID: failed to parse {}: {err}",
                prioritization_rel.display()
            )
        })?;
    let cross_workflow: PersistedCrossWorkflowArtifact =
        serde_json::from_str(&fs::read_to_string(&cross_workflow_path).map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_IO: failed to read {}: {err}",
                cross_workflow_rel.display()
            )
        })?)
        .map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_INVALID: failed to parse {}: {err}",
                cross_workflow_rel.display()
            )
        })?;
    let eval_report: PersistedEvalReportArtifact =
        serde_json::from_str(&fs::read_to_string(&eval_report_path).map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_IO: failed to read {}: {err}",
                eval_report_rel.display()
            )
        })?)
        .map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_INVALID: failed to parse {}: {err}",
                eval_report_rel.display()
            )
        })?;
    let promotion_decision: PersistedPromotionDecisionArtifact = serde_json::from_str(
        &fs::read_to_string(&promotion_decision_path).map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_IO: failed to read {}: {err}",
                promotion_decision_rel.display()
            )
        })?,
    )
    .map_err(|err| {
        anyhow::anyhow!(
            "GODEL_INSPECT_INVALID: failed to parse {}: {err}",
            promotion_decision_rel.display()
        )
    })?;

    let index: PersistedStageIndexEntry =
        serde_json::from_str(&fs::read_to_string(&obsmem_index_path).map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_IO: failed to read {}: {err}",
                obsmem_index_rel.display()
            )
        })?)
        .map_err(|err| {
            anyhow::anyhow!(
                "GODEL_INSPECT_INVALID: failed to parse {}: {err}",
                obsmem_index_rel.display()
            )
        })?;

    if record.record.run_id != run_id || index.entry.run_id != run_id {
        return Err(anyhow::anyhow!(
            "GODEL_INSPECT_INVALID: persisted run_id did not match requested run_id"
        ));
    }

    let summary = GodelInspectCliSummary {
        run_id,
        hypothesis_path: hypothesis_rel.display().to_string(),
        policy_path: policy_rel.display().to_string(),
        policy_comparison_path: policy_comparison_rel.display().to_string(),
        prioritization_path: prioritization_rel.display().to_string(),
        cross_workflow_path: cross_workflow_rel.display().to_string(),
        eval_report_path: eval_report_rel.display().to_string(),
        promotion_decision_path: promotion_decision_rel.display().to_string(),
        experiment_record_path: experiment_record_rel.display().to_string(),
        obsmem_index_path: obsmem_index_rel.display().to_string(),
        failure_code: record.record.failure_code.clone(),
        workflow_id: record.record.workflow_id.clone(),
        hypothesis_id: record.record.hypothesis_id.clone(),
        hypothesis_claim: hypothesis.claim.clone(),
        policy_id: policy.policy_id.clone(),
        policy_mode_before: comparison.before_policy.policy_mode.clone(),
        policy_mode_after: comparison.after_policy.policy_mode.clone(),
        changed_policy_fields: comparison.changed_fields.clone(),
        top_experiment_candidate_id: prioritization
            .ranked_candidates
            .first()
            .map(|c| c.candidate_id.clone())
            .unwrap_or_default(),
        top_experiment_confidence: prioritization
            .ranked_candidates
            .first()
            .map(|c| c.confidence)
            .unwrap_or(0.0),
        prioritization_tie_break_rule: prioritization.tie_break_rule.clone(),
        cross_workflow_learning_id: cross_workflow.learning_id.clone(),
        downstream_workflow_id: cross_workflow.downstream_decision.workflow_id.clone(),
        downstream_decision_id: cross_workflow.downstream_decision.decision_id.clone(),
        downstream_decision_class: cross_workflow.downstream_decision.decision_class.clone(),
        downstream_expected_behavior_change: cross_workflow
            .downstream_decision
            .expected_behavior_change
            .clone(),
        evaluation_id: eval_report.evaluation_id.clone(),
        evaluation_score: eval_report.score,
        promotion_id: promotion_decision.promotion_id.clone(),
        promotion_decision: promotion_decision.decision.clone(),
        promotion_reason: promotion_decision.decision_reason.clone(),
        mutation_id: record.record.mutation_id.clone(),
        evaluation_decision: record.record.evaluation_decision.clone(),
        improvement_delta: record.record.improvement_delta,
        obsmem_index_key: index.entry.index_key.clone(),
        experiment_outcome: index.entry.experiment_outcome.clone(),
    };
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

pub(crate) fn real_godel_evaluate(args: &[String]) -> Result<()> {
    let mut failure_code: Option<String> = None;
    let mut experiment_result: Option<String> = None;
    let mut score_delta: Option<i32> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--failure-code" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--failure-code requires a value"));
                };
                failure_code = Some(v.clone());
                i += 1;
            }
            "--experiment-result" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--experiment-result requires a value"));
                };
                experiment_result = Some(v.clone());
                i += 1;
            }
            "--score-delta" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--score-delta requires an integer"));
                };
                score_delta = Some(
                    v.parse::<i32>()
                        .map_err(|_| anyhow::anyhow!("--score-delta must be a valid i32"))?,
                );
                i += 1;
            }
            other => {
                return Err(anyhow::anyhow!(
                    "unknown godel evaluate arg '{other}' (supported: --failure-code, --experiment-result, --score-delta)"
                ));
            }
        }
        i += 1;
    }

    let failure_code = failure_code
        .ok_or_else(|| anyhow::anyhow!("godel evaluate requires --failure-code <code>"))?;
    let experiment_result = experiment_result.ok_or_else(|| {
        anyhow::anyhow!("godel evaluate requires --experiment-result <ok|blocked>")
    })?;
    if !matches!(experiment_result.as_str(), "ok" | "blocked") {
        return Err(anyhow::anyhow!(
            "godel evaluate requires --experiment-result <ok|blocked>"
        ));
    }
    let score_delta = score_delta
        .ok_or_else(|| anyhow::anyhow!("godel evaluate requires --score-delta <int>"))?;

    let outcome =
        godel::evaluation::evaluate_experiment(&failure_code, &experiment_result, score_delta);
    let summary = GodelEvaluateCliSummary {
        failure_code,
        experiment_result,
        score_delta,
        decision: format!("{:?}", outcome.decision).to_lowercase(),
        rationale: outcome.rationale,
        evaluation_plan_example: "adl-spec/examples/v0.8/evaluation_plan.v1.example.json"
            .to_string(),
    };
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

pub(crate) fn real_godel_affect_slice(args: &[String]) -> Result<()> {
    let mut initial_run_id: Option<String> = None;
    let mut adapted_run_id: Option<String> = None;
    let mut godel_run_id: Option<String> = None;
    let mut aee_runs_dir: Option<PathBuf> = None;
    let mut godel_runs_dir: Option<PathBuf> = None;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--initial-run-id" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--initial-run-id requires a value"));
                };
                initial_run_id = Some(v.clone());
                i += 1;
            }
            "--adapted-run-id" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--adapted-run-id requires a value"));
                };
                adapted_run_id = Some(v.clone());
                i += 1;
            }
            "--godel-run-id" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--godel-run-id requires a value"));
                };
                godel_run_id = Some(v.clone());
                i += 1;
            }
            "--aee-runs-dir" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!("--aee-runs-dir requires a directory path"));
                };
                aee_runs_dir = Some(PathBuf::from(v));
                i += 1;
            }
            "--godel-runs-dir" => {
                let Some(v) = args.get(i + 1) else {
                    return Err(anyhow::anyhow!(
                        "--godel-runs-dir requires a directory path"
                    ));
                };
                godel_runs_dir = Some(PathBuf::from(v));
                i += 1;
            }
            other => {
                return Err(anyhow::anyhow!(
                    "unknown godel affect-slice arg '{other}' (supported: --initial-run-id, --adapted-run-id, --godel-run-id, --aee-runs-dir, --godel-runs-dir)"
                ));
            }
        }
        i += 1;
    }

    let initial_run_id = initial_run_id
        .ok_or_else(|| anyhow::anyhow!("godel affect-slice requires --initial-run-id <id>"))?;
    let adapted_run_id = adapted_run_id
        .ok_or_else(|| anyhow::anyhow!("godel affect-slice requires --adapted-run-id <id>"))?;
    let godel_run_id = godel_run_id
        .ok_or_else(|| anyhow::anyhow!("godel affect-slice requires --godel-run-id <id>"))?;
    let default_runs_root =
        artifacts::runs_root().unwrap_or_else(|_| PathBuf::from(".adl").join("runs"));
    let aee_runs_dir = aee_runs_dir.unwrap_or_else(|| default_runs_root.clone());
    let godel_runs_dir = godel_runs_dir.unwrap_or(default_runs_root);

    let initial_affect_path = aee_runs_dir
        .join(&initial_run_id)
        .join("learning")
        .join("affect_state.v1.json");
    let initial_graph_path = aee_runs_dir
        .join(&initial_run_id)
        .join("learning")
        .join("reasoning_graph.v1.json");
    let adapted_affect_path = aee_runs_dir
        .join(&adapted_run_id)
        .join("learning")
        .join("affect_state.v1.json");
    let adapted_graph_path = aee_runs_dir
        .join(&adapted_run_id)
        .join("learning")
        .join("reasoning_graph.v1.json");
    let hypothesis_path = godel_runs_dir
        .join(&godel_run_id)
        .join("godel")
        .join("godel_hypothesis.v1.json");
    let policy_path = godel_runs_dir
        .join(&godel_run_id)
        .join("godel")
        .join("godel_policy.v1.json");
    let prioritization_path = godel_runs_dir
        .join(&godel_run_id)
        .join("godel")
        .join("godel_experiment_priority.v1.json");

    let initial_affect: PersistedAffectStateArtifact =
        serde_json::from_str(&fs::read_to_string(&initial_affect_path).map_err(|err| {
            anyhow::anyhow!(
                "GODEL_AFFECT_SLICE_IO: failed to read runs/{}/learning/affect_state.v1.json: {err}",
                initial_run_id
            )
        })?)
        .map_err(|err| {
            anyhow::anyhow!(
                "GODEL_AFFECT_SLICE_INVALID: failed to parse runs/{}/learning/affect_state.v1.json: {err}",
                initial_run_id
            )
        })?;
    let initial_graph: PersistedReasoningGraphArtifact =
        serde_json::from_str(&fs::read_to_string(&initial_graph_path).map_err(|err| {
            anyhow::anyhow!(
                "GODEL_AFFECT_SLICE_IO: failed to read runs/{}/learning/reasoning_graph.v1.json: {err}",
                initial_run_id
            )
        })?)
        .map_err(|err| {
            anyhow::anyhow!(
                "GODEL_AFFECT_SLICE_INVALID: failed to parse runs/{}/learning/reasoning_graph.v1.json: {err}",
                initial_run_id
            )
        })?;
    let adapted_affect: PersistedAffectStateArtifact =
        serde_json::from_str(&fs::read_to_string(&adapted_affect_path).map_err(|err| {
            anyhow::anyhow!(
                "GODEL_AFFECT_SLICE_IO: failed to read runs/{}/learning/affect_state.v1.json: {err}",
                adapted_run_id
            )
        })?)
        .map_err(|err| {
            anyhow::anyhow!(
                "GODEL_AFFECT_SLICE_INVALID: failed to parse runs/{}/learning/affect_state.v1.json: {err}",
                adapted_run_id
            )
        })?;
    let adapted_graph: PersistedReasoningGraphArtifact =
        serde_json::from_str(&fs::read_to_string(&adapted_graph_path).map_err(|err| {
            anyhow::anyhow!(
                "GODEL_AFFECT_SLICE_IO: failed to read runs/{}/learning/reasoning_graph.v1.json: {err}",
                adapted_run_id
            )
        })?)
        .map_err(|err| {
            anyhow::anyhow!(
                "GODEL_AFFECT_SLICE_INVALID: failed to parse runs/{}/learning/reasoning_graph.v1.json: {err}",
                adapted_run_id
            )
        })?;
    let hypothesis: godel::hypothesis::PersistedHypothesisArtifact =
        serde_json::from_str(&fs::read_to_string(&hypothesis_path).map_err(|err| {
            anyhow::anyhow!(
                "GODEL_AFFECT_SLICE_IO: failed to read runs/{}/godel/godel_hypothesis.v1.json: {err}",
                godel_run_id
            )
        })?)
        .map_err(|err| {
            anyhow::anyhow!(
                "GODEL_AFFECT_SLICE_INVALID: failed to parse runs/{}/godel/godel_hypothesis.v1.json: {err}",
                godel_run_id
            )
        })?;
    let policy: PersistedPolicyArtifact =
        serde_json::from_str(&fs::read_to_string(&policy_path).map_err(|err| {
            anyhow::anyhow!(
                "GODEL_AFFECT_SLICE_IO: failed to read runs/{}/godel/godel_policy.v1.json: {err}",
                godel_run_id
            )
        })?)
        .map_err(|err| {
            anyhow::anyhow!(
            "GODEL_AFFECT_SLICE_INVALID: failed to parse runs/{}/godel/godel_policy.v1.json: {err}",
            godel_run_id
        )
        })?;
    let prioritization: PersistedPrioritizationArtifact =
        serde_json::from_str(&fs::read_to_string(&prioritization_path).map_err(|err| {
            anyhow::anyhow!(
                "GODEL_AFFECT_SLICE_IO: failed to read runs/{}/godel/godel_experiment_priority.v1.json: {err}",
                godel_run_id
            )
        })?)
        .map_err(|err| {
            anyhow::anyhow!(
                "GODEL_AFFECT_SLICE_INVALID: failed to parse runs/{}/godel/godel_experiment_priority.v1.json: {err}",
                godel_run_id
            )
        })?;

    let hypothesis_artifact_rel = PathBuf::from("runs")
        .join(&godel_run_id)
        .join("godel")
        .join("godel_hypothesis.v1.json");
    let policy_artifact_rel = PathBuf::from("runs")
        .join(&godel_run_id)
        .join("godel")
        .join("godel_policy.v1.json");
    let prioritization_artifact_rel = PathBuf::from("runs")
        .join(&godel_run_id)
        .join("godel")
        .join("godel_experiment_priority.v1.json");

    let artifact = build_affect_godel_vertical_slice_artifact(AffectSliceInputs {
        initial_affect: &initial_affect,
        initial_graph: &initial_graph,
        adapted_affect: &adapted_affect,
        adapted_graph: &adapted_graph,
        hypothesis: &hypothesis,
        hypothesis_artifact_path: &hypothesis_artifact_rel,
        policy: &policy,
        policy_artifact_path: &policy_artifact_rel,
        prioritization: &prioritization,
        prioritization_artifact_path: &prioritization_artifact_rel,
    })?;
    let artifact_rel =
        persist_affect_godel_vertical_slice_artifact(&godel_runs_dir, &godel_run_id, &artifact)?;

    let summary = GodelAffectSliceCliSummary {
        initial_run_id,
        adapted_run_id,
        godel_run_id,
        artifact_path: artifact_rel.display().to_string(),
        baseline_candidate_id: artifact.baseline_candidate_id.clone(),
        initial_selected_candidate_id: artifact
            .downstream_change
            .initial_selected_candidate_id
            .clone(),
        initial_selected_strategy: artifact.downstream_change.initial_selected_strategy.clone(),
        adapted_selected_candidate_id: artifact
            .downstream_change
            .adapted_selected_candidate_id
            .clone(),
        adapted_selected_strategy: artifact.downstream_change.adapted_selected_strategy.clone(),
        changed_output_surface: artifact.downstream_change.output_surface.clone(),
        changed_downstream_decision: artifact.downstream_change.changed,
    };
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}
