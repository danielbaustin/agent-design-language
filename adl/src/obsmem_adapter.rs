use std::fs;
use std::path::Path;

use crate::obsmem_indexing::index_run_from_artifacts;
use crate::obsmem_transition_memory::build_write_request_from_transition_handoff;

use crate::obsmem_contract::{
    MemoryCitation, MemoryQuery, MemoryQueryResult, MemoryTemporalQuery, MemoryWriteAck,
    MemoryWriteRequest, ObsMemClient, ObsMemContractError, ObsMemContractErrorCode,
    OBSMEM_CONTRACT_VERSION,
};
use crate::obsmem_retrieval_policy::{
    apply_policy_to_results, RetrievalPolicyV1, RetrievalRequest,
};

/// Runtime bridge for ObsMem contract operations.
///
/// This adapter keeps runtime integration isolated to trait calls and deterministic
/// request construction from existing ADL run artifacts.
pub struct ObsMemAdapter<C: ObsMemClient> {
    client: C,
}

impl<C: ObsMemClient> ObsMemAdapter<C> {
    /// Construct a new adapter over a concrete [`ObsMemClient`] implementation.
    pub fn new(client: C) -> Self {
        Self { client }
    }

    /// Build a deterministic memory write request from persisted run artifacts
    /// and forward it to the configured ObsMem client.
    pub fn index_run_from_artifacts(
        &self,
        runs_root: &Path,
        run_id: &str,
    ) -> Result<MemoryWriteAck, ObsMemContractError> {
        let request = build_write_request_from_run_artifacts(runs_root, run_id)?;
        self.client.write_entry(&request)
    }

    /// Build a deterministic memory write request from a tracked C-SDLC
    /// transition handoff packet and forward it to the configured ObsMem
    /// client.
    pub fn index_transition_handoff(
        &self,
        repo_root: &Path,
        handoff_path: &Path,
    ) -> Result<MemoryWriteAck, ObsMemContractError> {
        let request = build_write_request_from_transition_handoff(repo_root, handoff_path)?;
        self.client.write_entry(&request)
    }

    /// Execute deterministic structured retrieval through the contract surface.
    ///
    /// v0.75 boundary: this query path must return the same records in the same
    /// order for identical query filters/tags/limit and identical backend state.
    pub fn query(
        &self,
        workflow_id: Option<&str>,
        failure_code: Option<&str>,
        tags: &[String],
        limit: usize,
    ) -> Result<MemoryQueryResult, ObsMemContractError> {
        let mut q = MemoryQuery {
            contract_version: OBSMEM_CONTRACT_VERSION,
            workflow_id: workflow_id.map(str::to_string),
            failure_code: failure_code.map(str::to_string),
            tags: tags.to_vec(),
            temporal: None,
            limit,
        };
        q.normalize();
        q.validate()?;
        self.client.query(&q)
    }

    pub fn query_with_policy(
        &self,
        policy: &RetrievalPolicyV1,
        request: &RetrievalRequest,
    ) -> Result<MemoryQueryResult, ObsMemContractError> {
        let query = request.to_query(policy)?;
        let result = self.client.query(&query)?;
        apply_policy_to_results(policy, request, result)
    }

    /// Execute deterministic temporal retrieval over ObsMem / Memory Palace
    /// records using Chronosense-compatible anchors.
    pub fn query_temporal(
        &self,
        workflow_id: Option<&str>,
        failure_code: Option<&str>,
        tags: &[String],
        temporal: MemoryTemporalQuery,
        limit: usize,
    ) -> Result<MemoryQueryResult, ObsMemContractError> {
        let mut q = MemoryQuery {
            contract_version: OBSMEM_CONTRACT_VERSION,
            workflow_id: workflow_id.map(str::to_string),
            failure_code: failure_code.map(str::to_string),
            tags: tags.to_vec(),
            temporal: Some(temporal),
            limit,
        };
        q.normalize();
        q.validate()?;
        self.client.query(&q)
    }
}

/// Build a validated deterministic write request from persisted ADL run
/// artifacts under `runs_root/<run_id>/...`.
///
/// Errors are surfaced as contract errors with stable error codes.
pub fn build_write_request_from_run_artifacts(
    runs_root: &Path,
    run_id: &str,
) -> Result<MemoryWriteRequest, ObsMemContractError> {
    let safe_run_id = crate::artifacts::validate_run_id_path_segment(run_id).map_err(|err| {
        ObsMemContractError::new(ObsMemContractErrorCode::InvalidRequest, err.to_string())
    })?;

    let run_dir = runs_root.join(&safe_run_id);
    let run_summary_path = run_dir.join("run_summary.json");
    let run_status_path = run_dir.join("run_status.json");
    let activation_log_path = run_dir.join("logs").join("activation_log.json");
    let indexed = index_run_from_artifacts(runs_root, &safe_run_id)?;

    let workflow_id = indexed.workflow_id;
    let failure_code = indexed.failure_code;
    let summary = indexed.summary;
    let tags = indexed.tags;
    let trace_event_refs = indexed.trace_event_refs;

    let mut citations = Vec::new();
    citations.push(citation_for_path(
        &run_summary_path,
        format!("runs/{safe_run_id}/run_summary.json"),
    )?);
    citations.push(citation_for_path(
        &run_status_path,
        format!("runs/{safe_run_id}/run_status.json"),
    )?);
    citations.push(citation_for_path(
        &activation_log_path,
        format!("runs/{safe_run_id}/logs/activation_log.json"),
    )?);

    // Contract requires a relative trace-bundle pointer; WP-08 uses a stable
    // contract anchor and WP-09 will wire full bundle indexing flow.
    let mut req = MemoryWriteRequest {
        contract_version: OBSMEM_CONTRACT_VERSION,
        run_id: safe_run_id.clone(),
        workflow_id: workflow_id.to_string(),
        trace_bundle_rel_path: "trace_bundle_v2/manifest.json".to_string(),
        activation_log_rel_path: format!("runs/{safe_run_id}/logs/activation_log.json"),
        failure_code,
        summary,
        tags,
        citations,
        trace_event_refs,
        temporal_anchor: None,
        review_findings: Vec::new(),
        residual_risks: Vec::new(),
        follow_on_refs: Vec::new(),
    };

    req.normalize();
    req.validate()?;
    Ok(req)
}

fn citation_for_path(path: &Path, rel_path: String) -> Result<MemoryCitation, ObsMemContractError> {
    let bytes = fs::read(path).map_err(|err| {
        ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            format!("failed reading citation source '{}': {err}", path.display()),
        )
    })?;
    Ok(MemoryCitation {
        path: rel_path,
        hash: stable_fingerprint_hex(&bytes),
    })
}

fn stable_fingerprint_hex(bytes: &[u8]) -> String {
    // Deterministic lightweight fingerprint for adapter-side citations.
    // WP-09 will align full index pipeline hashing with trace-bundle surfaces.
    let mut acc: u64 = 0xcbf2_9ce4_8422_2325;
    for (idx, b) in bytes.iter().enumerate() {
        acc ^= (*b as u64).wrapping_add(idx as u64);
        acc = acc.wrapping_mul(0x100_0000_01b3);
    }
    format!("det64:{acc:016x}")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::obsmem_contract::{
        MemoryRecord, MemoryTemporalAnchor, MemoryTraceRef, OBSMEM_CONTRACT_VERSION,
    };
    use crate::obsmem_store::FileObsMemClient;

    #[derive(Clone, Default)]
    struct ObsMemInMemory {
        writes: Arc<Mutex<Vec<MemoryWriteRequest>>>,
        queries: Arc<Mutex<Vec<MemoryQuery>>>,
    }

    impl ObsMemClient for ObsMemInMemory {
        fn write_entry(
            &self,
            request: &MemoryWriteRequest,
        ) -> Result<MemoryWriteAck, ObsMemContractError> {
            self.writes
                .lock()
                .expect("writes lock")
                .push(request.clone());
            Ok(MemoryWriteAck {
                entry_id: "mem-0000".to_string(),
                accepted: true,
            })
        }

        fn query(&self, query: &MemoryQuery) -> Result<MemoryQueryResult, ObsMemContractError> {
            self.queries
                .lock()
                .expect("queries lock")
                .push(query.clone());
            let writes = self.writes.lock().expect("writes lock");
            let mut hits: Vec<MemoryRecord> = writes
                .iter()
                .filter(|e| {
                    query
                        .workflow_id
                        .as_ref()
                        .is_none_or(|wid| &e.workflow_id == wid)
                        && query
                            .failure_code
                            .as_ref()
                            .is_none_or(|fc| e.failure_code.as_ref() == Some(fc))
                        && query.tags.iter().all(|t| e.tags.binary_search(t).is_ok())
                        && query
                            .temporal
                            .as_ref()
                            .is_none_or(|temporal| temporal_matches(temporal, e))
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
                    temporal_anchor: e.temporal_anchor.clone(),
                    review_findings: e.review_findings.clone(),
                    residual_risks: e.residual_risks.clone(),
                    follow_on_refs: e.follow_on_refs.clone(),
                })
                .collect();
            hits.sort_by(|a, b| a.id.cmp(&b.id));
            hits.truncate(query.limit);
            Ok(MemoryQueryResult { hits })
        }
    }

    fn temporal_matches(temporal: &MemoryTemporalQuery, entry: &MemoryWriteRequest) -> bool {
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
            && match (temporal.stale_at_epoch_ms, temporal.stale_after_ms) {
                (Some(at), Some(after)) => at.saturating_sub(effective) >= after,
                _ => true,
            }
    }

    fn write_fixture_run(root: &Path, run_id: &str) {
        write_fixture_run_with_status(root, run_id, "failed");
    }

    fn write_fixture_run_with_status(root: &Path, run_id: &str, overall_status: &str) {
        let run = root.join(run_id);
        std::fs::create_dir_all(run.join("logs")).expect("mkdir logs");
        std::fs::write(
            run.join("run_summary.json"),
            r#"{"run_summary_version":1,"run_id":"r1","workflow_id":"wf-a"}"#,
        )
        .expect("write run_summary");
        std::fs::write(
            run.join("run_status.json"),
            format!(
                r#"{{"run_status_version":1,"overall_status":"{overall_status}","failure_kind":"policy_denied"}}"#
            ),
        )
        .expect("write run_status");
        std::fs::write(
            run.join("logs").join("activation_log.json"),
            r#"{"activation_log_version":1,"ordering":"append_only_emission_order","stable_ids":{"step_id":"x","delegation_id":"x","run_id":"x"},"events":[{"kind":"StepStarted","step_id":"s1","agent_id":"a","provider_id":"local","task_id":"t","delegation_json":null},{"kind":"StepFinished","step_id":"s1","success":true}]}"#,
        )
        .expect("write activation log");
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let dir =
            std::env::temp_dir().join(format!("adl-obsmem-{label}-pid{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn temporal_write_request(
        run_id: &str,
        effective_epoch_ms: u128,
        event_sequence: usize,
        continuity_id: &str,
    ) -> MemoryWriteRequest {
        let mut request = MemoryWriteRequest {
            contract_version: OBSMEM_CONTRACT_VERSION,
            run_id: run_id.to_string(),
            workflow_id: "wf-memory-palace".to_string(),
            trace_bundle_rel_path: "trace_bundle_v2/manifest.json".to_string(),
            activation_log_rel_path: format!("runs/{run_id}/logs/activation_log.json"),
            failure_code: Some("temporal_lookup".to_string()),
            summary: format!("temporal memory {run_id}"),
            tags: vec![
                "status:known".to_string(),
                "workflow:wf-memory-palace".to_string(),
            ],
            citations: vec![MemoryCitation {
                path: format!("runs/{run_id}/run_summary.json"),
                hash: "det64:0000000000000042".to_string(),
            }],
            trace_event_refs: vec![MemoryTraceRef {
                event_sequence,
                event_kind: "memory_observed".to_string(),
                step_id: Some("memory-palace".to_string()),
                delegation_id: None,
            }],
            temporal_anchor: Some(MemoryTemporalAnchor {
                t_created_epoch_ms: effective_epoch_ms,
                t_observed_epoch_ms: Some(effective_epoch_ms),
                t_effective_epoch_ms: Some(effective_epoch_ms),
                continuity_id: Some(continuity_id.to_string()),
                event_sequence: Some(event_sequence),
            }),
            review_findings: Vec::new(),
            residual_risks: Vec::new(),
            follow_on_refs: Vec::new(),
        };
        request.normalize();
        request
    }

    #[test]
    fn build_write_request_from_artifacts_is_deterministic() {
        let tmp = unique_temp_dir("deterministic");
        write_fixture_run(&tmp, "r1");

        let a = build_write_request_from_run_artifacts(&tmp, "r1").expect("request a");
        let b = build_write_request_from_run_artifacts(&tmp, "r1").expect("request b");

        assert_eq!(a, b);
        assert_eq!(a.workflow_id, "wf-a");
        assert_eq!(a.failure_code.as_deref(), Some("policy_denied"));
        assert_eq!(
            a.activation_log_rel_path,
            "runs/r1/logs/activation_log.json".to_string()
        );
    }

    #[test]
    fn build_write_request_from_artifacts_rejects_unsafe_run_id_path_segments() {
        let tmp = unique_temp_dir("unsafe-run-id");
        let err = build_write_request_from_run_artifacts(&tmp, "../escape")
            .expect_err("unsafe run_id must fail");
        assert!(err.message.contains("safe path segment"));
    }

    #[test]
    fn adapter_calls_contract_client_for_write_and_query() {
        let tmp = unique_temp_dir("roundtrip");
        write_fixture_run(&tmp, "r1");

        let client = ObsMemInMemory::default();
        let adapter = ObsMemAdapter::new(client.clone());

        let ack = adapter
            .index_run_from_artifacts(&tmp, "r1")
            .expect("index run");
        assert!(ack.accepted);

        adapter
            .query(
                Some("wf-a"),
                Some("policy_denied"),
                &["status:failed".to_string(), "workflow:wf-a".to_string()],
                5,
            )
            .expect("query");

        let writes = client.writes.lock().expect("writes lock");
        let queries = client.queries.lock().expect("queries lock");
        assert_eq!(writes.len(), 1);
        assert_eq!(queries.len(), 1);
        assert_eq!(queries[0].tags, vec!["status:failed", "workflow:wf-a"]);
    }

    #[test]
    fn same_query_returns_same_records() {
        let tmp = unique_temp_dir("same-records");
        write_fixture_run(&tmp, "r1");
        write_fixture_run(&tmp, "r2");

        let client = ObsMemInMemory::default();
        let adapter = ObsMemAdapter::new(client);
        adapter
            .index_run_from_artifacts(&tmp, "r1")
            .expect("index r1");
        adapter
            .index_run_from_artifacts(&tmp, "r2")
            .expect("index r2");

        let qtags = vec!["status:failed".to_string()];
        let a = adapter
            .query(None, Some("policy_denied"), &qtags, 10)
            .expect("query a");
        let b = adapter
            .query(None, Some("policy_denied"), &qtags, 10)
            .expect("query b");

        let ids_a: Vec<String> = a.hits.iter().map(|h| h.id.clone()).collect();
        let ids_b: Vec<String> = b.hits.iter().map(|h| h.id.clone()).collect();
        assert_eq!(ids_a, ids_b);
    }

    #[test]
    fn same_query_returns_same_order() {
        let tmp = unique_temp_dir("same-order");
        write_fixture_run(&tmp, "r3");
        write_fixture_run(&tmp, "r1");
        write_fixture_run(&tmp, "r2");

        let client = ObsMemInMemory::default();
        let adapter = ObsMemAdapter::new(client);
        for run_id in ["r3", "r1", "r2"] {
            adapter
                .index_run_from_artifacts(&tmp, run_id)
                .expect("index run");
        }

        let qtags = vec!["status:failed".to_string()];
        let first = adapter
            .query(None, Some("policy_denied"), &qtags, 10)
            .expect("query first");
        let second = adapter
            .query(None, Some("policy_denied"), &qtags, 10)
            .expect("query second");

        let first_order: Vec<String> = first.hits.iter().map(|h| h.id.clone()).collect();
        let second_order: Vec<String> = second.hits.iter().map(|h| h.id.clone()).collect();
        assert_eq!(first_order, second_order);
    }

    #[test]
    fn query_with_policy_applies_default_limit_and_required_failure_tag() {
        let tmp = unique_temp_dir("policy");
        write_fixture_run(&tmp, "r1");

        let client = ObsMemInMemory::default();
        let adapter = ObsMemAdapter::new(client);
        adapter
            .index_run_from_artifacts(&tmp, "r1")
            .expect("index run");

        let mut policy = RetrievalPolicyV1 {
            default_limit: 1,
            required_tags: vec!["status:failed".to_string()],
            required_failure_code: Some("policy_denied".to_string()),
            order: crate::obsmem_retrieval_policy::RetrievalOrder::IdAsc,
        };
        policy.normalize();

        let request = RetrievalRequest {
            workflow_id: Some("wf-a".to_string()),
            failure_code: None,
            tags: vec!["workflow:wf-a".to_string()],
            limit_override: None,
        };

        let result = adapter
            .query_with_policy(&policy, &request)
            .expect("query with policy");
        assert_eq!(result.hits.len(), 1);
        assert_eq!(result.hits[0].workflow_id, "wf-a");
    }

    #[test]
    fn query_with_policy_supports_evidence_adjusted_order() {
        let tmp = unique_temp_dir("evidence-order");
        write_fixture_run_with_status(&tmp, "r-success", "success");
        write_fixture_run_with_status(&tmp, "r-failed", "failed");

        let client = ObsMemInMemory::default();
        let adapter = ObsMemAdapter::new(client);
        adapter
            .index_run_from_artifacts(&tmp, "r-success")
            .expect("index r-success");
        adapter
            .index_run_from_artifacts(&tmp, "r-failed")
            .expect("index r-failed");

        let mut policy = RetrievalPolicyV1 {
            default_limit: 10,
            required_tags: vec!["workflow:wf-a".to_string()],
            required_failure_code: Some("policy_denied".to_string()),
            order: crate::obsmem_retrieval_policy::RetrievalOrder::EvidenceAdjustedDescIdAsc,
        };
        policy.normalize();

        let request = RetrievalRequest {
            workflow_id: Some("wf-a".to_string()),
            failure_code: None,
            tags: vec![],
            limit_override: None,
        };

        let result = adapter
            .query_with_policy(&policy, &request)
            .expect("query with evidence-adjusted policy");
        let ids: Vec<&str> = result.hits.iter().map(|h| h.run_id.as_str()).collect();
        assert_eq!(ids, vec!["r-success", "r-failed"]);
    }

    #[test]
    fn adapter_query_temporal_uses_file_store_temporal_index() {
        let tmp = unique_temp_dir("temporal-adapter");
        let store_path = tmp.join("_shared/obsmem_store.v1.json");
        let writer = FileObsMemClient::new(&store_path);
        for (run_id, effective_epoch_ms, event_sequence, continuity_id) in [
            ("run-late", 3_000_u128, 3_usize, "chain-a"),
            ("run-early", 1_000_u128, 1_usize, "chain-a"),
            ("run-mid", 2_000_u128, 2_usize, "chain-a"),
            ("run-other", 1_500_u128, 4_usize, "chain-b"),
        ] {
            writer
                .write_entry(&temporal_write_request(
                    run_id,
                    effective_epoch_ms,
                    event_sequence,
                    continuity_id,
                ))
                .expect("write temporal entry");
        }

        let adapter = ObsMemAdapter::new(FileObsMemClient::new(&store_path));
        let temporal = MemoryTemporalQuery {
            interval_start_epoch_ms: Some(1_500),
            interval_end_epoch_ms: Some(3_000),
            continuity_id: Some("chain-a".to_string()),
            ..Default::default()
        };
        let first = adapter
            .query_temporal(
                Some("wf-memory-palace"),
                Some("temporal_lookup"),
                &["status:known".to_string()],
                temporal.clone(),
                10,
            )
            .expect("first temporal adapter query");
        let second = adapter
            .query_temporal(
                Some("wf-memory-palace"),
                Some("temporal_lookup"),
                &["status:known".to_string()],
                temporal,
                10,
            )
            .expect("second temporal adapter query");

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
    fn adapter_rejects_missing_run_artifacts_deterministically() {
        let tmp = unique_temp_dir("missing");
        let err = build_write_request_from_run_artifacts(&tmp, "missing")
            .expect_err("missing run should fail");
        assert_eq!(err.code.as_str(), "OBSMEM_INVALID_REQUEST");
    }
}
