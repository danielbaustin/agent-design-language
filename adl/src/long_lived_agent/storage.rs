use super::schema::OPERATOR_EVENT_SCHEMA;
use super::types::{LeaseRecord, LoadedAgentSpec, StatusRecord, StopRecord};
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
    write_json_pretty(&status_path(loaded), status)
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
