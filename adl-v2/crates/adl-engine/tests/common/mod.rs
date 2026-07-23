#![allow(dead_code)]

use adl_engine::{
    CompletionOutcome, EngineEffect, EngineLimits, EnginePolicy, ExecutionPlan, FailureClass,
    JoinPolicy, NodePolicy, PortCompletion, PortFailure, PortKind, PortOutput, ProviderCompletion,
    ProviderRequest, RetryPolicy, ToolCompletion, ToolRequest, TurnOutput,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub fn plan(node_ids: &[&str], edges: &[(&str, &str)]) -> ExecutionPlan {
    plan_with_edge_kind(node_ids, edges, "sequential")
}

pub fn plan_with_edge_kind(
    node_ids: &[&str],
    edges: &[(&str, &str)],
    edge_kind: &str,
) -> ExecutionPlan {
    let nodes = node_ids
        .iter()
        .enumerate()
        .map(|(index, node_id)| {
            let inputs = if edge_kind == "state_dependency" && index == 1 {
                json!({"input": "@state:state-0"})
            } else {
                json!({"ordinal": index})
            };
            json!({
                "id": node_id,
                "step_id": format!("step-{index}"),
                "task_ref": "task",
                "agent_ref": "agent",
                "provider_ref": "provider",
                "model": "model",
                "tools": ["tool-a"],
                "ports": {"inputs": ["input"], "outputs": ["output"]},
                "prompt": {"system": "system", "user": format!("node-{index}")},
                "inputs": inputs,
                "save_as": format!("state-{index}"),
                "provenance": {
                    "document_version": "0.5",
                    "workflow_identity": "flow",
                    "semantic_path": format!("$.run.workflow.steps[{index}]"),
                    "task_ref": "task",
                    "agent_ref": "agent",
                    "provider_ref": "provider"
                }
            })
        })
        .collect::<Vec<_>>();
    let edges = edges
        .iter()
        .map(|(from, to)| {
            let state = if edge_kind == "state_dependency" {
                let index = node_ids.iter().position(|node_id| node_id == from).unwrap();
                Some(format!("state-{index}"))
            } else {
                None
            };
            json!({
                "from": from,
                "to": to,
                "kind": edge_kind,
                "state": state
            })
        })
        .collect::<Vec<_>>();
    serde_json::from_value(json!({
        "contract": "adl.execution-plan.v1",
        "source_digest": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "run": {
            "identity": "run",
            "name": "test",
            "inputs": {"request": "hello"},
            "placement_target": null
        },
        "workflow": {"identity": "flow", "kind": "concurrent"},
        "nodes": nodes,
        "edges": edges
    }))
    .unwrap()
}

pub fn limits() -> EngineLimits {
    EngineLimits {
        max_plan_nodes: 32,
        max_dependency_edges: 64,
        max_plan_bytes: 1_048_576,
        max_policy_bytes: 1_048_576,
        max_ready_nodes: 32,
        max_in_flight: 32,
        max_total_attempts: 128,
        max_attempts_per_node: 8,
        max_request_bytes: 65_536,
        max_completion_bytes: 65_536,
        max_completions_per_turn: 32,
        max_cancellations_per_turn: 32,
        max_turn_input_bytes: 131_072,
        max_output_bytes: 65_536,
        max_events: 1_024,
        max_checkpoint_bytes: 1_048_576,
        max_logical_turns: 1_024,
    }
}

pub fn provider_policy(plan: &ExecutionPlan) -> EnginePolicy {
    EnginePolicy::provider_for(plan, 20)
}

pub fn retry_policy(plan: &ExecutionPlan, max_attempts: u32, delay: u64) -> EnginePolicy {
    let mut policies = provider_policy(plan);
    for policy in policies.nodes.values_mut() {
        policy.retry = RetryPolicy {
            max_attempts,
            retryable: BTreeSet::from([FailureClass::Retryable, FailureClass::Timeout]),
            delay_ticks: (1..max_attempts).map(|_| delay).collect(),
        };
    }
    policies
}

pub fn with_join(policy: &EnginePolicy, node_id: &str, join: JoinPolicy) -> EnginePolicy {
    let mut changed = policy.clone();
    changed.nodes.get_mut(node_id).unwrap().join = join;
    changed
}

pub fn with_tool(policy: &EnginePolicy, node_id: &str) -> EnginePolicy {
    let mut changed = policy.clone();
    changed.nodes.get_mut(node_id).unwrap().port = PortKind::Tool {
        name: "tool-a".into(),
    };
    changed
}

pub fn provider_request(output: &TurnOutput) -> ProviderRequest {
    output
        .effects
        .iter()
        .find_map(|effect| match effect {
            EngineEffect::Provider(request) => Some((**request).clone()),
            EngineEffect::Tool(_) | EngineEffect::Cancel(_) => None,
        })
        .unwrap()
}

pub fn provider_requests(output: &TurnOutput) -> Vec<ProviderRequest> {
    output
        .effects
        .iter()
        .filter_map(|effect| match effect {
            EngineEffect::Provider(request) => Some((**request).clone()),
            EngineEffect::Tool(_) | EngineEffect::Cancel(_) => None,
        })
        .collect()
}

pub fn tool_request(output: &TurnOutput) -> ToolRequest {
    output
        .effects
        .iter()
        .find_map(|effect| match effect {
            EngineEffect::Tool(request) => Some((**request).clone()),
            EngineEffect::Provider(_) | EngineEffect::Cancel(_) => None,
        })
        .unwrap()
}

pub fn provider_success(request: &ProviderRequest, bytes: &[u8]) -> PortCompletion {
    PortCompletion::Provider(Box::new(ProviderCompletion {
        request_id: request.request_id.clone(),
        node_id: request.node_id.clone(),
        attempt: request.attempt,
        outcome: CompletionOutcome::Success(PortOutput::new("text/plain", bytes.to_vec())),
    }))
}

pub fn provider_failure(request: &ProviderRequest, class: FailureClass) -> PortCompletion {
    PortCompletion::Provider(Box::new(ProviderCompletion {
        request_id: request.request_id.clone(),
        node_id: request.node_id.clone(),
        attempt: request.attempt,
        outcome: CompletionOutcome::Failure(PortFailure::new(class, "failure")),
    }))
}

pub fn tool_success(request: &ToolRequest, bytes: &[u8]) -> PortCompletion {
    PortCompletion::Tool(Box::new(ToolCompletion {
        request_id: request.request_id.clone(),
        node_id: request.node_id.clone(),
        attempt: request.attempt,
        outcome: CompletionOutcome::Success(PortOutput::new("text/plain", bytes.to_vec())),
    }))
}

pub fn node_policy(port: PortKind, join: JoinPolicy) -> NodePolicy {
    NodePolicy {
        port,
        join,
        retry: RetryPolicy::once(),
        timeout_ticks: 20,
    }
}

pub fn completion_value(completion: &PortCompletion) -> Value {
    serde_json::to_value(completion).unwrap()
}

pub fn policy_map(entries: &[(&str, NodePolicy)]) -> EnginePolicy {
    EnginePolicy::new(
        entries
            .iter()
            .map(|(node_id, policy)| (String::from(*node_id), policy.clone()))
            .collect::<BTreeMap<_, _>>(),
    )
}
