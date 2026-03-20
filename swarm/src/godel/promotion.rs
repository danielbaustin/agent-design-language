use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::cross_workflow::PersistedCrossWorkflowArtifact;
use super::hypothesis::PersistedHypothesisArtifact;
use super::policy::PersistedPolicyArtifact;
use super::prioritization::PersistedPrioritizationArtifact;

pub const EVAL_REPORT_ARTIFACT_VERSION: &str = "godel_eval_report.v1";
pub const PROMOTION_DECISION_ARTIFACT_VERSION: &str = "godel_promotion_decision.v1";

#[derive(Debug, Clone, Copy)]
pub struct PromotionInputs<'a> {
    pub hypothesis: &'a PersistedHypothesisArtifact,
    pub hypothesis_artifact_path: &'a Path,
    pub policy: &'a PersistedPolicyArtifact,
    pub policy_artifact_path: &'a Path,
    pub prioritization: &'a PersistedPrioritizationArtifact,
    pub prioritization_artifact_path: &'a Path,
    pub cross_workflow: &'a PersistedCrossWorkflowArtifact,
    pub cross_workflow_artifact_path: &'a Path,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedEvalReportArtifact {
    pub artifact_version: String,
    pub evaluation_id: String,
    pub run_id: String,
    pub workflow_id: String,
    pub hypothesis_id: String,
    pub policy_id: String,
    pub prioritization_id: String,
    pub cross_workflow_learning_id: String,
    pub score: u32,
    pub confidence_basis: String,
    pub report: String,
    pub hypothesis_artifact_path: String,
    pub policy_artifact_path: String,
    pub prioritization_artifact_path: String,
    pub cross_workflow_artifact_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedPromotionDecisionArtifact {
    pub artifact_version: String,
    pub promotion_id: String,
    pub run_id: String,
    pub workflow_id: String,
    pub evaluation_id: String,
    pub decision: String,
    pub decision_reason: String,
    pub evaluation_artifact_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromotionError {
    Invalid(String),
    Io(String),
    Serialize(String),
}

impl std::fmt::Display for PromotionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(msg) => write!(f, "GODEL_PROMOTION_INVALID: {msg}"),
            Self::Io(msg) => write!(f, "GODEL_PROMOTION_IO: {msg}"),
            Self::Serialize(msg) => write!(f, "GODEL_PROMOTION_SERIALIZE: {msg}"),
        }
    }
}

impl std::error::Error for PromotionError {}

pub fn build_eval_and_promotion_artifacts(
    inputs: PromotionInputs<'_>,
) -> Result<
    (
        PersistedEvalReportArtifact,
        PersistedPromotionDecisionArtifact,
    ),
    PromotionError,
> {
    if inputs.hypothesis.run_id != inputs.policy.run_id
        || inputs.hypothesis.run_id != inputs.prioritization.run_id
        || inputs.hypothesis.run_id != inputs.cross_workflow.source_run_id
        || inputs.hypothesis.workflow_id != inputs.policy.workflow_id
        || inputs.hypothesis.workflow_id != inputs.prioritization.workflow_id
        || inputs.hypothesis.workflow_id != inputs.cross_workflow.source_workflow_id
    {
        return Err(PromotionError::Invalid(
            "all upstream artifacts must share run_id and workflow_id".to_string(),
        ));
    }

    let top_ranked = inputs
        .prioritization
        .ranked_candidates
        .first()
        .ok_or_else(|| {
            PromotionError::Invalid(
                "prioritization artifact must contain at least one ranked candidate".to_string(),
            )
        })?;

    let score = 70
        + u32::from(
            inputs.policy.after_policy.retry_budget > inputs.policy.before_policy.retry_budget,
        ) * 10
        + u32::from(top_ranked.confidence >= 0.8) * 10
        + u32::from(
            inputs.cross_workflow.downstream_decision.selected_strategy == "retry_budget_probe",
        ) * 5;

    let evaluation = PersistedEvalReportArtifact {
        artifact_version: EVAL_REPORT_ARTIFACT_VERSION.to_string(),
        evaluation_id: format!(
            "evaluation:{}:{}",
            inputs.hypothesis.run_id, top_ranked.candidate_id
        ),
        run_id: inputs.hypothesis.run_id.clone(),
        workflow_id: inputs
            .cross_workflow
            .downstream_decision
            .workflow_id
            .clone(),
        hypothesis_id: inputs.hypothesis.hypothesis_id.clone(),
        policy_id: inputs.policy.policy_id.clone(),
        prioritization_id: inputs.prioritization.prioritization_id.clone(),
        cross_workflow_learning_id: inputs.cross_workflow.learning_id.clone(),
        score,
        confidence_basis: format!(
            "failure_class={}, policy_mode={}, ranked_candidate={}, confidence={:.2}",
            inputs.hypothesis.failure_class,
            inputs.policy.after_policy.policy_mode,
            top_ranked.candidate_id,
            top_ranked.confidence
        ),
        report: format!(
            "Evaluate downstream workflow {} using ranked candidate {} and retry budget {}.",
            inputs.cross_workflow.downstream_decision.workflow_id,
            top_ranked.candidate_id,
            inputs.policy.after_policy.retry_budget
        ),
        hypothesis_artifact_path: normalize_rel_path(inputs.hypothesis_artifact_path)?,
        policy_artifact_path: normalize_rel_path(inputs.policy_artifact_path)?,
        prioritization_artifact_path: normalize_rel_path(inputs.prioritization_artifact_path)?,
        cross_workflow_artifact_path: normalize_rel_path(inputs.cross_workflow_artifact_path)?,
    };

    let decision = if score >= 85 { "promote" } else { "reject" }.to_string();
    let promotion = PersistedPromotionDecisionArtifact {
        artifact_version: PROMOTION_DECISION_ARTIFACT_VERSION.to_string(),
        promotion_id: format!(
            "promotion:{}:{}",
            inputs.hypothesis.run_id, top_ranked.candidate_id
        ),
        run_id: inputs.hypothesis.run_id.clone(),
        workflow_id: inputs
            .cross_workflow
            .downstream_decision
            .workflow_id
            .clone(),
        evaluation_id: evaluation.evaluation_id.clone(),
        decision: decision.clone(),
        decision_reason: format!(
            "Deterministic threshold decision: score={} -> {}",
            evaluation.score, decision
        ),
        evaluation_artifact_path: PathBuf::from("runs")
            .join(&inputs.hypothesis.run_id)
            .join("godel")
            .join("godel_eval_report.v1.json")
            .display()
            .to_string(),
    };

    Ok((evaluation, promotion))
}

pub fn persist_eval_report_artifact(
    runs_root: &Path,
    run_id: &str,
    artifact: &PersistedEvalReportArtifact,
) -> Result<PathBuf, PromotionError> {
    let dir = runs_root.join(run_id).join("godel");
    fs::create_dir_all(&dir)
        .map_err(|err| PromotionError::Io(format!("create godel dir failed: {err}")))?;
    let path = dir.join("godel_eval_report.v1.json");
    let bytes = serde_json::to_vec_pretty(artifact)
        .map_err(|err| PromotionError::Serialize(format!("eval report serialize failed: {err}")))?;
    fs::write(&path, bytes)
        .map_err(|err| PromotionError::Io(format!("eval report write failed: {err}")))?;
    Ok(PathBuf::from("runs")
        .join(run_id)
        .join("godel")
        .join("godel_eval_report.v1.json"))
}

pub fn persist_promotion_decision_artifact(
    runs_root: &Path,
    run_id: &str,
    artifact: &PersistedPromotionDecisionArtifact,
) -> Result<PathBuf, PromotionError> {
    let dir = runs_root.join(run_id).join("godel");
    fs::create_dir_all(&dir)
        .map_err(|err| PromotionError::Io(format!("create godel dir failed: {err}")))?;
    let path = dir.join("godel_promotion_decision.v1.json");
    let bytes = serde_json::to_vec_pretty(artifact).map_err(|err| {
        PromotionError::Serialize(format!("promotion decision serialize failed: {err}"))
    })?;
    fs::write(&path, bytes)
        .map_err(|err| PromotionError::Io(format!("promotion decision write failed: {err}")))?;
    Ok(PathBuf::from("runs")
        .join(run_id)
        .join("godel")
        .join("godel_promotion_decision.v1.json"))
}

fn normalize_rel_path(path: &Path) -> Result<String, PromotionError> {
    let rendered = path.display().to_string();
    if rendered.starts_with('/') || rendered.contains('\\') || rendered.contains(':') {
        return Err(PromotionError::Invalid(format!(
            "artifact path '{}' must be a safe relative path",
            rendered
        )));
    }
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::godel::cross_workflow::{
        DownstreamWorkflowDecision, PersistedCrossWorkflowArtifact,
    };
    use crate::godel::hypothesis::PersistedHypothesisArtifact;
    use crate::godel::policy::{PersistedPolicyArtifact, PolicyState};
    use crate::godel::prioritization::{
        PersistedPrioritizationArtifact, PrioritizationInputCandidate, RankedExperimentCandidate,
    };

    fn fixture_hypothesis() -> PersistedHypothesisArtifact {
        PersistedHypothesisArtifact {
            artifact_version: "godel_hypothesis.v1".to_string(),
            hypothesis_id: "hyp:run-1:tool_failure:00".to_string(),
            run_id: "run-1".to_string(),
            workflow_id: "wf-godel".to_string(),
            failure_id: "failure:run-1:tool_failure".to_string(),
            failure_class: "tool_failure".to_string(),
            claim: "Primary hypothesis".to_string(),
            confidence: 0.67,
            evidence_refs: vec!["runs/run-1/run_status.json".to_string()],
            related_run_refs: vec!["run-1".to_string()],
        }
    }

    fn fixture_policy() -> PersistedPolicyArtifact {
        PersistedPolicyArtifact {
            artifact_version: "godel_policy.v1".to_string(),
            policy_id: "policy:run-1:tool_failure".to_string(),
            run_id: "run-1".to_string(),
            workflow_id: "wf-godel".to_string(),
            hypothesis_id: "hyp:run-1:tool_failure:00".to_string(),
            hypothesis_artifact_path: "runs/run-1/godel/godel_hypothesis.v1.json".to_string(),
            source_signal: "hypothesis:tool_failure:godel_hypothesis.v1".to_string(),
            selection_reason: "deterministic".to_string(),
            before_policy: PolicyState {
                retry_budget: 1,
                experiment_budget: 1,
                target_surface: "tool-invocation-config".to_string(),
                policy_mode: "baseline".to_string(),
            },
            after_policy: PolicyState {
                retry_budget: 2,
                experiment_budget: 2,
                target_surface: "tool-invocation-config".to_string(),
                policy_mode: "adaptive_reviewed".to_string(),
            },
        }
    }

    fn fixture_prioritization() -> PersistedPrioritizationArtifact {
        PersistedPrioritizationArtifact {
            artifact_version: "godel_experiment_priority.v1".to_string(),
            prioritization_id: "prioritize:run-1:tool_failure".to_string(),
            run_id: "run-1".to_string(),
            workflow_id: "wf-godel".to_string(),
            hypothesis_id: "hyp:run-1:tool_failure:00".to_string(),
            policy_id: "policy:run-1:tool_failure".to_string(),
            hypothesis_artifact_path: "runs/run-1/godel/godel_hypothesis.v1.json".to_string(),
            policy_artifact_path: "runs/run-1/godel/godel_policy.v1.json".to_string(),
            tie_break_rule:
                "sort by priority_score desc, then confidence desc, then candidate_id asc"
                    .to_string(),
            input_candidates: vec![PrioritizationInputCandidate {
                candidate_id: "exp:retry-budget".to_string(),
                strategy: "retry_budget_probe".to_string(),
                target_surface: "tool-invocation-config".to_string(),
            }],
            ranked_candidates: vec![RankedExperimentCandidate {
                candidate_id: "exp:retry-budget".to_string(),
                strategy: "retry_budget_probe".to_string(),
                target_surface: "tool-invocation-config".to_string(),
                priority_score: 95,
                confidence: 0.86,
                ranking_reason: "deterministic".to_string(),
            }],
        }
    }

    fn fixture_cross_workflow() -> PersistedCrossWorkflowArtifact {
        PersistedCrossWorkflowArtifact {
            artifact_version: "godel_cross_workflow_learning.v1".to_string(),
            learning_id: "cross-workflow:run-1:exp:retry-budget".to_string(),
            source_run_id: "run-1".to_string(),
            source_workflow_id: "wf-godel".to_string(),
            source_hypothesis_id: "hyp:run-1:tool_failure:00".to_string(),
            source_policy_id: "policy:run-1:tool_failure".to_string(),
            source_prioritization_id: "prioritize:run-1:tool_failure".to_string(),
            source_hypothesis_artifact_path: "runs/run-1/godel/godel_hypothesis.v1.json"
                .to_string(),
            source_policy_artifact_path: "runs/run-1/godel/godel_policy.v1.json".to_string(),
            source_prioritization_artifact_path:
                "runs/run-1/godel/godel_experiment_priority.v1.json".to_string(),
            linkage_rule: "consume highest-ranked experiment candidate and map strategy to downstream workflow decision".to_string(),
            downstream_decision: DownstreamWorkflowDecision {
                workflow_id: "wf-aee-retry-budget-adaptation".to_string(),
                decision_id: "decision:run-1:exp:retry-budget".to_string(),
                selected_candidate_id: "exp:retry-budget".to_string(),
                selected_strategy: "retry_budget_probe".to_string(),
                decision_class: "cross_workflow_learning_update".to_string(),
                expected_behavior_change: "Apply retry budget 2 to downstream recovery workflow for failure_class=tool_failure.".to_string(),
            },
        }
    }

    #[test]
    fn build_eval_and_promotion_artifacts_is_deterministic() {
        let hypothesis = fixture_hypothesis();
        let policy = fixture_policy();
        let prioritization = fixture_prioritization();
        let cross_workflow = fixture_cross_workflow();
        let left = build_eval_and_promotion_artifacts(PromotionInputs {
            hypothesis: &hypothesis,
            hypothesis_artifact_path: Path::new("runs/run-1/godel/godel_hypothesis.v1.json"),
            policy: &policy,
            policy_artifact_path: Path::new("runs/run-1/godel/godel_policy.v1.json"),
            prioritization: &prioritization,
            prioritization_artifact_path: Path::new(
                "runs/run-1/godel/godel_experiment_priority.v1.json",
            ),
            cross_workflow: &cross_workflow,
            cross_workflow_artifact_path: Path::new(
                "runs/run-1/godel/godel_cross_workflow_learning.v1.json",
            ),
        })
        .expect("left");
        let right = build_eval_and_promotion_artifacts(PromotionInputs {
            hypothesis: &hypothesis,
            hypothesis_artifact_path: Path::new("runs/run-1/godel/godel_hypothesis.v1.json"),
            policy: &policy,
            policy_artifact_path: Path::new("runs/run-1/godel/godel_policy.v1.json"),
            prioritization: &prioritization,
            prioritization_artifact_path: Path::new(
                "runs/run-1/godel/godel_experiment_priority.v1.json",
            ),
            cross_workflow: &cross_workflow,
            cross_workflow_artifact_path: Path::new(
                "runs/run-1/godel/godel_cross_workflow_learning.v1.json",
            ),
        })
        .expect("right");
        assert_eq!(left, right);
        assert_eq!(left.0.score, 95);
        assert_eq!(left.1.decision, "promote");
    }
}
