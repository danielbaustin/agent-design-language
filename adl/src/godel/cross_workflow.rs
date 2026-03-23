use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::hypothesis::PersistedHypothesisArtifact;
use super::policy::PersistedPolicyArtifact;
use super::prioritization::{PersistedPrioritizationArtifact, RankedExperimentCandidate};

pub const CROSS_WORKFLOW_ARTIFACT_VERSION: &str = "godel_cross_workflow_learning.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownstreamWorkflowDecision {
    pub workflow_id: String,
    pub decision_id: String,
    pub selected_candidate_id: String,
    pub selected_strategy: String,
    pub decision_class: String,
    pub expected_behavior_change: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedCrossWorkflowArtifact {
    pub artifact_version: String,
    pub learning_id: String,
    pub source_run_id: String,
    pub source_workflow_id: String,
    pub source_hypothesis_id: String,
    pub source_policy_id: String,
    pub source_prioritization_id: String,
    pub source_hypothesis_artifact_path: String,
    pub source_policy_artifact_path: String,
    pub source_prioritization_artifact_path: String,
    pub linkage_rule: String,
    pub downstream_decision: DownstreamWorkflowDecision,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrossWorkflowError {
    Invalid(String),
    Io(String),
    Serialize(String),
}

impl std::fmt::Display for CrossWorkflowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(msg) => write!(f, "GODEL_CROSS_WORKFLOW_INVALID: {msg}"),
            Self::Io(msg) => write!(f, "GODEL_CROSS_WORKFLOW_IO: {msg}"),
            Self::Serialize(msg) => write!(f, "GODEL_CROSS_WORKFLOW_SERIALIZE: {msg}"),
        }
    }
}

impl std::error::Error for CrossWorkflowError {}

pub fn build_cross_workflow_artifact(
    hypothesis: &PersistedHypothesisArtifact,
    hypothesis_artifact_path: &Path,
    policy: &PersistedPolicyArtifact,
    policy_artifact_path: &Path,
    prioritization: &PersistedPrioritizationArtifact,
    prioritization_artifact_path: &Path,
) -> Result<PersistedCrossWorkflowArtifact, CrossWorkflowError> {
    if hypothesis.run_id != policy.run_id
        || hypothesis.run_id != prioritization.run_id
        || hypothesis.workflow_id != policy.workflow_id
        || hypothesis.workflow_id != prioritization.workflow_id
    {
        return Err(CrossWorkflowError::Invalid(
            "hypothesis, policy, and prioritization artifacts must share run_id and workflow_id"
                .to_string(),
        ));
    }

    let selected = prioritization.ranked_candidates.first().ok_or_else(|| {
        CrossWorkflowError::Invalid(
            "prioritization artifact must contain at least one ranked candidate".to_string(),
        )
    })?;
    let downstream_decision =
        build_downstream_decision(hypothesis, policy, prioritization, selected);

    Ok(PersistedCrossWorkflowArtifact {
        artifact_version: CROSS_WORKFLOW_ARTIFACT_VERSION.to_string(),
        learning_id: format!(
            "cross-workflow:{}:{}",
            hypothesis.run_id, selected.candidate_id
        ),
        source_run_id: hypothesis.run_id.clone(),
        source_workflow_id: hypothesis.workflow_id.clone(),
        source_hypothesis_id: hypothesis.hypothesis_id.clone(),
        source_policy_id: policy.policy_id.clone(),
        source_prioritization_id: prioritization.prioritization_id.clone(),
        source_hypothesis_artifact_path: normalize_rel_path(hypothesis_artifact_path)?,
        source_policy_artifact_path: normalize_rel_path(policy_artifact_path)?,
        source_prioritization_artifact_path: normalize_rel_path(prioritization_artifact_path)?,
        linkage_rule:
            "consume highest-ranked experiment candidate and map strategy to downstream workflow decision"
                .to_string(),
        downstream_decision,
    })
}

pub fn persist_cross_workflow_artifact(
    runs_root: &Path,
    run_id: &str,
    artifact: &PersistedCrossWorkflowArtifact,
) -> Result<PathBuf, CrossWorkflowError> {
    let dir = runs_root.join(run_id).join("godel");
    fs::create_dir_all(&dir)
        .map_err(|err| CrossWorkflowError::Io(format!("create godel dir failed: {err}")))?;
    let path = dir.join("godel_cross_workflow_learning.v1.json");
    let bytes = serde_json::to_vec_pretty(artifact).map_err(|err| {
        CrossWorkflowError::Serialize(format!("cross-workflow artifact serialize failed: {err}"))
    })?;
    fs::write(&path, bytes).map_err(|err| {
        CrossWorkflowError::Io(format!("cross-workflow artifact write failed: {err}"))
    })?;
    Ok(PathBuf::from("runs")
        .join(run_id)
        .join("godel")
        .join("godel_cross_workflow_learning.v1.json"))
}

fn build_downstream_decision(
    hypothesis: &PersistedHypothesisArtifact,
    policy: &PersistedPolicyArtifact,
    prioritization: &PersistedPrioritizationArtifact,
    selected: &RankedExperimentCandidate,
) -> DownstreamWorkflowDecision {
    let (workflow_id, expected_behavior_change) = match selected.strategy.as_str() {
        "retry_budget_probe" => (
            "wf-aee-retry-budget-adaptation".to_string(),
            format!(
                "Apply retry budget {} to downstream recovery workflow for failure_class={}.",
                policy.after_policy.retry_budget, hypothesis.failure_class
            ),
        ),
        "parser_guardrail_probe" => (
            "wf-parser-guardrail-learning".to_string(),
            format!(
                "Escalate parser guardrail checks for target_surface={} using ranked experiment guidance.",
                selected.target_surface
            ),
        ),
        _ => (
            "wf-fallback-surface-learning".to_string(),
            format!(
                "Enable bounded fallback review for target_surface={} after ranked confidence {:.2}.",
                selected.target_surface, selected.confidence
            ),
        ),
    };

    DownstreamWorkflowDecision {
        workflow_id,
        decision_id: format!(
            "decision:{}:{}",
            prioritization.run_id, selected.candidate_id
        ),
        selected_candidate_id: selected.candidate_id.clone(),
        selected_strategy: selected.strategy.clone(),
        decision_class: "cross_workflow_learning_update".to_string(),
        expected_behavior_change,
    }
}

fn normalize_rel_path(path: &Path) -> Result<String, CrossWorkflowError> {
    let rendered = path.display().to_string();
    if rendered.starts_with('/') || rendered.contains('\\') || rendered.contains(':') {
        return Err(CrossWorkflowError::Invalid(format!(
            "artifact path '{}' must be a safe relative path",
            rendered
        )));
    }
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn build_cross_workflow_artifact_is_deterministic() {
        let hypothesis = fixture_hypothesis();
        let policy = fixture_policy();
        let prioritization = fixture_prioritization();
        let left = build_cross_workflow_artifact(
            &hypothesis,
            Path::new("runs/run-1/godel/godel_hypothesis.v1.json"),
            &policy,
            Path::new("runs/run-1/godel/godel_policy.v1.json"),
            &prioritization,
            Path::new("runs/run-1/godel/godel_experiment_priority.v1.json"),
        )
        .expect("left");
        let right = build_cross_workflow_artifact(
            &hypothesis,
            Path::new("runs/run-1/godel/godel_hypothesis.v1.json"),
            &policy,
            Path::new("runs/run-1/godel/godel_policy.v1.json"),
            &prioritization,
            Path::new("runs/run-1/godel/godel_experiment_priority.v1.json"),
        )
        .expect("right");
        assert_eq!(left, right);
        assert_eq!(
            left.downstream_decision.workflow_id,
            "wf-aee-retry-budget-adaptation"
        );
        assert_eq!(
            left.downstream_decision.selected_candidate_id,
            "exp:retry-budget"
        );
    }
}
