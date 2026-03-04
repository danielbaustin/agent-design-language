use std::fs;
use std::path::Path;

use serde_json::Value as JsonValue;

use crate::obsmem_contract::{
    MemoryCitation, MemoryQuery, MemoryQueryResult, MemoryWriteAck, MemoryWriteRequest,
    ObsMemClient, ObsMemContractError, ObsMemContractErrorCode, OBSMEM_CONTRACT_VERSION,
};

/// Runtime bridge for ObsMem contract operations.
///
/// This adapter keeps runtime integration isolated to trait calls and deterministic
/// request construction from existing ADL run artifacts.
pub struct ObsMemRuntimeAdapter<C: ObsMemClient> {
    client: C,
}

impl<C: ObsMemClient> ObsMemRuntimeAdapter<C> {
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

    /// Execute a structured deterministic query through the contract surface.
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
            limit,
        };
        q.normalize();
        q.validate()?;
        self.client.query(&q)
    }
}

pub fn build_write_request_from_run_artifacts(
    runs_root: &Path,
    run_id: &str,
) -> Result<MemoryWriteRequest, ObsMemContractError> {
    if run_id.trim().is_empty() {
        return Err(ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            "run_id must be non-empty",
        ));
    }

    let run_dir = runs_root.join(run_id);
    let run_summary_path = run_dir.join("run_summary.json");
    let run_status_path = run_dir.join("run_status.json");
    let activation_log_path = run_dir.join("logs").join("activation_log.json");

    let run_summary = read_json(&run_summary_path)?;
    let run_status = read_json(&run_status_path)?;

    let workflow_id = run_summary
        .get("workflow_id")
        .and_then(JsonValue::as_str)
        .ok_or_else(|| {
            ObsMemContractError::new(
                ObsMemContractErrorCode::InvalidRequest,
                "run_summary.json missing workflow_id",
            )
        })?;

    let status = run_status
        .get("overall_status")
        .and_then(JsonValue::as_str)
        .unwrap_or("unknown");
    let failure_code = run_status
        .get("failure_kind")
        .and_then(JsonValue::as_str)
        .map(str::to_string);

    let summary = match failure_code.as_deref() {
        Some(code) => format!(
            "workflow={} overall_status={} failure_kind={}",
            workflow_id, status, code
        ),
        None => format!(
            "workflow={} overall_status={} failure_kind=none",
            workflow_id, status
        ),
    };

    let mut tags = vec![
        format!("workflow:{workflow_id}"),
        format!("status:{status}"),
    ];
    if let Some(code) = failure_code.as_deref() {
        tags.push(format!("failure:{code}"));
    }

    let mut citations = Vec::new();
    citations.push(citation_for_path(
        &run_summary_path,
        format!("runs/{run_id}/run_summary.json"),
    )?);
    citations.push(citation_for_path(
        &run_status_path,
        format!("runs/{run_id}/run_status.json"),
    )?);
    citations.push(citation_for_path(
        &activation_log_path,
        format!("runs/{run_id}/logs/activation_log.json"),
    )?);

    // Contract requires a relative trace-bundle pointer; WP-08 uses a stable
    // contract anchor and WP-09 will wire full bundle indexing flow.
    let mut req = MemoryWriteRequest {
        contract_version: OBSMEM_CONTRACT_VERSION,
        run_id: run_id.to_string(),
        workflow_id: workflow_id.to_string(),
        trace_bundle_rel_path: "trace_bundle_v2/manifest.json".to_string(),
        activation_log_rel_path: format!("runs/{run_id}/logs/activation_log.json"),
        failure_code,
        summary,
        tags,
        citations,
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

fn read_json(path: &Path) -> Result<JsonValue, ObsMemContractError> {
    let raw = fs::read_to_string(path).map_err(|err| {
        ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            format!("failed reading '{}': {err}", path.display()),
        )
    })?;
    serde_json::from_str(&raw).map_err(|err| {
        ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            format!("failed parsing '{}' as json: {err}", path.display()),
        )
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

    #[derive(Clone, Default)]
    struct RecordingClient {
        writes: Arc<Mutex<Vec<MemoryWriteRequest>>>,
        queries: Arc<Mutex<Vec<MemoryQuery>>>,
    }

    impl ObsMemClient for RecordingClient {
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
            Ok(MemoryQueryResult { hits: vec![] })
        }
    }

    fn write_fixture_run(root: &Path, run_id: &str) {
        let run = root.join(run_id);
        std::fs::create_dir_all(run.join("logs")).expect("mkdir logs");
        std::fs::write(
            run.join("run_summary.json"),
            r#"{"run_summary_version":1,"run_id":"r1","workflow_id":"wf-a"}"#,
        )
        .expect("write run_summary");
        std::fs::write(
            run.join("run_status.json"),
            r#"{"run_status_version":1,"overall_status":"failed","failure_kind":"policy_denied"}"#,
        )
        .expect("write run_status");
        std::fs::write(
            run.join("logs").join("activation_log.json"),
            r#"{"activation_log_version":1,"ordering":"append_only_emission_order","stable_ids":{"step_id":"x","delegation_id":"x","run_id":"x"},"events":[]}"#,
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
    fn adapter_calls_contract_client_for_write_and_query() {
        let tmp = unique_temp_dir("roundtrip");
        write_fixture_run(&tmp, "r1");

        let client = RecordingClient::default();
        let adapter = ObsMemRuntimeAdapter::new(client.clone());

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
    fn adapter_rejects_missing_run_artifacts_deterministically() {
        let tmp = unique_temp_dir("missing");
        let err = build_write_request_from_run_artifacts(&tmp, "missing")
            .expect_err("missing run should fail");
        assert_eq!(err.code.as_str(), "OBSMEM_INVALID_REQUEST");
    }
}
