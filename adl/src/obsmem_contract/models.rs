use serde::{Deserialize, Serialize};

use super::validation::{contains_disallowed_content, validate_relative_path};
use super::{ObsMemContractError, ObsMemContractErrorCode};

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
        for citation in &self.citations {
            validate_relative_path(&citation.path)?;
            if citation.hash.trim().is_empty() {
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
        if contains_disallowed_content(&text) {
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
