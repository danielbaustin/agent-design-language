//! Experiment prioritization inputs and persisted ranking records.
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::hypothesis::PersistedHypothesisArtifact;
use super::policy::PersistedPolicyArtifact;

pub const PRIORITIZATION_ARTIFACT_VERSION: &str = "godel_experiment_priority.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrioritizationInputCandidate {
    pub candidate_id: String,
    pub strategy: String,
    pub target_surface: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankedExperimentCandidate {
    pub candidate_id: String,
    pub strategy: String,
    pub target_surface: String,
    pub priority_score: u32,
    pub confidence: f64,
    pub ranking_reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersistedPrioritizationArtifact {
    pub artifact_version: String,
    pub prioritization_id: String,
    pub run_id: String,
    pub workflow_id: String,
    pub hypothesis_id: String,
    pub policy_id: String,
    pub hypothesis_artifact_path: String,
    pub policy_artifact_path: String,
    pub tie_break_rule: String,
    pub input_candidates: Vec<PrioritizationInputCandidate>,
    pub ranked_candidates: Vec<RankedExperimentCandidate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrioritizationError {
    Invalid(String),
    Io(String),
    Serialize(String),
}

impl std::fmt::Display for PrioritizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(msg) => write!(f, "GODEL_PRIORITIZATION_INVALID: {msg}"),
            Self::Io(msg) => write!(f, "GODEL_PRIORITIZATION_IO: {msg}"),
            Self::Serialize(msg) => write!(f, "GODEL_PRIORITIZATION_SERIALIZE: {msg}"),
        }
    }
}

impl std::error::Error for PrioritizationError {}

pub fn build_prioritization_artifact(
    hypothesis: &PersistedHypothesisArtifact,
    hypothesis_artifact_path: &Path,
    policy: &PersistedPolicyArtifact,
    policy_artifact_path: &Path,
) -> Result<PersistedPrioritizationArtifact, PrioritizationError> {
    if hypothesis.run_id != policy.run_id || hypothesis.workflow_id != policy.workflow_id {
        return Err(PrioritizationError::Invalid(
            "hypothesis and policy artifacts must share run_id and workflow_id".to_string(),
        ));
    }

    let input_candidates = build_input_candidates(policy);
    let mut ranked_candidates = input_candidates
        .iter()
        .enumerate()
        .map(|(idx, candidate)| rank_candidate(idx, candidate, hypothesis, policy))
        .collect::<Vec<_>>();

    ranked_candidates.sort_by(|a, b| {
        b.priority_score
            .cmp(&a.priority_score)
            .then_with(|| b.confidence.total_cmp(&a.confidence))
            .then_with(|| a.candidate_id.cmp(&b.candidate_id))
    });

    Ok(PersistedPrioritizationArtifact {
        artifact_version: PRIORITIZATION_ARTIFACT_VERSION.to_string(),
        prioritization_id: format!(
            "prioritize:{}:{}",
            hypothesis.run_id, hypothesis.failure_class
        ),
        run_id: hypothesis.run_id.clone(),
        workflow_id: hypothesis.workflow_id.clone(),
        hypothesis_id: hypothesis.hypothesis_id.clone(),
        policy_id: policy.policy_id.clone(),
        hypothesis_artifact_path: normalize_rel_path(hypothesis_artifact_path)?,
        policy_artifact_path: normalize_rel_path(policy_artifact_path)?,
        tie_break_rule: "sort by priority_score desc, then confidence desc, then candidate_id asc"
            .to_string(),
        input_candidates,
        ranked_candidates,
    })
}

pub fn persist_prioritization_artifact(
    runs_root: &Path,
    run_id: &str,
    artifact: &PersistedPrioritizationArtifact,
) -> Result<PathBuf, PrioritizationError> {
    let dir = runs_root.join(run_id).join("godel");
    fs::create_dir_all(&dir)
        .map_err(|err| PrioritizationError::Io(format!("create godel dir failed: {err}")))?;
    let path = dir.join("godel_experiment_priority.v1.json");
    let bytes = serde_json::to_vec_pretty(artifact).map_err(|err| {
        PrioritizationError::Serialize(format!("prioritization artifact serialize failed: {err}"))
    })?;
    fs::write(&path, bytes).map_err(|err| {
        PrioritizationError::Io(format!("prioritization artifact write failed: {err}"))
    })?;
    Ok(PathBuf::from("runs")
        .join(run_id)
        .join("godel")
        .join("godel_experiment_priority.v1.json"))
}

fn build_input_candidates(policy: &PersistedPolicyArtifact) -> Vec<PrioritizationInputCandidate> {
    let target = policy.after_policy.target_surface.clone();
    let mut candidates = vec![
        PrioritizationInputCandidate {
            candidate_id: "exp:retry-budget".to_string(),
            strategy: "retry_budget_probe".to_string(),
            target_surface: target.clone(),
        },
        PrioritizationInputCandidate {
            candidate_id: "exp:parser-guardrail".to_string(),
            strategy: "parser_guardrail_probe".to_string(),
            target_surface: target.clone(),
        },
        PrioritizationInputCandidate {
            candidate_id: "exp:fallback-check".to_string(),
            strategy: "fallback_surface_probe".to_string(),
            target_surface: target,
        },
    ];
    candidates.sort_by(|a, b| a.candidate_id.cmp(&b.candidate_id));
    candidates
}

fn rank_candidate(
    idx: usize,
    candidate: &PrioritizationInputCandidate,
    hypothesis: &PersistedHypothesisArtifact,
    policy: &PersistedPolicyArtifact,
) -> RankedExperimentCandidate {
    let base_priority = match candidate.strategy.as_str() {
        "retry_budget_probe" => {
            if hypothesis.failure_class == "tool_failure" {
                90
            } else {
                60
            }
        }
        "parser_guardrail_probe" => 75,
        _ => 65,
    };
    let policy_bonus = policy
        .after_policy
        .retry_budget
        .saturating_sub(policy.before_policy.retry_budget)
        * 5;
    let priority_score = base_priority + policy_bonus.saturating_sub(idx as u32);
    let confidence = match candidate.strategy.as_str() {
        "retry_budget_probe" => 0.86,
        "parser_guardrail_probe" => 0.74,
        _ => 0.68,
    };

    RankedExperimentCandidate {
        candidate_id: candidate.candidate_id.clone(),
        strategy: candidate.strategy.clone(),
        target_surface: candidate.target_surface.clone(),
        priority_score,
        confidence,
        ranking_reason: format!(
            "Ranked from failure_class={} and policy_mode={} with bounded deterministic scoring.",
            hypothesis.failure_class, policy.after_policy.policy_mode
        ),
    }
}

fn normalize_rel_path(path: &Path) -> Result<String, PrioritizationError> {
    let rendered = path.display().to_string();
    if rendered.starts_with('/') || rendered.contains('\\') || rendered.contains(':') {
        return Err(PrioritizationError::Invalid(format!(
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

    #[test]
    fn build_prioritization_artifact_is_deterministic() {
        let hypothesis = fixture_hypothesis();
        let policy = fixture_policy();
        let left = build_prioritization_artifact(
            &hypothesis,
            Path::new("runs/run-1/godel/godel_hypothesis.v1.json"),
            &policy,
            Path::new("runs/run-1/godel/godel_policy.v1.json"),
        )
        .expect("left");
        let right = build_prioritization_artifact(
            &hypothesis,
            Path::new("runs/run-1/godel/godel_hypothesis.v1.json"),
            &policy,
            Path::new("runs/run-1/godel/godel_policy.v1.json"),
        )
        .expect("right");
        assert_eq!(left, right);
        assert_eq!(left.input_candidates.len(), 3);
        assert_eq!(left.ranked_candidates[0].candidate_id, "exp:retry-budget");
        assert_eq!(
            left.tie_break_rule,
            "sort by priority_score desc, then confidence desc, then candidate_id asc"
        );
    }

    #[test]
    fn persist_prioritization_artifact_writes_expected_path() {
        let base = std::env::temp_dir().join(format!("adl-godel-priority-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).expect("base");
        let hypothesis = fixture_hypothesis();
        let policy = fixture_policy();
        let artifact = build_prioritization_artifact(
            &hypothesis,
            Path::new("runs/run-1/godel/godel_hypothesis.v1.json"),
            &policy,
            Path::new("runs/run-1/godel/godel_policy.v1.json"),
        )
        .expect("artifact");
        let rel = persist_prioritization_artifact(&base, "run-1", &artifact).expect("persist");
        assert_eq!(
            rel,
            PathBuf::from("runs/run-1/godel/godel_experiment_priority.v1.json")
        );
        assert!(base
            .join("run-1/godel/godel_experiment_priority.v1.json")
            .is_file());
        let _ = fs::remove_dir_all(&base);
    }
}
