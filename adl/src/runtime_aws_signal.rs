use crate::agent_comms::{
    AcipAddressKindV1, AcipIntentV1, AcipMessageEnvelopeV1, AcipRouteClassV1,
};
use crate::long_lived_agent::{AgentStatusState, LoadedAgentSpec, StatusRecord};
use crate::observability::emit_event;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

const AWS_SIGNAL_SCHEMA_VERSION: &str = "adl.runtime.aws_signal.v1";
const HEARTBEAT_TARGET_KIND: &str = "cloudwatch_logs";
const MOCK_SIGNAL_ARTIFACT: &str = "aws_runtime_heartbeat_mock.jsonl";
#[allow(dead_code)]
const ACIP_SNS_TARGET_KIND: &str = "sns";
#[allow(dead_code)]
const ACIP_SNS_MOCK_SIGNAL_ARTIFACT: &str = "aws_acip_sns_projection_mock.jsonl";
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

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct AcipProjectionPublisherConfig {
    mode: AwsSignalMode,
    configured: bool,
    region: Option<String>,
    approved: bool,
    topic_configured: bool,
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

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct AcipSnsProjectionRequest<'a> {
    pub(crate) runtime_id: &'a str,
    pub(crate) agent_id: &'a str,
    pub(crate) cycle_id: Option<&'a str>,
    pub(crate) message: &'a AcipMessageEnvelopeV1,
    pub(crate) route_class: AcipRouteClassV1,
    pub(crate) projection_level: &'a str,
    pub(crate) message_ref: &'a str,
    pub(crate) trace_ref: Option<&'a str>,
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
        let topic_configured = env::var("ADL_AWS_SNS_TOPIC_ARN")
            .ok()
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false);
        Self {
            mode,
            configured: mode_env.is_some(),
            region,
            approved,
            topic_configured,
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
            "approval_missing"
        } else if self.region.is_none() || !self.topic_configured {
            "config_missing"
        } else {
            "publish_failed"
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
            }
        }
        AwsSignalMode::Live => {
            let failure_class = config.live_block_reason();
            emit_publish_failure(
                &config,
                loaded.spec.agent_instance_id.as_str(),
                cycle_id_for_status(status).as_str(),
                "not_allocated",
                runtime_signal_status(status),
                failure_class,
            );
            PublishOutcome {
                disposition: PublishDisposition::Blocked,
                failure_class: Some(failure_class.to_string()),
            }
        }
    }
}

#[allow(dead_code)]
pub(crate) fn publish_acip_sns_projection_signal(
    output_root: &Path,
    request: &AcipSnsProjectionRequest<'_>,
) -> PublishOutcome {
    let config = AcipProjectionPublisherConfig::from_env();
    if !config.configured {
        return PublishOutcome {
            disposition: PublishDisposition::Skipped,
            failure_class: None,
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
                }
            }
        },
        AwsSignalMode::Live => {
            let failure_class = config.live_block_reason();
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
                AwsSignalMode::Live => "publish_blocked".to_string(),
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
                "ADL_AWS_SNS_TOPIC_ARN",
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
            assert_eq!(
                config.live_block_reason(),
                "aws_signal_live_transport_not_implemented"
            );
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
            assert_eq!(blocked.failure_class.as_deref(), Some("config_missing"));
        }
    }
}
