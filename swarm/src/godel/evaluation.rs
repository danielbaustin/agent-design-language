use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvaluationDecision {
    Adopt,
    Reject,
    Review,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluationOutcome {
    pub decision: EvaluationDecision,
    pub rationale: String,
    pub score_delta: i32,
}

pub fn evaluate_experiment(
    failure_code: &str,
    experiment_result: &str,
    score_delta: i32,
) -> EvaluationOutcome {
    let decision = if score_delta > 0 {
        EvaluationDecision::Adopt
    } else if experiment_result == "blocked" {
        EvaluationDecision::Review
    } else {
        EvaluationDecision::Reject
    };

    EvaluationOutcome {
        decision,
        rationale: format!(
            "Evaluation for failure_code={failure_code} produced decision={:?} with score_delta={score_delta}.",
            decision
        ),
        score_delta,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_experiment_selects_expected_decisions() {
        assert_eq!(
            evaluate_experiment("tool_failure", "ok", 1).decision,
            EvaluationDecision::Adopt
        );
        assert_eq!(
            evaluate_experiment("tool_failure", "ok", 0).decision,
            EvaluationDecision::Reject
        );
        assert_eq!(
            evaluate_experiment("tool_failure", "blocked", 0).decision,
            EvaluationDecision::Review
        );
    }
}
