use std::collections::BTreeMap;
use std::fs;

use crate::artifacts;
use crate::instrumentation;
use crate::trace::{Trace, TraceEvent};
use serde_json::{json, Value as JsonValue};

use super::*;

pub(crate) fn write_governed_trace_bundle(root: &Path, run_id: &str, trace: &Trace) -> Result<()> {
    let run_paths = artifacts::RunArtifactPaths::for_run_in_root(run_id, root.join("artifacts"))?;
    run_paths.ensure_layout()?;
    run_paths.write_model_marker()?;
    instrumentation::write_trace_artifact(&run_paths.activation_log_json(), &trace.events)?;
    write_governed_trace_artifacts_for_run_paths(&run_paths, trace)
}

pub(crate) fn trace_ref_for_run(run_id: &str) -> String {
    format!("artifacts/{run_id}/logs/activation_log.json")
}

pub(crate) fn proposal_redaction_ref_for_run(run_id: &str) -> String {
    format!("artifacts/{run_id}/governed/proposal_arguments.redacted.json")
}

pub(crate) fn result_redaction_ref_for_run(run_id: &str) -> String {
    format!("artifacts/{run_id}/governed/result.redacted.json")
}

fn write_governed_trace_artifacts_for_run_paths(
    run_paths: &artifacts::RunArtifactPaths,
    trace: &Trace,
) -> Result<()> {
    let governed_dir = run_paths.run_dir().join("governed");
    fs::create_dir_all(&governed_dir).context("create governed artifact dir")?;
    let (proposal_artifact, result_artifact) = governed_trace_artifacts_for_run(trace);
    if let Some(proposal_artifact) = proposal_artifact {
        artifacts::atomic_write(
            &governed_dir.join("proposal_arguments.redacted.json"),
            &serde_json::to_vec_pretty(&proposal_artifact)
                .context("serialize governed proposal arguments artifact")?,
        )?;
    }
    if let Some(result_artifact) = result_artifact {
        artifacts::atomic_write(
            &governed_dir.join("result.redacted.json"),
            &serde_json::to_vec_pretty(&result_artifact)
                .context("serialize governed result artifact")?,
        )?;
    }
    Ok(())
}

fn governed_trace_artifacts_for_run(trace: &Trace) -> (Option<JsonValue>, Option<JsonValue>) {
    let mut proposal_redaction_summaries = BTreeMap::new();
    let mut proposal_redaction_details = BTreeMap::new();
    for event in &trace.events {
        match event {
            TraceEvent::GovernedFreedomGateDecided {
                proposal_id,
                redaction_summary,
                ..
            } => {
                proposal_redaction_summaries
                    .entry(proposal_id.clone())
                    .or_insert_with(|| redaction_summary.clone());
            }
            TraceEvent::GovernedRedactionDecisionRecorded {
                proposal_id,
                detail,
                ..
            } => {
                proposal_redaction_details
                    .entry(proposal_id.clone())
                    .or_insert_with(|| detail.clone());
            }
            _ => {}
        }
    }

    let mut proposal_entries = Vec::new();
    let mut result_entries = Vec::new();
    for event in &trace.events {
        match event {
            TraceEvent::GovernedProposalObserved {
                proposal_id,
                tool_name,
                redacted_arguments_ref,
                ..
            } => {
                proposal_entries.push(json!({
                    "proposal_id": proposal_id,
                    "tool_name": tool_name,
                    "redacted_arguments_ref": redacted_arguments_ref,
                    "redaction": {
                        "status": "redacted",
                        "detail": proposal_redaction_details.get(proposal_id).cloned().flatten(),
                        "summary": proposal_redaction_summaries.get(proposal_id).cloned(),
                    }
                }));
            }
            TraceEvent::GovernedExecutionResultRecorded {
                proposal_id,
                action_id,
                adapter_id,
                result_ref,
                evidence_refs,
                ..
            } => {
                result_entries.push(json!({
                    "proposal_id": proposal_id,
                    "action_id": action_id,
                    "adapter_id": adapter_id,
                    "result_ref": result_ref,
                    "result_status": "executed_redacted",
                    "evidence_refs": evidence_refs,
                }));
            }
            TraceEvent::GovernedRefusalRecorded {
                proposal_id,
                action_id,
                reason_code,
                evidence_refs,
                ..
            } => {
                result_entries.push(json!({
                    "proposal_id": proposal_id,
                    "action_id": action_id,
                    "result_status": "refused_redacted",
                    "reason_code": reason_code,
                    "evidence_refs": evidence_refs,
                }));
            }
            _ => {}
        }
    }

    let proposal_artifact = (!proposal_entries.is_empty()).then(|| {
        json!({
            "schema_version": "governed_trace_arguments.v1",
            "run_id": trace.run_id.clone(),
            "entries": proposal_entries,
        })
    });
    let result_artifact = (!result_entries.is_empty()).then(|| {
        json!({
            "schema_version": "governed_trace_results.v1",
            "run_id": trace.run_id.clone(),
            "entries": result_entries,
        })
    });
    (proposal_artifact, result_artifact)
}
