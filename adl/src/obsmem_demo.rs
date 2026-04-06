use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::artifacts::{self, atomic_write, RunArtifactPaths};
use crate::obsmem_adapter::ObsMemAdapter;
use crate::obsmem_contract::{MemoryRecord, MemoryWriteAck};
use crate::obsmem_indexing::index_run_from_artifacts;
use crate::obsmem_retrieval_policy::{RetrievalOrder, RetrievalPolicyV1, RetrievalRequest};
use crate::obsmem_store::FileObsMemClient;

pub const OBS_MEM_INDEX_SUMMARY_VERSION: u32 = 1;
pub const OBS_MEM_QUERY_RESULT_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ObsMemDemoArtifacts {
    pub shared_store: PathBuf,
    pub index_summary: PathBuf,
    pub query_result: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct ObsMemIndexSummaryArtifact {
    index_summary_version: u32,
    run_id: String,
    workflow_id: String,
    indexed_entry_count: usize,
    step_context_count: usize,
    trace_event_ref_count: usize,
    trace_event_refs: Vec<crate::obsmem_contract::MemoryTraceRef>,
    source_artifacts: Vec<String>,
    tags: Vec<String>,
    write_ack: MemoryWriteAck,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct ObsMemQueryDescriptor {
    workflow_id: Option<String>,
    failure_code: Option<String>,
    tags: Vec<String>,
    limit: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct ObsMemOrderingDescriptor {
    policy_order: String,
    tie_break_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct ObsMemQueryResultArtifact {
    query_result_version: u32,
    run_id: String,
    workflow_id: String,
    query: ObsMemQueryDescriptor,
    ordering: ObsMemOrderingDescriptor,
    returned_count: usize,
    entries: Vec<MemoryRecord>,
}

pub fn maybe_emit_obsmem_demo_artifacts(run_id: &str) -> Result<Option<ObsMemDemoArtifacts>> {
    // ObsMem demo integration pipeline (env-gated).
    // Enabled only when ADL_OBSMEM_DEMO=1 to keep default runtime behavior unchanged.
    // Demonstrates deterministic indexing + retrieval through a demo adapter.
    match std::env::var("ADL_OBSMEM_DEMO") {
        Ok(v) if v.trim() == "1" => {
            let runs_root = artifacts::runs_root()?;
            let artifacts = emit_obsmem_demo_artifacts(&runs_root, run_id)?;
            Ok(Some(artifacts))
        }
        _ => Ok(None),
    }
}

pub fn emit_obsmem_demo_artifacts(runs_root: &Path, run_id: &str) -> Result<ObsMemDemoArtifacts> {
    let indexed = index_run_from_artifacts(runs_root, run_id)
        .with_context(|| format!("index run artifacts for '{run_id}'"))?;

    let shared_store_path = runs_root.join("_shared").join("obsmem_store.v1.json");
    let client = FileObsMemClient::new(&shared_store_path);
    let adapter = ObsMemAdapter::new(client);
    let write_ack = adapter
        .index_run_from_artifacts(runs_root, run_id)
        .with_context(|| format!("write ObsMem entry for '{run_id}'"))?;

    let mut policy = RetrievalPolicyV1 {
        default_limit: 25,
        required_tags: vec![
            format!("workflow:{}", indexed.workflow_id),
            format!("run:{run_id}"),
        ],
        required_failure_code: indexed.failure_code.clone(),
        order: RetrievalOrder::EvidenceAdjustedDescIdAsc,
    };
    policy.normalize();

    let request = RetrievalRequest {
        workflow_id: Some(indexed.workflow_id.clone()),
        failure_code: indexed.failure_code.clone(),
        tags: vec![format!("run:{run_id}")],
        limit_override: Some(25),
    };

    let query = request
        .to_query(&policy)
        .context("build deterministic retrieval query")?;
    let query_result = adapter
        .query_with_policy(&policy, &request)
        .context("query deterministic retrieval results")?;

    let run_paths = RunArtifactPaths::for_run_in_root(run_id, runs_root)?;
    run_paths.ensure_layout()?;

    let index_summary_path = run_paths.learning_dir().join("obs_mem_index_summary.json");
    let query_result_path = run_paths.learning_dir().join("obs_mem_query_result.json");

    let index_summary_artifact = ObsMemIndexSummaryArtifact {
        index_summary_version: OBS_MEM_INDEX_SUMMARY_VERSION,
        run_id: run_id.to_string(),
        workflow_id: indexed.workflow_id.clone(),
        indexed_entry_count: 1,
        step_context_count: indexed.steps.len(),
        trace_event_ref_count: indexed.trace_event_refs.len(),
        trace_event_refs: indexed.trace_event_refs.clone(),
        source_artifacts: vec![
            format!("runs/{run_id}/run_summary.json"),
            format!("runs/{run_id}/run_status.json"),
            format!("runs/{run_id}/logs/activation_log.json"),
        ],
        tags: indexed.tags.clone(),
        write_ack,
    };

    let query_result_artifact = ObsMemQueryResultArtifact {
        query_result_version: OBS_MEM_QUERY_RESULT_VERSION,
        run_id: run_id.to_string(),
        workflow_id: indexed.workflow_id,
        query: ObsMemQueryDescriptor {
            workflow_id: query.workflow_id,
            failure_code: query.failure_code,
            tags: query.tags,
            limit: query.limit,
        },
        ordering: ObsMemOrderingDescriptor {
            policy_order: "evidence_adjusted_desc_id_asc".to_string(),
            tie_break_fields: vec![
                "id".to_string(),
                "run_id".to_string(),
                "workflow_id".to_string(),
                "payload".to_string(),
            ],
        },
        returned_count: query_result.hits.len(),
        entries: query_result.hits,
    };

    atomic_write(
        &index_summary_path,
        &serde_json::to_vec_pretty(&index_summary_artifact)
            .context("serialize obs_mem_index_summary.json")?,
    )?;
    atomic_write(
        &query_result_path,
        &serde_json::to_vec_pretty(&query_result_artifact)
            .context("serialize obs_mem_query_result.json")?,
    )?;

    Ok(ObsMemDemoArtifacts {
        shared_store: shared_store_path,
        index_summary: index_summary_path,
        query_result: query_result_path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_fixture_run(root: &Path, run_id: &str) {
        let run = root.join(run_id);
        std::fs::create_dir_all(run.join("logs")).expect("mkdir logs");
        std::fs::write(
            run.join("run_summary.json"),
            format!(
                r#"{{"run_summary_version":1,"run_id":"{run_id}","workflow_id":"wf-obsmem-demo"}}"#
            ),
        )
        .expect("write run_summary");
        std::fs::write(
            run.join("run_status.json"),
            r#"{"run_status_version":1,"run_id":"r1","overall_status":"failure","failure_kind":"tool_failure"}"#,
        )
        .expect("write run_status");
        let activation = serde_json::json!({
            "activation_log_version": 1,
            "ordering": "append_only_emission_order",
            "stable_ids": {
                "step_id": "stable within resolved execution plan",
                "delegation_id": "deterministic per run: del-<counter>",
                "run_id": "run-scoped identifier; not replay-stable across independent runs",
            },
            "events": [
                {
                    "kind": "StepStarted",
                    "step_id": "s1",
                    "agent_id": "a",
                    "provider_id": "local",
                    "task_id": "t",
                    "delegation_json": null
                },
                {
                    "kind": "StepFinished",
                    "step_id": "s1",
                    "success": false
                }
            ]
        });
        std::fs::write(
            run.join("logs").join("activation_log.json"),
            serde_json::to_vec_pretty(&activation).expect("serialize activation"),
        )
        .expect("write activation");
    }

    fn unique_temp_dir(label: &str) -> std::path::PathBuf {
        static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "adl-obsmem-demo-{label}-pid{}-{n}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root).expect("create tmp root");
        root
    }

    #[test]
    fn emit_obsmem_demo_artifacts_is_deterministic() {
        let runs_root = unique_temp_dir("deterministic");
        let run_id = "demo-run";
        write_fixture_run(&runs_root, run_id);

        let first = emit_obsmem_demo_artifacts(&runs_root, run_id).expect("first");
        let first_index = std::fs::read(&first.index_summary).expect("read index first");
        let first_query = std::fs::read(&first.query_result).expect("read query first");

        let second = emit_obsmem_demo_artifacts(&runs_root, run_id).expect("second");
        let first_store = std::fs::read(&first.shared_store).expect("read store first");
        let second_store = std::fs::read(&second.shared_store).expect("read store second");
        let second_index = std::fs::read(&second.index_summary).expect("read index second");
        let second_query = std::fs::read(&second.query_result).expect("read query second");

        assert_eq!(first_store, second_store);
        assert_eq!(first_index, second_index);
        assert_eq!(first_query, second_query);
    }
}
