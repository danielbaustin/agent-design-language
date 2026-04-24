//! Observation-memory index contracts for Gödel surface lookup.
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::experiment_record::{PersistedExperimentRecord, StageExperimentRecord};

pub const OBSMEM_INDEX_RUNTIME_SCHEMA: &str = "godel_obsmem_index.runtime.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageIndexEntry {
    pub index_key: String,
    pub run_id: String,
    pub workflow_id: String,
    pub failure_code: String,
    pub hypothesis_id: String,
    pub mutation_id: String,
    pub experiment_outcome: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObsMemIndexQuery {
    pub failure_code: String,
    pub hypothesis_id: Option<String>,
    pub experiment_outcome: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedStageIndexEntry {
    pub schema: String,
    pub entry: StageIndexEntry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObsMemIndexError {
    Invalid(String),
    Io(String),
    Serialize(String),
}

impl std::fmt::Display for ObsMemIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(msg) => write!(f, "GODEL_OBSMEM_INDEX_INVALID: {msg}"),
            Self::Io(msg) => write!(f, "GODEL_OBSMEM_INDEX_IO: {msg}"),
            Self::Serialize(msg) => write!(f, "GODEL_OBSMEM_INDEX_SERIALIZE: {msg}"),
        }
    }
}

impl std::error::Error for ObsMemIndexError {}

pub fn build_index_entry(record: &StageExperimentRecord, failure_code: &str) -> StageIndexEntry {
    StageIndexEntry {
        index_key: build_lookup_key(
            failure_code,
            &record.hypothesis_id,
            &record.evaluation_decision,
        ),
        run_id: record.run_id.clone(),
        workflow_id: record.workflow_id.clone(),
        failure_code: failure_code.to_string(),
        hypothesis_id: record.hypothesis_id.clone(),
        mutation_id: record.mutation_id.clone(),
        experiment_outcome: record.evaluation_decision.clone(),
    }
}

pub fn build_index_entry_from_persisted(
    persisted: &PersistedExperimentRecord,
) -> Result<StageIndexEntry, ObsMemIndexError> {
    if persisted.schema != super::experiment_record::EXPERIMENT_RECORD_RUNTIME_SCHEMA {
        return Err(ObsMemIndexError::Invalid(format!(
            "unexpected experiment record schema: {}",
            persisted.schema
        )));
    }
    Ok(build_index_entry(
        &persisted.record,
        &persisted.record.failure_code,
    ))
}

pub fn build_lookup_key(
    failure_code: &str,
    hypothesis_id: &str,
    experiment_outcome: &str,
) -> String {
    format!("{failure_code}:{hypothesis_id}:{experiment_outcome}")
}

pub fn matches_query(entry: &StageIndexEntry, query: &ObsMemIndexQuery) -> bool {
    if entry.failure_code != query.failure_code {
        return false;
    }
    if let Some(h) = query.hypothesis_id.as_ref() {
        if &entry.hypothesis_id != h {
            return false;
        }
    }
    if let Some(outcome) = query.experiment_outcome.as_ref() {
        if &entry.experiment_outcome != outcome {
            return false;
        }
    }
    true
}

pub fn persist_index_entry(
    runs_root: &Path,
    entry: &StageIndexEntry,
) -> Result<PathBuf, ObsMemIndexError> {
    validate_entry(entry)?;

    let rel_path = PathBuf::from("runs")
        .join(&entry.run_id)
        .join("godel")
        .join("obsmem_index_entry.runtime.v1.json");

    let out_dir = runs_root.join(&entry.run_id).join("godel");
    fs::create_dir_all(&out_dir)
        .map_err(|err| ObsMemIndexError::Io(format!("create dir failed: {err}")))?;

    let persisted = PersistedStageIndexEntry {
        schema: OBSMEM_INDEX_RUNTIME_SCHEMA.to_string(),
        entry: entry.clone(),
    };
    let json = serde_json::to_string_pretty(&persisted)
        .map_err(|err| ObsMemIndexError::Serialize(err.to_string()))?;

    fs::write(out_dir.join("obsmem_index_entry.runtime.v1.json"), json)
        .map_err(|err| ObsMemIndexError::Io(format!("write failed: {err}")))?;

    Ok(rel_path)
}

fn validate_entry(entry: &StageIndexEntry) -> Result<(), ObsMemIndexError> {
    if entry.run_id.trim().is_empty()
        || entry.workflow_id.trim().is_empty()
        || entry.failure_code.trim().is_empty()
        || entry.hypothesis_id.trim().is_empty()
        || entry.experiment_outcome.trim().is_empty()
    {
        return Err(ObsMemIndexError::Invalid(
            "required index fields must be non-empty".to_string(),
        ));
    }

    let scan = format!(
        "{}\n{}\n{}\n{}",
        entry.failure_code, entry.hypothesis_id, entry.experiment_outcome, entry.index_key
    );
    if scan.contains("/Users/")
        || scan.contains("/home/")
        || scan.contains("gho_")
        || scan.contains("sk-")
    {
        return Err(ObsMemIndexError::Invalid(
            "index entry contains disallowed host-path or token-like content".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::godel::experiment_record::{PersistedExperimentRecord, StageExperimentRecord};
    use std::path::PathBuf;

    fn test_tmp_dir(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("adl-godel-{label}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("mkdir test tmp");
        root
    }

    fn record() -> StageExperimentRecord {
        StageExperimentRecord {
            run_id: "run-746-a".to_string(),
            workflow_id: "wf1".to_string(),
            failure_code: "tool_failure".to_string(),
            hypothesis_id: "hyp:run-746-a:tool_failure".to_string(),
            mutation_id: "mut:run-746-a:tool_failure".to_string(),
            mutation_target_surface: "workflow-step-config".to_string(),
            evaluation_decision: "adopt".to_string(),
            evaluation_rationale: "ok".to_string(),
            improvement_delta: 1,
            evidence_refs: vec!["runs/run-746-a/run_status.json".to_string()],
        }
    }

    #[test]
    fn build_index_entry_has_stable_key() {
        let entry = build_index_entry(&record(), "tool_failure");
        assert_eq!(
            entry.index_key,
            "tool_failure:hyp:run-746-a:tool_failure:adopt"
        );
    }

    #[test]
    fn persisted_record_maps_to_index_entry() {
        let persisted = PersistedExperimentRecord {
            schema: super::super::experiment_record::EXPERIMENT_RECORD_RUNTIME_SCHEMA.to_string(),
            record: record(),
        };
        let entry = build_index_entry_from_persisted(&persisted).expect("index entry");
        assert_eq!(entry.failure_code, "tool_failure");
        assert_eq!(entry.experiment_outcome, "adopt");
    }

    #[test]
    fn query_boundary_matches_failure_hypothesis_and_outcome() {
        let entry = build_index_entry(&record(), "tool_failure");
        let query = ObsMemIndexQuery {
            failure_code: "tool_failure".to_string(),
            hypothesis_id: Some("hyp:run-746-a:tool_failure".to_string()),
            experiment_outcome: Some("adopt".to_string()),
        };
        assert!(matches_query(&entry, &query));
    }

    #[test]
    fn display_and_query_mismatch_paths_are_explicit() {
        let entry = build_index_entry(&record(), "tool_failure");
        let no_failure_match = ObsMemIndexQuery {
            failure_code: "verification_failed".to_string(),
            hypothesis_id: None,
            experiment_outcome: None,
        };
        assert!(!matches_query(&entry, &no_failure_match));

        let no_hypothesis_match = ObsMemIndexQuery {
            failure_code: "tool_failure".to_string(),
            hypothesis_id: Some("hyp:other".to_string()),
            experiment_outcome: None,
        };
        assert!(!matches_query(&entry, &no_hypothesis_match));

        let no_outcome_match = ObsMemIndexQuery {
            failure_code: "tool_failure".to_string(),
            hypothesis_id: None,
            experiment_outcome: Some("reject".to_string()),
        };
        assert!(!matches_query(&entry, &no_outcome_match));

        let msg = ObsMemIndexError::Invalid("x".to_string()).to_string();
        assert!(msg.contains("GODEL_OBSMEM_INDEX_INVALID"));
    }

    #[test]
    fn persisted_record_requires_expected_runtime_schema() {
        let persisted = PersistedExperimentRecord {
            schema: "experiment_record.runtime.v0".to_string(),
            record: record(),
        };
        let err =
            build_index_entry_from_persisted(&persisted).expect_err("unexpected schema must fail");
        assert!(err
            .to_string()
            .contains("unexpected experiment record schema"));
    }

    #[test]
    fn persist_index_entry_writes_expected_runtime_path() {
        let tmp = test_tmp_dir("obsmem-index");
        let entry = build_index_entry(&record(), "tool_failure");
        let rel = persist_index_entry(&tmp, &entry).expect("persist");
        assert_eq!(
            rel,
            PathBuf::from("runs/run-746-a/godel/obsmem_index_entry.runtime.v1.json")
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn persist_index_entry_rejects_invalid_fields_and_disallowed_content() {
        let tmp = test_tmp_dir("obsmem-index-invalid");
        let mut entry = build_index_entry(&record(), "tool_failure");
        entry.run_id.clear();
        let err = persist_index_entry(&tmp, &entry).expect_err("empty run_id must fail");
        assert!(err
            .to_string()
            .contains("required index fields must be non-empty"));

        let mut entry = build_index_entry(&record(), "tool_failure");
        entry.index_key = "gho_bad".to_string();
        let err = persist_index_entry(&tmp, &entry).expect_err("token-like content must fail");
        assert!(err
            .to_string()
            .contains("disallowed host-path or token-like content"));
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
