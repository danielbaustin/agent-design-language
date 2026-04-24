//! Serializable configuration and state types for long-lived agents.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

/// Parsed configuration for one long-lived agent instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    pub schema: String,
    pub agent_instance_id: String,
    pub display_name: String,
    pub state_root: PathBuf,
    pub workflow: WorkflowSpec,
    pub heartbeat: HeartbeatSpec,
    #[serde(default)]
    pub safety: Value,
    #[serde(default)]
    pub memory: Value,
}

/// Workflow selection for a long-lived agent spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowSpec {
    pub kind: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub path: Option<PathBuf>,
    #[serde(default)]
    pub run_args: Value,
}

/// Heartbeat configuration for recurring cycle execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatSpec {
    #[serde(default)]
    pub interval_secs: Option<u64>,
    #[serde(default)]
    pub max_cycles: Option<u64>,
    #[serde(default)]
    pub stale_lease_after_secs: Option<u64>,
}

/// Finite state for a running long-lived agent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatusState {
    NotStarted,
    Idle,
    Leased,
    RunningCycle,
    Stopped,
    Failed,
    Completed,
}

/// Active lease details for one cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseRecord {
    pub schema: String,
    pub agent_instance_id: String,
    pub lease_id: String,
    pub cycle_id: String,
    pub owner_pid: u32,
    pub hostname: String,
    pub started_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub status: String,
}

/// Error payload for stop records in status and run history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusError {
    pub class: String,
    pub message: String,
}

/// Canonical status checkpoint written during agent runs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusRecord {
    pub schema: String,
    pub agent_instance_id: String,
    pub state: AgentStatusState,
    pub last_cycle_id: Option<String>,
    pub last_cycle_status: Option<String>,
    pub completed_cycle_count: u64,
    #[serde(default)]
    pub consecutive_failure_count: u64,
    pub active_lease: Option<LeaseRecord>,
    pub stop_requested: bool,
    pub last_error: Option<StatusError>,
    #[serde(default)]
    pub safety_policy: Value,
    pub updated_at: DateTime<Utc>,
}

/// Stop request artifact persisted by operator/API calls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopRecord {
    pub schema: String,
    pub agent_instance_id: String,
    pub reason: String,
    #[serde(default = "super::default_requested_by")]
    pub requested_by: String,
    #[serde(default = "super::default_stop_mode")]
    pub mode: String,
    pub requested_at: DateTime<Utc>,
}

/// Resolved and normalized spec plus state paths after loading.
#[derive(Debug, Clone)]
pub struct LoadedAgentSpec {
    pub spec: AgentSpec,
    pub spec_path: PathBuf,
    pub state_root: PathBuf,
}

/// Tick execution options for one iteration.
#[derive(Debug, Clone, Copy, Default)]
pub struct TickOptions {
    pub recover_stale_lease: bool,
}

/// Runtime control options for agent execution loops.
#[derive(Debug, Clone, Copy)]
pub struct RunOptions {
    pub max_cycles: u64,
    pub interval_secs: Option<u64>,
    pub no_sleep: bool,
    pub recover_stale_lease: bool,
}

/// Inspection options for selecting a specific cycle.
#[derive(Debug, Clone, Default)]
pub struct InspectOptions {
    pub cycle_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct LedgerCursor {
    pub(crate) latest_cycle_id: Option<String>,
    pub(crate) latest_status: Option<String>,
    pub(crate) count: u64,
    pub(crate) max_cycle_number: u64,
}
