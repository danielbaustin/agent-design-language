use crate::agent_comms::{
    AcipAddressKindV1, AcipIntentV1, AcipMessageEnvelopeV1, AcipRouteClassV1,
};
use crate::long_lived_agent::{AgentStatusState, LoadedAgentSpec, StatusRecord};
use crate::observability::emit_event;
use anyhow::{Context, Result};
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_cloudwatchlogs as cloudwatchlogs;
use aws_sdk_eventbridge as eventbridge;
use aws_sdk_lambda as lambda;
use aws_sdk_sns as sns;
use chrono::{DateTime, Utc};
use cloudwatchlogs::operation::RequestId;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

const AWS_SIGNAL_SCHEMA_VERSION: &str = "adl.runtime.aws_signal.v1";
const HEARTBEAT_TARGET_KIND: &str = "cloudwatch_logs";
const MOCK_SIGNAL_ARTIFACT: &str = "aws_runtime_heartbeat_mock.jsonl";
#[allow(dead_code)]
const ACIP_SNS_TARGET_KIND: &str = "sns";
#[allow(dead_code)]
const ACIP_SNS_MOCK_SIGNAL_ARTIFACT: &str = "aws_acip_sns_projection_mock.jsonl";
const CSM_NOTICE_MOCK_SIGNAL_ARTIFACT: &str = "aws_csm_governed_notice_mock.jsonl";
const CSM_NOTICE_SNS_MOCK_SIGNAL_ARTIFACT: &str = "aws_csm_governed_notice_sns_mock.jsonl";
const CSM_NOTICE_CONTROL_PLANE_MOCK_ARTIFACT: &str = "csm_governed_notice_control_plane_mock.jsonl";
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
    profile: Option<String>,
    log_group: Option<String>,
    log_stream: Option<String>,
    log_group_configured: bool,
    log_stream_configured: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct AcipProjectionPublisherConfig {
    mode: AwsSignalMode,
    configured: bool,
    region: Option<String>,
    approved: bool,
    profile: Option<String>,
    topic_arn: Option<String>,
    topic_configured: bool,
}

#[derive(Debug, Clone)]
struct ControlPlaneNoticeConfig {
    mode: AwsSignalMode,
    configured: bool,
    approved: bool,
    target: String,
    region: Option<String>,
    profile: Option<String>,
    endpoint: Option<String>,
    lambda_function: Option<String>,
    event_bus: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublishDisposition {
    Skipped,
    PublishedMock,
    PublishedLive,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishOutcome {
    pub disposition: PublishDisposition,
    pub failure_class: Option<String>,
    pub provider_message_id: Option<String>,
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

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AcipSnsProjectionRequest<'a> {
    pub runtime_id: &'a str,
    pub agent_id: &'a str,
    pub cycle_id: Option<&'a str>,
    pub message: &'a AcipMessageEnvelopeV1,
    pub route_class: AcipRouteClassV1,
    pub projection_level: &'a str,
    pub message_ref: &'a str,
    pub trace_ref: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub(crate) struct AcipAwsSignalEnvelope {
    pub(crate) schema_version: String,
    pub(crate) signal_kind: String,
    pub(crate) runtime_id: String,
    pub(crate) agent_id: String,
    pub(crate) cycle_id: String,
    pub(crate) heartbeat_seq: Option<u64>,
    pub(crate) status: String,
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) capabilities: Vec<String>,
    pub(crate) failure_class: Option<String>,
    pub(crate) correlation_id: String,
    pub(crate) projection_level: String,
    pub(crate) transport: RuntimeAwsSignalTransport,
    pub(crate) payload: AcipSnsProjectionPayload,
}

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub(crate) struct AcipSnsProjectionPayload {
    pub(crate) message_kind: String,
    pub(crate) route_class: String,
    pub(crate) sender_class: String,
    pub(crate) recipient_class: String,
    pub(crate) delivery_outcome: String,
    pub(crate) message_ref: String,
    pub(crate) summary: Option<String>,
    pub(crate) trace_ref: Option<String>,
    pub(crate) content_sha256: Option<String>,
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
        let profile = env::var("ADL_AWS_PROFILE")
            .ok()
            .or_else(|| env::var("AWS_PROFILE").ok())
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let log_group = env::var("ADL_AWS_HEARTBEAT_LOG_GROUP")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let log_stream = env::var("ADL_AWS_HEARTBEAT_LOG_STREAM")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        Self {
            mode,
            configured: mode_env.is_some(),
            region,
            target_kind,
            approved,
            profile,
            log_group_configured: log_group.is_some(),
            log_stream_configured: log_stream.is_some(),
            log_group,
            log_stream,
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
        } else if self.profile.is_none() {
            "aws_signal_profile_missing"
        } else {
            "aws_signal_live_publish_failed"
        }
    }
}

#[allow(dead_code)]
impl AcipProjectionPublisherConfig {
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
        let approved = env::var("ADL_AWS_SIGNAL_APPROVED")
            .ok()
            .as_deref()
            .map(str::trim)
            .map(|value| matches!(value, "1" | "true" | "TRUE" | "yes" | "YES"))
            .unwrap_or(false);
        let profile = env::var("ADL_AWS_PROFILE")
            .ok()
            .or_else(|| env::var("AWS_PROFILE").ok())
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let topic_arn = env::var("ADL_AWS_SNS_TOPIC_ARN")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        Self {
            mode,
            configured: mode_env.is_some(),
            region,
            approved,
            topic_configured: topic_arn.is_some(),
            profile,
            topic_arn,
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
            "aws_acip_sns_live_not_approved"
        } else if self.region.is_none() {
            "aws_acip_sns_region_missing"
        } else if self.profile.is_none() {
            "aws_acip_sns_profile_missing"
        } else if !self.topic_configured {
            "aws_acip_sns_topic_missing"
        } else {
            "aws_acip_sns_publish_failed"
        }
    }
}

impl ControlPlaneNoticeConfig {
    fn from_env() -> Self {
        let mode_env = env::var("ADL_CSM_NOTICE_CONTROL_PLANE_MODE").ok();
        let target = env::var("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET")
            .ok()
            .map(|value| value.trim().to_ascii_lowercase())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "https".to_string());
        let region = env::var("ADL_AWS_REGION")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let profile = env::var("ADL_AWS_PROFILE")
            .ok()
            .or_else(|| env::var("AWS_PROFILE").ok())
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let endpoint = env::var("ADL_CSM_NOTICE_CONTROL_PLANE_URL")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let lambda_function = env::var("ADL_CSM_NOTICE_LAMBDA_FUNCTION")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let event_bus = env::var("ADL_CSM_NOTICE_EVENT_BUS")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
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
        let approved = env::var("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED")
            .ok()
            .as_deref()
            .map(str::trim)
            .map(|value| matches!(value, "1" | "true" | "TRUE" | "yes" | "YES"))
            .unwrap_or(false);
        Self {
            mode,
            configured: mode_env.is_some()
                || endpoint.is_some()
                || lambda_function.is_some()
                || event_bus.is_some(),
            approved,
            target,
            region,
            profile,
            endpoint,
            lambda_function,
            event_bus,
        }
    }

    fn mode_label(&self) -> &'static str {
        match self.mode {
            AwsSignalMode::Disabled => "disabled",
            AwsSignalMode::Mock => "mock",
            AwsSignalMode::Live => "live",
        }
    }

    fn target_hash(&self) -> Option<String> {
        self.lambda_function
            .as_ref()
            .or(self.event_bus.as_ref())
            .or(self.endpoint.as_ref())
            .map(|target| {
                let mut hasher = Sha256::new();
                hasher.update(target.as_bytes());
                format!("{:x}", hasher.finalize())
            })
    }

    fn live_block_reason(&self) -> &'static str {
        if !self.approved {
            "control_plane_live_not_approved"
        } else if self.target == "lambda" {
            if self.region.is_none() {
                "control_plane_lambda_region_missing"
            } else if self.profile.is_none() {
                "control_plane_lambda_profile_missing"
            } else if self.lambda_function.is_none() {
                "control_plane_lambda_function_missing"
            } else {
                "control_plane_lambda_invoke_failed"
            }
        } else if self.target == "eventbridge" {
            if self.region.is_none() {
                "control_plane_eventbridge_region_missing"
            } else if self.profile.is_none() {
                "control_plane_eventbridge_profile_missing"
            } else if self.event_bus.is_none() {
                "control_plane_event_bus_missing"
            } else {
                "control_plane_eventbridge_put_failed"
            }
        } else if self.endpoint.is_none() {
            "control_plane_url_missing"
        } else {
            "control_plane_http_transport_failed"
        }
    }
}

pub(crate) fn mock_signal_artifact_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join(MOCK_SIGNAL_ARTIFACT)
}

#[allow(dead_code)]
pub(crate) fn acip_mock_signal_artifact_path(root: &Path) -> PathBuf {
    root.join(ACIP_SNS_MOCK_SIGNAL_ARTIFACT)
}

pub(crate) fn csm_notice_mock_signal_artifact_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join(CSM_NOTICE_MOCK_SIGNAL_ARTIFACT)
}

pub(crate) fn csm_notice_sns_mock_signal_artifact_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join(CSM_NOTICE_SNS_MOCK_SIGNAL_ARTIFACT)
}

pub(crate) fn csm_notice_control_plane_mock_artifact_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded
        .state_root
        .join(CSM_NOTICE_CONTROL_PLANE_MOCK_ARTIFACT)
}

fn heartbeat_cursor_path(loaded: &LoadedAgentSpec) -> PathBuf {
    loaded.state_root.join(HEARTBEAT_CURSOR_ARTIFACT)
}

pub(crate) fn publish_csm_governed_notice_signal(
    loaded: &LoadedAgentSpec,
    notice: &serde_json::Value,
) -> Vec<serde_json::Value> {
    vec![
        publish_csm_notice_cloudwatch(loaded, notice),
        publish_csm_notice_sns(loaded, notice),
        publish_csm_notice_control_plane(loaded, notice),
    ]
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
            provider_message_id: None,
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
            provider_message_id: None,
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
            provider_message_id: None,
        };
    }

    match config.mode {
        AwsSignalMode::Disabled => unreachable!("disabled mode returns before sequence allocation"),
        AwsSignalMode::Mock => {
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
                        provider_message_id: None,
                    };
                }
            };

            let envelope = build_runtime_heartbeat_envelope(loaded, status, &config, heartbeat_seq);
            let heartbeat_seq_label = envelope.heartbeat_seq.to_string();
            match append_mock_signal(loaded, &envelope) {
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
                        provider_message_id: None,
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
                        provider_message_id: None,
                    }
                }
            }
        }
        AwsSignalMode::Live => {
            let live_block_reason = config.live_block_reason();
            if live_block_reason != "aws_signal_live_publish_failed" {
                emit_publish_failure(
                    &config,
                    loaded.spec.agent_instance_id.as_str(),
                    cycle_id_for_status(status).as_str(),
                    "not_allocated",
                    runtime_signal_status(status),
                    live_block_reason,
                );
                return PublishOutcome {
                    disposition: PublishDisposition::Blocked,
                    failure_class: Some(live_block_reason.to_string()),
                    provider_message_id: None,
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
                        provider_message_id: None,
                    };
                }
            };
            let envelope = build_runtime_heartbeat_envelope(loaded, status, &config, heartbeat_seq);
            let heartbeat_seq_label = envelope.heartbeat_seq.to_string();
            match publish_live_cloudwatch_heartbeat(&config, &envelope) {
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
                        disposition: PublishDisposition::PublishedLive,
                        failure_class: None,
                        provider_message_id: None,
                    }
                }
                Err(_) => {
                    let failure_class = "aws_signal_live_publish_failed";
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
                        provider_message_id: None,
                    }
                }
            }
        }
    }
}

fn publish_csm_notice_cloudwatch(
    loaded: &LoadedAgentSpec,
    notice: &serde_json::Value,
) -> serde_json::Value {
    let config = HeartbeatPublisherConfig::from_env();
    let base = csm_notice_attempt_base("cloudwatch_logs", config.mode_label());
    if !config.configured {
        return csm_notice_attempt(base, "not_configured", None, None);
    }
    if matches!(config.mode, AwsSignalMode::Disabled) {
        return csm_notice_attempt(base, "skipped_disabled", None, None);
    }
    if config.target_kind != HEARTBEAT_TARGET_KIND {
        return csm_notice_attempt(
            base,
            "blocked",
            Some("aws_signal_unsupported_target".to_string()),
            None,
        );
    }
    let envelope = csm_notice_envelope(loaded, notice, "cloudwatch_logs", config.mode_label());
    match config.mode {
        AwsSignalMode::Disabled => unreachable!("disabled mode returned earlier"),
        AwsSignalMode::Mock => {
            match append_jsonl_value(&csm_notice_mock_signal_artifact_path(loaded), &envelope) {
                Ok(()) => csm_notice_attempt(
                    base,
                    "published_mock",
                    None,
                    Some("aws_csm_governed_notice_mock.jsonl".to_string()),
                ),
                Err(_) => csm_notice_attempt(
                    base,
                    "failed",
                    Some("aws_signal_mock_write_failed".to_string()),
                    None,
                ),
            }
        }
        AwsSignalMode::Live => {
            let live_block_reason = config.live_block_reason();
            if live_block_reason != "aws_signal_live_publish_failed" {
                return csm_notice_attempt(
                    base,
                    "blocked",
                    Some(live_block_reason.to_string()),
                    None,
                );
            }
            let log_group = match config.log_group.as_deref() {
                Some(value) => value,
                None => {
                    return csm_notice_attempt(
                        base,
                        "blocked",
                        Some("aws_signal_log_group_missing".to_string()),
                        None,
                    )
                }
            };
            let log_stream = match config.log_stream.as_deref() {
                Some(value) => value,
                None => {
                    return csm_notice_attempt(
                        base,
                        "blocked",
                        Some("aws_signal_log_stream_missing".to_string()),
                        None,
                    )
                }
            };
            let message = match serde_json::to_string(&envelope) {
                Ok(value) => value,
                Err(_) => {
                    return csm_notice_attempt(
                        base,
                        "failed",
                        Some("csm_notice_serialization_failed".to_string()),
                        None,
                    )
                }
            };
            match run_cloudwatch_put(
                &config,
                log_group,
                log_stream,
                Utc::now().timestamp_millis(),
                message,
            ) {
                Ok(provider_receipt_id) => {
                    csm_notice_attempt(base, "published_live", None, provider_receipt_id)
                }
                Err(_) => csm_notice_attempt(
                    base,
                    "failed",
                    Some("aws_signal_live_publish_failed".to_string()),
                    None,
                ),
            }
        }
    }
}

fn publish_csm_notice_sns(
    loaded: &LoadedAgentSpec,
    notice: &serde_json::Value,
) -> serde_json::Value {
    let config = AcipProjectionPublisherConfig::from_env();
    let base = csm_notice_attempt_base("acip_sns", config.mode_label());
    if !config.configured {
        return csm_notice_attempt(base, "not_configured", None, None);
    }
    if matches!(config.mode, AwsSignalMode::Disabled) {
        return csm_notice_attempt(base, "skipped_disabled", None, None);
    }
    let envelope = csm_notice_envelope(loaded, notice, "acip_sns", config.mode_label());
    match config.mode {
        AwsSignalMode::Disabled => unreachable!("disabled mode returned earlier"),
        AwsSignalMode::Mock => {
            match append_jsonl_value(&csm_notice_sns_mock_signal_artifact_path(loaded), &envelope) {
                Ok(()) => csm_notice_attempt(
                    base,
                    "published_mock",
                    None,
                    Some("aws_csm_governed_notice_sns_mock.jsonl".to_string()),
                ),
                Err(_) => csm_notice_attempt(
                    base,
                    "failed",
                    Some("aws_acip_sns_mock_write_failed".to_string()),
                    None,
                ),
            }
        }
        AwsSignalMode::Live => {
            let failure_class = config.live_block_reason();
            if failure_class != "aws_acip_sns_publish_failed" {
                return csm_notice_attempt(base, "blocked", Some(failure_class.to_string()), None);
            }
            let topic_arn = match config.topic_arn.as_deref() {
                Some(value) => value,
                None => {
                    return csm_notice_attempt(
                        base,
                        "blocked",
                        Some("aws_acip_sns_topic_missing".to_string()),
                        None,
                    )
                }
            };
            let message = match serde_json::to_string(&envelope) {
                Ok(value) => value,
                Err(_) => {
                    return csm_notice_attempt(
                        base,
                        "failed",
                        Some("csm_notice_serialization_failed".to_string()),
                        None,
                    )
                }
            };
            let correlation_id = notice
                .get("notice_id")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("csm-notice")
                .to_string();
            match run_sns_publish_with_signal_kind(
                &config,
                topic_arn,
                message,
                correlation_id,
                "csm_governed_notice",
            ) {
                Ok(message_id) => {
                    csm_notice_attempt(base, "published_live", None, Some(message_id))
                }
                Err(_) => csm_notice_attempt(
                    base,
                    "failed",
                    Some("aws_acip_sns_publish_failed".to_string()),
                    None,
                ),
            }
        }
    }
}

fn publish_csm_notice_control_plane(
    loaded: &LoadedAgentSpec,
    notice: &serde_json::Value,
) -> serde_json::Value {
    let config = ControlPlaneNoticeConfig::from_env();
    let mut base = csm_notice_attempt_base("cloudfront_control_plane", config.mode_label());
    if let Some(object) = base.as_object_mut() {
        object.insert(
            "target_kind".to_string(),
            serde_json::Value::String(config.target.clone()),
        );
        object.insert(
            "target_sha256".to_string(),
            config
                .target_hash()
                .map(serde_json::Value::String)
                .unwrap_or(serde_json::Value::Null),
        );
        object.insert("dependency".to_string(), serde_json::json!("#4915"));
    }
    if !config.configured {
        return csm_notice_attempt(
            base,
            "not_configured",
            Some("control_plane_hook_pending_4915_or_env_missing".to_string()),
            None,
        );
    }
    if matches!(config.mode, AwsSignalMode::Disabled) {
        return csm_notice_attempt(base, "skipped_disabled", None, None);
    }
    let envelope = csm_notice_envelope(
        loaded,
        notice,
        "cloudfront_control_plane",
        config.mode_label(),
    );
    match config.mode {
        AwsSignalMode::Disabled => unreachable!("disabled mode returned earlier"),
        AwsSignalMode::Mock => match append_jsonl_value(
            &csm_notice_control_plane_mock_artifact_path(loaded),
            &envelope,
        ) {
            Ok(()) => csm_notice_attempt(
                base,
                "published_mock",
                None,
                Some("csm_governed_notice_control_plane_mock.jsonl".to_string()),
            ),
            Err(_) => csm_notice_attempt(
                base,
                "failed",
                Some("control_plane_mock_write_failed".to_string()),
                None,
            ),
        },
        AwsSignalMode::Live => {
            let live_block_reason = config.live_block_reason();
            if config.target == "lambda" {
                if live_block_reason != "control_plane_lambda_invoke_failed" {
                    return csm_notice_attempt(
                        base,
                        "blocked",
                        Some(live_block_reason.to_string()),
                        None,
                    );
                }
                return match invoke_csm_notice_lambda(&config, &envelope) {
                    Ok(request_id) => csm_notice_attempt(base, "published_live", None, request_id),
                    Err(_) => csm_notice_attempt(
                        base,
                        "failed",
                        Some("control_plane_lambda_invoke_failed".to_string()),
                        None,
                    ),
                };
            }
            if config.target == "eventbridge" {
                if live_block_reason != "control_plane_eventbridge_put_failed" {
                    return csm_notice_attempt(
                        base,
                        "blocked",
                        Some(live_block_reason.to_string()),
                        None,
                    );
                }
                return match put_csm_notice_eventbridge(&config, &envelope) {
                    Ok(event_id) => csm_notice_attempt(base, "published_live", None, event_id),
                    Err(_) => csm_notice_attempt(
                        base,
                        "failed",
                        Some("control_plane_eventbridge_put_failed".to_string()),
                        None,
                    ),
                };
            }
            if live_block_reason != "control_plane_http_transport_failed" {
                return csm_notice_attempt(
                    base,
                    "blocked",
                    Some(live_block_reason.to_string()),
                    None,
                );
            }
            let Some(endpoint) = config.endpoint.as_deref() else {
                return csm_notice_attempt(
                    base,
                    "blocked",
                    Some("control_plane_url_missing".to_string()),
                    None,
                );
            };
            let idempotency_key = envelope
                .get("correlation_id")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("csm-notice")
                .to_string();
            match reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .and_then(|client| {
                    client
                        .post(endpoint)
                        .header("Idempotency-Key", idempotency_key.as_str())
                        .json(&envelope)
                        .send()
                }) {
                Ok(response) if response.status().is_success() => {
                    let status = response.status().as_u16();
                    let provider_receipt_id = ["x-amzn-requestid", "x-request-id", "request-id"]
                        .into_iter()
                        .find_map(|name| {
                            response
                                .headers()
                                .get(name)
                                .and_then(|value| value.to_str().ok())
                                .map(str::to_string)
                        })
                        .unwrap_or_else(|| {
                            format!("http-{status}-idempotency-ack:{idempotency_key}")
                        });
                    csm_notice_attempt(base, "published_live", None, Some(provider_receipt_id))
                }
                Ok(response) => csm_notice_attempt(
                    base,
                    "failed",
                    Some(format!(
                        "control_plane_http_status_{}",
                        response.status().as_u16()
                    )),
                    None,
                ),
                Err(_) => csm_notice_attempt(
                    base,
                    "failed",
                    Some("control_plane_http_transport_failed".to_string()),
                    None,
                ),
            }
        }
    }
}

fn invoke_csm_notice_lambda(
    config: &ControlPlaneNoticeConfig,
    envelope: &serde_json::Value,
) -> Result<Option<String>> {
    let region = config
        .region
        .as_deref()
        .context("control plane lambda region missing")?;
    let function_name = config
        .lambda_function
        .as_deref()
        .context("control plane lambda function missing")?;
    let payload = serde_json::to_vec(envelope).context("serialize CSM notice lambda payload")?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("create lambda publish runtime")?;
    runtime.block_on(async move {
        let shared_config =
            load_control_plane_aws_config(region, control_plane_profile_name(config)).await;
        let client = lambda::Client::new(&shared_config);
        let response = client
            .invoke()
            .function_name(function_name)
            .invocation_type(lambda::types::InvocationType::RequestResponse)
            .payload(lambda::primitives::Blob::new(payload))
            .send()
            .await
            .context("invoke CSM notice lambda")?;
        let status = response.status_code();
        if !(200..300).contains(&status) {
            anyhow::bail!("lambda invoke returned status {status}");
        }
        Ok(response
            .executed_version()
            .map(|version| format!("lambda_executed_version:{version}")))
    })
}

fn put_csm_notice_eventbridge(
    config: &ControlPlaneNoticeConfig,
    envelope: &serde_json::Value,
) -> Result<Option<String>> {
    let region = config
        .region
        .as_deref()
        .context("control plane EventBridge region missing")?;
    let event_bus = config
        .event_bus
        .as_deref()
        .context("control plane EventBridge bus missing")?;
    let detail = serde_json::to_string(envelope).context("serialize CSM notice event detail")?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("create EventBridge publish runtime")?;
    runtime.block_on(async move {
        let shared_config =
            load_control_plane_aws_config(region, control_plane_profile_name(config)).await;
        let client = eventbridge::Client::new(&shared_config);
        let entry = eventbridge::types::PutEventsRequestEntry::builder()
            .event_bus_name(event_bus)
            .source("adl.csm")
            .detail_type("CSM Governed Notice")
            .detail(detail)
            .build();
        let response = client
            .put_events()
            .entries(entry)
            .send()
            .await
            .context("put CSM notice EventBridge event")?;
        if response.failed_entry_count() > 0 {
            anyhow::bail!("EventBridge returned failed entries");
        }
        let event_id = response
            .entries()
            .first()
            .and_then(|entry| entry.event_id())
            .map(ToString::to_string);
        Ok(event_id)
    })
}

fn control_plane_profile_name(config: &ControlPlaneNoticeConfig) -> Option<&str> {
    config.profile.as_deref()
}

async fn load_control_plane_aws_config(
    region: &str,
    profile_name: Option<&str>,
) -> aws_config::SdkConfig {
    let region_provider =
        RegionProviderChain::first_try(Some(aws_config::Region::new(region.to_string())));
    let timeout_config = aws_config::timeout::TimeoutConfig::builder()
        .connect_timeout(Duration::from_secs(5))
        .operation_timeout(Duration::from_secs(20))
        .operation_attempt_timeout(Duration::from_secs(10))
        .build();
    let loader = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .timeout_config(timeout_config);
    match profile_name {
        Some(profile_name) => loader.profile_name(profile_name).load().await,
        None => loader.load().await,
    }
}

fn csm_notice_envelope(
    loaded: &LoadedAgentSpec,
    notice: &serde_json::Value,
    target_kind: &str,
    mode: &str,
) -> serde_json::Value {
    serde_json::json!({
        "schema_version": AWS_SIGNAL_SCHEMA_VERSION,
        "signal_kind": "csm_governed_notice",
        "runtime_id": loaded.spec.agent_instance_id,
        "agent_id": loaded.spec.workflow.name.clone().unwrap_or_else(|| loaded.spec.display_name.clone()),
        "notice_id": notice.get("notice_id").cloned().unwrap_or(serde_json::Value::Null),
        "notice_kind": notice.get("notice_kind").cloned().unwrap_or(serde_json::Value::Null),
        "severity": notice.get("severity").cloned().unwrap_or(serde_json::Value::Null),
        "trigger": notice.get("trigger").cloned().unwrap_or(serde_json::Value::Null),
        "timestamp": Utc::now(),
        "correlation_id": notice.get("notice_id").cloned().unwrap_or(serde_json::json!("csm-notice")),
        "projection_level": "operations_safe",
        "transport": {
            "mode": mode,
            "target_kind": target_kind
        },
        "payload": {
            "recoverable_state": notice.get("recoverable_state").cloned().unwrap_or(serde_json::Value::Null),
            "safe_fail_ref": "safe_fail_bundle.json",
            "notice_ref": "csm_governed_notice_latest.json",
            "notice_ledger_ref": "csm_governed_notices.jsonl",
            "details": notice.get("details").cloned().unwrap_or(serde_json::Value::Null)
        }
    })
}

fn csm_notice_attempt_base(channel: &str, mode: &str) -> serde_json::Value {
    serde_json::json!({
        "channel": channel,
        "mode": mode,
        "attempted_at": Utc::now()
    })
}

fn csm_notice_attempt(
    mut base: serde_json::Value,
    status: &str,
    failure_class: Option<String>,
    artifact_or_message: Option<String>,
) -> serde_json::Value {
    if let Some(object) = base.as_object_mut() {
        object.insert("status".to_string(), serde_json::json!(status));
        if let Some(failure_class) = failure_class {
            object.insert(
                "failure_class".to_string(),
                serde_json::json!(failure_class),
            );
        }
        if let Some(artifact_or_message) = artifact_or_message {
            let value = artifact_or_message;
            let key = if status == "published_live" {
                "provider_message_id"
            } else {
                "artifact_ref"
            };
            object.insert(key.to_string(), serde_json::json!(value));
        }
    }
    base
}

#[allow(dead_code)]
pub fn publish_acip_sns_projection_signal(
    output_root: &Path,
    request: &AcipSnsProjectionRequest<'_>,
) -> PublishOutcome {
    let config = AcipProjectionPublisherConfig::from_env();
    if !config.configured {
        return PublishOutcome {
            disposition: PublishDisposition::Skipped,
            failure_class: None,
            provider_message_id: None,
        };
    }

    let correlation_id = acip_correlation_id(request.message);
    let projection_level = request.projection_level;
    if request.route_class != AcipRouteClassV1::CrossBoundaryDeferred
        || !matches!(projection_level, "delivery_metadata" | "content_summary")
    {
        let failure_class = "projection_denied";
        emit_acip_publish_failure(
            &config,
            request.runtime_id,
            request.cycle_id.unwrap_or("not_applicable"),
            correlation_id.as_str(),
            failure_class,
        );
        return PublishOutcome {
            disposition: PublishDisposition::Blocked,
            failure_class: Some(failure_class.to_string()),
            provider_message_id: None,
        };
    }

    if matches!(config.mode, AwsSignalMode::Disabled) {
        emit_event(
            "agent",
            "aws_acip_sns_projection",
            "skipped",
            &[
                ("mode", config.mode_label()),
                ("target_kind", ACIP_SNS_TARGET_KIND),
                ("runtime_id", request.runtime_id),
                ("cycle_id", request.cycle_id.unwrap_or("not_applicable")),
                ("correlation_id", correlation_id.as_str()),
                ("projection_level", projection_level),
            ],
        );
        return PublishOutcome {
            disposition: PublishDisposition::Skipped,
            failure_class: None,
            provider_message_id: None,
        };
    }

    let envelope = build_acip_sns_projection_envelope(request, &config);
    match config.mode {
        AwsSignalMode::Disabled => unreachable!("disabled mode returns before publish"),
        AwsSignalMode::Mock => match append_mock_acip_signal(output_root, &envelope) {
            Ok(()) => {
                emit_event(
                    "agent",
                    "aws_acip_sns_projection",
                    "completed",
                    &[
                        ("mode", config.mode_label()),
                        ("target_kind", ACIP_SNS_TARGET_KIND),
                        ("runtime_id", envelope.runtime_id.as_str()),
                        ("cycle_id", envelope.cycle_id.as_str()),
                        ("correlation_id", envelope.correlation_id.as_str()),
                        ("projection_level", envelope.projection_level.as_str()),
                    ],
                );
                PublishOutcome {
                    disposition: PublishDisposition::PublishedMock,
                    failure_class: None,
                    provider_message_id: None,
                }
            }
            Err(_) => {
                let failure_class = "publish_failed";
                emit_acip_publish_failure(
                    &config,
                    envelope.runtime_id.as_str(),
                    envelope.cycle_id.as_str(),
                    envelope.correlation_id.as_str(),
                    failure_class,
                );
                PublishOutcome {
                    disposition: PublishDisposition::Blocked,
                    failure_class: Some(failure_class.to_string()),
                    provider_message_id: None,
                }
            }
        },
        AwsSignalMode::Live => {
            let failure_class = config.live_block_reason();
            if failure_class != "aws_acip_sns_publish_failed" {
                emit_acip_publish_failure(
                    &config,
                    envelope.runtime_id.as_str(),
                    envelope.cycle_id.as_str(),
                    envelope.correlation_id.as_str(),
                    failure_class,
                );
                return PublishOutcome {
                    disposition: PublishDisposition::Blocked,
                    failure_class: Some(failure_class.to_string()),
                    provider_message_id: None,
                };
            }
            match publish_live_sns_acip_projection(&config, &envelope) {
                Ok(message_id) => {
                    emit_event(
                        "agent",
                        "aws_acip_sns_projection",
                        "completed",
                        &[
                            ("mode", config.mode_label()),
                            ("target_kind", ACIP_SNS_TARGET_KIND),
                            ("runtime_id", envelope.runtime_id.as_str()),
                            ("cycle_id", envelope.cycle_id.as_str()),
                            ("correlation_id", envelope.correlation_id.as_str()),
                            ("projection_level", envelope.projection_level.as_str()),
                        ],
                    );
                    PublishOutcome {
                        disposition: PublishDisposition::PublishedLive,
                        failure_class: None,
                        provider_message_id: Some(message_id),
                    }
                }
                Err(_) => {
                    let failure_class = "aws_acip_sns_publish_failed";
                    emit_acip_publish_failure(
                        &config,
                        envelope.runtime_id.as_str(),
                        envelope.cycle_id.as_str(),
                        envelope.correlation_id.as_str(),
                        failure_class,
                    );
                    PublishOutcome {
                        disposition: PublishDisposition::Blocked,
                        failure_class: Some(failure_class.to_string()),
                        provider_message_id: None,
                    }
                }
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

#[allow(dead_code)]
fn emit_acip_publish_failure(
    config: &AcipProjectionPublisherConfig,
    runtime_id: &str,
    cycle_id: &str,
    correlation_id: &str,
    failure_class: &str,
) {
    emit_event(
        "agent",
        "aws_acip_sns_projection",
        "failed",
        &[
            ("mode", config.mode_label()),
            ("target_kind", ACIP_SNS_TARGET_KIND),
            ("runtime_id", runtime_id),
            ("cycle_id", cycle_id),
            ("correlation_id", correlation_id),
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

#[allow(dead_code)]
fn build_acip_sns_projection_envelope(
    request: &AcipSnsProjectionRequest<'_>,
    config: &AcipProjectionPublisherConfig,
) -> AcipAwsSignalEnvelope {
    let cycle_id = request.cycle_id.unwrap_or("not_applicable");
    let correlation_id = acip_correlation_id(request.message);
    let projection_level = request.projection_level.to_string();
    let include_content_fields = projection_level == "content_summary";
    AcipAwsSignalEnvelope {
        schema_version: AWS_SIGNAL_SCHEMA_VERSION.to_string(),
        signal_kind: "acip_projection".to_string(),
        runtime_id: request.runtime_id.to_string(),
        agent_id: request.agent_id.to_string(),
        cycle_id: cycle_id.to_string(),
        heartbeat_seq: None,
        status: "completed".to_string(),
        timestamp: parse_acip_timestamp(request.message).unwrap_or_else(Utc::now),
        capabilities: vec![
            "acip_projection".to_string(),
            "sns_delivery_bridge".to_string(),
        ],
        failure_class: None,
        correlation_id,
        projection_level: projection_level.clone(),
        transport: RuntimeAwsSignalTransport {
            mode: config.mode_label().to_string(),
            target_kind: ACIP_SNS_TARGET_KIND.to_string(),
            region: config.region.clone(),
            approved: config.approved,
        },
        payload: AcipSnsProjectionPayload {
            message_kind: acip_intent_label(&request.message.intent).to_string(),
            route_class: acip_route_class_label(&request.route_class).to_string(),
            sender_class: acip_address_class_label(&request.message.sender.kind).to_string(),
            recipient_class: "approval_gated_external_subscriber".to_string(),
            delivery_outcome: match config.mode {
                AwsSignalMode::Mock => "mock_projected".to_string(),
                AwsSignalMode::Live => "published".to_string(),
                AwsSignalMode::Disabled => "publish_skipped".to_string(),
            },
            message_ref: request.message_ref.to_string(),
            summary: include_content_fields
                .then(|| acip_projection_summary(request.message, &projection_level)),
            trace_ref: request.trace_ref.map(str::to_string),
            content_sha256: include_content_fields
                .then(|| acip_projection_content_sha256(request.message)),
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

fn publish_live_cloudwatch_heartbeat(
    config: &HeartbeatPublisherConfig,
    envelope: &RuntimeAwsSignalEnvelope,
) -> Result<()> {
    let log_group = config
        .log_group
        .as_deref()
        .context("ADL_AWS_HEARTBEAT_LOG_GROUP is required for live heartbeat publish")?;
    let log_stream = config
        .log_stream
        .as_deref()
        .context("ADL_AWS_HEARTBEAT_LOG_STREAM is required for live heartbeat publish")?;
    let message = serde_json::to_string(envelope)?;
    let timestamp = envelope.timestamp.timestamp_millis();
    run_cloudwatch_put(config, log_group, log_stream, timestamp, message).map(|_| ())
}

fn run_cloudwatch_put(
    config: &HeartbeatPublisherConfig,
    log_group: &str,
    log_stream: &str,
    timestamp: i64,
    message: String,
) -> Result<Option<String>> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("failed to initialize heartbeat CloudWatch runtime")?;
    runtime.block_on(async move {
        let region = config
            .region
            .as_deref()
            .context("ADL_AWS_REGION is required for live heartbeat publish")?;
        let region_provider =
            RegionProviderChain::first_try(Some(aws_config::Region::new(region.to_string())));
        let timeout_config = aws_config::timeout::TimeoutConfig::builder()
            .connect_timeout(Duration::from_secs(5))
            .operation_timeout(Duration::from_secs(20))
            .operation_attempt_timeout(Duration::from_secs(10))
            .build();
        let loader = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .timeout_config(timeout_config);
        let shared_config = match config.profile.as_deref() {
            Some(profile_name) => loader.profile_name(profile_name).load().await,
            None => loader.load().await,
        };
        let client = cloudwatchlogs::Client::new(&shared_config);
        let event = cloudwatchlogs::types::InputLogEvent::builder()
            .timestamp(timestamp)
            .message(message)
            .build()
            .context("failed to build CloudWatch heartbeat log event")?;
        let response = client
            .put_log_events()
            .log_group_name(log_group)
            .log_stream_name(log_stream)
            .log_events(event)
            .send()
            .await
            .context("failed to publish runtime heartbeat to CloudWatch Logs")?;
        Ok::<Option<String>, anyhow::Error>(
            response
                .request_id()
                .map(ToString::to_string)
                .or_else(|| response.next_sequence_token().map(ToString::to_string)),
        )
    })
}

fn publish_live_sns_acip_projection(
    config: &AcipProjectionPublisherConfig,
    envelope: &AcipAwsSignalEnvelope,
) -> Result<String> {
    let topic_arn = config
        .topic_arn
        .as_deref()
        .context("ADL_AWS_SNS_TOPIC_ARN is required for live ACIP SNS publish")?;
    let message = serde_json::to_string(envelope)?;
    run_sns_publish_with_signal_kind(
        config,
        topic_arn,
        message,
        envelope.correlation_id.clone(),
        "acip_projection",
    )
}

fn run_sns_publish_with_signal_kind(
    config: &AcipProjectionPublisherConfig,
    topic_arn: &str,
    message: String,
    correlation_id: String,
    signal_kind: &str,
) -> Result<String> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("failed to initialize ACIP SNS runtime")?;
    runtime.block_on(async move {
        let region = config
            .region
            .as_deref()
            .context("ADL_AWS_REGION is required for live ACIP SNS publish")?;
        let region_provider =
            RegionProviderChain::first_try(Some(aws_config::Region::new(region.to_string())));
        let timeout_config = aws_config::timeout::TimeoutConfig::builder()
            .connect_timeout(Duration::from_secs(5))
            .operation_timeout(Duration::from_secs(20))
            .operation_attempt_timeout(Duration::from_secs(10))
            .build();
        let loader = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .timeout_config(timeout_config);
        let shared_config = match config.profile.as_deref() {
            Some(profile_name) => loader.profile_name(profile_name).load().await,
            None => loader.load().await,
        };
        let client = sns::Client::new(&shared_config);
        let response = client
            .publish()
            .topic_arn(topic_arn)
            .message(message)
            .message_attributes(
                "schema_version",
                sns::types::MessageAttributeValue::builder()
                    .data_type("String")
                    .string_value(AWS_SIGNAL_SCHEMA_VERSION)
                    .build()
                    .context("failed to build SNS schema_version attribute")?,
            )
            .message_attributes(
                "signal_kind",
                sns::types::MessageAttributeValue::builder()
                    .data_type("String")
                    .string_value(signal_kind)
                    .build()
                    .context("failed to build SNS signal_kind attribute")?,
            )
            .message_attributes(
                "correlation_id",
                sns::types::MessageAttributeValue::builder()
                    .data_type("String")
                    .string_value(correlation_id)
                    .build()
                    .context("failed to build SNS correlation_id attribute")?,
            )
            .send()
            .await
            .context("failed to publish ACIP projection to SNS")?;
        response
            .message_id()
            .map(str::to_string)
            .context("SNS publish response did not include a message id")
    })
}

fn append_jsonl_value(path: &Path, value: &serde_json::Value) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed creating {}", parent.display()))?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("failed opening {}", path.display()))?;
    serde_json::to_writer(&mut file, value)
        .with_context(|| format!("failed writing {}", path.display()))?;
    file.write_all(b"\n")
        .with_context(|| format!("failed finalizing {}", path.display()))?;
    Ok(())
}

#[allow(dead_code)]
fn append_mock_acip_signal(output_root: &Path, envelope: &AcipAwsSignalEnvelope) -> Result<()> {
    let path = acip_mock_signal_artifact_path(output_root);
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

#[allow(dead_code)]
fn acip_correlation_id(message: &AcipMessageEnvelopeV1) -> String {
    message
        .correlation_id
        .clone()
        .unwrap_or_else(|| message.message_id.clone())
}

#[allow(dead_code)]
fn parse_acip_timestamp(message: &AcipMessageEnvelopeV1) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(&message.timestamp_utc)
        .ok()
        .map(|value| value.with_timezone(&Utc))
}

#[allow(dead_code)]
fn acip_intent_label(intent: &AcipIntentV1) -> &'static str {
    match intent {
        AcipIntentV1::Conversation => "conversation",
        AcipIntentV1::Consultation => "consultation",
        AcipIntentV1::InvocationSetup => "invocation_setup",
        AcipIntentV1::ReviewRequest => "review_request",
        AcipIntentV1::CodingRequest => "coding_request",
        AcipIntentV1::Delegation => "delegation",
        AcipIntentV1::Negotiation => "negotiation",
    }
}

#[allow(dead_code)]
fn acip_route_class_label(route_class: &AcipRouteClassV1) -> &'static str {
    match route_class {
        AcipRouteClassV1::LocalOnly => "local_only",
        AcipRouteClassV1::CrossBoundaryDeferred => "cross_boundary_deferred",
    }
}

#[allow(dead_code)]
fn acip_address_class_label(kind: &AcipAddressKindV1) -> &'static str {
    match kind {
        AcipAddressKindV1::Agent => "workflow_agent",
        AcipAddressKindV1::Group => "workflow_group",
    }
}

#[allow(dead_code)]
fn acip_projection_summary(message: &AcipMessageEnvelopeV1, projection_level: &str) -> String {
    format!(
        "{} ACIP message projected as bounded {} only for approval-gated external delivery",
        acip_intent_label(&message.intent),
        projection_level
    )
}

#[allow(dead_code)]
fn acip_projection_content_sha256(message: &AcipMessageEnvelopeV1) -> String {
    let mut hasher = Sha256::new();
    hasher.update(message.content.as_bytes());
    hasher.update([0xff]);
    for payload in &message.payload_refs {
        hasher.update(payload.content_sha256.as_bytes());
        hasher.update([0xfe]);
    }
    format!("{:x}", hasher.finalize())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::long_lived_agent::AgentCheckpointSpec;
    use crate::long_lived_agent::{
        AgentSpec, AgentStatusState, HeartbeatSpec, LeaseRecord, StatusError, StatusRecord,
        WorkflowSpec,
    };
    use crate::observability::test_env_lock;
    use chrono::Duration as ChronoDuration;
    use serde_json::json;
    use std::ffi::OsString;
    use std::path::Path;
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
                "ADL_AWS_PROFILE",
                "AWS_PROFILE",
                "ADL_AWS_SNS_TOPIC_ARN",
                "ADL_CSM_NOTICE_CONTROL_PLANE_MODE",
                "ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED",
                "ADL_CSM_NOTICE_CONTROL_PLANE_TARGET",
                "ADL_CSM_NOTICE_CONTROL_PLANE_URL",
                "ADL_CSM_NOTICE_LAMBDA_FUNCTION",
                "ADL_CSM_NOTICE_EVENT_BUS",
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
            "adl-runtime-aws-signal-{prefix}-{}-{}",
            std::process::id(),
            TEMP_SEQ.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn sample_loaded(root: &Path) -> LoadedAgentSpec {
        let spec_path = root.join("agent.yaml");
        LoadedAgentSpec {
            spec: AgentSpec {
                schema: "adl.long_lived_agent_spec.v1".to_string(),
                agent_instance_id: "runtime-agent".to_string(),
                display_name: "Runtime Agent".to_string(),
                state_root: PathBuf::from("state"),
                workflow: WorkflowSpec {
                    kind: "demo_adapter".to_string(),
                    name: Some("runtime-heartbeat".to_string()),
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
            spec_path,
            state_root: root.join("state"),
        }
    }

    fn sample_status(state: AgentStatusState) -> StatusRecord {
        StatusRecord {
            schema: "adl.long_lived_agent_status.v1".to_string(),
            agent_instance_id: "runtime-agent".to_string(),
            state,
            last_cycle_id: Some("cycle-000123".to_string()),
            last_cycle_status: Some("success".to_string()),
            completed_cycle_count: 3,
            consecutive_failure_count: 0,
            active_lease: None,
            stop_requested: false,
            last_error: None,
            safety_policy: json!({}),
            updated_at: Utc::now(),
        }
    }

    fn sample_acip_message() -> AcipMessageEnvelopeV1 {
        AcipMessageEnvelopeV1 {
            schema_version: "acip.message.v1".to_string(),
            message_id: "msg-acip-0007".to_string(),
            conversation_id: "conv-acip-0003".to_string(),
            timestamp_utc: "2026-06-20T20:41:15Z".to_string(),
            monotonic_order: 7,
            sender: crate::agent_comms::AcipAddressV1 {
                kind: AcipAddressKindV1::Agent,
                id: "planner.agent".to_string(),
            },
            recipient: crate::agent_comms::AcipAddressV1 {
                kind: AcipAddressKindV1::Group,
                id: "external-subscribers".to_string(),
            },
            intent: AcipIntentV1::Delegation,
            visibility: crate::agent_comms::AcipVisibilityV1::Shared,
            trace_requirement: crate::agent_comms::AcipTraceRequirementV1::Summary,
            content: "private delegation content should not appear in the SNS projection"
                .to_string(),
            payload_refs: vec![crate::agent_comms::AcipPayloadRefV1 {
                payload_kind: "delegation_result".to_string(),
                payload_ref: "runtime/comms/delegation/result.json".to_string(),
                media_type: "application/json".to_string(),
                content_sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                    .to_string(),
                byte_length: 128,
                inline_summary: Some("bounded delegation summary".to_string()),
            }],
            artifact_refs: vec!["runtime/comms/delegation/result.json".to_string()],
            attachments: Vec::new(),
            authority_scope: None,
            correlation_id: Some("acip-msg-0007".to_string()),
            prior_message_id: Some("msg-acip-0006".to_string()),
        }
    }

    fn sample_csm_notice() -> serde_json::Value {
        json!({
            "schema": "adl.csm.governed_notice.v1",
            "notice_id": "notice-runtime-agent-bounded-test-supervisor-failure",
            "notice_kind": "shutdown",
            "severity": "critical",
            "trigger": "bounded_test_supervisor_failure",
            "recoverable_state": {
                "state": "failed",
                "status_ref": "status.json",
                "continuity_checkpoint_ref": "continuity_checkpoint.json",
                "safe_fail_ref": "safe_fail_bundle.json"
            },
            "details": {
                "restart_count": 1,
                "bounded_test_restart_limit": 1
            }
        })
    }

    #[test]
    fn runtime_aws_signal_config_parses_modes_and_live_block_reasons() {
        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_AWS_SIGNAL_MODE", "live"),
                ("ADL_AWS_SIGNAL_APPROVED", "true"),
                ("ADL_AWS_REGION", "us-west-2"),
            ]);
            let config = HeartbeatPublisherConfig::from_env();
            assert_eq!(config.mode, AwsSignalMode::Live);
            assert!(config.configured);
            assert_eq!(config.mode_label(), "live");
            assert_eq!(config.live_block_reason(), "aws_signal_log_group_missing");
        }

        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_AWS_SIGNAL_MODE", "live"),
                ("ADL_AWS_SIGNAL_APPROVED", "true"),
                ("ADL_AWS_REGION", "us-west-2"),
                ("ADL_AWS_HEARTBEAT_LOG_GROUP", "group"),
            ]);
            let config = HeartbeatPublisherConfig::from_env();
            assert_eq!(config.live_block_reason(), "aws_signal_log_stream_missing");
        }

        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_AWS_SIGNAL_MODE", "live"),
                ("ADL_AWS_SIGNAL_APPROVED", "true"),
                ("ADL_AWS_REGION", "us-west-2"),
                ("ADL_AWS_HEARTBEAT_LOG_GROUP", "group"),
                ("ADL_AWS_HEARTBEAT_LOG_STREAM", "stream"),
            ]);
            let config = HeartbeatPublisherConfig::from_env();
            assert_eq!(config.live_block_reason(), "aws_signal_profile_missing");
        }

        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_AWS_SIGNAL_MODE", "live"),
                ("ADL_AWS_SIGNAL_APPROVED", "true"),
                ("ADL_AWS_REGION", "us-west-2"),
                ("ADL_AWS_HEARTBEAT_LOG_GROUP", "group"),
                ("ADL_AWS_HEARTBEAT_LOG_STREAM", "stream"),
                ("ADL_AWS_PROFILE", "agent-logic-admin"),
            ]);
            let config = HeartbeatPublisherConfig::from_env();
            assert_eq!(config.profile.as_deref(), Some("agent-logic-admin"));
            assert_eq!(config.live_block_reason(), "aws_signal_live_publish_failed");
        }
    }

    #[test]
    fn runtime_aws_signal_helper_labels_cover_status_variants() {
        let mut status = sample_status(AgentStatusState::RunningCycle);
        let lease_started_at = status.updated_at - ChronoDuration::seconds(12);
        status.active_lease = Some(LeaseRecord {
            schema: "adl.long_lived_agent_lease.v1".to_string(),
            agent_instance_id: "runtime-agent".to_string(),
            lease_id: "lease-1".to_string(),
            cycle_id: "cycle-lease".to_string(),
            owner_pid: 42,
            hostname: "local".to_string(),
            started_at: lease_started_at,
            expires_at: status.updated_at + ChronoDuration::seconds(60),
            status: "active".to_string(),
        });
        assert_eq!(cycle_id_for_status(&status), "cycle-000123");
        assert_eq!(runtime_signal_status(&status), "heartbeat");
        assert_eq!(next_cycle_hint(&status), "cycle_in_progress");
        assert_eq!(lease_state_label(&status), "active");
        assert_eq!(agent_state_label(&status.state), "running_cycle");
        assert!(elapsed_ms(&status) >= 12_000);

        let mut failed = sample_status(AgentStatusState::Failed);
        failed.last_cycle_id = None;
        failed.stop_requested = true;
        failed.active_lease = None;
        failed.last_error = Some(StatusError {
            class: "workflow_failed".to_string(),
            message: "cycle failed".to_string(),
        });
        assert_eq!(cycle_id_for_status(&failed), "not_applicable");
        assert_eq!(runtime_signal_status(&failed), "failed");
        assert_eq!(
            next_cycle_hint(&failed),
            "inspect_status_and_cycle_artifacts"
        );
        assert_eq!(lease_state_label(&failed), "stop_requested");
        assert_eq!(agent_state_label(&failed.state), "failed");

        let idle = sample_status(AgentStatusState::Idle);
        assert_eq!(runtime_signal_status(&idle), "completed");
        assert_eq!(next_cycle_hint(&idle), "sleep_until_next_heartbeat");
    }

    #[test]
    fn runtime_aws_signal_mock_publish_writes_envelope_and_cursor() {
        let root = temp_dir("mock");
        let loaded = sample_loaded(&root);
        let _guard = MultiEnvGuard::set_all(&[
            ("ADL_AWS_SIGNAL_MODE", "mock"),
            ("ADL_AWS_REGION", "us-west-2"),
        ]);

        let outcome = publish_runtime_heartbeat_signal(
            &loaded,
            &sample_status(AgentStatusState::RunningCycle),
        );
        assert_eq!(outcome.disposition, PublishDisposition::PublishedMock);
        assert_eq!(outcome.failure_class, None);

        let artifact = fs::read_to_string(mock_signal_artifact_path(&loaded)).expect("artifact");
        let envelope: serde_json::Value =
            serde_json::from_str(artifact.lines().next().expect("jsonl line"))
                .expect("parse envelope");
        assert_eq!(envelope["schema_version"], AWS_SIGNAL_SCHEMA_VERSION);
        assert_eq!(envelope["signal_kind"], "heartbeat");
        assert_eq!(envelope["transport"]["mode"], "mock");
        assert_eq!(envelope["transport"]["target_kind"], HEARTBEAT_TARGET_KIND);
        assert_eq!(envelope["heartbeat_seq"], 1);
        assert_eq!(
            envelope["correlation_id"],
            "heartbeat:runtime-agent:cycle-000123:1"
        );
        assert_eq!(envelope["payload"]["state"], "running_cycle");
        assert_eq!(envelope["payload"]["next_cycle_hint"], "cycle_in_progress");

        let cursor: HeartbeatCursor = serde_json::from_str(
            &fs::read_to_string(heartbeat_cursor_path(&loaded)).expect("cursor"),
        )
        .expect("parse cursor");
        assert_eq!(cursor.schema, HEARTBEAT_CURSOR_SCHEMA);
        assert_eq!(cursor.next_heartbeat_seq, 2);
    }

    #[test]
    fn runtime_aws_signal_publish_handles_disabled_unsupported_and_live_blocked_modes() {
        let root = temp_dir("publish-modes");
        let loaded = sample_loaded(&root);

        {
            let _guard = MultiEnvGuard::set_all(&[("ADL_AWS_SIGNAL_MODE", "disabled")]);
            let disabled =
                publish_runtime_heartbeat_signal(&loaded, &sample_status(AgentStatusState::Idle));
            assert_eq!(disabled.disposition, PublishDisposition::Skipped);
            assert!(!mock_signal_artifact_path(&loaded).exists());
            assert!(!heartbeat_cursor_path(&loaded).exists());
        }

        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_AWS_SIGNAL_MODE", "mock"),
                ("ADL_AWS_HEARTBEAT_TARGET", "sns"),
            ]);
            let unsupported =
                publish_runtime_heartbeat_signal(&loaded, &sample_status(AgentStatusState::Idle));
            assert_eq!(unsupported.disposition, PublishDisposition::Blocked);
            assert_eq!(
                unsupported.failure_class.as_deref(),
                Some("aws_signal_unsupported_target")
            );
            assert!(!heartbeat_cursor_path(&loaded).exists());
        }

        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_AWS_SIGNAL_MODE", "live"),
                ("ADL_AWS_REGION", "us-west-2"),
            ]);
            let blocked =
                publish_runtime_heartbeat_signal(&loaded, &sample_status(AgentStatusState::Idle));
            assert_eq!(blocked.disposition, PublishDisposition::Blocked);
            assert_eq!(
                blocked.failure_class.as_deref(),
                Some("aws_signal_live_not_approved")
            );
            assert!(!heartbeat_cursor_path(&loaded).exists());
        }
    }

    #[test]
    fn runtime_aws_signal_live_blocked_mode_preserves_existing_cursor_state() {
        let root = temp_dir("live-blocked-cursor");
        let loaded = sample_loaded(&root);

        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_AWS_SIGNAL_MODE", "mock"),
                ("ADL_AWS_REGION", "us-west-2"),
            ]);
            let outcome = publish_runtime_heartbeat_signal(
                &loaded,
                &sample_status(AgentStatusState::RunningCycle),
            );
            assert_eq!(outcome.disposition, PublishDisposition::PublishedMock);
        }

        let before: HeartbeatCursor = serde_json::from_str(
            &fs::read_to_string(heartbeat_cursor_path(&loaded)).expect("cursor before"),
        )
        .expect("parse cursor before");
        assert_eq!(before.next_heartbeat_seq, 2);

        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_AWS_SIGNAL_MODE", "live"),
                ("ADL_AWS_REGION", "us-west-2"),
                ("ADL_AWS_HEARTBEAT_TARGET", "cloudwatch_logs"),
                ("ADL_AWS_HEARTBEAT_LOG_GROUP", "private"),
                ("ADL_AWS_HEARTBEAT_LOG_STREAM", "private"),
            ]);
            let blocked =
                publish_runtime_heartbeat_signal(&loaded, &sample_status(AgentStatusState::Idle));
            assert_eq!(blocked.disposition, PublishDisposition::Blocked);
            assert_eq!(
                blocked.failure_class.as_deref(),
                Some("aws_signal_live_not_approved")
            );
        }

        let after: HeartbeatCursor = serde_json::from_str(
            &fs::read_to_string(heartbeat_cursor_path(&loaded)).expect("cursor after"),
        )
        .expect("parse cursor after");
        assert_eq!(after.next_heartbeat_seq, 2);
    }

    #[test]
    fn runtime_aws_signal_sequence_and_envelope_helpers_are_stable() {
        let root = temp_dir("sequence");
        let loaded = sample_loaded(&root);
        let first = reserve_heartbeat_seq(&loaded).expect("first seq");
        let second = reserve_heartbeat_seq(&loaded).expect("second seq");
        assert_eq!(first, 1);
        assert_eq!(second, 2);

        let config = HeartbeatPublisherConfig {
            mode: AwsSignalMode::Mock,
            configured: true,
            region: Some("us-west-2".to_string()),
            target_kind: HEARTBEAT_TARGET_KIND.to_string(),
            approved: false,
            profile: None,
            log_group: None,
            log_stream: None,
            log_group_configured: false,
            log_stream_configured: false,
        };
        let mut stopped = sample_status(AgentStatusState::Stopped);
        stopped.last_cycle_id = None;
        let envelope = build_runtime_heartbeat_envelope(&loaded, &stopped, &config, second);
        assert_eq!(envelope.agent_id, "runtime-heartbeat");
        assert_eq!(envelope.cycle_id, "not_applicable");
        assert_eq!(envelope.status, "completed");
        assert_eq!(envelope.transport.region.as_deref(), Some("us-west-2"));
        assert!(!envelope.transport.approved);
        assert_eq!(envelope.payload.state, "stopped");
        assert_eq!(envelope.payload.next_cycle_hint, "stop_requested");
        assert_eq!(envelope.payload.lease_state, "clear");
    }

    #[test]
    fn acip_sns_projection_mock_publish_writes_metadata_only_envelope() {
        let root = temp_dir("acip-mock");
        let _guard = MultiEnvGuard::set_all(&[
            ("ADL_AWS_SIGNAL_MODE", "mock"),
            ("ADL_AWS_REGION", "us-west-2"),
        ]);
        let message = sample_acip_message();
        let request = AcipSnsProjectionRequest {
            runtime_id: "runtime-fire-up-rehearsal-001",
            agent_id: "temporary-agent-alpha",
            cycle_id: Some("cycle-000042"),
            message: &message,
            route_class: AcipRouteClassV1::CrossBoundaryDeferred,
            projection_level: "delivery_metadata",
            message_ref: "acip/messages/msg-0007.json",
            trace_ref: Some("runtime/comms/trace/public_summary.json"),
        };

        let outcome = publish_acip_sns_projection_signal(&root, &request);
        assert_eq!(outcome.disposition, PublishDisposition::PublishedMock);

        let artifact = fs::read_to_string(acip_mock_signal_artifact_path(&root)).expect("artifact");
        let envelope: serde_json::Value =
            serde_json::from_str(artifact.lines().next().expect("jsonl line"))
                .expect("parse envelope");
        assert_eq!(envelope["schema_version"], AWS_SIGNAL_SCHEMA_VERSION);
        assert_eq!(envelope["signal_kind"], "acip_projection");
        assert_eq!(envelope["transport"]["target_kind"], ACIP_SNS_TARGET_KIND);
        assert_eq!(envelope["transport"]["mode"], "mock");
        assert_eq!(envelope["heartbeat_seq"], serde_json::Value::Null);
        assert_eq!(envelope["correlation_id"], "acip-msg-0007");
        assert_eq!(
            envelope["payload"]["route_class"],
            "cross_boundary_deferred"
        );
        assert_eq!(
            envelope["payload"]["recipient_class"],
            "approval_gated_external_subscriber"
        );
        assert_eq!(envelope["payload"]["delivery_outcome"], "mock_projected");
        assert_eq!(envelope["payload"]["summary"], serde_json::Value::Null);
        assert_eq!(
            envelope["payload"]["content_sha256"],
            serde_json::Value::Null
        );
    }

    #[test]
    fn acip_sns_projection_content_summary_includes_redacted_content_fields() {
        let root = temp_dir("acip-content-summary");
        let _guard = MultiEnvGuard::set_all(&[
            ("ADL_AWS_SIGNAL_MODE", "mock"),
            ("ADL_AWS_REGION", "us-west-2"),
        ]);
        let message = sample_acip_message();
        let request = AcipSnsProjectionRequest {
            runtime_id: "runtime-fire-up-rehearsal-001",
            agent_id: "temporary-agent-alpha",
            cycle_id: Some("cycle-000042"),
            message: &message,
            route_class: AcipRouteClassV1::CrossBoundaryDeferred,
            projection_level: "content_summary",
            message_ref: "acip/messages/msg-0007.json",
            trace_ref: Some("runtime/comms/trace/public_summary.json"),
        };

        let outcome = publish_acip_sns_projection_signal(&root, &request);
        assert_eq!(outcome.disposition, PublishDisposition::PublishedMock);

        let artifact = fs::read_to_string(acip_mock_signal_artifact_path(&root)).expect("artifact");
        let envelope: serde_json::Value =
            serde_json::from_str(artifact.lines().next().expect("jsonl line"))
                .expect("parse envelope");
        let summary = envelope["payload"]["summary"].as_str().expect("summary");
        assert!(summary.contains("content_summary"));
        assert!(!summary.contains("private delegation content"));
        assert_ne!(
            envelope["payload"]["content_sha256"],
            serde_json::Value::Null
        );
    }

    #[test]
    fn acip_sns_projection_rejects_local_only_route_class() {
        let root = temp_dir("acip-local-only");
        let _guard = MultiEnvGuard::set_all(&[("ADL_AWS_SIGNAL_MODE", "mock")]);
        let message = sample_acip_message();
        let request = AcipSnsProjectionRequest {
            runtime_id: "runtime-acip-local",
            agent_id: "runtime-acip-local",
            cycle_id: None,
            message: &message,
            route_class: AcipRouteClassV1::LocalOnly,
            projection_level: "delivery_metadata",
            message_ref: "acip/messages/msg-acip-0007.json",
            trace_ref: None,
        };

        let outcome = publish_acip_sns_projection_signal(&root, &request);
        assert_eq!(outcome.disposition, PublishDisposition::Blocked);
        assert_eq!(outcome.failure_class.as_deref(), Some("projection_denied"));
        assert!(!acip_mock_signal_artifact_path(&root).exists());
    }

    #[test]
    fn acip_sns_projection_disabled_and_live_modes_stay_fail_closed() {
        let root = temp_dir("acip-modes");
        let message = sample_acip_message();
        let request = AcipSnsProjectionRequest {
            runtime_id: "runtime-acip-modes",
            agent_id: "runtime-acip-modes",
            cycle_id: Some("cycle-acip"),
            message: &message,
            route_class: AcipRouteClassV1::CrossBoundaryDeferred,
            projection_level: "content_summary",
            message_ref: "acip/messages/msg-acip-0007.json",
            trace_ref: None,
        };

        {
            let _guard = MultiEnvGuard::set_all(&[("ADL_AWS_SIGNAL_MODE", "disabled")]);
            let disabled = publish_acip_sns_projection_signal(&root, &request);
            assert_eq!(disabled.disposition, PublishDisposition::Skipped);
            assert!(!acip_mock_signal_artifact_path(&root).exists());
        }

        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_AWS_SIGNAL_MODE", "live"),
                ("ADL_AWS_REGION", "us-west-2"),
                ("ADL_AWS_SIGNAL_APPROVED", "1"),
            ]);
            let blocked = publish_acip_sns_projection_signal(&root, &request);
            assert_eq!(blocked.disposition, PublishDisposition::Blocked);
            assert_eq!(
                blocked.failure_class.as_deref(),
                Some("aws_acip_sns_profile_missing")
            );
            assert_eq!(blocked.provider_message_id, None);
        }

        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_AWS_SIGNAL_MODE", "live"),
                ("ADL_AWS_REGION", "us-west-2"),
                ("ADL_AWS_SIGNAL_APPROVED", "1"),
                ("ADL_AWS_PROFILE", "agent-logic-admin"),
            ]);
            let blocked = publish_acip_sns_projection_signal(&root, &request);
            assert_eq!(blocked.disposition, PublishDisposition::Blocked);
            assert_eq!(
                blocked.failure_class.as_deref(),
                Some("aws_acip_sns_topic_missing")
            );
            assert_eq!(blocked.provider_message_id, None);
        }
    }

    #[test]
    fn csm_control_plane_notice_config_parses_targets_and_live_blockers() {
        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live"),
                ("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "lambda"),
                ("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED", "1"),
            ]);
            let config = ControlPlaneNoticeConfig::from_env();
            assert_eq!(config.mode, AwsSignalMode::Live);
            assert!(config.configured);
            assert_eq!(config.target, "lambda");
            assert_eq!(
                config.live_block_reason(),
                "control_plane_lambda_region_missing"
            );
        }

        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live"),
                ("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "lambda"),
                ("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED", "1"),
                ("ADL_AWS_REGION", "us-west-2"),
                ("ADL_AWS_PROFILE", "agent-logic-admin"),
            ]);
            let config = ControlPlaneNoticeConfig::from_env();
            assert_eq!(
                config.live_block_reason(),
                "control_plane_lambda_function_missing"
            );
        }

        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live"),
                ("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "eventbridge"),
                ("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED", "1"),
                ("ADL_AWS_REGION", "us-west-2"),
                ("ADL_AWS_PROFILE", "agent-logic-admin"),
                ("ADL_CSM_NOTICE_EVENT_BUS", "adl-csm-notice-bus-4998"),
            ]);
            let config = ControlPlaneNoticeConfig::from_env();
            assert_eq!(config.target, "eventbridge");
            assert_eq!(
                control_plane_profile_name(&config),
                Some("agent-logic-admin")
            );
            assert_eq!(
                config.live_block_reason(),
                "control_plane_eventbridge_put_failed"
            );
            assert!(config.target_hash().is_some());
        }
    }

    #[test]
    fn csm_control_plane_notice_mock_writes_redacted_eventbridge_envelope() {
        let root = temp_dir("csm-control-plane-mock");
        let loaded = sample_loaded(&root);
        let _guard = MultiEnvGuard::set_all(&[
            ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "mock"),
            ("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "eventbridge"),
            ("ADL_CSM_NOTICE_EVENT_BUS", "adl-csm-notice-bus-4998"),
        ]);

        let attempt = publish_csm_notice_control_plane(&loaded, &sample_csm_notice());
        assert_eq!(attempt["channel"], "cloudfront_control_plane");
        assert_eq!(attempt["status"], "published_mock");
        assert_eq!(attempt["target_kind"], "eventbridge");
        assert_eq!(attempt["target_sha256"].as_str().map(str::len), Some(64));

        let artifact = fs::read_to_string(csm_notice_control_plane_mock_artifact_path(&loaded))
            .expect("control plane mock artifact");
        let envelope: serde_json::Value =
            serde_json::from_str(artifact.lines().next().expect("jsonl line"))
                .expect("parse control-plane envelope");
        assert_eq!(envelope["schema_version"], AWS_SIGNAL_SCHEMA_VERSION);
        assert_eq!(envelope["signal_kind"], "csm_governed_notice");
        assert_eq!(
            envelope["transport"]["target_kind"],
            "cloudfront_control_plane"
        );
        assert_eq!(envelope["transport"]["mode"], "mock");
        assert_eq!(
            envelope["payload"]["safe_fail_ref"],
            "safe_fail_bundle.json"
        );
        assert_eq!(
            envelope["payload"]["notice_ledger_ref"],
            "csm_governed_notices.jsonl"
        );
        assert!(!artifact.contains("arn:aws:"));
    }

    #[test]
    fn csm_governed_notice_signal_mock_publishes_all_configured_channels() {
        let root = temp_dir("csm-notice-all-mock");
        let loaded = sample_loaded(&root);
        let _guard = MultiEnvGuard::set_all(&[
            ("ADL_AWS_SIGNAL_MODE", "mock"),
            ("ADL_AWS_HEARTBEAT_TARGET", "cloudwatch_logs"),
            (
                "ADL_AWS_SNS_TOPIC_ARN",
                "arn:aws:sns:us-west-2:000000000000:adl-csm-test",
            ),
            ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "mock"),
            ("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "https"),
            (
                "ADL_CSM_NOTICE_CONTROL_PLANE_URL",
                "https://control.example.invalid/csm",
            ),
        ]);

        let attempts = publish_csm_governed_notice_signal(&loaded, &sample_csm_notice());
        assert_eq!(attempts.len(), 3);
        assert_eq!(attempts[0]["channel"], "cloudwatch_logs");
        assert_eq!(attempts[0]["status"], "published_mock");
        assert_eq!(attempts[1]["channel"], "acip_sns");
        assert_eq!(attempts[1]["status"], "published_mock");
        assert_eq!(attempts[2]["channel"], "cloudfront_control_plane");
        assert_eq!(attempts[2]["status"], "published_mock");

        assert!(csm_notice_mock_signal_artifact_path(&loaded).exists());
        assert!(csm_notice_sns_mock_signal_artifact_path(&loaded).exists());
        assert!(csm_notice_control_plane_mock_artifact_path(&loaded).exists());
    }

    #[test]
    fn csm_governed_notice_signal_reports_unconfigured_and_disabled_channels() {
        let root = temp_dir("csm-notice-disabled");
        let loaded = sample_loaded(&root);

        {
            let _guard = MultiEnvGuard::set_all(&[]);
            let attempts = publish_csm_governed_notice_signal(&loaded, &sample_csm_notice());
            assert_eq!(attempts[0]["status"], "not_configured");
            assert_eq!(attempts[1]["status"], "not_configured");
            assert_eq!(attempts[2]["status"], "not_configured");
            assert_eq!(
                attempts[2]["failure_class"],
                "control_plane_hook_pending_4915_or_env_missing"
            );
        }

        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_AWS_SIGNAL_MODE", "disabled"),
                (
                    "ADL_AWS_SNS_TOPIC_ARN",
                    "arn:aws:sns:us-west-2:000000000000:adl-csm-test",
                ),
                ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "disabled"),
                (
                    "ADL_CSM_NOTICE_CONTROL_PLANE_URL",
                    "https://control.example.invalid/csm",
                ),
            ]);
            let attempts = publish_csm_governed_notice_signal(&loaded, &sample_csm_notice());
            assert_eq!(attempts[0]["status"], "skipped_disabled");
            assert_eq!(attempts[1]["status"], "skipped_disabled");
            assert_eq!(attempts[2]["status"], "skipped_disabled");
        }
    }

    #[test]
    fn csm_governed_notice_signal_blocks_live_channels_before_provider_calls() {
        let root = temp_dir("csm-notice-live-blocked");
        let loaded = sample_loaded(&root);

        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_AWS_SIGNAL_MODE", "live"),
                ("ADL_AWS_SIGNAL_APPROVED", "1"),
                ("ADL_AWS_HEARTBEAT_TARGET", "sns"),
            ]);
            let attempt = publish_csm_notice_cloudwatch(&loaded, &sample_csm_notice());
            assert_eq!(attempt["status"], "blocked");
            assert_eq!(attempt["failure_class"], "aws_signal_unsupported_target");
        }

        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_AWS_SIGNAL_MODE", "live"),
                ("ADL_AWS_SIGNAL_APPROVED", "1"),
                ("ADL_AWS_REGION", "us-west-2"),
                ("ADL_AWS_PROFILE", "agent-logic-admin"),
                ("ADL_AWS_HEARTBEAT_LOG_GROUP", "group"),
                ("ADL_AWS_HEARTBEAT_LOG_STREAM", "stream"),
                ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live"),
                ("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED", "1"),
                ("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "eventbridge"),
            ]);
            let attempts = publish_csm_governed_notice_signal(&loaded, &sample_csm_notice());
            assert_eq!(attempts[1]["status"], "blocked");
            assert_eq!(attempts[1]["failure_class"], "aws_acip_sns_topic_missing");
            assert_eq!(attempts[2]["status"], "blocked");
            assert_eq!(
                attempts[2]["failure_class"],
                "control_plane_event_bus_missing"
            );
        }

        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live"),
                ("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED", "1"),
                ("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "lambda"),
                ("ADL_AWS_REGION", "us-west-2"),
            ]);
            let attempt = publish_csm_notice_control_plane(&loaded, &sample_csm_notice());
            assert_eq!(attempt["status"], "blocked");
            assert_eq!(
                attempt["failure_class"],
                "control_plane_lambda_profile_missing"
            );
        }
    }

    #[test]
    fn csm_control_plane_notice_config_covers_https_and_unapproved_live() {
        {
            let _guard = MultiEnvGuard::set_all(&[(
                "ADL_CSM_NOTICE_CONTROL_PLANE_URL",
                "https://control.example.invalid/csm",
            )]);
            let config = ControlPlaneNoticeConfig::from_env();
            assert!(config.configured);
            assert_eq!(config.mode, AwsSignalMode::Disabled);
            assert_eq!(config.mode_label(), "disabled");
            assert_eq!(config.target, "https");
            assert_eq!(config.target_hash().as_ref().map(String::len), Some(64));
            assert_eq!(
                config.live_block_reason(),
                "control_plane_live_not_approved"
            );
        }

        {
            let _guard = MultiEnvGuard::set_all(&[
                ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live"),
                ("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED", "1"),
                ("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "https"),
            ]);
            let config = ControlPlaneNoticeConfig::from_env();
            assert_eq!(config.mode, AwsSignalMode::Live);
            assert_eq!(config.live_block_reason(), "control_plane_url_missing");
        }
    }

    #[test]
    fn csm_notice_cloudwatch_live_block_matrix_avoids_provider_calls() {
        let root = temp_dir("csm-cloudwatch-live-blocks");
        let loaded = sample_loaded(&root);
        let notice = sample_csm_notice();
        let cases = [
            (
                vec![("ADL_AWS_SIGNAL_MODE", "live")],
                "aws_signal_live_not_approved",
            ),
            (
                vec![
                    ("ADL_AWS_SIGNAL_MODE", "live"),
                    ("ADL_AWS_SIGNAL_APPROVED", "1"),
                ],
                "aws_signal_region_missing",
            ),
            (
                vec![
                    ("ADL_AWS_SIGNAL_MODE", "live"),
                    ("ADL_AWS_SIGNAL_APPROVED", "1"),
                    ("ADL_AWS_REGION", "us-west-2"),
                ],
                "aws_signal_log_group_missing",
            ),
            (
                vec![
                    ("ADL_AWS_SIGNAL_MODE", "live"),
                    ("ADL_AWS_SIGNAL_APPROVED", "1"),
                    ("ADL_AWS_REGION", "us-west-2"),
                    ("ADL_AWS_HEARTBEAT_LOG_GROUP", "group"),
                ],
                "aws_signal_log_stream_missing",
            ),
            (
                vec![
                    ("ADL_AWS_SIGNAL_MODE", "live"),
                    ("ADL_AWS_SIGNAL_APPROVED", "1"),
                    ("ADL_AWS_REGION", "us-west-2"),
                    ("ADL_AWS_HEARTBEAT_LOG_GROUP", "group"),
                    ("ADL_AWS_HEARTBEAT_LOG_STREAM", "stream"),
                ],
                "aws_signal_profile_missing",
            ),
        ];

        for (env_values, expected_failure) in cases {
            let _guard = MultiEnvGuard::set_all(&env_values);
            let attempt = publish_csm_notice_cloudwatch(&loaded, &notice);
            assert_eq!(attempt["status"], "blocked");
            assert_eq!(attempt["failure_class"], expected_failure);
        }
    }

    #[test]
    fn csm_notice_sns_live_block_matrix_avoids_provider_calls() {
        let root = temp_dir("csm-sns-live-blocks");
        let loaded = sample_loaded(&root);
        let notice = sample_csm_notice();
        let cases = [
            (
                vec![("ADL_AWS_SIGNAL_MODE", "live")],
                "aws_acip_sns_live_not_approved",
            ),
            (
                vec![
                    ("ADL_AWS_SIGNAL_MODE", "live"),
                    ("ADL_AWS_SIGNAL_APPROVED", "1"),
                ],
                "aws_acip_sns_region_missing",
            ),
            (
                vec![
                    ("ADL_AWS_SIGNAL_MODE", "live"),
                    ("ADL_AWS_SIGNAL_APPROVED", "1"),
                    ("ADL_AWS_REGION", "us-west-2"),
                ],
                "aws_acip_sns_profile_missing",
            ),
            (
                vec![
                    ("ADL_AWS_SIGNAL_MODE", "live"),
                    ("ADL_AWS_SIGNAL_APPROVED", "1"),
                    ("ADL_AWS_REGION", "us-west-2"),
                    ("ADL_AWS_PROFILE", "agent-logic-admin"),
                ],
                "aws_acip_sns_topic_missing",
            ),
        ];

        for (env_values, expected_failure) in cases {
            let _guard = MultiEnvGuard::set_all(&env_values);
            let attempt = publish_csm_notice_sns(&loaded, &notice);
            assert_eq!(attempt["status"], "blocked");
            assert_eq!(attempt["failure_class"], expected_failure);
        }
    }

    #[test]
    fn csm_notice_control_plane_live_block_matrix_avoids_provider_calls() {
        let root = temp_dir("csm-control-plane-live-blocks");
        let loaded = sample_loaded(&root);
        let notice = sample_csm_notice();
        let cases = [
            (
                vec![("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live")],
                "control_plane_live_not_approved",
            ),
            (
                vec![
                    ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live"),
                    ("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED", "1"),
                    ("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "eventbridge"),
                ],
                "control_plane_eventbridge_region_missing",
            ),
            (
                vec![
                    ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live"),
                    ("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED", "1"),
                    ("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "eventbridge"),
                    ("ADL_AWS_REGION", "us-west-2"),
                ],
                "control_plane_eventbridge_profile_missing",
            ),
            (
                vec![
                    ("ADL_CSM_NOTICE_CONTROL_PLANE_MODE", "live"),
                    ("ADL_CSM_NOTICE_CONTROL_PLANE_APPROVED", "1"),
                    ("ADL_CSM_NOTICE_CONTROL_PLANE_TARGET", "lambda"),
                    ("ADL_AWS_REGION", "us-west-2"),
                    ("ADL_AWS_PROFILE", "agent-logic-admin"),
                ],
                "control_plane_lambda_function_missing",
            ),
        ];

        for (env_values, expected_failure) in cases {
            let _guard = MultiEnvGuard::set_all(&env_values);
            let attempt = publish_csm_notice_control_plane(&loaded, &notice);
            assert_eq!(attempt["status"], "blocked");
            assert_eq!(attempt["failure_class"], expected_failure);
        }
    }

    #[test]
    fn live_provider_helpers_fail_closed_before_unconfigured_provider_calls() {
        let root = temp_dir("provider-helper-preflight");
        let loaded = sample_loaded(&root);
        let heartbeat_config = HeartbeatPublisherConfig {
            mode: AwsSignalMode::Live,
            configured: true,
            region: None,
            target_kind: HEARTBEAT_TARGET_KIND.to_string(),
            approved: true,
            profile: Some("agent-logic-admin".to_string()),
            log_group: None,
            log_stream: None,
            log_group_configured: false,
            log_stream_configured: false,
        };
        let heartbeat_envelope = build_runtime_heartbeat_envelope(
            &loaded,
            &sample_status(AgentStatusState::RunningCycle),
            &heartbeat_config,
            7,
        );
        let missing_group =
            publish_live_cloudwatch_heartbeat(&heartbeat_config, &heartbeat_envelope)
                .expect_err("missing log group fails before provider call");
        assert!(missing_group.to_string().contains("LOG_GROUP"));

        let missing_region = run_cloudwatch_put(
            &heartbeat_config,
            "group",
            "stream",
            heartbeat_envelope.timestamp.timestamp_millis(),
            "{}".to_string(),
        )
        .expect_err("missing region fails before provider call");
        assert!(missing_region.to_string().contains("ADL_AWS_REGION"));

        let acip_config = AcipProjectionPublisherConfig {
            mode: AwsSignalMode::Live,
            configured: true,
            region: None,
            approved: true,
            profile: Some("agent-logic-admin".to_string()),
            topic_arn: None,
            topic_configured: false,
        };
        let message = sample_acip_message();
        let request = AcipSnsProjectionRequest {
            runtime_id: "runtime-provider-preflight",
            agent_id: "runtime-provider-preflight",
            cycle_id: Some("cycle-provider-preflight"),
            message: &message,
            route_class: AcipRouteClassV1::CrossBoundaryDeferred,
            projection_level: "content_summary",
            message_ref: "acip/messages/msg-acip-0007.json",
            trace_ref: Some("runtime/comms/trace/public_summary.json"),
        };
        let acip_envelope = build_acip_sns_projection_envelope(&request, &acip_config);
        let missing_topic = publish_live_sns_acip_projection(&acip_config, &acip_envelope)
            .expect_err("missing SNS topic fails before provider call");
        assert!(missing_topic.to_string().contains("SNS_TOPIC_ARN"));

        let missing_sns_region = run_sns_publish_with_signal_kind(
            &acip_config,
            "arn:aws:sns:us-west-2:000000000000:adl-csm-test",
            "{}".to_string(),
            "correlation-provider-preflight".to_string(),
            "csm_governed_notice",
        )
        .expect_err("missing SNS region fails before provider call");
        assert!(missing_sns_region.to_string().contains("ADL_AWS_REGION"));

        let lambda_config = ControlPlaneNoticeConfig {
            mode: AwsSignalMode::Live,
            configured: true,
            approved: true,
            target: "lambda".to_string(),
            region: None,
            profile: Some("agent-logic-admin".to_string()),
            endpoint: None,
            lambda_function: None,
            event_bus: None,
        };
        let csm_envelope = csm_notice_envelope(
            &loaded,
            &sample_csm_notice(),
            "cloudfront_control_plane",
            "live",
        );
        let lambda_missing_region = invoke_csm_notice_lambda(&lambda_config, &csm_envelope)
            .expect_err("missing lambda region fails before provider call");
        assert!(lambda_missing_region.to_string().contains("lambda region"));

        let eventbridge_config = ControlPlaneNoticeConfig {
            mode: AwsSignalMode::Live,
            configured: true,
            approved: true,
            target: "eventbridge".to_string(),
            region: None,
            profile: Some("agent-logic-admin".to_string()),
            endpoint: None,
            lambda_function: None,
            event_bus: None,
        };
        let eventbridge_missing_region =
            put_csm_notice_eventbridge(&eventbridge_config, &csm_envelope)
                .expect_err("missing EventBridge region fails before provider call");
        assert!(eventbridge_missing_region
            .to_string()
            .contains("EventBridge region"));
    }
}
