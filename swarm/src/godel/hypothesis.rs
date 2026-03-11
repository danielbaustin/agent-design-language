use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HypothesisCandidate {
    pub id: String,
    pub statement: String,
    pub failure_code: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HypothesisPipelineInput {
    pub run_id: String,
    pub failure_code: String,
    pub failure_summary: String,
    pub evidence_refs: Vec<String>,
}

pub fn generate_hypotheses(input: &HypothesisPipelineInput) -> Vec<HypothesisCandidate> {
    let mut refs = input.evidence_refs.clone();
    refs.sort();
    refs.dedup();

    let mut out = Vec::with_capacity(2);
    out.push(HypothesisCandidate {
        id: format!("hyp:{}:{}:00", input.run_id, input.failure_code),
        statement: format!(
            "Primary hypothesis: failure_code={} indicates a bounded execution weakness derived from '{}'.",
            input.failure_code, input.failure_summary
        ),
        failure_code: input.failure_code.clone(),
        evidence_refs: refs.clone(),
    });

    if !refs.is_empty() {
        out.push(HypothesisCandidate {
            id: format!("hyp:{}:{}:01", input.run_id, input.failure_code),
            statement: format!(
                "Secondary hypothesis: evidence-backed adjustment for failure_code={} should reduce repeat failures.",
                input.failure_code
            ),
            failure_code: input.failure_code.clone(),
            evidence_refs: refs,
        });
    }

    out.sort_by(|a, b| a.id.cmp(&b.id));
    out
}

pub fn derive_hypothesis(
    run_id: &str,
    failure_code: &str,
    failure_summary: &str,
    evidence_refs: &[String],
) -> HypothesisCandidate {
    let candidates = generate_hypotheses(&HypothesisPipelineInput {
        run_id: run_id.to_string(),
        failure_code: failure_code.to_string(),
        failure_summary: failure_summary.to_string(),
        evidence_refs: evidence_refs.to_vec(),
    });
    candidates
        .into_iter()
        .next()
        .expect("hypothesis pipeline must produce at least one candidate")
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
        assert_eq!(h.id, "hyp:run-1:tool_failure:00");
        assert_eq!(
            h.evidence_refs,
            vec!["runs/r1/evidence/a.json", "runs/r1/evidence/b.json"]
        );
    }

    #[test]
    fn generate_hypotheses_produces_stable_order_and_ids() {
        let generated = generate_hypotheses(&HypothesisPipelineInput {
            run_id: "run-9".to_string(),
            failure_code: "policy_denied".to_string(),
            failure_summary: "policy gate rejected request".to_string(),
            evidence_refs: vec![
                "runs/r9/evidence/c.json".to_string(),
                "runs/r9/evidence/a.json".to_string(),
                "runs/r9/evidence/a.json".to_string(),
            ],
        });
        assert_eq!(generated.len(), 2);
        assert_eq!(generated[0].id, "hyp:run-9:policy_denied:00");
        assert_eq!(generated[1].id, "hyp:run-9:policy_denied:01");
        assert_eq!(
            generated[0].evidence_refs,
            vec!["runs/r9/evidence/a.json", "runs/r9/evidence/c.json"]
        );
        assert_eq!(generated[0].evidence_refs, generated[1].evidence_refs);
    }
}
