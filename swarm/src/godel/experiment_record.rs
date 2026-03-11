use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::evaluation::EvaluationOutcome;
use super::mutation::MutationProposal;

pub const EXPERIMENT_RECORD_RUNTIME_SCHEMA: &str = "experiment_record.runtime.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageExperimentRecord {
    pub run_id: String,
    pub workflow_id: String,
    pub failure_code: String,
    pub hypothesis_id: String,
    pub mutation_id: String,
    pub mutation_target_surface: String,
    pub evaluation_decision: String,
    pub evaluation_rationale: String,
    pub improvement_delta: i32,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedExperimentRecord {
    pub schema: String,
    pub record: StageExperimentRecord,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExperimentRecordError {
    Invalid(String),
    Io(String),
    Serialize(String),
}

impl std::fmt::Display for ExperimentRecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(msg) => write!(f, "GODEL_EXPERIMENT_RECORD_INVALID: {msg}"),
            Self::Io(msg) => write!(f, "GODEL_EXPERIMENT_RECORD_IO: {msg}"),
            Self::Serialize(msg) => write!(f, "GODEL_EXPERIMENT_RECORD_SERIALIZE: {msg}"),
        }
    }
}

impl std::error::Error for ExperimentRecordError {}

pub fn build_record(
    run_id: &str,
    workflow_id: &str,
    failure_code: &str,
    evidence_refs: &[String],
    mutation: &MutationProposal,
    evaluation: &EvaluationOutcome,
) -> StageExperimentRecord {
    let mut refs = evidence_refs.to_vec();
    refs.sort();
    refs.dedup();

    StageExperimentRecord {
        run_id: run_id.to_string(),
        workflow_id: workflow_id.to_string(),
        failure_code: failure_code.to_string(),
        hypothesis_id: mutation.hypothesis_id.clone(),
        mutation_id: mutation.id.clone(),
        mutation_target_surface: mutation.target_surface.clone(),
        evaluation_decision: format!("{:?}", evaluation.decision).to_lowercase(),
        evaluation_rationale: evaluation.rationale.clone(),
        improvement_delta: evaluation.score_delta,
        evidence_refs: refs,
    }
}

pub fn persist_record(
    runs_root: &Path,
    record: &StageExperimentRecord,
) -> Result<PathBuf, ExperimentRecordError> {
    validate_record(record)?;

    let rel_path = PathBuf::from("runs")
        .join(&record.run_id)
        .join("godel")
        .join("experiment_record.runtime.v1.json");
    let out_path = runs_root.join(&record.run_id).join("godel");
    fs::create_dir_all(&out_path)
        .map_err(|err| ExperimentRecordError::Io(format!("create dir failed: {err}")))?;

    let persisted = PersistedExperimentRecord {
        schema: EXPERIMENT_RECORD_RUNTIME_SCHEMA.to_string(),
        record: record.clone(),
    };
    let json = serde_json::to_string_pretty(&persisted)
        .map_err(|err| ExperimentRecordError::Serialize(err.to_string()))?;

    fs::write(out_path.join("experiment_record.runtime.v1.json"), json)
        .map_err(|err| ExperimentRecordError::Io(format!("write failed: {err}")))?;

    Ok(rel_path)
}

fn validate_record(record: &StageExperimentRecord) -> Result<(), ExperimentRecordError> {
    if record.run_id.trim().is_empty()
        || record.workflow_id.trim().is_empty()
        || record.failure_code.trim().is_empty()
        || record.hypothesis_id.trim().is_empty()
        || record.mutation_id.trim().is_empty()
    {
        return Err(ExperimentRecordError::Invalid(
            "required id fields must be non-empty".to_string(),
        ));
    }

    for path in &record.evidence_refs {
        if path.trim().is_empty()
            || path.starts_with('/')
            || path.contains("..")
            || path.contains(':')
            || path.contains('\\')
        {
            return Err(ExperimentRecordError::Invalid(format!(
                "invalid evidence ref path: {path}"
            )));
        }
    }

    let mut content_scan = format!(
        "{}\n{}\n{}\n{}",
        record.failure_code,
        record.evaluation_rationale,
        record.mutation_target_surface,
        record.evidence_refs.join("\n")
    );
    content_scan.push('\n');
    content_scan.push_str(&record.hypothesis_id);

    if content_scan.contains("/Users/")
        || content_scan.contains("/home/")
        || content_scan.contains("gho_")
        || content_scan.contains("sk-")
    {
        return Err(ExperimentRecordError::Invalid(
            "record contains disallowed host-path or token-like content".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::godel::evaluation::{EvaluationDecision, EvaluationOutcome};
    use crate::godel::mutation::MutationProposal;
    use std::path::PathBuf;

    fn test_tmp_dir(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("adl-godel-{label}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("mkdir test tmp");
        root
    }

    #[test]
    fn build_record_uses_deterministic_field_mapping() {
        let m = MutationProposal {
            id: "mut:r1:tool_failure".to_string(),
            hypothesis_id: "hyp:r1:tool_failure".to_string(),
            target_surface: "workflow-step-config".to_string(),
            bounded_change: "y".to_string(),
        };
        let e = EvaluationOutcome {
            decision: EvaluationDecision::Adopt,
            rationale: "ok".to_string(),
            score_delta: 2,
        };
        let r = build_record(
            "r1",
            "wf1",
            "tool_failure",
            &["runs/r1/run_status.json".to_string()],
            &m,
            &e,
        );
        assert_eq!(r.evaluation_decision, "adopt");
        assert_eq!(r.improvement_delta, 2);
        assert_eq!(r.failure_code, "tool_failure");
    }

    #[test]
    fn persist_record_writes_expected_runtime_path() {
        let tmp = test_tmp_dir("record-persist");
        let record = StageExperimentRecord {
            run_id: "run-746-a".to_string(),
            workflow_id: "wf-godel".to_string(),
            failure_code: "tool_failure".to_string(),
            hypothesis_id: "hyp:run-746-a:tool_failure".to_string(),
            mutation_id: "mut:run-746-a:tool_failure".to_string(),
            mutation_target_surface: "workflow-step-config".to_string(),
            evaluation_decision: "adopt".to_string(),
            evaluation_rationale: "deterministic rationale".to_string(),
            improvement_delta: 1,
            evidence_refs: vec!["runs/run-746-a/run_status.json".to_string()],
        };

        let rel = persist_record(&tmp, &record).expect("persist");
        assert_eq!(
            rel,
            PathBuf::from("runs/run-746-a/godel/experiment_record.runtime.v1.json")
        );

        let raw = fs::read_to_string(
            tmp.join("run-746-a")
                .join("godel")
                .join("experiment_record.runtime.v1.json"),
        )
        .expect("read file");
        let persisted: PersistedExperimentRecord = serde_json::from_str(&raw).expect("json");
        assert_eq!(persisted.schema, EXPERIMENT_RECORD_RUNTIME_SCHEMA);
        assert_eq!(persisted.record, record);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn persist_record_rejects_unsafe_evidence_ref() {
        let tmp = test_tmp_dir("record-invalid");
        let record = StageExperimentRecord {
            run_id: "run-746-a".to_string(),
            workflow_id: "wf-godel".to_string(),
            failure_code: "tool_failure".to_string(),
            hypothesis_id: "hyp:run-746-a:tool_failure".to_string(),
            mutation_id: "mut:run-746-a:tool_failure".to_string(),
            mutation_target_surface: "workflow-step-config".to_string(),
            evaluation_decision: "adopt".to_string(),
            evaluation_rationale: "deterministic rationale".to_string(),
            improvement_delta: 1,
            evidence_refs: vec!["/Users/daniel/secret.json".to_string()],
        };

        let err = persist_record(&tmp, &record).expect_err("must fail");
        assert!(err.to_string().contains("GODEL_EXPERIMENT_RECORD_INVALID"));
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
