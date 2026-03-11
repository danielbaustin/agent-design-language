use serde::{Deserialize, Serialize};

use super::evaluation::{self, EvaluationOutcome};
use super::experiment_record::{self, StageExperimentRecord};
use super::hypothesis::{self, HypothesisCandidate, HypothesisPipelineInput};
use super::mutation::{self, MutationPlan, MutationProposal};
use super::obsmem_index::{self, StageIndexEntry};

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

pub const STAGE_ORDER: [GodelStage; 7] = [
    GodelStage::Failure,
    GodelStage::Hypothesis,
    GodelStage::Mutation,
    GodelStage::Experiment,
    GodelStage::Evaluation,
    GodelStage::Record,
    GodelStage::Indexing,
];

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

        let mut transitions = Vec::new();
        for stage in STAGE_ORDER {
            transitions.push(StageTransitionEvent {
                stage,
                transition: "entered".to_string(),
            });
            transitions.push(StageTransitionEvent {
                stage,
                transition: "completed".to_string(),
            });
        }

        let refs = input.normalized_evidence_refs();
        let hypotheses = hypothesis::generate_hypotheses(&HypothesisPipelineInput {
            run_id: input.run_id.clone(),
            failure_code: input.failure_code.clone(),
            failure_summary: input.failure_summary.clone(),
            evidence_refs: refs,
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
            &mutation,
            &evaluation,
        );
        let index_entry = obsmem_index::build_index_entry(&record, &input.failure_code);

        let run = StageLoopRun {
            stage_order: STAGE_ORDER.to_vec(),
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

    fn validate_deterministic_contract(&self, run: &StageLoopRun) -> Result<(), StageLoopError> {
        if run.stage_order != STAGE_ORDER {
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

    #[test]
    fn stage_loop_executes_canonical_stage_sequence() {
        let exec = GodelStageLoopExecutor::new(StageLoopConfig::default());
        let run = exec.execute(&fixture_input()).expect("stage loop run");
        assert_eq!(run.stage_order, STAGE_ORDER);
        assert_eq!(run.transitions.len(), STAGE_ORDER.len() * 2);
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
}
