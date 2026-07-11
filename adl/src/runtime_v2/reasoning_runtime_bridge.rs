//! One-way bridge from merged WP-11 objects into the native CSM reasoning runtime.

use std::collections::BTreeMap;

use adl_runtime::reasoning_runtime::{
    BoundedLoop, ReasoningGraph, ReasoningNode, ReasoningNodeKind, ReasoningObject,
};
use anyhow::{anyhow, Result};

use super::{
    RuntimeV2LoopRuntimePacket, RuntimeV2ReasoningGraphPacket, RuntimeV2ReasoningNodeKind,
};

pub fn native_reasoning_graph(packet: &RuntimeV2ReasoningGraphPacket) -> Result<ReasoningGraph> {
    packet.validate()?;
    let mut dependencies = BTreeMap::<String, Vec<String>>::new();
    for edge in &packet.graph.edges {
        dependencies
            .entry(edge.to.clone())
            .or_default()
            .push(edge.from.clone());
    }
    let nodes = packet
        .graph
        .nodes
        .iter()
        .map(|node| {
            let mut dependencies = dependencies.remove(&node.node_id).unwrap_or_default();
            dependencies.sort();
            dependencies.dedup();
            ReasoningNode {
                id: node.node_id.clone(),
                kind: match node.node_kind {
                    RuntimeV2ReasoningNodeKind::PromptInput => ReasoningNodeKind::PromptInput,
                    RuntimeV2ReasoningNodeKind::Hypothesis => ReasoningNodeKind::Hypothesis,
                    RuntimeV2ReasoningNodeKind::Evidence => ReasoningNodeKind::Evidence,
                    RuntimeV2ReasoningNodeKind::Decision => ReasoningNodeKind::Decision,
                    RuntimeV2ReasoningNodeKind::Outcome => ReasoningNodeKind::Outcome,
                },
                dependencies,
            }
        })
        .collect();
    Ok(ReasoningGraph {
        graph_id: packet.graph_id.clone(),
        nodes,
    })
}

pub fn native_reasoning_loop(
    graph: &RuntimeV2ReasoningGraphPacket,
    loop_runtime: &RuntimeV2LoopRuntimePacket,
) -> Result<ReasoningObject> {
    graph.validate()?;
    loop_runtime.validate()?;
    if loop_runtime.reasoning_graph_id != graph.graph_id
        || loop_runtime.loop_definition.graph_id != graph.graph_id
    {
        return Err(anyhow!("WP-11 loop runtime graph binding mismatch"));
    }
    let exit_node_id = loop_runtime
        .loop_definition
        .terminal_node_ids
        .first()
        .cloned()
        .ok_or_else(|| anyhow!("WP-11 loop runtime has no terminal node"))?;
    Ok(ReasoningObject::Loop(BoundedLoop {
        loop_id: loop_runtime.loop_definition.loop_id.clone(),
        graph: native_reasoning_graph(graph)?,
        max_iterations: loop_runtime.loop_definition.max_iterations,
        exit_node_id,
    }))
}

#[cfg(test)]
mod tests {
    use adl_runtime::reasoning_runtime::{
        FreedomGateDisposition, GovernanceContext, ReasoningAdmission, ReasoningCore,
    };

    use super::*;
    use crate::runtime_v2::{
        runtime_v2_loop_runtime_contract, runtime_v2_reasoning_graph_contract,
    };

    #[test]
    fn runtime_v2_reasoning_objects_execute_through_native_component_core() {
        let graph = runtime_v2_reasoning_graph_contract().expect("WP-11 reasoning graph");
        let loop_runtime = runtime_v2_loop_runtime_contract().expect("WP-11 loop runtime");
        let object = native_reasoning_loop(&graph, &loop_runtime).expect("native reasoning object");
        let execution = ReasoningCore::default()
            .execute(ReasoningAdmission {
                admission_id: "wp-11-native-proof".to_string(),
                object,
                governance: GovernanceContext {
                    freedom_gate: FreedomGateDisposition::Approved,
                    aee_available: true,
                    policy_ref: "wp-11/aee-handoff".to_string(),
                },
                checkpoint: None,
                provider_result: None,
                replay_only: true,
            })
            .expect("native execution");
        assert_eq!(execution.object_id, loop_runtime.loop_definition.loop_id);
        assert_eq!(
            execution.iterations,
            loop_runtime.loop_definition.max_iterations
        );
        assert!(execution.aee_request.is_some());
    }
}
