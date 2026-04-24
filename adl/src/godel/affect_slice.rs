//! Gödel affect-slice experiments and persisted candidate selection contracts.
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use super::hypothesis::PersistedHypothesisArtifact;
use super::policy::PersistedPolicyArtifact;
use super::prioritization::PersistedPrioritizationArtifact;

pub const AFFECT_GODEL_VERTICAL_SLICE_ARTIFACT_VERSION: &str = "godel_affect_vertical_slice.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedAffectGodelVerticalSliceArtifact {
    pub artifact_version: String,
    pub slice_id: String,
    pub godel_run_id: String,
    pub workflow_id: String,
    pub hypothesis_id: String,
    pub policy_id: String,
    pub prioritization_id: String,
    pub baseline_candidate_id: String,
    pub input_condition: VerticalSliceInputCondition,
    pub affect_transition: VerticalSliceAffectTransition,
    pub downstream_change: VerticalSliceDownstreamChange,
    pub proof_paths: VerticalSliceProofPaths,
    pub deterministic_linkage_rule: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerticalSliceInputCondition {
    pub initial_run_id: String,
    pub adapted_run_id: String,
    pub trigger: String,
    pub evidence_summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerticalSliceAffectTransition {
    pub initial_affect_state_id: String,
    pub initial_affect_mode: String,
    pub initial_recovery_bias: u32,
    pub adapted_affect_state_id: String,
    pub adapted_affect_mode: String,
    pub adapted_recovery_bias: u32,
    pub changed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerticalSliceDownstreamChange {
    pub output_surface: String,
    pub initial_selected_candidate_id: String,
    pub initial_selected_strategy: String,
    pub adapted_selected_candidate_id: String,
    pub adapted_selected_strategy: String,
    pub changed: bool,
    pub baseline_candidate_id: String,
    pub initial_ranking: Vec<VerticalSliceRankedCandidate>,
    pub adapted_ranking: Vec<VerticalSliceRankedCandidate>,
    pub expected_behavior_change: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerticalSliceRankedCandidate {
    pub candidate_id: String,
    pub strategy: String,
    pub source_node_id: String,
    pub priority_score: u32,
    pub rank: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerticalSliceProofPaths {
    pub initial_affect_path: String,
    pub initial_reasoning_graph_path: String,
    pub adapted_affect_path: String,
    pub adapted_reasoning_graph_path: String,
    pub hypothesis_artifact_path: String,
    pub policy_artifact_path: String,
    pub prioritization_artifact_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AffectSliceError {
    Invalid(String),
    Io(String),
    Serialize(String),
}

impl std::fmt::Display for AffectSliceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(msg) => write!(f, "GODEL_AFFECT_SLICE_INVALID: {msg}"),
            Self::Io(msg) => write!(f, "GODEL_AFFECT_SLICE_IO: {msg}"),
            Self::Serialize(msg) => write!(f, "GODEL_AFFECT_SLICE_SERIALIZE: {msg}"),
        }
    }
}

impl std::error::Error for AffectSliceError {}

#[derive(Debug, Clone)]
pub struct AffectSliceInputs<'a> {
    pub initial_affect: &'a PersistedAffectStateArtifact,
    pub initial_graph: &'a PersistedReasoningGraphArtifact,
    pub adapted_affect: &'a PersistedAffectStateArtifact,
    pub adapted_graph: &'a PersistedReasoningGraphArtifact,
    pub hypothesis: &'a PersistedHypothesisArtifact,
    pub hypothesis_artifact_path: &'a Path,
    pub policy: &'a PersistedPolicyArtifact,
    pub policy_artifact_path: &'a Path,
    pub prioritization: &'a PersistedPrioritizationArtifact,
    pub prioritization_artifact_path: &'a Path,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedAffectStateArtifact {
    pub run_id: String,
    pub affect: PersistedAffectStateRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedAffectStateRecord {
    pub affect_state_id: String,
    pub affect_mode: String,
    pub recovery_bias: u32,
    pub downstream_priority: String,
    pub update_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedReasoningGraphArtifact {
    pub run_id: String,
    pub graph: PersistedReasoningGraphRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedReasoningGraphRecord {
    pub dominant_affect_mode: String,
    pub ranking_rule: String,
    pub selected_path: PersistedReasoningGraphSelection,
    pub nodes: Vec<PersistedReasoningGraphNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedReasoningGraphSelection {
    pub selected_node_id: String,
    pub selected_intent: String,
    pub selected_target: String,
    pub graph_derived_output: String,
    pub affect_changed_ranking: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedReasoningGraphNode {
    pub node_id: String,
    pub node_kind: String,
    pub rank: u32,
    pub priority_score: u32,
}

pub fn build_affect_godel_vertical_slice_artifact(
    inputs: AffectSliceInputs<'_>,
) -> Result<PersistedAffectGodelVerticalSliceArtifact, AffectSliceError> {
    if inputs.hypothesis.run_id != inputs.policy.run_id
        || inputs.hypothesis.run_id != inputs.prioritization.run_id
        || inputs.hypothesis.workflow_id != inputs.policy.workflow_id
        || inputs.hypothesis.workflow_id != inputs.prioritization.workflow_id
    {
        return Err(AffectSliceError::Invalid(
            "hypothesis, policy, and prioritization artifacts must share run_id and workflow_id"
                .to_string(),
        ));
    }

    let baseline_candidate_id = inputs
        .prioritization
        .ranked_candidates
        .first()
        .ok_or_else(|| {
            AffectSliceError::Invalid(
                "prioritization artifact must contain at least one ranked candidate".to_string(),
            )
        })?
        .candidate_id
        .clone();

    let initial_ranking = ranking_from_graph(&inputs.initial_graph.graph)?;
    let adapted_ranking = ranking_from_graph(&inputs.adapted_graph.graph)?;
    let initial_selected = initial_ranking.first().cloned().ok_or_else(|| {
        AffectSliceError::Invalid("initial reasoning graph ranking must not be empty".to_string())
    })?;
    let adapted_selected = adapted_ranking.first().cloned().ok_or_else(|| {
        AffectSliceError::Invalid("adapted reasoning graph ranking must not be empty".to_string())
    })?;

    Ok(PersistedAffectGodelVerticalSliceArtifact {
        artifact_version: AFFECT_GODEL_VERTICAL_SLICE_ARTIFACT_VERSION.to_string(),
        slice_id: format!(
            "affect-slice:{}:{}:{}",
            inputs.hypothesis.run_id, inputs.initial_affect.run_id, inputs.adapted_affect.run_id
        ),
        godel_run_id: inputs.hypothesis.run_id.clone(),
        workflow_id: inputs.hypothesis.workflow_id.clone(),
        hypothesis_id: inputs.hypothesis.hypothesis_id.clone(),
        policy_id: inputs.policy.policy_id.clone(),
        prioritization_id: inputs.prioritization.prioritization_id.clone(),
        baseline_candidate_id: baseline_candidate_id.clone(),
        input_condition: VerticalSliceInputCondition {
            initial_run_id: inputs.initial_affect.run_id.clone(),
            adapted_run_id: inputs.adapted_affect.run_id.clone(),
            trigger: "bounded recovery rerun after failure evidence".to_string(),
            evidence_summary: format!(
                "initial={} bias={} -> adapted={} bias={}",
                inputs.initial_affect.affect.affect_mode,
                inputs.initial_affect.affect.recovery_bias,
                inputs.adapted_affect.affect.affect_mode,
                inputs.adapted_affect.affect.recovery_bias
            ),
        },
        affect_transition: VerticalSliceAffectTransition {
            initial_affect_state_id: inputs.initial_affect.affect.affect_state_id.clone(),
            initial_affect_mode: inputs.initial_affect.affect.affect_mode.clone(),
            initial_recovery_bias: inputs.initial_affect.affect.recovery_bias,
            adapted_affect_state_id: inputs.adapted_affect.affect.affect_state_id.clone(),
            adapted_affect_mode: inputs.adapted_affect.affect.affect_mode.clone(),
            adapted_recovery_bias: inputs.adapted_affect.affect.recovery_bias,
            changed: inputs.initial_affect.affect.affect_mode
                != inputs.adapted_affect.affect.affect_mode
                || inputs.initial_affect.affect.recovery_bias
                    != inputs.adapted_affect.affect.recovery_bias,
        },
        downstream_change: VerticalSliceDownstreamChange {
            output_surface: "godel_strategy_ranking".to_string(),
            initial_selected_candidate_id: initial_selected.candidate_id.clone(),
            initial_selected_strategy: initial_selected.strategy.clone(),
            adapted_selected_candidate_id: adapted_selected.candidate_id.clone(),
            adapted_selected_strategy: adapted_selected.strategy.clone(),
            changed: initial_selected.candidate_id != adapted_selected.candidate_id,
            baseline_candidate_id,
            initial_ranking,
            adapted_ranking,
            expected_behavior_change: format!(
                "Affect change flips the top Godel-adjacent strategy from {} to {} while preserving hypothesis {} and policy {}.",
                initial_selected.strategy,
                adapted_selected.strategy,
                inputs.hypothesis.hypothesis_id,
                inputs.policy.policy_id
            ),
        },
        proof_paths: VerticalSliceProofPaths {
            initial_affect_path: format!(
                "runs/{}/learning/affect_state.v1.json",
                inputs.initial_affect.run_id
            ),
            initial_reasoning_graph_path: format!(
                "runs/{}/learning/reasoning_graph.v1.json",
                inputs.initial_affect.run_id
            ),
            adapted_affect_path: format!(
                "runs/{}/learning/affect_state.v1.json",
                inputs.adapted_affect.run_id
            ),
            adapted_reasoning_graph_path: format!(
                "runs/{}/learning/reasoning_graph.v1.json",
                inputs.adapted_affect.run_id
            ),
            hypothesis_artifact_path: normalize_rel_path(inputs.hypothesis_artifact_path)?,
            policy_artifact_path: normalize_rel_path(inputs.policy_artifact_path)?,
            prioritization_artifact_path: normalize_rel_path(inputs.prioritization_artifact_path)?,
        },
        deterministic_linkage_rule:
            "map affect-linked reasoning graph action ranks to bounded Godel strategy candidates by node rank asc, preserving stable node_id tie-breaks and fixed candidate mapping".to_string(),
    })
}

pub fn persist_affect_godel_vertical_slice_artifact(
    runs_root: &Path,
    run_id: &str,
    artifact: &PersistedAffectGodelVerticalSliceArtifact,
) -> Result<PathBuf, AffectSliceError> {
    let dir = runs_root.join(run_id).join("godel");
    fs::create_dir_all(&dir)
        .map_err(|err| AffectSliceError::Io(format!("create godel dir failed: {err}")))?;
    let path = dir.join("godel_affect_vertical_slice.v1.json");
    let bytes = serde_json::to_vec_pretty(artifact).map_err(|err| {
        AffectSliceError::Serialize(format!(
            "affect-plus-godel vertical slice artifact serialize failed: {err}"
        ))
    })?;
    fs::write(&path, bytes).map_err(|err| {
        AffectSliceError::Io(format!(
            "affect-plus-godel vertical slice artifact write failed: {err}"
        ))
    })?;
    Ok(PathBuf::from("runs")
        .join(run_id)
        .join("godel")
        .join("godel_affect_vertical_slice.v1.json"))
}

fn ranking_from_graph(
    graph: &PersistedReasoningGraphRecord,
) -> Result<Vec<VerticalSliceRankedCandidate>, AffectSliceError> {
    let mut ranked: Vec<VerticalSliceRankedCandidate> = graph
        .nodes
        .iter()
        .filter(|node| node.node_kind == "action")
        .map(|node| {
            let (candidate_id, strategy) = map_node_to_candidate(&node.node_id);
            VerticalSliceRankedCandidate {
                candidate_id: candidate_id.to_string(),
                strategy: strategy.to_string(),
                source_node_id: node.node_id.clone(),
                priority_score: node.priority_score,
                rank: node.rank,
            }
        })
        .collect();
    ranked.sort_by(|left, right| {
        left.rank
            .cmp(&right.rank)
            .then_with(|| left.candidate_id.cmp(&right.candidate_id))
    });
    if ranked.is_empty() {
        return Err(AffectSliceError::Invalid(
            "reasoning graph must contain at least one action node".to_string(),
        ));
    }
    Ok(ranked)
}

fn map_node_to_candidate(node_id: &str) -> (&'static str, &'static str) {
    match node_id {
        "action.retry_budget" => ("exp:retry-budget", "retry_budget_probe"),
        "action.maintain_policy" => ("exp:maintain-policy", "maintain_policy_review"),
        _ => ("exp:fallback-check", "fallback_surface_probe"),
    }
}

fn normalize_rel_path(path: &Path) -> Result<String, AffectSliceError> {
    let rendered = path.display().to_string();
    if rendered.starts_with('/') || rendered.contains('\\') || rendered.contains(':') {
        return Err(AffectSliceError::Invalid(format!(
            "artifact path '{}' must be a safe relative path",
            rendered
        )));
    }
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::godel::policy::PolicyState;
    use crate::godel::prioritization::{
        PersistedPrioritizationArtifact, PrioritizationInputCandidate, RankedExperimentCandidate,
    };

    fn fixture_affect(run_id: &str, affect_mode: &str, bias: u32) -> PersistedAffectStateArtifact {
        PersistedAffectStateArtifact {
            run_id: run_id.to_string(),
            affect: PersistedAffectStateRecord {
                affect_state_id: format!("affect:{run_id}"),
                affect_mode: affect_mode.to_string(),
                recovery_bias: bias,
                downstream_priority: if bias > 0 {
                    "retry_recovery".to_string()
                } else {
                    "maintain_current_policy".to_string()
                },
                update_reason: "deterministic fixture".to_string(),
            },
        }
    }

    fn fixture_graph(
        run_id: &str,
        dominant_affect_mode: &str,
        first_node: (&str, u32, u32),
        second_node: (&str, u32, u32),
    ) -> PersistedReasoningGraphArtifact {
        PersistedReasoningGraphArtifact {
            run_id: run_id.to_string(),
            graph: PersistedReasoningGraphRecord {
                dominant_affect_mode: dominant_affect_mode.to_string(),
                ranking_rule: "sort by priority_score desc, then node_id asc".to_string(),
                selected_path: PersistedReasoningGraphSelection {
                    selected_node_id: first_node.0.to_string(),
                    selected_intent: "deterministic".to_string(),
                    selected_target: "workflow-runtime".to_string(),
                    graph_derived_output: "deterministic".to_string(),
                    affect_changed_ranking: true,
                },
                nodes: vec![
                    PersistedReasoningGraphNode {
                        node_id: first_node.0.to_string(),
                        node_kind: "action".to_string(),
                        rank: first_node.1,
                        priority_score: first_node.2,
                    },
                    PersistedReasoningGraphNode {
                        node_id: second_node.0.to_string(),
                        node_kind: "action".to_string(),
                        rank: second_node.1,
                        priority_score: second_node.2,
                    },
                ],
            },
        }
    }

    fn fixture_hypothesis() -> PersistedHypothesisArtifact {
        PersistedHypothesisArtifact {
            artifact_version: "godel_hypothesis.v1".to_string(),
            hypothesis_id: "hyp:run-1:tool_failure:00".to_string(),
            run_id: "run-1".to_string(),
            workflow_id: "wf-godel-loop".to_string(),
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
            workflow_id: "wf-godel-loop".to_string(),
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
            workflow_id: "wf-godel-loop".to_string(),
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
    fn build_affect_godel_vertical_slice_is_deterministic_and_changes_candidate() {
        let initial_affect = fixture_affect("v0-3-aee-recovery-initial", "recovery_focus", 2);
        let adapted_affect = fixture_affect("v0-3-aee-recovery-adapted", "steady_state", 0);
        let initial_graph = fixture_graph(
            "v0-3-aee-recovery-initial",
            "recovery_focus",
            ("action.retry_budget", 1, 92),
            ("action.maintain_policy", 2, 36),
        );
        let adapted_graph = fixture_graph(
            "v0-3-aee-recovery-adapted",
            "steady_state",
            ("action.maintain_policy", 1, 88),
            ("action.retry_budget", 2, 22),
        );

        let left = build_affect_godel_vertical_slice_artifact(AffectSliceInputs {
            initial_affect: &initial_affect,
            initial_graph: &initial_graph,
            adapted_affect: &adapted_affect,
            adapted_graph: &adapted_graph,
            hypothesis: &fixture_hypothesis(),
            hypothesis_artifact_path: Path::new("runs/run-1/godel/godel_hypothesis.v1.json"),
            policy: &fixture_policy(),
            policy_artifact_path: Path::new("runs/run-1/godel/godel_policy.v1.json"),
            prioritization: &fixture_prioritization(),
            prioritization_artifact_path: Path::new(
                "runs/run-1/godel/godel_experiment_priority.v1.json",
            ),
        })
        .expect("build left");

        let right = build_affect_godel_vertical_slice_artifact(AffectSliceInputs {
            initial_affect: &initial_affect,
            initial_graph: &initial_graph,
            adapted_affect: &adapted_affect,
            adapted_graph: &adapted_graph,
            hypothesis: &fixture_hypothesis(),
            hypothesis_artifact_path: Path::new("runs/run-1/godel/godel_hypothesis.v1.json"),
            policy: &fixture_policy(),
            policy_artifact_path: Path::new("runs/run-1/godel/godel_policy.v1.json"),
            prioritization: &fixture_prioritization(),
            prioritization_artifact_path: Path::new(
                "runs/run-1/godel/godel_experiment_priority.v1.json",
            ),
        })
        .expect("build right");

        assert_eq!(left, right);
        assert_eq!(
            left.downstream_change.initial_selected_candidate_id,
            "exp:retry-budget"
        );
        assert_eq!(
            left.downstream_change.adapted_selected_candidate_id,
            "exp:maintain-policy"
        );
        assert!(left.downstream_change.changed);
    }

    #[test]
    fn persist_affect_godel_vertical_slice_writes_expected_path() {
        let base = std::env::temp_dir().join(format!("adl-affect-slice-{}", std::process::id()));
        let artifact = PersistedAffectGodelVerticalSliceArtifact {
            artifact_version: AFFECT_GODEL_VERTICAL_SLICE_ARTIFACT_VERSION.to_string(),
            slice_id: "affect-slice:run-1".to_string(),
            godel_run_id: "run-1".to_string(),
            workflow_id: "wf-godel-loop".to_string(),
            hypothesis_id: "hyp:run-1".to_string(),
            policy_id: "policy:run-1".to_string(),
            prioritization_id: "prioritize:run-1".to_string(),
            baseline_candidate_id: "exp:retry-budget".to_string(),
            input_condition: VerticalSliceInputCondition {
                initial_run_id: "initial".to_string(),
                adapted_run_id: "adapted".to_string(),
                trigger: "test".to_string(),
                evidence_summary: "test".to_string(),
            },
            affect_transition: VerticalSliceAffectTransition {
                initial_affect_state_id: "a".to_string(),
                initial_affect_mode: "recovery_focus".to_string(),
                initial_recovery_bias: 2,
                adapted_affect_state_id: "b".to_string(),
                adapted_affect_mode: "steady_state".to_string(),
                adapted_recovery_bias: 0,
                changed: true,
            },
            downstream_change: VerticalSliceDownstreamChange {
                output_surface: "godel_strategy_ranking".to_string(),
                initial_selected_candidate_id: "exp:retry-budget".to_string(),
                initial_selected_strategy: "retry_budget_probe".to_string(),
                adapted_selected_candidate_id: "exp:maintain-policy".to_string(),
                adapted_selected_strategy: "maintain_policy_review".to_string(),
                changed: true,
                baseline_candidate_id: "exp:retry-budget".to_string(),
                initial_ranking: vec![],
                adapted_ranking: vec![],
                expected_behavior_change: "test".to_string(),
            },
            proof_paths: VerticalSliceProofPaths {
                initial_affect_path: "runs/initial/learning/affect_state.v1.json".to_string(),
                initial_reasoning_graph_path: "runs/initial/learning/reasoning_graph.v1.json"
                    .to_string(),
                adapted_affect_path: "runs/adapted/learning/affect_state.v1.json".to_string(),
                adapted_reasoning_graph_path: "runs/adapted/learning/reasoning_graph.v1.json"
                    .to_string(),
                hypothesis_artifact_path: "runs/run-1/godel/godel_hypothesis.v1.json".to_string(),
                policy_artifact_path: "runs/run-1/godel/godel_policy.v1.json".to_string(),
                prioritization_artifact_path: "runs/run-1/godel/godel_experiment_priority.v1.json"
                    .to_string(),
            },
            deterministic_linkage_rule: "test".to_string(),
        };

        let rel = persist_affect_godel_vertical_slice_artifact(&base, "run-1", &artifact)
            .expect("persist artifact");
        assert_eq!(
            rel,
            PathBuf::from("runs/run-1/godel/godel_affect_vertical_slice.v1.json")
        );
        assert!(base
            .join("run-1/godel/godel_affect_vertical_slice.v1.json")
            .is_file());
    }
}
