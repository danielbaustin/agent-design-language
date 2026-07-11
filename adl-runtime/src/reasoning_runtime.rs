//! Native CSM reasoning graph, bounded-loop, and adaptive-DAG runtime.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tokio::sync::{mpsc, oneshot, watch};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

pub const REASONING_RUNTIME_SCHEMA: &str = "adl.csm.reasoning_runtime.v1";
pub const REASONING_RUNTIME_STATUS_SCHEMA: &str = "adl.csm.reasoning_runtime.status.v1";
pub const REASONING_RUNTIME_CHECKPOINT_SCHEMA: &str = "adl.csm.reasoning_runtime.checkpoint.v1";
pub const REASONING_RUNTIME_LIFELOG_SCHEMA: &str = "adl.csm.reasoning_runtime.lifelog.v1";
pub const REASONING_RUNTIME_TELEMETRY_SCHEMA: &str = "adl.csm.reasoning_runtime.telemetry.v1";
pub const REASONING_RUNTIME_COMPONENT: &str = "reasoning_runtime";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningNodeKind {
    PromptInput,
    Hypothesis,
    Evidence,
    Decision,
    Outcome,
    ProviderObservation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningNode {
    pub id: String,
    pub kind: ReasoningNodeKind,
    #[serde(default)]
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningGraph {
    pub graph_id: String,
    pub nodes: Vec<ReasoningNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundedLoop {
    pub loop_id: String,
    pub graph: ReasoningGraph,
    pub max_iterations: u32,
    pub exit_node_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptiveDag {
    pub graph: ReasoningGraph,
    pub max_adaptations: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ReasoningObject {
    Graph(ReasoningGraph),
    Loop(BoundedLoop),
    AdaptiveDag(AdaptiveDag),
}

impl ReasoningObject {
    pub fn object_id(&self) -> &str {
        match self {
            Self::Graph(graph) | Self::AdaptiveDag(AdaptiveDag { graph, .. }) => &graph.graph_id,
            Self::Loop(loop_object) => &loop_object.loop_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapturedProviderResult {
    pub event_id: String,
    pub source: String,
    pub retention_ref: String,
    pub payload_fingerprint: String,
    #[serde(default)]
    pub proposed_nodes: Vec<ReasoningNode>,
}

impl CapturedProviderResult {
    pub fn capture(
        event_id: impl Into<String>,
        source: impl Into<String>,
        retention_ref: impl Into<String>,
        payload: &Value,
        proposed_nodes: Vec<ReasoningNode>,
    ) -> Result<Self, ReasoningRuntimeError> {
        let canonical = serde_jcs::to_vec(payload)
            .map_err(|_| ReasoningRuntimeError::InvalidProviderCapture)?;
        Ok(Self {
            event_id: event_id.into(),
            source: source.into(),
            retention_ref: retention_ref.into(),
            payload_fingerprint: sha256_hex(&canonical),
            proposed_nodes,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreedomGateDisposition {
    Approved,
    Denied,
    Deferred,
    Challenged,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceContext {
    pub freedom_gate: FreedomGateDisposition,
    pub aee_available: bool,
    pub policy_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningAdmission {
    pub admission_id: String,
    pub object: ReasoningObject,
    pub governance: GovernanceContext,
    pub checkpoint: Option<ReasoningCheckpoint>,
    pub provider_result: Option<CapturedProviderResult>,
    pub replay_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningCheckpoint {
    pub schema: String,
    pub object_id: String,
    pub lineage_id: String,
    pub checkpoint_version: u64,
    pub replay_cursor: u64,
    pub state_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningLifelogEvent {
    pub schema: String,
    pub sequence: u64,
    pub object_id: String,
    pub event: String,
    pub reason_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernedAeeRequest {
    pub request_id: String,
    pub object_id: String,
    pub decision_node_id: String,
    pub policy_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningTelemetry {
    pub schema: String,
    pub component: String,
    pub object_id_hash: String,
    pub outcome: String,
    pub nodes_executed: u64,
    pub iterations: u32,
    pub adaptations: u32,
    pub provider_payload_retained: bool,
    pub redacted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningExecution {
    pub schema: String,
    pub admission_id: String,
    pub object_id: String,
    pub canonical_order: Vec<String>,
    pub iterations: u32,
    pub adaptations: u32,
    pub provider_event_id: Option<String>,
    pub aee_request: Option<GovernedAeeRequest>,
    pub checkpoint: ReasoningCheckpoint,
    pub lifelog: Vec<ReasoningLifelogEvent>,
    pub telemetry: ReasoningTelemetry,
    pub execution_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningRuntimeError {
    EmptyIdentifier,
    EmptyGraph,
    DuplicateNode,
    MissingDependency,
    CycleDetected,
    InvalidLoopLimit,
    MissingLoopExit,
    AdaptationLimitExceeded,
    ProviderCaptureRequired,
    InvalidProviderCapture,
    StaleCheckpoint,
    CheckpointObjectMismatch,
    FreedomGateDenied,
    FreedomGateDeferred,
    AeeUnavailable,
    QueueFull,
    ComponentStopped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuarantinedReasoningObject {
    pub object_id: String,
    pub admission_id: String,
    pub reason: ReasoningRuntimeError,
    pub input_evidence_preserved: bool,
}

#[derive(Debug, Default)]
pub struct ReasoningCore {
    checkpoint_versions: BTreeMap<String, u64>,
    replay_cursors: BTreeMap<String, u64>,
    lifelog_sequence: u64,
    quarantined: BTreeMap<String, QuarantinedReasoningObject>,
}

impl ReasoningCore {
    pub fn execute(
        &mut self,
        admission: ReasoningAdmission,
    ) -> Result<ReasoningExecution, ReasoningRuntimeError> {
        let object_id = admission.object.object_id().to_string();
        let result = self.execute_inner(&admission);
        if let Err(reason) = &result {
            self.quarantined.insert(
                object_id.clone(),
                QuarantinedReasoningObject {
                    object_id,
                    admission_id: admission.admission_id,
                    reason: reason.clone(),
                    input_evidence_preserved: true,
                },
            );
        }
        result
    }

    pub fn quarantined(&self) -> Vec<QuarantinedReasoningObject> {
        self.quarantined.values().cloned().collect()
    }

    fn execute_inner(
        &mut self,
        admission: &ReasoningAdmission,
    ) -> Result<ReasoningExecution, ReasoningRuntimeError> {
        validate_identifier(&admission.admission_id)?;
        let object_id = admission.object.object_id();
        validate_identifier(object_id)?;
        self.validate_checkpoint(object_id, admission.checkpoint.as_ref())?;

        let (mut graph, iterations, max_adaptations) = match &admission.object {
            ReasoningObject::Graph(graph) => (graph.clone(), 1, 0),
            ReasoningObject::Loop(loop_object) => {
                if loop_object.max_iterations == 0 {
                    return Err(ReasoningRuntimeError::InvalidLoopLimit);
                }
                if !loop_object
                    .graph
                    .nodes
                    .iter()
                    .any(|node| node.id == loop_object.exit_node_id)
                {
                    return Err(ReasoningRuntimeError::MissingLoopExit);
                }
                (loop_object.graph.clone(), loop_object.max_iterations, 0)
            }
            ReasoningObject::AdaptiveDag(dag) => (dag.graph.clone(), 1, dag.max_adaptations),
        };

        let adaptations = if max_adaptations > 0 {
            let capture = admission
                .provider_result
                .as_ref()
                .ok_or(ReasoningRuntimeError::ProviderCaptureRequired)?;
            if !admission.replay_only && capture.retention_ref.trim().is_empty() {
                return Err(ReasoningRuntimeError::InvalidProviderCapture);
            }
            let count = u32::try_from(capture.proposed_nodes.len()).unwrap_or(u32::MAX);
            if count > max_adaptations {
                return Err(ReasoningRuntimeError::AdaptationLimitExceeded);
            }
            let mut proposed = capture.proposed_nodes.clone();
            proposed.sort_by(|left, right| left.id.cmp(&right.id));
            graph.nodes.extend(proposed);
            count
        } else {
            0
        };

        let canonical_order = canonical_topological_order(&graph)?;
        let decision_node_id = canonical_order
            .iter()
            .find(|id| {
                graph
                    .nodes
                    .iter()
                    .any(|node| node.id == **id && node.kind == ReasoningNodeKind::Decision)
            })
            .cloned();
        let aee_request = match admission.governance.freedom_gate {
            FreedomGateDisposition::Denied | FreedomGateDisposition::Challenged => {
                return Err(ReasoningRuntimeError::FreedomGateDenied)
            }
            FreedomGateDisposition::Deferred => {
                return Err(ReasoningRuntimeError::FreedomGateDeferred)
            }
            FreedomGateDisposition::Approved if !admission.governance.aee_available => {
                return Err(ReasoningRuntimeError::AeeUnavailable)
            }
            FreedomGateDisposition::Approved => {
                decision_node_id.map(|decision_node_id| GovernedAeeRequest {
                    request_id: format!("aee:{}", admission.admission_id),
                    object_id: object_id.to_string(),
                    decision_node_id,
                    policy_ref: admission.governance.policy_ref.clone(),
                })
            }
        };

        let replay_cursor = self
            .replay_cursors
            .get(object_id)
            .copied()
            .unwrap_or(0)
            .saturating_add(u64::try_from(canonical_order.len()).unwrap_or(u64::MAX));
        let checkpoint_version = self
            .checkpoint_versions
            .get(object_id)
            .copied()
            .unwrap_or(0)
            .saturating_add(1);
        let state = json!({
            "object_id": object_id,
            "canonical_order": canonical_order,
            "iterations": iterations,
            "adaptations": adaptations,
            "provider_event_id": admission.provider_result.as_ref().map(|event| &event.event_id),
            "replay_cursor": replay_cursor,
            "checkpoint_version": checkpoint_version,
        });
        let state_bytes =
            serde_jcs::to_vec(&state).map_err(|_| ReasoningRuntimeError::InvalidProviderCapture)?;
        let checkpoint = ReasoningCheckpoint {
            schema: REASONING_RUNTIME_CHECKPOINT_SCHEMA.to_string(),
            object_id: object_id.to_string(),
            lineage_id: admission
                .checkpoint
                .as_ref()
                .map(|checkpoint| checkpoint.lineage_id.clone())
                .unwrap_or_else(|| format!("lineage:{object_id}")),
            checkpoint_version,
            replay_cursor,
            state_fingerprint: sha256_hex(&state_bytes),
        };
        self.checkpoint_versions
            .insert(object_id.to_string(), checkpoint_version);
        self.replay_cursors
            .insert(object_id.to_string(), replay_cursor);

        let mut lifelog = Vec::new();
        for (event, reason_code) in [
            ("admitted", "typed_scheduler_admission"),
            ("completed", "governed_reasoning_completed"),
        ] {
            self.lifelog_sequence = self.lifelog_sequence.saturating_add(1);
            lifelog.push(ReasoningLifelogEvent {
                schema: REASONING_RUNTIME_LIFELOG_SCHEMA.to_string(),
                sequence: self.lifelog_sequence,
                object_id: object_id.to_string(),
                event: event.to_string(),
                reason_code: reason_code.to_string(),
            });
        }
        let telemetry = ReasoningTelemetry {
            schema: REASONING_RUNTIME_TELEMETRY_SCHEMA.to_string(),
            component: REASONING_RUNTIME_COMPONENT.to_string(),
            object_id_hash: sha256_hex(object_id.as_bytes()),
            outcome: "completed".to_string(),
            nodes_executed: u64::try_from(canonical_order.len()).unwrap_or(u64::MAX),
            iterations,
            adaptations,
            provider_payload_retained: false,
            redacted: true,
        };
        let fingerprint_input = json!({
            "admission_id": admission.admission_id,
            "object_id": object_id,
            "canonical_order": canonical_order,
            "iterations": iterations,
            "adaptations": adaptations,
            "provider_event_id": admission.provider_result.as_ref().map(|event| &event.event_id),
            "checkpoint": checkpoint,
            "aee_request": aee_request,
        });
        let fingerprint_bytes = serde_jcs::to_vec(&fingerprint_input)
            .map_err(|_| ReasoningRuntimeError::InvalidProviderCapture)?;
        Ok(ReasoningExecution {
            schema: REASONING_RUNTIME_SCHEMA.to_string(),
            admission_id: admission.admission_id.clone(),
            object_id: object_id.to_string(),
            canonical_order,
            iterations,
            adaptations,
            provider_event_id: admission
                .provider_result
                .as_ref()
                .map(|event| event.event_id.clone()),
            aee_request,
            checkpoint,
            lifelog,
            telemetry,
            execution_fingerprint: sha256_hex(&fingerprint_bytes),
        })
    }

    fn validate_checkpoint(
        &self,
        object_id: &str,
        checkpoint: Option<&ReasoningCheckpoint>,
    ) -> Result<(), ReasoningRuntimeError> {
        let Some(checkpoint) = checkpoint else {
            return Ok(());
        };
        if checkpoint.object_id != object_id {
            return Err(ReasoningRuntimeError::CheckpointObjectMismatch);
        }
        let expected = self
            .checkpoint_versions
            .get(object_id)
            .copied()
            .unwrap_or(checkpoint.checkpoint_version);
        if checkpoint.checkpoint_version != expected {
            return Err(ReasoningRuntimeError::StaleCheckpoint);
        }
        Ok(())
    }
}

fn canonical_topological_order(
    graph: &ReasoningGraph,
) -> Result<Vec<String>, ReasoningRuntimeError> {
    validate_identifier(&graph.graph_id)?;
    if graph.nodes.is_empty() {
        return Err(ReasoningRuntimeError::EmptyGraph);
    }
    let mut nodes = BTreeMap::new();
    for node in &graph.nodes {
        validate_identifier(&node.id)?;
        if nodes.insert(node.id.clone(), node).is_some() {
            return Err(ReasoningRuntimeError::DuplicateNode);
        }
    }
    let mut dependents = BTreeMap::<String, BTreeSet<String>>::new();
    let mut remaining = BTreeMap::<String, usize>::new();
    for node in nodes.values() {
        let dependencies = node.dependencies.iter().cloned().collect::<BTreeSet<_>>();
        for dependency in &dependencies {
            if !nodes.contains_key(dependency) {
                return Err(ReasoningRuntimeError::MissingDependency);
            }
            dependents
                .entry(dependency.clone())
                .or_default()
                .insert(node.id.clone());
        }
        remaining.insert(node.id.clone(), dependencies.len());
    }
    let mut ready = remaining
        .iter()
        .filter_map(|(id, count)| (*count == 0).then_some(id.clone()))
        .collect::<BTreeSet<_>>();
    let mut order = Vec::with_capacity(nodes.len());
    while let Some(id) = ready.pop_first() {
        order.push(id.clone());
        for dependent in dependents.get(&id).into_iter().flatten() {
            let count = remaining
                .get_mut(dependent)
                .expect("validated dependent must have degree");
            *count -= 1;
            if *count == 0 {
                ready.insert(dependent.clone());
            }
        }
    }
    if order.len() != nodes.len() {
        return Err(ReasoningRuntimeError::CycleDetected);
    }
    Ok(order)
}

fn validate_identifier(value: &str) -> Result<(), ReasoningRuntimeError> {
    if value.trim().is_empty() {
        Err(ReasoningRuntimeError::EmptyIdentifier)
    } else {
        Ok(())
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningHealth {
    Starting,
    Ready,
    Degraded,
    Overloaded,
    Stopped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningRuntimeStatus {
    pub schema: String,
    pub component: String,
    pub health: ReasoningHealth,
    pub accepted: u64,
    pub completed: u64,
    pub quarantined: u64,
    pub rejected_backpressure: u64,
    pub queue_capacity: usize,
    pub reason_code: String,
}

impl ReasoningRuntimeStatus {
    fn starting(capacity: usize) -> Self {
        Self {
            schema: REASONING_RUNTIME_STATUS_SCHEMA.to_string(),
            component: REASONING_RUNTIME_COMPONENT.to_string(),
            health: ReasoningHealth::Starting,
            accepted: 0,
            completed: 0,
            quarantined: 0,
            rejected_backpressure: 0,
            queue_capacity: capacity,
            reason_code: "component_starting".to_string(),
        }
    }

    fn ready(capacity: usize) -> Self {
        let mut status = Self::starting(capacity);
        status.health = ReasoningHealth::Ready;
        status.reason_code = "typed_channel_ready".to_string();
        status
    }
}

struct RuntimeCommand {
    admission: ReasoningAdmission,
    response: oneshot::Sender<Result<ReasoningExecution, ReasoningRuntimeError>>,
}

#[derive(Clone)]
pub struct ReasoningRuntimeHandle {
    sender: mpsc::Sender<RuntimeCommand>,
    status: watch::Receiver<ReasoningRuntimeStatus>,
}

impl ReasoningRuntimeHandle {
    pub fn try_admit(
        &self,
        admission: ReasoningAdmission,
    ) -> Result<
        oneshot::Receiver<Result<ReasoningExecution, ReasoningRuntimeError>>,
        ReasoningRuntimeError,
    > {
        let (response, receiver) = oneshot::channel();
        self.sender
            .try_send(RuntimeCommand {
                admission,
                response,
            })
            .map_err(|error| match error {
                mpsc::error::TrySendError::Full(_) => ReasoningRuntimeError::QueueFull,
                mpsc::error::TrySendError::Closed(_) => ReasoningRuntimeError::ComponentStopped,
            })?;
        Ok(receiver)
    }

    pub fn status(&self) -> ReasoningRuntimeStatus {
        self.status.borrow().clone()
    }
}

pub struct ReasoningRuntimeComponent {
    pub handle: ReasoningRuntimeHandle,
    cancellation: CancellationToken,
    task: JoinHandle<()>,
}

impl ReasoningRuntimeComponent {
    pub fn start(capacity: usize) -> Self {
        assert!(
            capacity > 0,
            "reasoning runtime queue must be bounded and non-zero"
        );
        let (sender, mut receiver) = mpsc::channel::<RuntimeCommand>(capacity);
        let (status_tx, status_rx) = watch::channel(ReasoningRuntimeStatus::ready(capacity));
        let cancellation = CancellationToken::new();
        let component_cancellation = cancellation.clone();
        let task = tokio::spawn(async move {
            let core = Arc::new(Mutex::new(ReasoningCore::default()));
            let mut status = ReasoningRuntimeStatus::ready(capacity);
            loop {
                tokio::select! {
                    _ = component_cancellation.cancelled() => break,
                    command = receiver.recv() => {
                        let Some(command) = command else { break; };
                        status.accepted = status.accepted.saturating_add(1);
                        let result = core.lock()
                            .map_err(|_| ReasoningRuntimeError::ComponentStopped)
                            .and_then(|mut core| core.execute(command.admission));
                        match &result {
                            Ok(_) => {
                                status.completed = status.completed.saturating_add(1);
                                status.health = ReasoningHealth::Ready;
                                status.reason_code = "object_completed".to_string();
                            }
                            Err(_) => {
                                status.quarantined = status.quarantined.saturating_add(1);
                                status.health = ReasoningHealth::Degraded;
                                status.reason_code = "object_quarantined".to_string();
                            }
                        }
                        let _ = status_tx.send(status.clone());
                        let _ = command.response.send(result);
                    }
                }
            }
            status.health = ReasoningHealth::Stopped;
            status.reason_code = "governed_cancellation".to_string();
            let _ = status_tx.send(status);
        });
        Self {
            handle: ReasoningRuntimeHandle {
                sender,
                status: status_rx,
            },
            cancellation,
            task,
        }
    }

    pub async fn shutdown(self) {
        self.cancellation.cancel();
        let _ = self.task.await;
    }
}

pub fn runtime_api_status(status: &ReasoningRuntimeStatus) -> Value {
    serde_json::to_value(status).unwrap_or_else(|_| {
        json!({
            "schema": REASONING_RUNTIME_STATUS_SCHEMA,
            "component": REASONING_RUNTIME_COMPONENT,
            "health": "degraded",
            "reason_code": "status_serialization_failed"
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph() -> ReasoningGraph {
        ReasoningGraph {
            graph_id: "graph-1".to_string(),
            nodes: vec![
                ReasoningNode {
                    id: "input".into(),
                    kind: ReasoningNodeKind::PromptInput,
                    dependencies: vec![],
                },
                ReasoningNode {
                    id: "hypothesis".into(),
                    kind: ReasoningNodeKind::Hypothesis,
                    dependencies: vec!["input".into()],
                },
                ReasoningNode {
                    id: "evidence".into(),
                    kind: ReasoningNodeKind::Evidence,
                    dependencies: vec!["input".into()],
                },
                ReasoningNode {
                    id: "decision".into(),
                    kind: ReasoningNodeKind::Decision,
                    dependencies: vec!["evidence".into(), "hypothesis".into()],
                },
                ReasoningNode {
                    id: "outcome".into(),
                    kind: ReasoningNodeKind::Outcome,
                    dependencies: vec!["decision".into()],
                },
            ],
        }
    }

    fn admission(object: ReasoningObject) -> ReasoningAdmission {
        ReasoningAdmission {
            admission_id: "admission-1".into(),
            object,
            governance: GovernanceContext {
                freedom_gate: FreedomGateDisposition::Approved,
                aee_available: true,
                policy_ref: "policy/freedom-gate/approved".into(),
            },
            checkpoint: None,
            provider_result: None,
            replay_only: false,
        }
    }

    #[test]
    fn graph_order_is_canonical_and_governed_before_aee() {
        let mut reversed = graph();
        reversed.nodes.reverse();
        let first = ReasoningCore::default()
            .execute(admission(ReasoningObject::Graph(graph())))
            .unwrap();
        let second = ReasoningCore::default()
            .execute(admission(ReasoningObject::Graph(reversed)))
            .unwrap();
        assert_eq!(first.canonical_order, second.canonical_order);
        assert_eq!(first.execution_fingerprint, second.execution_fingerprint);
        assert_eq!(first.aee_request.unwrap().decision_node_id, "decision");
        assert_ne!(first.checkpoint.schema, first.lifelog[0].schema);
        assert!(first.telemetry.redacted);
        assert!(!first.telemetry.provider_payload_retained);
    }

    #[test]
    fn bounded_loop_and_checkpoint_restore_advance_continuity() {
        let object = ReasoningObject::Loop(BoundedLoop {
            loop_id: "loop-1".into(),
            graph: graph(),
            max_iterations: 3,
            exit_node_id: "outcome".into(),
        });
        let mut core = ReasoningCore::default();
        let first = core.execute(admission(object.clone())).unwrap();
        let mut resumed = admission(object);
        resumed.admission_id = "admission-2".into();
        resumed.checkpoint = Some(first.checkpoint.clone());
        let second = core.execute(resumed).unwrap();
        assert_eq!(second.iterations, 3);
        assert_eq!(second.checkpoint.checkpoint_version, 2);
        assert!(second.checkpoint.replay_cursor > first.checkpoint.replay_cursor);
        assert_eq!(second.checkpoint.lineage_id, first.checkpoint.lineage_id);
    }

    #[test]
    fn adaptive_dag_replays_without_provider_access() {
        let proposed = ReasoningNode {
            id: "provider-observation".into(),
            kind: ReasoningNodeKind::ProviderObservation,
            dependencies: vec!["evidence".into()],
        };
        let capture = CapturedProviderResult::capture(
            "provider-event-1",
            "provider_model_io",
            "evidence/provider-event-1.json",
            &json!({"private": "never copied into telemetry"}),
            vec![proposed],
        )
        .unwrap();
        let object = ReasoningObject::AdaptiveDag(AdaptiveDag {
            graph: graph(),
            max_adaptations: 1,
        });
        let mut live = admission(object.clone());
        live.provider_result = Some(capture.clone());
        let mut replay = admission(object);
        replay.provider_result = Some(capture);
        replay.replay_only = true;
        let live_result = ReasoningCore::default().execute(live).unwrap();
        let replay_result = ReasoningCore::default().execute(replay).unwrap();
        assert_eq!(
            live_result.execution_fingerprint,
            replay_result.execution_fingerprint
        );
        assert_eq!(replay_result.adaptations, 1);
    }

    #[test]
    fn malformed_cycle_runaway_and_gate_fail_closed_are_quarantined() {
        let mut core = ReasoningCore::default();
        let mut cyclic = graph();
        cyclic.nodes[0].dependencies.push("outcome".into());
        assert_eq!(
            core.execute(admission(ReasoningObject::Graph(cyclic))),
            Err(ReasoningRuntimeError::CycleDetected)
        );

        let runaway = ReasoningObject::Loop(BoundedLoop {
            loop_id: "loop-0".into(),
            graph: graph(),
            max_iterations: 0,
            exit_node_id: "outcome".into(),
        });
        assert_eq!(
            core.execute(admission(runaway)),
            Err(ReasoningRuntimeError::InvalidLoopLimit)
        );

        let mut denied = admission(ReasoningObject::Graph(graph()));
        denied.governance.freedom_gate = FreedomGateDisposition::Denied;
        assert_eq!(
            core.execute(denied),
            Err(ReasoningRuntimeError::FreedomGateDenied)
        );
        assert_eq!(core.quarantined().len(), 2);
        assert!(core
            .quarantined()
            .iter()
            .all(|item| item.input_evidence_preserved));
    }

    #[tokio::test]
    async fn component_exposes_lifecycle_health_and_typed_admission() {
        let component = ReasoningRuntimeComponent::start(1);
        assert_eq!(component.handle.status().health, ReasoningHealth::Ready);
        let response = component
            .handle
            .try_admit(admission(ReasoningObject::Graph(graph())))
            .unwrap();
        let execution = response.await.unwrap().unwrap();
        assert_eq!(execution.object_id, "graph-1");
        assert_eq!(component.handle.status().completed, 1);
        component.shutdown().await;
    }
}
