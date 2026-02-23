use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::execution_plan::ExecutionPlan;
use crate::trace::TraceEvent;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum TraceEventNormalized {
    RunFailed {
        message: String,
    },
    RunFinished {
        success: bool,
    },
    StepStarted {
        step_id: String,
        agent_id: String,
        provider_id: String,
        task_id: String,
        delegation_json: Option<String>,
    },
    PromptAssembled {
        step_id: String,
        prompt_hash: String,
    },
    StepOutputChunk {
        step_id: String,
        chunk_bytes: usize,
    },
    StepFinished {
        step_id: String,
        success: bool,
    },
    CallEntered {
        caller_step_id: String,
        callee_workflow_id: String,
        namespace: String,
    },
    CallExited {
        caller_step_id: String,
        status: String,
        namespace: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceReplay {
    pub step_started_order: Vec<String>,
    pub step_finished_order: Vec<String>,
    pub step_output_chunk_order: Vec<String>,
    pub events: Vec<TraceEventNormalized>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlanDiff {
    pub added_nodes: Vec<String>,
    pub removed_nodes: Vec<String>,
    pub changed_dependencies: Vec<String>,
    pub changed_save_as: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceDiff {
    pub changed_indices: Vec<usize>,
    pub left_only: Vec<String>,
    pub right_only: Vec<String>,
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

pub fn write_trace_artifact(path: &Path, events: &[TraceEvent]) -> Result<()> {
    let normalized = normalize_trace_events(events);
    let body = serde_json::to_vec_pretty(&normalized).context("serialize trace artifact")?;
    fs::write(path, body)
        .with_context(|| format!("failed writing trace artifact '{}'", path.display()))
}

pub fn load_trace_artifact(path: &Path) -> Result<Vec<TraceEventNormalized>> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed reading trace artifact '{}'", path.display()))?;
    serde_json::from_str(&raw).with_context(|| {
        format!(
            "failed parsing trace artifact '{}' as normalized trace json",
            path.display()
        )
    })
}

pub fn replay_trace(events: &[TraceEventNormalized]) -> TraceReplay {
    let mut step_started_order = Vec::new();
    let mut step_finished_order = Vec::new();
    let mut step_output_chunk_order = Vec::new();

    for ev in events {
        match ev {
            TraceEventNormalized::StepStarted { step_id, .. } => {
                step_started_order.push(step_id.clone())
            }
            TraceEventNormalized::StepFinished { step_id, .. } => {
                step_finished_order.push(step_id.clone())
            }
            TraceEventNormalized::StepOutputChunk { step_id, .. } => {
                step_output_chunk_order.push(step_id.clone())
            }
            _ => {}
        }
    }

    TraceReplay {
        step_started_order,
        step_finished_order,
        step_output_chunk_order,
        events: events.to_vec(),
    }
}

pub fn diff_plans(left: &ExecutionPlan, right: &ExecutionPlan) -> PlanDiff {
    let to_map = |plan: &ExecutionPlan| -> BTreeMap<String, (Vec<String>, Option<String>)> {
        let mut map = BTreeMap::new();
        for node in &plan.nodes {
            let mut deps = node.depends_on.clone();
            deps.sort();
            map.insert(node.step_id.clone(), (deps, node.save_as.clone()));
        }
        map
    };

    let left_map = to_map(left);
    let right_map = to_map(right);

    let left_ids: BTreeSet<String> = left_map.keys().cloned().collect();
    let right_ids: BTreeSet<String> = right_map.keys().cloned().collect();

    let added_nodes = right_ids.difference(&left_ids).cloned().collect();
    let removed_nodes = left_ids.difference(&right_ids).cloned().collect();

    let mut changed_dependencies = Vec::new();
    let mut changed_save_as = Vec::new();
    for id in left_ids.intersection(&right_ids) {
        let (left_deps, left_save_as) = &left_map[id];
        let (right_deps, right_save_as) = &right_map[id];
        if left_deps != right_deps {
            changed_dependencies.push(id.clone());
        }
        if left_save_as != right_save_as {
            changed_save_as.push(id.clone());
        }
    }

    PlanDiff {
        added_nodes,
        removed_nodes,
        changed_dependencies,
        changed_save_as,
    }
}

pub fn diff_traces(left: &[TraceEventNormalized], right: &[TraceEventNormalized]) -> TraceDiff {
    let mut changed_indices = Vec::new();
    let min_len = left.len().min(right.len());
    for idx in 0..min_len {
        if left[idx] != right[idx] {
            changed_indices.push(idx);
        }
    }

    let left_only = if left.len() > min_len {
        left[min_len..]
            .iter()
            .map(format_normalized_event)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let right_only = if right.len() > min_len {
        right[min_len..]
            .iter()
            .map(format_normalized_event)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    TraceDiff {
        changed_indices,
        left_only,
        right_only,
    }
}

pub fn normalize_trace_events(events: &[TraceEvent]) -> Vec<TraceEventNormalized> {
    events
        .iter()
        .map(|ev| match ev {
            TraceEvent::RunFailed { message, .. } => TraceEventNormalized::RunFailed {
                message: message.clone(),
            },
            TraceEvent::RunFinished { success, .. } => {
                TraceEventNormalized::RunFinished { success: *success }
            }
            TraceEvent::StepStarted {
                step_id,
                agent_id,
                provider_id,
                task_id,
                delegation,
                ..
            } => TraceEventNormalized::StepStarted {
                step_id: step_id.clone(),
                agent_id: agent_id.clone(),
                provider_id: provider_id.clone(),
                task_id: task_id.clone(),
                delegation_json: delegation.as_ref().and_then(|d| {
                    if d.is_effectively_empty() {
                        None
                    } else {
                        serde_json::to_string(&d.canonicalized()).ok()
                    }
                }),
            },
            TraceEvent::PromptAssembled {
                step_id,
                prompt_hash,
                ..
            } => TraceEventNormalized::PromptAssembled {
                step_id: step_id.clone(),
                prompt_hash: prompt_hash.clone(),
            },
            TraceEvent::StepOutputChunk {
                step_id,
                chunk_bytes,
                ..
            } => TraceEventNormalized::StepOutputChunk {
                step_id: step_id.clone(),
                chunk_bytes: *chunk_bytes,
            },
            TraceEvent::StepFinished {
                step_id, success, ..
            } => TraceEventNormalized::StepFinished {
                step_id: step_id.clone(),
                success: *success,
            },
            TraceEvent::CallEntered {
                caller_step_id,
                callee_workflow_id,
                namespace,
                ..
            } => TraceEventNormalized::CallEntered {
                caller_step_id: caller_step_id.clone(),
                callee_workflow_id: callee_workflow_id.clone(),
                namespace: namespace.clone(),
            },
            TraceEvent::CallExited {
                caller_step_id,
                status,
                namespace,
                ..
            } => TraceEventNormalized::CallExited {
                caller_step_id: caller_step_id.clone(),
                status: status.clone(),
                namespace: namespace.clone(),
            },
        })
        .collect()
}

pub fn format_normalized_event(ev: &TraceEventNormalized) -> String {
    match ev {
        TraceEventNormalized::RunFailed { message } => {
            format!("RunFailed message={message}")
        }
        TraceEventNormalized::RunFinished { success } => {
            format!("RunFinished success={success}")
        }
        TraceEventNormalized::StepStarted {
            step_id,
            agent_id,
            provider_id,
            task_id,
            delegation_json,
        } => {
            let base = format!(
                "StepStarted step={step_id} agent={agent_id} provider={provider_id} task={task_id}"
            );
            if let Some(d) = delegation_json {
                format!("{base} delegation={d}")
            } else {
                base
            }
        }
        TraceEventNormalized::PromptAssembled {
            step_id,
            prompt_hash,
        } => {
            format!("PromptAssembled step={step_id} hash={prompt_hash}")
        }
        TraceEventNormalized::StepOutputChunk {
            step_id,
            chunk_bytes,
        } => {
            format!("StepOutputChunk step={step_id} bytes={chunk_bytes}")
        }
        TraceEventNormalized::StepFinished { step_id, success } => {
            format!("StepFinished step={step_id} success={success}")
        }
        TraceEventNormalized::CallEntered {
            caller_step_id,
            callee_workflow_id,
            namespace,
        } => {
            format!("CallEntered caller_step={caller_step_id} callee_workflow={callee_workflow_id} namespace={namespace}")
        }
        TraceEventNormalized::CallExited {
            caller_step_id,
            status,
            namespace,
        } => {
            format!("CallExited caller_step={caller_step_id} status={status} namespace={namespace}")
        }
    }
}

fn escape_dot(v: &str) -> String {
    v.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adl::{DelegationSpec, WorkflowKind};
    use crate::execution_plan::ExecutionNode;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn sample_plan() -> ExecutionPlan {
        ExecutionPlan {
            workflow_kind: WorkflowKind::Concurrent,
            nodes: vec![
                ExecutionNode {
                    step_id: "b".to_string(),
                    depends_on: vec!["a".to_string()],
                    save_as: Some("b_out".to_string()),
                    delegation: None,
                },
                ExecutionNode {
                    step_id: "a".to_string(),
                    depends_on: vec![],
                    save_as: Some("a_out".to_string()),
                    delegation: None,
                },
            ],
        }
    }

    #[test]
    fn graph_exports_are_byte_stable() {
        let plan = sample_plan();
        let j1 = export_graph_json(&plan).unwrap();
        let j2 = export_graph_json(&plan).unwrap();
        let d1 = export_graph_dot(&plan);
        let d2 = export_graph_dot(&plan);
        assert_eq!(j1, j2);
        assert_eq!(d1, d2);
        assert!(d1.contains("\"a\" -> \"b\""));
    }

    #[test]
    fn replay_ignores_timestamps_and_is_stable() {
        let events = vec![
            TraceEvent::StepStarted {
                ts_ms: 1,
                elapsed_ms: 1,
                step_id: "s1".to_string(),
                agent_id: "a".to_string(),
                provider_id: "p".to_string(),
                task_id: "t".to_string(),
                delegation: None,
            },
            TraceEvent::StepOutputChunk {
                ts_ms: 2,
                elapsed_ms: 2,
                step_id: "s1".to_string(),
                chunk_bytes: 5,
            },
            TraceEvent::StepFinished {
                ts_ms: 10,
                elapsed_ms: 10,
                step_id: "s1".to_string(),
                success: true,
                duration_ms: 9,
            },
        ];
        let normalized = normalize_trace_events(&events);
        let replay1 = replay_trace(&normalized);

        let mut events_2 = events.clone();
        if let TraceEvent::StepStarted { ts_ms, .. } = &mut events_2[0] {
            *ts_ms = 99;
        }
        let replay2 = replay_trace(&normalize_trace_events(&events_2));
        assert_eq!(replay1.step_started_order, replay2.step_started_order);
        assert_eq!(
            replay1.step_output_chunk_order,
            replay2.step_output_chunk_order
        );
        assert_eq!(replay1.step_finished_order, replay2.step_finished_order);
    }

    #[test]
    fn trace_diff_output_order_is_deterministic() {
        let left = vec![
            TraceEventNormalized::StepStarted {
                step_id: "s1".to_string(),
                agent_id: "a".to_string(),
                provider_id: "p".to_string(),
                task_id: "t".to_string(),
                delegation_json: None,
            },
            TraceEventNormalized::StepFinished {
                step_id: "s1".to_string(),
                success: true,
            },
        ];
        let right = vec![
            TraceEventNormalized::StepStarted {
                step_id: "s1".to_string(),
                agent_id: "a".to_string(),
                provider_id: "p".to_string(),
                task_id: "t".to_string(),
                delegation_json: None,
            },
            TraceEventNormalized::StepFinished {
                step_id: "s1".to_string(),
                success: false,
            },
        ];

        let d1 = diff_traces(&left, &right);
        let d2 = diff_traces(&left, &right);
        assert_eq!(d1, d2);
        assert_eq!(d1.changed_indices, vec![1]);
    }

    #[test]
    fn export_graph_sorts_nodes_and_dedupes_edges() {
        let plan = ExecutionPlan {
            workflow_kind: WorkflowKind::Concurrent,
            nodes: vec![
                ExecutionNode {
                    step_id: "b".to_string(),
                    depends_on: vec!["a".to_string(), "a".to_string()],
                    save_as: None,
                    delegation: None,
                },
                ExecutionNode {
                    step_id: "a".to_string(),
                    depends_on: vec![],
                    save_as: None,
                    delegation: None,
                },
            ],
        };
        let graph = export_graph(&plan);
        assert_eq!(graph.nodes[0].id, "a");
        assert_eq!(graph.nodes[1].id, "b");
        assert_eq!(
            graph.edges,
            vec![GraphEdge {
                from: "a".to_string(),
                to: "b".to_string()
            }]
        );
    }

    #[test]
    fn diff_plans_detects_all_surface_changes() {
        let left = ExecutionPlan {
            workflow_kind: WorkflowKind::Sequential,
            nodes: vec![
                ExecutionNode {
                    step_id: "a".to_string(),
                    depends_on: vec![],
                    save_as: Some("a_out".to_string()),
                    delegation: None,
                },
                ExecutionNode {
                    step_id: "b".to_string(),
                    depends_on: vec!["a".to_string()],
                    save_as: None,
                    delegation: None,
                },
            ],
        };
        let right = ExecutionPlan {
            workflow_kind: WorkflowKind::Sequential,
            nodes: vec![
                ExecutionNode {
                    step_id: "a".to_string(),
                    depends_on: vec!["x".to_string()],
                    save_as: None,
                    delegation: None,
                },
                ExecutionNode {
                    step_id: "c".to_string(),
                    depends_on: vec!["a".to_string()],
                    save_as: None,
                    delegation: None,
                },
            ],
        };
        let diff = diff_plans(&left, &right);
        assert_eq!(diff.added_nodes, vec!["c".to_string()]);
        assert_eq!(diff.removed_nodes, vec!["b".to_string()]);
        assert_eq!(diff.changed_dependencies, vec!["a".to_string()]);
        assert_eq!(diff.changed_save_as, vec!["a".to_string()]);
    }

    #[test]
    fn diff_traces_reports_tail_items_as_side_specific() {
        let left = vec![
            TraceEventNormalized::RunFinished { success: true },
            TraceEventNormalized::StepFinished {
                step_id: "s1".to_string(),
                success: true,
            },
        ];
        let right = vec![TraceEventNormalized::RunFinished { success: true }];
        let diff = diff_traces(&left, &right);
        assert!(diff.changed_indices.is_empty());
        assert_eq!(
            diff.left_only,
            vec!["StepFinished step=s1 success=true".to_string()]
        );
        assert!(diff.right_only.is_empty());
    }

    #[test]
    fn normalize_step_started_omits_effectively_empty_delegation() {
        let events = vec![TraceEvent::StepStarted {
            ts_ms: 1,
            elapsed_ms: 1,
            step_id: "s1".to_string(),
            agent_id: "a".to_string(),
            provider_id: "p".to_string(),
            task_id: "t".to_string(),
            delegation: Some(DelegationSpec {
                role: None,
                requires_verification: Some(false),
                escalation_target: None,
                tags: vec![],
            }),
        }];
        let normalized = normalize_trace_events(&events);
        let TraceEventNormalized::StepStarted {
            delegation_json, ..
        } = &normalized[0]
        else {
            panic!("expected StepStarted");
        };
        assert!(delegation_json.is_none());
    }

    #[test]
    fn normalize_step_started_canonicalizes_delegation_payload() {
        let events = vec![TraceEvent::StepStarted {
            ts_ms: 1,
            elapsed_ms: 1,
            step_id: "s1".to_string(),
            agent_id: "a".to_string(),
            provider_id: "p".to_string(),
            task_id: "t".to_string(),
            delegation: Some(DelegationSpec {
                role: Some("review".to_string()),
                requires_verification: Some(true),
                escalation_target: Some("human".to_string()),
                tags: vec!["z".to_string(), "a".to_string(), "z".to_string()],
            }),
        }];
        let normalized = normalize_trace_events(&events);
        let TraceEventNormalized::StepStarted {
            delegation_json, ..
        } = &normalized[0]
        else {
            panic!("expected StepStarted");
        };
        let payload = delegation_json.as_ref().expect("delegation json");
        assert!(payload.contains("\"requires_verification\":true"));
        assert!(
            payload.contains("\"tags\":[\"a\",\"z\"]"),
            "delegation should be sorted+deduped: {payload}"
        );
    }

    #[test]
    fn write_and_load_trace_artifact_round_trip() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "swarm-trace-artifact-{now}-{}.json",
            std::process::id()
        ));
        let events = vec![
            TraceEvent::RunFinished {
                ts_ms: 10,
                elapsed_ms: 10,
                success: true,
            },
            TraceEvent::StepOutputChunk {
                ts_ms: 11,
                elapsed_ms: 11,
                step_id: "s1".to_string(),
                chunk_bytes: 4,
            },
        ];
        write_trace_artifact(&path, &events).expect("write artifact");
        let loaded = load_trace_artifact(&path).expect("load artifact");
        assert_eq!(loaded, normalize_trace_events(&events));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn load_trace_artifact_invalid_json_returns_context_error() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "swarm-trace-invalid-{now}-{}.json",
            std::process::id()
        ));
        std::fs::write(&path, "{").expect("write invalid json");
        let err = load_trace_artifact(&path).expect_err("invalid json should fail");
        assert!(
            err.to_string().contains("failed parsing trace artifact"),
            "unexpected error: {err:#}"
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn format_normalized_event_covers_variants() {
        let messages = [
            format_normalized_event(&TraceEventNormalized::RunFailed {
                message: "boom".to_string(),
            }),
            format_normalized_event(&TraceEventNormalized::RunFinished { success: true }),
            format_normalized_event(&TraceEventNormalized::StepStarted {
                step_id: "s1".to_string(),
                agent_id: "a".to_string(),
                provider_id: "p".to_string(),
                task_id: "t".to_string(),
                delegation_json: Some("{\"role\":\"r\"}".to_string()),
            }),
            format_normalized_event(&TraceEventNormalized::PromptAssembled {
                step_id: "s1".to_string(),
                prompt_hash: "abc".to_string(),
            }),
            format_normalized_event(&TraceEventNormalized::StepOutputChunk {
                step_id: "s1".to_string(),
                chunk_bytes: 3,
            }),
            format_normalized_event(&TraceEventNormalized::StepFinished {
                step_id: "s1".to_string(),
                success: false,
            }),
            format_normalized_event(&TraceEventNormalized::CallEntered {
                caller_step_id: "caller".to_string(),
                callee_workflow_id: "wf".to_string(),
                namespace: "ns".to_string(),
            }),
            format_normalized_event(&TraceEventNormalized::CallExited {
                caller_step_id: "caller".to_string(),
                status: "success".to_string(),
                namespace: "ns".to_string(),
            }),
        ];
        assert!(messages.iter().any(|m| m.contains("RunFailed")));
        assert!(messages.iter().any(|m| m.contains("delegation=")));
        assert!(messages.iter().any(|m| m.contains("CallExited")));
    }
}
