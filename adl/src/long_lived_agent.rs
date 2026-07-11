//! Long-lived agent orchestration surfaces.
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use crate::chronosense::{
    capture_runtime_time_sync_status, start_runtime_time_observation, ChronosenseRuntimeService,
    ChronosenseRuntimeServiceConfig,
};
use crate::csm_freedom_gate;
use crate::csm_godel_snapshot::{validate_recovery_read, write_checkpoint_snapshot_diff};
use crate::csm_resident_agents;
use crate::csm_runtime_api::{serve_runtime_api, CsmRuntimeApiOptions};
use crate::csm_shepherd_agent;
use crate::runtime_aws_signal::publish_csm_governed_notice_signal;
use crate::{adl, execute, resolve, trace};

mod inspection;
mod schema;
mod storage;
mod types;

use schema::*;
use storage::*;
use types::LedgerCursor;
pub use types::{
    AgentCheckpointSpec, AgentSpec, AgentStatusState, DaemonOptions, DaemonStatusRecord,
    HeartbeatSpec, InspectOptions, LeaseRecord, LoadedAgentSpec, RunOptions, StatusError,
    StatusRecord, StopRecord, TickOptions, WorkflowSpec,
};

const DAEMON_DEFAULT_INTERVAL_SECS: u64 = 3;

fn utc_now() -> DateTime<Utc> {
    Utc::now()
}

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
        persist_status(&loaded, &status, "stop_observed_before_cycle")?;
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
    persist_status(&loaded, &running, "cycle_running")?;

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
            persist_status(&loaded, &status, "cycle_completed")?;
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
            persist_status(&loaded, &status, "cycle_failed")?;
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

    build_long_lived_runtime()?.block_on(run_with_tokio_cadence(
        spec_path.to_path_buf(),
        options,
        loaded,
        sleep_secs,
    ))
}

pub fn daemon(spec_path: &Path, options: DaemonOptions) -> Result<DaemonStatusRecord> {
    if options.checkpoint_interval_secs == 0 {
        return Err(anyhow!(
            "csm daemon requires --checkpoint-interval-secs greater than zero"
        ));
    }
    if options.interval_secs == Some(0) {
        return Err(anyhow!(
            "csm daemon requires --interval-secs greater than zero"
        ));
    }
    let loaded = load_spec(spec_path)?;
    ensure_state_root(&loaded)?;
    let checkpoint_interval_secs =
        effective_checkpoint_interval_secs(&loaded, options.checkpoint_interval_secs)?;
    let runtime_context = CsmRuntimeContext::new()?;
    let runtime_api_shutdown = embedded_runtime_api_shutdown_path(&loaded);
    let _runtime_api_shutdown_guard = RuntimeApiShutdownGuard {
        path: runtime_api_shutdown.clone(),
    };
    let runtime_api_status =
        start_embedded_runtime_api_module(spec_path, &options, &runtime_api_shutdown);
    let mut restart_count = 0u64;
    let mut last_child_exit = None;
    let _ = write_daemon_status(
        &runtime_context,
        &loaded,
        DaemonStatusInput {
            state: "starting",
            bounded_test_mode: options.no_sleep,
            restart_count,
            bounded_test_restart_limit: options.bounded_test_restart_limit,
            checkpoint_interval_secs,
            last_event: "daemon_started",
            last_child_exit: None,
            next_backoff_secs: 0,
        },
    )?;
    let mut daemon_status: DaemonStatusRecord;
    emit_daemon_event(
        &runtime_context,
        &loaded,
        "daemon_started",
        "started",
        restart_count,
        json!({
                "checkpoint_interval_secs": checkpoint_interval_secs,
                "agent_checkpoint_policy": agent_checkpoint_policy(&loaded),
                "bounded_test_restart_limit": options.bounded_test_restart_limit,
                "restart_policy": daemon_restart_policy(),
                "service_mode": daemon_service_mode(options.no_sleep),
                "bounded_test_mode": options.no_sleep,
            "cycle_count_lifetime_boundary": "not_applicable",
            "runtime_api": runtime_api_status,
            "unsupported_permanence_claims": unsupported_permanence_claims()
        }),
    )?;

    loop {
        if let Some(stop) = read_stop(&loaded)? {
            let status = status(spec_path)?;
            persist_status(&loaded, &status, "daemon_stop_observed")?;
            let governed_stop = stop.classification == "governed_emergency_stop_recorded";
            let daemon_state = if governed_stop {
                "governed_stopped"
            } else {
                "stopped"
            };
            let daemon_event = if governed_stop {
                "governed_emergency_stop_recorded"
            } else {
                "stop_completed"
            };
            let daemon_status = write_daemon_status(
                &runtime_context,
                &loaded,
                DaemonStatusInput {
                    state: daemon_state,
                    bounded_test_mode: options.no_sleep,
                    restart_count,
                    bounded_test_restart_limit: options.bounded_test_restart_limit,
                    checkpoint_interval_secs,
                    last_event: daemon_event,
                    last_child_exit: last_child_exit.clone(),
                    next_backoff_secs: 0,
                },
            )?;
            let safe_fail = record_safe_fail_event(
                &runtime_context,
                &loaded,
                SafeFailRecord {
                    status: &status,
                    trigger: "graceful_stop",
                    restart_count,
                    bounded_test_restart_limit: options.bounded_test_restart_limit,
                    last_child_exit: last_child_exit.clone(),
                    details: json!({"stop_ref": "stop.json"}),
                },
            )?;
            let governed_notice = record_governed_runtime_notice(
                &runtime_context,
                &loaded,
                GovernedNoticeInput {
                    notice_kind: "graceful_shutdown",
                    severity: "operator_notice",
                    trigger: "graceful_stop",
                    status: &status,
                    restart_count,
                    bounded_test_restart_limit: options.bounded_test_restart_limit,
                    last_child_exit: last_child_exit.clone(),
                    safe_fail: safe_fail.clone(),
                    details: json!({"stop_ref": "stop.json"}),
                },
            )?;
            emit_daemon_event(
                &runtime_context,
                &loaded,
                daemon_event,
                "completed",
                restart_count,
                json!({
                    "recoverable_state": status.state,
                    "safe_fail": safe_fail,
                    "governed_notice": governed_notice
                }),
            )?;
            return Ok(daemon_status);
        }

        emit_daemon_event(
            &runtime_context,
            &loaded,
            "child_spawn",
            "started",
            restart_count,
            json!({"supervised_unit": "long_lived_agent_tick"}),
        )?;
        let _ = write_daemon_status(
            &runtime_context,
            &loaded,
            DaemonStatusInput {
                state: "running",
                bounded_test_mode: options.no_sleep,
                restart_count,
                bounded_test_restart_limit: options.bounded_test_restart_limit,
                checkpoint_interval_secs,
                last_event: "child_spawn",
                last_child_exit: last_child_exit.clone(),
                next_backoff_secs: 0,
            },
        )?;

        match tick(
            spec_path,
            TickOptions {
                recover_stale_lease: options.recover_stale_lease,
            },
        ) {
            Ok(status) => {
                last_child_exit = Some("success".to_string());
                emit_daemon_event(
                    &runtime_context,
                    &loaded,
                    "child_exit",
                    "completed",
                    restart_count,
                    json!({
                        "exit_class": "success",
                        "recoverable_state": status.state,
                        "checkpoint_ref": "continuity_checkpoint.json"
                    }),
                )?;
                daemon_status = write_daemon_status(
                    &runtime_context,
                    &loaded,
                    DaemonStatusInput {
                        state: "running",
                        bounded_test_mode: options.no_sleep,
                        restart_count,
                        bounded_test_restart_limit: options.bounded_test_restart_limit,
                        checkpoint_interval_secs,
                        last_event: "child_exit",
                        last_child_exit: last_child_exit.clone(),
                        next_backoff_secs: 0,
                    },
                )?;
            }
            Err(err) => {
                last_child_exit = Some(format!("error:{err}"));
                let mut status = read_status(&loaded)?.unwrap_or_else(|| {
                    status_with_state(
                        &loaded,
                        AgentStatusState::Failed,
                        None,
                        None,
                        None,
                        false,
                        None,
                    )
                });
                status.state = AgentStatusState::Failed;
                status.active_lease = read_lease(&loaded)?;
                status.last_error = Some(StatusError {
                    class: "daemon_child_failed".to_string(),
                    message: err.to_string(),
                });
                status.updated_at = Utc::now();
                persist_status(&loaded, &status, "daemon_child_failed_recoverable")?;
                emit_daemon_event(
                    &runtime_context,
                    &loaded,
                    "child_exit",
                    "failed",
                    restart_count,
                    json!({
                        "exit_class": "error",
                        "error": err.to_string(),
                        "recoverable_state": status.state,
                        "checkpoint_ref": "continuity_checkpoint.json"
                    }),
                )?;
                let child_safe_fail = record_safe_fail_event(
                    &runtime_context,
                    &loaded,
                    SafeFailRecord {
                        status: &status,
                        trigger: "daemon_child_failed",
                        restart_count,
                        bounded_test_restart_limit: options.bounded_test_restart_limit,
                        last_child_exit: last_child_exit.clone(),
                        details: json!({
                            "error": err.to_string(),
                            "checkpoint_ref": "continuity_checkpoint.json"
                        }),
                    },
                )?;
                let child_notice = record_governed_runtime_notice(
                    &runtime_context,
                    &loaded,
                    GovernedNoticeInput {
                        notice_kind: "degradation",
                        severity: "warning",
                        trigger: "daemon_child_failed",
                        status: &status,
                        restart_count,
                        bounded_test_restart_limit: options.bounded_test_restart_limit,
                        last_child_exit: last_child_exit.clone(),
                        safe_fail: child_safe_fail.clone(),
                        details: json!({
                            "error_class": "daemon_child_failed",
                            "checkpoint_ref": "continuity_checkpoint.json"
                        }),
                    },
                )?;
                if options
                    .bounded_test_restart_limit
                    .is_some_and(|limit| restart_count >= limit)
                {
                    let _ = write_daemon_status(
                        &runtime_context,
                        &loaded,
                        DaemonStatusInput {
                            state: "failed",
                            bounded_test_mode: options.no_sleep,
                            restart_count,
                            bounded_test_restart_limit: options.bounded_test_restart_limit,
                            checkpoint_interval_secs,
                            last_event: "bounded_test_supervisor_failure",
                            last_child_exit: last_child_exit.clone(),
                            next_backoff_secs: 0,
                        },
                    )?;
                    let supervisor_failure_safe_fail = record_safe_fail_event(
                        &runtime_context,
                        &loaded,
                        SafeFailRecord {
                            status: &status,
                            trigger: "bounded_test_supervisor_failure",
                            restart_count,
                            bounded_test_restart_limit: options.bounded_test_restart_limit,
                            last_child_exit: last_child_exit.clone(),
                            details: json!({
                                "previous_safe_fail": child_safe_fail,
                                "previous_notice": child_notice.clone(),
                                "checkpoint_ref": "continuity_checkpoint.json"
                            }),
                        },
                    )?;
                    let exhausted_notice = record_governed_runtime_notice(
                        &runtime_context,
                        &loaded,
                        GovernedNoticeInput {
                            notice_kind: "shutdown",
                            severity: "critical",
                            trigger: "bounded_test_supervisor_failure",
                            status: &status,
                            restart_count,
                            bounded_test_restart_limit: options.bounded_test_restart_limit,
                            last_child_exit: last_child_exit.clone(),
                            safe_fail: supervisor_failure_safe_fail.clone(),
                            details: json!({
                                "previous_notice": child_notice,
                                "checkpoint_ref": "continuity_checkpoint.json"
                            }),
                        },
                    )?;
                    emit_daemon_event(
                        &runtime_context,
                        &loaded,
                        "bounded_test_supervisor_failure",
                        "failed",
                        restart_count,
                        json!({
                            "recoverable_state": status.state,
                            "safe_fail": supervisor_failure_safe_fail,
                            "governed_notice": exhausted_notice
                        }),
                    )?;
                    return Err(err.context("daemon bounded test supervisor failure"));
                }
                restart_count += 1;
                let backoff_secs = restart_backoff_secs(restart_count);
                daemon_status = write_daemon_status(
                    &runtime_context,
                    &loaded,
                    DaemonStatusInput {
                        state: "restarting",
                        bounded_test_mode: options.no_sleep,
                        restart_count,
                        bounded_test_restart_limit: options.bounded_test_restart_limit,
                        checkpoint_interval_secs,
                        last_event: "restart_scheduled",
                        last_child_exit: last_child_exit.clone(),
                        next_backoff_secs: backoff_secs,
                    },
                )?;
                emit_daemon_event(
                    &runtime_context,
                    &loaded,
                    "restart_scheduled",
                    "scheduled",
                    restart_count,
                    json!({"backoff_secs": backoff_secs}),
                )?;
                let stop_observed = sleep_with_partial_checkpoints(
                    &runtime_context,
                    &loaded,
                    &mut daemon_status,
                    PartialCheckpointSleep {
                        total_sleep_secs: backoff_secs,
                        checkpoint_interval_secs,
                        restart_count,
                        bounded_test_restart_limit: options.bounded_test_restart_limit,
                        last_child_exit: last_child_exit.clone(),
                        recoverable_error: status.last_error.clone(),
                        event: "restart_backoff",
                        no_sleep: options.no_sleep,
                    },
                )?;
                if stop_observed {
                    continue;
                }
                emit_daemon_event(
                    &runtime_context,
                    &loaded,
                    "restart_attempted",
                    "started",
                    restart_count,
                    json!({"previous_exit": last_child_exit}),
                )?;
                continue;
            }
        }

        let sleep_secs = daemon_interval_secs(&loaded, options.interval_secs)?;
        let stop_observed = sleep_with_partial_checkpoints(
            &runtime_context,
            &loaded,
            &mut daemon_status,
            PartialCheckpointSleep {
                total_sleep_secs: sleep_secs,
                checkpoint_interval_secs,
                restart_count,
                bounded_test_restart_limit: options.bounded_test_restart_limit,
                last_child_exit: last_child_exit.clone(),
                recoverable_error: None,
                event: "daemon_heartbeat",
                no_sleep: options.no_sleep,
            },
        )?;
        if stop_observed {
            continue;
        }
        if options.no_sleep {
            daemon_status = write_daemon_status(
                &runtime_context,
                &loaded,
                DaemonStatusInput {
                    state: "completed",
                    bounded_test_mode: true,
                    restart_count,
                    bounded_test_restart_limit: options.bounded_test_restart_limit,
                    checkpoint_interval_secs,
                    last_event: "daemon_completed",
                    last_child_exit: last_child_exit.clone(),
                    next_backoff_secs: 0,
                },
            )?;
            emit_daemon_event(
                &runtime_context,
                &loaded,
                "daemon_completed",
                "completed",
                restart_count,
                json!({
                    "reason": "no_sleep_test_boundary",
                    "restart_policy": daemon_restart_policy(),
                    "service_mode": "bounded_test_only",
                    "bounded_test_mode": true
                }),
            )?;
            return Ok(daemon_status);
        }
    }
}

fn start_embedded_runtime_api_module(
    spec_path: &Path,
    options: &DaemonOptions,
    shutdown_file: &Path,
) -> Value {
    let Some(bind) = options.api_bind.clone() else {
        return json!({
            "status": "disabled",
            "reason": "api_bind_not_configured"
        });
    };
    let _ = fs::remove_file(shutdown_file);
    let api_options = CsmRuntimeApiOptions {
        spec_path: spec_path.to_path_buf(),
        bind: bind.clone(),
        test_max_requests: None,
        idle_timeout_ms: None,
        shutdown_file: Some(shutdown_file.to_path_buf()),
        otel_status_path: options.api_otel_status_path.clone(),
        otel_log_path: options.api_otel_log_path.clone(),
    };
    thread::Builder::new()
        .name("csm-runtime-api".to_string())
        .spawn(move || {
            if let Err(err) = serve_runtime_api(api_options) {
                let error = err
                    .to_string()
                    .replace(['\n', '\r', '\t'], " ")
                    .replace(' ', "_");
                eprintln!(
                    "adl_event schema=adl.observability.event.v1 command=csm stage=runtime_api_embedded result=failed error={error}"
                );
            }
        })
        .map(|_| {
            json!({
                "status": "embedded",
                "bind": bind,
                "thread": "csm-runtime-api",
                "pid_model": "same_process_as_csm_daemon",
                "shutdown_ref": path_artifact_ref(shutdown_file)
            })
        })
        .unwrap_or_else(|err| {
            json!({
                "status": "failed_to_start",
                "bind": bind,
                "error": err.to_string()
            })
        })
}

struct RuntimeApiShutdownGuard {
    path: PathBuf,
}

impl Drop for RuntimeApiShutdownGuard {
    fn drop(&mut self) {
        if let Some(parent) = self.path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(&self.path, "shutdown\n");
    }
}

fn embedded_runtime_api_shutdown_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("csm_runtime_api_shutdown")
}

fn daemon_interval_secs(loaded: &LoadedAgentSpec, override_secs: Option<u64>) -> Result<u64> {
    match override_secs.or(loaded.spec.heartbeat.interval_secs) {
        Some(0) => Err(anyhow!(
            "csm daemon requires heartbeat.interval_secs or --interval-secs greater than zero"
        )),
        Some(secs) => Ok(secs),
        None => Ok(DAEMON_DEFAULT_INTERVAL_SECS),
    }
}

fn effective_checkpoint_interval_secs(
    loaded: &LoadedAgentSpec,
    daemon_interval_secs: u64,
) -> Result<u64> {
    if daemon_interval_secs == 0 {
        return Err(anyhow!(
            "csm daemon requires --checkpoint-interval-secs greater than zero"
        ));
    }
    match loaded.spec.checkpoint.interval_secs {
        Some(0) => Err(anyhow!(
            "agent spec checkpoint.interval_secs must be greater than zero"
        )),
        Some(agent_interval) => Ok(daemon_interval_secs.min(agent_interval)),
        None => Ok(daemon_interval_secs),
    }
}

async fn run_with_tokio_cadence(
    spec_path: PathBuf,
    options: RunOptions,
    loaded: LoadedAgentSpec,
    sleep_secs: u64,
) -> Result<StatusRecord> {
    let mut last_status = status(&spec_path)?;
    for index in 0..options.max_cycles {
        if read_stop(&loaded)?.is_some() {
            last_status = run_tick_blocking(spec_path.clone(), options.recover_stale_lease).await?;
            break;
        }
        match run_tick_blocking(spec_path.clone(), options.recover_stale_lease).await {
            Ok(status) => {
                last_status = status;
            }
            Err(err) => {
                last_status = status(&spec_path)?;
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
            tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
        }
    }

    if last_status.state != AgentStatusState::Stopped {
        last_status.state = AgentStatusState::Completed;
        last_status.updated_at = Utc::now();
        persist_status(&loaded, &last_status, "run_completed")?;
    }
    Ok(last_status)
}

async fn run_tick_blocking(spec_path: PathBuf, recover_stale_lease: bool) -> Result<StatusRecord> {
    tokio::task::spawn_blocking(move || {
        tick(
            &spec_path,
            TickOptions {
                recover_stale_lease,
            },
        )
    })
    .await
    .map_err(|err| anyhow!("long-lived agent cadence task failed to join: {err}"))?
}

fn build_long_lived_runtime() -> Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .context("failed building Tokio runtime for long-lived agent cadence")
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

    let stop = read_stop(&loaded)?;
    let lease = read_lease(&loaded)?;
    current.state = derive_visible_status_state(
        current.state.clone(),
        stop.is_some(),
        coordination_lease_state(lease.as_ref()),
    );
    current.stop_requested = stop.is_some();
    current.active_lease = if matches!(
        coordination_lease_state(lease.as_ref()),
        CoordinationLeaseState::Active | CoordinationLeaseState::Stale
    ) && current.state != AgentStatusState::Stopped
    {
        lease
    } else {
        None
    };
    current.last_error = if let Some(stop) = stop {
        Some(StatusError {
            class: "operator_stop_requested".to_string(),
            message: stop.reason,
        })
    } else if matches!(
        coordination_lease_state(current.active_lease.as_ref()),
        CoordinationLeaseState::Stale
    ) {
        Some(StatusError {
            class: "lease_stale".to_string(),
            message: "active lease is stale and requires explicit recovery".to_string(),
        })
    } else {
        None
    };
    current.updated_at = Utc::now();
    persist_status(&loaded, &current, "status_refreshed")?;
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

pub fn stop_requested(spec_path: &Path) -> Result<bool> {
    let loaded = load_spec(spec_path)?;
    ensure_state_root(&loaded)?;
    Ok(read_stop(&loaded)?.is_some())
}

pub fn clear_stop_for_service_start(spec_path: &Path, requested_by: &str) -> Result<Value> {
    let loaded = load_spec(spec_path)?;
    ensure_state_root(&loaded)?;
    let existing = read_stop(&loaded)?;
    if existing.is_some() {
        remove_stop(&loaded)?;
    }
    append_operator_event(
        &loaded,
        "service_start_cleared_stop_intent",
        json!({
            "requested_by": requested_by,
            "had_stop_intent": existing.is_some(),
            "stop_ref": "stop.json",
            "reason": "csm service start requested a new runtime lifetime"
        }),
    )?;
    if existing.is_some() {
        let status = status_with_state(
            &loaded,
            AgentStatusState::Idle,
            None,
            None,
            None,
            false,
            None,
        );
        persist_status(&loaded, &status, "service_start_cleared_stop")?;
    }
    Ok(json!({
        "schema": "adl.csm.service_start_clear_stop.v1",
        "status": "completed",
        "had_stop_intent": existing.is_some(),
        "stop_ref": "stop.json"
    }))
}

#[derive(Debug, Clone)]
pub struct GovernedStopRequest {
    pub reason: String,
    pub operator_identity: String,
    pub authorization: String,
    pub intent: String,
    pub requested_at: DateTime<Utc>,
}

pub fn governed_stop(spec_path: &Path, request: GovernedStopRequest) -> Result<Value> {
    validate_governed_stop_request(&request)?;
    let loaded = load_spec(spec_path)?;
    ensure_state_root(&loaded)?;
    let runtime_context = CsmRuntimeContext::new()?;
    let restart_count = daemon_restart_count_hint(&loaded);
    let governed_stop_id = governed_stop_id(&loaded, &request);

    let mut checkpoint_status = status(spec_path)?;
    checkpoint_status.stop_requested = false;
    persist_status(
        &loaded,
        &checkpoint_status,
        "governed_emergency_stop_pre_stop_checkpoint",
    )?;
    emit_daemon_event(
        &runtime_context,
        &loaded,
        "governed_emergency_stop_checkpoint",
        "completed",
        restart_count,
        json!({
            "governed_stop_id": governed_stop_id,
            "checkpoint_reason": "governed_emergency_stop_pre_stop_checkpoint",
            "checkpoint_ref": "continuity_checkpoint.json",
            "status_ref": "status.json"
        }),
    )?;

    let safe_fail = record_safe_fail_event(
        &runtime_context,
        &loaded,
        SafeFailRecord {
            status: &checkpoint_status,
            trigger: "governed_emergency_stop_pre_stop",
            restart_count,
            bounded_test_restart_limit: None,
            last_child_exit: None,
            details: json!({
                "governed_stop_id": governed_stop_id,
                "checkpoint_reason": "governed_emergency_stop_pre_stop_checkpoint",
                "requested_by": request.operator_identity.clone(),
                "authorization_ref": governed_authorization_ref(&request.authorization),
                "intent": request.intent.clone(),
                "recoverability_requirement": "checkpoint_and_safe_fail_before_stop"
            }),
        },
    )?;

    let governed_stop = json!({
        "schema": "adl.csm.governed_stop.v1",
        "governed_stop_id": governed_stop_id,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "classification": "governed_emergency_stop",
        "distinguishes_from": [
            "crash",
            "budget_exhaustion",
            "test_harness_termination",
            "service_manager_failure",
            "ordinary_api_request"
        ],
        "operator_intent": {
            "reason": request.reason.clone(),
            "operator_identity": request.operator_identity.clone(),
            "authorization_ref": governed_authorization_ref(&request.authorization),
            "intent": request.intent.clone(),
            "requested_at": request.requested_at,
            "recorded_at": Utc::now()
        },
        "pre_stop_checkpoint": {
            "status": "completed",
            "status_ref": "status.json",
            "continuity_checkpoint_ref": "continuity_checkpoint.json",
            "continuity_replay_manifest_ref": "continuity_replay_manifest.json"
        },
        "safe_fail": safe_fail,
        "agent_recoverability": {
            "state_before_stop": checkpoint_status.state.clone(),
            "recoverability_class": "recoverable_checkpointed",
            "recovery_refs": [
                "status.json",
                "continuity_checkpoint.json",
                "continuity_replay_manifest.json",
                "safe_fail_bundle.json"
            ]
        },
        "authorization_policy": {
            "required_fields": ["reason", "operator_identity", "authorization", "intent", "requested_at"],
            "ordinary_api_requests_can_stop_runtime": false,
            "runtime_budget": "not_applicable"
        }
    });
    write_json_pretty(&governed_stop_path(&loaded), &governed_stop)?;
    record_lifecycle_lifelog(
        &loaded,
        "governed_emergency_stop_requested",
        &governed_stop_id,
        &governed_stop,
    )?;
    append_operator_event(
        &loaded,
        "governed_emergency_stop_requested",
        json!({
            "governed_stop_id": governed_stop_id,
            "reason": governed_stop["operator_intent"]["reason"].clone(),
            "operator_identity": governed_stop["operator_intent"]["operator_identity"].clone(),
            "authorization_ref": governed_stop["operator_intent"]["authorization_ref"].clone(),
            "intent": governed_stop["operator_intent"]["intent"].clone(),
            "checkpoint_ref": "continuity_checkpoint.json",
            "safe_fail_ref": "safe_fail_bundle.json",
            "governed_stop_ref": "governed_stop.json"
        }),
    )?;

    let stopped = write_stop_record(
        &loaded,
        governed_stop["operator_intent"]["reason"]
            .as_str()
            .unwrap_or("governed emergency stop requested"),
        governed_stop["operator_intent"]["operator_identity"]
            .as_str()
            .unwrap_or("operator"),
        "governed_emergency_stop_recorded",
    )?;
    let daemon_status = write_daemon_status(
        &runtime_context,
        &loaded,
        DaemonStatusInput {
            state: "governed_stopped",
            bounded_test_mode: false,
            restart_count,
            bounded_test_restart_limit: None,
            checkpoint_interval_secs: loaded.spec.checkpoint.interval_secs.unwrap_or(1).max(1),
            last_event: "governed_emergency_stop_recorded",
            last_child_exit: None,
            next_backoff_secs: 0,
        },
    )?;
    emit_daemon_event(
        &runtime_context,
        &loaded,
        "governed_emergency_stop_recorded",
        "completed",
        restart_count,
        json!({
            "governed_stop_id": governed_stop_id,
            "governed_stop_ref": "governed_stop.json",
            "stop_ref": "stop.json",
            "daemon_status_ref": "daemon_status.json"
        }),
    )?;
    record_lifecycle_lifelog(
        &loaded,
        "governed_emergency_stop_recorded",
        &governed_stop_id,
        &json!({
            "status": "completed",
            "stop_ref": "stop.json",
            "daemon_status_ref": "daemon_status.json"
        }),
    )?;
    let notice = record_governed_runtime_notice(
        &runtime_context,
        &loaded,
        GovernedNoticeInput {
            notice_kind: "governed_emergency_stop",
            severity: "critical",
            trigger: "governed_emergency_stop",
            status: &stopped,
            restart_count,
            bounded_test_restart_limit: None,
            last_child_exit: None,
            safe_fail: governed_stop["safe_fail"].clone(),
            details: json!({
                "governed_stop_id": governed_stop_id,
                "governed_stop_ref": "governed_stop.json",
                "stop_ref": "stop.json",
                "daemon_status_ref": "daemon_status.json",
                "lifecycle_lifelog_db_ref": "csm_lifecycle_lifelog.db.jsonl",
                "operator_identity": governed_stop["operator_intent"]["operator_identity"].clone(),
                "authorization_ref": governed_stop["operator_intent"]["authorization_ref"].clone()
            }),
        },
    )?;

    Ok(json!({
        "schema": "adl.csm.governed_stop.result.v1",
        "status": "completed",
        "runtime_owner": "csm",
        "governed_stop_id": governed_stop_id,
        "classification": "governed_emergency_stop",
        "governed_stop_ref": "governed_stop.json",
        "stop_ref": "stop.json",
        "status_ref": "status.json",
        "daemon_status_ref": "daemon_status.json",
        "continuity_checkpoint_ref": "continuity_checkpoint.json",
        "safe_fail_ref": "safe_fail_bundle.json",
        "lifecycle_lifelog_db_ref": "csm_lifecycle_lifelog.db.jsonl",
        "notice": notice,
        "agent_recoverability": governed_stop["agent_recoverability"].clone(),
        "daemon_status": daemon_status
    }))
}

fn validate_governed_stop_request(request: &GovernedStopRequest) -> Result<()> {
    let fields = [
        ("--reason", request.reason.as_str()),
        ("--operator", request.operator_identity.as_str()),
        ("--authorization", request.authorization.as_str()),
        ("--intent", request.intent.as_str()),
    ];
    for (name, value) in fields {
        if value.trim().is_empty() {
            return Err(anyhow!("csm governed-stop requires non-empty {name}"));
        }
    }
    let intent = request.intent.trim();
    if !matches!(
        intent,
        "emergency_polis_stop" | "operator_safety_stop" | "recoverability_drill"
    ) {
        return Err(anyhow!(
            "csm governed-stop unsupported --intent '{intent}' (expected emergency_polis_stop, operator_safety_stop, or recoverability_drill)"
        ));
    }
    Ok(())
}

fn governed_stop_id(loaded: &LoadedAgentSpec, request: &GovernedStopRequest) -> String {
    let mut hasher = Sha256::new();
    hasher.update(loaded.spec.agent_instance_id.as_bytes());
    hasher.update([0xff]);
    hasher.update(request.reason.trim().as_bytes());
    hasher.update([0xff]);
    hasher.update(request.operator_identity.trim().as_bytes());
    hasher.update([0xff]);
    hasher.update(request.intent.trim().as_bytes());
    hasher.update([0xff]);
    hasher.update(request.requested_at.to_rfc3339().as_bytes());
    format!("csm-governed-stop-{:x}", hasher.finalize())
}

fn governed_authorization_ref(authorization: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(authorization.trim().as_bytes());
    format!("sha256:{:x}", hasher.finalize())
}

fn daemon_restart_count_hint(loaded: &LoadedAgentSpec) -> u64 {
    read_json_optional::<Value>(&daemon_status_path(loaded))
        .ok()
        .flatten()
        .and_then(|value| value.get("restart_count").and_then(Value::as_u64))
        .unwrap_or(0)
}

fn record_lifecycle_lifelog<T: Serialize>(
    loaded: &LoadedAgentSpec,
    event: &str,
    event_id: &str,
    payload: &T,
) -> Result<()> {
    let payload = serde_json::to_value(payload).context("serialize lifecycle lifelog payload")?;
    let sequence = lifecycle_lifelog_sequence(loaded)?;
    let record = json!({
        "schema": "adl.csm.lifecycle_lifelog.row.v1",
        "database_schema": "adl.csm.lifecycle_lifelog.db.v1",
        "sequence": sequence,
        "event_id": event_id,
        "event": event,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "recorded_at": Utc::now(),
        "payload": payload
    });
    append_jsonl(&csm_lifecycle_lifelog_db_path(loaded), &record)?;
    write_json_pretty(
        &csm_lifecycle_lifelog_index_path(loaded),
        &json!({
            "schema": "adl.csm.lifecycle_lifelog.index.v1",
            "database_schema": "adl.csm.lifecycle_lifelog.db.v1",
            "database_ref": "csm_lifecycle_lifelog.db.jsonl",
            "latest_sequence": sequence,
            "latest_event_id": event_id,
            "latest_event": event,
            "retention_policy": "permanent_local_runtime_lifecycle_log",
            "backend": "jsonl_database",
            "future_backends": ["sqlite", "immutable_ledger"],
            "updated_at": Utc::now()
        }),
    )?;
    Ok(())
}

fn lifecycle_lifelog_sequence(loaded: &LoadedAgentSpec) -> Result<u64> {
    let path = csm_lifecycle_lifelog_db_path(loaded);
    if !path.exists() {
        return Ok(1);
    }
    let file = File::open(&path).with_context(|| format!("failed opening {}", path.display()))?;
    let count = BufReader::new(file)
        .lines()
        .filter(|line| line.is_ok())
        .count();
    Ok(count as u64 + 1)
}

pub fn inspect(spec_path: &Path, options: InspectOptions) -> Result<Value> {
    inspection::inspect(spec_path, options)
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
    if spec.checkpoint.interval_secs == Some(0) {
        return Err(anyhow!(
            "agent spec checkpoint.interval_secs must be greater than zero"
        ));
    }
    if spec.checkpoint.min_request_interval_secs == Some(0) {
        return Err(anyhow!(
            "agent spec checkpoint.min_request_interval_secs must be greater than zero"
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
        let (status, checkpoint_reason, restore_event) =
            if let Some(status) = restore_status_from_checkpoint(loaded)? {
                (
                    status,
                    "status_restored_from_checkpoint",
                    Some("status_restored_from_checkpoint"),
                )
            } else if let Some(status) = restore_status_from_ledger(loaded)? {
                (
                    status,
                    "status_restored_from_ledger",
                    Some("status_restored_from_ledger"),
                )
            } else {
                (
                    status_with_state(
                        loaded,
                        AgentStatusState::NotStarted,
                        None,
                        None,
                        None,
                        false,
                        None,
                    ),
                    "state_initialized",
                    None,
                )
            };
        persist_status(loaded, &status, checkpoint_reason)?;
        if let Some(event) = restore_event {
            append_operator_event(
                loaded,
                event,
                json!({
                    "status_ref": "status.json",
                    "checkpoint_ref": "continuity_checkpoint.json",
                    "replay_manifest_ref": "continuity_replay_manifest.json"
                }),
            )?;
        }
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

fn restore_status_from_checkpoint(loaded: &LoadedAgentSpec) -> Result<Option<StatusRecord>> {
    let Some(checkpoint) = read_json_optional::<Value>(&continuity_checkpoint_path(loaded))? else {
        return Ok(None);
    };
    if checkpoint.get("godel_agent_snapshot_diff").is_some() {
        validate_recovery_read(&loaded.state_root).with_context(|| {
            format!(
                "Godel last-known-good pointer did not validate before recovering {}",
                loaded.spec.agent_instance_id
            )
        })?;
    }
    let ledger = ledger_cursor(loaded)?;
    let checkpoint_cycle_id = checkpoint
        .get("latest_cycle_id")
        .and_then(Value::as_str)
        .map(str::to_string);
    let checkpoint_cycle_number = checkpoint_cycle_id
        .as_deref()
        .and_then(cycle_number)
        .unwrap_or(0);
    let ledger_cycle_number = ledger
        .latest_cycle_id
        .as_deref()
        .and_then(cycle_number)
        .unwrap_or(0);
    if ledger_cycle_number > checkpoint_cycle_number {
        return Ok(Some(status_with_state(
            loaded,
            AgentStatusState::Completed,
            ledger.latest_cycle_id,
            ledger.latest_status,
            read_lease(loaded)?,
            false,
            None,
        )));
    }
    let state = checkpoint
        .get("state")
        .cloned()
        .map(serde_json::from_value::<AgentStatusState>)
        .transpose()?
        .unwrap_or(AgentStatusState::Idle);
    let latest_cycle_id = checkpoint_cycle_id;
    let latest_cycle_status = checkpoint
        .get("latest_cycle_status")
        .and_then(Value::as_str)
        .map(str::to_string);
    let stop_requested = checkpoint
        .get("stop_requested")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let active_lease = read_lease(loaded)?;
    Ok(Some(status_with_state(
        loaded,
        state,
        latest_cycle_id,
        latest_cycle_status,
        active_lease,
        stop_requested,
        None,
    )))
}

fn restore_status_from_ledger(loaded: &LoadedAgentSpec) -> Result<Option<StatusRecord>> {
    let ledger = ledger_cursor(loaded)?;
    if ledger.latest_cycle_id.is_none() {
        return Ok(None);
    }
    Ok(Some(status_with_state(
        loaded,
        AgentStatusState::Completed,
        ledger.latest_cycle_id,
        ledger.latest_status,
        None,
        false,
        None,
    )))
}

fn acquire_lease(
    loaded: &LoadedAgentSpec,
    cycle_id: &str,
    recover_stale_lease: bool,
) -> Result<LeaseRecord> {
    let path = lease_path(loaded);
    let stop = read_stop(loaded)?;
    let existing = read_lease(loaded)?;
    match activation_decision(
        stop.is_some(),
        coordination_lease_state(existing.as_ref()),
        recover_stale_lease,
    ) {
        ActivationDecision::Start => {}
        ActivationDecision::StopRequested => {
            let reason = stop
                .map(|record| record.reason)
                .unwrap_or_else(|| "operator stop requested".to_string());
            let status = stopped_status(loaded, reason.clone());
            write_status(loaded, &status)?;
            write_continuity_restore_artifacts(
                loaded,
                &status,
                "stop_requested_during_activation",
            )?;
            return Err(anyhow!(
                "stop_requested: {reason}; do not start a new cycle while stop is active"
            ));
        }
        ActivationDecision::LeaseActive => {
            let existing = existing.expect("active lease should be present");
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
            persist_status(loaded, &status, "lease_active")?;
            return Err(anyhow!(
                "lease_active: another cycle already holds the agent lease"
            ));
        }
        ActivationDecision::LeaseStaleRecoverable => {
            let existing = existing.expect("stale lease should be present");
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
        }
        ActivationDecision::LeaseStaleBlocked => {
            let existing = existing.expect("stale lease should be present");
            let status = status_with_state(
                loaded,
                AgentStatusState::Failed,
                None,
                None,
                Some(existing),
                false,
                Some(StatusError {
                    class: "lease_stale".to_string(),
                    message: "active lease is stale; rerun with --recover-stale-lease".to_string(),
                }),
            );
            persist_status(loaded, &status, "lease_stale_blocked")?;
            return Err(anyhow!(
                "lease_stale: active lease is stale; rerun with --recover-stale-lease"
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

    let freedom_gate_admission =
        if workflow_supported && loaded.spec.workflow.kind == "adl_workflow" {
            let policy_decision = loaded
                .spec
                .workflow
                .run_args
                .get("freedom_gate_policy_decision")
                .and_then(Value::as_str);
            let event = csm_freedom_gate::adl_workflow_admission_event(
                &loaded.spec.agent_instance_id,
                cycle_id,
                policy_decision,
            );
            write_json_pretty(&cycle_dir.join("freedom_gate_decision.json"), &event)?;
            if event.stopped_before_executor {
                rejected_actions.push(format!("freedom_gate_{}", event.reason_code));
                dedup_strings(&mut rejected_actions);
            }
            Some(event)
        } else {
            None
        };

    let guardrail_pass = workflow_supported
        && rejected_actions.is_empty()
        && sanitization.passed
        && max_runtime_not_exceeded;
    let adl_run = if guardrail_pass && loaded.spec.workflow.kind == "adl_workflow" {
        Some(run_adl_workflow_cycle(loaded, cycle_id, &cycle_dir)?)
    } else {
        None
    };

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
    let mut decision_result = json!({
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
    if let Some(event) = freedom_gate_admission.as_ref() {
        if let Some(obj) = decision_result.as_object_mut() {
            obj.insert(
                "freedom_gate_decision".to_string(),
                serde_json::to_value(event)?,
            );
        }
    }
    write_json_pretty(&cycle_dir.join("decision_result.json"), &decision_result)?;

    let run_ref = if loaded.spec.workflow.kind == "adl_workflow" {
        let run_status_ref = adl_run
            .as_ref()
            .map(|run| run.status_ref.clone())
            .unwrap_or_else(|| "csm_adl_run_status.json".to_string());
        let trace_ref = adl_run
            .as_ref()
            .map(|run| run.trace_ref.clone())
            .unwrap_or_else(|| "csm_adl_run_status.json#trace".to_string());
        json!({
            "schema": RUN_REF_SCHEMA,
            "workflow_kind": "adl_workflow",
            "workflow_ref": workflow_ref,
            "run_status_ref": run_status_ref,
            "trace_ref": trace_ref,
            "execution_note": "CSM executed the configured ADL DAG through the canonical resolver/executor inside this supervised runtime cycle."
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
        "csm_runtime": {
            "runtime_owner": "csm",
            "adl_role": "tooling_control_plane",
            "aee": "integrated",
            "chronosense": "integrated",
            "scheduler_watcher": "integrated",
            "resilience_middleware": "integrated"
        },
        "artifacts": {
            "observations": "observations.json",
            "decision_request": "decision_request.json",
            "decision_result": "decision_result.json",
            "run_ref": "run_ref.json",
            "memory_writes": "memory_writes.jsonl",
            "guardrail_report": "guardrail_report.json",
            "cycle_summary": "cycle_summary.md",
            "csm_adl_run_status": adl_run.as_ref().map(|run| run.status_ref.as_str())
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

#[derive(Debug)]
struct AdlWorkflowRunSummary {
    status_ref: String,
    trace_ref: String,
}

fn run_adl_workflow_cycle(
    loaded: &LoadedAgentSpec,
    cycle_id: &str,
    cycle_dir: &Path,
) -> Result<AdlWorkflowRunSummary> {
    let workflow_path = loaded.spec.workflow.path.as_ref().ok_or_else(|| {
        anyhow!("adl_workflow requires workflow.path so CSM can execute the configured DAG")
    })?;
    let adl_path = if workflow_path.is_absolute() {
        workflow_path.clone()
    } else {
        loaded
            .spec_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(workflow_path)
    };
    let adl_path_str = adl_path
        .to_str()
        .context("adl_workflow path must be valid UTF-8")?;
    let doc = adl::AdlDoc::load_from_file(adl_path_str)
        .with_context(|| format!("failed loading CSM ADL workflow {}", adl_path.display()))?;
    let resolved = resolve::resolve_run(&doc)
        .with_context(|| format!("failed resolving CSM ADL workflow {}", adl_path.display()))?;
    let adl_base_dir = adl_path.parent().unwrap_or_else(|| Path::new("."));
    let out_dir = cycle_dir.join("adl_runtime");
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("failed creating CSM ADL runtime dir {}", out_dir.display()))?;

    let mut tr = trace::Trace::new(
        resolved.run_id.clone(),
        resolved.workflow_id.clone(),
        resolved.doc.version.clone(),
    );
    let result =
        execute::execute_sequential(&resolved, &mut tr, false, false, adl_base_dir, &out_dir)
            .with_context(|| format!("CSM ADL DAG execution failed for cycle {cycle_id}"))?;
    tr.run_finished(result.pause.is_none());

    let records: Vec<Value> = result
        .records
        .iter()
        .map(|record| {
            json!({
                "step_id": record.step_id,
                "provider_id": record.provider_id,
                "status": record.status,
                "attempts": record.attempts,
                "output_bytes": record.output_bytes
            })
        })
        .collect();
    let artifacts: Vec<String> = result
        .artifacts
        .iter()
        .map(|path| path_artifact_ref(path))
        .collect();
    let scheduler_policy = execute::scheduler_policy_for_run(&resolved)?
        .map(|(max_concurrency, source)| {
            json!({
                "max_concurrency": max_concurrency,
                "source": source.as_str()
            })
        })
        .unwrap_or(Value::Null);
    let trace_events: Vec<String> = tr.events.iter().map(|event| event.summarize()).collect();
    let status = json!({
        "schema": "adl.csm.adl_workflow_run_status.v1",
        "runtime_owner": "csm",
        "adl_role": "tooling_control_plane",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "cycle_id": cycle_id,
        "workflow_path": path_artifact_ref(&adl_path),
        "run_id": resolved.run_id,
        "workflow_id": resolved.workflow_id,
        "status": if result.pause.is_some() { "paused" } else { "success" },
        "step_count": result.records.len(),
        "records": records,
        "artifacts": artifacts,
        "out_dir": path_artifact_ref(&out_dir),
        "scheduler_policy": scheduler_policy,
        "runtime_control": serde_json::to_value(&result.runtime_control)?,
        "trace_event_count": trace_events.len(),
        "trace_events": trace_events,
        "aee_resilience_trace": "retained_in_trace_events",
        "chronosense_runtime": "retained_in_csm_daemon_events",
        "completed_at": Utc::now()
    });
    write_json_pretty(&cycle_dir.join("csm_adl_run_status.json"), &status)?;
    Ok(AdlWorkflowRunSummary {
        status_ref: "csm_adl_run_status.json".to_string(),
        trace_ref: "csm_adl_run_status.json#trace_events".to_string(),
    })
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

fn persist_status(
    loaded: &LoadedAgentSpec,
    status: &StatusRecord,
    checkpoint_reason: &str,
) -> Result<()> {
    write_status(loaded, status)?;
    write_continuity_restore_artifacts(loaded, status, checkpoint_reason)
}

struct SafeFailRecord<'a> {
    status: &'a StatusRecord,
    trigger: &'a str,
    restart_count: u64,
    bounded_test_restart_limit: Option<u64>,
    last_child_exit: Option<String>,
    details: Value,
}

fn record_safe_fail_bundle(
    runtime_context: &CsmRuntimeContext,
    loaded: &LoadedAgentSpec,
    record: &SafeFailRecord<'_>,
) -> Result<Value> {
    let sequence = next_safe_fail_sequence(loaded)?;
    let bundle_ref = format!("safe_fail_artifacts/safe-fail-{sequence:06}.json");
    let bundle = json!({
        "schema": SAFE_FAIL_BUNDLE_SCHEMA,
        "format_version": "csm.safe-fail.v1",
        "runtime_owner": "csm",
        "adl_role": "tooling_control_plane",
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "captured_at": Utc::now(),
        "safe_fail_sequence": sequence,
        "trigger": record.trigger,
        "trigger_model": safe_fail_trigger_model(),
        "agent_checkpoint_policy": agent_checkpoint_policy(loaded),
        "restart_count": record.restart_count,
        "bounded_test_restart_limit": record.bounded_test_restart_limit,
        "last_child_exit": record.last_child_exit.clone(),
        "agent_outcome": safe_fail_agent_outcome(record.status),
        "recoverability": safe_fail_recoverability(record.status),
        "monotonicity": {
            "policy": "append_new_bundle_then_update_latest_pointer",
            "latest_pointer_ref": "safe_fail_bundle.json",
            "bundle_ref": bundle_ref,
            "does_not_rewrite_continuity_checkpoint": true,
            "does_not_rewrite_cycle_ledger": true
        },
        "serialized_refs": safe_fail_serialized_refs(loaded),
        "serialized_state": safe_fail_serialized_state(loaded),
        "observability": {
            "schema": "adl.csm.safe_fail_observability.v1",
            "event_command": "csm",
            "event_stage": "safe_fail_serialization",
            "otel_service_name": "csm-runtime-daemon",
            "trace_id": daemon_trace_id(loaded),
            "span_id": daemon_span_id("safe_fail_serialization", record.restart_count),
            "parent_span_id": daemon_parent_span_id(loaded),
            "runtime_capabilities": csm_runtime_capabilities(runtime_context, &loaded.spec.agent_instance_id),
            "chronosense_clock_stack": csm_chronosense_clock_stack(runtime_context)
        },
        "recovery_hints": safe_fail_recovery_hints(record.status),
        "negative_case_boundaries": [
            "graceful_stop_and_observed_failures_are_serialized",
            "kill_9_or_host_loss_may_only_preserve_last_completed_partial_checkpoint",
            "unsafe_or_ambiguous_active_lease_requires_quarantine_review",
            "malformed_prior_state_is_retained_as_unreadable_artifact_evidence"
        ],
        "non_claims": [
            "not_mid_step_checkpointing",
            "not_kill_9_resistant",
            "not_distributed_consensus_checkpoint",
            "not_secret_material_capture"
        ],
        "details": record.details.clone()
    });
    let sequence_path =
        safe_fail_artifacts_dir(loaded).join(format!("safe-fail-{sequence:06}.json"));
    let low_disk = record_low_disk_preflight(&sequence_path, "safe_fail_sequence_write")?;
    if low_disk {
        let mut degraded_bundle = bundle.clone();
        if let Some(monotonicity) = degraded_bundle
            .get_mut("monotonicity")
            .and_then(Value::as_object_mut)
        {
            monotonicity.insert(
                "policy".to_string(),
                json!("low_disk_latest_pointer_only_no_new_sequence_artifact"),
            );
            monotonicity.insert(
                "sequence_artifact_suppressed".to_string(),
                json!("storage_low_disk"),
            );
        }
        write_json_pretty(&safe_fail_bundle_path(loaded), &degraded_bundle)?;
        return Ok(json!({
            "schema": SAFE_FAIL_BUNDLE_SCHEMA,
            "status": "serialized_degraded",
            "bundle_ref": "safe_fail_bundle.json",
            "sequence_ref": Value::Null,
            "safe_fail_sequence": sequence,
            "agent_outcome": degraded_bundle["agent_outcome"].clone(),
            "recoverability": degraded_bundle["recoverability"].clone(),
            "storage_pressure": "low_disk",
            "suppressed_artifact": bundle_ref
        }));
    }
    write_json_pretty(&sequence_path, &bundle)?;
    write_json_pretty(&safe_fail_bundle_path(loaded), &bundle)?;
    Ok(json!({
        "schema": SAFE_FAIL_BUNDLE_SCHEMA,
        "status": "serialized",
        "bundle_ref": "safe_fail_bundle.json",
        "sequence_ref": bundle_ref,
        "safe_fail_sequence": sequence,
        "agent_outcome": bundle["agent_outcome"].clone(),
        "recoverability": bundle["recoverability"].clone()
    }))
}

fn record_safe_fail_event(
    runtime_context: &CsmRuntimeContext,
    loaded: &LoadedAgentSpec,
    record: SafeFailRecord<'_>,
) -> Result<Value> {
    match record_safe_fail_bundle(runtime_context, loaded, &record) {
        Ok(summary) => {
            emit_daemon_event(
                runtime_context,
                loaded,
                "safe_fail_serialization",
                "completed",
                record.restart_count,
                summary.clone(),
            )?;
            Ok(summary)
        }
        Err(err) => {
            let summary = json!({
                "schema": SAFE_FAIL_BUNDLE_SCHEMA,
                "status": "serialization_failed",
                "trigger": record.trigger,
                "error": err.to_string(),
                "fallback_refs": {
                    "status_ref": safe_fail_existing_ref(&status_path(loaded)),
                    "continuity_checkpoint_ref": safe_fail_existing_ref(&continuity_checkpoint_path(loaded)),
                    "operator_events_ref": safe_fail_existing_ref(&operator_events_path(loaded))
                }
            });
            emit_daemon_event(
                runtime_context,
                loaded,
                "safe_fail_serialization",
                "failed",
                record.restart_count,
                summary.clone(),
            )?;
            Ok(summary)
        }
    }
}

struct GovernedNoticeInput<'a> {
    notice_kind: &'a str,
    severity: &'a str,
    trigger: &'a str,
    status: &'a StatusRecord,
    restart_count: u64,
    bounded_test_restart_limit: Option<u64>,
    last_child_exit: Option<String>,
    safe_fail: Value,
    details: Value,
}

fn record_governed_runtime_notice(
    runtime_context: &CsmRuntimeContext,
    loaded: &LoadedAgentSpec,
    input: GovernedNoticeInput<'_>,
) -> Result<Value> {
    let captured_at = Utc::now();
    let notice_id = governed_notice_id(loaded, input.trigger, input.restart_count, captured_at);
    let local_notice = json!({
        "schema": "adl.csm.governed_notice.v1",
        "notice_id": notice_id,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "notice_kind": input.notice_kind,
        "severity": input.severity,
        "trigger": input.trigger,
        "captured_at": captured_at,
        "restart_count": input.restart_count,
        "bounded_test_restart_limit": input.bounded_test_restart_limit,
        "last_child_exit": input.last_child_exit.clone(),
        "recoverable_state": {
            "state": input.status.state,
            "status_ref": "status.json",
            "continuity_checkpoint_ref": "continuity_checkpoint.json",
            "safe_fail_ref": "safe_fail_bundle.json"
        },
        "safe_fail": input.safe_fail,
        "local_first_policy": {
            "source_of_truth": "local_safe_fail_and_checkpoint_artifacts",
            "outbound_delivery_may_fail": true,
            "transport_failure_policy": "retain_delivery_failure_and_continue_recovery"
        },
        "observability": {
            "event_command": "csm",
            "event_stage": "governed_runtime_notice",
            "otel_service_name": "csm-runtime-daemon",
            "trace_id": daemon_trace_id(loaded),
            "span_id": daemon_span_id("governed_runtime_notice", input.restart_count),
            "parent_span_id": daemon_parent_span_id(loaded),
            "runtime_capabilities": csm_runtime_capabilities(runtime_context, &loaded.spec.agent_instance_id),
            "chronosense_clock_stack": csm_chronosense_clock_stack(runtime_context)
        },
        "delivery_policy": {
            "channels": ["cloudwatch_logs", "acip_sns", "cloudfront_control_plane"],
            "cloudfront_control_plane_dependency": "#4915",
            "redaction": "operations_safe_no_secret_payloads"
        },
        "delivery_attempts": [{
            "channel": "local_notice_ledger",
            "status": "recorded",
            "artifact_ref": "csm_governed_notices.jsonl"
        }],
        "details": input.details
    });
    write_json_pretty(&csm_notice_latest_path(loaded), &local_notice)?;
    append_jsonl(&csm_notice_ledger_path(loaded), &local_notice)?;

    let mut final_notice = local_notice;
    let mut attempts = vec![json!({
        "channel": "local_notice_ledger",
        "status": "recorded",
        "artifact_ref": "csm_governed_notices.jsonl"
    })];
    attempts.extend(publish_csm_governed_notice_signal(loaded, &final_notice));
    if let Some(object) = final_notice.as_object_mut() {
        object.insert("delivery_attempts".to_string(), json!(attempts));
        object.insert("delivery_completed_at".to_string(), json!(Utc::now()));
    }
    write_json_pretty(&csm_notice_latest_path(loaded), &final_notice)?;
    append_jsonl(&csm_notice_ledger_path(loaded), &final_notice)?;
    emit_daemon_event(
        runtime_context,
        loaded,
        "governed_runtime_notice",
        "completed",
        input.restart_count,
        json!({
            "notice_id": notice_id,
            "notice_kind": input.notice_kind,
            "severity": input.severity,
            "trigger": input.trigger,
            "notice_ref": "csm_governed_notice_latest.json",
            "notice_ledger_ref": "csm_governed_notices.jsonl"
        }),
    )?;
    Ok(json!({
        "schema": "adl.csm.governed_notice.summary.v1",
        "notice_id": notice_id,
        "notice_kind": input.notice_kind,
        "severity": input.severity,
        "trigger": input.trigger,
        "notice_ref": "csm_governed_notice_latest.json",
        "notice_ledger_ref": "csm_governed_notices.jsonl",
        "delivery_attempts": final_notice["delivery_attempts"].clone()
    }))
}

fn governed_notice_id(
    loaded: &LoadedAgentSpec,
    trigger: &str,
    restart_count: u64,
    captured_at: DateTime<Utc>,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(loaded.spec.agent_instance_id.as_bytes());
    hasher.update([0xff]);
    hasher.update(trigger.as_bytes());
    hasher.update([0xfe]);
    hasher.update(restart_count.to_string().as_bytes());
    hasher.update([0xfd]);
    hasher.update(captured_at.to_rfc3339().as_bytes());
    format!("csm-notice-{:x}", hasher.finalize())
}

fn next_safe_fail_sequence(loaded: &LoadedAgentSpec) -> Result<u64> {
    let dir = safe_fail_artifacts_dir(loaded);
    if !dir.exists() {
        return Ok(1);
    }
    let mut max_seen = 0u64;
    for entry in fs::read_dir(&dir).with_context(|| format!("failed reading {}", dir.display()))? {
        let entry = entry?;
        let Some(name) = entry.file_name().to_str().map(str::to_string) else {
            continue;
        };
        let Some(stem) = name
            .strip_prefix("safe-fail-")
            .and_then(|value| value.strip_suffix(".json"))
        else {
            continue;
        };
        if let Ok(sequence) = stem.parse::<u64>() {
            max_seen = max_seen.max(sequence);
        }
    }
    Ok(max_seen + 1)
}

fn safe_fail_trigger_model() -> Value {
    json!({
        "schema": "adl.csm.safe_fail_trigger_model.v1",
        "supported_triggers": [
            "graceful_stop",
            "daemon_partial_checkpoint",
            "daemon_child_failed",
            "bounded_test_supervisor_failure",
            "restart_backoff",
            "daemon_heartbeat"
        ],
        "observability_exporter_failure_policy": "recorded_in_observability_status_when_exporter_reports_failure",
        "checkpoint_failure_policy": "best_effort_fallback_refs_if_safe_fail_writer_cannot_complete",
        "unclaimed_triggers": [
            "kill_9",
            "host_power_loss_before_last_checkpoint_flush",
            "kernel_storage_loss"
        ]
    })
}

fn agent_checkpoint_policy(loaded: &LoadedAgentSpec) -> Value {
    json!({
        "schema": "adl.csm.agent_checkpoint_policy.v1",
        "interval_secs": loaded.spec.checkpoint.interval_secs,
        "allow_agent_requested": loaded.spec.checkpoint.allow_agent_requested,
        "min_request_interval_secs": loaded.spec.checkpoint.min_request_interval_secs.unwrap_or(30),
        "request_ref": "checkpoint_request.json",
        "request_contract": {
            "schema": "adl.csm.agent_checkpoint_request.v1",
            "required_fields": ["schema", "reason", "requested_at"],
            "governance": "CSM may accept or block; agent request is advisory and rate limited"
        }
    })
}

fn observe_agent_checkpoint_request(
    runtime_context: &CsmRuntimeContext,
    loaded: &LoadedAgentSpec,
    restart_count: u64,
    last_checkpoint_at: DateTime<Utc>,
) -> Result<Option<Value>> {
    let request_path = checkpoint_request_path(loaded);
    if !request_path.exists() {
        return Ok(None);
    }
    let request = read_json_artifact_value(&request_path);
    let policy = agent_checkpoint_policy(loaded);
    let request_validation = validate_agent_checkpoint_request(&request);
    let min_interval_secs = loaded
        .spec
        .checkpoint
        .min_request_interval_secs
        .unwrap_or(30);
    let elapsed_secs = Utc::now()
        .signed_duration_since(last_checkpoint_at)
        .num_seconds()
        .max(0) as u64;
    let (decision, reason) = if let Some(reason) = request_validation {
        ("blocked_malformed", reason)
    } else if !loaded.spec.checkpoint.allow_agent_requested {
        (
            "blocked_disabled",
            "agent-requested checkpoints are disabled by spec policy",
        )
    } else if elapsed_secs < min_interval_secs {
        (
            "blocked_rate_limited",
            "agent-requested checkpoint was inside the minimum request interval",
        )
    } else {
        (
            "accepted",
            "agent-requested checkpoint accepted under current policy",
        )
    };
    let outcome = json!({
        "schema": "adl.csm.agent_checkpoint_request_outcome.v1",
        "decision": decision,
        "reason": reason,
        "request_ref": "checkpoint_request.json",
        "request": request,
        "request_validation": if let Some(reason) = request_validation {
            json!({"status": "failed", "reason": reason})
        } else {
            json!({"status": "passed"})
        },
        "policy": policy,
        "elapsed_since_last_checkpoint_secs": elapsed_secs
    });
    emit_daemon_event(
        runtime_context,
        loaded,
        "agent_checkpoint_request",
        decision,
        restart_count,
        outcome.clone(),
    )?;
    match fs::remove_file(&request_path) {
        Ok(()) => {}
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => {
            emit_daemon_event(
                runtime_context,
                loaded,
                "agent_checkpoint_request_cleanup",
                "failed",
                restart_count,
                json!({
                    "request_ref": "checkpoint_request.json",
                    "error": err.to_string()
                }),
            )?;
        }
    }
    Ok(Some(outcome))
}

fn validate_agent_checkpoint_request(request: &Value) -> Option<&'static str> {
    if request.get("status").and_then(Value::as_str) != Some("serialized") {
        return Some("checkpoint request must be readable JSON");
    }
    let Some(value) = request.get("value") else {
        return Some("checkpoint request must include a parsed value");
    };
    let object = match value.as_object() {
        Some(object) => object,
        None => return Some("checkpoint request must be a JSON object"),
    };
    if object.get("schema").and_then(Value::as_str) != Some("adl.csm.agent_checkpoint_request.v1") {
        return Some("checkpoint request schema must be adl.csm.agent_checkpoint_request.v1");
    }
    for field in ["reason", "requested_at"] {
        if object
            .get(field)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .is_none()
        {
            return Some("checkpoint request requires non-empty reason and requested_at fields");
        }
    }
    None
}

fn safe_fail_agent_outcome(status: &StatusRecord) -> Value {
    let (state, action_allowed, reason) = match status.state {
        AgentStatusState::Completed => (
            "completed",
            false,
            "bounded run completed; next activation requires a new runtime decision",
        ),
        AgentStatusState::Stopped => (
            "sleeping",
            false,
            "operator or daemon stop observed; continuity artifacts are retained for later wake",
        ),
        AgentStatusState::Idle | AgentStatusState::NotStarted => (
            "sleeping",
            false,
            "no active cycle is running; next heartbeat may decide whether to wake",
        ),
        AgentStatusState::Leased | AgentStatusState::RunningCycle => (
            "quarantined",
            false,
            "active work may be ambiguous; review lease and trace evidence before activation",
        ),
        AgentStatusState::Failed => {
            if status.active_lease.is_some() {
                (
                    "quarantined",
                    false,
                    "failure retained an active lease; unsafe resume requires review",
                )
            } else {
                (
                    "recoverable",
                    false,
                    "failure reached a serialized checkpoint without an active lease",
                )
            }
        }
    };
    json!({
        "state": state,
        "action_allowed_without_review": action_allowed,
        "reason": reason,
        "source_status_state": status.state,
        "last_cycle_id": status.last_cycle_id,
        "last_cycle_status": status.last_cycle_status
    })
}

fn safe_fail_recoverability(status: &StatusRecord) -> Value {
    let class = match status.state {
        AgentStatusState::Completed => "already_completed",
        AgentStatusState::Stopped | AgentStatusState::Idle | AgentStatusState::NotStarted => {
            "recoverable_sleeping"
        }
        AgentStatusState::Failed if status.active_lease.is_none() => "recoverable_checkpointed",
        AgentStatusState::Failed | AgentStatusState::Leased | AgentStatusState::RunningCycle => {
            "quarantine_required"
        }
    };
    json!({
        "class": class,
        "allowed_next_actions": match class {
            "already_completed" => json!(["inspect", "start_new_cycle_after_policy_check"]),
            "recoverable_sleeping" => json!(["inspect", "wake_after_policy_check"]),
            "recoverable_checkpointed" => json!(["inspect", "retry_after_operator_review", "capture_continuity_capsule"]),
            _ => json!(["inspect", "quarantine_review", "capture_continuity_capsule"])
        },
        "last_error": status.last_error
    })
}

fn safe_fail_recovery_hints(status: &StatusRecord) -> Value {
    json!({
        "inspect": "csm daemon/status artifacts under the agent state root",
        "continuity_checkpoint_ref": "continuity_checkpoint.json",
        "continuity_replay_manifest_ref": "continuity_replay_manifest.json",
        "capture_for_transfer": "csm continuity capture --spec <agent.yaml> --out <bundle-dir>",
        "resume_guard": if matches!(status.state, AgentStatusState::Failed | AgentStatusState::Leased | AgentStatusState::RunningCycle) {
            "review_required_before_wake"
        } else {
            "policy_check_required_before_wake"
        }
    })
}

fn safe_fail_serialized_refs(loaded: &LoadedAgentSpec) -> Vec<Value> {
    [
        (
            "runtime_identity",
            "agent_spec.locked.json",
            locked_spec_path(loaded),
        ),
        ("status", "status.json", status_path(loaded)),
        (
            "daemon_status",
            "daemon_status.json",
            daemon_status_path(loaded),
        ),
        (
            "continuity_checkpoint",
            "continuity_checkpoint.json",
            continuity_checkpoint_path(loaded),
        ),
        (
            "continuity_replay_manifest",
            "continuity_replay_manifest.json",
            continuity_replay_manifest_path(loaded),
        ),
        (
            "cycle_ledger",
            "cycle_ledger.jsonl",
            cycle_ledger_path(loaded),
        ),
        (
            "memory_index",
            "memory_index.json",
            memory_index_path(loaded),
        ),
        (
            "provider_binding_history",
            "provider_binding_history.jsonl",
            provider_binding_history_path(loaded),
        ),
        (
            "operator_events_tail",
            "operator_events.jsonl",
            operator_events_path(loaded),
        ),
    ]
    .into_iter()
    .map(|(role, reference, path)| {
        json!({
            "role": role,
            "ref": reference,
            "status": if path.exists() { "retained" } else { "missing" },
            "bytes": fs::metadata(path).map(|metadata| metadata.len()).ok()
        })
    })
    .collect()
}

fn safe_fail_serialized_state(loaded: &LoadedAgentSpec) -> Value {
    json!({
        "status": read_json_artifact_value(&status_path(loaded)),
        "daemon_status": read_json_artifact_value(&daemon_status_path(loaded)),
        "continuity_checkpoint": read_json_artifact_value(&continuity_checkpoint_path(loaded)),
        "continuity_replay_manifest": read_json_artifact_value(&continuity_replay_manifest_path(loaded)),
        "lease": read_json_artifact_value(&lease_path(loaded)),
        "stop": read_json_artifact_value(&stop_path(loaded)),
        "cycle_ledger_tail": read_jsonl_tail_artifact(&cycle_ledger_path(loaded), 20),
        "operator_event_tail": read_jsonl_tail_artifact(&operator_events_path(loaded), 40),
        "provider_binding_tail": read_jsonl_tail_artifact(&provider_binding_history_path(loaded), 20)
    })
}

fn read_json_artifact_value(path: &Path) -> Value {
    if !path.exists() {
        return json!({"status": "missing"});
    }
    match fs::read_to_string(path) {
        Ok(raw) => match serde_json::from_str::<Value>(&raw) {
            Ok(value) => json!({"status": "serialized", "value": value}),
            Err(err) => json!({"status": "unreadable", "reason": err.to_string()}),
        },
        Err(err) => json!({"status": "unreadable", "reason": err.to_string()}),
    }
}

fn read_jsonl_tail_artifact(path: &Path, limit: usize) -> Value {
    if !path.exists() {
        return json!({"status": "missing", "entries": []});
    }
    let Ok(raw) = fs::read_to_string(path) else {
        return json!({"status": "unreadable", "entries": []});
    };
    let mut entries = Vec::new();
    let mut unreadable = 0usize;
    let lines = raw.lines().collect::<Vec<_>>();
    let start = lines.len().saturating_sub(limit);
    for line in &lines[start..] {
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<Value>(line) {
            Ok(value) => entries.push(value),
            Err(_) => unreadable += 1,
        }
    }
    json!({
        "status": if unreadable == 0 { "serialized" } else { "partial" },
        "tail_limit": limit,
        "unreadable_lines": unreadable,
        "entries": entries
    })
}

fn safe_fail_existing_ref(path: &Path) -> Value {
    if path.exists() {
        json!({"status": "retained", "bytes": fs::metadata(path).map(|m| m.len()).ok()})
    } else {
        json!({"status": "missing"})
    }
}

struct DaemonStatusInput<'a> {
    state: &'a str,
    bounded_test_mode: bool,
    restart_count: u64,
    bounded_test_restart_limit: Option<u64>,
    checkpoint_interval_secs: u64,
    last_event: &'a str,
    last_child_exit: Option<String>,
    next_backoff_secs: u64,
}

struct PartialCheckpointSleep<'a> {
    total_sleep_secs: u64,
    checkpoint_interval_secs: u64,
    restart_count: u64,
    bounded_test_restart_limit: Option<u64>,
    last_child_exit: Option<String>,
    recoverable_error: Option<StatusError>,
    event: &'a str,
    no_sleep: bool,
}

struct CsmRuntimeContext {
    chronosense: ChronosenseRuntimeService,
}

impl CsmRuntimeContext {
    fn new() -> Result<Self> {
        let started_at_epoch_ms = epoch_millis_now();
        let chronosense = ChronosenseRuntimeService::new(ChronosenseRuntimeServiceConfig::utc(
            started_at_epoch_ms,
        ))
        .context("failed initializing CSM Chronosense runtime service")?;
        let _initial_time_sync = start_runtime_time_observation();
        Ok(Self { chronosense })
    }

    fn time_sync_status(&self) -> crate::chronosense::ChronosenseTimeSyncStatus {
        capture_runtime_time_sync_status()
    }
}

fn write_daemon_status(
    runtime_context: &CsmRuntimeContext,
    loaded: &LoadedAgentSpec,
    input: DaemonStatusInput<'_>,
) -> Result<DaemonStatusRecord> {
    let now = Utc::now();
    let existing_status = read_json_optional::<DaemonStatusRecord>(&daemon_status_path(loaded))?;
    let started_at = existing_status
        .as_ref()
        .map(|status| status.started_at)
        .unwrap_or(now);
    let governed_stop_active = read_stop(loaded)?
        .is_some_and(|stop| stop.classification == "governed_emergency_stop_recorded");
    let preserve_governed_terminal = governed_stop_active
        && existing_status
            .as_ref()
            .is_some_and(|status| status.state == "governed_stopped")
        && input.state != "governed_stopped";
    let effective_state = if preserve_governed_terminal {
        "governed_stopped"
    } else {
        input.state
    };
    let effective_last_event = if preserve_governed_terminal {
        "governed_emergency_stop_recorded"
    } else {
        input.last_event
    };
    let resident_agents_status =
        csm_resident_agents::resident_agent_set_status(&loaded.spec.agent_instance_id);
    let status = DaemonStatusRecord {
        schema: DAEMON_STATUS_SCHEMA.to_string(),
        agent_instance_id: loaded.spec.agent_instance_id.clone(),
        runtime_capabilities: csm_runtime_capabilities_with_resident_agents(
            runtime_context,
            resident_agents_status.clone(),
        ),
        state: effective_state.to_string(),
        supervisor_pid: std::process::id(),
        restart_policy: daemon_restart_policy().to_string(),
        service_mode: daemon_status_service_mode(effective_state, input.bounded_test_mode)
            .to_string(),
        bounded_test_mode: input.bounded_test_mode,
        restart_count: input.restart_count,
        bounded_test_restart_limit: input.bounded_test_restart_limit,
        checkpoint_interval_secs: input.checkpoint_interval_secs,
        last_event: effective_last_event.to_string(),
        last_child_exit: input.last_child_exit,
        started_at,
        last_checkpoint_at: now,
        next_backoff_secs: input.next_backoff_secs,
        trace_id: daemon_trace_id(loaded),
        span_id: daemon_span_id(effective_last_event, input.restart_count),
        parent_span_id: Some(daemon_parent_span_id(loaded)),
        unsupported_permanence_claims: unsupported_permanence_claims(),
        updated_at: now,
    };
    write_json_pretty(&daemon_status_path(loaded), &status)?;
    let agent_state = read_status(loaded)?
        .as_ref()
        .and_then(|status| serde_json::to_value(&status.state).ok())
        .and_then(|value| value.as_str().map(str::to_string));
    let checkpoint_observed = continuity_checkpoint_path(loaded).exists();
    if let Err(err) = csm_shepherd_agent::write_status_snapshot(
        &loaded.state_root,
        &loaded.spec.agent_instance_id,
        effective_state,
        agent_state.as_deref(),
        checkpoint_observed,
        None,
    ) {
        let _ = append_operator_event(
            loaded,
            "csm_shepherd_agent_status_write_failed",
            json!({
                "schema": "adl.csm.shepherd_agent.write_failure.v1",
                "runtime_owner": "csm",
                "component": "polis_shepherd_agent",
                "status": "degraded_nonfatal",
                "reason": err.to_string(),
                "recovery_policy": "continue_runtime_and_surface_missing_or_degraded_shepherd_status"
            }),
        );
    }
    if let Err(err) = write_json_pretty(
        &loaded
            .state_root
            .join(csm_resident_agents::CSM_RESIDENT_AGENTS_STATUS_REF),
        &resident_agents_status,
    ) {
        let _ = append_operator_event(
            loaded,
            "csm_resident_agents_status_write_failed",
            json!({
                "schema": "adl.csm.resident_agents.write_failure.v1",
                "runtime_owner": "csm",
                "component": "resident_agents",
                "status": "degraded_nonfatal",
                "reason": err.to_string(),
                "recovery_policy": "continue_runtime_and_surface_computed_resident_agent_status"
            }),
        );
    }
    if let Err(err) =
        csm_freedom_gate::write_status_snapshot(&loaded.state_root, &loaded.spec.agent_instance_id)
    {
        let _ = append_operator_event(
            loaded,
            "csm_freedom_gate_status_write_failed",
            json!({
                "schema": "adl.csm.freedom_gate.write_failure.v1",
                "runtime_owner": "csm",
                "component": "freedom_gate",
                "status": "degraded_nonfatal",
                "reason": err.to_string(),
                "recovery_policy": "continue_runtime_and_fail_closed_for_executor_admission"
            }),
        );
    }
    Ok(status)
}

fn sleep_with_partial_checkpoints(
    runtime_context: &CsmRuntimeContext,
    loaded: &LoadedAgentSpec,
    daemon_status: &mut DaemonStatusRecord,
    sleep: PartialCheckpointSleep<'_>,
) -> Result<bool> {
    let mut remaining = sleep.total_sleep_secs;
    if remaining == 0 || sleep.no_sleep {
        let last_checkpoint_at = daemon_status.last_checkpoint_at;
        let mut current = status(&loaded.spec_path)?;
        if let Some(error) = sleep.recoverable_error.clone() {
            current.state = AgentStatusState::Failed;
            current.last_error = Some(error);
        }
        persist_status(loaded, &current, "daemon_partial_checkpoint")?;
        *daemon_status = write_daemon_status(
            runtime_context,
            loaded,
            DaemonStatusInput {
                state: daemon_status.state.as_str(),
                bounded_test_mode: sleep.no_sleep,
                restart_count: sleep.restart_count,
                bounded_test_restart_limit: sleep.bounded_test_restart_limit,
                checkpoint_interval_secs: sleep.checkpoint_interval_secs,
                last_event: "checkpoint_write",
                last_child_exit: sleep.last_child_exit.clone(),
                next_backoff_secs: 0,
            },
        )?;
        emit_daemon_event(
            runtime_context,
            loaded,
            "checkpoint_write",
            "completed",
            sleep.restart_count,
            json!({
                "checkpoint_reason": "daemon_partial_checkpoint",
                "checkpoint_ref": "continuity_checkpoint.json",
                "status_ref": "status.json",
                "trigger": sleep.event
            }),
        )?;
        let agent_checkpoint_request = observe_agent_checkpoint_request(
            runtime_context,
            loaded,
            sleep.restart_count,
            last_checkpoint_at,
        )?;
        let _ = record_safe_fail_event(
            runtime_context,
            loaded,
            SafeFailRecord {
                status: &current,
                trigger: "daemon_partial_checkpoint",
                restart_count: sleep.restart_count,
                bounded_test_restart_limit: sleep.bounded_test_restart_limit,
                last_child_exit: sleep.last_child_exit.clone(),
                details: json!({
                    "checkpoint_reason": "daemon_partial_checkpoint",
                    "trigger": sleep.event,
                    "checkpoint_ref": "continuity_checkpoint.json",
                    "status_ref": "status.json",
                    "agent_checkpoint_request": agent_checkpoint_request
                }),
            },
        )?;
        return Ok(false);
    }

    let mut stop_observed = false;
    while remaining > 0 {
        let slice = remaining.min(sleep.checkpoint_interval_secs);
        std::thread::sleep(Duration::from_secs(slice));
        remaining -= slice;
        let last_checkpoint_at = daemon_status.last_checkpoint_at;
        let mut current = status(&loaded.spec_path)?;
        if let Some(error) = sleep.recoverable_error.clone() {
            current.state = AgentStatusState::Failed;
            current.last_error = Some(error);
        }
        persist_status(loaded, &current, "daemon_partial_checkpoint")?;
        let next_backoff_secs = if sleep.event == "restart_backoff" {
            remaining
        } else {
            0
        };
        *daemon_status = write_daemon_status(
            runtime_context,
            loaded,
            DaemonStatusInput {
                state: daemon_status.state.as_str(),
                bounded_test_mode: sleep.no_sleep,
                restart_count: sleep.restart_count,
                bounded_test_restart_limit: sleep.bounded_test_restart_limit,
                checkpoint_interval_secs: sleep.checkpoint_interval_secs,
                last_event: "checkpoint_write",
                last_child_exit: sleep.last_child_exit.clone(),
                next_backoff_secs,
            },
        )?;
        emit_daemon_event(
            runtime_context,
            loaded,
            "checkpoint_write",
            "completed",
            sleep.restart_count,
            json!({
                "checkpoint_reason": "daemon_partial_checkpoint",
                "checkpoint_ref": "continuity_checkpoint.json",
                "status_ref": "status.json",
                "trigger": sleep.event,
                "remaining_sleep_secs": remaining
            }),
        )?;
        let agent_checkpoint_request = observe_agent_checkpoint_request(
            runtime_context,
            loaded,
            sleep.restart_count,
            last_checkpoint_at,
        )?;
        let _ = record_safe_fail_event(
            runtime_context,
            loaded,
            SafeFailRecord {
                status: &current,
                trigger: "daemon_partial_checkpoint",
                restart_count: sleep.restart_count,
                bounded_test_restart_limit: sleep.bounded_test_restart_limit,
                last_child_exit: sleep.last_child_exit.clone(),
                details: json!({
                    "checkpoint_reason": "daemon_partial_checkpoint",
                    "trigger": sleep.event,
                    "remaining_sleep_secs": remaining,
                    "checkpoint_ref": "continuity_checkpoint.json",
                    "status_ref": "status.json",
                    "agent_checkpoint_request": agent_checkpoint_request
                }),
            },
        )?;
        if read_stop(loaded)?.is_some() {
            stop_observed = true;
            emit_daemon_event(
                runtime_context,
                loaded,
                "graceful_shutdown_requested",
                "observed",
                sleep.restart_count,
                json!({"stop_ref": "stop.json"}),
            )?;
            break;
        }
    }
    Ok(stop_observed)
}

fn emit_daemon_event(
    runtime_context: &CsmRuntimeContext,
    loaded: &LoadedAgentSpec,
    event: &str,
    result: &str,
    restart_count: u64,
    details: Value,
) -> Result<()> {
    let trace_id = daemon_trace_id(loaded);
    let span_id = daemon_span_id(event, restart_count);
    let parent_span_id = daemon_parent_span_id(loaded);
    let event_details = json!({
        "event": event,
        "result": result,
        "trace_id": trace_id,
        "span_id": span_id,
        "parent_span_id": parent_span_id,
        "otel": {
            "trace_id": trace_id,
            "span_id": span_id,
            "parent_span_id": parent_span_id,
            "service_name": "csm-runtime-daemon",
            "event_name": event
        },
        "runtime_capabilities": csm_runtime_capabilities(runtime_context, &loaded.spec.agent_instance_id),
        "chronosense_clock_stack": csm_chronosense_clock_stack(runtime_context),
        "details": details
    });
    append_operator_event(loaded, event, event_details)?;
    let restart_count_s = restart_count.to_string();
    let time_sync_status = runtime_context.time_sync_status();
    crate::observability::emit_event(
        "csm",
        event,
        result,
        &[
            ("process_class", "csm_runtime_daemon"),
            ("agent_instance_id", loaded.spec.agent_instance_id.as_str()),
            ("trace_id", trace_id.as_str()),
            ("span_id", span_id.as_str()),
            ("parent_span_id", parent_span_id.as_str()),
            ("otel_service_name", "csm-runtime-daemon"),
            ("runtime_role", "csm_runtime"),
            ("adl_role", "tooling_control_plane"),
            ("chronosense", "integrated"),
            ("chronosense_time_sync", time_sync_status.health.as_str()),
            (
                "chronosense_time_sync_reason",
                time_sync_status.reason.as_str(),
            ),
            ("aee_recovery", "integrated"),
            ("scheduler_watcher", "integrated"),
            ("resilience_middleware", "integrated"),
            ("observability", "integrated"),
            ("restart_count", restart_count_s.as_str()),
        ],
    );
    Ok(())
}

fn csm_runtime_capabilities(runtime_context: &CsmRuntimeContext, agent_instance_id: &str) -> Value {
    csm_runtime_capabilities_with_resident_agents(
        runtime_context,
        csm_resident_agents::resident_agent_set_status(agent_instance_id),
    )
}

fn csm_runtime_capabilities_with_resident_agents(
    runtime_context: &CsmRuntimeContext,
    resident_agents_status: Value,
) -> Value {
    json!({
        "schema": "adl.csm.runtime_capabilities.v1",
        "runtime_owner": "csm",
        "adl_role": "tooling_control_plane",
        "process_class": "csm_runtime_daemon",
        "supervisor": {
            "status": "integrated",
            "restart_policy": daemon_restart_policy(),
            "service_mode": "permanent",
            "bounded_test_mode": "explicit_only",
            "host_supervisor_compatibility": [
                "launchd_keepalive",
                "systemd_restart_always",
                "rustysd_service_manager",
                "rinit_service_manager"
            ],
            "lifetime_boundary": "operator_stop_or_fatal_supervisor_failure_only"
        },
        "chronosense": {
            "status": "integrated",
            "service_schema": runtime_context.chronosense.config().schema_version,
            "clock_stack_schema": crate::chronosense::CHRONOSENSE_CLOCK_STACK_SCHEMA,
            "clock_stack_capture": "daemon_event_time",
            "time_substrate": "SNTP",
            "time_process_model": "csm_in_process_component_no_separate_binary",
            "time_primary_status_source": "rsntp::AsyncSntpClient",
            "time_compatibility_fallback": "none_in_runtime_path",
            "time_sync": runtime_context.time_sync_status()
        },
        "aee": {
            "status": "integrated",
            "recoverable_states": ["idle", "completed", "failed", "stopped", "leased"],
            "failure_recovery": "permanent_restart_loop_with_checkpoint_restore"
        },
        "scheduler_watcher": {
            "status": "integrated",
            "cadence_source": "heartbeat.interval_secs_or_daemon_default",
            "partial_checkpoint_interval": "checkpoint_interval_secs",
            "stop_observation": "stop_json_checked_between_cycles_and_sleep_slices"
        },
        "resilience_middleware": {
            "status": "integrated",
            "lease_policy": "active_stale_recoverable_blocked",
            "restart_backoff": "bounded_exponential",
            "partial_checkpoints": "daemon_partial_checkpoint",
            "safe_fail_serialization": "integrated_safe_fail_bundle"
        },
        "resident_agents": resident_agents_status,
        "polis_shepherd_agent": csm_shepherd_agent::runtime_capability(),
        "freedom_gate": csm_freedom_gate::runtime_capability(),
        "observability": {
            "status": "integrated",
            "event_command": "csm",
            "otel_service_name": "csm-runtime-daemon",
            "retained_outputs": ["operator_events.jsonl", "daemon_status.json", "safe_fail_bundle.json", "safe_fail_artifacts/", "csm_governed_notices.jsonl", "csm_governed_notice_latest.json", "ADL_OBSERVABILITY_LOG", "ADL_OTEL_LOG", "ADL_OTEL_STATUS"]
        },
        "governed_shutdown_notices": {
            "status": "integrated",
            "local_notice_ledger": "csm_governed_notices.jsonl",
            "outbound_channels": ["cloudwatch_logs", "acip_sns", "cloudfront_control_plane"],
            "transport_failure_policy": "record_delivery_failure_without_blocking_safe_fail_serialization",
            "cloudfront_control_plane_dependency": "#4915"
        }
    })
}

fn csm_chronosense_clock_stack(runtime_context: &CsmRuntimeContext) -> Value {
    runtime_context
        .chronosense
        .capture_epoch_millis(epoch_millis_now())
        .and_then(|clock| serde_json::to_value(clock).context("serialize chronosense clock stack"))
        .unwrap_or_else(|err| {
            json!({
                "schema": crate::chronosense::CHRONOSENSE_CLOCK_STACK_SCHEMA,
                "capture_status": "failed",
                "error": err.to_string()
            })
        })
}

fn epoch_millis_now() -> u128 {
    Utc::now().timestamp_millis().max(0) as u128
}

fn daemon_trace_id(loaded: &LoadedAgentSpec) -> String {
    format!("agent.{}.daemon", loaded.spec.agent_instance_id)
}

fn daemon_parent_span_id(loaded: &LoadedAgentSpec) -> String {
    format!("daemon:{}:supervisor", loaded.spec.agent_instance_id)
}

fn daemon_span_id(event: &str, restart_count: u64) -> String {
    format!("daemon:{event}:{restart_count}")
}

fn restart_backoff_secs(restart_count: u64) -> u64 {
    2u64.saturating_pow(restart_count.min(4) as u32).min(30)
}

fn daemon_restart_policy() -> &'static str {
    "always"
}

fn daemon_service_mode(bounded_test_mode: bool) -> &'static str {
    if bounded_test_mode {
        "bounded_test_only"
    } else {
        "permanent"
    }
}

fn daemon_status_service_mode(state: &str, bounded_test_mode: bool) -> &'static str {
    if bounded_test_mode || state == "completed" {
        "bounded_test_only"
    } else {
        "permanent"
    }
}

fn unsupported_permanence_claims() -> Vec<String> {
    vec![
        "not_os_boot_persistent".to_string(),
        "not_kill_9_resistant".to_string(),
        "not_host_resource_exhaustion_resistant".to_string(),
        "not_missing_binary_resistant".to_string(),
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CoordinationLeaseState {
    Clear,
    Active,
    Stale,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ActivationDecision {
    Start,
    StopRequested,
    LeaseActive,
    LeaseStaleRecoverable,
    LeaseStaleBlocked,
}

fn coordination_lease_state(lease: Option<&LeaseRecord>) -> CoordinationLeaseState {
    match lease {
        Some(lease) if lease_is_stale(lease) => CoordinationLeaseState::Stale,
        Some(_) => CoordinationLeaseState::Active,
        None => CoordinationLeaseState::Clear,
    }
}

fn activation_decision(
    stop_requested: bool,
    lease_state: CoordinationLeaseState,
    recover_stale_lease: bool,
) -> ActivationDecision {
    if stop_requested {
        ActivationDecision::StopRequested
    } else {
        match (lease_state, recover_stale_lease) {
            (CoordinationLeaseState::Clear, _) => ActivationDecision::Start,
            (CoordinationLeaseState::Active, _) => ActivationDecision::LeaseActive,
            (CoordinationLeaseState::Stale, true) => ActivationDecision::LeaseStaleRecoverable,
            (CoordinationLeaseState::Stale, false) => ActivationDecision::LeaseStaleBlocked,
        }
    }
}

fn derive_visible_status_state(
    current: AgentStatusState,
    stop_requested: bool,
    lease_state: CoordinationLeaseState,
) -> AgentStatusState {
    if stop_requested {
        AgentStatusState::Stopped
    } else {
        match lease_state {
            CoordinationLeaseState::Active => AgentStatusState::Leased,
            CoordinationLeaseState::Stale => AgentStatusState::Failed,
            CoordinationLeaseState::Clear => current,
        }
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
        classification: event.to_string(),
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
    persist_status(loaded, &status, "stop_recorded")?;
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
    let mut statuses = Vec::new();
    for value in read_cycle_ledger_entries(loaded)? {
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
    let mut cursor = LedgerCursor::default();
    for value in read_cycle_ledger_entries(loaded)? {
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

fn read_cycle_ledger_entries(loaded: &LoadedAgentSpec) -> Result<Vec<Value>> {
    let path = cycle_ledger_path(loaded);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("failed reading cycle ledger {}", path.display()))?;
    let mut entries = Vec::new();
    let line_count = raw.lines().count();
    let terminated = raw.ends_with('\n');
    for (index, line) in raw.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<Value>(line) {
            Ok(value) => entries.push(value),
            Err(err) if !terminated && index + 1 == line_count => {
                let error = err.to_string();
                crate::observability::emit_event(
                    "csm",
                    "cycle_ledger",
                    "partial_tail_skipped",
                    &[("path_ref", "cycle_ledger.jsonl"), ("error", &error)],
                );
            }
            Err(err) => {
                return Err(err)
                    .with_context(|| format!("failed parsing cycle ledger {}", path.display()));
            }
        }
    }
    Ok(entries)
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

fn write_continuity_restore_artifacts(
    loaded: &LoadedAgentSpec,
    status: &StatusRecord,
    checkpoint_reason: &str,
) -> Result<()> {
    let continuity: Value = read_json_required(&continuity_path(loaded))?;
    let ledger = ledger_cursor(loaded)?;
    let status_cycle_number = status
        .last_cycle_id
        .as_deref()
        .and_then(cycle_number)
        .unwrap_or(0);
    let ledger_cycle_number = ledger
        .latest_cycle_id
        .as_deref()
        .and_then(cycle_number)
        .unwrap_or(0);
    let (latest_cycle_id, latest_cycle_status) = if ledger_cycle_number > status_cycle_number {
        (ledger.latest_cycle_id.clone(), ledger.latest_status.clone())
    } else {
        (
            status
                .last_cycle_id
                .clone()
                .or_else(|| ledger.latest_cycle_id.clone()),
            status
                .last_cycle_status
                .clone()
                .or_else(|| ledger.latest_status.clone()),
        )
    };
    let lease_state = match read_lease(loaded)?.as_ref() {
        Some(lease) if lease_is_stale(lease) => "stale",
        Some(_) => "active",
        None => "clear",
    };
    let next_cycle_number = latest_cycle_id
        .as_deref()
        .and_then(cycle_number)
        .unwrap_or(0)
        .max(ledger.max_cycle_number)
        + 1;
    let mut checkpoint = json!({
        "schema": CONTINUITY_CHECKPOINT_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "captured_at": Utc::now(),
        "checkpoint_reason": checkpoint_reason,
        "state": status.state,
        "stop_requested": status.stop_requested,
        "latest_cycle_id": latest_cycle_id,
        "latest_cycle_status": latest_cycle_status,
        "completed_cycle_count": status.completed_cycle_count,
        "consecutive_failure_count": status.consecutive_failure_count,
        "continuity_kind": continuity.get("continuity_kind").cloned().unwrap_or_else(|| json!("pre_v0_92_handle")),
        "continuity_ref": "continuity.json",
        "status_ref": "status.json",
        "cycle_ledger_ref": "cycle_ledger.jsonl",
        "lease_ref": if lease_state == "clear" { Value::Null } else { json!("lease.json") },
        "lease_state": lease_state,
        "restore_basis": {
            "ledger_entry_count": ledger.count,
            "max_cycle_number": ledger.max_cycle_number,
            "expected_next_cycle_id": format!("cycle-{next_cycle_number:06}")
        }
    });
    let checkpoint_path = continuity_checkpoint_path(loaded);
    let checkpoint_low_disk =
        record_low_disk_preflight(&checkpoint_path, "continuity_checkpoint_write")?;
    let replay_manifest_path = continuity_replay_manifest_path(loaded);
    let replay_low_disk =
        record_low_disk_preflight(&replay_manifest_path, "continuity_replay_manifest_write")?;
    if checkpoint_low_disk || replay_low_disk {
        emit_storage_degraded_event(
            loaded,
            if checkpoint_low_disk {
                "continuity_checkpoint_write"
            } else {
                "continuity_replay_manifest_write"
            },
            checkpoint_reason,
            &anyhow!("low disk preflight blocked Godel snapshot chain advancement"),
        );
        return Ok(());
    }

    let godel_snapshot_diff = write_checkpoint_snapshot_diff(
        loaded,
        status,
        checkpoint_reason,
        loaded.spec.checkpoint.interval_secs.unwrap_or(1).max(1),
    )?;
    checkpoint["godel_agent_snapshot_diff"] = serde_json::to_value(&godel_snapshot_diff)?;

    write_json_pretty(&checkpoint_path, &checkpoint)?;

    let replay_manifest = json!({
        "schema": CONTINUITY_REPLAY_MANIFEST_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "generated_at": Utc::now(),
        "continuity_ref": "continuity.json",
        "checkpoint_ref": "continuity_checkpoint.json",
        "status_ref": "status.json",
        "cycle_ledger_ref": "cycle_ledger.jsonl",
        "expected_resume": {
            "next_cycle_id": format!("cycle-{next_cycle_number:06}"),
            "latest_cycle_id": checkpoint.get("latest_cycle_id").cloned().unwrap_or(Value::Null),
            "active_lease_state": lease_state,
            "recover_stale_lease_required": lease_state == "stale"
        },
        "godel_agent_snapshot_diff": godel_snapshot_diff,
        "restore_invariants": [
            "append_only_cycle_ledger",
            "latest_cycle_id_matches_checkpoint",
            "lease_file_blocks_duplicate_active_cycle_without_explicit_recovery",
            "godel_snapshot_diff_last_known_good_pointer_validates_before_recovery"
        ],
        "reviewer_steps": [
            "Inspect continuity_checkpoint.json for the latest captured cycle state.",
            "Inspect continuity_replay_manifest.json for the next expected cycle id and lease posture.",
            "Inspect cycle_ledger.jsonl to confirm resume continues at the next cycle id without duplicates.",
            "Inspect godel_snapshots/godel_agent_snapshot_chain.json and validate the last-known-good pointer before agent recovery."
        ],
        "non_claims": [
            "not_mid_step_checkpointing",
            "not_full_runtime_persistence",
            "not_distributed_recovery"
        ]
    });
    write_json_pretty(&replay_manifest_path, &replay_manifest)?;
    Ok(())
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

fn default_stop_classification() -> String {
    "operator_stop_requested".to_string()
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
