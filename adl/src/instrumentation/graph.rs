use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::execution_plan::ExecutionPlan;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphNode {
    pub id: String,
    pub save_as: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphExport {
    pub workflow_kind: String,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

pub fn export_graph(plan: &ExecutionPlan) -> GraphExport {
    let mut nodes: Vec<GraphNode> = plan
        .nodes
        .iter()
        .map(|n| GraphNode {
            id: n.step_id.clone(),
            save_as: n.save_as.clone(),
        })
        .collect();
    nodes.sort_by(|a, b| a.id.cmp(&b.id));

    let mut edges: Vec<GraphEdge> = Vec::new();
    for node in &plan.nodes {
        for dep in &node.depends_on {
            edges.push(GraphEdge {
                from: dep.clone(),
                to: node.step_id.clone(),
            });
        }
    }
    edges.sort();
    edges.dedup();

    GraphExport {
        workflow_kind: format!("{:?}", plan.workflow_kind).to_lowercase(),
        nodes,
        edges,
    }
}

pub fn export_graph_json(plan: &ExecutionPlan) -> Result<String> {
    let graph = export_graph(plan);
    serde_json::to_string_pretty(&graph).context("serialize graph json")
}

pub fn export_graph_dot(plan: &ExecutionPlan) -> String {
    let graph = export_graph(plan);
    let mut out = String::new();
    out.push_str("digraph execution_plan {\n");
    out.push_str("  rankdir=LR;\n");

    for node in &graph.nodes {
        let label = if let Some(save_as) = node.save_as.as_deref() {
            format!("{}\\nsave_as={}", escape_dot(&node.id), escape_dot(save_as))
        } else {
            escape_dot(&node.id)
        };
        out.push_str(&format!(
            "  \"{}\" [label=\"{}\"];\n",
            escape_dot(&node.id),
            label
        ));
    }

    for edge in &graph.edges {
        out.push_str(&format!(
            "  \"{}\" -> \"{}\";\n",
            escape_dot(&edge.from),
            escape_dot(&edge.to)
        ));
    }

    out.push_str("}\n");
    out
}

fn escape_dot(v: &str) -> String {
    v.replace('\\', "\\\\").replace('"', "\\\"")
}
