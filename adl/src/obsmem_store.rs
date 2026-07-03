use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::obsmem_contract::{
    MemoryQuery, MemoryQueryResult, MemoryRecord, MemoryTemporalQuery, MemoryWriteAck,
    MemoryWriteRequest, ObsMemClient, ObsMemContractError, ObsMemContractErrorCode,
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
        let mut hits: Vec<MemoryRecord> = if let Some(temporal) = &normalized_query.temporal {
            temporal_index_for(&store.entries)
                .into_iter()
                .filter(|indexed| {
                    base_query_matches(&normalized_query, indexed.entry)
                        && temporal_query_matches(temporal, indexed.entry)
                })
                .map(|indexed| memory_record_from_entry(indexed.entry))
                .collect()
        } else {
            store
                .entries
                .iter()
                .filter(|entry| base_query_matches(&normalized_query, entry))
                .map(memory_record_from_entry)
                .collect()
        };
        if normalized_query.temporal.is_none() {
            hits.sort_by(|a, b| {
                a.id.cmp(&b.id)
                    .then_with(|| a.run_id.cmp(&b.run_id))
                    .then_with(|| a.workflow_id.cmp(&b.workflow_id))
                    .then_with(|| a.payload.cmp(&b.payload))
            });
        }
        hits.truncate(normalized_query.limit);
        Ok(MemoryQueryResult { hits })
    }
}

struct TemporalIndexedEntry<'a> {
    effective_epoch_ms: u128,
    event_sequence: usize,
    id: String,
    entry: &'a MemoryWriteRequest,
}

fn temporal_index_for(entries: &[MemoryWriteRequest]) -> Vec<TemporalIndexedEntry<'_>> {
    let mut indexed: Vec<TemporalIndexedEntry<'_>> = entries
        .iter()
        .filter_map(|entry| {
            let anchor = entry.temporal_anchor.as_ref()?;
            Some(TemporalIndexedEntry {
                effective_epoch_ms: anchor.effective_epoch_ms(),
                event_sequence: anchor.event_sequence.unwrap_or(usize::MAX),
                id: format!("{}::{}", entry.run_id, entry.workflow_id),
                entry,
            })
        })
        .collect();
    indexed.sort_by(|a, b| {
        a.effective_epoch_ms
            .cmp(&b.effective_epoch_ms)
            .then_with(|| a.event_sequence.cmp(&b.event_sequence))
            .then_with(|| a.id.cmp(&b.id))
            .then_with(|| a.entry.run_id.cmp(&b.entry.run_id))
            .then_with(|| a.entry.workflow_id.cmp(&b.entry.workflow_id))
            .then_with(|| a.entry.summary.cmp(&b.entry.summary))
    });
    indexed
}

fn base_query_matches(query: &MemoryQuery, entry: &MemoryWriteRequest) -> bool {
    query
        .workflow_id
        .as_ref()
        .is_none_or(|wid| &entry.workflow_id == wid)
        && query
            .failure_code
            .as_ref()
            .is_none_or(|fc| entry.failure_code.as_ref() == Some(fc))
        && query
            .tags
            .iter()
            .all(|tag| entry.tags.binary_search(tag).is_ok())
}

fn temporal_query_matches(temporal: &MemoryTemporalQuery, entry: &MemoryWriteRequest) -> bool {
    let Some(anchor) = &entry.temporal_anchor else {
        return false;
    };
    let effective = anchor.effective_epoch_ms();
    temporal
        .after_epoch_ms
        .is_none_or(|after| effective >= after)
        && temporal
            .before_epoch_ms
            .is_none_or(|before| effective <= before)
        && temporal
            .interval_start_epoch_ms
            .is_none_or(|start| effective >= start)
        && temporal
            .interval_end_epoch_ms
            .is_none_or(|end| effective <= end)
        && temporal
            .continuity_id
            .as_ref()
            .is_none_or(|id| anchor.continuity_id.as_ref() == Some(id))
        && temporal_staleness_matches(temporal, effective)
}

fn temporal_staleness_matches(temporal: &MemoryTemporalQuery, effective: u128) -> bool {
    match (temporal.stale_at_epoch_ms, temporal.stale_after_ms) {
        (Some(at), Some(after)) => at.saturating_sub(effective) >= after,
        _ => true,
    }
}

fn memory_record_from_entry(entry: &MemoryWriteRequest) -> MemoryRecord {
    MemoryRecord {
        id: format!("{}::{}", entry.run_id, entry.workflow_id),
        run_id: entry.run_id.clone(),
        workflow_id: entry.workflow_id.clone(),
        tags: entry.tags.clone(),
        payload: entry.summary.clone(),
        score: "1.00".to_string(),
        citations: entry.citations.clone(),
        trace_event_refs: entry.trace_event_refs.clone(),
        temporal_anchor: entry.temporal_anchor.clone(),
        review_findings: entry.review_findings.clone(),
        residual_risks: entry.residual_risks.clone(),
        follow_on_refs: entry.follow_on_refs.clone(),
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
            trace_event_refs: vec![crate::obsmem_contract::MemoryTraceRef {
                event_sequence: 0,
                event_kind: "step_finished".to_string(),
                step_id: Some("s1".to_string()),
                delegation_id: None,
            }],
            temporal_anchor: None,
            review_findings: Vec::new(),
            residual_risks: Vec::new(),
            follow_on_refs: Vec::new(),
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
                temporal: None,
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
                temporal: None,
                limit: 1,
            })
            .expect("query");

        assert_eq!(result.hits.len(), 1);
        assert_eq!(result.hits[0].run_id, "run-a");
        assert_eq!(result.hits[0].payload, "alpha");
    }

    #[test]
    fn file_store_temporal_query_filters_and_orders_deterministically() {
        let root = unique_temp_dir("temporal-query");
        let store_path = root.join("_shared/obsmem_store.v1.json");
        let client = FileObsMemClient::new(&store_path);

        for (run_id, t, seq, continuity) in [
            ("run-late", 3000_u128, 3_usize, "chain-a"),
            ("run-early", 1000_u128, 1_usize, "chain-a"),
            ("run-mid", 2000_u128, 2_usize, "chain-a"),
            ("run-other", 1500_u128, 4_usize, "chain-b"),
        ] {
            let mut req = request(run_id, run_id);
            req.temporal_anchor = Some(crate::obsmem_contract::MemoryTemporalAnchor {
                t_created_epoch_ms: t,
                t_observed_epoch_ms: Some(t),
                t_effective_epoch_ms: Some(t),
                continuity_id: Some(continuity.to_string()),
                event_sequence: Some(seq),
            });
            req.normalize();
            client.write_entry(&req).expect("write temporal entry");
        }

        let query = MemoryQuery {
            contract_version: OBSMEM_CONTRACT_VERSION,
            workflow_id: Some("wf-shared".to_string()),
            failure_code: Some("tool_failure".to_string()),
            tags: vec!["status:failed".to_string()],
            temporal: Some(crate::obsmem_contract::MemoryTemporalQuery {
                after_epoch_ms: Some(1000),
                before_epoch_ms: Some(3000),
                interval_start_epoch_ms: Some(1500),
                interval_end_epoch_ms: Some(3000),
                stale_at_epoch_ms: None,
                stale_after_ms: None,
                continuity_id: Some("chain-a".to_string()),
            }),
            limit: 10,
        };

        let first = client.query(&query).expect("first temporal query");
        let second = client.query(&query).expect("second temporal query");
        assert_eq!(first, second);
        assert_eq!(
            first
                .hits
                .iter()
                .map(|hit| hit.run_id.as_str())
                .collect::<Vec<_>>(),
            vec!["run-mid", "run-late"]
        );
    }

    #[test]
    fn file_store_plain_query_preserves_existing_order_with_temporal_anchors() {
        let root = unique_temp_dir("plain-query-temporal-anchors");
        let store_path = root.join("_shared/obsmem_store.v1.json");
        let client = FileObsMemClient::new(&store_path);

        for (run_id, t) in [("run-b", 1_000_u128), ("run-a", 3_000_u128)] {
            let mut req = request(run_id, run_id);
            req.temporal_anchor = Some(crate::obsmem_contract::MemoryTemporalAnchor {
                t_created_epoch_ms: t,
                t_observed_epoch_ms: Some(t),
                t_effective_epoch_ms: Some(t),
                continuity_id: Some("chain-a".to_string()),
                event_sequence: Some(1),
            });
            req.normalize();
            client.write_entry(&req).expect("write anchored entry");
        }

        let result = client
            .query(&MemoryQuery {
                contract_version: OBSMEM_CONTRACT_VERSION,
                workflow_id: Some("wf-shared".to_string()),
                failure_code: Some("tool_failure".to_string()),
                tags: vec!["status:failed".to_string()],
                temporal: None,
                limit: 10,
            })
            .expect("plain query");

        assert_eq!(
            result
                .hits
                .iter()
                .map(|hit| hit.run_id.as_str())
                .collect::<Vec<_>>(),
            vec!["run-a", "run-b"]
        );
    }

    #[test]
    fn file_store_temporal_query_supports_staleness() {
        let root = unique_temp_dir("temporal-stale");
        let store_path = root.join("_shared/obsmem_store.v1.json");
        let client = FileObsMemClient::new(&store_path);

        let mut old = request("run-old", "old");
        old.temporal_anchor = Some(crate::obsmem_contract::MemoryTemporalAnchor {
            t_created_epoch_ms: 1_000,
            t_observed_epoch_ms: Some(1_000),
            t_effective_epoch_ms: Some(1_000),
            continuity_id: Some("chain-a".to_string()),
            event_sequence: Some(1),
        });
        old.normalize();
        client.write_entry(&old).expect("write old");

        let mut fresh = request("run-fresh", "fresh");
        fresh.temporal_anchor = Some(crate::obsmem_contract::MemoryTemporalAnchor {
            t_created_epoch_ms: 9_500,
            t_observed_epoch_ms: Some(9_500),
            t_effective_epoch_ms: Some(9_500),
            continuity_id: Some("chain-a".to_string()),
            event_sequence: Some(2),
        });
        fresh.normalize();
        client.write_entry(&fresh).expect("write fresh");

        let result = client
            .query(&MemoryQuery {
                contract_version: OBSMEM_CONTRACT_VERSION,
                workflow_id: Some("wf-shared".to_string()),
                failure_code: Some("tool_failure".to_string()),
                tags: vec!["status:failed".to_string()],
                temporal: Some(crate::obsmem_contract::MemoryTemporalQuery {
                    stale_at_epoch_ms: Some(10_000),
                    stale_after_ms: Some(5_000),
                    ..Default::default()
                }),
                limit: 10,
            })
            .expect("staleness query");

        assert_eq!(result.hits.len(), 1);
        assert_eq!(result.hits[0].run_id, "run-old");
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
                temporal: None,
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
                temporal: None,
                limit: 10,
            })
            .expect_err("schema mismatch should fail");

        assert_eq!(err.code, ObsMemContractErrorCode::BackendUnavailable);
        assert!(err.message.contains("unsupported ObsMem store schema"));
    }

    #[test]
    fn file_store_round_trips_structured_review_fields() {
        let root = unique_temp_dir("structured-review-fields");
        let store_path = root.join("_shared/obsmem_store.v1.json");
        let client = FileObsMemClient::new(&store_path);

        let mut request = request("run-review", "review-memory");
        request.review_findings = vec![crate::obsmem_contract::MemoryReviewFinding {
            id: "finding-1".to_string(),
            severity: "P2".to_string(),
            summary: "bounded review fact".to_string(),
            disposition: "fixed".to_string(),
        }];
        request.residual_risks = vec!["later release work remains".to_string()];
        request.follow_on_refs = vec![crate::obsmem_contract::MemoryFollowOnRef {
            issue_number: 3356,
            title: "ObsMem transition memory integration".to_string(),
            status: "active".to_string(),
        }];
        request.normalize();

        client.write_entry(&request).expect("write request");
        let result = client
            .query(&MemoryQuery {
                contract_version: OBSMEM_CONTRACT_VERSION,
                workflow_id: Some("wf-shared".to_string()),
                failure_code: Some("tool_failure".to_string()),
                tags: vec![
                    "status:failed".to_string(),
                    "workflow:wf-shared".to_string(),
                ],
                temporal: None,
                limit: 10,
            })
            .expect("query");

        assert_eq!(result.hits.len(), 1);
        assert_eq!(result.hits[0].review_findings, request.review_findings);
        assert_eq!(result.hits[0].residual_risks, request.residual_risks);
        assert_eq!(result.hits[0].follow_on_refs, request.follow_on_refs);
    }
}
