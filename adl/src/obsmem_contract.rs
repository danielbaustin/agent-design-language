use serde::{Deserialize, Serialize};

/// Current ObsMem contract schema version expected by runtime surfaces.
pub const OBSMEM_CONTRACT_VERSION: u32 = 1;

/// Citation to a deterministic artifact path/hash pair that supports replay-safe
/// evidence references.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryCitation {
    /// Repository-relative or run-relative path to a cited artifact.
    pub path: String,
    /// Stable deterministic hash/fingerprint for cited artifact contents.
    pub hash: String,
}

/// Deterministic reference back to the trace event(s) that support a memory
/// record. v0.87 uses event sequence plus bounded identity fields as the
/// "event_id or equivalent" coherence contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryTraceRef {
    pub event_sequence: usize,
    pub event_kind: String,
    pub step_id: Option<String>,
    pub delegation_id: Option<String>,
}

/// Contract payload used to write a normalized run summary into ObsMem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryWriteRequest {
    pub contract_version: u32,
    pub run_id: String,
    pub workflow_id: String,
    pub trace_bundle_rel_path: String,
    pub activation_log_rel_path: String,
    pub failure_code: Option<String>,
    pub summary: String,
    pub tags: Vec<String>,
    pub citations: Vec<MemoryCitation>,
    pub trace_event_refs: Vec<MemoryTraceRef>,
}

impl MemoryWriteRequest {
    /// Canonicalize in-memory ordering for deterministic equality, hashing, and
    /// serialization across runs.
    pub fn normalize(&mut self) {
        self.tags.sort();
        self.tags.dedup();
        self.citations
            .sort_by(|a, b| a.path.cmp(&b.path).then_with(|| a.hash.cmp(&b.hash)));
        self.citations
            .dedup_by(|a, b| a.path == b.path && a.hash == b.hash);
        self.trace_event_refs.sort_by(|a, b| {
            a.event_sequence
                .cmp(&b.event_sequence)
                .then_with(|| a.event_kind.cmp(&b.event_kind))
                .then_with(|| a.step_id.cmp(&b.step_id))
                .then_with(|| a.delegation_id.cmp(&b.delegation_id))
        });
        self.trace_event_refs.dedup_by(|a, b| {
            a.event_sequence == b.event_sequence
                && a.event_kind == b.event_kind
                && a.step_id == b.step_id
                && a.delegation_id == b.delegation_id
        });
    }

    /// Validate request semantics and privacy guards for contract ingestion.
    ///
    /// This is the canonical validation path used by runtime and tests.
    pub fn validate(&self) -> Result<(), ObsMemContractError> {
        if self.contract_version != OBSMEM_CONTRACT_VERSION {
            return Err(ObsMemContractError::new(
                ObsMemContractErrorCode::ContractVersionMismatch,
                format!(
                    "unsupported ObsMem contract version {} (expected {})",
                    self.contract_version, OBSMEM_CONTRACT_VERSION
                ),
            ));
        }
        if self.run_id.trim().is_empty() || self.workflow_id.trim().is_empty() {
            return Err(ObsMemContractError::new(
                ObsMemContractErrorCode::InvalidRequest,
                "run_id and workflow_id must be non-empty",
            ));
        }
        if self.summary.trim().is_empty() {
            return Err(ObsMemContractError::new(
                ObsMemContractErrorCode::InvalidRequest,
                "summary must be non-empty",
            ));
        }

        validate_relative_path(&self.trace_bundle_rel_path)?;
        validate_relative_path(&self.activation_log_rel_path)?;
        for c in &self.citations {
            validate_relative_path(&c.path)?;
            if c.hash.trim().is_empty() {
                return Err(ObsMemContractError::new(
                    ObsMemContractErrorCode::InvalidRequest,
                    "citation hash must be non-empty",
                ));
            }
        }
        for trace_ref in &self.trace_event_refs {
            if trace_ref.event_kind.trim().is_empty() {
                return Err(ObsMemContractError::new(
                    ObsMemContractErrorCode::InvalidRequest,
                    "trace event refs require non-empty event_kind",
                ));
            }
        }

        let text = format!(
            "{}\n{}\n{:?}\n{:?}\n{:?}",
            self.summary,
            self.trace_bundle_rel_path,
            self.tags,
            self.citations,
            self.trace_event_refs
        );
        if text.contains("/Users/")
            || text.contains("/home/")
            || text.contains("gho_")
            || text.contains("sk-")
        {
            return Err(ObsMemContractError::new(
                ObsMemContractErrorCode::PrivacyViolation,
                "memory write request contains disallowed host-path or token-like content",
            ));
        }

        Ok(())
    }
}

/// Acknowledgement returned by a contract write operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryWriteAck {
    pub entry_id: String,
    pub accepted: bool,
}

/// Structured deterministic query surface for v0.75 retrieval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryQuery {
    pub contract_version: u32,
    pub workflow_id: Option<String>,
    pub failure_code: Option<String>,
    pub tags: Vec<String>,
    pub limit: usize,
}

impl MemoryQuery {
    /// Canonicalize query tags for deterministic backend behavior.
    pub fn normalize(&mut self) {
        self.tags.sort();
        self.tags.dedup();
    }

    /// Validate query contract invariants before dispatching to backend clients.
    pub fn validate(&self) -> Result<(), ObsMemContractError> {
        if self.contract_version != OBSMEM_CONTRACT_VERSION {
            return Err(ObsMemContractError::new(
                ObsMemContractErrorCode::ContractVersionMismatch,
                format!(
                    "unsupported ObsMem contract version {} (expected {})",
                    self.contract_version, OBSMEM_CONTRACT_VERSION
                ),
            ));
        }
        if self.limit == 0 {
            return Err(ObsMemContractError::new(
                ObsMemContractErrorCode::InvalidQuery,
                "query limit must be >= 1",
            ));
        }
        if self.limit > 1000 {
            return Err(ObsMemContractError::new(
                ObsMemContractErrorCode::InvalidQuery,
                "query limit must be <= 1000",
            ));
        }
        Ok(())
    }
}

/// Normalized memory hit returned by an ObsMem query.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub id: String,
    pub run_id: String,
    pub workflow_id: String,
    pub tags: Vec<String>,
    pub payload: String,
    pub score: String,
    pub citations: Vec<MemoryCitation>,
    pub trace_event_refs: Vec<MemoryTraceRef>,
}

/// Query response wrapper for deterministic hit lists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryQueryResult {
    pub hits: Vec<MemoryRecord>,
}

/// Stable machine-readable error taxonomy for ObsMem contract boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObsMemContractErrorCode {
    ContractVersionMismatch,
    InvalidRequest,
    InvalidQuery,
    PrivacyViolation,
    BackendUnavailable,
}

impl ObsMemContractErrorCode {
    /// Return the stable wire/log error code string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ContractVersionMismatch => "OBSMEM_CONTRACT_VERSION_MISMATCH",
            Self::InvalidRequest => "OBSMEM_INVALID_REQUEST",
            Self::InvalidQuery => "OBSMEM_INVALID_QUERY",
            Self::PrivacyViolation => "OBSMEM_PRIVACY_VIOLATION",
            Self::BackendUnavailable => "OBSMEM_BACKEND_UNAVAILABLE",
        }
    }
}

/// Structured contract error with stable code plus human-readable message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObsMemContractError {
    pub code: ObsMemContractErrorCode,
    pub message: String,
}

impl ObsMemContractError {
    /// Construct a new contract error.
    pub fn new(code: ObsMemContractErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ObsMemContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code.as_str(), self.message)
    }
}

impl std::error::Error for ObsMemContractError {}

/// ObsMem boundary trait. Runtime code depends on this abstraction only.
///
/// Implementations may wrap local files, an embedded index, or a remote service,
/// but runtime behavior must remain deterministic for identical requests.
pub trait ObsMemClient {
    fn write_entry(
        &self,
        request: &MemoryWriteRequest,
    ) -> Result<MemoryWriteAck, ObsMemContractError>;
    /// v0.75 retrieval boundary: structured deterministic query only.
    ///
    /// Backends may support semantic retrieval internally, but returned records
    /// must be normalized into deterministic ordering for identical query+index
    /// inputs before results are exposed to runtime callers.
    fn query(&self, query: &MemoryQuery) -> Result<MemoryQueryResult, ObsMemContractError>;
}

fn validate_relative_path(path: &str) -> Result<(), ObsMemContractError> {
    if path.trim().is_empty() {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            "relative path must be non-empty",
        ));
    }
    if path.starts_with('/') || path.contains(':') || path.contains('\\') || path.contains("..") {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            "paths must be relative and must not escape",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
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
                .position(|e| e == &normalized)
                .expect("entry exists");
            Ok(MemoryWriteAck {
                entry_id: format!("mem-{idx:04}"),
                accepted: true,
            })
        }

        fn query(&self, query: &MemoryQuery) -> Result<MemoryQueryResult, ObsMemContractError> {
            query.validate()?;
            let mut q = query.clone();
            q.normalize();

            let entries = self.entries.lock().expect("lock entries");
            let mut hits: Vec<MemoryRecord> = entries
                .iter()
                .filter(|e| {
                    q.workflow_id
                        .as_ref()
                        .is_none_or(|wid| wid == &e.workflow_id)
                        && q.failure_code
                            .as_ref()
                            .is_none_or(|fc| e.failure_code.as_ref() == Some(fc))
                        && q.tags.iter().all(|tag| e.tags.binary_search(tag).is_ok())
                })
                .map(|e| MemoryRecord {
                    id: format!("{}::{}", e.run_id, e.workflow_id),
                    run_id: e.run_id.clone(),
                    workflow_id: e.workflow_id.clone(),
                    tags: e.tags.clone(),
                    payload: e.summary.clone(),
                    score: "1.0".to_string(),
                    citations: e.citations.clone(),
                    trace_event_refs: e.trace_event_refs.clone(),
                })
                .collect();

            hits.sort_by(|a, b| {
                b.score
                    .cmp(&a.score)
                    .then_with(|| a.run_id.cmp(&b.run_id))
                    .then_with(|| a.id.cmp(&b.id))
            });
            hits.truncate(q.limit);
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
        let mut req = sample_request();
        req.tags = vec!["z".to_string(), "a".to_string(), "a".to_string()];
        req.citations = vec![
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

        req.normalize();
        assert_eq!(req.tags, vec!["a", "z"]);
        assert_eq!(req.citations.len(), 2);
        assert_eq!(req.citations[0].path, "a");
        assert_eq!(req.citations[1].path, "b");
    }

    #[test]
    fn write_request_validation_rejects_absolute_and_parent_paths() {
        let mut req = sample_request();
        req.trace_bundle_rel_path = "/Users/runner/leak.json".to_string();
        let err = req.validate().expect_err("absolute path should fail");
        assert_eq!(err.code.as_str(), "OBSMEM_INVALID_REQUEST");

        req.trace_bundle_rel_path = "trace_bundle_v2/manifest.json".to_string();
        req.activation_log_rel_path = "../outside.json".to_string();
        let err = req
            .validate()
            .expect_err("parent traversal path should fail");
        assert_eq!(err.code.as_str(), "OBSMEM_INVALID_REQUEST");
    }

    #[test]
    fn write_request_validation_rejects_version_and_empty_fields() {
        let mut req = sample_request();
        req.contract_version = OBSMEM_CONTRACT_VERSION + 1;
        let err = req.validate().expect_err("version mismatch should fail");
        assert_eq!(err.code.as_str(), "OBSMEM_CONTRACT_VERSION_MISMATCH");

        req.contract_version = OBSMEM_CONTRACT_VERSION;
        req.run_id = "   ".to_string();
        let err = req.validate().expect_err("empty run_id should fail");
        assert_eq!(err.code.as_str(), "OBSMEM_INVALID_REQUEST");

        req.run_id = "run-001".to_string();
        req.summary = " ".to_string();
        let err = req.validate().expect_err("empty summary should fail");
        assert_eq!(err.code.as_str(), "OBSMEM_INVALID_REQUEST");
    }

    #[test]
    fn write_request_validation_rejects_citation_and_privacy_violations() {
        let mut req = sample_request();
        req.citations[0].hash = " ".to_string();
        let err = req
            .validate()
            .expect_err("empty citation hash should be invalid");
        assert_eq!(err.code.as_str(), "OBSMEM_INVALID_REQUEST");

        req.citations[0].hash = "abc123".to_string();
        req.summary = "token leak sk-test".to_string();
        let err = req
            .validate()
            .expect_err("token-like content should be blocked");
        assert_eq!(err.code.as_str(), "OBSMEM_PRIVACY_VIOLATION");
    }

    #[test]
    fn query_validation_rejects_invalid_bounds_and_version() {
        let mut q = MemoryQuery {
            contract_version: OBSMEM_CONTRACT_VERSION + 1,
            workflow_id: None,
            failure_code: None,
            tags: vec![],
            limit: 1,
        };
        let err = q.validate().expect_err("version mismatch should fail");
        assert_eq!(err.code.as_str(), "OBSMEM_CONTRACT_VERSION_MISMATCH");

        q.contract_version = OBSMEM_CONTRACT_VERSION;
        q.limit = 0;
        let err = q.validate().expect_err("zero limit should fail");
        assert_eq!(err.code.as_str(), "OBSMEM_INVALID_QUERY");

        q.limit = 1001;
        let err = q.validate().expect_err("oversized limit should fail");
        assert_eq!(err.code.as_str(), "OBSMEM_INVALID_QUERY");
    }

    #[test]
    fn in_memory_client_round_trip_is_deterministic() {
        let client = ObsMemInMemory::default();
        let req = sample_request();

        let ack = client.write_entry(&req).expect("write entry");
        assert!(ack.accepted);

        let query = MemoryQuery {
            contract_version: OBSMEM_CONTRACT_VERSION,
            workflow_id: Some("wf-a".to_string()),
            failure_code: Some("TOOL_FAILURE".to_string()),
            tags: vec!["tool".to_string(), "failure".to_string()],
            limit: 5,
        };

        let r1 = client.query(&query).expect("query result 1");
        let r2 = client.query(&query).expect("query result 2");
        assert_eq!(r1, r2, "query result ordering must be deterministic");
        assert_eq!(r1.hits.len(), 1);
        assert_eq!(r1.hits[0].run_id, "run-001");
    }
}
