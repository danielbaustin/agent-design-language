use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::obsmem_contract::{
    MemoryQuery, MemoryQueryResult, MemoryRecord, MemoryWriteAck, MemoryWriteRequest, ObsMemClient,
    ObsMemContractError, ObsMemContractErrorCode,
};

pub const OBSMEM_STORE_SCHEMA_NAME: &str = "obsmem_store.v1";
pub const OBSMEM_STORE_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct ObsMemStoreFile {
    schema_name: String,
    schema_version: u32,
    entries: Vec<MemoryWriteRequest>,
}

pub struct FileObsMemClient {
    store_path: PathBuf,
}

impl FileObsMemClient {
    pub fn new(store_path: impl Into<PathBuf>) -> Self {
        Self {
            store_path: store_path.into(),
        }
    }

    pub fn store_path(&self) -> &Path {
        &self.store_path
    }

    fn load_store(&self) -> Result<ObsMemStoreFile, ObsMemContractError> {
        if !self.store_path.exists() {
            return Ok(ObsMemStoreFile {
                schema_name: OBSMEM_STORE_SCHEMA_NAME.to_string(),
                schema_version: OBSMEM_STORE_SCHEMA_VERSION,
                entries: Vec::new(),
            });
        }

        let bytes = fs::read(&self.store_path).map_err(|err| {
            ObsMemContractError::new(
                ObsMemContractErrorCode::BackendUnavailable,
                format!(
                    "failed reading ObsMem store '{}': {err}",
                    self.store_path.display()
                ),
            )
        })?;
        let store: ObsMemStoreFile = serde_json::from_slice(&bytes).map_err(|err| {
            ObsMemContractError::new(
                ObsMemContractErrorCode::BackendUnavailable,
                format!(
                    "failed parsing ObsMem store '{}': {err}",
                    self.store_path.display()
                ),
            )
        })?;
        if store.schema_name != OBSMEM_STORE_SCHEMA_NAME
            || store.schema_version != OBSMEM_STORE_SCHEMA_VERSION
        {
            return Err(ObsMemContractError::new(
                ObsMemContractErrorCode::BackendUnavailable,
                format!(
                    "unsupported ObsMem store schema '{}'/{}",
                    store.schema_name, store.schema_version
                ),
            ));
        }
        Ok(store)
    }

    fn write_store(&self, store: &ObsMemStoreFile) -> Result<(), ObsMemContractError> {
        if let Some(parent) = self.store_path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                ObsMemContractError::new(
                    ObsMemContractErrorCode::BackendUnavailable,
                    format!(
                        "failed creating ObsMem store directory '{}': {err}",
                        parent.display()
                    ),
                )
            })?;
        }

        let bytes = serde_json::to_vec_pretty(store).map_err(|err| {
            ObsMemContractError::new(
                ObsMemContractErrorCode::BackendUnavailable,
                format!(
                    "failed serializing ObsMem store '{}': {err}",
                    self.store_path.display()
                ),
            )
        })?;
        fs::write(&self.store_path, bytes).map_err(|err| {
            ObsMemContractError::new(
                ObsMemContractErrorCode::BackendUnavailable,
                format!(
                    "failed writing ObsMem store '{}': {err}",
                    self.store_path.display()
                ),
            )
        })?;
        Ok(())
    }
}

impl ObsMemClient for FileObsMemClient {
    fn write_entry(
        &self,
        request: &MemoryWriteRequest,
    ) -> Result<MemoryWriteAck, ObsMemContractError> {
        request.validate()?;
        let mut normalized = request.clone();
        normalized.normalize();

        let mut store = self.load_store()?;
        if !store.entries.iter().any(|entry| entry == &normalized) {
            store.entries.push(normalized.clone());
        }
        store.entries.sort_by(|a, b| {
            a.run_id
                .cmp(&b.run_id)
                .then_with(|| a.workflow_id.cmp(&b.workflow_id))
                .then_with(|| a.summary.cmp(&b.summary))
        });
        self.write_store(&store)?;

        let idx = store
            .entries
            .iter()
            .position(|entry| entry == &normalized)
            .expect("normalized entry should exist in store");
        Ok(MemoryWriteAck {
            entry_id: format!("mem-{idx:04}"),
            accepted: true,
        })
    }

    fn query(&self, query: &MemoryQuery) -> Result<MemoryQueryResult, ObsMemContractError> {
        let mut normalized_query = query.clone();
        normalized_query.normalize();
        normalized_query.validate()?;

        let store = self.load_store()?;
        let mut hits: Vec<MemoryRecord> = store
            .entries
            .iter()
            .filter(|entry| {
                normalized_query
                    .workflow_id
                    .as_ref()
                    .is_none_or(|wid| &entry.workflow_id == wid)
                    && normalized_query
                        .failure_code
                        .as_ref()
                        .is_none_or(|fc| entry.failure_code.as_ref() == Some(fc))
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
                score: "1.00".to_string(),
                citations: entry.citations.clone(),
            })
            .collect();
        hits.sort_by(|a, b| {
            a.id.cmp(&b.id)
                .then_with(|| a.run_id.cmp(&b.run_id))
                .then_with(|| a.workflow_id.cmp(&b.workflow_id))
                .then_with(|| a.payload.cmp(&b.payload))
        });
        hits.truncate(normalized_query.limit);
        Ok(MemoryQueryResult { hits })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;
    use crate::obsmem_contract::{MemoryCitation, OBSMEM_CONTRACT_VERSION};

    fn unique_temp_dir(label: &str) -> PathBuf {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "adl-obsmem-store-{label}-pid{}-{n}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn request(run_id: &str, summary: &str) -> MemoryWriteRequest {
        let mut request = MemoryWriteRequest {
            contract_version: OBSMEM_CONTRACT_VERSION,
            run_id: run_id.to_string(),
            workflow_id: "wf-shared".to_string(),
            trace_bundle_rel_path: "trace_bundle_v2/manifest.json".to_string(),
            activation_log_rel_path: format!("runs/{run_id}/logs/activation_log.json"),
            failure_code: Some("tool_failure".to_string()),
            summary: summary.to_string(),
            tags: vec![
                "status:failed".to_string(),
                "workflow:wf-shared".to_string(),
            ],
            citations: vec![MemoryCitation {
                path: format!("runs/{run_id}/run_summary.json"),
                hash: "det64:0000000000000001".to_string(),
            }],
        };
        request.normalize();
        request
    }

    #[test]
    fn file_store_persists_entries_across_client_instances() {
        let root = unique_temp_dir("shared");
        let store_path = root.join("_shared/obsmem_store.v1.json");
        let client_a = FileObsMemClient::new(&store_path);
        let client_b = FileObsMemClient::new(&store_path);

        client_a
            .write_entry(&request("run-a", "first"))
            .expect("write a");
        client_b
            .write_entry(&request("run-b", "second"))
            .expect("write b");

        let result = client_a
            .query(&MemoryQuery {
                contract_version: OBSMEM_CONTRACT_VERSION,
                workflow_id: Some("wf-shared".to_string()),
                failure_code: Some("tool_failure".to_string()),
                tags: vec![
                    "status:failed".to_string(),
                    "workflow:wf-shared".to_string(),
                ],
                limit: 10,
            })
            .expect("query");
        assert_eq!(result.hits.len(), 2);
        assert_eq!(result.hits[0].run_id, "run-a");
        assert_eq!(result.hits[1].run_id, "run-b");
    }

    #[test]
    fn file_store_dedupes_identical_entries_and_keeps_stable_ack() {
        let root = unique_temp_dir("dedupe");
        let store_path = root.join("_shared/obsmem_store.v1.json");
        let client = FileObsMemClient::new(&store_path);
        let request = request("run-a", "same");

        let first = client.write_entry(&request).expect("first");
        let second = client.write_entry(&request).expect("second");

        assert_eq!(first, second);
        let bytes = fs::read(store_path).expect("read store");
        let store: ObsMemStoreFile = serde_json::from_slice(&bytes).expect("parse store");
        assert_eq!(store.entries.len(), 1);
    }

    #[test]
    fn file_store_query_filters_by_workflow_failure_and_tags_and_truncates() {
        let root = unique_temp_dir("query-filter");
        let store_path = root.join("_shared/obsmem_store.v1.json");
        let client = FileObsMemClient::new(&store_path);

        let mut same_workflow = request("run-a", "alpha");
        same_workflow.tags.push("topic:memory".to_string());
        same_workflow.normalize();
        client.write_entry(&same_workflow).expect("write alpha");

        let mut same_workflow_2 = request("run-b", "beta");
        same_workflow_2.tags.push("topic:memory".to_string());
        same_workflow_2.normalize();
        client.write_entry(&same_workflow_2).expect("write beta");

        let mut different_failure = request("run-c", "gamma");
        different_failure.failure_code = Some("other_failure".to_string());
        different_failure.normalize();
        client.write_entry(&different_failure).expect("write gamma");

        let mut different_workflow = request("run-d", "delta");
        different_workflow.workflow_id = "wf-other".to_string();
        different_workflow.normalize();
        client
            .write_entry(&different_workflow)
            .expect("write delta");

        let result = client
            .query(&MemoryQuery {
                contract_version: OBSMEM_CONTRACT_VERSION,
                workflow_id: Some("wf-shared".to_string()),
                failure_code: Some("tool_failure".to_string()),
                tags: vec![
                    "status:failed".to_string(),
                    "workflow:wf-shared".to_string(),
                    "topic:memory".to_string(),
                ],
                limit: 1,
            })
            .expect("query");

        assert_eq!(result.hits.len(), 1);
        assert_eq!(result.hits[0].run_id, "run-a");
        assert_eq!(result.hits[0].payload, "alpha");
    }

    #[test]
    fn file_store_rejects_malformed_json_store() {
        let root = unique_temp_dir("malformed");
        let store_path = root.join("_shared/obsmem_store.v1.json");
        fs::create_dir_all(store_path.parent().expect("parent")).expect("mkdir");
        fs::write(&store_path, b"{not-json").expect("write malformed");

        let client = FileObsMemClient::new(&store_path);
        let err = client
            .query(&MemoryQuery {
                contract_version: OBSMEM_CONTRACT_VERSION,
                workflow_id: None,
                failure_code: None,
                tags: Vec::new(),
                limit: 10,
            })
            .expect_err("malformed store should fail");

        assert_eq!(err.code, ObsMemContractErrorCode::BackendUnavailable);
        assert!(err.message.contains("failed parsing ObsMem store"));
    }

    #[test]
    fn file_store_rejects_unsupported_schema_version() {
        let root = unique_temp_dir("schema-mismatch");
        let store_path = root.join("_shared/obsmem_store.v1.json");
        fs::create_dir_all(store_path.parent().expect("parent")).expect("mkdir");
        let raw = serde_json::to_vec_pretty(&serde_json::json!({
            "schema_name": OBSMEM_STORE_SCHEMA_NAME,
            "schema_version": 999,
            "entries": [],
        }))
        .expect("serialize");
        fs::write(&store_path, raw).expect("write schema mismatch");

        let client = FileObsMemClient::new(&store_path);
        let err = client
            .query(&MemoryQuery {
                contract_version: OBSMEM_CONTRACT_VERSION,
                workflow_id: None,
                failure_code: None,
                tags: Vec::new(),
                limit: 10,
            })
            .expect_err("schema mismatch should fail");

        assert_eq!(err.code, ObsMemContractErrorCode::BackendUnavailable);
        assert!(err.message.contains("unsupported ObsMem store schema"));
    }
}
