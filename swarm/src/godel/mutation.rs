use serde::{Deserialize, Serialize};

use super::hypothesis::HypothesisCandidate;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationProposal {
    pub id: String,
    pub hypothesis_id: String,
    pub target_surface: String,
    pub bounded_change: String,
}

pub fn propose_mutation(run_id: &str, hypothesis: &HypothesisCandidate) -> MutationProposal {
    MutationProposal {
        id: format!("mut:{run_id}:{}", hypothesis.failure_code),
        hypothesis_id: hypothesis.id.clone(),
        target_surface: "workflow-step-config".to_string(),
        bounded_change: format!(
            "Apply deterministic bounded adjustment for failure_code={} without introducing autonomous behavior.",
            hypothesis.failure_code
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_mutation_is_deterministic() {
        let hypothesis = HypothesisCandidate {
            id: "hyp:run-1:tool_failure".to_string(),
            statement: "test".to_string(),
            failure_code: "tool_failure".to_string(),
            evidence_refs: vec![],
        };
        let left = propose_mutation("run-1", &hypothesis);
        let right = propose_mutation("run-1", &hypothesis);
        assert_eq!(left, right);
    }
}
