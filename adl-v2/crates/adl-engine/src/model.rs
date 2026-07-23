use adl_compiler::{PlanPrompt, PlanRun};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const ENGINE_CONTRACT_VERSION: &str = "adl.engine.v1";
pub const CHECKPOINT_CONTRACT_VERSION: &str = "adl.engine-checkpoint.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineErrorCode {
    InvalidPlan,
    InvalidLimits,
    InvalidPolicy,
    Protocol,
    ResourceLimit,
    CheckpointNotQuiescent,
    CheckpointIncompatible,
    Serialization,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineError {
    pub code: EngineErrorCode,
    pub path: String,
    pub message: String,
}

impl EngineError {
    pub(crate) fn new(code: EngineErrorCode, path: &str, message: &str) -> Self {
        Self {
            code,
            path: String::from(path),
            message: String::from(message),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineLimits {
    pub max_plan_nodes: u64,
    pub max_dependency_edges: u64,
    pub max_plan_bytes: u64,
    pub max_policy_bytes: u64,
    pub max_ready_nodes: u64,
    pub max_in_flight: u64,
    pub max_total_attempts: u64,
    pub max_attempts_per_node: u32,
    pub max_request_bytes: u64,
    pub max_completion_bytes: u64,
    pub max_completions_per_turn: u64,
    pub max_cancellations_per_turn: u64,
    pub max_turn_input_bytes: u64,
    pub max_output_bytes: u64,
    pub max_events: u64,
    pub max_checkpoint_bytes: u64,
    pub max_logical_turns: u64,
}

impl Default for EngineLimits {
    fn default() -> Self {
        Self {
            max_plan_nodes: 10_000,
            max_dependency_edges: 100_000,
            max_plan_bytes: 16_777_216,
            max_policy_bytes: 16_777_216,
            max_ready_nodes: 1_000,
            max_in_flight: 64,
            max_total_attempts: 100_000,
            max_attempts_per_node: 8,
            max_request_bytes: 1_048_576,
            max_completion_bytes: 16_777_216,
            max_completions_per_turn: 1_000,
            max_cancellations_per_turn: 10_000,
            max_turn_input_bytes: 33_554_432,
            max_output_bytes: 16_777_216,
            max_events: 1_000_000,
            max_checkpoint_bytes: 33_554_432,
            max_logical_turns: 1_000_000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureClass {
    Retryable,
    Permanent,
    InvalidRequest,
    PolicyDenied,
    Cancelled,
    Dependency,
    Saturation,
    Protocol,
    Timeout,
    RetryExhausted,
    Resource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PortFailure {
    pub class: FailureClass,
    pub message: String,
}

impl PortFailure {
    pub fn new(class: FailureClass, message: &str) -> Self {
        Self {
            class,
            message: String::from(message),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PortOutput {
    pub media_type: String,
    pub bytes: Vec<u8>,
}

impl PortOutput {
    pub fn new(media_type: &str, bytes: Vec<u8>) -> Self {
        Self {
            media_type: String::from(media_type),
            bytes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionOutcome {
    Success(PortOutput),
    Failure(PortFailure),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortKind {
    Provider,
    Tool { name: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JoinPolicy {
    All,
    AtLeast { required: u64 },
    FailFast,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub retryable: BTreeSet<FailureClass>,
    pub delay_ticks: Vec<u64>,
}

impl RetryPolicy {
    pub fn once() -> Self {
        Self {
            max_attempts: 1,
            retryable: BTreeSet::new(),
            delay_ticks: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodePolicy {
    pub port: PortKind,
    pub join: JoinPolicy,
    pub retry: RetryPolicy,
    pub timeout_ticks: u64,
}

impl NodePolicy {
    pub fn provider_once(timeout_ticks: u64) -> Self {
        Self {
            port: PortKind::Provider,
            join: JoinPolicy::FailFast,
            retry: RetryPolicy::once(),
            timeout_ticks,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnginePolicy {
    pub nodes: BTreeMap<String, NodePolicy>,
}

impl EnginePolicy {
    pub fn new(nodes: BTreeMap<String, NodePolicy>) -> Self {
        Self { nodes }
    }

    pub fn provider_for(plan: &adl_compiler::ExecutionPlan, timeout_ticks: u64) -> Self {
        let mut nodes = BTreeMap::new();
        for node in &plan.nodes {
            nodes.insert(node.id.clone(), NodePolicy::provider_once(timeout_ticks));
        }
        Self { nodes }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderRequest {
    pub request_id: String,
    pub idempotency_key: String,
    pub sequence: u64,
    pub node_id: String,
    pub attempt: u32,
    pub provider_ref: String,
    pub model: Option<String>,
    pub prompt: PlanPrompt,
    pub inputs: BTreeMap<String, Value>,
    pub timeout_ticks: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolRequest {
    pub request_id: String,
    pub idempotency_key: String,
    pub sequence: u64,
    pub node_id: String,
    pub attempt: u32,
    pub tool: String,
    pub run: PlanRun,
    pub inputs: BTreeMap<String, Value>,
    pub timeout_ticks: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelRequest {
    pub request_id: String,
    pub idempotency_key: String,
    pub node_id: String,
    pub attempt: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineEffect {
    Provider(Box<ProviderRequest>),
    Tool(Box<ToolRequest>),
    Cancel(CancelRequest),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderCompletion {
    pub request_id: String,
    pub node_id: String,
    pub attempt: u32,
    pub outcome: CompletionOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolCompletion {
    pub request_id: String,
    pub node_id: String,
    pub attempt: u32,
    pub outcome: CompletionOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelCompletion {
    pub request_id: String,
    pub node_id: String,
    pub attempt: u32,
    pub acknowledged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortCompletion {
    Provider(Box<ProviderCompletion>),
    Tool(Box<ToolCompletion>),
    Cancel(CancelCompletion),
}

impl PortCompletion {
    pub(crate) fn request_id(&self) -> &str {
        match self {
            Self::Provider(completion) => &completion.request_id,
            Self::Tool(completion) => &completion.request_id,
            Self::Cancel(completion) => &completion.request_id,
        }
    }

    pub(crate) fn identity(&self) -> (&str, u32) {
        match self {
            Self::Provider(completion) => (&completion.node_id, completion.attempt),
            Self::Tool(completion) => (&completion.node_id, completion.attempt),
            Self::Cancel(completion) => (&completion.node_id, completion.attempt),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeState {
    Pending,
    Ready,
    Dispatched {
        request_id: String,
        attempt: u32,
        sequence: u64,
        input_digest: String,
    },
    RetryWait {
        ready_at_tick: u64,
    },
    Cancelling {
        request_id: String,
        attempt: u32,
        sequence: u64,
        input_digest: String,
    },
    Succeeded {
        output: PortOutput,
    },
    Failed {
        failure: PortFailure,
    },
    Cancelled,
}

impl NodeState {
    pub fn is_terminal(&self) -> bool {
        match self {
            Self::Succeeded { .. } | Self::Failed { .. } | Self::Cancelled => true,
            Self::Pending
            | Self::Ready
            | Self::Dispatched { .. }
            | Self::RetryWait { .. }
            | Self::Cancelling { .. } => false,
        }
    }

    pub fn is_in_flight(&self) -> bool {
        match self {
            Self::Dispatched { .. } | Self::Cancelling { .. } => true,
            Self::Pending
            | Self::Ready
            | Self::RetryWait { .. }
            | Self::Succeeded { .. }
            | Self::Failed { .. }
            | Self::Cancelled => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeSnapshot {
    pub state: NodeState,
    pub attempts: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompletionReceipt {
    pub node_id: String,
    pub attempt: u32,
    pub sequence: u64,
    pub input_digest: String,
    pub completed_at_tick: u64,
    pub completion: PortCompletion,
    pub completion_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineSnapshot {
    pub checkpoint_contract: String,
    pub engine_contract: String,
    pub plan_contract: String,
    pub plan_source_digest: String,
    pub plan_digest: String,
    pub policy_digest: String,
    pub node_ids: Vec<String>,
    pub edge_ids: Vec<String>,
    pub limits: EngineLimits,
    pub logical_tick: u64,
    pub logical_turns: u64,
    pub attempts_consumed: u64,
    pub output_bytes: u64,
    pub event_count: u64,
    pub next_event_sequence: u64,
    pub next_request_sequence: u64,
    pub nodes: BTreeMap<String, NodeSnapshot>,
    pub turn_journal: Vec<TurnInput>,
    pub consumed_completion_digests: BTreeMap<String, CompletionReceipt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    NodeReady,
    RequestDispatched { request_id: String, attempt: u32 },
    RetryScheduled { ready_at_tick: u64 },
    NodeSucceeded,
    NodeFailed { failure: PortFailure },
    NodeCancelled,
    CancellationRequested { request_id: String },
    Backpressure { queued: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineEvent {
    pub sequence: u64,
    pub node_id: Option<String>,
    pub kind: EventKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TurnInput {
    pub logical_tick: u64,
    pub completions: Vec<PortCompletion>,
    pub cancellations: Vec<String>,
}

impl TurnInput {
    pub fn tick(logical_tick: u64) -> Self {
        Self {
            logical_tick,
            completions: Vec::new(),
            cancellations: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TurnOutput {
    pub snapshot: EngineSnapshot,
    pub effects: Vec<EngineEffect>,
    pub events: Vec<EngineEvent>,
}
