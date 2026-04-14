use std::sync::{Arc, Mutex};

use super::*;

#[derive(Default)]
struct ObsMemInMemory {
    entries: Arc<Mutex<Vec<MemoryWriteRequest>>>,
}

impl ObsMemClient for ObsMemInMemory {
    fn write_entry(
        &self,
        request: &MemoryWriteRequest,
    ) -> Result<MemoryWriteAck, ObsMemContractError> {
        request.validate()?;
        let mut normalized = request.clone();
        normalized.normalize();
        let mut entries = self.entries.lock().expect("lock entries");
        entries.push(normalized.clone());
        entries.sort_by(|a, b| {
            a.run_id
                .cmp(&b.run_id)
                .then_with(|| a.workflow_id.cmp(&b.workflow_id))
                .then_with(|| a.summary.cmp(&b.summary))
        });
        let idx = entries
            .iter()
            .position(|entry| entry == &normalized)
            .expect("entry exists");
        Ok(MemoryWriteAck {
            entry_id: format!("mem-{idx:04}"),
            accepted: true,
        })
    }

    fn query(&self, query: &MemoryQuery) -> Result<MemoryQueryResult, ObsMemContractError> {
        query.validate()?;
        let mut normalized_query = query.clone();
        normalized_query.normalize();

        let entries = self.entries.lock().expect("lock entries");
        let mut hits: Vec<MemoryRecord> = entries
            .iter()
            .filter(|entry| {
                normalized_query
                    .workflow_id
                    .as_ref()
                    .is_none_or(|workflow_id| workflow_id == &entry.workflow_id)
                    && normalized_query
                        .failure_code
                        .as_ref()
                        .is_none_or(|failure_code| {
                            entry.failure_code.as_ref() == Some(failure_code)
                        })
                    && normalized_query
                        .tags
                        .iter()
                        .all(|tag| entry.tags.binary_search(tag).is_ok())
            })
            .map(|entry| MemoryRecord {
                id: format!("{}::{}", entry.run_id, entry.workflow_id),
                run_id: entry.run_id.clone(),
                workflow_id: entry.workflow_id.clone(),
                tags: entry.tags.clone(),
                payload: entry.summary.clone(),
                score: "1.0".to_string(),
                citations: entry.citations.clone(),
                trace_event_refs: entry.trace_event_refs.clone(),
            })
            .collect();

        hits.sort_by(|a, b| {
            b.score
                .cmp(&a.score)
                .then_with(|| a.run_id.cmp(&b.run_id))
                .then_with(|| a.id.cmp(&b.id))
        });
        hits.truncate(normalized_query.limit);
        Ok(MemoryQueryResult { hits })
    }
}

fn sample_request() -> MemoryWriteRequest {
    MemoryWriteRequest {
        contract_version: OBSMEM_CONTRACT_VERSION,
        run_id: "run-001".to_string(),
        workflow_id: "wf-a".to_string(),
        trace_bundle_rel_path: "trace_bundle_v2/manifest.json".to_string(),
        activation_log_rel_path: "trace_bundle_v2/runs/run-001/logs/activation_log.json"
            .to_string(),
        failure_code: Some("TOOL_FAILURE".to_string()),
        summary: "step failed deterministically".to_string(),
        tags: vec!["failure".to_string(), "tool".to_string()],
        citations: vec![MemoryCitation {
            path: "trace_bundle_v2/runs/run-001/steps.json".to_string(),
            hash: "abc123".to_string(),
        }],
        trace_event_refs: vec![MemoryTraceRef {
            event_sequence: 1,
            event_kind: "step_finished".to_string(),
            step_id: Some("s1".to_string()),
            delegation_id: None,
        }],
    }
}

#[test]
fn write_request_normalization_is_deterministic() {
    let mut request = sample_request();
    request.tags = vec!["z".to_string(), "a".to_string(), "a".to_string()];
    request.citations = vec![
        MemoryCitation {
            path: "b".to_string(),
            hash: "2".to_string(),
        },
        MemoryCitation {
            path: "a".to_string(),
            hash: "1".to_string(),
        },
        MemoryCitation {
            path: "a".to_string(),
            hash: "1".to_string(),
        },
    ];

    request.normalize();
    assert_eq!(request.tags, vec!["a", "z"]);
    assert_eq!(request.citations.len(), 2);
    assert_eq!(request.citations[0].path, "a");
    assert_eq!(request.citations[1].path, "b");
}

#[test]
fn write_request_validation_rejects_absolute_and_parent_paths() {
    let mut request = sample_request();
    request.trace_bundle_rel_path = "/Users/runner/leak.json".to_string();
    let err = request.validate().expect_err("absolute path should fail");
    assert_eq!(err.code.as_str(), "OBSMEM_INVALID_REQUEST");

    request.trace_bundle_rel_path = "trace_bundle_v2/manifest.json".to_string();
    request.activation_log_rel_path = "../outside.json".to_string();
    let err = request
        .validate()
        .expect_err("parent traversal path should fail");
    assert_eq!(err.code.as_str(), "OBSMEM_INVALID_REQUEST");
}

#[test]
fn write_request_validation_rejects_version_and_empty_fields() {
    let mut request = sample_request();
    request.contract_version = OBSMEM_CONTRACT_VERSION + 1;
    let err = request
        .validate()
        .expect_err("version mismatch should fail");
    assert_eq!(err.code.as_str(), "OBSMEM_CONTRACT_VERSION_MISMATCH");

    request.contract_version = OBSMEM_CONTRACT_VERSION;
    request.run_id = "   ".to_string();
    let err = request.validate().expect_err("empty run_id should fail");
    assert_eq!(err.code.as_str(), "OBSMEM_INVALID_REQUEST");

    request.run_id = "run-001".to_string();
    request.summary = " ".to_string();
    let err = request.validate().expect_err("empty summary should fail");
    assert_eq!(err.code.as_str(), "OBSMEM_INVALID_REQUEST");
}

#[test]
fn write_request_validation_rejects_citation_and_privacy_violations() {
    let mut request = sample_request();
    request.citations[0].hash = " ".to_string();
    let err = request
        .validate()
        .expect_err("empty citation hash should be invalid");
    assert_eq!(err.code.as_str(), "OBSMEM_INVALID_REQUEST");

    request.citations[0].hash = "abc123".to_string();
    request.summary = "token leak sk-test".to_string();
    let err = request
        .validate()
        .expect_err("token-like content should be blocked");
    assert_eq!(err.code.as_str(), "OBSMEM_PRIVACY_VIOLATION");
}

#[test]
fn query_validation_rejects_invalid_bounds_and_version() {
    let mut query = MemoryQuery {
        contract_version: OBSMEM_CONTRACT_VERSION + 1,
        workflow_id: None,
        failure_code: None,
        tags: vec![],
        limit: 1,
    };
    let err = query.validate().expect_err("version mismatch should fail");
    assert_eq!(err.code.as_str(), "OBSMEM_CONTRACT_VERSION_MISMATCH");

    query.contract_version = OBSMEM_CONTRACT_VERSION;
    query.limit = 0;
    let err = query.validate().expect_err("zero limit should fail");
    assert_eq!(err.code.as_str(), "OBSMEM_INVALID_QUERY");

    query.limit = 1001;
    let err = query.validate().expect_err("oversized limit should fail");
    assert_eq!(err.code.as_str(), "OBSMEM_INVALID_QUERY");
}

#[test]
fn query_normalization_is_deterministic() {
    let mut query = MemoryQuery {
        contract_version: OBSMEM_CONTRACT_VERSION,
        workflow_id: Some("wf-a".to_string()),
        failure_code: None,
        tags: vec!["z".to_string(), "a".to_string(), "a".to_string()],
        limit: 5,
    };

    query.normalize();

    assert_eq!(query.tags, vec!["a", "z"]);
}

#[test]
fn error_code_display_strings_are_stable() {
    assert_eq!(
        ObsMemContractErrorCode::BackendUnavailable.as_str(),
        "OBSMEM_BACKEND_UNAVAILABLE"
    );

    let error = ObsMemContractError::new(
        ObsMemContractErrorCode::InvalidQuery,
        "query limit must be >= 1",
    );
    assert_eq!(
        error.to_string(),
        "OBSMEM_INVALID_QUERY: query limit must be >= 1"
    );
}

#[test]
fn in_memory_client_round_trip_is_deterministic() {
    let client = ObsMemInMemory::default();
    let request = sample_request();

    let ack = client.write_entry(&request).expect("write entry");
    assert!(ack.accepted);

    let query = MemoryQuery {
        contract_version: OBSMEM_CONTRACT_VERSION,
        workflow_id: Some("wf-a".to_string()),
        failure_code: Some("TOOL_FAILURE".to_string()),
        tags: vec!["tool".to_string(), "failure".to_string()],
        limit: 5,
    };

    let first = client.query(&query).expect("query result 1");
    let second = client.query(&query).expect("query result 2");
    assert_eq!(first, second, "query result ordering must be deterministic");
    assert_eq!(first.hits.len(), 1);
    assert_eq!(first.hits[0].run_id, "run-001");
}
