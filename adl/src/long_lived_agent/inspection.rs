//! Inspection packet generation for long-lived agent cycles.
use super::{
    ledger_cursor, load_spec, path_artifact_ref, status, status_path, LoadedAgentSpec,
    INSPECTION_PACKET_SCHEMA,
};
use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use serde_json::{json, Value};
use std::path::Path;

pub fn inspect(spec_path: &Path, options: super::InspectOptions) -> Result<Value> {
    let status = status(spec_path)?;
    let loaded = load_spec(spec_path)?;
    let ledger = ledger_cursor(&loaded)?;
    let selected_cycle_id = options
        .cycle_id
        .clone()
        .or_else(|| status.last_cycle_id.clone())
        .or(ledger.latest_cycle_id);
    let selected_cycle = selected_cycle_id
        .as_deref()
        .map(|cycle_id| inspect_cycle(&loaded, cycle_id))
        .transpose()?;
    let proof_status = if selected_cycle.is_some() {
        "pass"
    } else {
        "no_cycle_available"
    };

    Ok(json!({
        "schema": INSPECTION_PACKET_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "generated_at": Utc::now(),
        "state_root_ref": path_artifact_ref(&loaded.spec.state_root),
        "status_ref": "status.json",
        "status": status,
        "cycle_count": ledger.count,
        "selected_cycle_id": selected_cycle_id,
        "selected_cycle": selected_cycle,
        "reviewer_proof": {
            "status": proof_status,
            "status_ref": {
                "path": "status.json",
                "exists": status_path(&loaded).exists()
            },
            "cycle_ref_status": if proof_status == "pass" {
                "selected_cycle_artifacts_present"
            } else {
                "no_cycle_selected"
            }
        },
        "trace_query_decision": {
            "status": "deferred_full_platform",
            "minimal_v0_90_boundary": "inspection packet over status, cycle manifest, guardrail report, run_ref, and cycle summary artifacts",
            "full_tql_platform": "deferred",
            "full_signed_trace_architecture": "deferred",
            "reason": "WP-06 accepts the narrow reviewer inspection slice and does not widen into TQL or signed-trace architecture."
        }
    }))
}

fn inspect_cycle(loaded: &LoadedAgentSpec, cycle_id: &str) -> Result<Value> {
    validate_cycle_ref(cycle_id)?;
    let manifest_ref = format!("cycles/{cycle_id}/cycle_manifest.json");
    let guardrail_ref = format!("cycles/{cycle_id}/guardrail_report.json");
    let summary_ref = format!("cycles/{cycle_id}/cycle_summary.md");
    let decision_result_ref = format!("cycles/{cycle_id}/decision_result.json");
    let run_ref_ref = format!("cycles/{cycle_id}/run_ref.json");
    let memory_writes_ref = format!("cycles/{cycle_id}/memory_writes.jsonl");

    let manifest = read_state_json_artifact(loaded, &manifest_ref)?;
    let guardrail_report = read_state_json_artifact(loaded, &guardrail_ref)?;
    let run_ref = read_state_json_artifact(loaded, &run_ref_ref)?;
    let summary = read_state_text_artifact(loaded, &summary_ref)?;
    let guardrail_checks = guardrail_check_summary(&guardrail_report);
    let failed_guardrail_checks = guardrail_checks
        .iter()
        .filter_map(|check| {
            if check.get("result").and_then(Value::as_str) == Some("fail") {
                check
                    .get("check_id")
                    .and_then(Value::as_str)
                    .map(str::to_string)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(json!({
        "cycle_id": cycle_id,
        "status": manifest.get("status").cloned().unwrap_or(Value::Null),
        "workflow_kind": manifest.get("workflow_kind").cloned().unwrap_or(Value::Null),
        "workflow_ref": manifest.get("workflow_ref").cloned().unwrap_or(Value::Null),
        "refs": {
            "manifest": manifest_ref,
            "guardrail_report": guardrail_ref,
            "cycle_summary": summary_ref,
            "decision_result": decision_result_ref,
            "run_ref": run_ref_ref,
            "memory_writes": memory_writes_ref
        },
        "manifest": {
            "input_hash": manifest.get("input_hash").cloned().unwrap_or(Value::Null),
            "output_hash": manifest.get("output_hash").cloned().unwrap_or(Value::Null),
            "previous_cycle_id": manifest.get("previous_cycle_id").cloned().unwrap_or(Value::Null),
            "not_financial_advice": manifest.get("not_financial_advice").cloned().unwrap_or(Value::Null)
        },
        "guardrails": {
            "status": guardrail_report.get("status").cloned().unwrap_or(Value::Null),
            "checks": guardrail_checks,
            "failed_checks": failed_guardrail_checks,
            "rejected_actions": guardrail_report
                .get("rejected_actions")
                .cloned()
                .unwrap_or_else(|| json!([]))
        },
        "summary_preview": summary_preview(&summary),
        "trace_boundary": {
            "run_ref": run_ref_ref,
            "trace_ref": run_ref.get("trace_ref").cloned().unwrap_or(Value::Null),
            "status": if run_ref.get("trace_ref").is_some_and(|value| !value.is_null()) {
                "trace_ref_available"
            } else {
                "cycle_artifact_only"
            }
        }
    }))
}

fn validate_cycle_ref(cycle_id: &str) -> Result<()> {
    let Some(suffix) = cycle_id.strip_prefix("cycle-") else {
        return Err(anyhow!(
            "agent inspect --cycle must use a generated cycle id like cycle-000001"
        ));
    };
    if suffix.len() != 6 || !suffix.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(anyhow!(
            "agent inspect --cycle must use a generated cycle id like cycle-000001"
        ));
    }
    Ok(())
}

fn read_state_json_artifact(loaded: &LoadedAgentSpec, artifact_ref: &str) -> Result<Value> {
    let path = loaded.state_root.join(artifact_ref);
    if !path.exists() {
        return Err(anyhow!("inspection artifact missing: {artifact_ref}"));
    }
    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("failed reading inspection artifact {artifact_ref}"))?;
    serde_json::from_str(&raw)
        .with_context(|| format!("failed parsing inspection artifact {artifact_ref}"))
}

fn read_state_text_artifact(loaded: &LoadedAgentSpec, artifact_ref: &str) -> Result<String> {
    let path = loaded.state_root.join(artifact_ref);
    if !path.exists() {
        return Err(anyhow!("inspection artifact missing: {artifact_ref}"));
    }
    std::fs::read_to_string(&path)
        .with_context(|| format!("failed reading inspection artifact {artifact_ref}"))
}

fn guardrail_check_summary(guardrail_report: &Value) -> Vec<Value> {
    guardrail_report
        .get("checks")
        .and_then(Value::as_array)
        .map(|checks| {
            checks
                .iter()
                .map(|check| {
                    json!({
                        "check_id": check.get("check_id").cloned().unwrap_or(Value::Null),
                        "result": check.get("result").cloned().unwrap_or(Value::Null)
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn summary_preview(summary: &str) -> Vec<String> {
    summary
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .take(6)
        .map(str::to_string)
        .collect()
}
