use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub const HYPOTHESIS_ARTIFACT_VERSION: &str = "godel_hypothesis.v1";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersistedHypothesisArtifact {
    pub artifact_version: String,
    pub hypothesis_id: String,
    pub run_id: String,
    pub workflow_id: String,
    pub failure_id: String,
    pub failure_class: String,
    pub claim: String,
    pub confidence: f64,
    pub evidence_refs: Vec<String>,
    pub related_run_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HypothesisArtifactError {
    Invalid(String),
    Io(String),
    Serialize(String),
}

impl std::fmt::Display for HypothesisArtifactError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(msg) => write!(f, "GODEL_HYPOTHESIS_INVALID: {msg}"),
            Self::Io(msg) => write!(f, "GODEL_HYPOTHESIS_IO: {msg}"),
            Self::Serialize(msg) => write!(f, "GODEL_HYPOTHESIS_SERIALIZE: {msg}"),
        }
    }
}

impl std::error::Error for HypothesisArtifactError {}

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
    pub workflow_id: String,
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
        workflow_id: "workflow".to_string(),
        failure_code: failure_code.to_string(),
        failure_summary: failure_summary.to_string(),
        evidence_refs: evidence_refs.to_vec(),
    });
    candidates
        .into_iter()
        .next()
        .expect("hypothesis pipeline must produce at least one candidate")
}

pub fn build_persisted_hypothesis_artifact(
    input: &HypothesisPipelineInput,
    hypothesis: &HypothesisCandidate,
) -> PersistedHypothesisArtifact {
    let mut evidence_refs = hypothesis.evidence_refs.clone();
    evidence_refs.sort();
    evidence_refs.dedup();

    let mut related_run_refs = vec![input.run_id.clone()];
    related_run_refs.sort();
    related_run_refs.dedup();

    let confidence = if hypothesis.id.ends_with(":00") {
        0.67
    } else {
        0.41
    };

    PersistedHypothesisArtifact {
        artifact_version: HYPOTHESIS_ARTIFACT_VERSION.to_string(),
        hypothesis_id: hypothesis.id.clone(),
        run_id: input.run_id.clone(),
        workflow_id: input.workflow_id.clone(),
        failure_id: format!("failure:{}:{}", input.run_id, input.failure_code),
        failure_class: input.failure_code.clone(),
        claim: hypothesis.statement.clone(),
        confidence,
        evidence_refs,
        related_run_refs,
    }
}

pub fn persist_hypothesis_artifact(
    runs_root: &Path,
    run_id: &str,
    artifact: &PersistedHypothesisArtifact,
) -> Result<PathBuf, HypothesisArtifactError> {
    if run_id.trim().is_empty() {
        return Err(HypothesisArtifactError::Invalid(
            "run_id must be non-empty".to_string(),
        ));
    }
    if artifact.hypothesis_id.trim().is_empty()
        || artifact.failure_class.trim().is_empty()
        || artifact.claim.trim().is_empty()
    {
        return Err(HypothesisArtifactError::Invalid(
            "hypothesis artifact requires non-empty id, failure class, and claim".to_string(),
        ));
    }
    if artifact
        .evidence_refs
        .iter()
        .any(|path| path.trim().is_empty() || path.starts_with('/') || path.contains(".."))
    {
        return Err(HypothesisArtifactError::Invalid(
            "hypothesis evidence refs must be safe relative paths".to_string(),
        ));
    }

    let rel_path = PathBuf::from("runs")
        .join(run_id)
        .join("godel")
        .join("godel_hypothesis.v1.json");
    let out_dir = runs_root.join(run_id).join("godel");
    fs::create_dir_all(&out_dir)
        .map_err(|err| HypothesisArtifactError::Io(format!("create dir failed: {err}")))?;
    let json = serde_json::to_string_pretty(artifact)
        .map_err(|err| HypothesisArtifactError::Serialize(err.to_string()))?;
    fs::write(out_dir.join("godel_hypothesis.v1.json"), json)
        .map_err(|err| HypothesisArtifactError::Io(format!("write failed: {err}")))?;
    Ok(rel_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

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
            workflow_id: "wf-9".to_string(),
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

    #[test]
    fn persisted_hypothesis_artifact_is_deterministic() {
        let input = HypothesisPipelineInput {
            run_id: "run-12".to_string(),
            workflow_id: "wf-12".to_string(),
            failure_code: "verification_failed".to_string(),
            failure_summary: "verification mode omitted".to_string(),
            evidence_refs: vec![
                "runs/run-12/evidence/b.json".to_string(),
                "runs/run-12/evidence/a.json".to_string(),
            ],
        };
        let hypothesis = derive_hypothesis(
            &input.run_id,
            &input.failure_code,
            &input.failure_summary,
            &input.evidence_refs,
        );
        let first = build_persisted_hypothesis_artifact(&input, &hypothesis);
        let second = build_persisted_hypothesis_artifact(&input, &hypothesis);
        assert_eq!(first, second);
        assert_eq!(first.artifact_version, HYPOTHESIS_ARTIFACT_VERSION);
        assert_eq!(first.failure_id, "failure:run-12:verification_failed");
        assert_eq!(first.related_run_refs, vec!["run-12"]);
    }

    #[test]
    fn persist_hypothesis_artifact_writes_expected_runtime_path() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let tmp = std::env::temp_dir().join(format!(
            "adl-hypothesis-persist-pid{}-{nonce}",
            std::process::id()
        ));
        let input = HypothesisPipelineInput {
            run_id: "run-15".to_string(),
            workflow_id: "wf-15".to_string(),
            failure_code: "tool_failure".to_string(),
            failure_summary: "tool returned invalid json".to_string(),
            evidence_refs: vec!["runs/run-15/run_status.json".to_string()],
        };
        let hypothesis = derive_hypothesis(
            &input.run_id,
            &input.failure_code,
            &input.failure_summary,
            &input.evidence_refs,
        );
        let artifact = build_persisted_hypothesis_artifact(&input, &hypothesis);
        let rel = persist_hypothesis_artifact(&tmp, &input.run_id, &artifact).expect("persist");
        assert_eq!(
            rel,
            PathBuf::from("runs/run-15/godel/godel_hypothesis.v1.json")
        );
        assert!(tmp.join("run-15/godel/godel_hypothesis.v1.json").is_file());
        let _ = fs::remove_dir_all(tmp);
    }
}
