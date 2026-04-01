use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::instrumentation::{load_trace_artifact, TraceEventNormalized};
use crate::obsmem_contract::{ObsMemContractError, ObsMemContractErrorCode};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexedStepContext {
    pub sequence: usize,
    pub step_id: String,
    pub event_kind: String,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexedMemoryEntry {
    pub run_id: String,
    pub workflow_id: String,
    pub status: String,
    pub failure_code: Option<String>,
    pub summary: String,
    pub tags: Vec<String>,
    pub steps: Vec<IndexedStepContext>,
}

impl IndexedMemoryEntry {
    pub fn normalize(&mut self) {
        self.tags.sort();
        self.tags.dedup();
        self.steps.sort_by(|a, b| {
            a.sequence
                .cmp(&b.sequence)
                .then_with(|| a.step_id.cmp(&b.step_id))
                .then_with(|| a.event_kind.cmp(&b.event_kind))
                .then_with(|| a.context.cmp(&b.context))
        });
        self.steps.dedup_by(|a, b| {
            a.sequence == b.sequence
                && a.step_id == b.step_id
                && a.event_kind == b.event_kind
                && a.context == b.context
        });
    }

    pub fn validate(&self) -> Result<(), ObsMemContractError> {
        if self.run_id.trim().is_empty() || self.workflow_id.trim().is_empty() {
            return Err(ObsMemContractError::new(
                ObsMemContractErrorCode::InvalidRequest,
                "indexed memory entry requires non-empty run_id and workflow_id",
            ));
        }
        if self.summary.trim().is_empty() {
            return Err(ObsMemContractError::new(
                ObsMemContractErrorCode::InvalidRequest,
                "indexed memory entry requires non-empty summary",
            ));
        }

        for w in self.steps.windows(2) {
            if w[0].sequence > w[1].sequence {
                return Err(ObsMemContractError::new(
                    ObsMemContractErrorCode::InvalidRequest,
                    "indexed step contexts must be ordered by non-decreasing sequence",
                ));
            }
        }

        let mut text = self.summary.clone();
        for tag in &self.tags {
            text.push('\n');
            text.push_str(tag);
        }
        for step in &self.steps {
            text.push('\n');
            text.push_str(&step.context);
        }
        if contains_disallowed_text(&text) {
            return Err(ObsMemContractError::new(
                ObsMemContractErrorCode::PrivacyViolation,
                "indexed memory entry contains disallowed host-path or token-like content",
            ));
        }
        Ok(())
    }
}

pub fn index_run_from_artifacts(
    runs_root: &Path,
    run_id: &str,
) -> Result<IndexedMemoryEntry, ObsMemContractError> {
    let safe_run_id = crate::artifacts::validate_run_id_path_segment(run_id).map_err(|err| {
        ObsMemContractError::new(ObsMemContractErrorCode::InvalidRequest, err.to_string())
    })?;

    let run_dir = runs_root.join(&safe_run_id);
    let run_summary_path = run_dir.join("run_summary.json");
    let run_status_path = run_dir.join("run_status.json");
    let activation_log_path = run_dir.join("logs").join("activation_log.json");

    let run_summary = read_json(&run_summary_path)?;
    let run_status = read_json(&run_status_path)?;
    let trace = load_trace_artifact(&activation_log_path).map_err(|err| {
        ObsMemContractError::new(
            ObsMemContractErrorCode::InvalidRequest,
            format!(
                "failed reading activation log '{}': {err}",
                activation_log_path.display()
            ),
        )
    })?;

    let workflow_id = run_summary
        .get("workflow_id")
        .and_then(JsonValue::as_str)
        .ok_or_else(|| {
            ObsMemContractError::new(
                ObsMemContractErrorCode::InvalidRequest,
                "run_summary.json missing workflow_id",
            )
        })?
        .to_string();

    let status = run_status
        .get("overall_status")
        .and_then(JsonValue::as_str)
        .unwrap_or("unknown")
        .to_string();

    let failure_code = run_status
        .get("failure_kind")
        .and_then(JsonValue::as_str)
        .map(str::to_string);

    let mut steps = Vec::new();
    for (sequence, event) in trace.iter().enumerate() {
        if let Some(step_ctx) = to_step_context(sequence, event) {
            steps.push(step_ctx);
        }
    }

    let mut tags = vec![
        format!("run:{safe_run_id}"),
        format!("workflow:{workflow_id}"),
        format!("status:{status}"),
        format!("step_context_count:{}", steps.len()),
    ];
    if let Some(code) = failure_code.as_deref() {
        tags.push(format!("failure:{code}"));
    }

    let summary = format!(
        "workflow={workflow_id} overall_status={status} failure_kind={} step_context_count={}",
        failure_code.as_deref().unwrap_or("none"),
        steps.len()
    );

    let mut entry = IndexedMemoryEntry {
        run_id: safe_run_id,
        workflow_id,
        status,
        failure_code,
        summary,
        tags,
        steps,
    };
    entry.normalize();
    entry.validate()?;
    Ok(entry)
}

fn to_step_context(sequence: usize, event: &TraceEventNormalized) -> Option<IndexedStepContext> {
    match event {
        TraceEventNormalized::StepStarted {
            step_id,
            provider_id,
            ..
        } => Some(IndexedStepContext {
            sequence,
            step_id: step_id.clone(),
            event_kind: "step_started".to_string(),
            context: format!("provider={provider_id}"),
        }),
        TraceEventNormalized::PromptAssembled {
            step_id,
            prompt_hash,
        } => Some(IndexedStepContext {
            sequence,
            step_id: step_id.clone(),
            event_kind: "prompt_assembled".to_string(),
            context: format!("prompt_hash={prompt_hash}"),
        }),
        TraceEventNormalized::StepOutputChunk {
            step_id,
            chunk_bytes,
        } => Some(IndexedStepContext {
            sequence,
            step_id: step_id.clone(),
            event_kind: "step_output_chunk".to_string(),
            context: format!("chunk_bytes={chunk_bytes}"),
        }),
        TraceEventNormalized::StepFinished { step_id, success } => Some(IndexedStepContext {
            sequence,
            step_id: step_id.clone(),
            event_kind: "step_finished".to_string(),
            context: format!("success={success}"),
        }),
        _ => None,
    }
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

fn contains_disallowed_text(text: &str) -> bool {
    text.contains("/Users/")
        || text.contains("/home/")
        || text.contains("gho_")
        || text.contains("sk-")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_fixture_run(root: &Path, run_id: &str) {
        let run = root.join(run_id);
        std::fs::create_dir_all(run.join("logs")).expect("mkdir logs");
        std::fs::write(
            run.join("run_summary.json"),
            format!(r#"{{"run_summary_version":1,"run_id":"{run_id}","workflow_id":"wf-index"}}"#),
        )
        .expect("write run_summary");
        std::fs::write(
            run.join("run_status.json"),
            r#"{"run_status_version":1,"run_id":"r1","overall_status":"success","failure_kind":null}"#,
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
                    "kind": "PromptAssembled",
                    "step_id": "s1",
                    "prompt_hash": "abc123"
                },
                {
                    "kind": "StepFinished",
                    "step_id": "s1",
                    "success": true
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
            "adl-obsmem-indexing-{label}-pid{}-{n}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root).expect("create tmp root");
        root
    }

    #[test]
    fn index_run_from_artifacts_is_deterministic() {
        let tmp = unique_temp_dir("deterministic");
        write_fixture_run(&tmp, "r1");
        let left = index_run_from_artifacts(&tmp, "r1").expect("left");
        let right = index_run_from_artifacts(&tmp, "r1").expect("right");
        assert_eq!(left, right);
        assert_eq!(
            left.steps.iter().map(|s| s.sequence).collect::<Vec<_>>(),
            vec![0, 1, 2]
        );
    }

    #[test]
    fn index_run_from_artifacts_captures_step_context_fields() {
        let tmp = unique_temp_dir("step-context");
        write_fixture_run(&tmp, "r2");
        let indexed = index_run_from_artifacts(&tmp, "r2").expect("indexed");
        assert_eq!(indexed.workflow_id, "wf-index");
        assert!(indexed
            .tags
            .binary_search(&"workflow:wf-index".to_string())
            .is_ok());
        assert!(indexed.summary.contains("step_context_count=3"));
        assert_eq!(indexed.steps[0].step_id, "s1");
        assert_eq!(indexed.steps[0].event_kind, "step_started");
    }

    #[test]
    fn indexed_memory_entry_validate_rejects_invalid_order_and_content() {
        let mut entry = IndexedMemoryEntry {
            run_id: "r1".to_string(),
            workflow_id: "wf".to_string(),
            status: "success".to_string(),
            failure_code: None,
            summary: "ok".to_string(),
            tags: vec!["a".to_string()],
            steps: vec![
                IndexedStepContext {
                    sequence: 2,
                    step_id: "s2".to_string(),
                    event_kind: "step_finished".to_string(),
                    context: "success=true".to_string(),
                },
                IndexedStepContext {
                    sequence: 1,
                    step_id: "s1".to_string(),
                    event_kind: "step_started".to_string(),
                    context: "provider=local".to_string(),
                },
            ],
        };
        let err = entry
            .validate()
            .expect_err("out-of-order sequence must fail");
        assert!(err.message.contains("ordered by non-decreasing sequence"));

        entry.steps.sort_by_key(|s| s.sequence);
        entry.summary = "/Users/alice/private".to_string();
        let err = entry
            .validate()
            .expect_err("host-path content must be rejected");
        assert!(err.message.contains("disallowed host-path"));
    }

    #[test]
    fn index_run_from_artifacts_rejects_empty_and_missing_run_inputs() {
        let tmp = unique_temp_dir("missing-inputs");
        let err = index_run_from_artifacts(&tmp, "").expect_err("empty run_id must fail");
        assert!(err.message.contains("run_id must not be empty"));

        let err =
            index_run_from_artifacts(&tmp, "missing-run").expect_err("missing files must fail");
        assert!(err.message.contains("failed reading"));
    }

    #[test]
    fn index_run_from_artifacts_rejects_unsafe_run_id_path_segments() {
        let tmp = unique_temp_dir("unsafe-run-id");
        let err = index_run_from_artifacts(&tmp, "../escape")
            .expect_err("unsafe run_id must fail before filesystem access");
        assert!(err.message.contains("safe path segment"));
    }

    #[test]
    fn index_run_from_artifacts_requires_workflow_id_and_uses_status_fallback() {
        let tmp = unique_temp_dir("status-fallback");
        let run_id = "r3";
        let run = tmp.join(run_id);
        std::fs::create_dir_all(run.join("logs")).expect("mkdir logs");
        std::fs::write(run.join("run_summary.json"), r#"{"run_summary_version":1}"#)
            .expect("write bad run summary");
        std::fs::write(run.join("run_status.json"), r#"{"run_status_version":1}"#)
            .expect("write run_status");
        std::fs::write(
            run.join("logs").join("activation_log.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "activation_log_version": 1,
                "ordering": "append_only_emission_order",
                "stable_ids": {
                    "step_id": "stable",
                    "delegation_id": "stable",
                    "run_id": "not replay-stable"
                },
                "events": []
            }))
            .expect("serialize activation"),
        )
        .expect("write activation");

        let err =
            index_run_from_artifacts(&tmp, run_id).expect_err("missing workflow_id must fail");
        assert!(err.message.contains("missing workflow_id"));

        std::fs::write(
            run.join("run_summary.json"),
            r#"{"run_summary_version":1,"run_id":"r3","workflow_id":"wf-index"}"#,
        )
        .expect("repair run summary");
        let indexed = index_run_from_artifacts(&tmp, run_id).expect("index should succeed");
        assert_eq!(indexed.status, "unknown");
        assert!(indexed.summary.contains("overall_status=unknown"));
    }
}
