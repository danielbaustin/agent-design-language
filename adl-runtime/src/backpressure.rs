//! CSM runtime backpressure contract.

use redb::{Database, Durability, ReadableDatabase, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

const SPOOL_MESSAGES: TableDefinition<u64, &[u8]> = TableDefinition::new("messages");
const SPOOL_META: TableDefinition<&str, u64> = TableDefinition::new("spool_meta");
const PUBLISH_CURSOR: TableDefinition<&str, u64> = TableDefinition::new("publish_cursor");
const PUBLISH_RECEIPTS: TableDefinition<u64, &[u8]> = TableDefinition::new("publish_receipts");

pub const CSM_BACKPRESSURE_REPORT_SCHEMA: &str = "adl.csm.backpressure_report.v1";
pub const CSM_BACKPRESSURE_STATE_SCHEMA: &str = "adl.csm.backpressure_state.v1";
pub const CSM_BACKPRESSURE_COMMAND_RESULT_SCHEMA: &str = "adl.csm.backpressure_command_result.v1";

pub const REQUIRED_STATE_LOSS_POLICY: &str = "never_silent_drop";
pub const NONCRITICAL_LOSS_POLICY: &str = "explicit_defer_or_shed";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeChannelId {
    SchedulerToReasoningRuntime,
    ReasoningRuntimeToAee,
    AeeToCheckpoint,
    ComponentsToLifelog,
    ComponentsToObservability,
    CloudBridgeToAwsRoutes,
    RuntimeApiToControlPlane,
}

impl RuntimeChannelId {
    pub const ALL: [Self; 7] = [
        Self::SchedulerToReasoningRuntime,
        Self::ReasoningRuntimeToAee,
        Self::AeeToCheckpoint,
        Self::ComponentsToLifelog,
        Self::ComponentsToObservability,
        Self::CloudBridgeToAwsRoutes,
        Self::RuntimeApiToControlPlane,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SchedulerToReasoningRuntime => "scheduler_to_reasoning_runtime",
            Self::ReasoningRuntimeToAee => "reasoning_runtime_to_aee",
            Self::AeeToCheckpoint => "aee_to_checkpoint",
            Self::ComponentsToLifelog => "components_to_lifelog",
            Self::ComponentsToObservability => "components_to_observability",
            Self::CloudBridgeToAwsRoutes => "cloud_bridge_to_aws_routes",
            Self::RuntimeApiToControlPlane => "runtime_api_to_control_plane",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelPriority {
    CriticalContinuity,
    GovernedExecution,
    Evidence,
    Audit,
    LowPriorityObservability,
    ControlPlane,
}

impl ChannelPriority {
    pub const fn is_required(self) -> bool {
        !matches!(self, Self::LowPriorityObservability)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FullQueuePolicy {
    BlockProducer,
    ThrottleProducer,
    DurableSpool,
    ShedLowPriorityOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionOutcome {
    Accepted,
    Blocked,
    Throttled,
    Spooled,
    Shed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessState {
    Ready,
    Degraded,
    Overloaded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CursorAdvancePolicy {
    OnAccept,
    AfterDurableSpool,
    AfterPublishableAck,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct RuntimeChannelPolicy {
    pub id: RuntimeChannelId,
    pub source: &'static str,
    pub target: &'static str,
    pub capacity: usize,
    pub priority: ChannelPriority,
    pub full_queue_policy: FullQueuePolicy,
    pub loss_policy: &'static str,
    pub cursor_advance_policy: CursorAdvancePolicy,
    pub health_signal: &'static str,
    pub readiness_projection: &'static str,
}

impl RuntimeChannelPolicy {
    pub fn preserves_required_state(self) -> bool {
        !self.priority.is_required()
            || matches!(self.loss_policy, REQUIRED_STATE_LOSS_POLICY)
            || matches!(self.full_queue_policy, FullQueuePolicy::ShedLowPriorityOnly)
    }

    pub fn allows_low_priority_shed(self) -> bool {
        matches!(self.full_queue_policy, FullQueuePolicy::ShedLowPriorityOnly)
            && matches!(self.loss_policy, NONCRITICAL_LOSS_POLICY)
    }

    pub fn decide(
        self,
        snapshot: ChannelQueueSnapshot,
        priority: ChannelPriority,
    ) -> AdmissionDecision {
        if snapshot.depth < self.capacity {
            return AdmissionDecision {
                channel: self.id,
                outcome: AdmissionOutcome::Accepted,
                readiness: ReadinessState::Ready,
                drop_accounted: false,
                preserves_required_state: true,
                cursor_may_advance: matches!(
                    self.cursor_advance_policy,
                    CursorAdvancePolicy::OnAccept
                ),
                health_signal: self.health_signal,
                reason: "bounded_capacity_available",
            };
        }

        match self.full_queue_policy {
            FullQueuePolicy::BlockProducer => AdmissionDecision {
                channel: self.id,
                outcome: AdmissionOutcome::Blocked,
                readiness: ReadinessState::Overloaded,
                drop_accounted: false,
                preserves_required_state: true,
                cursor_may_advance: false,
                health_signal: self.health_signal,
                reason: "full_queue_blocks_required_state",
            },
            FullQueuePolicy::ThrottleProducer => AdmissionDecision {
                channel: self.id,
                outcome: AdmissionOutcome::Throttled,
                readiness: ReadinessState::Degraded,
                drop_accounted: false,
                preserves_required_state: true,
                cursor_may_advance: false,
                health_signal: self.health_signal,
                reason: "full_queue_throttles_control_plane",
            },
            FullQueuePolicy::DurableSpool => AdmissionDecision {
                channel: self.id,
                outcome: AdmissionOutcome::Spooled,
                readiness: ReadinessState::Degraded,
                drop_accounted: false,
                preserves_required_state: true,
                cursor_may_advance: matches!(
                    self.cursor_advance_policy,
                    CursorAdvancePolicy::AfterDurableSpool
                ),
                health_signal: self.health_signal,
                reason: "full_queue_uses_durable_spool",
            },
            FullQueuePolicy::ShedLowPriorityOnly if priority.is_required() => AdmissionDecision {
                channel: self.id,
                outcome: AdmissionOutcome::Spooled,
                readiness: ReadinessState::Degraded,
                drop_accounted: false,
                preserves_required_state: true,
                cursor_may_advance: false,
                health_signal: self.health_signal,
                reason: "full_queue_spools_required_observability",
            },
            FullQueuePolicy::ShedLowPriorityOnly => AdmissionDecision {
                channel: self.id,
                outcome: AdmissionOutcome::Shed,
                readiness: ReadinessState::Degraded,
                drop_accounted: true,
                preserves_required_state: true,
                cursor_may_advance: false,
                health_signal: self.health_signal,
                reason: "full_queue_sheds_accounted_low_priority_observability",
            },
        }
    }

    pub fn to_json(self) -> Value {
        json!({
            "channel_id": self.id.as_str(),
            "source": self.source,
            "target": self.target,
            "capacity": self.capacity,
            "priority": self.priority,
            "full_queue_policy": self.full_queue_policy,
            "loss_policy": self.loss_policy,
            "cursor_advance_policy": self.cursor_advance_policy,
            "health_signal": self.health_signal,
            "readiness_projection": self.readiness_projection,
            "preserves_required_state": self.preserves_required_state(),
            "allows_low_priority_shed": self.allows_low_priority_shed()
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChannelQueueSnapshot {
    pub depth: usize,
    pub dropped_count: u64,
    pub deferred_count: u64,
    pub throttled_count: u64,
    pub durable_spool_depth: usize,
}

impl ChannelQueueSnapshot {
    pub const fn empty() -> Self {
        Self {
            depth: 0,
            dropped_count: 0,
            deferred_count: 0,
            throttled_count: 0,
            durable_spool_depth: 0,
        }
    }

    pub const fn full(capacity: usize) -> Self {
        Self {
            depth: capacity,
            dropped_count: 0,
            deferred_count: 0,
            throttled_count: 0,
            durable_spool_depth: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct AdmissionDecision {
    pub channel: RuntimeChannelId,
    pub outcome: AdmissionOutcome,
    pub readiness: ReadinessState,
    pub drop_accounted: bool,
    pub preserves_required_state: bool,
    pub cursor_may_advance: bool,
    pub health_signal: &'static str,
    pub reason: &'static str,
}

pub fn typed_channel_policy_matrix() -> Vec<RuntimeChannelPolicy> {
    vec![
        RuntimeChannelPolicy {
            id: RuntimeChannelId::SchedulerToReasoningRuntime,
            source: "scheduler",
            target: "reasoning_runtime",
            capacity: 64,
            priority: ChannelPriority::GovernedExecution,
            full_queue_policy: FullQueuePolicy::BlockProducer,
            loss_policy: REQUIRED_STATE_LOSS_POLICY,
            cursor_advance_policy: CursorAdvancePolicy::OnAccept,
            health_signal: "scheduler_reasoning_queue_overloaded",
            readiness_projection: "/ready.scheduler_reasoning_admission",
        },
        RuntimeChannelPolicy {
            id: RuntimeChannelId::ReasoningRuntimeToAee,
            source: "reasoning_runtime",
            target: "aee",
            capacity: 128,
            priority: ChannelPriority::GovernedExecution,
            full_queue_policy: FullQueuePolicy::BlockProducer,
            loss_policy: REQUIRED_STATE_LOSS_POLICY,
            cursor_advance_policy: CursorAdvancePolicy::OnAccept,
            health_signal: "reasoning_aee_queue_overloaded",
            readiness_projection: "/ready.governed_execution_admission",
        },
        RuntimeChannelPolicy {
            id: RuntimeChannelId::AeeToCheckpoint,
            source: "aee",
            target: "checkpoint",
            capacity: 32,
            priority: ChannelPriority::CriticalContinuity,
            full_queue_policy: FullQueuePolicy::DurableSpool,
            loss_policy: REQUIRED_STATE_LOSS_POLICY,
            cursor_advance_policy: CursorAdvancePolicy::AfterDurableSpool,
            health_signal: "checkpoint_spool_lag",
            readiness_projection: "/ready.continuity_checkpoint_lag",
        },
        RuntimeChannelPolicy {
            id: RuntimeChannelId::ComponentsToLifelog,
            source: "components",
            target: "lifelog",
            capacity: 256,
            priority: ChannelPriority::Evidence,
            full_queue_policy: FullQueuePolicy::DurableSpool,
            loss_policy: REQUIRED_STATE_LOSS_POLICY,
            cursor_advance_policy: CursorAdvancePolicy::AfterDurableSpool,
            health_signal: "lifelog_spool_lag",
            readiness_projection: "/ready.lifecycle_evidence_lag",
        },
        RuntimeChannelPolicy {
            id: RuntimeChannelId::ComponentsToObservability,
            source: "components",
            target: "observability",
            capacity: 1024,
            priority: ChannelPriority::Audit,
            full_queue_policy: FullQueuePolicy::ShedLowPriorityOnly,
            loss_policy: NONCRITICAL_LOSS_POLICY,
            cursor_advance_policy: CursorAdvancePolicy::AfterDurableSpool,
            health_signal: "observability_backpressure",
            readiness_projection: "/ready.observability_degraded",
        },
        RuntimeChannelPolicy {
            id: RuntimeChannelId::CloudBridgeToAwsRoutes,
            source: "cloud_bridge",
            target: "aws_routes",
            capacity: 64,
            priority: ChannelPriority::Evidence,
            full_queue_policy: FullQueuePolicy::DurableSpool,
            loss_policy: REQUIRED_STATE_LOSS_POLICY,
            cursor_advance_policy: CursorAdvancePolicy::AfterPublishableAck,
            health_signal: "cloud_publish_spool_lag",
            readiness_projection: "/ready.cloud_publishability",
        },
        RuntimeChannelPolicy {
            id: RuntimeChannelId::RuntimeApiToControlPlane,
            source: "runtime_api",
            target: "control_plane",
            capacity: 32,
            priority: ChannelPriority::ControlPlane,
            full_queue_policy: FullQueuePolicy::ThrottleProducer,
            loss_policy: REQUIRED_STATE_LOSS_POLICY,
            cursor_advance_policy: CursorAdvancePolicy::OnAccept,
            health_signal: "runtime_api_control_plane_overload",
            readiness_projection: "/ready.control_plane_overload",
        },
    ]
}

pub fn runtime_channel_policy(id: RuntimeChannelId) -> RuntimeChannelPolicy {
    typed_channel_policy_matrix()
        .into_iter()
        .find(|policy| policy.id == id)
        .expect("runtime channel policy matrix covers every typed channel")
}

pub fn typed_channel_policy_matrix_json() -> Value {
    json!({
        "schema": "adl.csm.typed_channel_backpressure_policy_matrix.v1",
        "runtime_owner": "csm",
        "channels": typed_channel_policy_matrix()
            .into_iter()
            .map(RuntimeChannelPolicy::to_json)
            .collect::<Vec<_>>()
    })
}

pub fn typed_channel_full_queue_readiness_projection_json() -> Value {
    let mut decisions = typed_channel_policy_matrix()
        .into_iter()
        .map(|policy| policy.decide(ChannelQueueSnapshot::full(policy.capacity), policy.priority))
        .collect::<Vec<_>>();
    let observability = runtime_channel_policy(RuntimeChannelId::ComponentsToObservability);
    decisions.push(observability.decide(
        ChannelQueueSnapshot::full(observability.capacity),
        ChannelPriority::LowPriorityObservability,
    ));
    readiness_projection(&decisions)
}

pub fn readiness_projection(decisions: &[AdmissionDecision]) -> Value {
    let overloaded = decisions
        .iter()
        .filter(|decision| decision.readiness == ReadinessState::Overloaded)
        .count();
    let degraded = decisions
        .iter()
        .filter(|decision| decision.readiness == ReadinessState::Degraded)
        .count();
    let accounted_drops: u64 = decisions
        .iter()
        .filter(|decision| decision.drop_accounted)
        .count() as u64;
    let required_state_silently_dropped = decisions
        .iter()
        .any(|decision| !decision.preserves_required_state);
    let state = if overloaded > 0 {
        "overloaded"
    } else if degraded > 0 {
        "degraded"
    } else {
        "ready"
    };

    json!({
        "schema": "adl.csm.typed_channel_backpressure_readiness.v1",
        "state": state,
        "overloaded_channel_count": overloaded,
        "degraded_channel_count": degraded,
        "accounted_drop_count": accounted_drops,
        "required_state_silently_dropped": required_state_silently_dropped,
        "decisions": decisions
    })
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeMessage {
    pub id: String,
    pub priority: ChannelPriority,
    pub payload: Value,
}

impl RuntimeMessage {
    pub fn new(id: impl Into<String>, priority: ChannelPriority, payload: Value) -> Self {
        Self {
            id: id.into(),
            priority,
            payload,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RuntimeEnvelope {
    message: RuntimeMessage,
    spool_sequence: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RuntimeDelivery {
    pub message: RuntimeMessage,
    pub spool_sequence: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TransportPublishReceipt {
    pub spool_sequence: u64,
    pub cursor: u64,
    pub transport: String,
    pub receipt_id: String,
}

impl TransportPublishReceipt {
    pub fn verified(
        spool_sequence: u64,
        cursor: u64,
        transport: impl Into<String>,
        receipt_id: impl Into<String>,
    ) -> Result<Self, RuntimeChannelError> {
        let transport = transport.into();
        let receipt_id = receipt_id.into();
        if transport.trim().is_empty() || receipt_id.trim().is_empty() {
            return Err(RuntimeChannelError::InvalidPublishReceipt(
                "transport and receipt_id must be non-empty".to_string(),
            ));
        }
        Ok(Self {
            spool_sequence,
            cursor,
            transport,
            receipt_id,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeSendOutcome {
    Accepted,
    BlockedThenAccepted,
    Throttled,
    DurablySpooled,
    Shed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuntimeSendReceipt {
    pub channel: RuntimeChannelId,
    pub message_id: String,
    pub outcome: RuntimeSendOutcome,
    pub spool_sequence: Option<u64>,
    pub cursor_may_advance: bool,
    pub readiness: ReadinessState,
    pub reason: &'static str,
}

#[derive(Debug)]
pub enum RuntimeChannelError {
    Closed(RuntimeChannelId),
    Spool(String),
    Serialization(String),
    Join(String),
    UnknownSpoolSequence(u64),
    InvalidPublishReceipt(String),
    NonMonotonicPublishCursor { current: u64, proposed: u64 },
}

impl fmt::Display for RuntimeChannelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Closed(channel) => write!(formatter, "runtime channel {channel:?} is closed"),
            Self::Spool(reason) => write!(formatter, "durable spool failure: {reason}"),
            Self::Serialization(reason) => {
                write!(formatter, "message serialization failure: {reason}")
            }
            Self::Join(reason) => write!(formatter, "durable spool worker failure: {reason}"),
            Self::UnknownSpoolSequence(sequence) => {
                write!(
                    formatter,
                    "durable spool sequence {sequence} does not exist"
                )
            }
            Self::InvalidPublishReceipt(reason) => {
                write!(formatter, "invalid transport publish receipt: {reason}")
            }
            Self::NonMonotonicPublishCursor { current, proposed } => write!(
                formatter,
                "publish cursor must advance contiguously: current={current} proposed={proposed}"
            ),
        }
    }
}

impl std::error::Error for RuntimeChannelError {}

#[derive(Debug, Default)]
struct RuntimeChannelMetrics {
    accepted: AtomicU64,
    blocked: AtomicU64,
    throttled: AtomicU64,
    spooled: AtomicU64,
    shed: AtomicU64,
    cancelled: AtomicU64,
    publish_acked: AtomicU64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct RuntimeChannelSnapshot {
    pub channel: RuntimeChannelId,
    pub capacity: usize,
    pub depth: usize,
    pub accepted_count: u64,
    pub blocked_count: u64,
    pub throttled_count: u64,
    pub durable_spool_depth: usize,
    pub spooled_count: u64,
    pub shed_count: u64,
    pub cancelled_count: u64,
    pub publish_acked_count: u64,
    pub publish_cursor: u64,
    pub readiness: ReadinessState,
}

#[derive(Debug)]
struct DurableSpool {
    database: Database,
}

impl DurableSpool {
    fn open(path: &Path) -> Result<Self, RuntimeChannelError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
        }
        let database = Database::create(path)
            .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
        Ok(Self { database })
    }

    fn append(&self, message: &RuntimeMessage) -> Result<u64, RuntimeChannelError> {
        let encoded = serde_json::to_vec(message)
            .map_err(|error| RuntimeChannelError::Serialization(error.to_string()))?;
        let mut write = self
            .database
            .begin_write()
            .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
        write
            .set_durability(Durability::Immediate)
            .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
        let last_message_sequence = {
            let table = write
                .open_table(SPOOL_MESSAGES)
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
            let sequence = table
                .last()
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?
                .map_or(0, |(key, _)| key.value());
            sequence
        };
        let publish_cursor = {
            let table = write
                .open_table(PUBLISH_CURSOR)
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
            let cursor = table
                .get("cloud_bridge")
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?
                .map_or(0, |value| value.value());
            cursor
        };
        let sequence = {
            let mut meta = write
                .open_table(SPOOL_META)
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
            let persisted_next = meta
                .get("next_sequence")
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?
                .map(|value| value.value());
            let next = persisted_next.unwrap_or_else(|| {
                last_message_sequence
                    .max(publish_cursor)
                    .saturating_add(1)
                    .max(1)
            });
            meta.insert("next_sequence", next.saturating_add(1))
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
            next
        };
        {
            let mut table = write
                .open_table(SPOOL_MESSAGES)
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
            table
                .insert(sequence, encoded.as_slice())
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
        }
        write
            .commit()
            .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
        Ok(sequence)
    }

    fn remove(&self, sequence: u64) -> Result<bool, RuntimeChannelError> {
        let mut write = self
            .database
            .begin_write()
            .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
        write
            .set_durability(Durability::Immediate)
            .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
        let removed = {
            let mut table = write
                .open_table(SPOOL_MESSAGES)
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
            let removed = table
                .remove(sequence)
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?
                .is_some();
            removed
        };
        write
            .commit()
            .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
        Ok(removed)
    }

    fn acknowledge_published(
        &self,
        receipt: &TransportPublishReceipt,
    ) -> Result<(), RuntimeChannelError> {
        let encoded = serde_json::to_vec(receipt)
            .map_err(|error| RuntimeChannelError::Serialization(error.to_string()))?;
        let mut write = self
            .database
            .begin_write()
            .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
        write
            .set_durability(Durability::Immediate)
            .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;

        let message_exists = {
            let messages = write
                .open_table(SPOOL_MESSAGES)
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
            let exists = messages
                .get(receipt.spool_sequence)
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?
                .is_some();
            exists
        };
        if !message_exists {
            return Err(RuntimeChannelError::UnknownSpoolSequence(
                receipt.spool_sequence,
            ));
        }
        let current = {
            let cursors = write
                .open_table(PUBLISH_CURSOR)
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
            let current = cursors
                .get("cloud_bridge")
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?
                .map_or(0, |value| value.value());
            current
        };
        if receipt.cursor != current.saturating_add(1) {
            return Err(RuntimeChannelError::NonMonotonicPublishCursor {
                current,
                proposed: receipt.cursor,
            });
        }
        {
            let mut messages = write
                .open_table(SPOOL_MESSAGES)
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
            messages
                .remove(receipt.spool_sequence)
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
        }
        {
            let mut cursors = write
                .open_table(PUBLISH_CURSOR)
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
            cursors
                .insert("cloud_bridge", receipt.cursor)
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
        }
        {
            let mut receipts = write
                .open_table(PUBLISH_RECEIPTS)
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
            receipts
                .insert(receipt.cursor, encoded.as_slice())
                .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
        }
        write
            .commit()
            .map_err(|error| RuntimeChannelError::Spool(error.to_string()))
    }

    fn publish_cursor(&self) -> Result<u64, RuntimeChannelError> {
        let read = self
            .database
            .begin_read()
            .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
        let table = match read.open_table(PUBLISH_CURSOR) {
            Ok(table) => table,
            Err(redb::TableError::TableDoesNotExist(_)) => return Ok(0),
            Err(error) => return Err(RuntimeChannelError::Spool(error.to_string())),
        };
        Ok(table
            .get("cloud_bridge")
            .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?
            .map_or(0, |value| value.value()))
    }

    fn entries(&self) -> Result<Vec<(u64, RuntimeMessage)>, RuntimeChannelError> {
        let read = self
            .database
            .begin_read()
            .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
        let table = match read.open_table(SPOOL_MESSAGES) {
            Ok(table) => table,
            Err(redb::TableError::TableDoesNotExist(_)) => return Ok(Vec::new()),
            Err(error) => return Err(RuntimeChannelError::Spool(error.to_string())),
        };
        table
            .iter()
            .map_err(|error| RuntimeChannelError::Spool(error.to_string()))?
            .map(|entry| {
                let (sequence, payload) =
                    entry.map_err(|error| RuntimeChannelError::Spool(error.to_string()))?;
                let message = serde_json::from_slice(payload.value())
                    .map_err(|error| RuntimeChannelError::Serialization(error.to_string()))?;
                Ok((sequence.value(), message))
            })
            .collect()
    }

    fn len(&self) -> Result<usize, RuntimeChannelError> {
        Ok(self.entries()?.len())
    }
}

#[derive(Clone)]
pub struct RuntimeChannelSender {
    policy: RuntimeChannelPolicy,
    sender: mpsc::Sender<RuntimeEnvelope>,
    metrics: Arc<RuntimeChannelMetrics>,
    spool: Arc<DurableSpool>,
    replay_in_flight: Arc<Mutex<BTreeSet<u64>>>,
}

pub struct RuntimeChannelReceiver {
    policy: RuntimeChannelPolicy,
    receiver: mpsc::Receiver<RuntimeEnvelope>,
}

struct RuntimeChannelNode {
    sender: RuntimeChannelSender,
    receiver: RuntimeChannelReceiver,
}

pub struct RuntimeChannelFabric {
    channels: BTreeMap<RuntimeChannelId, RuntimeChannelNode>,
}

impl RuntimeChannelFabric {
    pub fn open(spool_root: impl AsRef<Path>) -> Result<Self, RuntimeChannelError> {
        let spool_root = spool_root.as_ref();
        let mut channels = BTreeMap::new();
        for id in RuntimeChannelId::ALL {
            let (sender, receiver) = runtime_channel(
                runtime_channel_policy(id),
                spool_root.join(format!("{}.redb", id.as_str())),
            )?;
            channels.insert(id, RuntimeChannelNode { sender, receiver });
        }
        Ok(Self { channels })
    }

    pub async fn transit(
        &mut self,
        id: RuntimeChannelId,
        message: RuntimeMessage,
        cancellation: &CancellationToken,
    ) -> Result<(RuntimeSendReceipt, Option<RuntimeDelivery>), RuntimeChannelError> {
        let node = self
            .channels
            .get_mut(&id)
            .expect("runtime channel fabric covers every RuntimeChannelId");
        let receipt = node.sender.send(message, cancellation).await?;
        let delivery = if matches!(
            receipt.outcome,
            RuntimeSendOutcome::Accepted | RuntimeSendOutcome::BlockedThenAccepted
        ) {
            node.receiver.recv().await
        } else {
            None
        };
        Ok((receipt, delivery))
    }

    pub async fn snapshots(&self) -> Result<Vec<RuntimeChannelSnapshot>, RuntimeChannelError> {
        let mut snapshots = Vec::with_capacity(self.channels.len());
        for id in RuntimeChannelId::ALL {
            snapshots.push(
                self.channels
                    .get(&id)
                    .expect("runtime channel fabric covers every RuntimeChannelId")
                    .sender
                    .snapshot()
                    .await?,
            );
        }
        Ok(snapshots)
    }

    pub async fn persist_required(
        &self,
        id: RuntimeChannelId,
        message: RuntimeMessage,
    ) -> Result<RuntimeSendReceipt, RuntimeChannelError> {
        let node = self
            .channels
            .get(&id)
            .expect("runtime channel fabric covers every RuntimeChannelId");
        let message_id = message.id.clone();
        node.sender
            .spool_required_with_reason(
                message,
                message_id,
                "required_message_persisted_before_external_delivery",
            )
            .await
    }

    pub async fn replay_next(
        &mut self,
        id: RuntimeChannelId,
        cancellation: &CancellationToken,
    ) -> Result<Option<RuntimeDelivery>, RuntimeChannelError> {
        let node = self
            .channels
            .get_mut(&id)
            .expect("runtime channel fabric covers every RuntimeChannelId");
        if !node.sender.replay_next_spooled(cancellation).await? {
            return Ok(None);
        }
        Ok(node.receiver.recv().await)
    }

    pub async fn acknowledge_processed(
        &self,
        id: RuntimeChannelId,
        sequence: u64,
    ) -> Result<(), RuntimeChannelError> {
        self.channels
            .get(&id)
            .expect("runtime channel fabric covers every RuntimeChannelId")
            .sender
            .acknowledge_processed(sequence)
            .await
    }

    pub fn release_replay(
        &self,
        id: RuntimeChannelId,
        sequence: u64,
    ) -> Result<(), RuntimeChannelError> {
        self.channels
            .get(&id)
            .expect("runtime channel fabric covers every RuntimeChannelId")
            .sender
            .clear_replay_in_flight(sequence)
    }

    pub async fn acknowledge_published(
        &self,
        receipt: TransportPublishReceipt,
    ) -> Result<(), RuntimeChannelError> {
        self.channels
            .get(&RuntimeChannelId::CloudBridgeToAwsRoutes)
            .expect("runtime channel fabric covers cloud bridge")
            .sender
            .acknowledge_published(receipt)
            .await
    }
}

pub fn runtime_channel(
    policy: RuntimeChannelPolicy,
    spool_path: impl AsRef<Path>,
) -> Result<(RuntimeChannelSender, RuntimeChannelReceiver), RuntimeChannelError> {
    let (sender, receiver) = mpsc::channel(policy.capacity);
    let metrics = Arc::new(RuntimeChannelMetrics::default());
    let spool = Arc::new(DurableSpool::open(spool_path.as_ref())?);
    let replay_in_flight = Arc::new(Mutex::new(BTreeSet::new()));
    Ok((
        RuntimeChannelSender {
            policy,
            sender,
            metrics: Arc::clone(&metrics),
            spool,
            replay_in_flight,
        },
        RuntimeChannelReceiver { policy, receiver },
    ))
}

impl RuntimeChannelSender {
    pub async fn send(
        &self,
        message: RuntimeMessage,
        cancellation: &CancellationToken,
    ) -> Result<RuntimeSendReceipt, RuntimeChannelError> {
        let message_id = message.id.clone();
        match self.sender.try_send(RuntimeEnvelope {
            message,
            spool_sequence: None,
        }) {
            Ok(()) => {
                self.metrics.accepted.fetch_add(1, Ordering::SeqCst);
                Ok(self.receipt(
                    message_id,
                    RuntimeSendOutcome::Accepted,
                    None,
                    matches!(
                        self.policy.cursor_advance_policy,
                        CursorAdvancePolicy::OnAccept
                    ),
                    ReadinessState::Ready,
                    "bounded_capacity_available",
                ))
            }
            Err(mpsc::error::TrySendError::Closed(_)) => {
                Err(RuntimeChannelError::Closed(self.policy.id))
            }
            Err(mpsc::error::TrySendError::Full(envelope)) => {
                self.handle_full(envelope.message, message_id, cancellation)
                    .await
            }
        }
    }

    async fn handle_full(
        &self,
        message: RuntimeMessage,
        message_id: String,
        cancellation: &CancellationToken,
    ) -> Result<RuntimeSendReceipt, RuntimeChannelError> {
        match self.policy.full_queue_policy {
            FullQueuePolicy::BlockProducer => {
                self.metrics.blocked.fetch_add(1, Ordering::SeqCst);
                tokio::select! {
                    biased;
                    () = cancellation.cancelled() => {
                        self.metrics.cancelled.fetch_add(1, Ordering::SeqCst);
                        self.spool_required_with_reason(
                            message,
                            message_id,
                            "blocked_send_cancelled_required_message_spooled",
                        ).await
                    }
                    permit = self.sender.reserve() => {
                        let permit = permit.map_err(|_| RuntimeChannelError::Closed(self.policy.id))?;
                        permit.send(RuntimeEnvelope { message, spool_sequence: None });
                        self.metrics.accepted.fetch_add(1, Ordering::SeqCst);
                        Ok(self.receipt(message_id, RuntimeSendOutcome::BlockedThenAccepted, None, matches!(self.policy.cursor_advance_policy, CursorAdvancePolicy::OnAccept), ReadinessState::Ready, "capacity_recovered_after_block"))
                    }
                }
            }
            FullQueuePolicy::ThrottleProducer => {
                self.metrics.throttled.fetch_add(1, Ordering::SeqCst);
                Ok(self.receipt(
                    message_id,
                    RuntimeSendOutcome::Throttled,
                    None,
                    false,
                    ReadinessState::Degraded,
                    "full_queue_throttles_control_plane",
                ))
            }
            FullQueuePolicy::DurableSpool => self.spool_required(message, message_id).await,
            FullQueuePolicy::ShedLowPriorityOnly if message.priority.is_required() => {
                self.spool_required(message, message_id).await
            }
            FullQueuePolicy::ShedLowPriorityOnly => {
                self.metrics.shed.fetch_add(1, Ordering::SeqCst);
                Ok(self.receipt(
                    message_id,
                    RuntimeSendOutcome::Shed,
                    None,
                    false,
                    ReadinessState::Degraded,
                    "accounted_low_priority_shed",
                ))
            }
        }
    }

    async fn spool_required(
        &self,
        message: RuntimeMessage,
        message_id: String,
    ) -> Result<RuntimeSendReceipt, RuntimeChannelError> {
        self.spool_required_with_reason(message, message_id, "required_message_durably_spooled")
            .await
    }

    async fn spool_required_with_reason(
        &self,
        message: RuntimeMessage,
        message_id: String,
        reason: &'static str,
    ) -> Result<RuntimeSendReceipt, RuntimeChannelError> {
        let spool = Arc::clone(&self.spool);
        let sequence = tokio::task::spawn_blocking(move || spool.append(&message))
            .await
            .map_err(|error| RuntimeChannelError::Join(error.to_string()))??;
        self.metrics.spooled.fetch_add(1, Ordering::SeqCst);
        Ok(self.receipt(
            message_id,
            RuntimeSendOutcome::DurablySpooled,
            Some(sequence),
            matches!(
                self.policy.cursor_advance_policy,
                CursorAdvancePolicy::AfterDurableSpool
            ),
            ReadinessState::Degraded,
            reason,
        ))
    }

    pub async fn replay_spooled(
        &self,
        cancellation: &CancellationToken,
    ) -> Result<usize, RuntimeChannelError> {
        let spool = Arc::clone(&self.spool);
        let entries = tokio::task::spawn_blocking(move || spool.entries())
            .await
            .map_err(|error| RuntimeChannelError::Join(error.to_string()))??;
        let mut replayed = 0;
        for (sequence, message) in entries {
            {
                let mut in_flight = self.replay_in_flight.lock().map_err(|_| {
                    RuntimeChannelError::Spool("replay in-flight lock poisoned".to_string())
                })?;
                if in_flight.contains(&sequence) {
                    return Ok(replayed);
                }
                in_flight.insert(sequence);
            }
            tokio::select! {
                biased;
                () = cancellation.cancelled() => {
                    self.clear_replay_in_flight(sequence)?;
                    break
                },
                result = self.sender.send(RuntimeEnvelope {
                    message,
                    spool_sequence: Some(sequence),
                }) => {
                    if result.is_err() {
                        self.clear_replay_in_flight(sequence)?;
                        return Err(RuntimeChannelError::Closed(self.policy.id));
                    }
                    self.metrics.accepted.fetch_add(1, Ordering::SeqCst);
                    replayed += 1;
                }
            }
        }
        Ok(replayed)
    }

    async fn replay_next_spooled(
        &self,
        cancellation: &CancellationToken,
    ) -> Result<bool, RuntimeChannelError> {
        let spool = Arc::clone(&self.spool);
        let entries = tokio::task::spawn_blocking(move || spool.entries())
            .await
            .map_err(|error| RuntimeChannelError::Join(error.to_string()))??;
        for (sequence, message) in entries {
            {
                let mut in_flight = self.replay_in_flight.lock().map_err(|_| {
                    RuntimeChannelError::Spool("replay in-flight lock poisoned".to_string())
                })?;
                if in_flight.contains(&sequence) {
                    return Ok(false);
                }
                in_flight.insert(sequence);
            }
            let sent = tokio::select! {
                biased;
                () = cancellation.cancelled() => false,
                result = self.sender.send(RuntimeEnvelope {
                    message,
                    spool_sequence: Some(sequence),
                }) => {
                    result.map_err(|_| RuntimeChannelError::Closed(self.policy.id))?;
                    self.metrics.accepted.fetch_add(1, Ordering::SeqCst);
                    true
                }
            };
            if !sent {
                self.clear_replay_in_flight(sequence)?;
            }
            return Ok(sent);
        }
        Ok(false)
    }

    pub async fn acknowledge_processed(&self, sequence: u64) -> Result<(), RuntimeChannelError> {
        let spool = Arc::clone(&self.spool);
        let removed = tokio::task::spawn_blocking(move || spool.remove(sequence))
            .await
            .map_err(|error| RuntimeChannelError::Join(error.to_string()))??;
        if !removed {
            return Err(RuntimeChannelError::UnknownSpoolSequence(sequence));
        }
        self.clear_replay_in_flight(sequence)?;
        Ok(())
    }

    pub async fn acknowledge_published(
        &self,
        receipt: TransportPublishReceipt,
    ) -> Result<(), RuntimeChannelError> {
        if self.policy.id != RuntimeChannelId::CloudBridgeToAwsRoutes {
            return Err(RuntimeChannelError::InvalidPublishReceipt(
                "publish acknowledgement is valid only for cloud_bridge_to_aws_routes".to_string(),
            ));
        }
        let sequence = receipt.spool_sequence;
        let spool = Arc::clone(&self.spool);
        tokio::task::spawn_blocking(move || spool.acknowledge_published(&receipt))
            .await
            .map_err(|error| RuntimeChannelError::Join(error.to_string()))??;
        self.metrics.publish_acked.fetch_add(1, Ordering::SeqCst);
        self.clear_replay_in_flight(sequence)?;
        Ok(())
    }

    fn clear_replay_in_flight(&self, sequence: u64) -> Result<(), RuntimeChannelError> {
        self.replay_in_flight
            .lock()
            .map_err(|_| RuntimeChannelError::Spool("replay in-flight lock poisoned".to_string()))?
            .remove(&sequence);
        Ok(())
    }

    pub async fn snapshot(&self) -> Result<RuntimeChannelSnapshot, RuntimeChannelError> {
        let spool = Arc::clone(&self.spool);
        let durable_spool_depth = tokio::task::spawn_blocking(move || spool.len())
            .await
            .map_err(|error| RuntimeChannelError::Join(error.to_string()))??;
        let depth = self.policy.capacity.saturating_sub(self.sender.capacity());
        let spool = Arc::clone(&self.spool);
        let publish_cursor = tokio::task::spawn_blocking(move || spool.publish_cursor())
            .await
            .map_err(|error| RuntimeChannelError::Join(error.to_string()))??;
        let readiness = if depth >= self.policy.capacity {
            ReadinessState::Overloaded
        } else if durable_spool_depth > 0
            || self.metrics.throttled.load(Ordering::SeqCst) > 0
            || self.metrics.shed.load(Ordering::SeqCst) > 0
        {
            ReadinessState::Degraded
        } else {
            ReadinessState::Ready
        };
        Ok(RuntimeChannelSnapshot {
            channel: self.policy.id,
            capacity: self.policy.capacity,
            depth,
            accepted_count: self.metrics.accepted.load(Ordering::SeqCst),
            blocked_count: self.metrics.blocked.load(Ordering::SeqCst),
            throttled_count: self.metrics.throttled.load(Ordering::SeqCst),
            durable_spool_depth,
            spooled_count: self.metrics.spooled.load(Ordering::SeqCst),
            shed_count: self.metrics.shed.load(Ordering::SeqCst),
            cancelled_count: self.metrics.cancelled.load(Ordering::SeqCst),
            publish_acked_count: self.metrics.publish_acked.load(Ordering::SeqCst),
            publish_cursor,
            readiness,
        })
    }

    fn receipt(
        &self,
        message_id: String,
        outcome: RuntimeSendOutcome,
        spool_sequence: Option<u64>,
        cursor_may_advance: bool,
        readiness: ReadinessState,
        reason: &'static str,
    ) -> RuntimeSendReceipt {
        RuntimeSendReceipt {
            channel: self.policy.id,
            message_id,
            outcome,
            spool_sequence,
            cursor_may_advance,
            readiness,
            reason,
        }
    }
}

impl RuntimeChannelReceiver {
    pub async fn recv(&mut self) -> Option<RuntimeDelivery> {
        self.receiver.recv().await.map(|envelope| RuntimeDelivery {
            message: envelope.message,
            spool_sequence: envelope.spool_sequence,
        })
    }

    pub fn close(&mut self) {
        self.receiver.close();
    }

    pub fn policy(&self) -> RuntimeChannelPolicy {
        self.policy
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn backpressure_contract_keeps_required_state_fail_closed() {
        assert_eq!(REQUIRED_STATE_LOSS_POLICY, "never_silent_drop");
        assert_eq!(
            CSM_BACKPRESSURE_STATE_SCHEMA,
            "adl.csm.backpressure_state.v1"
        );
    }

    #[test]
    fn typed_matrix_covers_required_runtime_channels() {
        let channels = typed_channel_policy_matrix()
            .into_iter()
            .map(|policy| policy.id.as_str())
            .collect::<BTreeSet<_>>();

        assert_eq!(
            channels,
            BTreeSet::from([
                "aee_to_checkpoint",
                "cloud_bridge_to_aws_routes",
                "components_to_lifelog",
                "components_to_observability",
                "reasoning_runtime_to_aee",
                "runtime_api_to_control_plane",
                "scheduler_to_reasoning_runtime",
            ])
        );
    }

    #[test]
    fn critical_channels_do_not_silently_drop_when_full() {
        for id in [
            RuntimeChannelId::SchedulerToReasoningRuntime,
            RuntimeChannelId::ReasoningRuntimeToAee,
            RuntimeChannelId::AeeToCheckpoint,
            RuntimeChannelId::ComponentsToLifelog,
            RuntimeChannelId::CloudBridgeToAwsRoutes,
            RuntimeChannelId::RuntimeApiToControlPlane,
        ] {
            let policy = runtime_channel_policy(id);
            let decision =
                policy.decide(ChannelQueueSnapshot::full(policy.capacity), policy.priority);

            assert!(decision.preserves_required_state, "{id:?}");
            assert!(!decision.drop_accounted, "{id:?}");
            assert_ne!(decision.outcome, AdmissionOutcome::Shed, "{id:?}");
        }
    }

    #[test]
    fn scheduler_to_reasoning_runtime_blocks_full_queue() {
        let policy = runtime_channel_policy(RuntimeChannelId::SchedulerToReasoningRuntime);
        let decision = policy.decide(ChannelQueueSnapshot::full(policy.capacity), policy.priority);

        assert_eq!(decision.outcome, AdmissionOutcome::Blocked);
        assert_eq!(decision.readiness, ReadinessState::Overloaded);
        assert_eq!(decision.reason, "full_queue_blocks_required_state");
    }

    #[test]
    fn checkpoint_and_lifelog_use_durable_spool_under_pressure() {
        for id in [
            RuntimeChannelId::AeeToCheckpoint,
            RuntimeChannelId::ComponentsToLifelog,
        ] {
            let policy = runtime_channel_policy(id);
            let decision =
                policy.decide(ChannelQueueSnapshot::full(policy.capacity), policy.priority);

            assert_eq!(decision.outcome, AdmissionOutcome::Spooled, "{id:?}");
            assert!(decision.cursor_may_advance, "{id:?}");
            assert_eq!(decision.readiness, ReadinessState::Degraded, "{id:?}");
        }
    }

    #[test]
    fn observability_sheds_only_low_priority_metrics_with_accounting() {
        let policy = runtime_channel_policy(RuntimeChannelId::ComponentsToObservability);
        let full = ChannelQueueSnapshot::full(policy.capacity);

        let audit = policy.decide(full, ChannelPriority::Audit);
        assert_eq!(audit.outcome, AdmissionOutcome::Spooled);
        assert!(!audit.drop_accounted);
        assert!(audit.preserves_required_state);

        let metric = policy.decide(full, ChannelPriority::LowPriorityObservability);
        assert_eq!(metric.outcome, AdmissionOutcome::Shed);
        assert!(metric.drop_accounted);
        assert!(metric.preserves_required_state);
    }

    #[test]
    fn cloud_bridge_cursor_waits_for_publishable_ack() {
        let policy = runtime_channel_policy(RuntimeChannelId::CloudBridgeToAwsRoutes);
        let decision = policy.decide(ChannelQueueSnapshot::full(policy.capacity), policy.priority);

        assert_eq!(decision.outcome, AdmissionOutcome::Spooled);
        assert!(!decision.cursor_may_advance);
        assert_eq!(
            policy.cursor_advance_policy,
            CursorAdvancePolicy::AfterPublishableAck
        );
    }

    #[test]
    fn runtime_api_projects_overload_readiness_for_control_plane() {
        let policy = runtime_channel_policy(RuntimeChannelId::RuntimeApiToControlPlane);
        let decision = policy.decide(ChannelQueueSnapshot::full(policy.capacity), policy.priority);
        let projection = readiness_projection(&[decision]);

        assert_eq!(decision.outcome, AdmissionOutcome::Throttled);
        assert_eq!(projection["state"], "degraded");
        assert_eq!(projection["degraded_channel_count"], 1);
        assert_eq!(projection["required_state_silently_dropped"], false);
    }

    #[test]
    fn full_queue_projection_reports_overload_and_accounted_degrade() {
        let projection = typed_channel_full_queue_readiness_projection_json();

        assert_eq!(projection["state"], "overloaded");
        assert_eq!(projection["required_state_silently_dropped"], false);
        assert_eq!(projection["accounted_drop_count"], 1);
        assert_eq!(projection["overloaded_channel_count"], 2);
    }

    fn tiny_policy(
        id: RuntimeChannelId,
        full_queue_policy: FullQueuePolicy,
    ) -> RuntimeChannelPolicy {
        RuntimeChannelPolicy {
            id,
            source: "test_source",
            target: "test_target",
            capacity: 1,
            priority: ChannelPriority::CriticalContinuity,
            full_queue_policy,
            loss_policy: REQUIRED_STATE_LOSS_POLICY,
            cursor_advance_policy: CursorAdvancePolicy::AfterDurableSpool,
            health_signal: "test_backpressure",
            readiness_projection: "/ready.test",
        }
    }

    fn message(id: &str, priority: ChannelPriority) -> RuntimeMessage {
        RuntimeMessage::new(id, priority, json!({"id": id}))
    }

    #[tokio::test]
    async fn real_bounded_channel_blocks_until_receiver_releases_capacity() {
        let directory = tempfile::tempdir().expect("tempdir");
        let policy = tiny_policy(
            RuntimeChannelId::SchedulerToReasoningRuntime,
            FullQueuePolicy::BlockProducer,
        );
        let (sender, mut receiver) =
            runtime_channel(policy, directory.path().join("spool.redb")).expect("channel");
        let cancellation = CancellationToken::new();

        sender
            .send(
                message("first", ChannelPriority::GovernedExecution),
                &cancellation,
            )
            .await
            .expect("first send");
        let second_sender = sender.clone();
        let second_cancel = cancellation.clone();
        let blocked = tokio::spawn(async move {
            second_sender
                .send(
                    message("second", ChannelPriority::GovernedExecution),
                    &second_cancel,
                )
                .await
        });

        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        assert!(
            !blocked.is_finished(),
            "producer must remain blocked while full"
        );
        assert_eq!(
            receiver.recv().await.expect("first receive").message.id,
            "first"
        );
        let receipt = blocked.await.expect("join").expect("second send");
        assert_eq!(receipt.outcome, RuntimeSendOutcome::BlockedThenAccepted);
        assert_eq!(
            receiver.recv().await.expect("second receive").message.id,
            "second"
        );
        let snapshot = sender.snapshot().await.expect("snapshot");
        assert_eq!(snapshot.depth, 0);
        assert_eq!(snapshot.blocked_count, 1);
    }

    #[tokio::test]
    async fn blocked_send_cancellation_durably_spools_required_message() {
        let directory = tempfile::tempdir().expect("tempdir");
        let policy = tiny_policy(
            RuntimeChannelId::ReasoningRuntimeToAee,
            FullQueuePolicy::BlockProducer,
        );
        let (sender, mut receiver) =
            runtime_channel(policy, directory.path().join("spool.redb")).expect("channel");
        let cancellation = CancellationToken::new();
        sender
            .send(
                message("retained", ChannelPriority::GovernedExecution),
                &cancellation,
            )
            .await
            .expect("initial send");
        cancellation.cancel();
        let receipt = sender
            .send(
                message("cancelled", ChannelPriority::GovernedExecution),
                &cancellation,
            )
            .await
            .expect("cancelled receipt");
        assert_eq!(receipt.outcome, RuntimeSendOutcome::DurablySpooled);
        let cancelled_sequence = receipt.spool_sequence.expect("cancelled spool sequence");
        assert_eq!(
            receiver.recv().await.expect("retained message").message.id,
            "retained"
        );
        let replay_cancellation = CancellationToken::new();
        assert_eq!(
            sender
                .replay_spooled(&replay_cancellation)
                .await
                .expect("replay cancelled message"),
            1
        );
        let replayed = receiver.recv().await.expect("cancelled message replayed");
        assert_eq!(replayed.message.id, "cancelled");
        assert_eq!(replayed.spool_sequence, Some(cancelled_sequence));
        assert_eq!(
            sender
                .snapshot()
                .await
                .expect("snapshot")
                .durable_spool_depth,
            1
        );
        sender
            .acknowledge_processed(cancelled_sequence)
            .await
            .expect("processed acknowledgement");
        assert_eq!(
            sender.snapshot().await.expect("snapshot").cancelled_count,
            1
        );
    }

    #[tokio::test]
    async fn required_message_is_committed_and_replayed_from_redb_spool() {
        let directory = tempfile::tempdir().expect("tempdir");
        let spool_path = directory.path().join("spool.redb");
        let policy = tiny_policy(
            RuntimeChannelId::AeeToCheckpoint,
            FullQueuePolicy::DurableSpool,
        );
        let cancellation = CancellationToken::new();
        let (sender, receiver) = runtime_channel(policy, &spool_path).expect("channel");
        sender
            .send(
                message("queued", ChannelPriority::CriticalContinuity),
                &cancellation,
            )
            .await
            .expect("queued");
        let receipt = sender
            .send(
                message("spooled", ChannelPriority::CriticalContinuity),
                &cancellation,
            )
            .await
            .expect("spooled");
        assert_eq!(receipt.outcome, RuntimeSendOutcome::DurablySpooled);
        assert!(receipt.cursor_may_advance);
        assert_eq!(
            sender
                .snapshot()
                .await
                .expect("snapshot")
                .durable_spool_depth,
            1
        );
        drop(receiver);
        drop(sender);

        let (sender, mut receiver) = runtime_channel(policy, &spool_path).expect("reopen channel");
        assert_eq!(
            sender
                .snapshot()
                .await
                .expect("reopened snapshot")
                .durable_spool_depth,
            1
        );
        assert_eq!(
            sender.replay_spooled(&cancellation).await.expect("replay"),
            1
        );
        assert_eq!(
            sender
                .replay_spooled(&cancellation)
                .await
                .expect("duplicate replay suppression"),
            0
        );
        let replayed = receiver.recv().await.expect("replayed message");
        assert_eq!(replayed.message.id, "spooled");
        let sequence = replayed.spool_sequence.expect("replay spool sequence");
        assert_eq!(
            sender
                .snapshot()
                .await
                .expect("unacknowledged snapshot")
                .durable_spool_depth,
            1
        );
        drop(receiver);
        drop(sender);

        let (sender, mut receiver) = runtime_channel(policy, &spool_path).expect("crash reopen");
        assert_eq!(
            sender
                .replay_spooled(&cancellation)
                .await
                .expect("post-crash replay"),
            1
        );
        let replayed_after_crash = receiver.recv().await.expect("post-crash delivery");
        assert_eq!(replayed_after_crash.message.id, "spooled");
        assert_eq!(replayed_after_crash.spool_sequence, Some(sequence));
        sender
            .acknowledge_processed(sequence)
            .await
            .expect("processed acknowledgement");
        assert_eq!(
            sender
                .snapshot()
                .await
                .expect("acknowledged snapshot")
                .durable_spool_depth,
            0
        );
    }

    #[tokio::test]
    async fn observability_spools_audit_and_accounts_low_priority_shed() {
        let directory = tempfile::tempdir().expect("tempdir");
        let policy = tiny_policy(
            RuntimeChannelId::ComponentsToObservability,
            FullQueuePolicy::ShedLowPriorityOnly,
        );
        let (sender, _receiver) =
            runtime_channel(policy, directory.path().join("spool.redb")).expect("channel");
        let cancellation = CancellationToken::new();
        sender
            .send(message("capacity", ChannelPriority::Audit), &cancellation)
            .await
            .expect("capacity message");
        let audit = sender
            .send(message("audit", ChannelPriority::Audit), &cancellation)
            .await
            .expect("audit spool");
        let metric = sender
            .send(
                message("metric", ChannelPriority::LowPriorityObservability),
                &cancellation,
            )
            .await
            .expect("metric shed");
        assert_eq!(audit.outcome, RuntimeSendOutcome::DurablySpooled);
        assert_eq!(metric.outcome, RuntimeSendOutcome::Shed);
        let snapshot = sender.snapshot().await.expect("snapshot");
        assert_eq!(snapshot.durable_spool_depth, 1);
        assert_eq!(snapshot.shed_count, 1);
        assert_eq!(snapshot.readiness, ReadinessState::Overloaded);
    }

    #[tokio::test]
    async fn cloud_cursor_waits_for_explicit_publishable_ack() {
        let directory = tempfile::tempdir().expect("tempdir");
        let mut policy = tiny_policy(
            RuntimeChannelId::CloudBridgeToAwsRoutes,
            FullQueuePolicy::DurableSpool,
        );
        policy.cursor_advance_policy = CursorAdvancePolicy::AfterPublishableAck;
        let (sender, _receiver) =
            runtime_channel(policy, directory.path().join("spool.redb")).expect("channel");
        let cancellation = CancellationToken::new();
        sender
            .send(
                message("capacity", ChannelPriority::Evidence),
                &cancellation,
            )
            .await
            .expect("capacity message");
        let receipt = sender
            .send(message("publish", ChannelPriority::Evidence), &cancellation)
            .await
            .expect("spooled publish");
        assert!(!receipt.cursor_may_advance);
        let sequence = receipt.spool_sequence.expect("spool sequence");
        let transport_receipt =
            TransportPublishReceipt::verified(sequence, 1, "aws_eventbridge", "event-1")
                .expect("verified transport receipt");
        sender
            .acknowledge_published(transport_receipt)
            .await
            .expect("publish ack");
        let snapshot = sender.snapshot().await.expect("snapshot");
        assert_eq!(snapshot.durable_spool_depth, 0);
        assert_eq!(snapshot.publish_acked_count, 1);
        assert_eq!(snapshot.publish_cursor, 1);
    }

    #[tokio::test]
    async fn cloud_publish_ack_rejects_unknown_spool_sequence() {
        let directory = tempfile::tempdir().expect("tempdir");
        let policy = tiny_policy(
            RuntimeChannelId::CloudBridgeToAwsRoutes,
            FullQueuePolicy::DurableSpool,
        );
        let (sender, _receiver) =
            runtime_channel(policy, directory.path().join("spool.redb")).expect("channel");

        let transport_receipt =
            TransportPublishReceipt::verified(404, 1, "aws_eventbridge", "event-404")
                .expect("verified transport receipt");
        let error = sender
            .acknowledge_published(transport_receipt)
            .await
            .expect_err("unknown sequence must fail closed");
        assert!(matches!(
            error,
            RuntimeChannelError::UnknownSpoolSequence(404)
        ));
        assert_eq!(
            sender
                .snapshot()
                .await
                .expect("snapshot")
                .publish_acked_count,
            0
        );
    }

    #[tokio::test]
    async fn cloud_cursor_rejects_gaps_and_sequence_survives_restart() {
        let directory = tempfile::tempdir().expect("tempdir");
        let spool_path = directory.path().join("spool.redb");
        let policy = tiny_policy(
            RuntimeChannelId::CloudBridgeToAwsRoutes,
            FullQueuePolicy::DurableSpool,
        );
        let cancellation = CancellationToken::new();
        let (sender, receiver) = runtime_channel(policy, &spool_path).expect("channel");
        sender
            .send(
                message("capacity", ChannelPriority::Evidence),
                &cancellation,
            )
            .await
            .expect("capacity");
        let first = sender
            .send(message("first", ChannelPriority::Evidence), &cancellation)
            .await
            .expect("first spool")
            .spool_sequence
            .expect("first sequence");
        let second = sender
            .send(message("second", ChannelPriority::Evidence), &cancellation)
            .await
            .expect("second spool")
            .spool_sequence
            .expect("second sequence");
        assert_eq!((first, second), (1, 2));

        let gap = sender
            .acknowledge_published(
                TransportPublishReceipt::verified(second, second, "eventbridge", "event-2")
                    .expect("receipt"),
            )
            .await
            .expect_err("cursor gap must fail closed");
        assert!(matches!(
            gap,
            RuntimeChannelError::NonMonotonicPublishCursor {
                current: 0,
                proposed: 2
            }
        ));
        sender
            .acknowledge_published(
                TransportPublishReceipt::verified(first, first, "eventbridge", "event-1")
                    .expect("receipt"),
            )
            .await
            .expect("first ack");
        sender
            .acknowledge_published(
                TransportPublishReceipt::verified(second, second, "eventbridge", "event-2")
                    .expect("receipt"),
            )
            .await
            .expect("second ack");
        drop(receiver);
        drop(sender);

        let (sender, _receiver) = runtime_channel(policy, &spool_path).expect("restart channel");
        let third = sender
            .spool_required_with_reason(
                message("third", ChannelPriority::Evidence),
                "third".to_string(),
                "restart_sequence_probe",
            )
            .await
            .expect("third spool")
            .spool_sequence
            .expect("third sequence");
        assert_eq!(third, 3);
        assert_eq!(sender.snapshot().await.expect("snapshot").publish_cursor, 2);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn concurrent_receive_cannot_underflow_queue_depth() {
        let directory = tempfile::tempdir().expect("tempdir");
        let policy = tiny_policy(
            RuntimeChannelId::SchedulerToReasoningRuntime,
            FullQueuePolicy::BlockProducer,
        );
        let (sender, mut receiver) =
            runtime_channel(policy, directory.path().join("spool.redb")).expect("channel");
        let cancellation = CancellationToken::new();

        for index in 0..100 {
            let producer = sender.clone();
            let cancellation = cancellation.clone();
            let send = tokio::spawn(async move {
                producer
                    .send(
                        message(&format!("race-{index}"), ChannelPriority::GovernedExecution),
                        &cancellation,
                    )
                    .await
            });
            let delivery = receiver.recv().await.expect("delivery");
            assert_eq!(delivery.message.id, format!("race-{index}"));
            send.await.expect("send join").expect("send result");
            assert!(sender.snapshot().await.expect("snapshot").depth <= policy.capacity);
        }
        assert_eq!(sender.snapshot().await.expect("final snapshot").depth, 0);
    }

    #[tokio::test]
    async fn runtime_channel_fabric_owns_all_channels_and_transits_messages() {
        let directory = tempfile::tempdir().expect("tempdir");
        let mut fabric = RuntimeChannelFabric::open(directory.path()).expect("fabric");
        let cancellation = CancellationToken::new();
        let (receipt, delivery) = fabric
            .transit(
                RuntimeChannelId::SchedulerToReasoningRuntime,
                message("cycle-admission", ChannelPriority::GovernedExecution),
                &cancellation,
            )
            .await
            .expect("transit");

        assert_eq!(receipt.outcome, RuntimeSendOutcome::Accepted);
        assert_eq!(delivery.expect("delivery").message.id, "cycle-admission");
        let snapshots = fabric.snapshots().await.expect("snapshots");
        assert_eq!(snapshots.len(), RuntimeChannelId::ALL.len());
        assert!(snapshots.iter().all(|snapshot| snapshot.depth == 0));
    }

    #[tokio::test]
    async fn runtime_channel_fabric_replays_and_acknowledges_oldest_durable_record() {
        let directory = tempfile::tempdir().expect("tempdir");
        let cancellation = CancellationToken::new();
        let channel = RuntimeChannelId::ComponentsToLifelog;
        let first_sequence = {
            let fabric = RuntimeChannelFabric::open(directory.path()).expect("fabric");
            fabric
                .persist_required(
                    channel,
                    message("retained-lifecycle", ChannelPriority::Audit),
                )
                .await
                .expect("persist")
                .spool_sequence
                .expect("sequence")
        };

        let mut restarted = RuntimeChannelFabric::open(directory.path()).expect("restart fabric");
        let delivery = restarted
            .replay_next(channel, &cancellation)
            .await
            .expect("replay")
            .expect("delivery");
        assert_eq!(delivery.message.id, "retained-lifecycle");
        assert_eq!(delivery.spool_sequence, Some(first_sequence));
        restarted
            .acknowledge_processed(channel, first_sequence)
            .await
            .expect("acknowledge");
        assert!(restarted
            .replay_next(channel, &cancellation)
            .await
            .expect("empty replay")
            .is_none());
    }
}
