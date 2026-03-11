use serde::{Deserialize, Serialize};

use super::evaluation::EvaluationOutcome;
use super::mutation::MutationProposal;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageExperimentRecord {
    pub run_id: String,
    pub workflow_id: String,
    pub hypothesis_id: String,
    pub mutation_id: String,
    pub evaluation_decision: String,
    pub improvement_delta: i32,
}

pub fn build_record(
    run_id: &str,
    workflow_id: &str,
    mutation: &MutationProposal,
    evaluation: &EvaluationOutcome,
) -> StageExperimentRecord {
    StageExperimentRecord {
        run_id: run_id.to_string(),
        workflow_id: workflow_id.to_string(),
        hypothesis_id: mutation.hypothesis_id.clone(),
        mutation_id: mutation.id.clone(),
        evaluation_decision: format!("{:?}", evaluation.decision).to_lowercase(),
        improvement_delta: evaluation.score_delta,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::godel::evaluation::{EvaluationDecision, EvaluationOutcome};
    use crate::godel::mutation::MutationProposal;

    #[test]
    fn build_record_uses_deterministic_field_mapping() {
        let m = MutationProposal {
            id: "mut:r1:tool_failure".to_string(),
            hypothesis_id: "hyp:r1:tool_failure".to_string(),
            target_surface: "x".to_string(),
            bounded_change: "y".to_string(),
        };
        let e = EvaluationOutcome {
            decision: EvaluationDecision::Adopt,
            rationale: "ok".to_string(),
            score_delta: 2,
        };
        let r = build_record("r1", "wf1", &m, &e);
        assert_eq!(r.evaluation_decision, "adopt");
        assert_eq!(r.improvement_delta, 2);
    }
}
