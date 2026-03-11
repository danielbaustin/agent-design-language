use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HypothesisCandidate {
    pub id: String,
    pub statement: String,
    pub failure_code: String,
    pub evidence_refs: Vec<String>,
}

pub fn derive_hypothesis(
    run_id: &str,
    failure_code: &str,
    failure_summary: &str,
    evidence_refs: &[String],
) -> HypothesisCandidate {
    let mut refs = evidence_refs.to_vec();
    refs.sort();
    refs.dedup();

    HypothesisCandidate {
        id: format!("hyp:{run_id}:{failure_code}"),
        statement: format!(
            "Hypothesis: failure_code={failure_code} indicates a bounded execution weakness derived from '{failure_summary}'."
        ),
        failure_code: failure_code.to_string(),
        evidence_refs: refs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_hypothesis_normalizes_evidence_refs() {
        let refs = vec![
            "runs/r1/evidence/b.json".to_string(),
            "runs/r1/evidence/a.json".to_string(),
            "runs/r1/evidence/a.json".to_string(),
        ];
        let h = derive_hypothesis("run-1", "tool_failure", "boom", &refs);
        assert_eq!(h.id, "hyp:run-1:tool_failure");
        assert_eq!(
            h.evidence_refs,
            vec!["runs/r1/evidence/a.json", "runs/r1/evidence/b.json"]
        );
    }
}
