//! Persistent storage helpers for long-lived agent state artifacts.
use super::schema::OPERATOR_EVENT_SCHEMA;
use super::types::{LeaseRecord, LoadedAgentSpec, StatusRecord, StopRecord};
use crate::runtime_aws_signal::publish_runtime_heartbeat_signal;
use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub(super) fn cycles_dir(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("cycles")
}

pub(super) fn locked_spec_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("agent_spec.locked.json")
}

pub(super) fn continuity_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("continuity.json")
}

pub(super) fn cycle_ledger_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("cycle_ledger.jsonl")
}

pub(super) fn provider_binding_history_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("provider_binding_history.jsonl")
}

pub(super) fn memory_index_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("memory_index.json")
}

pub(super) fn operator_events_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("operator_events.jsonl")
}

pub(super) fn status_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("status.json")
}

pub(super) fn lease_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("lease.json")
}

pub(super) fn stop_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("stop.json")
}

pub(super) fn ensure_jsonl_file(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed creating {}", parent.display()))?;
    }
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("failed ensuring jsonl file {}", path.display()))?;
    Ok(())
}

pub(super) fn read_status(loaded: &LoadedAgentSpec) -> Result<Option<StatusRecord>> {
    read_json_optional(&status_path(loaded))
}

pub(super) fn write_status(loaded: &LoadedAgentSpec, status: &StatusRecord) -> Result<()> {
    write_json_pretty(&status_path(loaded), status)?;
    let _ = publish_runtime_heartbeat_signal(loaded, status);
    Ok(())
}

pub(super) fn read_lease(loaded: &LoadedAgentSpec) -> Result<Option<LeaseRecord>> {
    read_json_optional(&lease_path(loaded))
}

pub(super) fn read_stop(loaded: &LoadedAgentSpec) -> Result<Option<StopRecord>> {
    read_json_optional(&stop_path(loaded))
}

pub(super) fn remove_lease(loaded: &LoadedAgentSpec) -> Result<()> {
    let path = lease_path(loaded);
    match fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err).with_context(|| format!("failed removing lease {}", path.display())),
    }
}

pub(super) fn read_json_optional<T>(path: &Path) -> Result<Option<T>>
where
    T: for<'de> Deserialize<'de>,
{
    if !path.exists() {
        return Ok(None);
    }
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed reading json artifact {}", path.display()))?;
    let value = serde_json::from_str(&raw)
        .with_context(|| format!("failed parsing json artifact {}", path.display()))?;
    Ok(Some(value))
}

pub(super) fn read_json_required(path: &Path) -> Result<Value> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed reading json artifact {}", path.display()))?;
    serde_json::from_str(&raw)
        .with_context(|| format!("failed parsing json artifact {}", path.display()))
}

pub(super) fn write_json_pretty<T>(path: &Path, value: &T) -> Result<()>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed creating {}", parent.display()))?;
    }
    let file = File::create(path).with_context(|| format!("failed creating {}", path.display()))?;
    serde_json::to_writer_pretty(&file, value)
        .with_context(|| format!("failed writing {}", path.display()))?;
    fs::OpenOptions::new()
        .append(true)
        .open(path)?
        .write_all(b"\n")
        .with_context(|| format!("failed finalizing {}", path.display()))?;
    Ok(())
}

pub(super) fn write_jsonl(path: &Path, values: &[Value]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed creating {}", parent.display()))?;
    }
    let mut file =
        File::create(path).with_context(|| format!("failed creating {}", path.display()))?;
    for value in values {
        serde_json::to_writer(&mut file, value)
            .with_context(|| format!("failed writing {}", path.display()))?;
        file.write_all(b"\n")
            .with_context(|| format!("failed finalizing {}", path.display()))?;
    }
    Ok(())
}

pub(super) fn append_jsonl(path: &Path, value: &Value) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed creating {}", parent.display()))?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("failed opening jsonl file {}", path.display()))?;
    serde_json::to_writer(&mut file, value)
        .with_context(|| format!("failed writing jsonl file {}", path.display()))?;
    file.write_all(b"\n")
        .with_context(|| format!("failed finalizing jsonl file {}", path.display()))?;
    Ok(())
}

pub(super) fn append_operator_event(
    loaded: &LoadedAgentSpec,
    event: &str,
    details: Value,
) -> Result<()> {
    let record = json!({
        "schema": OPERATOR_EVENT_SCHEMA,
        "agent_instance_id": loaded.spec.agent_instance_id.clone(),
        "event": event,
        "at": Utc::now(),
        "operator": "local",
        "details": details
    });
    append_jsonl(&operator_events_path(loaded), &record)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::long_lived_agent::{
        AgentSpec, AgentStatusState, HeartbeatSpec, LeaseRecord, StatusError, WorkflowSpec,
    };
    use crate::observability::test_env_lock;
    use chrono::Duration as ChronoDuration;
    use std::env;
    use std::ffi::OsString;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::MutexGuard;

    static TEMP_SEQ: AtomicU64 = AtomicU64::new(0);

    struct MultiEnvGuard {
        saved: Vec<(String, Option<OsString>)>,
        _lock: MutexGuard<'static, ()>,
    }

    impl MultiEnvGuard {
        fn set_all(values: &[(&str, &str)]) -> Self {
            let lock = test_env_lock();
            let tracked = [
                "ADL_AWS_SIGNAL_MODE",
                "ADL_AWS_REGION",
                "ADL_AWS_HEARTBEAT_TARGET",
                "ADL_AWS_SIGNAL_APPROVED",
                "ADL_AWS_HEARTBEAT_LOG_GROUP",
                "ADL_AWS_HEARTBEAT_LOG_STREAM",
            ];
            let mut saved = Vec::with_capacity(tracked.len());
            for key in tracked {
                saved.push((key.to_string(), env::var_os(key)));
                unsafe {
                    env::remove_var(key);
                }
            }
            for (key, value) in values {
                unsafe {
                    env::set_var(key, value);
                }
            }
            Self { saved, _lock: lock }
        }
    }

    impl Drop for MultiEnvGuard {
        fn drop(&mut self) {
            unsafe {
                for (key, old) in self.saved.iter().rev() {
                    match old {
                        Some(value) => env::set_var(key, value),
                        None => env::remove_var(key),
                    }
                }
            }
        }
    }

    fn temp_dir(prefix: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "adl-storage-tests-{prefix}-{}-{}",
            std::process::id(),
            TEMP_SEQ.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn sample_loaded(root: &Path) -> LoadedAgentSpec {
        LoadedAgentSpec {
            spec: AgentSpec {
                schema: "adl.long_lived_agent_spec.v1".to_string(),
                agent_instance_id: "storage-agent".to_string(),
                display_name: "Storage Agent".to_string(),
                state_root: PathBuf::from("state"),
                workflow: WorkflowSpec {
                    kind: "demo_adapter".to_string(),
                    name: Some("storage-heartbeat".to_string()),
                    path: None,
                    run_args: json!({}),
                },
                heartbeat: HeartbeatSpec {
                    interval_secs: Some(30),
                    max_cycles: Some(5),
                    stale_lease_after_secs: Some(60),
                },
                safety: json!({}),
                memory: json!({}),
            },
            spec_path: root.join("agent.yaml"),
            state_root: root.join("state"),
        }
    }

    fn sample_status(state: AgentStatusState) -> StatusRecord {
        StatusRecord {
            schema: "adl.long_lived_agent_status.v1".to_string(),
            agent_instance_id: "storage-agent".to_string(),
            state,
            last_cycle_id: Some("cycle-000001".to_string()),
            last_cycle_status: Some("success".to_string()),
            completed_cycle_count: 1,
            consecutive_failure_count: 0,
            active_lease: None,
            stop_requested: false,
            last_error: None,
            safety_policy: json!({"allow_network": false}),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn storage_paths_and_json_helpers_round_trip() {
        let root = temp_dir("helpers");
        let loaded = sample_loaded(&root);
        assert_eq!(cycles_dir(&loaded), root.join("state/cycles"));
        assert_eq!(
            locked_spec_path(&loaded),
            root.join("state/agent_spec.locked.json")
        );
        assert_eq!(continuity_path(&loaded), root.join("state/continuity.json"));
        assert_eq!(
            cycle_ledger_path(&loaded),
            root.join("state/cycle_ledger.jsonl")
        );
        assert_eq!(
            provider_binding_history_path(&loaded),
            root.join("state/provider_binding_history.jsonl")
        );
        assert_eq!(
            memory_index_path(&loaded),
            root.join("state/memory_index.json")
        );
        assert_eq!(
            operator_events_path(&loaded),
            root.join("state/operator_events.jsonl")
        );
        assert_eq!(status_path(&loaded), root.join("state/status.json"));
        assert_eq!(lease_path(&loaded), root.join("state/lease.json"));
        assert_eq!(stop_path(&loaded), root.join("state/stop.json"));

        let json_path = root.join("nested/object.json");
        write_json_pretty(&json_path, &json!({"hello": "world"})).expect("write json");
        let parsed = read_json_required(&json_path).expect("read required");
        assert_eq!(parsed["hello"], "world");

        let optional_missing: Option<StatusRecord> =
            read_json_optional(&root.join("missing.json")).expect("optional missing");
        assert!(optional_missing.is_none());
    }

    #[test]
    fn storage_jsonl_and_operator_event_helpers_append_reviewable_rows() {
        let root = temp_dir("jsonl");
        let loaded = sample_loaded(&root);
        let jsonl = root.join("state/rows.jsonl");
        ensure_jsonl_file(&jsonl).expect("ensure jsonl");
        append_jsonl(&jsonl, &json!({"step": 1})).expect("append row");
        write_jsonl(&jsonl, &[json!({"step": 2}), json!({"step": 3})]).expect("rewrite rows");
        let rows = fs::read_to_string(&jsonl).expect("rows");
        assert_eq!(rows.lines().count(), 2);
        assert!(rows.contains("\"step\":2"));
        assert!(rows.contains("\"step\":3"));

        append_operator_event(&loaded, "storage_test", json!({"detail": "ok"}))
            .expect("append operator event");
        let events = fs::read_to_string(operator_events_path(&loaded)).expect("events");
        assert!(events.contains("\"event\":\"storage_test\""));
        assert!(events.contains("\"schema\":\"adl.long_lived_agent_operator_event.v1\""));
    }

    #[test]
    fn storage_status_and_control_records_round_trip_with_mock_heartbeat() {
        let root = temp_dir("status");
        let loaded = sample_loaded(&root);
        let _guard = MultiEnvGuard::set_all(&[
            ("ADL_AWS_SIGNAL_MODE", "mock"),
            ("ADL_AWS_REGION", "us-west-2"),
        ]);

        let mut status = sample_status(AgentStatusState::RunningCycle);
        status.active_lease = Some(LeaseRecord {
            schema: "adl.long_lived_agent_lease.v1".to_string(),
            agent_instance_id: "storage-agent".to_string(),
            lease_id: "lease-1".to_string(),
            cycle_id: "cycle-000001".to_string(),
            owner_pid: 55,
            hostname: "local".to_string(),
            started_at: status.updated_at - ChronoDuration::seconds(5),
            expires_at: status.updated_at + ChronoDuration::seconds(55),
            status: "active".to_string(),
        });
        write_status(&loaded, &status).expect("write status");

        let persisted = read_status(&loaded)
            .expect("read status")
            .expect("status exists");
        assert_eq!(persisted.state, AgentStatusState::RunningCycle);
        assert!(persisted.active_lease.is_some());

        let heartbeat =
            fs::read_to_string(loaded.state_root.join("aws_runtime_heartbeat_mock.jsonl"))
                .expect("heartbeat");
        assert_eq!(heartbeat.lines().count(), 1);

        let lease = LeaseRecord {
            schema: "adl.long_lived_agent_lease.v1".to_string(),
            agent_instance_id: "storage-agent".to_string(),
            lease_id: "lease-2".to_string(),
            cycle_id: "cycle-000002".to_string(),
            owner_pid: 77,
            hostname: "local".to_string(),
            started_at: Utc::now(),
            expires_at: Utc::now() + ChronoDuration::seconds(60),
            status: "active".to_string(),
        };
        write_json_pretty(&lease_path(&loaded), &lease).expect("write lease");
        let persisted_lease = read_lease(&loaded)
            .expect("read lease")
            .expect("lease exists");
        assert_eq!(persisted_lease.cycle_id, "cycle-000002");
        remove_lease(&loaded).expect("remove lease");
        remove_lease(&loaded).expect("remove missing lease");
        assert!(read_lease(&loaded).expect("read removed lease").is_none());

        let stop = StopRecord {
            schema: "adl.long_lived_agent_stop.v1".to_string(),
            agent_instance_id: "storage-agent".to_string(),
            reason: "operator pause".to_string(),
            requested_by: "operator".to_string(),
            mode: "stop_before_next_cycle".to_string(),
            requested_at: Utc::now(),
        };
        write_json_pretty(&stop_path(&loaded), &stop).expect("write stop");
        let persisted_stop = read_stop(&loaded).expect("read stop").expect("stop exists");
        assert_eq!(persisted_stop.reason, "operator pause");
    }

    #[test]
    fn storage_json_optional_reports_parse_failures() {
        let root = temp_dir("parse-failure");
        let broken = root.join("broken.json");
        fs::create_dir_all(broken.parent().expect("parent")).expect("mkdir");
        fs::write(&broken, "{not-json").expect("write broken json");
        let err = read_json_optional::<StatusRecord>(&broken).expect_err("invalid json");
        assert!(err.to_string().contains("failed parsing json artifact"));

        let status = StatusRecord {
            last_error: Some(StatusError {
                class: "workflow_failed".to_string(),
                message: "boom".to_string(),
            }),
            ..sample_status(AgentStatusState::Failed)
        };
        write_json_pretty(&root.join("status.json"), &status).expect("write failed status");
        let persisted = read_json_required(&root.join("status.json")).expect("read status");
        assert_eq!(persisted["last_error"]["class"], "workflow_failed");
    }
}
