use serde::{Deserialize, Serialize};

use super::hypothesis::HypothesisCandidate;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationProposal {
    pub id: String,
    pub hypothesis_id: String,
    pub target_surface: String,
    pub bounded_change: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationPlan {
    pub selected_hypothesis_id: String,
    pub proposals: Vec<MutationProposal>,
    pub ordering_rule: String,
}

pub fn propose_mutations(run_id: &str, hypotheses: &[HypothesisCandidate]) -> MutationPlan {
    let mut sorted_hypotheses = hypotheses.to_vec();
    sorted_hypotheses.sort_by(|a, b| a.id.cmp(&b.id));

    let mut proposals = Vec::with_capacity(sorted_hypotheses.len());
    for (idx, hypothesis) in sorted_hypotheses.iter().enumerate() {
        let target_surface = match hypothesis.failure_code.as_str() {
            "tool_failure" => "tool-invocation-config",
            "policy_denied" => "delegation-policy-input",
            "verification_failed" => "verification-gate-input",
            _ => "workflow-step-config",
        };
        proposals.push(MutationProposal {
            id: format!("mut:{run_id}:{}:{idx:02}", hypothesis.failure_code),
            hypothesis_id: hypothesis.id.clone(),
            target_surface: target_surface.to_string(),
            bounded_change: format!(
                "Apply deterministic bounded adjustment for failure_code={} based on hypothesis_id={}.",
                hypothesis.failure_code, hypothesis.id
            ),
        });
    }
    proposals.sort_by(|a, b| a.id.cmp(&b.id));

    let selected_hypothesis_id = sorted_hypotheses
        .first()
        .map(|h| h.id.clone())
        .unwrap_or_else(|| format!("hyp:{run_id}:none:00"));

    MutationPlan {
        selected_hypothesis_id,
        proposals,
        ordering_rule: "proposal order sorted lexicographically by mutation_id".to_string(),
    }
}

pub fn propose_mutation(run_id: &str, hypothesis: &HypothesisCandidate) -> MutationProposal {
    let plan = propose_mutations(run_id, std::slice::from_ref(hypothesis));
    plan.proposals
        .into_iter()
        .next()
        .expect("mutation pipeline must produce at least one proposal")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_mutation_is_deterministic() {
        let hypothesis = HypothesisCandidate {
            id: "hyp:run-1:tool_failure:00".to_string(),
            statement: "test".to_string(),
            failure_code: "tool_failure".to_string(),
            evidence_refs: vec![],
        };
        let left = propose_mutation("run-1", &hypothesis);
        let right = propose_mutation("run-1", &hypothesis);
        assert_eq!(left, right);
    }

    #[test]
    fn propose_mutations_orders_by_hypothesis_id_deterministically() {
        let hypotheses = vec![
            HypothesisCandidate {
                id: "hyp:run-1:tool_failure:01".to_string(),
                statement: "secondary".to_string(),
                failure_code: "tool_failure".to_string(),
                evidence_refs: vec![],
            },
            HypothesisCandidate {
                id: "hyp:run-1:tool_failure:00".to_string(),
                statement: "primary".to_string(),
                failure_code: "tool_failure".to_string(),
                evidence_refs: vec![],
            },
        ];

        let plan = propose_mutations("run-1", &hypotheses);
        assert_eq!(plan.selected_hypothesis_id, "hyp:run-1:tool_failure:00");
        assert_eq!(plan.proposals.len(), 2);
        assert!(plan.proposals[0].id < plan.proposals[1].id);
        assert_eq!(plan.proposals[0].target_surface, "tool-invocation-config");
    }
}
