use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

mod schema;
mod storage;
mod types;

use schema::*;
use storage::*;
use types::LedgerCursor;
pub use types::{
    AgentSpec, AgentStatusState, HeartbeatSpec, InspectOptions, LeaseRecord, LoadedAgentSpec,
    RunOptions, StatusError, StatusRecord, StopRecord, TickOptions, WorkflowSpec,
};

pub fn load_spec(spec_path: &Path) -> Result<LoadedAgentSpec> {
    let raw = fs::read_to_string(spec_path)
        .with_context(|| format!("failed reading agent spec {}", spec_path.display()))?;
    let spec: AgentSpec = serde_yaml::from_str(&raw)
        .with_context(|| format!("failed parsing agent spec {}", spec_path.display()))?;
    validate_spec(&spec)?;
    let base = spec_path.parent().unwrap_or_else(|| Path::new("."));
    let state_root = if spec.state_root.is_absolute() {
        spec.state_root.clone()
    } else {
        base.join(&spec.state_root)
    };
    Ok(LoadedAgentSpec {
        spec,
        spec_path: spec_path.to_path_buf(),
        state_root,
    })
}

pub fn tick(spec_path: &Path, options: TickOptions) -> Result<StatusRecord> {
    let loaded = load_spec(spec_path)?;
    ensure_state_root(&loaded)?;
    if let Some(stop) = read_stop(&loaded)? {
        let status = stopped_status(&loaded, stop.reason);
        write_status(&loaded, &status)?;
        return Ok(status);
    }

    let cycle_id = next_cycle_id(&loaded)?;
    let lease = acquire_lease(&loaded, &cycle_id, options.recover_stale_lease)?;
    let running = status_with_state(
        &loaded,
        AgentStatusState::RunningCycle,
        None,
        None,
        Some(lease.clone()),
        false,
        None,
    );
    write_status(&loaded, &running)?;

    let result = write_cycle_artifacts(&loaded, &cycle_id);
    remove_lease(&loaded)?;

    match result {
        Ok(()) => {
            let status = status_with_state(
                &loaded,
                AgentStatusState::Idle,
                Some(cycle_id),
                Some("success".to_string()),
                None,
                false,
                None,
            );
            write_status(&loaded, &status)?;
            Ok(status)
        }
        Err(err) => {
            let cursor = ledger_cursor(&loaded).unwrap_or_default();
            let status = status_with_state(
                &loaded,
                AgentStatusState::Failed,
                cursor.latest_cycle_id.or(Some(cycle_id)),
                cursor.latest_status.or_else(|| Some("failed".to_string())),
                None,
                false,
                Some(StatusError {
                    class: "workflow_failed".to_string(),
                    message: err.to_string(),
                }),
            );
            write_status(&loaded, &status)?;
            Err(err)
        }
    }
}

pub fn run(spec_path: &Path, options: RunOptions) -> Result<StatusRecord> {
    if options.max_cycles == 0 {
        return Err(anyhow!("agent run requires --max-cycles greater than zero"));
    }
    let loaded = load_spec(spec_path)?;
    let sleep_secs = options
        .interval_secs
        .or(loaded.spec.heartbeat.interval_secs)
        .unwrap_or(0);

    let mut last_status = status(spec_path)?;
    for index in 0..options.max_cycles {
        if read_stop(&loaded)?.is_some() {
            last_status = tick(
                spec_path,
                TickOptions {
                    recover_stale_lease: options.recover_stale_lease,
                },
            )?;
            break;
        }
        match tick(
            spec_path,
            TickOptions {
                recover_stale_lease: options.recover_stale_lease,
            },
        ) {
            Ok(status) => {
                last_status = status;
            }
            Err(err) => {
                last_status = status(spec_path)?;
                let failures = consecutive_failure_count(&loaded)?;
                if failures >= max_consecutive_failures(&loaded) {
                    last_status = write_stop_record(
                        &loaded,
                        &format!(
                            "max_consecutive_failures reached after {failures} blocked or failed cycles"
                        ),
                        "supervisor",
                        "max_consecutive_failures",
                    )?;
                    break;
                }
                if index + 1 >= options.max_cycles {
                    return Err(err);
                }
            }
        }
        if last_status.state == AgentStatusState::Stopped {
            break;
        }
        if index + 1 < options.max_cycles && !options.no_sleep && sleep_secs > 0 {
            thread::sleep(Duration::from_secs(sleep_secs));
        }
    }

    if last_status.state != AgentStatusState::Stopped {
        last_status.state = AgentStatusState::Completed;
        last_status.updated_at = Utc::now();
        write_status(&loaded, &last_status)?;
    }
    Ok(last_status)
}

pub fn status(spec_path: &Path) -> Result<StatusRecord> {
    let loaded = load_spec(spec_path)?;
    ensure_state_root(&loaded)?;
    let ledger = ledger_cursor(&loaded)?;
    let mut current = read_status(&loaded)?.unwrap_or_else(|| {
        if ledger.latest_cycle_id.is_some() {
            status_with_state(
                &loaded,
                AgentStatusState::Idle,
                ledger.latest_cycle_id.clone(),
                ledger.latest_status.clone(),
                None,
                false,
                None,
            )
        } else {
            status_with_state(
                &loaded,
                AgentStatusState::NotStarted,
                None,
                None,
                None,
                false,
                None,
            )
        }
    });
    if let Some(latest_cycle_id) = ledger.latest_cycle_id.clone() {
        current.last_cycle_id = Some(latest_cycle_id);
        current.last_cycle_status = ledger.latest_status.clone();
        if current.state == AgentStatusState::NotStarted {
            current.state = AgentStatusState::Idle;
        }
    }
    current.completed_cycle_count = completed_cycle_count(&loaded)?;

    if let Some(stop) = read_stop(&loaded)? {
        current.state = AgentStatusState::Stopped;
        current.stop_requested = true;
        current.last_error = Some(StatusError {
            class: "operator_stop_requested".to_string(),
            message: stop.reason,
        });
    } else if let Some(lease) = read_lease(&loaded)? {
        if lease_is_stale(&lease) {
            current.state = AgentStatusState::Failed;
            current.active_lease = Some(lease);
            current.last_error = Some(StatusError {
                class: "lease_stale".to_string(),
                message: "active lease is stale and requires explicit recovery".to_string(),
            });
        } else {
            current.state = AgentStatusState::Leased;
            current.active_lease = Some(lease);
            current.last_error = None;
        }
    }
    current.updated_at = Utc::now();
    write_status(&loaded, &current)?;
    Ok(current)
}

pub fn stop(spec_path: &Path, reason: &str) -> Result<StatusRecord> {
    if reason.trim().is_empty() {
        return Err(anyhow!("agent stop requires a non-empty --reason"));
    }
    let loaded = load_spec(spec_path)?;
    ensure_state_root(&loaded)?;
    write_stop_record(
        &loaded,
        reason.trim(),
        "operator",
        "operator_stop_requested",
    )
}

pub fn inspect(spec_path: &Path, options: InspectOptions) -> Result<Value> {
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

fn validate_spec(spec: &AgentSpec) -> Result<()> {
    if spec.schema != SPEC_SCHEMA {
        return Err(anyhow!(
            "unsupported agent spec schema '{}' (expected {SPEC_SCHEMA})",
            spec.schema
        ));
    }
    if spec.agent_instance_id.trim().is_empty() {
        return Err(anyhow!("agent spec requires agent_instance_id"));
    }
    if spec.display_name.trim().is_empty() {
        return Err(anyhow!("agent spec requires display_name"));
    }
    if spec.workflow.kind.trim().is_empty() {
        return Err(anyhow!("agent spec requires workflow.kind"));
    }
    let stale = spec.heartbeat.stale_lease_after_secs.unwrap_or(900);
    if stale == 0 {
        return Err(anyhow!(
            "agent spec heartbeat.stale_lease_after_secs must be greater than zero"
        ));
    }
    if safety_u64(
        &spec.safety,
        "max_cycle_runtime_secs",
        DEFAULT_MAX_CYCLE_RUNTIME_SECS,
    ) == 0
    {
        return Err(anyhow!(
            "agent spec safety.max_cycle_runtime_secs must be greater than zero"
        ));
    }
    if safety_u64(
        &spec.safety,
        "max_consecutive_failures",
        DEFAULT_MAX_CONSECUTIVE_FAILURES,
    ) == 0
    {
        return Err(anyhow!(
            "agent spec safety.max_consecutive_failures must be greater than zero"
        ));
    }
    Ok(())
}

fn ensure_state_root(loaded: &LoadedAgentSpec) -> Result<()> {
    fs::create_dir_all(cycles_dir(loaded))
        .with_context(|| format!("failed creating {}", cycles_dir(loaded).display()))?;
    ensure_locked_spec(loaded)?;
    ensure_jsonl_file(&cycle_ledger_path(loaded))?;
    ensure_jsonl_file(&provider_binding_history_path(loaded))?;
    ensure_continuity(loaded)?;
    ensure_memory_index(loaded)?;
    if !status_path(loaded).exists() {
        let status = status_with_state(
            loaded,
            AgentStatusState::NotStarted,
            None,
            None,
            None,
            false,
            None,
        );
        write_status(loaded, &status)?;
    }
    Ok(())
}

fn ensure_locked_spec(loaded: &LoadedAgentSpec) -> Result<()> {
    let locked = locked_spec_path(loaded);
    let current = serde_json::to_value(&loaded.spec)?;
    if locked.exists() {
        let locked_value: Value = read_json_required(&locked)?;
        if locked_value != current {
            append_operator_event(
                loaded,
                "spec_revision_requested",
                json!({
                    "reason": "operator spec changed after lock creation",
                    "locked_spec_ref": "agent_spec.locked.json"
                }),
            )?;
            return Err(anyhow!(
                "spec_revision_required: {} differs from the locked continuity spec",
                loaded.spec_path.display()
            ));
        }
    } else {
        write_json_pretty(&locked, &loaded.spec)?;
        append_operator_event(
            loaded,
            "created",
            json!({
                "locked_spec_ref": "agent_spec.locked.json",
                "continuity_kind": "pre_v0_92_handle"
            }),
        )?;
    }
    Ok(())
}

fn ensure_continuity(loaded: &LoadedAgentSpec) -> Result<()> {
    let path = continuity_path(loaded);
    if path.exists() {
        return Ok(());
    }
    let continuity = json!({
        "schema": CONTINUITY_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "display_name": loaded.spec.display_name.clone(),
        "created_at": Utc::now(),
        "created_by": "operator",
        "continuity_kind": "pre_v0_92_handle",
        "status": "active",
        "state_root": path_artifact_ref(&loaded.spec.state_root),
        "memory_namespace": memory_namespace(loaded),
        "cycle_ledger_ref": "cycle_ledger.jsonl",
        "latest_cycle_id": Value::Null,
        "future_identity_ref": Value::Null,
        "non_claims": [
            "not_v0_92_identity_tuple",
            "not_capability_governance",
            "not_autonomous_legal_personhood"
        ]
    });
    write_json_pretty(&path, &continuity)
}

fn ensure_memory_index(loaded: &LoadedAgentSpec) -> Result<()> {
    let path = memory_index_path(loaded);
    if path.exists() {
        return Ok(());
    }
    let memory_index = json!({
        "schema": MEMORY_INDEX_SCHEMA,
        "memory_namespace": memory_namespace(loaded),
        "append_only": true,
        "local_memory_refs": [],
        "obsmem_export_status": "not_exported"
    });
    write_json_pretty(&path, &memory_index)
}

fn acquire_lease(
    loaded: &LoadedAgentSpec,
    cycle_id: &str,
    recover_stale_lease: bool,
) -> Result<LeaseRecord> {
    let path = lease_path(loaded);
    if let Some(existing) = read_lease(loaded)? {
        if lease_is_stale(&existing) {
            if !recover_stale_lease {
                let status = status_with_state(
                    loaded,
                    AgentStatusState::Failed,
                    None,
                    None,
                    Some(existing),
                    false,
                    Some(StatusError {
                        class: "lease_stale".to_string(),
                        message: "active lease is stale; rerun with --recover-stale-lease"
                            .to_string(),
                    }),
                );
                write_status(loaded, &status)?;
                return Err(anyhow!(
                    "lease_stale: active lease is stale; rerun with --recover-stale-lease"
                ));
            }
            append_operator_event(
                loaded,
                "stale_lease_recovered",
                json!({
                    "lease_id": existing.lease_id,
                    "stale_cycle_id": existing.cycle_id,
                    "recovered_for_cycle_id": cycle_id
                }),
            )?;
            remove_lease(loaded)?;
        } else {
            let status = status_with_state(
                loaded,
                AgentStatusState::Leased,
                None,
                None,
                Some(existing),
                false,
                Some(StatusError {
                    class: "lease_active".to_string(),
                    message: "another cycle already holds the agent lease".to_string(),
                }),
            );
            write_status(loaded, &status)?;
            return Err(anyhow!(
                "lease_active: another cycle already holds the agent lease"
            ));
        }
    }

    let now = Utc::now();
    let expires_at = now
        + ChronoDuration::seconds(
            loaded
                .spec
                .heartbeat
                .stale_lease_after_secs
                .unwrap_or(900)
                .try_into()
                .unwrap_or(i64::MAX),
        );
    let lease = LeaseRecord {
        schema: LEASE_SCHEMA.to_string(),
        agent_instance_id: loaded.spec.agent_instance_id.clone(),
        lease_id: format!(
            "lease-{}-{}",
            loaded.spec.agent_instance_id,
            cycle_id.trim_start_matches("cycle-")
        ),
        cycle_id: cycle_id.to_string(),
        owner_pid: std::process::id(),
        hostname: hostname(),
        started_at: now,
        expires_at,
        status: "active".to_string(),
    };
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .with_context(|| format!("failed creating lease {}", path.display()))?;
    let raw = serde_json::to_vec_pretty(&lease)?;
    file.write_all(&raw)
        .with_context(|| format!("failed writing lease {}", path.display()))?;
    file.write_all(b"\n")
        .with_context(|| format!("failed finalizing lease {}", path.display()))?;
    Ok(lease)
}

fn write_cycle_artifacts(loaded: &LoadedAgentSpec, cycle_id: &str) -> Result<()> {
    let cycle_dir = cycles_dir(loaded).join(cycle_id);
    fs::create_dir_all(&cycle_dir)
        .with_context(|| format!("failed creating cycle dir {}", cycle_dir.display()))?;
    let started_at = Utc::now();
    let previous_cycle_id = latest_cycle_id(loaded)?;
    let workflow_ref = workflow_ref(&loaded.spec.workflow);
    let provider_binding = provider_binding(loaded, cycle_id, started_at);
    let safety_policy = effective_safety_policy(loaded);
    let workflow_supported = workflow_kind_supported(&loaded.spec.workflow.kind);
    let broker_allowed = safety_bool_default(&loaded.spec.safety, "allow_broker", false);
    let financial_advice_allowed =
        safety_bool_default(&loaded.spec.safety, "financial_advice", false);
    let outside_writes_allowed = safety_bool_default(
        &loaded.spec.safety,
        "allow_filesystem_writes_outside_state_root",
        false,
    );
    let real_world_side_effects_allowed =
        safety_bool_default(&loaded.spec.safety, "allow_real_world_side_effects", false);
    let require_sanitization = safety_bool_default(
        &loaded.spec.safety,
        "require_public_artifact_sanitization",
        true,
    );
    let mut rejected_actions = rejected_actions_for_policy(loaded);
    if !workflow_supported {
        rejected_actions.push("unsupported_workflow_kind".to_string());
    }
    dedup_strings(&mut rejected_actions);

    let observations = json!({
        "schema": OBSERVATIONS_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "cycle_id": cycle_id,
        "observed_at": started_at,
        "sources": [
            {
                "source_id": "agent_spec",
                "kind": "locked_supervisor_spec",
                "trust_level": "operator_configured",
                "artifact_ref": "../../agent_spec.locked.json"
            }
        ],
        "facts": [
            {
                "key": "workflow.kind",
                "value": loaded.spec.workflow.kind.clone(),
                "as_of": cycle_id
            },
            {
                "key": "workflow.ref",
                "value": workflow_ref.clone(),
                "as_of": cycle_id
            }
        ]
    });
    write_json_pretty(&cycle_dir.join("observations.json"), &observations)?;

    let decision_request = json!({
        "schema": DECISION_REQUEST_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "cycle_id": cycle_id,
        "agent_context_ref": "../../agent_spec.locked.json",
        "observations_ref": "observations.json",
        "memory_refs": [],
        "allowed_actions": ["record_cycle", "explain"],
        "forbidden_actions": ["execute_order", "connect_broker", "personalized_advice"],
        "not_financial_advice": true
    });
    write_json_pretty(&cycle_dir.join("decision_request.json"), &decision_request)?;

    let sanitization = if require_sanitization {
        sanitize_public_artifacts(&[
            ("observations.json", &observations),
            ("decision_request.json", &decision_request),
            ("provider_binding", &provider_binding),
        ])?
    } else {
        SanitizationResult::skipped()
    };
    if !sanitization.passed {
        rejected_actions.push("artifact_sanitization".to_string());
        dedup_strings(&mut rejected_actions);
    }

    let max_runtime_not_exceeded =
        Utc::now() <= started_at + ChronoDuration::seconds(max_cycle_runtime_secs(loaded) as i64);
    if !max_runtime_not_exceeded {
        rejected_actions.push("max_cycle_runtime_exceeded".to_string());
        dedup_strings(&mut rejected_actions);
    }

    let guardrail_pass = workflow_supported
        && rejected_actions.is_empty()
        && sanitization.passed
        && max_runtime_not_exceeded;
    let cycle_status = if guardrail_pass { "success" } else { "blocked" };
    let decision_status = if guardrail_pass {
        "accepted"
    } else {
        "rejected"
    };
    let guardrail_status = if guardrail_pass { "pass" } else { "fail" };

    let decision = if guardrail_pass {
        json!({
            "action": "record_cycle",
            "summary": "Bounded long-lived agent cycle completed under the v0.90 artifact contract.",
            "workflow_ref": workflow_ref,
            "paper_only": true
        })
    } else {
        json!({
            "action": "blocked",
            "summary": "Cycle blocked before workflow execution because the configured contract failed required guardrails.",
            "workflow_ref": workflow_ref,
            "paper_only": true
        })
    };
    let decision_result = json!({
        "schema": DECISION_RESULT_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "cycle_id": cycle_id,
        "status": decision_status,
        "decision": decision,
        "provider": {
            "source": provider_binding["provider_id"].clone(),
            "model": provider_binding["model"].clone()
        },
        "not_financial_advice": true
    });
    write_json_pretty(&cycle_dir.join("decision_result.json"), &decision_result)?;

    let run_ref = if loaded.spec.workflow.kind == "adl_workflow" {
        json!({
            "schema": RUN_REF_SCHEMA,
            "workflow_kind": "adl_workflow",
            "workflow_ref": workflow_ref,
            "run_status_ref": null,
            "trace_ref": null,
            "execution_note": "WP-03 records the cycle artifact contract; full workflow invocation remains bounded by the configured supervisor cycle."
        })
    } else {
        json!({
            "schema": RUN_REF_SCHEMA,
            "workflow_kind": loaded.spec.workflow.kind.clone(),
            "adapter": workflow_ref,
            "adapter_artifact_ref": "decision_result.json"
        })
    };
    write_json_pretty(&cycle_dir.join("run_ref.json"), &run_ref)?;

    let memory_write = json!({
        "schema": MEMORY_WRITE_SCHEMA,
        "cycle_id": cycle_id,
        "memory_id": format!("mem-{}", cycle_id.trim_start_matches("cycle-")),
        "summary": if guardrail_pass {
            "Recorded a bounded cycle artifact bundle."
        } else {
            "Recorded a blocked cycle with machine-readable guardrail evidence."
        },
        "tags": [
            format!("agent:{}", loaded.spec.agent_instance_id),
            format!("cycle:{cycle_id}"),
            "long-lived-agent",
            "paper-only"
        ],
        "source_refs": if guardrail_pass {
            json!(["decision_result.json", "cycle_manifest.json"])
        } else {
            json!(["guardrail_report.json", "cycle_manifest.json"])
        },
        "write_policy": "append_only"
    });
    write_jsonl(&cycle_dir.join("memory_writes.jsonl"), &[memory_write])?;
    update_memory_index(loaded, cycle_id)?;
    append_jsonl(&provider_binding_history_path(loaded), &provider_binding)?;

    let checks = vec![
        json!({
            "check_id": "spec_policy_loaded",
            "result": "pass",
            "policy": safety_policy
        }),
        json!({
            "check_id": "lease_valid",
            "result": "pass"
        }),
        json!({
            "check_id": "stop_not_requested",
            "result": "pass"
        }),
        json!({
            "check_id": "workflow_kind_supported",
            "result": if workflow_supported { "pass" } else { "fail" },
            "details": loaded.spec.workflow.kind.clone()
        }),
        json!({
            "check_id": "no_forbidden_action",
            "result": if rejected_actions.is_empty() { "pass" } else { "fail" },
            "rejected_actions": rejected_actions.clone()
        }),
        json!({
            "check_id": "artifact_sanitization",
            "result": if sanitization.passed { "pass" } else { "fail" },
            "findings": sanitization.findings
        }),
        json!({
            "check_id": "max_runtime_not_exceeded",
            "result": if max_runtime_not_exceeded { "pass" } else { "fail" },
            "max_cycle_runtime_secs": max_cycle_runtime_secs(loaded)
        }),
        json!({
            "check_id": "no_real_trading",
            "result": if rejected_actions.iter().any(|action| action == "execute_order" || action == "place_order") {
                "fail"
            } else {
                "pass"
            }
        }),
        json!({
            "check_id": "no_broker_integration",
            "result": if broker_allowed || rejected_actions.iter().any(|action| action == "connect_broker") {
                "fail"
            } else {
                "pass"
            }
        }),
        json!({
            "check_id": "not_financial_advice",
            "result": if financial_advice_allowed || rejected_actions.iter().any(|action| action == "personalized_advice") {
                "fail"
            } else {
                "pass"
            }
        }),
        json!({
            "check_id": "no_real_world_side_effects",
            "result": if real_world_side_effects_allowed { "fail" } else { "pass" }
        }),
        json!({
            "check_id": "writes_within_allowed_roots",
            "result": if outside_writes_allowed { "fail" } else { "pass" }
        }),
        json!({
            "check_id": "paper_only_ledger",
            "result": if rejected_actions.iter().any(|action| action == "execute_order" || action == "place_order" || action == "connect_broker") {
                "fail"
            } else {
                "pass"
            }
        }),
    ];
    let guardrail_report = json!({
        "schema": GUARDRAIL_REPORT_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "cycle_id": cycle_id,
        "status": guardrail_status,
        "checks": checks,
        "rejected_actions": rejected_actions.clone(),
        "policy_defaults": effective_safety_policy(loaded)
    });
    write_json_pretty(&cycle_dir.join("guardrail_report.json"), &guardrail_report)?;

    let completed_at = Utc::now();
    let manifest_input = json!({
        "observations_ref": "observations.json",
        "decision_request_ref": "decision_request.json",
        "previous_cycle_id": previous_cycle_id,
        "workflow_kind": loaded.spec.workflow.kind.clone(),
        "workflow_ref": workflow_ref
    });
    let manifest_output = json!({
        "decision_result_ref": "decision_result.json",
        "run_ref": "run_ref.json",
        "memory_writes_ref": "memory_writes.jsonl",
        "guardrail_report_ref": "guardrail_report.json",
        "status": cycle_status
    });
    let manifest = json!({
        "schema": CYCLE_MANIFEST_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "cycle_id": cycle_id,
        "status": cycle_status,
        "started_at": started_at,
        "completed_at": completed_at,
        "workflow_kind": loaded.spec.workflow.kind.clone(),
        "workflow_ref": manifest_input["workflow_ref"].clone(),
        "input_hash": sha256_json(&manifest_input)?,
        "output_hash": sha256_json(&manifest_output)?,
        "previous_cycle_id": manifest_input["previous_cycle_id"].clone(),
        "next_cycle_hint": "sleep_until_next_heartbeat",
        "artifacts": {
            "observations": "observations.json",
            "decision_request": "decision_request.json",
            "decision_result": "decision_result.json",
            "run_ref": "run_ref.json",
            "memory_writes": "memory_writes.jsonl",
            "guardrail_report": "guardrail_report.json",
            "cycle_summary": "cycle_summary.md"
        },
        "not_financial_advice": true
    });
    write_json_pretty(&cycle_dir.join("cycle_manifest.json"), &manifest)?;

    fs::write(
        cycle_dir.join("cycle_summary.md"),
        format!(
            "# Long-Lived Agent Cycle {cycle_id}\n\n- Agent: `{}`\n- Workflow kind: `{}`\n- Cycle status: `{cycle_status}`\n- Observations: `observations.json`\n- Decision request: `decision_request.json`\n- Decision result: `decision_result.json`\n- Guardrail result: `{guardrail_status}`\n- Memory writes: `memory_writes.jsonl`\n- Next-cycle note: `sleep_until_next_heartbeat`\n- Safety: paper-only; not financial advice; no broker execution\n",
            loaded.spec.agent_instance_id, loaded.spec.workflow.kind
        ),
    )
    .with_context(|| format!("failed writing cycle summary for {cycle_id}"))?;
    append_cycle_ledger_entry(
        loaded,
        cycle_id,
        cycle_status,
        started_at,
        completed_at,
        previous_cycle_id.as_deref(),
    )?;
    update_continuity_after_cycle(loaded, cycle_id, cycle_status)?;

    if !guardrail_pass {
        return Err(anyhow!(
            "cycle_blocked: cycle {cycle_id} failed required guardrails; see {}",
            cycle_dir.join("guardrail_report.json").display()
        ));
    }

    Ok(())
}

fn status_with_state(
    loaded: &LoadedAgentSpec,
    state: AgentStatusState,
    last_cycle_id: Option<String>,
    last_cycle_status: Option<String>,
    active_lease: Option<LeaseRecord>,
    stop_requested: bool,
    last_error: Option<StatusError>,
) -> StatusRecord {
    StatusRecord {
        schema: STATUS_SCHEMA.to_string(),
        agent_instance_id: loaded.spec.agent_instance_id.clone(),
        state,
        last_cycle_id,
        last_cycle_status,
        completed_cycle_count: completed_cycle_count(loaded).unwrap_or(0),
        consecutive_failure_count: consecutive_failure_count(loaded).unwrap_or(0),
        active_lease,
        stop_requested,
        last_error,
        safety_policy: effective_safety_policy(loaded),
        updated_at: Utc::now(),
    }
}

fn write_stop_record(
    loaded: &LoadedAgentSpec,
    reason: &str,
    requested_by: &str,
    event: &str,
) -> Result<StatusRecord> {
    let stop = StopRecord {
        schema: STOP_SCHEMA.to_string(),
        agent_instance_id: loaded.spec.agent_instance_id.clone(),
        reason: reason.to_string(),
        requested_by: requested_by.to_string(),
        mode: STOP_MODE_BEFORE_NEXT_CYCLE.to_string(),
        requested_at: Utc::now(),
    };
    write_json_pretty(&stop_path(loaded), &stop)?;
    append_operator_event(
        loaded,
        event,
        json!({
            "reason": reason,
            "mode": STOP_MODE_BEFORE_NEXT_CYCLE
        }),
    )?;
    let status = stopped_status(loaded, stop.reason);
    write_status(loaded, &status)?;
    Ok(status)
}

fn stopped_status(loaded: &LoadedAgentSpec, reason: String) -> StatusRecord {
    status_with_state(
        loaded,
        AgentStatusState::Stopped,
        read_status(loaded)
            .ok()
            .flatten()
            .and_then(|s| s.last_cycle_id),
        read_status(loaded)
            .ok()
            .flatten()
            .and_then(|s| s.last_cycle_status),
        None,
        true,
        Some(StatusError {
            class: "operator_stop_requested".to_string(),
            message: reason,
        }),
    )
}

fn next_cycle_id(loaded: &LoadedAgentSpec) -> Result<String> {
    let latest = ledger_cursor(loaded)?.max_cycle_number;
    let directory_latest = completed_cycle_count_from_dirs(loaded)?;
    let next = latest.max(directory_latest) + 1;
    Ok(format!("cycle-{number:06}", number = next))
}

fn latest_cycle_id(loaded: &LoadedAgentSpec) -> Result<Option<String>> {
    Ok(ledger_cursor(loaded)?.latest_cycle_id)
}

fn completed_cycle_count(loaded: &LoadedAgentSpec) -> Result<u64> {
    let ledger = ledger_cursor(loaded)?;
    if ledger.count > 0 {
        return Ok(ledger.count);
    }
    completed_cycle_count_from_dirs(loaded)
}

fn consecutive_failure_count(loaded: &LoadedAgentSpec) -> Result<u64> {
    let path = cycle_ledger_path(loaded);
    if !path.exists() {
        return Ok(0);
    }
    let file = File::open(&path)
        .with_context(|| format!("failed opening cycle ledger {}", path.display()))?;
    let mut statuses = Vec::new();
    for line in BufReader::new(file).lines() {
        let line =
            line.with_context(|| format!("failed reading cycle ledger {}", path.display()))?;
        if line.trim().is_empty() {
            continue;
        }
        let value: Value = serde_json::from_str(&line)
            .with_context(|| format!("failed parsing cycle ledger {}", path.display()))?;
        if let Some(status) = value.get("status").and_then(Value::as_str) {
            statuses.push(status.to_string());
        }
    }
    Ok(statuses
        .iter()
        .rev()
        .take_while(|status| status.as_str() != "success")
        .count() as u64)
}

fn completed_cycle_count_from_dirs(loaded: &LoadedAgentSpec) -> Result<u64> {
    let dir = cycles_dir(loaded);
    if !dir.exists() {
        return Ok(0);
    }
    let mut max_seen = 0u64;
    for entry in fs::read_dir(&dir).with_context(|| format!("failed reading {}", dir.display()))? {
        let entry = entry?;
        let Some(name) = entry.file_name().to_str().map(|s| s.to_string()) else {
            continue;
        };
        if let Some(number) = name.strip_prefix("cycle-") {
            if let Ok(parsed) = number.parse::<u64>() {
                max_seen = max_seen.max(parsed);
            }
        }
    }
    Ok(max_seen)
}

fn ledger_cursor(loaded: &LoadedAgentSpec) -> Result<LedgerCursor> {
    let path = cycle_ledger_path(loaded);
    if !path.exists() {
        return Ok(LedgerCursor::default());
    }
    let file = File::open(&path)
        .with_context(|| format!("failed opening cycle ledger {}", path.display()))?;
    let mut cursor = LedgerCursor::default();
    for line in BufReader::new(file).lines() {
        let line =
            line.with_context(|| format!("failed reading cycle ledger {}", path.display()))?;
        if line.trim().is_empty() {
            continue;
        }
        let value: Value = serde_json::from_str(&line)
            .with_context(|| format!("failed parsing cycle ledger {}", path.display()))?;
        cursor.count += 1;
        let Some(cycle_id) = value.get("cycle_id").and_then(Value::as_str) else {
            continue;
        };
        let Some(number) = cycle_number(cycle_id) else {
            continue;
        };
        if number >= cursor.max_cycle_number {
            cursor.max_cycle_number = number;
            cursor.latest_cycle_id = Some(cycle_id.to_string());
            cursor.latest_status = value
                .get("status")
                .and_then(Value::as_str)
                .map(str::to_string);
        }
    }
    Ok(cursor)
}

fn cycle_number(cycle_id: &str) -> Option<u64> {
    cycle_id.strip_prefix("cycle-")?.parse::<u64>().ok()
}

fn append_cycle_ledger_entry(
    loaded: &LoadedAgentSpec,
    cycle_id: &str,
    status: &str,
    started_at: DateTime<Utc>,
    completed_at: DateTime<Utc>,
    previous_cycle_id: Option<&str>,
) -> Result<()> {
    let entry = json!({
        "schema": CYCLE_LEDGER_ENTRY_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "cycle_id": cycle_id,
        "status": status,
        "started_at": started_at,
        "completed_at": completed_at,
        "previous_cycle_id": previous_cycle_id,
        "manifest_ref": format!("cycles/{cycle_id}/cycle_manifest.json"),
        "summary_ref": format!("cycles/{cycle_id}/cycle_summary.md"),
        "memory_writes_ref": format!("cycles/{cycle_id}/memory_writes.jsonl"),
        "guardrail_report_ref": format!("cycles/{cycle_id}/guardrail_report.json"),
        "continuity_kind": "pre_v0_92_handle"
    });
    append_jsonl(&cycle_ledger_path(loaded), &entry)
}

fn update_continuity_after_cycle(
    loaded: &LoadedAgentSpec,
    cycle_id: &str,
    cycle_status: &str,
) -> Result<()> {
    ensure_continuity(loaded)?;
    let path = continuity_path(loaded);
    let mut continuity: Value = read_json_required(&path)?;
    continuity["latest_cycle_id"] = json!(cycle_id);
    continuity["latest_cycle_status"] = json!(cycle_status);
    continuity["status"] = json!("active");
    continuity["updated_at"] = json!(Utc::now());
    write_json_pretty(&path, &continuity)
}

fn update_memory_index(loaded: &LoadedAgentSpec, cycle_id: &str) -> Result<()> {
    ensure_memory_index(loaded)?;
    let path = memory_index_path(loaded);
    let mut index: Value = read_json_required(&path)?;
    let memory_ref = format!("cycles/{cycle_id}/memory_writes.jsonl");
    let refs = index
        .get_mut("local_memory_refs")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| anyhow!("memory index local_memory_refs must be an array"))?;
    if !refs.iter().any(|value| value.as_str() == Some(&memory_ref)) {
        refs.push(json!(memory_ref));
    }
    write_json_pretty(&path, &index)
}

fn provider_binding(loaded: &LoadedAgentSpec, cycle_id: &str, bound_at: DateTime<Utc>) -> Value {
    let provider_id = loaded
        .spec
        .workflow
        .run_args
        .get("provider_id")
        .or_else(|| loaded.spec.workflow.run_args.get("provider"))
        .and_then(Value::as_str)
        .map(str::to_string);
    let model = loaded
        .spec
        .workflow
        .run_args
        .get("model")
        .and_then(Value::as_str)
        .map(str::to_string);
    let binding_status = if provider_id.is_some() || model.is_some() {
        "available"
    } else {
        "not_available"
    };
    json!({
        "schema": PROVIDER_BINDING_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "cycle_id": cycle_id,
        "provider_id": provider_id.unwrap_or_else(|| loaded.spec.workflow.kind.clone()),
        "model": model,
        "binding_status": binding_status,
        "source": if binding_status == "available" {
            "workflow_run_args"
        } else {
            "workflow_kind_fallback"
        },
        "bound_at": bound_at
    })
}

fn lease_is_stale(lease: &LeaseRecord) -> bool {
    lease.status == "active" && lease.expires_at <= Utc::now()
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
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("failed reading inspection artifact {artifact_ref}"))?;
    serde_json::from_str(&raw)
        .with_context(|| format!("failed parsing inspection artifact {artifact_ref}"))
}

fn read_state_text_artifact(loaded: &LoadedAgentSpec, artifact_ref: &str) -> Result<String> {
    let path = loaded.state_root.join(artifact_ref);
    if !path.exists() {
        return Err(anyhow!("inspection artifact missing: {artifact_ref}"));
    }
    fs::read_to_string(&path)
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

fn sha256_json(value: &Value) -> Result<String> {
    let bytes = serde_json::to_vec(value)?;
    let digest = Sha256::digest(bytes);
    Ok(format!("sha256:{digest:x}"))
}

fn workflow_kind_supported(kind: &str) -> bool {
    matches!(kind, "demo_adapter" | "adl_workflow")
}

fn workflow_ref(workflow: &WorkflowSpec) -> String {
    workflow
        .name
        .clone()
        .or_else(|| workflow.path.as_deref().map(path_artifact_ref))
        .unwrap_or_else(|| workflow.kind.clone())
}

#[derive(Debug, Clone)]
struct SanitizationResult {
    passed: bool,
    findings: Vec<Value>,
}

impl SanitizationResult {
    fn skipped() -> Self {
        Self {
            passed: true,
            findings: vec![json!({
                "status": "skipped",
                "reason": "require_public_artifact_sanitization is false"
            })],
        }
    }
}

fn sanitize_public_artifacts(artifacts: &[(&str, &Value)]) -> Result<SanitizationResult> {
    let mut findings = Vec::new();
    let banned = [
        ("host_path", "/users/"),
        ("bearer_token", "bearer "),
        ("private_key", "private key"),
        ("api_key", "api_key"),
        ("api_key", "api key"),
        ("broker_account", "broker_account"),
        ("broker_token", "broker_token"),
        ("personal_portfolio", "personal_portfolio"),
        ("personal_risk_profile", "personal_risk_profile"),
    ];
    for (artifact, value) in artifacts {
        let raw = serde_json::to_string(value)
            .with_context(|| format!("failed serializing public artifact {artifact}"))?;
        let lower = raw.to_ascii_lowercase();
        for (finding, needle) in &banned {
            if lower.contains(needle) {
                findings.push(json!({
                    "artifact": artifact,
                    "finding": finding,
                    "pattern": needle
                }));
            }
        }
    }
    Ok(SanitizationResult {
        passed: findings.is_empty(),
        findings,
    })
}

fn rejected_actions_for_policy(loaded: &LoadedAgentSpec) -> Vec<String> {
    let mut rejected = Vec::new();
    let run_args = &loaded.spec.workflow.run_args;
    for action in requested_actions(run_args) {
        match action.to_ascii_lowercase().as_str() {
            "execute_order" | "place_order" | "trade" | "buy" | "sell" => {
                rejected.push("execute_order".to_string());
            }
            "connect_broker" | "broker_connect" => {
                rejected.push("connect_broker".to_string());
            }
            "personalized_advice" | "financial_advice" | "recommend_to_user" => {
                rejected.push("personalized_advice".to_string());
            }
            _ => {}
        }
    }
    if safety_bool_default(&loaded.spec.safety, "allow_broker", false)
        || contains_any_key(
            run_args,
            &[
                "broker_url",
                "broker_account_id",
                "broker_token",
                "broker_api_key",
                "broker_credentials",
            ],
        )
    {
        rejected.push("connect_broker".to_string());
    }
    if safety_bool_default(&loaded.spec.safety, "financial_advice", false)
        || contains_any_key(
            run_args,
            &[
                "personal_portfolio",
                "personal_risk_profile",
                "personal_assets",
                "private_portfolio_data",
            ],
        )
    {
        rejected.push("personalized_advice".to_string());
    }
    if safety_bool_default(&loaded.spec.safety, "allow_real_world_side_effects", false) {
        rejected.push("real_world_side_effect".to_string());
    }
    if safety_bool_default(
        &loaded.spec.safety,
        "allow_filesystem_writes_outside_state_root",
        false,
    ) {
        rejected.push("writes_outside_allowed_roots".to_string());
    }
    dedup_strings(&mut rejected);
    rejected
}

fn requested_actions(value: &Value) -> Vec<String> {
    let mut actions = Vec::new();
    collect_requested_actions(value, &mut actions);
    dedup_strings(&mut actions);
    actions
}

fn collect_requested_actions(value: &Value, actions: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                if matches!(
                    key.as_str(),
                    "action" | "requested_action" | "tool" | "tool_name"
                ) {
                    if let Some(action) = value.as_str() {
                        actions.push(action.to_string());
                    }
                }
                if key == "actions" {
                    if let Some(items) = value.as_array() {
                        for item in items {
                            if let Some(action) = item.as_str() {
                                actions.push(action.to_string());
                            }
                        }
                    }
                }
                collect_requested_actions(value, actions);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_requested_actions(item, actions);
            }
        }
        _ => {}
    }
}

fn contains_any_key(value: &Value, keys: &[&str]) -> bool {
    match value {
        Value::Object(map) => map.iter().any(|(key, value)| {
            keys.iter().any(|candidate| key == candidate) || contains_any_key(value, keys)
        }),
        Value::Array(items) => items.iter().any(|item| contains_any_key(item, keys)),
        _ => false,
    }
}

fn dedup_strings(values: &mut Vec<String>) {
    let mut seen = Vec::new();
    values.retain(|value| {
        if seen.contains(value) {
            false
        } else {
            seen.push(value.clone());
            true
        }
    });
}

fn path_artifact_ref(path: &Path) -> String {
    if path.is_absolute() {
        return path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| format!("absolute-path-redacted/{name}"))
            .unwrap_or_else(|| "absolute-path-redacted/workflow".to_string());
    }
    path.to_string_lossy().to_string()
}

fn safety_bool_default(safety: &Value, key: &str, default: bool) -> bool {
    safety.get(key).and_then(Value::as_bool).unwrap_or(default)
}

fn safety_u64(safety: &Value, key: &str, default: u64) -> u64 {
    safety.get(key).and_then(Value::as_u64).unwrap_or(default)
}

fn effective_safety_policy(loaded: &LoadedAgentSpec) -> Value {
    json!({
        "allow_network": safety_bool_default(&loaded.spec.safety, "allow_network", false),
        "allow_broker": safety_bool_default(&loaded.spec.safety, "allow_broker", false),
        "allow_filesystem_writes_outside_state_root": safety_bool_default(
            &loaded.spec.safety,
            "allow_filesystem_writes_outside_state_root",
            false,
        ),
        "allow_real_world_side_effects": safety_bool_default(
            &loaded.spec.safety,
            "allow_real_world_side_effects",
            false,
        ),
        "require_public_artifact_sanitization": safety_bool_default(
            &loaded.spec.safety,
            "require_public_artifact_sanitization",
            true,
        ),
        "financial_advice": safety_bool_default(&loaded.spec.safety, "financial_advice", false),
        "max_cycle_runtime_secs": safety_u64(
            &loaded.spec.safety,
            "max_cycle_runtime_secs",
            DEFAULT_MAX_CYCLE_RUNTIME_SECS,
        ),
        "max_consecutive_failures": max_consecutive_failures(loaded)
    })
}

fn max_cycle_runtime_secs(loaded: &LoadedAgentSpec) -> u64 {
    safety_u64(
        &loaded.spec.safety,
        "max_cycle_runtime_secs",
        DEFAULT_MAX_CYCLE_RUNTIME_SECS,
    )
}

fn max_consecutive_failures(loaded: &LoadedAgentSpec) -> u64 {
    safety_u64(
        &loaded.spec.safety,
        "max_consecutive_failures",
        DEFAULT_MAX_CONSECUTIVE_FAILURES,
    )
}

fn default_requested_by() -> String {
    "operator".to_string()
}

fn default_stop_mode() -> String {
    STOP_MODE_BEFORE_NEXT_CYCLE.to_string()
}

fn memory_namespace(loaded: &LoadedAgentSpec) -> String {
    loaded
        .spec
        .memory
        .get("namespace")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| loaded.spec.agent_instance_id.clone())
}

fn hostname() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "local".to_string())
}

#[cfg(test)]
mod tests;
