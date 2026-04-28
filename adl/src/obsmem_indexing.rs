use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::instrumentation::{load_trace_artifact, TraceEventNormalized};
use crate::obsmem_contract::{MemoryTraceRef, ObsMemContractError, ObsMemContractErrorCode};

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
    pub trace_event_refs: Vec<MemoryTraceRef>,
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

    pub fn validate(&self) -> Result<(), ObsMemContractError> {
        if self.run_id.trim().is_empty() || self.workflow_id.trim().is_empty() {
            return Err(ObsMemContractError::new(
                ObsMemContractErrorCode::InvalidRequest,
                "indexed memory entry requires non-empty run_id and workflow_id",
            ));
        }
        if self.trace_event_refs.is_empty() {
            return Err(ObsMemContractError::new(
                ObsMemContractErrorCode::InvalidRequest,
                "indexed memory entry requires at least one trace event reference",
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

    let trace_event_refs: Vec<MemoryTraceRef> = trace
        .iter()
        .enumerate()
        .filter_map(|(sequence, event)| to_trace_ref(sequence, event))
        .collect();

    let mut tags = vec![
        format!("run:{safe_run_id}"),
        format!("workflow:{workflow_id}"),
        format!("status:{status}"),
        format!("step_context_count:{}", steps.len()),
        format!("trace_ref_count:{}", trace_event_refs.len()),
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
        trace_event_refs,
    };
    entry.normalize();
    entry.validate()?;
    Ok(entry)
}

fn to_trace_ref(sequence: usize, event: &TraceEventNormalized) -> Option<MemoryTraceRef> {
    match event {
        TraceEventNormalized::LifecyclePhaseEntered { .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "lifecycle_phase_entered".to_string(),
            step_id: None,
            delegation_id: None,
        }),
        TraceEventNormalized::ExecutionBoundaryCrossed { .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "execution_boundary_crossed".to_string(),
            step_id: None,
            delegation_id: None,
        }),
        TraceEventNormalized::GovernedProposalObserved { .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "governed_proposal_observed".to_string(),
            step_id: None,
            delegation_id: None,
        }),
        TraceEventNormalized::GovernedProposalNormalized { .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "governed_proposal_normalized".to_string(),
            step_id: None,
            delegation_id: None,
        }),
        TraceEventNormalized::GovernedAccConstructed { .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "governed_acc_constructed".to_string(),
            step_id: None,
            delegation_id: None,
        }),
        TraceEventNormalized::GovernedPolicyInjected { .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "governed_policy_injected".to_string(),
            step_id: None,
            delegation_id: None,
        }),
        TraceEventNormalized::GovernedVisibilityResolved { .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "governed_visibility_resolved".to_string(),
            step_id: None,
            delegation_id: None,
        }),
        TraceEventNormalized::GovernedFreedomGateDecided { .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "governed_freedom_gate_decided".to_string(),
            step_id: None,
            delegation_id: None,
        }),
        TraceEventNormalized::GovernedActionSelected { .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "governed_action_selected".to_string(),
            step_id: None,
            delegation_id: None,
        }),
        TraceEventNormalized::GovernedActionRejected { .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "governed_action_rejected".to_string(),
            step_id: None,
            delegation_id: None,
        }),
        TraceEventNormalized::GovernedExecutionResultRecorded { .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "governed_execution_result_recorded".to_string(),
            step_id: None,
            delegation_id: None,
        }),
        TraceEventNormalized::GovernedRefusalRecorded { .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "governed_refusal_recorded".to_string(),
            step_id: None,
            delegation_id: None,
        }),
        TraceEventNormalized::GovernedRedactionDecisionRecorded { .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "governed_redaction_decision_recorded".to_string(),
            step_id: None,
            delegation_id: None,
        }),
        TraceEventNormalized::SchedulerPolicy { .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "scheduler_policy".to_string(),
            step_id: None,
            delegation_id: None,
        }),
        TraceEventNormalized::RunFailed { .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "run_failed".to_string(),
            step_id: None,
            delegation_id: None,
        }),
        TraceEventNormalized::RunFinished { .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "run_finished".to_string(),
            step_id: None,
            delegation_id: None,
        }),
        TraceEventNormalized::StepStarted { step_id, .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "step_started".to_string(),
            step_id: Some(step_id.clone()),
            delegation_id: None,
        }),
        TraceEventNormalized::PromptAssembled { step_id, .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "prompt_assembled".to_string(),
            step_id: Some(step_id.clone()),
            delegation_id: None,
        }),
        TraceEventNormalized::StepOutputChunk { step_id, .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "step_output_chunk".to_string(),
            step_id: Some(step_id.clone()),
            delegation_id: None,
        }),
        TraceEventNormalized::DelegationRequested {
            delegation_id,
            step_id,
            ..
        } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "delegation_requested".to_string(),
            step_id: Some(step_id.clone()),
            delegation_id: Some(delegation_id.clone()),
        }),
        TraceEventNormalized::DelegationPolicyEvaluated {
            delegation_id,
            step_id,
            ..
        } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "delegation_policy_evaluated".to_string(),
            step_id: Some(step_id.clone()),
            delegation_id: Some(delegation_id.clone()),
        }),
        TraceEventNormalized::DelegationApproved {
            delegation_id,
            step_id,
        } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "delegation_approved".to_string(),
            step_id: Some(step_id.clone()),
            delegation_id: Some(delegation_id.clone()),
        }),
        TraceEventNormalized::DelegationDenied {
            delegation_id,
            step_id,
            ..
        } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "delegation_denied".to_string(),
            step_id: Some(step_id.clone()),
            delegation_id: Some(delegation_id.clone()),
        }),
        TraceEventNormalized::DelegationDispatched {
            delegation_id,
            step_id,
            ..
        } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "delegation_dispatched".to_string(),
            step_id: Some(step_id.clone()),
            delegation_id: Some(delegation_id.clone()),
        }),
        TraceEventNormalized::DelegationResultReceived {
            delegation_id,
            step_id,
            ..
        } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "delegation_result_received".to_string(),
            step_id: Some(step_id.clone()),
            delegation_id: Some(delegation_id.clone()),
        }),
        TraceEventNormalized::DelegationCompleted {
            delegation_id,
            step_id,
            ..
        } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "delegation_completed".to_string(),
            step_id: Some(step_id.clone()),
            delegation_id: Some(delegation_id.clone()),
        }),
        TraceEventNormalized::StepFinished { step_id, .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "step_finished".to_string(),
            step_id: Some(step_id.clone()),
            delegation_id: None,
        }),
        TraceEventNormalized::CallEntered { caller_step_id, .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "call_entered".to_string(),
            step_id: Some(caller_step_id.clone()),
            delegation_id: None,
        }),
        TraceEventNormalized::CallExited { caller_step_id, .. } => Some(MemoryTraceRef {
            event_sequence: sequence,
            event_kind: "call_exited".to_string(),
            step_id: Some(caller_step_id.clone()),
            delegation_id: None,
        }),
    }
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
    use crate::{instrumentation, trace::Trace};

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
    fn trace_index_run_from_artifacts_is_deterministic() {
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
    fn trace_index_run_from_artifacts_captures_step_context_fields() {
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
    fn trace_indexed_memory_entry_validate_rejects_invalid_order_and_content() {
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
            trace_event_refs: vec![MemoryTraceRef {
                event_sequence: 0,
                event_kind: "step_started".to_string(),
                step_id: Some("s1".to_string()),
                delegation_id: None,
            }],
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
    fn trace_index_run_from_artifacts_rejects_empty_and_missing_run_inputs() {
        let tmp = unique_temp_dir("missing-inputs");
        let err = index_run_from_artifacts(&tmp, "").expect_err("empty run_id must fail");
        assert!(err.message.contains("run_id must not be empty"));

        let err =
            index_run_from_artifacts(&tmp, "missing-run").expect_err("missing files must fail");
        assert!(err.message.contains("failed reading"));
    }

    #[test]
    fn trace_index_run_from_artifacts_rejects_unsafe_run_id_path_segments() {
        let tmp = unique_temp_dir("unsafe-run-id");
        let err = index_run_from_artifacts(&tmp, "../escape")
            .expect_err("unsafe run_id must fail before filesystem access");
        assert!(err.message.contains("safe path segment"));
    }

    #[test]
    fn trace_index_run_from_artifacts_requires_workflow_id_and_uses_status_fallback() {
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
                "events": [
                    {
                        "kind": "StepStarted",
                        "step_id": "s1",
                        "agent_id": "a",
                        "provider_id": "local",
                        "task_id": "t",
                        "delegation_json": null
                    }
                ]
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

    #[test]
    fn trace_index_run_from_artifacts_captures_governed_trace_event_refs() {
        let tmp = unique_temp_dir("governed-trace-refs");
        let run_id = "r-governed";
        let run = tmp.join(run_id);
        std::fs::create_dir_all(run.join("logs")).expect("mkdir logs");
        std::fs::write(
            run.join("run_summary.json"),
            format!(
                r#"{{"run_summary_version":1,"run_id":"{run_id}","workflow_id":"wf-governed"}}"#
            ),
        )
        .expect("write run_summary");
        std::fs::write(
            run.join("run_status.json"),
            r#"{"run_status_version":1,"run_id":"r-governed","overall_status":"success","failure_kind":null}"#,
        )
        .expect("write run_status");

        let mut trace = Trace::new(
            run_id.to_string(),
            "wf-governed".to_string(),
            "0.90.5".to_string(),
        );
        trace.governed_proposal_observed(
            "proposal.safe-read",
            "tool.safe_read",
            "governed/proposal_arguments.redacted.json",
        );
        trace.governed_proposal_normalized(
            "proposal.safe-read",
            "governed/proposal.normalized.json",
            "governed/proposal_arguments.redacted.json",
        );
        trace.governed_acc_constructed("proposal.safe-read", "acc.safe-read", "portable_replay");
        trace.governed_policy_injected(
            "proposal.safe-read",
            "governed/policy_evidence.json",
            "allowed",
        );
        trace.governed_visibility_resolved(
            "proposal.safe-read",
            "actor:redacted",
            "operator:reviewable",
            "reviewer:reviewable",
            "public:withheld",
            "observatory:bounded",
        );
        trace.governed_freedom_gate_decided(
            "proposal.safe-read",
            "candidate.safe-read",
            "allowed",
            "policy_ok",
            "citizen_boundary",
            "arguments_redacted",
        );
        trace.governed_action_selected(
            "proposal.safe-read",
            "action.safe-read",
            "tool.safe_read",
            "adapter.fixture.safe_read.dry_run",
            vec![
                "gate:policy_ok".to_string(),
                "policy:portable_replay".to_string(),
            ],
        );
        trace.governed_action_rejected(
            "proposal.safe-read",
            "action.write",
            "tool.write",
            "adapter.fixture.write.dry_run",
            "write_not_allowed",
            vec!["gate:denied".to_string()],
        );
        trace.governed_execution_result(
            "proposal.safe-read",
            "action.safe-read",
            "adapter.fixture.safe_read.dry_run",
            "governed/result.redacted.json",
            vec!["result:fixture_read_completed".to_string()],
        );
        trace.governed_refusal(
            "proposal.safe-read",
            "action.write",
            "write_not_allowed",
            vec!["gate:denied".to_string()],
        );
        trace.governed_redaction_decision(
            "proposal.safe-read",
            "reviewer",
            vec!["arguments".to_string(), "result".to_string()],
            "redacted",
            Some("bounded disclosure"),
        );
        instrumentation::write_trace_artifact(
            &run.join("logs").join("activation_log.json"),
            &trace.events,
        )
        .expect("write activation");

        let indexed = index_run_from_artifacts(&tmp, run_id).expect("index should succeed");
        let event_kinds: Vec<&str> = indexed
            .trace_event_refs
            .iter()
            .map(|reference| reference.event_kind.as_str())
            .collect();
        for expected in [
            "governed_proposal_observed",
            "governed_proposal_normalized",
            "governed_acc_constructed",
            "governed_policy_injected",
            "governed_visibility_resolved",
            "governed_freedom_gate_decided",
            "governed_action_selected",
            "governed_action_rejected",
            "governed_execution_result_recorded",
            "governed_refusal_recorded",
            "governed_redaction_decision_recorded",
        ] {
            assert!(
                event_kinds.contains(&expected),
                "missing governed trace ref for {expected}"
            );
        }
    }
}
