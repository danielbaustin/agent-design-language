use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const TASK_ADAPTER_CONTRACT_VERSION: &str = "adl.workcell-task-adapter.v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterLimits {
    pub max_context_bytes: usize,
    pub max_idempotency_key_bytes: usize,
    pub max_idempotency_entries: usize,
    pub max_evidence_refs: usize,
    pub max_evidence_ref_bytes: usize,
    pub max_deadline_ms: u64,
}

impl Default for AdapterLimits {
    fn default() -> Self {
        Self {
            max_context_bytes: 64 * 1024,
            max_idempotency_key_bytes: 256,
            max_idempotency_entries: 10_000,
            max_evidence_refs: 128,
            max_evidence_ref_bytes: 1_024,
            max_deadline_ms: 300_000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskRequest {
    pub contract: String,
    pub idempotency_key: String,
    pub operation: TaskOperation,
    pub authority: TaskAuthority,
    pub assignment_digest: String,
    pub dependency_digest: String,
    pub context: ContextPacket,
    pub observed_unix_seconds: u64,
    pub deadline_ms: u64,
    pub caller: CallerAuthority,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskAuthority {
    pub issue: u64,
    pub claim_id: String,
    pub claim_owner: String,
    pub claim_generation: u64,
    pub branch: String,
    pub worktree: String,
    pub protected_paths: Vec<String>,
    pub write_paths: Vec<String>,
    pub freshness_token: String,
    pub expires_unix_seconds: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextPacket {
    pub provenance: Vec<String>,
    pub scope: Vec<String>,
    pub expected_output: String,
    pub validation: Vec<String>,
    pub freshness_token: String,
    pub content_digest: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CallerAuthority {
    pub subject: String,
    pub may_cancel: bool,
    pub may_escalate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskOperation {
    Create { client_task_key: String },
    Attach { task: TaskRef },
    Message { task: TaskRef },
    Handoff { task: TaskRef, output_ref: String },
    Inspect { task: TaskRef },
    Cancel { task: TaskRef },
    Escalate { task: TaskRef, reason_code: String },
}

impl TaskOperation {
    pub fn kind(&self) -> OperationKind {
        match self {
            Self::Create { .. } => OperationKind::Create,
            Self::Attach { .. } => OperationKind::Attach,
            Self::Message { .. } => OperationKind::Message,
            Self::Handoff { .. } => OperationKind::Handoff,
            Self::Inspect { .. } => OperationKind::Inspect,
            Self::Cancel { .. } => OperationKind::Cancel,
            Self::Escalate { .. } => OperationKind::Escalate,
        }
    }

    pub(crate) fn task(&self) -> Option<&TaskRef> {
        match self {
            Self::Create { .. } => None,
            Self::Attach { task }
            | Self::Message { task }
            | Self::Handoff { task, .. }
            | Self::Inspect { task }
            | Self::Cancel { task }
            | Self::Escalate { task, .. } => Some(task),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationKind {
    Create,
    Attach,
    Message,
    Handoff,
    Inspect,
    Cancel,
    Escalate,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskRef {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskObservation {
    pub task: TaskRef,
    pub status: TaskStatus,
    pub sequence: u64,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Running,
    Completed,
    Cancelled,
    Failed,
    Unknown,
}

impl TaskStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Cancelled | Self::Failed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskOutcome {
    Created,
    Attached,
    MessageAccepted,
    HandoffAccepted,
    Observed(TaskObservation),
    Cancelled,
    CompletedBeforeCancel,
    CancelRejected,
    Escalated,
    Indeterminate,
}

impl TaskOutcome {
    pub(crate) fn status(&self) -> Option<TaskStatus> {
        match self {
            Self::Observed(observation) => Some(observation.status.clone()),
            Self::Cancelled => Some(TaskStatus::Cancelled),
            Self::CompletedBeforeCancel => Some(TaskStatus::Completed),
            Self::CancelRejected => Some(TaskStatus::Failed),
            Self::Created
            | Self::Attached
            | Self::MessageAccepted
            | Self::HandoffAccepted
            | Self::Escalated
            | Self::Indeterminate => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportReceipt {
    pub task: Option<TaskRef>,
    pub outcome: TaskOutcome,
    pub transport_timestamp_ms: u64,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskReceipt {
    pub contract: String,
    pub idempotency_key: String,
    pub request_digest: String,
    pub operation: OperationKind,
    pub task: Option<TaskRef>,
    pub outcome: TaskOutcome,
    pub transport_timestamp_ms: u64,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportFailureClass {
    Unavailable,
    Rejected,
    Protocol,
    Saturated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportFailure {
    pub class: TransportFailureClass,
    pub private_detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityFailure {
    pub private_detail: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskTransportErrorCode {
    InvalidRequest,
    InvalidContext,
    InvalidPath,
    AuthorityDenied,
    IdempotencyCollision,
    TerminalTask,
    Indeterminate,
    ResourceLimit,
    Serialization,
    TransportUnavailable,
    TransportRejected,
    TransportProtocol,
    TransportSaturated,
}

#[derive(Debug, Clone, PartialEq, Eq, Error, Serialize, Deserialize)]
#[error("task adapter error: {code:?}")]
#[serde(deny_unknown_fields)]
pub struct TaskTransportError {
    pub code: TaskTransportErrorCode,
}

impl TaskTransportError {
    pub(crate) fn invalid_request(_: &str) -> Self {
        Self {
            code: TaskTransportErrorCode::InvalidRequest,
        }
    }

    pub(crate) fn invalid_context() -> Self {
        Self {
            code: TaskTransportErrorCode::InvalidContext,
        }
    }

    pub(crate) fn invalid_path() -> Self {
        Self {
            code: TaskTransportErrorCode::InvalidPath,
        }
    }

    pub(crate) fn authority_denied() -> Self {
        Self {
            code: TaskTransportErrorCode::AuthorityDenied,
        }
    }

    pub(crate) fn idempotency_collision() -> Self {
        Self {
            code: TaskTransportErrorCode::IdempotencyCollision,
        }
    }

    pub(crate) fn terminal_task() -> Self {
        Self {
            code: TaskTransportErrorCode::TerminalTask,
        }
    }

    pub(crate) fn indeterminate() -> Self {
        Self {
            code: TaskTransportErrorCode::Indeterminate,
        }
    }

    pub(crate) fn resource_limit() -> Self {
        Self {
            code: TaskTransportErrorCode::ResourceLimit,
        }
    }

    pub(crate) fn serialization() -> Self {
        Self {
            code: TaskTransportErrorCode::Serialization,
        }
    }

    pub(crate) fn from_authority(_: AuthorityFailure) -> Self {
        Self::authority_denied()
    }

    pub(crate) fn from_transport(failure: TransportFailure) -> Self {
        let code = match failure.class {
            TransportFailureClass::Unavailable => TaskTransportErrorCode::TransportUnavailable,
            TransportFailureClass::Rejected => TaskTransportErrorCode::TransportRejected,
            TransportFailureClass::Protocol => TaskTransportErrorCode::TransportProtocol,
            TransportFailureClass::Saturated => TaskTransportErrorCode::TransportSaturated,
        };
        Self { code }
    }
}
