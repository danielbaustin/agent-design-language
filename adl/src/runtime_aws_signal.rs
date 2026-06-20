use crate::long_lived_agent::{AgentStatusState, LoadedAgentSpec, StatusRecord};
use crate::observability::emit_event;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

const AWS_SIGNAL_SCHEMA_VERSION: &str = "adl.runtime.aws_signal.v1";
const HEARTBEAT_TARGET_KIND: &str = "cloudwatch_logs";
const MOCK_SIGNAL_ARTIFACT: &str = "aws_runtime_heartbeat_mock.jsonl";
const HEARTBEAT_CURSOR_ARTIFACT: &str = "aws_runtime_heartbeat_cursor.json";
const HEARTBEAT_CURSOR_SCHEMA: &str = "adl.runtime.aws_signal_heartbeat_cursor.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AwsSignalMode {
    Disabled,
    Mock,
    Live,
}

#[derive(Debug, Clone)]
struct HeartbeatPublisherConfig {
    mode: AwsSignalMode,
    configured: bool,
    region: Option<String>,
    target_kind: String,
    approved: bool,
    log_group_configured: bool,
    log_stream_configured: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishDisposition {
    Skipped,
    PublishedMock,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PublishOutcome {
    pub(crate) disposition: PublishDisposition,
    pub(crate) failure_class: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HeartbeatCursor {
    schema: String,
    next_heartbeat_seq: u64,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct RuntimeAwsSignalEnvelope {
    pub(crate) schema_version: String,
    pub(crate) signal_kind: String,
    pub(crate) runtime_id: String,
    pub(crate) agent_id: String,
    pub(crate) cycle_id: String,
    pub(crate) heartbeat_seq: u64,
    pub(crate) status: String,
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) capabilities: Vec<String>,
    pub(crate) failure_class: Option<String>,
    pub(crate) correlation_id: String,
    pub(crate) projection_level: String,
    pub(crate) transport: RuntimeAwsSignalTransport,
    pub(crate) payload: RuntimeHeartbeatPayload,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct RuntimeAwsSignalTransport {
    pub(crate) mode: String,
    pub(crate) target_kind: String,
    pub(crate) region: Option<String>,
    pub(crate) approved: bool,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct RuntimeHeartbeatPayload {
    pub(crate) state: String,
    pub(crate) elapsed_ms: i64,
    pub(crate) next_cycle_hint: String,
    pub(crate) stop_requested: bool,
    pub(crate) lease_state: String,
}

impl HeartbeatPublisherConfig {
    fn from_env() -> Self {
        let mode_env = env::var("ADL_AWS_SIGNAL_MODE").ok();
        let mode = match mode_env
            .as_deref()
            .map(str::trim)
            .map(str::to_ascii_lowercase)
            .as_deref()
        {
            Some("mock") => AwsSignalMode::Mock,
            Some("live") => AwsSignalMode::Live,
            _ => AwsSignalMode::Disabled,
        };
        let region = env::var("ADL_AWS_REGION")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let target_kind = env::var("ADL_AWS_HEARTBEAT_TARGET")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| HEARTBEAT_TARGET_KIND.to_string());
        let approved = env::var("ADL_AWS_SIGNAL_APPROVED")
            .ok()
            .as_deref()
            .map(str::trim)
            .map(|value| matches!(value, "1" | "true" | "TRUE" | "yes" | "YES"))
            .unwrap_or(false);
        let log_group_configured = env::var("ADL_AWS_HEARTBEAT_LOG_GROUP")
            .ok()
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false);
        let log_stream_configured = env::var("ADL_AWS_HEARTBEAT_LOG_STREAM")
            .ok()
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false);
        Self {
            mode,
            configured: mode_env.is_some(),
            region,
            target_kind,
            approved,
            log_group_configured,
            log_stream_configured,
        }
    }

    fn mode_label(&self) -> &'static str {
        match self.mode {
            AwsSignalMode::Disabled => "disabled",
            AwsSignalMode::Mock => "mock",
            AwsSignalMode::Live => "live",
        }
    }

    fn live_block_reason(&self) -> &'static str {
        if !self.approved {
            "aws_signal_live_not_approved"
        } else if self.region.is_none() {
            "aws_signal_region_missing"
        } else if self.target_kind != HEARTBEAT_TARGET_KIND {
            "aws_signal_unsupported_target"
        } else if !self.log_group_configured {
            "aws_signal_log_group_missing"
        } else if !self.log_stream_configured {
            "aws_signal_log_stream_missing"
        } else {
            "aws_signal_live_transport_not_implemented"
        }
    }
}

pub(crate) fn mock_signal_artifact_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join(MOCK_SIGNAL_ARTIFACT)
}

fn heartbeat_cursor_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join(HEARTBEAT_CURSOR_ARTIFACT)
}

pub(crate) fn publish_runtime_heartbeat_signal(
    loaded: &LoadedAgentSpec,
    status: &StatusRecord,
) -> PublishOutcome {
    let config = HeartbeatPublisherConfig::from_env();
    if !config.configured {
        return PublishOutcome {
            disposition: PublishDisposition::Skipped,
            failure_class: None,
        };
    }

    if matches!(config.mode, AwsSignalMode::Disabled) {
        emit_event(
            "agent",
            "aws_runtime_heartbeat",
            "skipped",
            &[
                ("mode", config.mode_label()),
                ("target_kind", config.target_kind.as_str()),
                ("runtime_id", loaded.spec.agent_instance_id.as_str()),
                ("cycle_id", cycle_id_for_status(status).as_str()),
                ("heartbeat_seq", "not_allocated"),
                ("signal_status", runtime_signal_status(status)),
            ],
        );
        return PublishOutcome {
            disposition: PublishDisposition::Skipped,
            failure_class: None,
        };
    }

    if matches!(config.mode, AwsSignalMode::Mock | AwsSignalMode::Live)
        && config.target_kind != HEARTBEAT_TARGET_KIND
    {
        let failure_class = "aws_signal_unsupported_target";
        emit_publish_failure(
            &config,
            loaded.spec.agent_instance_id.as_str(),
            cycle_id_for_status(status).as_str(),
            "not_allocated",
            runtime_signal_status(status),
            failure_class,
        );
        return PublishOutcome {
            disposition: PublishDisposition::Blocked,
            failure_class: Some(failure_class.to_string()),
        };
    }

    let heartbeat_seq = match reserve_heartbeat_seq(loaded) {
        Ok(sequence) => sequence,
        Err(_) => {
            let failure_class = "aws_signal_cursor_write_failed";
            emit_publish_failure(
                &config,
                loaded.spec.agent_instance_id.as_str(),
                cycle_id_for_status(status).as_str(),
                "not_allocated",
                runtime_signal_status(status),
                failure_class,
            );
            return PublishOutcome {
                disposition: PublishDisposition::Blocked,
                failure_class: Some(failure_class.to_string()),
            };
        }
    };

    let envelope = build_runtime_heartbeat_envelope(loaded, status, &config, heartbeat_seq);
    let heartbeat_seq_label = envelope.heartbeat_seq.to_string();

    match config.mode {
        AwsSignalMode::Disabled => unreachable!("disabled mode returns before sequence allocation"),
        AwsSignalMode::Mock => match append_mock_signal(loaded, &envelope) {
            Ok(()) => {
                emit_event(
                    "agent",
                    "aws_runtime_heartbeat",
                    "completed",
                    &[
                        ("mode", config.mode_label()),
                        ("target_kind", config.target_kind.as_str()),
                        ("runtime_id", envelope.runtime_id.as_str()),
                        ("cycle_id", envelope.cycle_id.as_str()),
                        ("heartbeat_seq", heartbeat_seq_label.as_str()),
                        ("signal_status", envelope.status.as_str()),
                    ],
                );
                PublishOutcome {
                    disposition: PublishDisposition::PublishedMock,
                    failure_class: None,
                }
            }
            Err(_) => {
                let failure_class = "aws_signal_mock_write_failed";
                emit_publish_failure(
                    &config,
                    envelope.runtime_id.as_str(),
                    envelope.cycle_id.as_str(),
                    heartbeat_seq_label.as_str(),
                    envelope.status.as_str(),
                    failure_class,
                );
                PublishOutcome {
                    disposition: PublishDisposition::Blocked,
                    failure_class: Some(failure_class.to_string()),
                }
            }
        },
        AwsSignalMode::Live => {
            let failure_class = config.live_block_reason();
            emit_publish_failure(
                &config,
                envelope.runtime_id.as_str(),
                envelope.cycle_id.as_str(),
                heartbeat_seq_label.as_str(),
                envelope.status.as_str(),
                failure_class,
            );
            PublishOutcome {
                disposition: PublishDisposition::Blocked,
                failure_class: Some(failure_class.to_string()),
            }
        }
    }
}

fn emit_publish_failure(
    config: &HeartbeatPublisherConfig,
    runtime_id: &str,
    cycle_id: &str,
    heartbeat_seq: &str,
    signal_status: &str,
    failure_class: &str,
) {
    emit_event(
        "agent",
        "aws_runtime_heartbeat",
        "failed",
        &[
            ("mode", config.mode_label()),
            ("target_kind", config.target_kind.as_str()),
            ("runtime_id", runtime_id),
            ("cycle_id", cycle_id),
            ("heartbeat_seq", heartbeat_seq),
            ("signal_status", signal_status),
            ("failure_class", failure_class),
        ],
    );
}

fn build_runtime_heartbeat_envelope(
    loaded: &LoadedAgentSpec,
    status: &StatusRecord,
    config: &HeartbeatPublisherConfig,
    heartbeat_seq: u64,
) -> RuntimeAwsSignalEnvelope {
    let cycle_id = cycle_id_for_status(status);
    let runtime_id = loaded.spec.agent_instance_id.clone();
    let agent_id = loaded
        .spec
        .workflow
        .name
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| loaded.spec.display_name.clone());
    RuntimeAwsSignalEnvelope {
        schema_version: AWS_SIGNAL_SCHEMA_VERSION.to_string(),
        signal_kind: "heartbeat".to_string(),
        runtime_id: runtime_id.clone(),
        agent_id,
        cycle_id: cycle_id.clone(),
        heartbeat_seq,
        status: runtime_signal_status(status).to_string(),
        timestamp: status.updated_at,
        capabilities: vec![
            "long_lived_agent".to_string(),
            "heartbeat".to_string(),
            loaded.spec.workflow.kind.clone(),
        ],
        failure_class: status.last_error.as_ref().map(|err| err.class.clone()),
        correlation_id: format!("heartbeat:{runtime_id}:{cycle_id}:{heartbeat_seq}"),
        projection_level: "operations_safe".to_string(),
        transport: RuntimeAwsSignalTransport {
            mode: config.mode_label().to_string(),
            target_kind: config.target_kind.clone(),
            region: config.region.clone(),
            approved: config.approved,
        },
        payload: RuntimeHeartbeatPayload {
            state: agent_state_label(&status.state).to_string(),
            elapsed_ms: elapsed_ms(status),
            next_cycle_hint: next_cycle_hint(status).to_string(),
            stop_requested: status.stop_requested,
            lease_state: lease_state_label(status).to_string(),
        },
    }
}

fn append_mock_signal(loaded: &LoadedAgentSpec, envelope: &RuntimeAwsSignalEnvelope) -> Result<()> {
    let path = mock_signal_artifact_path(loaded);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed creating {}", parent.display()))?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .with_context(|| format!("failed opening {}", path.display()))?;
    serde_json::to_writer(&mut file, envelope)
        .with_context(|| format!("failed writing {}", path.display()))?;
    file.write_all(b"\n")
        .with_context(|| format!("failed finalizing {}", path.display()))?;
    Ok(())
}

fn cycle_id_for_status(status: &StatusRecord) -> String {
    status
        .last_cycle_id
        .clone()
        .or_else(|| {
            status
                .active_lease
                .as_ref()
                .map(|lease| lease.cycle_id.clone())
        })
        .unwrap_or_else(|| "not_applicable".to_string())
}

fn runtime_signal_status(status: &StatusRecord) -> &'static str {
    match status.state {
        AgentStatusState::NotStarted => "started",
        AgentStatusState::RunningCycle | AgentStatusState::Leased => "heartbeat",
        AgentStatusState::Idle | AgentStatusState::Completed | AgentStatusState::Stopped => {
            "completed"
        }
        AgentStatusState::Failed => "failed",
    }
}

fn next_cycle_hint(status: &StatusRecord) -> &'static str {
    match status.state {
        AgentStatusState::RunningCycle | AgentStatusState::Leased => "cycle_in_progress",
        AgentStatusState::Idle | AgentStatusState::Completed => "sleep_until_next_heartbeat",
        AgentStatusState::Stopped => "stop_requested",
        AgentStatusState::Failed => "inspect_status_and_cycle_artifacts",
        AgentStatusState::NotStarted => "await_first_cycle",
    }
}

fn lease_state_label(status: &StatusRecord) -> &'static str {
    if status.active_lease.is_some() {
        "active"
    } else if status.stop_requested {
        "stop_requested"
    } else {
        "clear"
    }
}

fn agent_state_label(state: &AgentStatusState) -> &'static str {
    match state {
        AgentStatusState::NotStarted => "not_started",
        AgentStatusState::Idle => "idle",
        AgentStatusState::Leased => "leased",
        AgentStatusState::RunningCycle => "running_cycle",
        AgentStatusState::Stopped => "stopped",
        AgentStatusState::Failed => "failed",
        AgentStatusState::Completed => "completed",
    }
}

fn elapsed_ms(status: &StatusRecord) -> i64 {
    status
        .active_lease
        .as_ref()
        .map(|lease| {
            status
                .updated_at
                .signed_duration_since(lease.started_at)
                .num_milliseconds()
                .max(0)
        })
        .unwrap_or(0)
}

fn reserve_heartbeat_seq(loaded: &LoadedAgentSpec) -> Result<u64> {
    let path = heartbeat_cursor_path(loaded);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed creating {}", parent.display()))?;
    }
    let mut cursor = if path.exists() {
        let raw = fs::read_to_string(&path)
            .with_context(|| format!("failed reading {}", path.display()))?;
        serde_json::from_str::<HeartbeatCursor>(&raw)
            .with_context(|| format!("failed parsing {}", path.display()))?
    } else {
        HeartbeatCursor {
            schema: HEARTBEAT_CURSOR_SCHEMA.to_string(),
            next_heartbeat_seq: 1,
        }
    };
    let reserved = cursor.next_heartbeat_seq;
    cursor.next_heartbeat_seq = cursor.next_heartbeat_seq.saturating_add(1);
    let file =
        File::create(&path).with_context(|| format!("failed creating {}", path.display()))?;
    serde_json::to_writer_pretty(&file, &cursor)
        .with_context(|| format!("failed writing {}", path.display()))?;
    OpenOptions::new()
        .append(true)
        .open(&path)
        .with_context(|| format!("failed finalizing {}", path.display()))?
        .write_all(b"\n")
        .with_context(|| format!("failed finalizing {}", path.display()))?;
    Ok(reserved)
}
