use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::hypothesis::PersistedHypothesisArtifact;

pub const POLICY_ARTIFACT_VERSION: &str = "godel_policy.v1";
pub const POLICY_COMPARISON_ARTIFACT_VERSION: &str = "godel_policy_comparison.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyState {
    pub retry_budget: u32,
    pub experiment_budget: u32,
    pub target_surface: String,
    pub policy_mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedPolicyArtifact {
    pub artifact_version: String,
    pub policy_id: String,
    pub run_id: String,
    pub workflow_id: String,
    pub hypothesis_id: String,
    pub hypothesis_artifact_path: String,
    pub source_signal: String,
    pub selection_reason: String,
    pub before_policy: PolicyState,
    pub after_policy: PolicyState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedPolicyComparisonArtifact {
    pub artifact_version: String,
    pub comparison_id: String,
    pub run_id: String,
    pub workflow_id: String,
    pub policy_id: String,
    pub hypothesis_id: String,
    pub changed_fields: Vec<String>,
    pub deterministic_mapping: String,
    pub before_policy: PolicyState,
    pub after_policy: PolicyState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyArtifactError {
    Invalid(String),
    Io(String),
    Serialize(String),
}

impl std::fmt::Display for PolicyArtifactError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(msg) => write!(f, "GODEL_POLICY_INVALID: {msg}"),
            Self::Io(msg) => write!(f, "GODEL_POLICY_IO: {msg}"),
            Self::Serialize(msg) => write!(f, "GODEL_POLICY_SERIALIZE: {msg}"),
        }
    }
}

impl std::error::Error for PolicyArtifactError {}

pub fn build_policy_artifacts(
    hypothesis: &PersistedHypothesisArtifact,
    hypothesis_artifact_path: &Path,
) -> Result<(PersistedPolicyArtifact, PersistedPolicyComparisonArtifact), PolicyArtifactError> {
    if hypothesis.run_id.trim().is_empty()
        || hypothesis.workflow_id.trim().is_empty()
        || hypothesis.hypothesis_id.trim().is_empty()
    {
        return Err(PolicyArtifactError::Invalid(
            "run_id, workflow_id, and hypothesis_id must be non-empty".to_string(),
        ));
    }

    let before_policy = baseline_policy_for_failure(&hypothesis.failure_class);
    let after_policy = adapted_policy_from_hypothesis(hypothesis, &before_policy);

    let mut changed_fields = Vec::new();
    if before_policy.retry_budget != after_policy.retry_budget {
        changed_fields.push("retry_budget".to_string());
    }
    if before_policy.experiment_budget != after_policy.experiment_budget {
        changed_fields.push("experiment_budget".to_string());
    }
    if before_policy.policy_mode != after_policy.policy_mode {
        changed_fields.push("policy_mode".to_string());
    }
    if before_policy.target_surface != after_policy.target_surface {
        changed_fields.push("target_surface".to_string());
    }
    changed_fields.sort();

    let hypothesis_artifact_path = normalize_rel_path(hypothesis_artifact_path)?;
    let policy_id = format!("policy:{}:{}", hypothesis.run_id, hypothesis.failure_class);
    let source_signal = format!(
        "hypothesis:{}:{}",
        hypothesis.failure_class, hypothesis.artifact_version
    );
    let selection_reason = format!(
        "Deterministic policy update derived from hypothesis_id={} and failure_class={}.",
        hypothesis.hypothesis_id, hypothesis.failure_class
    );

    let policy = PersistedPolicyArtifact {
        artifact_version: POLICY_ARTIFACT_VERSION.to_string(),
        policy_id: policy_id.clone(),
        run_id: hypothesis.run_id.clone(),
        workflow_id: hypothesis.workflow_id.clone(),
        hypothesis_id: hypothesis.hypothesis_id.clone(),
        hypothesis_artifact_path,
        source_signal,
        selection_reason,
        before_policy: before_policy.clone(),
        after_policy: after_policy.clone(),
    };
    let comparison = PersistedPolicyComparisonArtifact {
        artifact_version: POLICY_COMPARISON_ARTIFACT_VERSION.to_string(),
        comparison_id: format!("cmp:{}:{}", hypothesis.run_id, hypothesis.failure_class),
        run_id: hypothesis.run_id.clone(),
        workflow_id: hypothesis.workflow_id.clone(),
        policy_id,
        hypothesis_id: hypothesis.hypothesis_id.clone(),
        changed_fields,
        deterministic_mapping:
            "stable failure_class -> baseline policy -> bounded policy adjustment".to_string(),
        before_policy,
        after_policy,
    };

    Ok((policy, comparison))
}

pub fn persist_policy_artifact(
    runs_root: &Path,
    run_id: &str,
    artifact: &PersistedPolicyArtifact,
) -> Result<PathBuf, PolicyArtifactError> {
    persist_json(
        runs_root,
        run_id,
        "godel_policy.v1.json",
        artifact,
        "policy artifact",
    )
}

pub fn persist_policy_comparison_artifact(
    runs_root: &Path,
    run_id: &str,
    artifact: &PersistedPolicyComparisonArtifact,
) -> Result<PathBuf, PolicyArtifactError> {
    persist_json(
        runs_root,
        run_id,
        "godel_policy_comparison.v1.json",
        artifact,
        "policy comparison artifact",
    )
}

fn baseline_policy_for_failure(failure_class: &str) -> PolicyState {
    match failure_class {
        "tool_failure" => PolicyState {
            retry_budget: 1,
            experiment_budget: 1,
            target_surface: "tool-invocation-config".to_string(),
            policy_mode: "baseline".to_string(),
        },
        "policy_denied" => PolicyState {
            retry_budget: 0,
            experiment_budget: 1,
            target_surface: "delegation-policy-input".to_string(),
            policy_mode: "baseline".to_string(),
        },
        _ => PolicyState {
            retry_budget: 0,
            experiment_budget: 1,
            target_surface: "workflow-step-config".to_string(),
            policy_mode: "baseline".to_string(),
        },
    }
}

fn adapted_policy_from_hypothesis(
    hypothesis: &PersistedHypothesisArtifact,
    baseline: &PolicyState,
) -> PolicyState {
    let mut next = baseline.clone();
    next.policy_mode = "adaptive_reviewed".to_string();
    if hypothesis.failure_class == "tool_failure" {
        next.retry_budget = baseline.retry_budget + 1;
        next.experiment_budget = baseline.experiment_budget + 1;
    } else if hypothesis.failure_class == "policy_denied" {
        next.experiment_budget = baseline.experiment_budget + 1;
    }
    next
}

fn normalize_rel_path(path: &Path) -> Result<String, PolicyArtifactError> {
    let rendered = path.display().to_string();
    if rendered.starts_with('/') || rendered.contains('\\') || rendered.contains(':') {
        return Err(PolicyArtifactError::Invalid(format!(
            "artifact path '{}' must be a safe relative path",
            rendered
        )));
    }
    Ok(rendered)
}

fn persist_json<T: Serialize>(
    runs_root: &Path,
    run_id: &str,
    file_name: &str,
    artifact: &T,
    label: &str,
) -> Result<PathBuf, PolicyArtifactError> {
    let dir = runs_root.join(run_id).join("godel");
    fs::create_dir_all(&dir)
        .map_err(|err| PolicyArtifactError::Io(format!("create godel dir failed: {err}")))?;
    let path = dir.join(file_name);
    let bytes = serde_json::to_vec_pretty(artifact).map_err(|err| {
        PolicyArtifactError::Serialize(format!("{label} serialize failed: {err}"))
    })?;
    fs::write(&path, bytes)
        .map_err(|err| PolicyArtifactError::Io(format!("{label} write failed: {err}")))?;
    Ok(PathBuf::from("runs")
        .join(run_id)
        .join("godel")
        .join(file_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::godel::hypothesis::PersistedHypothesisArtifact;

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

    #[test]
    fn build_policy_artifacts_is_deterministic() {
        let hypothesis = fixture_hypothesis();
        let left = build_policy_artifacts(
            &hypothesis,
            Path::new("runs/run-1/godel/godel_hypothesis.v1.json"),
        )
        .expect("left");
        let right = build_policy_artifacts(
            &hypothesis,
            Path::new("runs/run-1/godel/godel_hypothesis.v1.json"),
        )
        .expect("right");
        assert_eq!(left, right);
        assert_eq!(left.0.after_policy.retry_budget, 2);
        assert_eq!(
            left.1.changed_fields,
            vec!["experiment_budget", "policy_mode", "retry_budget"]
        );
    }

    #[test]
    fn persist_policy_artifacts_write_expected_paths() {
        let base = std::env::temp_dir().join(format!("adl-godel-policy-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).expect("base");
        let hypothesis = fixture_hypothesis();
        let (policy, comparison) = build_policy_artifacts(
            &hypothesis,
            Path::new("runs/run-1/godel/godel_hypothesis.v1.json"),
        )
        .expect("artifacts");
        let policy_rel = persist_policy_artifact(&base, "run-1", &policy).expect("policy");
        let comparison_rel =
            persist_policy_comparison_artifact(&base, "run-1", &comparison).expect("comparison");
        assert_eq!(
            policy_rel,
            PathBuf::from("runs/run-1/godel/godel_policy.v1.json")
        );
        assert_eq!(
            comparison_rel,
            PathBuf::from("runs/run-1/godel/godel_policy_comparison.v1.json")
        );
        assert!(base.join("run-1/godel/godel_policy.v1.json").is_file());
        assert!(base
            .join("run-1/godel/godel_policy_comparison.v1.json")
            .is_file());
        let _ = fs::remove_dir_all(&base);
    }
}
