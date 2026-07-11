//! Persistent storage helpers for long-lived agent state artifacts.
use super::schema::OPERATOR_EVENT_SCHEMA;
use super::types::{LeaseRecord, LoadedAgentSpec, StatusRecord, StopRecord};
use crate::runtime_aws_signal::publish_runtime_heartbeat_signal;
use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const CSM_BACKPRESSURE_STATE_SCHEMA: &str = "adl.csm.backpressure_state.v1";
const CSM_LOW_DISK_RECOVERY_MANIFEST_SCHEMA: &str = "adl.csm.low_disk_recovery_manifest.v1";
const DEFAULT_CSM_DISK_FLOOR_BYTES: u64 = 32 * 1024 * 1024 * 1024;
const ADL_CSM_DISK_FLOOR_BYTES: &str = "ADL_CSM_DISK_FLOOR_BYTES";
const ADL_CSM_TEST_AVAILABLE_BYTES: &str = "ADL_CSM_TEST_AVAILABLE_BYTES";
static JSON_WRITE_TMP_COUNTER: AtomicU64 = AtomicU64::new(0);

pub(super) fn cycles_dir(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("cycles")
}

pub(super) fn locked_spec_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("agent_spec.locked.json")
}

pub(super) fn continuity_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("continuity.json")
}

pub(super) fn continuity_checkpoint_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("continuity_checkpoint.json")
}

pub(super) fn continuity_replay_manifest_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("continuity_replay_manifest.json")
}

pub(super) fn safe_fail_bundle_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("safe_fail_bundle.json")
}

pub(super) fn safe_fail_artifacts_dir(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("safe_fail_artifacts")
}

pub(super) fn csm_notice_ledger_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("csm_governed_notices.jsonl")
}

pub(super) fn csm_notice_latest_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("csm_governed_notice_latest.json")
}

pub(super) fn shutdown_state_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("csm_shutdown_state.json")
}

pub(super) fn shutdown_disposition_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("csm_shutdown_disposition.json")
}

pub(super) fn governed_stop_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("governed_stop.json")
}

pub(super) fn csm_lifecycle_lifelog_db_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("csm_lifecycle_lifelog.db.jsonl")
}

pub(super) fn csm_lifecycle_lifelog_index_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("csm_lifecycle_lifelog.index.json")
}

pub(super) fn checkpoint_request_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("checkpoint_request.json")
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

#[cfg(test)]
fn csm_backpressure_state_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("csm_backpressure_state.json")
}

#[cfg(test)]
fn csm_low_disk_recovery_manifest_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded
        .state_root
        .join("csm_low_disk_recovery_manifest.json")
}

pub(super) fn daemon_status_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join("daemon_status.json")
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

pub(super) fn remove_stop(loaded: &LoadedAgentSpec) -> Result<()> {
    let path = stop_path(loaded);
    match fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err).with_context(|| format!("failed removing stop {}", path.display())),
    }
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
    record_low_disk_preflight(path, "json_write")?;
    write_json_pretty_without_preflight(path, value)
}

fn write_json_pretty_without_preflight<T>(path: &Path, value: &T) -> Result<()>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed creating {}", parent.display()))?;
    }
    let tmp_path = path.with_extension(format!(
        "{}tmp-{}",
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| format!("{extension}."))
            .unwrap_or_default(),
        unique_json_write_tmp_suffix()
    ));
    {
        let mut file = File::create(&tmp_path)
            .with_context(|| format!("failed creating {}", tmp_path.display()))?;
        serde_json::to_writer_pretty(&mut file, value)
            .with_context(|| format!("failed writing {}", tmp_path.display()))?;
        file.write_all(b"\n")
            .with_context(|| format!("failed finalizing {}", tmp_path.display()))?;
        file.sync_all()
            .with_context(|| format!("failed syncing {}", tmp_path.display()))?;
    }
    fs::rename(&tmp_path, path).with_context(|| {
        format!(
            "failed replacing {} with {}",
            path.display(),
            tmp_path.display()
        )
    })?;
    Ok(())
}

fn unique_json_write_tmp_suffix() -> String {
    let sequence = JSON_WRITE_TMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}-{sequence}", std::process::id())
}

pub(super) fn write_jsonl(path: &Path, values: &[Value]) -> Result<()> {
    record_low_disk_preflight(path, "jsonl_write")?;
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
    record_low_disk_preflight(path, "jsonl_append")?;
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

pub(super) fn emit_storage_degraded_event(
    loaded: &LoadedAgentSpec,
    operation: &str,
    context: &str,
    err: &anyhow::Error,
) {
    eprintln!(
        "adl_event schema=adl.observability.event.v1 command=csm stage=runtime_storage_degraded result=write_failed agent_instance_id={} operation={} context={} error={}",
        loaded.spec.agent_instance_id,
        operation.replace(char::is_whitespace, "_"),
        context.replace(char::is_whitespace, "_"),
        err.to_string().replace(char::is_whitespace, "_")
    );
    let _ = append_jsonl(
        &operator_events_path(loaded),
        &json!({
            "schema": OPERATOR_EVENT_SCHEMA,
            "agent_instance_id": loaded.spec.agent_instance_id.clone(),
            "event": "runtime_storage_degraded",
            "at": Utc::now(),
            "operator": "local",
            "details": {
                "operation": operation,
                "context": context,
                "error_class": "artifact_write_failed",
                "policy": "runtime_liveness_continues_when_nonessential_evidence_write_fails"
            }
        }),
    );
}

pub(super) fn record_low_disk_preflight(path: &Path, operation: &str) -> Result<bool> {
    if is_low_disk_internal_artifact(path) {
        return Ok(false);
    }
    let floor_bytes = csm_disk_floor_bytes();
    let available_bytes = available_bytes_for_path(path).with_context(|| {
        format!(
            "failed checking available disk before writing {}",
            path.display()
        )
    })?;
    if available_bytes >= floor_bytes {
        return Ok(false);
    }
    let state_root = infer_state_root(path);
    let artifact_ref = artifact_ref(&state_root, path);
    let captured_at = Utc::now();
    let storage_pressure = json!({
        "state": "low_disk",
        "health": "storage_low_disk_degraded",
        "operation": operation,
        "target_ref": artifact_ref,
        "available_bytes": available_bytes,
        "disk_floor_bytes": floor_bytes,
        "policy": "preflight_before_required_runtime_state_write",
        "degraded_state": "recoverable",
        "optional_artifact_policy": "do_not_delete_retained_evidence_silently"
    });
    let manifest = json!({
        "schema": CSM_LOW_DISK_RECOVERY_MANIFEST_SCHEMA,
        "runtime_owner": "csm",
        "captured_at": captured_at,
        "status": "degraded_recoverable",
        "trigger": "low_disk_preflight",
        "storage_pressure": storage_pressure,
        "minimal_checkpoint_bundle": minimal_checkpoint_bundle(&state_root),
        "recovery_pointer": {
            "state_root_ref": ".",
            "checkpoint_ref": "continuity_checkpoint.json",
            "replay_manifest_ref": "continuity_replay_manifest.json",
            "status_ref": "status.json",
            "safe_fail_ref": "safe_fail_bundle.json",
            "operator_events_ref": "operator_events.jsonl"
        },
        "non_claims": [
            "does_not_delete_retained_evidence",
            "does_not_claim_host_reboot_recovery",
            "does_not_claim_successful_full_artifact_write_after_enospc"
        ]
    });
    let state = json!({
        "schema": CSM_BACKPRESSURE_STATE_SCHEMA,
        "runtime_owner": "csm",
        "updated_at": captured_at,
        "profile": "low_disk_runtime_preflight",
        "summary": {
            "health": "storage_low_disk_degraded",
            "deferred_count": 0,
            "shed_count": 0,
            "required_state_silently_dropped": false,
            "optional_artifact_deletion": "not_performed"
        },
        "storage_pressure": storage_pressure,
        "safe_fail_action": {
            "action": "preserve_minimal_checkpoint_bundle",
            "status": "degraded_recoverable",
            "recovery_manifest_ref": "csm_low_disk_recovery_manifest.json"
        },
        "observability": {
            "event_stage": "low_disk_preflight",
            "retained_evidence": ["csm_backpressure_state.json", "csm_low_disk_recovery_manifest.json"]
        }
    });
    write_json_pretty_without_preflight(
        &state_root.join("csm_low_disk_recovery_manifest.json"),
        &manifest,
    )?;
    write_json_pretty_without_preflight(&state_root.join("csm_backpressure_state.json"), &state)?;
    append_low_disk_event_raw(
        &state_root.join("operator_events.jsonl"),
        &json!({
            "schema": OPERATOR_EVENT_SCHEMA,
            "event": "low_disk_preflight",
            "at": captured_at,
            "operator": "local",
            "details": {
                "target_ref": artifact_ref,
                "available_bytes": available_bytes,
                "disk_floor_bytes": floor_bytes,
                "recovery_manifest_ref": "csm_low_disk_recovery_manifest.json",
                "backpressure_state_ref": "csm_backpressure_state.json"
            }
        }),
    )?;
    Ok(true)
}

fn csm_disk_floor_bytes() -> u64 {
    std::env::var(ADL_CSM_DISK_FLOOR_BYTES)
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .unwrap_or(DEFAULT_CSM_DISK_FLOOR_BYTES)
}

fn available_bytes_for_path(path: &Path) -> Result<u64> {
    if let Ok(raw) = std::env::var(ADL_CSM_TEST_AVAILABLE_BYTES) {
        return raw
            .parse::<u64>()
            .map_err(|err| anyhow!("invalid {ADL_CSM_TEST_AVAILABLE_BYTES}: {err}"));
    }
    available_bytes_for_path_impl(path)
}

#[cfg(unix)]
fn available_bytes_for_path_impl(path: &Path) -> Result<u64> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let probe = existing_probe_path(path);
    let c_path = CString::new(probe.as_os_str().as_bytes())
        .map_err(|_| anyhow!("path contains interior nul byte: {}", probe.display()))?;
    let mut stat = std::mem::MaybeUninit::<libc::statvfs>::uninit();
    let result = unsafe { libc::statvfs(c_path.as_ptr(), stat.as_mut_ptr()) };
    if result != 0 {
        return Err(std::io::Error::last_os_error())
            .with_context(|| format!("statvfs {}", probe.display()));
    }
    let stat = unsafe { stat.assume_init() };
    let available_blocks = stat.f_bavail as u128;
    let fragment_size = stat.f_frsize as u128;
    let available_bytes = available_blocks
        .saturating_mul(fragment_size)
        .min(u64::MAX as u128);
    Ok(available_bytes as u64)
}

#[cfg(not(unix))]
fn available_bytes_for_path_impl(_path: &Path) -> Result<u64> {
    Ok(u64::MAX)
}

fn is_low_disk_internal_artifact(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| {
            matches!(
                name,
                "csm_backpressure_state.json" | "csm_low_disk_recovery_manifest.json"
            )
        })
}

fn existing_probe_path(path: &Path) -> &Path {
    let mut probe = if path.exists() {
        path
    } else {
        path.parent().unwrap_or_else(|| Path::new("."))
    };
    while !probe.exists() {
        probe = probe.parent().unwrap_or_else(|| Path::new("."));
        if probe == Path::new(".") {
            break;
        }
    }
    probe
}

fn infer_state_root(path: &Path) -> PathBuf {
    if path
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|name| name.to_str())
        == Some("safe_fail_artifacts")
    {
        return path
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
    }
    path.parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf()
}

fn artifact_ref(state_root: &Path, path: &Path) -> String {
    path.strip_prefix(state_root)
        .ok()
        .and_then(|stripped| stripped.to_str())
        .unwrap_or_else(|| {
            path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown")
        })
        .to_string()
}

fn minimal_checkpoint_bundle(state_root: &Path) -> Vec<Value> {
    [
        ("status", "status.json"),
        ("daemon_status", "daemon_status.json"),
        ("continuity_checkpoint", "continuity_checkpoint.json"),
        (
            "continuity_replay_manifest",
            "continuity_replay_manifest.json",
        ),
        ("safe_fail_bundle", "safe_fail_bundle.json"),
        ("operator_events_tail", "operator_events.jsonl"),
    ]
    .into_iter()
    .map(|(role, reference)| {
        let path = state_root.join(reference);
        json!({
            "role": role,
            "ref": reference,
            "status": if path.exists() { "retained" } else { "missing" },
            "bytes": fs::metadata(path).map(|metadata| metadata.len()).ok()
        })
    })
    .collect()
}

fn append_low_disk_event_raw(path: &Path, value: &Value) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::long_lived_agent::{
        AgentCheckpointSpec, AgentSpec, AgentStatusState, HeartbeatSpec, LeaseRecord, StatusError,
        WorkflowSpec,
    };
    use crate::observability::test_env_lock;
    use chrono::Duration as ChronoDuration;
    use std::env;
    use std::ffi::OsString;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Arc, Barrier, MutexGuard};

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
                ADL_CSM_DISK_FLOOR_BYTES,
                ADL_CSM_TEST_AVAILABLE_BYTES,
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
                checkpoint: AgentCheckpointSpec::default(),
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
    fn storage_json_atomic_writes_use_unique_temp_paths_under_concurrency() {
        let root = temp_dir("concurrent-json-write");
        let path = root.join("state/csm_low_disk_recovery_manifest.json");
        let writers = 8;
        let barrier = Arc::new(Barrier::new(writers));
        let mut handles = Vec::new();

        for index in 0..writers {
            let path = path.clone();
            let barrier = Arc::clone(&barrier);
            handles.push(std::thread::spawn(move || {
                barrier.wait();
                write_json_pretty_without_preflight(
                    &path,
                    &json!({
                        "schema": CSM_LOW_DISK_RECOVERY_MANIFEST_SCHEMA,
                        "writer": index
                    }),
                )
                .expect("concurrent json write should not collide on temp path");
            }));
        }

        for handle in handles {
            handle.join().expect("writer thread should not panic");
        }

        let persisted: Value = read_json_required(&path).expect("read final json");
        assert_eq!(persisted["schema"], CSM_LOW_DISK_RECOVERY_MANIFEST_SCHEMA);
        assert!(persisted["writer"].as_u64().is_some());
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
            classification: "operator_stop_requested".to_string(),
            mode: "stop_before_next_cycle".to_string(),
            requested_at: Utc::now(),
        };
        write_json_pretty(&stop_path(&loaded), &stop).expect("write stop");
        let persisted_stop = read_stop(&loaded).expect("read stop").expect("stop exists");
        assert_eq!(persisted_stop.reason, "operator pause");
    }

    #[test]
    fn low_disk_preflight_records_degraded_state_before_required_write() {
        let _guard = MultiEnvGuard::set_all(&[
            (ADL_CSM_DISK_FLOOR_BYTES, "4096"),
            (ADL_CSM_TEST_AVAILABLE_BYTES, "1024"),
        ]);
        let root = temp_dir("low-disk-preflight");
        let loaded = sample_loaded(&root);
        fs::create_dir_all(&loaded.state_root).expect("state root");

        write_json_pretty(
            &continuity_checkpoint_path(&loaded),
            &json!({
                "schema": "adl.long_lived_agent_continuity_checkpoint.v1",
                "state": "idle"
            }),
        )
        .expect("write checkpoint under injected low disk");

        let backpressure: Value =
            read_json_required(&csm_backpressure_state_path(&loaded)).expect("backpressure state");
        assert_eq!(backpressure["storage_pressure"]["state"], "low_disk");
        assert_eq!(
            backpressure["safe_fail_action"]["action"],
            "preserve_minimal_checkpoint_bundle"
        );
        let manifest: Value = read_json_required(&csm_low_disk_recovery_manifest_path(&loaded))
            .expect("low disk recovery manifest");
        assert_eq!(manifest["schema"], CSM_LOW_DISK_RECOVERY_MANIFEST_SCHEMA);
        assert_eq!(
            manifest["recovery_pointer"]["checkpoint_ref"],
            "continuity_checkpoint.json"
        );
        assert_eq!(
            manifest["storage_pressure"]["target_ref"],
            "continuity_checkpoint.json"
        );
        let events = fs::read_to_string(operator_events_path(&loaded)).expect("operator events");
        assert!(events.contains("\"event\":\"low_disk_preflight\""));
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
