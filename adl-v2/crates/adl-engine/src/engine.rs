use crate::model::{
    CancelRequest, CompletionOutcome, CompletionReceipt, EngineEffect, EngineError,
    EngineErrorCode, EngineEvent, EngineLimits, EnginePolicy, EngineSnapshot, EventKind,
    FailureClass, JoinPolicy, NodeSnapshot, NodeState, PortCompletion, PortFailure, PortKind,
    PortOutput, ProviderRequest, ToolRequest, TurnInput, TurnOutput, CHECKPOINT_CONTRACT_VERSION,
    ENGINE_CONTRACT_VERSION,
};
use adl_compiler::{ExecutionPlan, PlanEdgeKind, PlanNode, EXECUTION_PLAN_VERSION};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::io::{Error as IoError, Result as IoResult, Write};

const PLAN_DIGEST_DOMAIN: &[u8] = b"adl.engine.plan.v1\0";
const POLICY_DIGEST_DOMAIN: &[u8] = b"adl.engine.policy.v1\0";
const EDGE_DIGEST_DOMAIN: &[u8] = b"adl.engine.edge.v1\0";
const REQUEST_ID_DOMAIN: &[u8] = b"adl.engine.request.v1\0";
const IDEMPOTENCY_DOMAIN: &[u8] = b"adl.engine.idempotency.v1\0";
const CANCEL_IDEMPOTENCY_DOMAIN: &[u8] = b"adl.engine.cancel.v1\0";
const COMPLETION_DIGEST_DOMAIN: &[u8] = b"adl.engine.completion.v1\0";
const INPUT_DIGEST_DOMAIN: &[u8] = b"adl.engine.input.v1\0";

#[derive(Debug, Clone)]
pub struct Engine {
    plan: ExecutionPlan,
    policy: EnginePolicy,
    predecessors: BTreeMap<String, Vec<String>>,
    state_bindings: BTreeMap<String, BTreeMap<String, String>>,
    nodes: BTreeMap<String, PlanNode>,
    snapshot: EngineSnapshot,
}

impl Engine {
    pub fn new(
        plan: ExecutionPlan,
        policy: EnginePolicy,
        limits: EngineLimits,
    ) -> Result<Self, EngineError> {
        validate_limits(&limits)?;
        preflight_plan(&plan, limits.max_plan_bytes)?;
        preflight_policy(&policy, limits.max_policy_bytes)?;
        let plan_bytes = encode_bounded(&plan, limits.max_plan_bytes, "plan")?;
        let policy_bytes = encode_bounded(&policy, limits.max_policy_bytes, "policy")?;
        let (nodes, predecessors, state_bindings, node_ids, edge_ids) =
            validate_plan(&plan, &limits)?;
        validate_policy(&plan, &policy, &predecessors, &limits)?;

        let plan_digest = hash_parts(PLAN_DIGEST_DOMAIN, &[&plan_bytes]);
        let policy_digest = hash_parts(POLICY_DIGEST_DOMAIN, &[&policy_bytes]);

        validate_request_envelopes(&plan, &policy, &nodes, &plan_digest, &limits)?;

        let mut node_states = BTreeMap::new();
        for node_id in &node_ids {
            node_states.insert(
                node_id.clone(),
                NodeSnapshot {
                    state: NodeState::Pending,
                    attempts: 0,
                },
            );
        }
        let snapshot = EngineSnapshot {
            checkpoint_contract: String::from(CHECKPOINT_CONTRACT_VERSION),
            engine_contract: String::from(ENGINE_CONTRACT_VERSION),
            plan_contract: plan.contract.clone(),
            plan_source_digest: plan.source_digest.clone(),
            plan_digest,
            policy_digest,
            node_ids,
            edge_ids,
            limits,
            logical_tick: 0,
            logical_turns: 0,
            attempts_consumed: 0,
            output_bytes: 0,
            event_count: 0,
            next_event_sequence: 0,
            next_request_sequence: 0,
            nodes: node_states,
            turn_journal: Vec::new(),
            consumed_completion_digests: BTreeMap::new(),
        };
        ensure_snapshot_bound(&snapshot)?;
        Ok(Self {
            plan,
            policy,
            predecessors,
            state_bindings,
            nodes,
            snapshot,
        })
    }

    pub fn resume(
        plan: ExecutionPlan,
        policy: EnginePolicy,
        limits: EngineLimits,
        checkpoint: &[u8],
    ) -> Result<Self, EngineError> {
        if count_u64(checkpoint.len(), "checkpoint")? > limits.max_checkpoint_bytes {
            return Err(EngineError::new(
                EngineErrorCode::ResourceLimit,
                "checkpoint",
                "checkpoint byte limit exceeded",
            ));
        }
        let expected = Self::new(plan, policy, limits)?;
        let snapshot: EngineSnapshot = serde_json::from_slice(checkpoint).map_err(|error| {
            EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint",
                &error.to_string(),
            )
        })?;
        let canonical = encode(&snapshot, "checkpoint")?;
        if canonical != checkpoint {
            return Err(EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint",
                "checkpoint encoding is not canonical",
            ));
        }
        validate_resumed_snapshot(&snapshot, &expected)?;
        let mut resumed = expected;
        resumed.snapshot = snapshot;
        Ok(resumed)
    }

    pub fn snapshot(&self) -> &EngineSnapshot {
        &self.snapshot
    }

    pub fn is_quiescent(&self) -> bool {
        self.snapshot
            .nodes
            .values()
            .all(|node| !node.state.is_in_flight())
    }

    pub fn is_terminal(&self) -> bool {
        self.snapshot
            .nodes
            .values()
            .all(|node| node.state.is_terminal())
    }

    pub fn checkpoint(&self) -> Result<Vec<u8>, EngineError> {
        if !self.is_quiescent() {
            return Err(EngineError::new(
                EngineErrorCode::CheckpointNotQuiescent,
                "checkpoint",
                "checkpoint requires a quiescent engine",
            ));
        }
        encode_bounded(
            &self.snapshot,
            self.snapshot.limits.max_checkpoint_bytes,
            "checkpoint",
        )
    }

    pub fn turn(&mut self, mut input: TurnInput) -> Result<TurnOutput, EngineError> {
        validate_turn_input(&input, &self.snapshot.limits)?;
        if input.logical_tick <= self.snapshot.logical_tick {
            return Err(EngineError::new(
                EngineErrorCode::Protocol,
                "turn.logical_tick",
                "logical tick must increase",
            ));
        }
        if self.snapshot.logical_turns >= self.snapshot.limits.max_logical_turns {
            return Err(EngineError::new(
                EngineErrorCode::ResourceLimit,
                "turn",
                "logical turn limit exhausted",
            ));
        }

        let mut working = self.snapshot.clone();
        working.logical_tick = input.logical_tick;
        working.logical_turns += 1;
        let mut effects = Vec::new();
        let mut events = Vec::new();
        let mut completed_nodes = BTreeSet::new();

        input
            .completions
            .sort_by(|left, right| left.request_id().cmp(right.request_id()));
        for completion in &input.completions {
            if self.apply_completion(&mut working, completion, &mut events, &mut completed_nodes)? {
                completed_nodes.insert(String::from(completion.identity().0));
            }
        }

        input.cancellations.sort();
        input.cancellations.dedup();
        for node_id in &input.cancellations {
            if completed_nodes.contains(node_id) {
                continue;
            }
            self.apply_cancellation(&mut working, node_id, &mut effects, &mut events)?;
        }

        self.promote_ready(&mut working, &mut events)?;
        self.dispatch(&mut working, &mut effects, &mut events)?;
        working.turn_journal.push(input);
        ensure_snapshot_bound(&working)?;

        self.snapshot = working.clone();
        Ok(TurnOutput {
            snapshot: working,
            effects,
            events,
        })
    }

    fn apply_completion(
        &self,
        snapshot: &mut EngineSnapshot,
        completion: &PortCompletion,
        events: &mut Vec<EngineEvent>,
        completed_nodes: &mut BTreeSet<String>,
    ) -> Result<bool, EngineError> {
        let encoded = encode_bounded(
            completion,
            snapshot.limits.max_completion_bytes,
            "completion",
        )?;
        let digest = hash_parts(COMPLETION_DIGEST_DOMAIN, &[&encoded]);
        let request_id = completion.request_id();
        if let Some(previous) = snapshot.consumed_completion_digests.get(request_id) {
            if previous.completion_digest == digest {
                return Ok(false);
            }
            return Err(EngineError::new(
                EngineErrorCode::Protocol,
                "completion.request_id",
                "non-identical duplicate completion",
            ));
        }

        let mut active_node = None;
        for (node_id, node) in &snapshot.nodes {
            let active = match &node.state {
                NodeState::Dispatched {
                    request_id: active, ..
                }
                | NodeState::Cancelling {
                    request_id: active, ..
                } => active == request_id,
                NodeState::Pending
                | NodeState::Ready
                | NodeState::RetryWait { .. }
                | NodeState::Succeeded { .. }
                | NodeState::Failed { .. }
                | NodeState::Cancelled => false,
            };
            if active {
                active_node = Some(node_id.clone());
                break;
            }
        }
        let node_id = active_node.ok_or_else(|| {
            EngineError::new(
                EngineErrorCode::Protocol,
                "completion.request_id",
                "unknown completion request identity",
            )
        })?;
        let (declared_node, declared_attempt) = completion.identity();
        if declared_node != node_id {
            return Err(EngineError::new(
                EngineErrorCode::Protocol,
                "completion.node_id",
                "completion node identity mismatch",
            ));
        }

        let current = snapshot.nodes.get(&node_id).ok_or_else(|| {
            EngineError::new(
                EngineErrorCode::InvalidPlan,
                "snapshot.nodes",
                "active node is absent",
            )
        })?;
        let (active_attempt, sequence, input_digest, cancelling) = match &current.state {
            NodeState::Dispatched {
                attempt,
                sequence,
                input_digest,
                ..
            } => (*attempt, *sequence, input_digest.clone(), false),
            NodeState::Cancelling {
                attempt,
                sequence,
                input_digest,
                ..
            } => (*attempt, *sequence, input_digest.clone(), true),
            NodeState::Pending
            | NodeState::Ready
            | NodeState::RetryWait { .. }
            | NodeState::Succeeded { .. }
            | NodeState::Failed { .. }
            | NodeState::Cancelled => {
                return Err(EngineError::new(
                    EngineErrorCode::Protocol,
                    "completion",
                    "completion targets a non-active node",
                ));
            }
        };
        if declared_attempt != active_attempt || current.attempts != active_attempt {
            return Err(EngineError::new(
                EngineErrorCode::Protocol,
                "completion.attempt",
                "completion attempt mismatch",
            ));
        }

        let node_policy = self.policy.nodes.get(&node_id).ok_or_else(|| {
            EngineError::new(
                EngineErrorCode::InvalidPolicy,
                "policy.nodes",
                "node policy is absent",
            )
        })?;
        let outcome = match completion {
            PortCompletion::Provider(value) => {
                if node_policy.port != PortKind::Provider {
                    return Err(EngineError::new(
                        EngineErrorCode::Protocol,
                        "completion",
                        "provider completion targets a tool request",
                    ));
                }
                Some(value.outcome.clone())
            }
            PortCompletion::Tool(value) => {
                match &node_policy.port {
                    PortKind::Tool { .. } => {}
                    PortKind::Provider => {
                        return Err(EngineError::new(
                            EngineErrorCode::Protocol,
                            "completion",
                            "tool completion targets a provider request",
                        ));
                    }
                }
                Some(value.outcome.clone())
            }
            PortCompletion::Cancel(value) => {
                if !cancelling {
                    return Err(EngineError::new(
                        EngineErrorCode::Protocol,
                        "completion",
                        "cancel acknowledgement targets a dispatched request",
                    ));
                }
                if !value.acknowledged {
                    return Err(EngineError::new(
                        EngineErrorCode::Protocol,
                        "completion.acknowledged",
                        "cancel acknowledgement was rejected",
                    ));
                }
                None
            }
        };

        snapshot.consumed_completion_digests.insert(
            String::from(request_id),
            CompletionReceipt {
                node_id: node_id.clone(),
                attempt: active_attempt,
                sequence,
                input_digest,
                completed_at_tick: snapshot.logical_tick,
                completion: completion.clone(),
                completion_digest: digest,
            },
        );
        if let Some(value) = outcome {
            self.apply_outcome(snapshot, &node_id, value, events)?;
        } else {
            let node = snapshot.nodes.get_mut(&node_id).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "snapshot.nodes",
                    "cancelled node is absent",
                )
            })?;
            node.state = NodeState::Cancelled;
            emit(
                snapshot,
                events,
                Some(node_id.clone()),
                EventKind::NodeCancelled,
            )?;
        }
        completed_nodes.insert(node_id);
        Ok(true)
    }

    fn apply_outcome(
        &self,
        snapshot: &mut EngineSnapshot,
        node_id: &str,
        outcome: CompletionOutcome,
        events: &mut Vec<EngineEvent>,
    ) -> Result<(), EngineError> {
        match outcome {
            CompletionOutcome::Success(output) => {
                let new_total = snapshot
                    .output_bytes
                    .checked_add(count_u64(output.bytes.len(), "completion.output")?)
                    .ok_or_else(|| {
                        EngineError::new(
                            EngineErrorCode::ResourceLimit,
                            "completion.output",
                            "output byte accounting overflow",
                        )
                    })?;
                if new_total > snapshot.limits.max_output_bytes {
                    return Err(EngineError::new(
                        EngineErrorCode::ResourceLimit,
                        "completion.output",
                        "retained output byte limit exceeded",
                    ));
                }
                snapshot.output_bytes = new_total;
                let node = snapshot.nodes.get_mut(node_id).ok_or_else(|| {
                    EngineError::new(
                        EngineErrorCode::InvalidPlan,
                        "snapshot.nodes",
                        "completed node is absent",
                    )
                })?;
                node.state = NodeState::Succeeded { output };
                emit(
                    snapshot,
                    events,
                    Some(String::from(node_id)),
                    EventKind::NodeSucceeded,
                )?;
            }
            CompletionOutcome::Failure(failure) => {
                let node_policy = self.policy.nodes.get(node_id).ok_or_else(|| {
                    EngineError::new(
                        EngineErrorCode::InvalidPolicy,
                        "policy.nodes",
                        "node policy is absent",
                    )
                })?;
                let attempts = snapshot
                    .nodes
                    .get(node_id)
                    .ok_or_else(|| {
                        EngineError::new(
                            EngineErrorCode::InvalidPlan,
                            "snapshot.nodes",
                            "failed node is absent",
                        )
                    })?
                    .attempts;
                if node_policy.retry.retryable.contains(&failure.class)
                    && attempts < node_policy.retry.max_attempts
                {
                    let index = usize::try_from(attempts - 1).map_err(|error| {
                        EngineError::new(
                            EngineErrorCode::ResourceLimit,
                            "policy.retry.delay_ticks",
                            &error.to_string(),
                        )
                    })?;
                    let delay = node_policy.retry.delay_ticks[index];
                    let ready_at_tick =
                        snapshot.logical_tick.checked_add(delay).ok_or_else(|| {
                            EngineError::new(
                                EngineErrorCode::ResourceLimit,
                                "policy.retry.delay_ticks",
                                "retry logical tick overflow",
                            )
                        })?;
                    let node = snapshot.nodes.get_mut(node_id).ok_or_else(|| {
                        EngineError::new(
                            EngineErrorCode::InvalidPlan,
                            "snapshot.nodes",
                            "retry node is absent",
                        )
                    })?;
                    node.state = NodeState::RetryWait { ready_at_tick };
                    emit(
                        snapshot,
                        events,
                        Some(String::from(node_id)),
                        EventKind::RetryScheduled { ready_at_tick },
                    )?;
                } else {
                    let terminal = if node_policy.retry.retryable.contains(&failure.class) {
                        PortFailure::new(FailureClass::RetryExhausted, "retry attempts exhausted")
                    } else {
                        failure
                    };
                    fail_node(snapshot, events, node_id, terminal)?;
                }
            }
        }
        Ok(())
    }

    fn apply_cancellation(
        &self,
        snapshot: &mut EngineSnapshot,
        node_id: &str,
        effects: &mut Vec<EngineEffect>,
        events: &mut Vec<EngineEvent>,
    ) -> Result<(), EngineError> {
        let current = snapshot.nodes.get(node_id).ok_or_else(|| {
            EngineError::new(
                EngineErrorCode::Protocol,
                "turn.cancellations",
                "cancellation targets an unknown node",
            )
        })?;
        match current.state.clone() {
            NodeState::Pending | NodeState::Ready | NodeState::RetryWait { .. } => {
                let node = snapshot.nodes.get_mut(node_id).ok_or_else(|| {
                    EngineError::new(
                        EngineErrorCode::InvalidPlan,
                        "snapshot.nodes",
                        "cancelled node is absent",
                    )
                })?;
                node.state = NodeState::Cancelled;
                emit(
                    snapshot,
                    events,
                    Some(String::from(node_id)),
                    EventKind::NodeCancelled,
                )?;
            }
            NodeState::Dispatched {
                request_id,
                attempt,
                sequence,
                input_digest,
            } => {
                let cancel_key = hash_parts(
                    CANCEL_IDEMPOTENCY_DOMAIN,
                    &[request_id.as_bytes(), node_id.as_bytes()],
                );
                let node = snapshot.nodes.get_mut(node_id).ok_or_else(|| {
                    EngineError::new(
                        EngineErrorCode::InvalidPlan,
                        "snapshot.nodes",
                        "cancelling node is absent",
                    )
                })?;
                node.state = NodeState::Cancelling {
                    request_id: request_id.clone(),
                    attempt,
                    sequence,
                    input_digest,
                };
                effects.push(EngineEffect::Cancel(CancelRequest {
                    request_id: request_id.clone(),
                    idempotency_key: cancel_key,
                    node_id: String::from(node_id),
                    attempt,
                }));
                emit(
                    snapshot,
                    events,
                    Some(String::from(node_id)),
                    EventKind::CancellationRequested { request_id },
                )?;
            }
            NodeState::Cancelling { .. }
            | NodeState::Succeeded { .. }
            | NodeState::Failed { .. }
            | NodeState::Cancelled => {}
        }
        Ok(())
    }

    fn promote_ready(
        &self,
        snapshot: &mut EngineSnapshot,
        events: &mut Vec<EngineEvent>,
    ) -> Result<(), EngineError> {
        for node in snapshot.nodes.values_mut() {
            if let NodeState::RetryWait { ready_at_tick } = node.state {
                if ready_at_tick <= snapshot.logical_tick {
                    node.state = NodeState::Pending;
                }
            }
        }

        loop {
            let mut failures = Vec::new();
            for node_id in &snapshot.node_ids {
                let node = snapshot.nodes.get(node_id).ok_or_else(|| {
                    EngineError::new(
                        EngineErrorCode::InvalidPlan,
                        "snapshot.nodes",
                        "planned node is absent",
                    )
                })?;
                if node.state != NodeState::Pending {
                    continue;
                }
                let predecessors = self.predecessors.get(node_id).ok_or_else(|| {
                    EngineError::new(
                        EngineErrorCode::InvalidPlan,
                        "plan.edges",
                        "predecessor set is absent",
                    )
                })?;
                if dependency_decision(
                    snapshot,
                    predecessors,
                    &self.policy.nodes[node_id],
                    &self.state_bindings[node_id],
                ) == DependencyDecision::Fail
                {
                    failures.push(node_id.clone());
                }
            }
            if failures.is_empty() {
                break;
            }
            for node_id in failures {
                fail_node(
                    snapshot,
                    events,
                    &node_id,
                    PortFailure::new(FailureClass::Dependency, "join condition became impossible"),
                )?;
            }
        }

        let mut eligible = Vec::new();
        for node_id in &snapshot.node_ids {
            let node = snapshot.nodes.get(node_id).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "snapshot.nodes",
                    "planned node is absent",
                )
            })?;
            if node.state != NodeState::Pending {
                continue;
            }
            let predecessors = self.predecessors.get(node_id).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "plan.edges",
                    "predecessor set is absent",
                )
            })?;
            if dependency_decision(
                snapshot,
                predecessors,
                &self.policy.nodes[node_id],
                &self.state_bindings[node_id],
            ) == DependencyDecision::Ready
            {
                eligible.push(node_id.clone());
            }
        }
        eligible.sort();
        let ready_count = snapshot
            .nodes
            .values()
            .filter(|node| node.state == NodeState::Ready)
            .count();
        let ready_limit = limit_usize(snapshot.limits.max_ready_nodes, "limits.max_ready_nodes")?;
        let capacity = ready_limit.saturating_sub(ready_count);
        let promote_count = capacity.min(eligible.len());
        for node_id in eligible.iter().take(promote_count) {
            let node = snapshot.nodes.get_mut(node_id).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "snapshot.nodes",
                    "ready node is absent",
                )
            })?;
            node.state = NodeState::Ready;
            emit(
                snapshot,
                events,
                Some(node_id.clone()),
                EventKind::NodeReady,
            )?;
        }
        if eligible.len() > promote_count {
            emit(
                snapshot,
                events,
                None,
                EventKind::Backpressure {
                    queued: count_u64(
                        eligible.len() - promote_count,
                        "events.backpressure.queued",
                    )?,
                },
            )?;
        }
        Ok(())
    }

    fn dispatch(
        &self,
        snapshot: &mut EngineSnapshot,
        effects: &mut Vec<EngineEffect>,
        events: &mut Vec<EngineEvent>,
    ) -> Result<(), EngineError> {
        let in_flight = snapshot
            .nodes
            .values()
            .filter(|node| node.state.is_in_flight())
            .count();
        let in_flight_limit = limit_usize(snapshot.limits.max_in_flight, "limits.max_in_flight")?;
        let mut available = in_flight_limit.saturating_sub(in_flight);
        let ready = snapshot
            .nodes
            .iter()
            .filter_map(|(node_id, node)| {
                if node.state == NodeState::Ready {
                    Some(node_id.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if ready.len() > available {
            emit(
                snapshot,
                events,
                None,
                EventKind::Backpressure {
                    queued: count_u64(ready.len() - available, "events.backpressure.queued")?,
                },
            )?;
        }
        for node_id in ready {
            if available == 0 {
                break;
            }
            let attempts = snapshot.nodes[&node_id].attempts;
            if attempts >= snapshot.limits.max_attempts_per_node
                || snapshot.attempts_consumed >= snapshot.limits.max_total_attempts
            {
                fail_node(
                    snapshot,
                    events,
                    &node_id,
                    PortFailure::new(FailureClass::RetryExhausted, "attempt budget exhausted"),
                )?;
                continue;
            }
            let attempt = attempts + 1;
            let sequence = snapshot.next_request_sequence;
            let node = self.nodes.get(&node_id).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "plan.nodes",
                    "dispatch node is absent",
                )
            })?;
            let node_policy = self.policy.nodes.get(&node_id).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPolicy,
                    "policy.nodes",
                    "dispatch policy is absent",
                )
            })?;
            let resolved_inputs = self.resolve_inputs(snapshot, node)?;
            let input_bytes = encode_bounded(
                &resolved_inputs,
                snapshot.limits.max_request_bytes,
                "effect.inputs",
            )?;
            let input_digest = hash_parts(INPUT_DIGEST_DOMAIN, &[&input_bytes]);
            let effect = make_effect(
                &self.plan,
                node,
                node_policy,
                &snapshot.plan_digest,
                DispatchInput {
                    attempt,
                    sequence,
                    inputs: &resolved_inputs,
                    input_digest: &input_digest,
                },
            );
            encode_bounded(&effect, snapshot.limits.max_request_bytes, "effect")?;
            let request_id = effect_request_id(&effect);
            let state = snapshot.nodes.get_mut(&node_id).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "snapshot.nodes",
                    "dispatch state is absent",
                )
            })?;
            state.attempts = attempt;
            state.state = NodeState::Dispatched {
                request_id: request_id.clone(),
                attempt,
                sequence,
                input_digest,
            };
            snapshot.attempts_consumed += 1;
            snapshot.next_request_sequence += 1;
            effects.push(effect);
            emit(
                snapshot,
                events,
                Some(node_id),
                EventKind::RequestDispatched {
                    request_id,
                    attempt,
                },
            )?;
            available -= 1;
        }
        Ok(())
    }

    fn resolve_inputs(
        &self,
        snapshot: &EngineSnapshot,
        node: &PlanNode,
    ) -> Result<BTreeMap<String, Value>, EngineError> {
        let bindings = self.state_bindings.get(&node.id).ok_or_else(|| {
            EngineError::new(
                EngineErrorCode::InvalidPlan,
                "plan.edges",
                "state binding set is absent",
            )
        })?;
        let maximum = snapshot.limits.max_request_bytes;
        let mut materialized = 1_u64;
        let mut remaining = BTreeMap::new();
        for value in node.inputs.values() {
            collect_state_reference_counts(value, &mut remaining)?;
        }
        let mut cache = BTreeMap::new();
        let mut resolved = BTreeMap::new();
        for (name, value) in &node.inputs {
            add_text(&mut materialized, name, maximum, "effect.inputs")?;
            let value = resolve_state_value(
                value,
                bindings,
                snapshot,
                &mut cache,
                &mut remaining,
                &mut materialized,
                maximum,
            )?;
            resolved.insert(name.clone(), value);
        }
        Ok(resolved)
    }
}

fn resolve_state_value(
    value: &Value,
    bindings: &BTreeMap<String, String>,
    snapshot: &EngineSnapshot,
    cache: &mut BTreeMap<String, Value>,
    remaining: &mut BTreeMap<String, u64>,
    materialized: &mut u64,
    maximum: u64,
) -> Result<Value, EngineError> {
    match value {
        Value::String(text) => {
            let Some(state) = text.strip_prefix("@state:") else {
                add_text(materialized, text, maximum, "effect.inputs")?;
                return Ok(value.clone());
            };
            let source = bindings.get(state).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "plan.nodes.inputs",
                    "state reference has no typed dependency edge",
                )
            })?;
            let node = snapshot.nodes.get(source).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "snapshot.nodes",
                    "state source is absent",
                )
            })?;
            match &node.state {
                NodeState::Succeeded { output } => {
                    if !cache.contains_key(state) {
                        add_size(
                            materialized,
                            count_u64(output.bytes.len(), "effect.inputs")?,
                            maximum,
                            "effect.inputs",
                        )?;
                        cache.insert(state.to_owned(), state_output_value(output)?);
                    }
                    let uses = remaining.get_mut(state).ok_or_else(|| {
                        EngineError::new(
                            EngineErrorCode::InvalidPlan,
                            "plan.nodes.inputs",
                            "state reference count is absent",
                        )
                    })?;
                    *uses = uses.checked_sub(1).ok_or_else(|| {
                        EngineError::new(
                            EngineErrorCode::InvalidPlan,
                            "plan.nodes.inputs",
                            "state reference count underflow",
                        )
                    })?;
                    if *uses == 0 {
                        cache.remove(state).ok_or_else(|| {
                            EngineError::new(
                                EngineErrorCode::Protocol,
                                "completion.output",
                                "parsed state output cache is absent",
                            )
                        })
                    } else {
                        let cached = cache.get(state).ok_or_else(|| {
                            EngineError::new(
                                EngineErrorCode::Protocol,
                                "completion.output",
                                "parsed state output cache is absent",
                            )
                        })?;
                        add_value(materialized, cached, maximum, "effect.inputs")?;
                        Ok(cached.clone())
                    }
                }
                NodeState::Pending
                | NodeState::Ready
                | NodeState::Dispatched { .. }
                | NodeState::RetryWait { .. }
                | NodeState::Cancelling { .. }
                | NodeState::Failed { .. }
                | NodeState::Cancelled => Err(EngineError::new(
                    EngineErrorCode::Protocol,
                    "snapshot.nodes",
                    "state dependency output is unavailable",
                )),
            }
        }
        Value::Array(values) => {
            add_size(materialized, 1, maximum, "effect.inputs")?;
            let mut resolved = Vec::new();
            for item in values {
                resolved.push(resolve_state_value(
                    item,
                    bindings,
                    snapshot,
                    cache,
                    remaining,
                    materialized,
                    maximum,
                )?);
            }
            Ok(Value::Array(resolved))
        }
        Value::Object(values) => {
            add_size(materialized, 1, maximum, "effect.inputs")?;
            let mut resolved = serde_json::Map::new();
            for (key, item) in values {
                add_text(materialized, key, maximum, "effect.inputs")?;
                resolved.insert(
                    key.clone(),
                    resolve_state_value(
                        item,
                        bindings,
                        snapshot,
                        cache,
                        remaining,
                        materialized,
                        maximum,
                    )?,
                );
            }
            Ok(Value::Object(resolved))
        }
        Value::Null | Value::Bool(_) => {
            add_size(materialized, 1, maximum, "effect.inputs")?;
            Ok(value.clone())
        }
        Value::Number(_) => {
            add_size(materialized, 32, maximum, "effect.inputs")?;
            Ok(value.clone())
        }
    }
}

fn collect_state_reference_counts(
    value: &Value,
    references: &mut BTreeMap<String, u64>,
) -> Result<(), EngineError> {
    match value {
        Value::String(text) => {
            if let Some(reference) = text.strip_prefix("@state:") {
                let count = references.entry(reference.to_owned()).or_default();
                *count = count.checked_add(1).ok_or_else(|| {
                    EngineError::new(
                        EngineErrorCode::ResourceLimit,
                        "plan.nodes.inputs",
                        "state reference count overflow",
                    )
                })?;
            }
        }
        Value::Array(values) => {
            for item in values {
                collect_state_reference_counts(item, references)?;
            }
        }
        Value::Object(values) => {
            for item in values.values() {
                collect_state_reference_counts(item, references)?;
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
    }
    Ok(())
}

fn state_output_value(output: &PortOutput) -> Result<Value, EngineError> {
    let media_type = output
        .media_type
        .split(';')
        .next()
        .unwrap_or_default()
        .trim();
    if media_type == "application/json" {
        serde_json::from_slice(&output.bytes).map_err(|error| {
            EngineError::new(
                EngineErrorCode::Protocol,
                "completion.output",
                &error.to_string(),
            )
        })
    } else if media_type.starts_with("text/") {
        String::from_utf8(output.bytes.clone())
            .map(Value::String)
            .map_err(|error| {
                EngineError::new(
                    EngineErrorCode::Protocol,
                    "completion.output",
                    &error.to_string(),
                )
            })
    } else {
        Err(EngineError::new(
            EngineErrorCode::Protocol,
            "completion.output.media_type",
            "state dependency requires application/json or text/* output",
        ))
    }
}

fn collect_state_references(value: &Value, references: &mut BTreeSet<String>) {
    match value {
        Value::String(text) => {
            if let Some(reference) = text.strip_prefix("@state:") {
                references.insert(reference.to_owned());
            }
        }
        Value::Array(values) => {
            for item in values {
                collect_state_references(item, references);
            }
        }
        Value::Object(values) => {
            for item in values.values() {
                collect_state_references(item, references);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DependencyDecision {
    Wait,
    Ready,
    Fail,
}

fn dependency_decision(
    snapshot: &EngineSnapshot,
    predecessors: &[String],
    policy: &crate::model::NodePolicy,
    state_bindings: &BTreeMap<String, String>,
) -> DependencyDecision {
    for source in state_bindings.values() {
        match snapshot.nodes[source].state {
            NodeState::Succeeded { .. } => {}
            NodeState::Failed { .. } | NodeState::Cancelled => {
                return DependencyDecision::Fail;
            }
            NodeState::Pending
            | NodeState::Ready
            | NodeState::Dispatched { .. }
            | NodeState::RetryWait { .. }
            | NodeState::Cancelling { .. } => return DependencyDecision::Wait,
        }
    }
    if predecessors.is_empty() {
        return DependencyDecision::Ready;
    }
    let mut succeeded = 0_u64;
    let mut terminal = 0_u64;
    let mut failed = 0_u64;
    let predecessor_count = u64::try_from(predecessors.len()).unwrap_or(u64::MAX);
    for predecessor in predecessors {
        let state = &snapshot.nodes[predecessor].state;
        match state {
            NodeState::Succeeded { .. } => {
                succeeded += 1;
                terminal += 1;
            }
            NodeState::Failed { .. } | NodeState::Cancelled => {
                failed += 1;
                terminal += 1;
            }
            NodeState::Pending
            | NodeState::Ready
            | NodeState::Dispatched { .. }
            | NodeState::RetryWait { .. }
            | NodeState::Cancelling { .. } => {}
        }
    }
    match policy.join {
        JoinPolicy::All => {
            if succeeded == predecessor_count {
                DependencyDecision::Ready
            } else if terminal == predecessor_count {
                DependencyDecision::Fail
            } else {
                DependencyDecision::Wait
            }
        }
        JoinPolicy::FailFast => {
            if failed > 0 {
                DependencyDecision::Fail
            } else if succeeded == predecessor_count {
                DependencyDecision::Ready
            } else {
                DependencyDecision::Wait
            }
        }
        JoinPolicy::AtLeast { required } => {
            let possible = succeeded + (predecessor_count - terminal);
            if succeeded >= required {
                DependencyDecision::Ready
            } else if possible < required {
                DependencyDecision::Fail
            } else {
                DependencyDecision::Wait
            }
        }
    }
}

fn validate_limits(limits: &EngineLimits) -> Result<(), EngineError> {
    if limits.max_plan_nodes == 0
        || limits.max_dependency_edges == 0
        || limits.max_plan_bytes == 0
        || limits.max_policy_bytes == 0
        || limits.max_ready_nodes == 0
        || limits.max_in_flight == 0
        || limits.max_total_attempts == 0
        || limits.max_attempts_per_node == 0
        || limits.max_request_bytes == 0
        || limits.max_completion_bytes == 0
        || limits.max_completions_per_turn == 0
        || limits.max_cancellations_per_turn == 0
        || limits.max_turn_input_bytes == 0
        || limits.max_output_bytes == 0
        || limits.max_events == 0
        || limits.max_checkpoint_bytes == 0
        || limits.max_logical_turns == 0
    {
        return Err(EngineError::new(
            EngineErrorCode::InvalidLimits,
            "limits",
            "all engine limits must be nonzero",
        ));
    }
    if limits.max_in_flight > limits.max_ready_nodes
        || limits.max_ready_nodes > limits.max_plan_nodes
        || u64::from(limits.max_attempts_per_node) > limits.max_total_attempts
        || limits.max_completion_bytes > limits.max_turn_input_bytes
    {
        return Err(EngineError::new(
            EngineErrorCode::InvalidLimits,
            "limits",
            "engine limits are contradictory",
        ));
    }
    limit_usize(limits.max_plan_nodes, "limits.max_plan_nodes")?;
    limit_usize(limits.max_dependency_edges, "limits.max_dependency_edges")?;
    limit_usize(limits.max_plan_bytes, "limits.max_plan_bytes")?;
    limit_usize(limits.max_policy_bytes, "limits.max_policy_bytes")?;
    limit_usize(limits.max_ready_nodes, "limits.max_ready_nodes")?;
    limit_usize(limits.max_in_flight, "limits.max_in_flight")?;
    limit_usize(limits.max_request_bytes, "limits.max_request_bytes")?;
    limit_usize(limits.max_completion_bytes, "limits.max_completion_bytes")?;
    limit_usize(
        limits.max_completions_per_turn,
        "limits.max_completions_per_turn",
    )?;
    limit_usize(
        limits.max_cancellations_per_turn,
        "limits.max_cancellations_per_turn",
    )?;
    limit_usize(limits.max_turn_input_bytes, "limits.max_turn_input_bytes")?;
    limit_usize(limits.max_output_bytes, "limits.max_output_bytes")?;
    limit_usize(limits.max_checkpoint_bytes, "limits.max_checkpoint_bytes")?;
    Ok(())
}

type PlanIndex = (
    BTreeMap<String, PlanNode>,
    BTreeMap<String, Vec<String>>,
    BTreeMap<String, BTreeMap<String, String>>,
    Vec<String>,
    Vec<String>,
);

fn validate_plan(plan: &ExecutionPlan, limits: &EngineLimits) -> Result<PlanIndex, EngineError> {
    if plan.contract != EXECUTION_PLAN_VERSION {
        return Err(EngineError::new(
            EngineErrorCode::InvalidPlan,
            "plan.contract",
            "execution plan contract version mismatch",
        ));
    }
    if !is_hex_digest(&plan.source_digest) {
        return Err(EngineError::new(
            EngineErrorCode::InvalidPlan,
            "plan.source_digest",
            "plan source digest is not canonical SHA-256 hex",
        ));
    }
    if plan.nodes.is_empty() || count_u64(plan.nodes.len(), "plan.nodes")? > limits.max_plan_nodes {
        return Err(EngineError::new(
            EngineErrorCode::InvalidPlan,
            "plan.nodes",
            "plan node admission limit violated",
        ));
    }
    if count_u64(plan.edges.len(), "plan.edges")? > limits.max_dependency_edges {
        return Err(EngineError::new(
            EngineErrorCode::InvalidPlan,
            "plan.edges",
            "plan edge admission limit violated",
        ));
    }
    if u64::try_from(plan.nodes.len()).map_err(|error| {
        EngineError::new(
            EngineErrorCode::InvalidLimits,
            "limits.max_total_attempts",
            &error.to_string(),
        )
    })? > limits.max_total_attempts
    {
        return Err(EngineError::new(
            EngineErrorCode::InvalidLimits,
            "limits.max_total_attempts",
            "total attempt limit cannot admit every plan node",
        ));
    }

    let mut nodes = BTreeMap::new();
    for node in &plan.nodes {
        if node.id.is_empty() || nodes.insert(node.id.clone(), node.clone()).is_some() {
            return Err(EngineError::new(
                EngineErrorCode::InvalidPlan,
                "plan.nodes",
                "plan node identities must be nonempty and unique",
            ));
        }
    }
    let node_ids = nodes.keys().cloned().collect::<Vec<_>>();
    let mut predecessor_sets = node_ids
        .iter()
        .map(|node_id| (node_id.clone(), BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    let mut successor_sets = predecessor_sets.clone();
    let mut state_bindings = node_ids
        .iter()
        .map(|node_id| (node_id.clone(), BTreeMap::new()))
        .collect::<BTreeMap<_, _>>();
    let mut edge_encodings = BTreeSet::new();
    let mut edge_ids = Vec::new();
    for edge in &plan.edges {
        if edge.from == edge.to || !nodes.contains_key(&edge.from) || !nodes.contains_key(&edge.to)
        {
            return Err(EngineError::new(
                EngineErrorCode::InvalidPlan,
                "plan.edges",
                "plan edge has an unknown or self-referential endpoint",
            ));
        }
        let encoded = encode(edge, "plan.edges")?;
        if !edge_encodings.insert(encoded.clone()) {
            return Err(EngineError::new(
                EngineErrorCode::InvalidPlan,
                "plan.edges",
                "duplicate plan edge",
            ));
        }
        match edge.kind {
            PlanEdgeKind::Sequential if edge.state.is_some() => {
                return Err(EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "plan.edges.state",
                    "sequential edge cannot carry state identity",
                ));
            }
            PlanEdgeKind::Sequential => {}
            PlanEdgeKind::StateDependency => {
                let state = edge
                    .state
                    .as_deref()
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| {
                        EngineError::new(
                            EngineErrorCode::InvalidPlan,
                            "plan.edges.state",
                            "state dependency requires a state identity",
                        )
                    })?;
                if nodes[&edge.from].save_as.as_deref() != Some(state) {
                    return Err(EngineError::new(
                        EngineErrorCode::InvalidPlan,
                        "plan.edges.state",
                        "state dependency does not match its source output",
                    ));
                }
                let mut references = BTreeSet::new();
                for value in nodes[&edge.to].inputs.values() {
                    collect_state_references(value, &mut references);
                }
                if !references.contains(state) {
                    return Err(EngineError::new(
                        EngineErrorCode::InvalidPlan,
                        "plan.edges.state",
                        "state dependency is not referenced by its target inputs",
                    ));
                }
                let bindings = state_bindings.get_mut(&edge.to).ok_or_else(|| {
                    EngineError::new(
                        EngineErrorCode::InvalidPlan,
                        "plan.edges",
                        "state dependency target is absent",
                    )
                })?;
                if bindings
                    .insert(state.to_owned(), edge.from.clone())
                    .is_some()
                {
                    return Err(EngineError::new(
                        EngineErrorCode::InvalidPlan,
                        "plan.edges.state",
                        "state dependency identity is ambiguous for its target",
                    ));
                }
            }
        }
        edge_ids.push(hash_parts(EDGE_DIGEST_DOMAIN, &[&encoded]));
        predecessor_sets
            .get_mut(&edge.to)
            .ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "plan.edges",
                    "edge target is absent",
                )
            })?
            .insert(edge.from.clone());
        successor_sets
            .get_mut(&edge.from)
            .ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "plan.edges",
                    "edge source is absent",
                )
            })?
            .insert(edge.to.clone());
    }
    edge_ids.sort();
    validate_acyclic(&node_ids, &predecessor_sets, &successor_sets)?;
    for node_id in &node_ids {
        let mut references = BTreeSet::new();
        for value in nodes[node_id].inputs.values() {
            collect_state_references(value, &mut references);
        }
        let bound = state_bindings[node_id]
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>();
        if references != bound {
            return Err(EngineError::new(
                EngineErrorCode::InvalidPlan,
                "plan.nodes.inputs",
                "state references must exactly match typed state dependency edges",
            ));
        }
    }
    let predecessors = predecessor_sets
        .into_iter()
        .map(|(node_id, values)| (node_id, values.into_iter().collect()))
        .collect();
    Ok((nodes, predecessors, state_bindings, node_ids, edge_ids))
}

fn validate_acyclic(
    node_ids: &[String],
    predecessors: &BTreeMap<String, BTreeSet<String>>,
    successors: &BTreeMap<String, BTreeSet<String>>,
) -> Result<(), EngineError> {
    let mut indegree = predecessors
        .iter()
        .map(|(node_id, values)| (node_id.clone(), values.len()))
        .collect::<BTreeMap<_, _>>();
    let mut ready = node_ids
        .iter()
        .filter(|node_id| indegree[*node_id] == 0)
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut visited = 0;
    while let Some(node_id) = ready.pop_first() {
        visited += 1;
        for successor in &successors[&node_id] {
            let count = indegree.get_mut(successor).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "plan.edges",
                    "successor is absent",
                )
            })?;
            *count -= 1;
            if *count == 0 {
                ready.insert(successor.clone());
            }
        }
    }
    if visited != node_ids.len() {
        return Err(EngineError::new(
            EngineErrorCode::InvalidPlan,
            "plan.edges",
            "execution plan contains a dependency cycle",
        ));
    }
    Ok(())
}

fn validate_policy(
    plan: &ExecutionPlan,
    policy: &EnginePolicy,
    predecessors: &BTreeMap<String, Vec<String>>,
    limits: &EngineLimits,
) -> Result<(), EngineError> {
    let plan_ids = plan
        .nodes
        .iter()
        .map(|node| node.id.clone())
        .collect::<BTreeSet<_>>();
    let policy_ids = policy.nodes.keys().cloned().collect::<BTreeSet<_>>();
    if plan_ids != policy_ids {
        return Err(EngineError::new(
            EngineErrorCode::InvalidPolicy,
            "policy.nodes",
            "policy node identities must exactly match the plan",
        ));
    }
    for node in &plan.nodes {
        let node_policy = &policy.nodes[&node.id];
        if node_policy.timeout_ticks == 0
            || node_policy.retry.max_attempts == 0
            || node_policy.retry.max_attempts > limits.max_attempts_per_node
        {
            return Err(EngineError::new(
                EngineErrorCode::InvalidPolicy,
                "policy.nodes",
                "node retry or timeout bounds are invalid",
            ));
        }
        let expected_delays =
            usize::try_from(node_policy.retry.max_attempts - 1).map_err(|error| {
                EngineError::new(
                    EngineErrorCode::InvalidPolicy,
                    "policy.nodes.retry.delay_ticks",
                    &error.to_string(),
                )
            })?;
        if node_policy.retry.delay_ticks.len() != expected_delays
            || node_policy.retry.delay_ticks.contains(&0)
        {
            return Err(EngineError::new(
                EngineErrorCode::InvalidPolicy,
                "policy.nodes.retry.delay_ticks",
                "retry delay schedule must be positive and exact",
            ));
        }
        match &node_policy.port {
            PortKind::Provider => {
                if node.provider_ref.is_empty() {
                    return Err(EngineError::new(
                        EngineErrorCode::InvalidPolicy,
                        "policy.nodes.port",
                        "provider request has no provider identity",
                    ));
                }
            }
            PortKind::Tool { name } => {
                if name.is_empty() || !node.tools.contains(name) {
                    return Err(EngineError::new(
                        EngineErrorCode::InvalidPolicy,
                        "policy.nodes.port",
                        "tool policy is not allowed by the plan node",
                    ));
                }
            }
        }
        let incoming = count_u64(predecessors[&node.id].len(), "policy.nodes.join")?;
        if let JoinPolicy::AtLeast { required } = node_policy.join {
            if required == 0 || required > incoming {
                return Err(EngineError::new(
                    EngineErrorCode::InvalidPolicy,
                    "policy.nodes.join",
                    "at-least join threshold is impossible",
                ));
            }
        }
    }
    Ok(())
}

fn validate_request_envelopes(
    plan: &ExecutionPlan,
    policy: &EnginePolicy,
    nodes: &BTreeMap<String, PlanNode>,
    plan_digest: &str,
    limits: &EngineLimits,
) -> Result<(), EngineError> {
    for (node_id, node) in nodes {
        let node_policy = &policy.nodes[node_id];
        let input_bytes = encode_bounded(&node.inputs, limits.max_request_bytes, "effect.inputs")?;
        let input_digest = hash_parts(INPUT_DIGEST_DOMAIN, &[&input_bytes]);
        let effect = make_effect(
            plan,
            node,
            node_policy,
            plan_digest,
            DispatchInput {
                attempt: node_policy.retry.max_attempts,
                sequence: limits.max_total_attempts,
                inputs: &node.inputs,
                input_digest: &input_digest,
            },
        );
        encode_bounded(&effect, limits.max_request_bytes, "effect").map_err(|_| {
            EngineError::new(
                EngineErrorCode::InvalidLimits,
                "limits.max_request_bytes",
                "request byte limit cannot admit a plan node",
            )
        })?;
    }
    Ok(())
}

struct DispatchInput<'a> {
    attempt: u32,
    sequence: u64,
    inputs: &'a BTreeMap<String, Value>,
    input_digest: &'a str,
}

fn make_effect(
    plan: &ExecutionPlan,
    node: &PlanNode,
    policy: &crate::model::NodePolicy,
    plan_digest: &str,
    dispatch: DispatchInput<'_>,
) -> EngineEffect {
    let request_id = request_identity(
        plan_digest,
        &node.id,
        dispatch.attempt,
        dispatch.sequence,
        dispatch.input_digest,
    );
    let idempotency_key = hash_parts(
        IDEMPOTENCY_DOMAIN,
        &[request_id.as_bytes(), plan.source_digest.as_bytes()],
    );
    match &policy.port {
        PortKind::Provider => EngineEffect::Provider(Box::new(ProviderRequest {
            request_id,
            idempotency_key,
            sequence: dispatch.sequence,
            node_id: node.id.clone(),
            attempt: dispatch.attempt,
            provider_ref: node.provider_ref.clone(),
            model: node.model.clone(),
            prompt: node.prompt.clone(),
            inputs: dispatch.inputs.clone(),
            timeout_ticks: policy.timeout_ticks,
        })),
        PortKind::Tool { name } => EngineEffect::Tool(Box::new(ToolRequest {
            request_id,
            idempotency_key,
            sequence: dispatch.sequence,
            node_id: node.id.clone(),
            attempt: dispatch.attempt,
            tool: name.clone(),
            run: plan.run.clone(),
            inputs: dispatch.inputs.clone(),
            timeout_ticks: policy.timeout_ticks,
        })),
    }
}

fn request_identity(
    plan_digest: &str,
    node_id: &str,
    attempt: u32,
    sequence: u64,
    input_digest: &str,
) -> String {
    let attempt_bytes = attempt.to_be_bytes();
    let sequence_bytes = sequence.to_be_bytes();
    hash_parts(
        REQUEST_ID_DOMAIN,
        &[
            plan_digest.as_bytes(),
            node_id.as_bytes(),
            &attempt_bytes,
            &sequence_bytes,
            input_digest.as_bytes(),
        ],
    )
}

fn effect_request_id(effect: &EngineEffect) -> String {
    match effect {
        EngineEffect::Provider(request) => request.request_id.clone(),
        EngineEffect::Tool(request) => request.request_id.clone(),
        EngineEffect::Cancel(request) => request.request_id.clone(),
    }
}

fn fail_node(
    snapshot: &mut EngineSnapshot,
    events: &mut Vec<EngineEvent>,
    node_id: &str,
    failure: PortFailure,
) -> Result<(), EngineError> {
    let node = snapshot.nodes.get_mut(node_id).ok_or_else(|| {
        EngineError::new(
            EngineErrorCode::InvalidPlan,
            "snapshot.nodes",
            "failed node is absent",
        )
    })?;
    node.state = NodeState::Failed {
        failure: failure.clone(),
    };
    emit(
        snapshot,
        events,
        Some(String::from(node_id)),
        EventKind::NodeFailed { failure },
    )
}

fn emit(
    snapshot: &mut EngineSnapshot,
    events: &mut Vec<EngineEvent>,
    node_id: Option<String>,
    kind: EventKind,
) -> Result<(), EngineError> {
    if snapshot.event_count >= snapshot.limits.max_events {
        return Err(EngineError::new(
            EngineErrorCode::ResourceLimit,
            "events",
            "event budget exhausted",
        ));
    }
    let sequence = snapshot.next_event_sequence;
    snapshot.event_count += 1;
    snapshot.next_event_sequence += 1;
    events.push(EngineEvent {
        sequence,
        node_id,
        kind,
    });
    Ok(())
}

fn ensure_snapshot_bound(snapshot: &EngineSnapshot) -> Result<(), EngineError> {
    encode_bounded(snapshot, snapshot.limits.max_checkpoint_bytes, "checkpoint")?;
    Ok(())
}

fn validate_resumed_snapshot(
    snapshot: &EngineSnapshot,
    expected_engine: &Engine,
) -> Result<(), EngineError> {
    let expected = &expected_engine.snapshot;
    if snapshot.checkpoint_contract != expected.checkpoint_contract
        || snapshot.engine_contract != expected.engine_contract
        || snapshot.plan_contract != expected.plan_contract
        || snapshot.plan_source_digest != expected.plan_source_digest
        || snapshot.plan_digest != expected.plan_digest
        || snapshot.policy_digest != expected.policy_digest
        || snapshot.node_ids != expected.node_ids
        || snapshot.edge_ids != expected.edge_ids
        || snapshot.limits != expected.limits
    {
        return Err(EngineError::new(
            EngineErrorCode::CheckpointIncompatible,
            "checkpoint",
            "checkpoint plan, policy, limits, or contract mismatch",
        ));
    }
    let observed_ids = snapshot.nodes.keys().cloned().collect::<Vec<_>>();
    if observed_ids != snapshot.node_ids
        || snapshot.logical_turns > snapshot.limits.max_logical_turns
        || snapshot.event_count > snapshot.limits.max_events
        || snapshot.event_count != snapshot.next_event_sequence
        || snapshot.attempts_consumed > snapshot.limits.max_total_attempts
        || snapshot.attempts_consumed != snapshot.next_request_sequence
        || snapshot.output_bytes > snapshot.limits.max_output_bytes
        || count_u64(snapshot.turn_journal.len(), "checkpoint.turn_journal")?
            != snapshot.logical_turns
        || snapshot.logical_tick < snapshot.logical_turns
    {
        return Err(EngineError::new(
            EngineErrorCode::CheckpointIncompatible,
            "checkpoint",
            "checkpoint counters or identity set are invalid",
        ));
    }
    let mut replayed = expected_engine.clone();
    for input in &snapshot.turn_journal {
        replayed.turn(input.clone()).map_err(|_| {
            EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint.turn_journal",
                "checkpoint turn journal is not replayable",
            )
        })?;
    }
    if replayed.snapshot != *snapshot {
        return Err(EngineError::new(
            EngineErrorCode::CheckpointIncompatible,
            "checkpoint.turn_journal",
            "checkpoint does not equal deterministic replay of its turn journal",
        ));
    }
    if snapshot.logical_turns == 0 {
        if snapshot != expected {
            return Err(EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint",
                "zero-turn checkpoint differs from the exact initial snapshot",
            ));
        }
        return ensure_snapshot_bound(snapshot);
    }
    if snapshot.logical_tick == 0 {
        return Err(EngineError::new(
            EngineErrorCode::CheckpointIncompatible,
            "checkpoint.logical_tick",
            "executed checkpoint has no logical tick",
        ));
    }
    let mut attempts = 0_u64;
    let mut output_bytes = 0_u64;
    for (node_id, node) in &snapshot.nodes {
        if node.attempts > snapshot.limits.max_attempts_per_node {
            return Err(EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint.nodes",
                "checkpoint node attempt counter exceeds its limit",
            ));
        }
        attempts = attempts
            .checked_add(u64::from(node.attempts))
            .ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::CheckpointIncompatible,
                    "checkpoint.nodes",
                    "checkpoint attempt counter overflow",
                )
            })?;
        match &node.state {
            NodeState::Dispatched { .. } | NodeState::Cancelling { .. } => {
                return Err(EngineError::new(
                    EngineErrorCode::CheckpointIncompatible,
                    "checkpoint.nodes",
                    "checkpoint contains an in-flight request",
                ));
            }
            NodeState::Pending if node.attempts != 0 => {
                return Err(EngineError::new(
                    EngineErrorCode::CheckpointIncompatible,
                    "checkpoint.nodes",
                    "pending node cannot retain consumed attempts",
                ));
            }
            NodeState::Ready => {
                return Err(EngineError::new(
                    EngineErrorCode::CheckpointIncompatible,
                    "checkpoint.nodes",
                    "quiescent checkpoint cannot retain ready work",
                ));
            }
            NodeState::RetryWait { ready_at_tick }
                if node.attempts == 0 || *ready_at_tick <= snapshot.logical_tick =>
            {
                return Err(EngineError::new(
                    EngineErrorCode::CheckpointIncompatible,
                    "checkpoint.nodes",
                    "retry wait is missing an attempt or is already mature",
                ));
            }
            NodeState::Succeeded { .. } if node.attempts == 0 => {
                return Err(EngineError::new(
                    EngineErrorCode::CheckpointIncompatible,
                    "checkpoint.nodes",
                    "successful node has no consumed attempt",
                ));
            }
            NodeState::Succeeded { output } => {
                output_bytes = output_bytes
                    .checked_add(count_u64(output.bytes.len(), "checkpoint.nodes.output")?)
                    .ok_or_else(|| {
                        EngineError::new(
                            EngineErrorCode::CheckpointIncompatible,
                            "checkpoint.nodes",
                            "checkpoint output counter overflow",
                        )
                    })?;
            }
            NodeState::Pending
            | NodeState::RetryWait { .. }
            | NodeState::Failed { .. }
            | NodeState::Cancelled => {}
        }
        let policy = &expected_engine.policy.nodes[node_id];
        if node.attempts > policy.retry.max_attempts {
            return Err(EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint.nodes",
                "node attempts exceed its retry policy",
            ));
        }
        if let NodeState::RetryWait { .. } = &node.state {
            if node.attempts >= policy.retry.max_attempts {
                return Err(EngineError::new(
                    EngineErrorCode::CheckpointIncompatible,
                    "checkpoint.nodes",
                    "nonterminal node has exhausted its retry policy",
                ));
            }
        }
        let decision = dependency_decision(
            snapshot,
            &expected_engine.predecessors[node_id],
            policy,
            &expected_engine.state_bindings[node_id],
        );
        match &node.state {
            NodeState::Pending if decision != DependencyDecision::Wait => {
                return Err(EngineError::new(
                    EngineErrorCode::CheckpointIncompatible,
                    "checkpoint.nodes",
                    "pending node is not waiting on its dependency graph",
                ));
            }
            NodeState::RetryWait { .. } if decision != DependencyDecision::Ready => {
                return Err(EngineError::new(
                    EngineErrorCode::CheckpointIncompatible,
                    "checkpoint.nodes",
                    "retrying node contradicts its dependency graph",
                ));
            }
            NodeState::Failed { failure } if node.attempts == 0 => {
                let dependency = failure.class == FailureClass::Dependency
                    && failure.message == "join condition became impossible"
                    && decision == DependencyDecision::Fail;
                let exhausted = failure.class == FailureClass::RetryExhausted
                    && failure.message == "attempt budget exhausted"
                    && decision == DependencyDecision::Ready
                    && snapshot.attempts_consumed >= snapshot.limits.max_total_attempts;
                if !dependency && !exhausted {
                    return Err(EngineError::new(
                        EngineErrorCode::CheckpointIncompatible,
                        "checkpoint.nodes",
                        "undispatched failure contradicts graph or attempt truth",
                    ));
                }
            }
            NodeState::Pending
            | NodeState::Ready
            | NodeState::Dispatched { .. }
            | NodeState::RetryWait { .. }
            | NodeState::Cancelling { .. }
            | NodeState::Succeeded { .. }
            | NodeState::Failed { .. }
            | NodeState::Cancelled => {}
        }
    }
    if attempts != snapshot.attempts_consumed || output_bytes != snapshot.output_bytes {
        return Err(EngineError::new(
            EngineErrorCode::CheckpointIncompatible,
            "checkpoint",
            "checkpoint accounting does not match node state",
        ));
    }
    if count_u64(
        snapshot.consumed_completion_digests.len(),
        "checkpoint.completions",
    )? != snapshot.attempts_consumed
    {
        return Err(EngineError::new(
            EngineErrorCode::CheckpointIncompatible,
            "checkpoint.completions",
            "checkpoint completion journal is truncated",
        ));
    }
    let mut sequences = BTreeSet::new();
    let mut attempts_by_node = BTreeMap::<String, BTreeSet<u32>>::new();
    for (request_id, receipt) in &snapshot.consumed_completion_digests {
        let node = snapshot.nodes.get(&receipt.node_id).ok_or_else(|| {
            EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint.completions",
                "completion journal names an unknown node",
            )
        })?;
        if !is_hex_digest(request_id)
            || !is_hex_digest(&receipt.input_digest)
            || !is_hex_digest(&receipt.completion_digest)
            || receipt.attempt == 0
            || receipt.attempt > node.attempts
            || receipt.completed_at_tick == 0
            || receipt.completed_at_tick > snapshot.logical_tick
            || receipt.sequence >= snapshot.attempts_consumed
            || !sequences.insert(receipt.sequence)
        {
            return Err(EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint.completions",
                "completion journal identity or sequence is invalid",
            ));
        }
        let (completion_node, completion_attempt) = receipt.completion.identity();
        if receipt.completion.request_id() != request_id
            || completion_node != receipt.node_id
            || completion_attempt != receipt.attempt
        {
            return Err(EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint.completions",
                "completion journal typed completion identity is inconsistent",
            ));
        }
        let completion_bytes = encode_bounded(
            &receipt.completion,
            snapshot.limits.max_completion_bytes,
            "checkpoint.completions.outcome",
        )?;
        if hash_parts(COMPLETION_DIGEST_DOMAIN, &[&completion_bytes]) != receipt.completion_digest {
            return Err(EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint.completions",
                "completion journal digest does not bind its typed completion",
            ));
        }
        let node_policy = &expected_engine.policy.nodes[&receipt.node_id];
        let kind_matches = match (&receipt.completion, &node_policy.port) {
            (PortCompletion::Provider(_), PortKind::Provider)
            | (PortCompletion::Tool(_), PortKind::Tool { .. }) => true,
            (PortCompletion::Cancel(value), _) => value.acknowledged,
            (PortCompletion::Provider(_), PortKind::Tool { .. })
            | (PortCompletion::Tool(_), PortKind::Provider) => false,
        };
        if !kind_matches {
            return Err(EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint.completions",
                "completion journal port kind is invalid",
            ));
        }
        let plan_node = &expected_engine.nodes[&receipt.node_id];
        let inputs = expected_engine
            .resolve_inputs(snapshot, plan_node)
            .map_err(|_| {
                EngineError::new(
                    EngineErrorCode::CheckpointIncompatible,
                    "checkpoint.completions",
                    "completion journal inputs cannot be resolved",
                )
            })?;
        let input_bytes = encode_bounded(
            &inputs,
            snapshot.limits.max_request_bytes,
            "checkpoint.completions.inputs",
        )?;
        let input_digest = hash_parts(INPUT_DIGEST_DOMAIN, &[&input_bytes]);
        let expected_request = request_identity(
            &snapshot.plan_digest,
            &receipt.node_id,
            receipt.attempt,
            receipt.sequence,
            &input_digest,
        );
        if input_digest != receipt.input_digest || expected_request != *request_id {
            return Err(EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint.completions",
                "completion journal does not match deterministic request identity",
            ));
        }
        if !attempts_by_node
            .entry(receipt.node_id.clone())
            .or_default()
            .insert(receipt.attempt)
        {
            return Err(EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint.completions",
                "completion journal repeats a node attempt",
            ));
        }
    }
    for (node_id, node) in &snapshot.nodes {
        let observed = attempts_by_node.get(node_id).cloned().unwrap_or_default();
        let expected_attempts = (1..=node.attempts).collect::<BTreeSet<_>>();
        if observed != expected_attempts {
            return Err(EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint.completions",
                "completion journal does not contiguously cover node attempts",
            ));
        }
        if node.attempts > 0 {
            let receipt = snapshot
                .consumed_completion_digests
                .values()
                .find(|receipt| {
                    receipt.node_id.as_str() == node_id && receipt.attempt == node.attempts
                })
                .ok_or_else(|| {
                    EngineError::new(
                        EngineErrorCode::CheckpointIncompatible,
                        "checkpoint.completions",
                        "completion journal lacks the final node attempt",
                    )
                })?;
            validate_final_completion(snapshot, node_id, node, receipt, expected_engine)?;
        }
    }
    ensure_snapshot_bound(snapshot)
}

fn validate_final_completion(
    snapshot: &EngineSnapshot,
    node_id: &str,
    node: &NodeSnapshot,
    receipt: &CompletionReceipt,
    engine: &Engine,
) -> Result<(), EngineError> {
    let policy = &engine.policy.nodes[node_id];
    let outcome = match &receipt.completion {
        PortCompletion::Provider(value) => Some(&value.outcome),
        PortCompletion::Tool(value) => Some(&value.outcome),
        PortCompletion::Cancel(_) => None,
    };
    let valid = match (&node.state, outcome) {
        (NodeState::Succeeded { output }, Some(CompletionOutcome::Success(observed))) => {
            output == observed
        }
        (NodeState::RetryWait { ready_at_tick }, Some(CompletionOutcome::Failure(failure))) => {
            if !policy.retry.retryable.contains(&failure.class)
                || receipt.attempt >= policy.retry.max_attempts
            {
                false
            } else {
                let index = usize::try_from(receipt.attempt - 1).map_err(|error| {
                    EngineError::new(
                        EngineErrorCode::CheckpointIncompatible,
                        "checkpoint.completions",
                        &error.to_string(),
                    )
                })?;
                receipt
                    .completed_at_tick
                    .checked_add(policy.retry.delay_ticks[index])
                    == Some(*ready_at_tick)
            }
        }
        (NodeState::Failed { failure }, Some(CompletionOutcome::Failure(observed))) => {
            if policy.retry.retryable.contains(&observed.class) {
                let policy_exhausted = receipt.attempt >= policy.retry.max_attempts
                    && *failure
                        == PortFailure::new(
                            FailureClass::RetryExhausted,
                            "retry attempts exhausted",
                        );
                let global_exhausted = receipt.attempt < policy.retry.max_attempts
                    && snapshot.attempts_consumed >= snapshot.limits.max_total_attempts
                    && *failure
                        == PortFailure::new(
                            FailureClass::RetryExhausted,
                            "attempt budget exhausted",
                        );
                policy_exhausted || global_exhausted
            } else {
                failure == observed
            }
        }
        (NodeState::Cancelled, None) => true,
        (NodeState::Cancelled, Some(CompletionOutcome::Failure(failure))) => {
            policy.retry.retryable.contains(&failure.class)
                && receipt.attempt < policy.retry.max_attempts
        }
        (NodeState::Pending, _)
        | (NodeState::Ready, _)
        | (NodeState::Dispatched { .. }, _)
        | (NodeState::RetryWait { .. }, _)
        | (NodeState::Cancelling { .. }, _)
        | (NodeState::Succeeded { .. }, _)
        | (NodeState::Failed { .. }, _)
        | (NodeState::Cancelled, _) => false,
    };
    if !valid {
        return Err(EngineError::new(
            EngineErrorCode::CheckpointIncompatible,
            "checkpoint.completions",
            "final node state is not explained by its final typed completion",
        ));
    }
    Ok(())
}

fn count_u64(value: usize, path: &str) -> Result<u64, EngineError> {
    u64::try_from(value)
        .map_err(|error| EngineError::new(EngineErrorCode::ResourceLimit, path, &error.to_string()))
}

fn validate_turn_input(input: &TurnInput, limits: &EngineLimits) -> Result<(), EngineError> {
    if count_u64(input.completions.len(), "turn.completions")? > limits.max_completions_per_turn
        || count_u64(input.cancellations.len(), "turn.cancellations")?
            > limits.max_cancellations_per_turn
    {
        return Err(EngineError::new(
            EngineErrorCode::ResourceLimit,
            "turn",
            "turn input cardinality limit exceeded",
        ));
    }
    preflight_turn(input, limits.max_turn_input_bytes)?;
    encode_bounded(input, limits.max_turn_input_bytes, "turn")?;
    for completion in &input.completions {
        preflight_completion(completion, limits.max_completion_bytes)?;
        encode_bounded(completion, limits.max_completion_bytes, "turn.completions")?;
    }
    Ok(())
}

fn preflight_plan(plan: &ExecutionPlan, maximum: u64) -> Result<(), EngineError> {
    let mut size = 1_u64;
    add_text(&mut size, &plan.contract, maximum, "plan")?;
    add_text(&mut size, &plan.source_digest, maximum, "plan")?;
    add_text(&mut size, &plan.run.identity, maximum, "plan")?;
    add_text(&mut size, &plan.run.name, maximum, "plan")?;
    if let Some(target) = &plan.run.placement_target {
        add_text(&mut size, target, maximum, "plan")?;
    }
    for (key, value) in &plan.run.inputs {
        add_text(&mut size, key, maximum, "plan")?;
        add_value(&mut size, value, maximum, "plan")?;
    }
    add_text(&mut size, &plan.workflow.identity, maximum, "plan")?;
    for node in &plan.nodes {
        add_size(&mut size, 1, maximum, "plan")?;
        for text in [
            &node.id,
            &node.step_id,
            &node.task_ref,
            &node.agent_ref,
            &node.provider_ref,
            &node.prompt.user,
            &node.provenance.document_version,
            &node.provenance.workflow_identity,
            &node.provenance.semantic_path,
            &node.provenance.task_ref,
            &node.provenance.agent_ref,
            &node.provenance.provider_ref,
        ] {
            add_text(&mut size, text, maximum, "plan")?;
        }
        for text in node
            .model
            .iter()
            .chain(node.save_as.iter())
            .chain(node.prompt.system.iter())
            .chain(node.tools.iter())
            .chain(node.ports.inputs.iter())
            .chain(node.ports.outputs.iter())
        {
            add_text(&mut size, text, maximum, "plan")?;
        }
        for (key, value) in &node.inputs {
            add_text(&mut size, key, maximum, "plan")?;
            add_value(&mut size, value, maximum, "plan")?;
        }
    }
    for edge in &plan.edges {
        add_size(&mut size, 1, maximum, "plan")?;
        add_text(&mut size, &edge.from, maximum, "plan")?;
        add_text(&mut size, &edge.to, maximum, "plan")?;
        if let Some(state) = &edge.state {
            add_text(&mut size, state, maximum, "plan")?;
        }
    }
    Ok(())
}

fn preflight_policy(policy: &EnginePolicy, maximum: u64) -> Result<(), EngineError> {
    let mut size = 1_u64;
    for (node_id, node) in &policy.nodes {
        add_text(&mut size, node_id, maximum, "policy")?;
        add_size(&mut size, 1, maximum, "policy")?;
        add_size(
            &mut size,
            count_u64(node.retry.retryable.len(), "policy")?,
            maximum,
            "policy",
        )?;
        add_size(
            &mut size,
            count_u64(node.retry.delay_ticks.len(), "policy")?,
            maximum,
            "policy",
        )?;
        if let PortKind::Tool { name } = &node.port {
            add_text(&mut size, name, maximum, "policy")?;
        }
    }
    Ok(())
}

fn preflight_turn(input: &TurnInput, maximum: u64) -> Result<(), EngineError> {
    let mut size = 1_u64;
    for completion in &input.completions {
        add_completion(&mut size, completion, maximum, "turn")?;
    }
    for node_id in &input.cancellations {
        add_text(&mut size, node_id, maximum, "turn")?;
    }
    Ok(())
}

fn preflight_completion(completion: &PortCompletion, maximum: u64) -> Result<(), EngineError> {
    let mut size = 1_u64;
    add_completion(&mut size, completion, maximum, "turn.completions")
}

fn add_completion(
    size: &mut u64,
    completion: &PortCompletion,
    maximum: u64,
    path: &str,
) -> Result<(), EngineError> {
    add_size(size, 1, maximum, path)?;
    match completion {
        PortCompletion::Provider(value) => {
            add_text(size, &value.request_id, maximum, path)?;
            add_text(size, &value.node_id, maximum, path)?;
            add_outcome(size, &value.outcome, maximum, path)
        }
        PortCompletion::Tool(value) => {
            add_text(size, &value.request_id, maximum, path)?;
            add_text(size, &value.node_id, maximum, path)?;
            add_outcome(size, &value.outcome, maximum, path)
        }
        PortCompletion::Cancel(value) => {
            add_text(size, &value.request_id, maximum, path)?;
            add_text(size, &value.node_id, maximum, path)
        }
    }
}

fn add_outcome(
    size: &mut u64,
    outcome: &CompletionOutcome,
    maximum: u64,
    path: &str,
) -> Result<(), EngineError> {
    match outcome {
        CompletionOutcome::Success(output) => {
            add_text(size, &output.media_type, maximum, path)?;
            add_size(size, count_u64(output.bytes.len(), path)?, maximum, path)
        }
        CompletionOutcome::Failure(failure) => add_text(size, &failure.message, maximum, path),
    }
}

fn add_value(size: &mut u64, value: &Value, maximum: u64, path: &str) -> Result<(), EngineError> {
    add_size(size, 1, maximum, path)?;
    match value {
        Value::String(text) => add_text(size, text, maximum, path),
        Value::Array(values) => {
            for item in values {
                add_value(size, item, maximum, path)?;
            }
            Ok(())
        }
        Value::Object(values) => {
            for (key, item) in values {
                add_text(size, key, maximum, path)?;
                add_value(size, item, maximum, path)?;
            }
            Ok(())
        }
        Value::Null | Value::Bool(_) => Ok(()),
        Value::Number(_) => add_size(size, 32, maximum, path),
    }
}

fn add_text(size: &mut u64, value: &str, maximum: u64, path: &str) -> Result<(), EngineError> {
    let bytes = count_u64(value.len(), path)?
        .checked_add(1)
        .ok_or_else(|| {
            EngineError::new(
                EngineErrorCode::ResourceLimit,
                path,
                "preflight string byte accounting overflow",
            )
        })?;
    add_size(size, bytes, maximum, path)
}

fn add_size(size: &mut u64, value: u64, maximum: u64, path: &str) -> Result<(), EngineError> {
    *size = size.checked_add(value).ok_or_else(|| {
        EngineError::new(
            EngineErrorCode::ResourceLimit,
            path,
            "preflight byte accounting overflow",
        )
    })?;
    if *size > maximum {
        return Err(EngineError::new(
            EngineErrorCode::ResourceLimit,
            path,
            "preflight byte limit exceeded",
        ));
    }
    Ok(())
}

fn limit_usize(value: u64, path: &str) -> Result<usize, EngineError> {
    usize::try_from(value)
        .map_err(|error| EngineError::new(EngineErrorCode::InvalidLimits, path, &error.to_string()))
}

fn encode<T: Serialize>(value: &T, path: &str) -> Result<Vec<u8>, EngineError> {
    serde_json::to_vec(value)
        .map_err(|error| EngineError::new(EngineErrorCode::Serialization, path, &error.to_string()))
}

struct CappedBuffer {
    bytes: Vec<u8>,
    maximum: usize,
    exceeded: bool,
}

impl Write for CappedBuffer {
    fn write(&mut self, bytes: &[u8]) -> IoResult<usize> {
        let Some(next) = self.bytes.len().checked_add(bytes.len()) else {
            self.exceeded = true;
            return Err(IoError::other("serialized byte accounting overflow"));
        };
        if next > self.maximum {
            self.exceeded = true;
            return Err(IoError::other("serialized byte limit exceeded"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }
}

fn encode_bounded<T: Serialize>(
    value: &T,
    maximum: u64,
    path: &str,
) -> Result<Vec<u8>, EngineError> {
    let maximum = limit_usize(maximum, path)?;
    let mut writer = CappedBuffer {
        bytes: Vec::with_capacity(maximum.min(4096)),
        maximum,
        exceeded: false,
    };
    let result = serde_json::to_writer(&mut writer, value);
    if writer.exceeded {
        return Err(EngineError::new(
            EngineErrorCode::ResourceLimit,
            path,
            "serialized byte limit exceeded",
        ));
    }
    result.map_err(|error| {
        EngineError::new(EngineErrorCode::Serialization, path, &error.to_string())
    })?;
    Ok(writer.bytes)
}

fn hash_parts(domain: &[u8], parts: &[&[u8]]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    for part in parts {
        let length = u64::try_from(part.len()).unwrap_or(u64::MAX).to_be_bytes();
        hasher.update(length);
        hasher.update(part);
    }
    hex::encode(hasher.finalize())
}

fn is_hex_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}
