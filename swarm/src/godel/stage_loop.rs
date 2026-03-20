use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::canonical_evidence;
use super::evaluation::{self, EvaluationOutcome};
use super::experiment_record::{self, StageExperimentRecord};
use super::hypothesis::{self, HypothesisCandidate, HypothesisPipelineInput};
use super::mutation::{self, MutationPlan, MutationProposal};
use super::obsmem_index::{self, StageIndexEntry};
use super::policy;
use super::workflow_template::{embedded_v08_workflow_template, GodelWorkflowTemplate};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GodelStage {
    Failure,
    Hypothesis,
    Mutation,
    Experiment,
    Evaluation,
    Record,
    Indexing,
}

impl GodelStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failure => "failure",
            Self::Hypothesis => "hypothesis",
            Self::Mutation => "mutation",
            Self::Experiment => "experiment",
            Self::Evaluation => "evaluation",
            Self::Record => "record",
            Self::Indexing => "indexing",
        }
    }
}

const FALLBACK_RUNTIME_STAGE_ORDER: [GodelStage; 7] = [
    GodelStage::Failure,
    GodelStage::Hypothesis,
    GodelStage::Mutation,
    GodelStage::Experiment,
    GodelStage::Evaluation,
    GodelStage::Record,
    GodelStage::Indexing,
];

fn stage_from_template_id(stage_id: &str) -> Result<GodelStage, StageLoopError> {
    match stage_id {
        "failure" => Ok(GodelStage::Failure),
        "hypothesis" => Ok(GodelStage::Hypothesis),
        "mutation" => Ok(GodelStage::Mutation),
        "experiment" => Ok(GodelStage::Experiment),
        "evaluation" => Ok(GodelStage::Evaluation),
        "record" => Ok(GodelStage::Record),
        other => Err(StageLoopError::DeterminismViolation(format!(
            "workflow template declares unsupported stage '{other}'"
        ))),
    }
}

fn runtime_stage_order_from_template(
    template: &GodelWorkflowTemplate,
) -> Result<Vec<GodelStage>, StageLoopError> {
    let mut runtime_order = Vec::with_capacity(template.stage_order.len() + 1);
    for stage_id in &template.stage_order {
        runtime_order.push(stage_from_template_id(stage_id)?);
    }
    runtime_order.push(GodelStage::Indexing);
    Ok(runtime_order)
}

fn canonical_runtime_stage_order() -> Result<Vec<GodelStage>, StageLoopError> {
    let template = embedded_v08_workflow_template().map_err(|err| {
        StageLoopError::DeterminismViolation(format!(
            "failed to load embedded workflow template: {err}"
        ))
    })?;
    let runtime_order = runtime_stage_order_from_template(&template)?;
    if runtime_order != FALLBACK_RUNTIME_STAGE_ORDER {
        return Err(StageLoopError::DeterminismViolation(format!(
            "embedded workflow template diverged from supported runtime stage order: expected {:?}, got {:?}",
            FALLBACK_RUNTIME_STAGE_ORDER,
            runtime_order
        )));
    }
    Ok(runtime_order)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageLoopConfig {
    pub bounded_mode: bool,
}

impl Default for StageLoopConfig {
    fn default() -> Self {
        Self { bounded_mode: true }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageLoopInput {
    pub run_id: String,
    pub workflow_id: String,
    pub failure_code: String,
    pub failure_summary: String,
    pub evidence_refs: Vec<String>,
}

impl StageLoopInput {
    pub fn validate(&self) -> Result<(), StageLoopError> {
        if self.run_id.trim().is_empty()
            || self.workflow_id.trim().is_empty()
            || self.failure_code.trim().is_empty()
            || self.failure_summary.trim().is_empty()
        {
            return Err(StageLoopError::InvalidInput(
                "run_id, workflow_id, failure_code, and failure_summary must be non-empty"
                    .to_string(),
            ));
        }

        for path in &self.evidence_refs {
            if path.trim().is_empty()
                || path.starts_with('/')
                || path.contains("..")
                || path.contains(':')
                || path.contains('\\')
            {
                return Err(StageLoopError::InvalidInput(format!(
                    "evidence ref '{path}' must be a safe relative path"
                )));
            }
        }
        Ok(())
    }

    fn normalized_evidence_refs(&self) -> Vec<String> {
        let mut refs = self.evidence_refs.clone();
        refs.sort();
        refs.dedup();
        refs
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageTransitionEvent {
    pub stage: GodelStage,
    pub transition: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageLoopRun {
    pub stage_order: Vec<GodelStage>,
    pub transitions: Vec<StageTransitionEvent>,
    pub hypotheses: Vec<HypothesisCandidate>,
    pub mutation_plan: MutationPlan,
    pub hypothesis: HypothesisCandidate,
    pub mutation: MutationProposal,
    pub experiment_result: String,
    pub evaluation: EvaluationOutcome,
    pub record: StageExperimentRecord,
    pub index_entry: StageIndexEntry,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageLoopPersistenceResult {
    pub run: StageLoopRun,
    pub hypothesis_rel_path: PathBuf,
    pub policy_rel_path: PathBuf,
    pub policy_comparison_rel_path: PathBuf,
    pub canonical_evaluation_plan_rel_path: PathBuf,
    pub canonical_mutation_rel_path: PathBuf,
    pub canonical_evidence_rel_path: PathBuf,
    pub experiment_record_rel_path: PathBuf,
    pub canonical_experiment_record_rel_path: PathBuf,
    pub obsmem_index_rel_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StageLoopError {
    InvalidInput(String),
    DeterminismViolation(String),
}

impl std::fmt::Display for StageLoopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "GODEL_STAGE_LOOP_INVALID_INPUT: {msg}"),
            Self::DeterminismViolation(msg) => {
                write!(f, "GODEL_STAGE_LOOP_DETERMINISM_VIOLATION: {msg}")
            }
        }
    }
}

impl std::error::Error for StageLoopError {}

#[derive(Debug, Clone)]
pub struct GodelStageLoopExecutor {
    config: StageLoopConfig,
}

impl GodelStageLoopExecutor {
    pub fn new(config: StageLoopConfig) -> Self {
        Self { config }
    }

    pub fn execute(&self, input: &StageLoopInput) -> Result<StageLoopRun, StageLoopError> {
        if !self.config.bounded_mode {
            return Err(StageLoopError::InvalidInput(
                "only bounded_mode=true is supported in v0.8".to_string(),
            ));
        }
        input.validate()?;
        let stage_order = canonical_runtime_stage_order()?;

        let mut transitions = Vec::new();
        for stage in &stage_order {
            transitions.push(StageTransitionEvent {
                stage: *stage,
                transition: "entered".to_string(),
            });
            transitions.push(StageTransitionEvent {
                stage: *stage,
                transition: "completed".to_string(),
            });
        }

        let refs = input.normalized_evidence_refs();
        let hypotheses = hypothesis::generate_hypotheses(&HypothesisPipelineInput {
            run_id: input.run_id.clone(),
            workflow_id: input.workflow_id.clone(),
            failure_code: input.failure_code.clone(),
            failure_summary: input.failure_summary.clone(),
            evidence_refs: refs.clone(),
        });
        let mutation_plan = mutation::propose_mutations(&input.run_id, &hypotheses);
        let hypothesis = hypotheses
            .iter()
            .find(|h| h.id == mutation_plan.selected_hypothesis_id)
            .cloned()
            .or_else(|| hypotheses.first().cloned())
            .ok_or_else(|| {
                StageLoopError::DeterminismViolation(
                    "hypothesis pipeline produced no deterministic candidate".to_string(),
                )
            })?;
        let mutation = mutation_plan
            .proposals
            .iter()
            .find(|m| m.hypothesis_id == hypothesis.id)
            .cloned()
            .or_else(|| mutation_plan.proposals.first().cloned())
            .ok_or_else(|| {
                StageLoopError::DeterminismViolation(
                    "mutation pipeline produced no deterministic proposal".to_string(),
                )
            })?;

        let experiment_result = "bounded_experiment_completed".to_string();
        let score_delta = if input.failure_code.contains("transient") {
            0
        } else {
            1
        };
        let evaluation =
            evaluation::evaluate_experiment(&input.failure_code, &experiment_result, score_delta);
        let record = experiment_record::build_record(
            &input.run_id,
            &input.workflow_id,
            &input.failure_code,
            &refs,
            &mutation,
            &evaluation,
        );
        let index_entry = obsmem_index::build_index_entry(&record, &input.failure_code);

        let run = StageLoopRun {
            stage_order,
            transitions,
            hypotheses,
            mutation_plan,
            hypothesis,
            mutation,
            experiment_result,
            evaluation,
            record,
            index_entry,
        };
        self.validate_deterministic_contract(&run)?;
        Ok(run)
    }

    pub fn execute_and_persist(
        &self,
        input: &StageLoopInput,
        runs_root: &Path,
    ) -> Result<StageLoopPersistenceResult, StageLoopError> {
        let run = self.execute(input)?;
        let hypothesis_artifact = hypothesis::build_persisted_hypothesis_artifact(
            &HypothesisPipelineInput {
                run_id: input.run_id.clone(),
                workflow_id: input.workflow_id.clone(),
                failure_code: input.failure_code.clone(),
                failure_summary: input.failure_summary.clone(),
                evidence_refs: input.normalized_evidence_refs(),
            },
            &run.hypothesis,
        );
        let hypothesis_rel_path =
            hypothesis::persist_hypothesis_artifact(runs_root, &input.run_id, &hypothesis_artifact)
                .map_err(|err| {
                    StageLoopError::InvalidInput(format!(
                        "hypothesis artifact persistence failed: {err}"
                    ))
                })?;
        let (policy_artifact, policy_comparison_artifact) =
            policy::build_policy_artifacts(&hypothesis_artifact, &hypothesis_rel_path).map_err(
                |err| StageLoopError::InvalidInput(format!("policy artifact build failed: {err}")),
            )?;
        let policy_rel_path = policy::persist_policy_artifact(
            runs_root,
            &input.run_id,
            &policy_artifact,
        )
        .map_err(|err| {
            StageLoopError::InvalidInput(format!("policy artifact persistence failed: {err}"))
        })?;
        let policy_comparison_rel_path = policy::persist_policy_comparison_artifact(
            runs_root,
            &input.run_id,
            &policy_comparison_artifact,
        )
        .map_err(|err| {
            StageLoopError::InvalidInput(format!("policy comparison persistence failed: {err}"))
        })?;
        let canonical_mutation = mutation::build_canonical_mutation(
            &input.run_id,
            &input.workflow_id,
            &input.failure_code,
            &run.hypothesis,
            &run.mutation,
        )
        .map_err(|err| {
            StageLoopError::InvalidInput(format!("canonical mutation build failed: {err}"))
        })?;
        let canonical_mutation_rel_path =
            mutation::persist_canonical_mutation(runs_root, &input.run_id, &canonical_mutation)
                .map_err(|err| {
                    StageLoopError::InvalidInput(format!(
                        "canonical mutation persistence failed: {err}"
                    ))
                })?;
        let canonical_evidence =
            canonical_evidence::build_canonical_evidence(input).map_err(|err| {
                StageLoopError::InvalidInput(format!("canonical evidence build failed: {err}"))
            })?;
        let canonical_evidence_rel_path =
            canonical_evidence::persist_canonical_evidence(runs_root, &canonical_evidence)
                .map_err(|err| {
                    StageLoopError::InvalidInput(format!(
                        "canonical evidence persistence failed: {err}"
                    ))
                })?;
        let canonical_evaluation_plan = evaluation::build_canonical_evaluation_plan(
            &input.run_id,
            &input.workflow_id,
            &input.failure_code,
            &input.normalized_evidence_refs(),
            &run.hypothesis,
            &run.mutation,
        )
        .map_err(|err| {
            StageLoopError::InvalidInput(format!("canonical evaluation plan build failed: {err}"))
        })?;
        let canonical_evaluation_plan_rel_path = evaluation::persist_canonical_evaluation_plan(
            runs_root,
            &input.run_id,
            &canonical_evaluation_plan,
        )
        .map_err(|err| {
            StageLoopError::InvalidInput(format!(
                "canonical evaluation plan persistence failed: {err}"
            ))
        })?;
        let experiment_record_rel_path = experiment_record::persist_record(runs_root, &run.record)
            .map_err(|err| {
                StageLoopError::InvalidInput(format!("experiment record persistence failed: {err}"))
            })?;
        let obsmem_index_rel_path = obsmem_index::persist_index_entry(runs_root, &run.index_entry)
            .map_err(|err| {
                StageLoopError::InvalidInput(format!("obsmem index persistence failed: {err}"))
            })?;
        let canonical_record = experiment_record::build_canonical_record(
            runs_root,
            &run.record,
            &experiment_record_rel_path,
            &obsmem_index_rel_path,
        )
        .map_err(|err| {
            StageLoopError::InvalidInput(format!("canonical experiment record build failed: {err}"))
        })?;
        let canonical_experiment_record_rel_path =
            experiment_record::persist_canonical_record(runs_root, &canonical_record).map_err(
                |err| {
                    StageLoopError::InvalidInput(format!(
                        "canonical experiment record persistence failed: {err}"
                    ))
                },
            )?;

        Ok(StageLoopPersistenceResult {
            run,
            hypothesis_rel_path,
            policy_rel_path,
            policy_comparison_rel_path,
            canonical_evaluation_plan_rel_path,
            canonical_mutation_rel_path,
            canonical_evidence_rel_path,
            experiment_record_rel_path,
            canonical_experiment_record_rel_path,
            obsmem_index_rel_path,
        })
    }

    fn validate_deterministic_contract(&self, run: &StageLoopRun) -> Result<(), StageLoopError> {
        if run.stage_order != canonical_runtime_stage_order()? {
            return Err(StageLoopError::DeterminismViolation(
                "stage order diverged from canonical sequence".to_string(),
            ));
        }
        for chunk in run.transitions.chunks(2) {
            if chunk.len() != 2
                || chunk[0].transition != "entered"
                || chunk[1].transition != "completed"
            {
                return Err(StageLoopError::DeterminismViolation(
                    "stage transitions must be entered->completed for each stage".to_string(),
                ));
            }
            if chunk[0].stage != chunk[1].stage {
                return Err(StageLoopError::DeterminismViolation(
                    "transition stage mismatch".to_string(),
                ));
            }
        }
        if !run.hypotheses.windows(2).all(|w| w[0].id <= w[1].id) {
            return Err(StageLoopError::DeterminismViolation(
                "hypothesis candidates must be sorted lexicographically by hypothesis_id"
                    .to_string(),
            ));
        }
        if !run
            .mutation_plan
            .proposals
            .windows(2)
            .all(|w| w[0].id <= w[1].id)
        {
            return Err(StageLoopError::DeterminismViolation(
                "mutation proposals must be sorted lexicographically by mutation_id".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture_input() -> StageLoopInput {
        StageLoopInput {
            run_id: "run-745-a".to_string(),
            workflow_id: "wf-godel-loop".to_string(),
            failure_code: "tool_failure".to_string(),
            failure_summary: "step failed with deterministic parse error".to_string(),
            evidence_refs: vec![
                "runs/run-745-a/logs/activation_log.json".to_string(),
                "runs/run-745-a/run_status.json".to_string(),
                "runs/run-745-a/run_status.json".to_string(),
            ],
        }
    }

    fn test_tmp_dir(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("adl-godel-{label}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("mkdir test tmp");
        root
    }

    #[test]
    fn stage_loop_executes_canonical_stage_sequence() {
        let exec = GodelStageLoopExecutor::new(StageLoopConfig::default());
        let run = exec.execute(&fixture_input()).expect("stage loop run");
        assert_eq!(
            run.stage_order,
            canonical_runtime_stage_order().expect("template-derived runtime stage order")
        );
        assert_eq!(run.transitions.len(), run.stage_order.len() * 2);
        assert_eq!(run.transitions[0].stage, GodelStage::Failure);
        assert_eq!(
            run.transitions.last().expect("final transition").stage,
            GodelStage::Indexing
        );
    }

    #[test]
    fn stage_loop_is_deterministic_for_identical_inputs() {
        let exec = GodelStageLoopExecutor::new(StageLoopConfig::default());
        let input = fixture_input();
        let left = exec.execute(&input).expect("left run");
        let right = exec.execute(&input).expect("right run");
        assert_eq!(left, right);
    }

    #[test]
    fn stage_loop_exposes_deterministic_hypothesis_and_mutation_pipeline() {
        let exec = GodelStageLoopExecutor::new(StageLoopConfig::default());
        let run = exec.execute(&fixture_input()).expect("stage loop run");
        assert!(!run.hypotheses.is_empty());
        assert!(!run.mutation_plan.proposals.is_empty());
        assert!(run.hypotheses.windows(2).all(|w| w[0].id <= w[1].id));
        assert!(run
            .mutation_plan
            .proposals
            .windows(2)
            .all(|w| w[0].id <= w[1].id));
        assert_eq!(run.hypothesis.id, run.mutation_plan.selected_hypothesis_id);
        assert_eq!(run.mutation.hypothesis_id, run.hypothesis.id);
    }

    #[test]
    fn stage_loop_rejects_unsafe_evidence_paths() {
        let exec = GodelStageLoopExecutor::new(StageLoopConfig::default());
        let mut input = fixture_input();
        input.evidence_refs = vec!["/Users/daniel/secret.json".to_string()];
        let err = exec.execute(&input).expect_err("must reject absolute path");
        assert!(err.to_string().contains("GODEL_STAGE_LOOP_INVALID_INPUT"));
    }

    #[test]
    fn stage_loop_rejects_unbounded_mode() {
        let exec = GodelStageLoopExecutor::new(StageLoopConfig {
            bounded_mode: false,
        });
        let err = exec
            .execute(&fixture_input())
            .expect_err("unbounded mode must be rejected");
        assert!(err.to_string().contains("GODEL_STAGE_LOOP_INVALID_INPUT"));
    }

    #[test]
    fn stage_loop_rejects_missing_required_fields() {
        let exec = GodelStageLoopExecutor::new(StageLoopConfig::default());
        let mut input = fixture_input();
        input.failure_summary.clear();
        let err = exec
            .execute(&input)
            .expect_err("empty required fields must be rejected");
        assert!(err.to_string().contains("must be non-empty"));
    }

    #[test]
    fn stage_loop_rejects_windows_style_evidence_paths() {
        let exec = GodelStageLoopExecutor::new(StageLoopConfig::default());
        let mut input = fixture_input();
        input.evidence_refs = vec!["runs\\r1\\activation_log.json".to_string()];
        let err = exec
            .execute(&input)
            .expect_err("windows path separators must be rejected");
        assert!(err.to_string().contains("safe relative path"));
    }

    #[test]
    fn stage_loop_transient_failure_branch_sets_zero_improvement() {
        let exec = GodelStageLoopExecutor::new(StageLoopConfig::default());
        let mut input = fixture_input();
        input.failure_code = "transient_timeout".to_string();
        let run = exec.execute(&input).expect("stage loop run");
        assert_eq!(run.evaluation.score_delta, 0);
    }

    #[test]
    fn deterministic_contract_rejects_stage_order_drift() {
        let exec = GodelStageLoopExecutor::new(StageLoopConfig::default());
        let mut run = exec.execute(&fixture_input()).expect("stage loop run");
        run.stage_order.swap(0, 1);
        let err = exec
            .validate_deterministic_contract(&run)
            .expect_err("must reject stage order drift");
        assert!(err
            .to_string()
            .contains("GODEL_STAGE_LOOP_DETERMINISM_VIOLATION"));
    }

    #[test]
    fn deterministic_contract_rejects_transition_pattern_drift() {
        let exec = GodelStageLoopExecutor::new(StageLoopConfig::default());
        let mut run = exec.execute(&fixture_input()).expect("stage loop run");
        run.transitions[1].transition = "entered".to_string();
        let err = exec
            .validate_deterministic_contract(&run)
            .expect_err("must reject transition drift");
        assert!(err
            .to_string()
            .contains("GODEL_STAGE_LOOP_DETERMINISM_VIOLATION"));
    }

    #[test]
    fn deterministic_contract_rejects_transition_stage_mismatch() {
        let exec = GodelStageLoopExecutor::new(StageLoopConfig::default());
        let mut run = exec.execute(&fixture_input()).expect("stage loop run");
        run.transitions[1].stage = GodelStage::Mutation;
        let err = exec
            .validate_deterministic_contract(&run)
            .expect_err("must reject stage mismatch");
        assert!(err.to_string().contains("transition stage mismatch"));
    }

    #[test]
    fn deterministic_contract_rejects_hypothesis_order_drift() {
        let exec = GodelStageLoopExecutor::new(StageLoopConfig::default());
        let mut run = exec.execute(&fixture_input()).expect("stage loop run");
        if run.hypotheses.len() > 1 {
            run.hypotheses.swap(0, 1);
        } else {
            run.hypotheses.push(HypothesisCandidate {
                id: "hyp:z".to_string(),
                statement: "z".to_string(),
                failure_code: "tool_failure".to_string(),
                evidence_refs: vec![],
            });
            run.hypotheses.insert(
                0,
                HypothesisCandidate {
                    id: "hyp:y".to_string(),
                    statement: "y".to_string(),
                    failure_code: "tool_failure".to_string(),
                    evidence_refs: vec![],
                },
            );
        }
        let err = exec
            .validate_deterministic_contract(&run)
            .expect_err("must reject hypothesis order drift");
        assert!(err
            .to_string()
            .contains("GODEL_STAGE_LOOP_DETERMINISM_VIOLATION"));
    }

    #[test]
    fn deterministic_contract_rejects_mutation_order_drift() {
        let exec = GodelStageLoopExecutor::new(StageLoopConfig::default());
        let mut run = exec.execute(&fixture_input()).expect("stage loop run");
        if run.mutation_plan.proposals.len() > 1 {
            run.mutation_plan.proposals.swap(0, 1);
        } else {
            run.mutation_plan.proposals.push(MutationProposal {
                id: "mut:z".to_string(),
                hypothesis_id: run.hypothesis.id.clone(),
                target_surface: "workflow-step-config".to_string(),
                bounded_change: "z".to_string(),
            });
            run.mutation_plan.proposals.insert(
                0,
                MutationProposal {
                    id: "mut:y".to_string(),
                    hypothesis_id: run.hypothesis.id.clone(),
                    target_surface: "workflow-step-config".to_string(),
                    bounded_change: "y".to_string(),
                },
            );
        }
        let err = exec
            .validate_deterministic_contract(&run)
            .expect_err("must reject mutation order drift");
        assert!(err
            .to_string()
            .contains("GODEL_STAGE_LOOP_DETERMINISM_VIOLATION"));
    }

    #[test]
    fn execute_and_persist_writes_record_and_indexing_runtime_artifacts() {
        let exec = GodelStageLoopExecutor::new(StageLoopConfig::default());
        let tmp = test_tmp_dir("stage-loop-persist");
        let persisted = exec
            .execute_and_persist(&fixture_input(), &tmp)
            .expect("persisted stage loop");

        assert_eq!(
            persisted.hypothesis_rel_path,
            PathBuf::from("runs/run-745-a/godel/godel_hypothesis.v1.json")
        );
        assert_eq!(
            persisted.policy_rel_path,
            PathBuf::from("runs/run-745-a/godel/godel_policy.v1.json")
        );
        assert_eq!(
            persisted.policy_comparison_rel_path,
            PathBuf::from("runs/run-745-a/godel/godel_policy_comparison.v1.json")
        );
        assert_eq!(
            persisted.canonical_evaluation_plan_rel_path,
            PathBuf::from("runs/run-745-a/godel/evaluation_plan.v1.json")
        );
        assert_eq!(
            persisted.canonical_mutation_rel_path,
            PathBuf::from("runs/run-745-a/godel/mutation.v1.json")
        );
        assert_eq!(
            persisted.canonical_evidence_rel_path,
            PathBuf::from("runs/run-745-a/godel/canonical_evidence_view.v1.json")
        );
        assert_eq!(
            persisted.experiment_record_rel_path,
            PathBuf::from("runs/run-745-a/godel/experiment_record.runtime.v1.json")
        );
        assert_eq!(
            persisted.canonical_experiment_record_rel_path,
            PathBuf::from("runs/run-745-a/godel/experiment_record.v1.json")
        );
        assert_eq!(
            persisted.obsmem_index_rel_path,
            PathBuf::from("runs/run-745-a/godel/obsmem_index_entry.runtime.v1.json")
        );
        assert!(tmp.join("run-745-a/godel/mutation.v1.json").is_file());
        assert!(tmp
            .join("run-745-a/godel/godel_hypothesis.v1.json")
            .is_file());
        assert!(tmp.join("run-745-a/godel/godel_policy.v1.json").is_file());
        assert!(tmp
            .join("run-745-a/godel/godel_policy_comparison.v1.json")
            .is_file());
        assert!(tmp
            .join("run-745-a/godel/canonical_evidence_view.v1.json")
            .is_file());
        assert!(tmp
            .join("run-745-a/godel/evaluation_plan.v1.json")
            .is_file());
        assert!(tmp
            .join("run-745-a/godel/experiment_record.v1.json")
            .is_file());
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn execute_and_persist_surfaces_record_persistence_error() {
        let exec = GodelStageLoopExecutor::new(StageLoopConfig::default());
        let tmp = test_tmp_dir("stage-loop-persist-error");
        let file_root = tmp.join("file-root");
        std::fs::write(&file_root, "not-a-directory").expect("write file root");

        let err = exec
            .execute_and_persist(&fixture_input(), &file_root)
            .expect_err("persist should fail against file root");
        assert!(err.to_string().contains("GODEL_STAGE_LOOP_INVALID_INPUT"));
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn template_derived_runtime_stage_order_appends_runtime_managed_indexing() {
        let template = embedded_v08_workflow_template().expect("template");
        let runtime_order =
            runtime_stage_order_from_template(&template).expect("template-derived runtime order");
        assert_eq!(
            runtime_order,
            vec![
                GodelStage::Failure,
                GodelStage::Hypothesis,
                GodelStage::Mutation,
                GodelStage::Experiment,
                GodelStage::Evaluation,
                GodelStage::Record,
                GodelStage::Indexing,
            ]
        );
    }

    #[test]
    fn template_derived_runtime_stage_order_rejects_unknown_stage_ids() {
        let template = GodelWorkflowTemplate {
            template_name: "godel_experiment_workflow".to_string(),
            template_version: 1,
            stage_order: vec!["failure".to_string(), "publish".to_string()],
            stages: vec![],
            determinism: embedded_v08_workflow_template()
                .expect("embedded template")
                .determinism,
            security_privacy: embedded_v08_workflow_template()
                .expect("embedded template")
                .security_privacy,
            replay_audit: embedded_v08_workflow_template()
                .expect("embedded template")
                .replay_audit,
            downstream: embedded_v08_workflow_template()
                .expect("embedded template")
                .downstream,
        };
        let err = runtime_stage_order_from_template(&template)
            .expect_err("unsupported template stage must fail");
        assert!(err.to_string().contains("unsupported stage"));
    }
}
