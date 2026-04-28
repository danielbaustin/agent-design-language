//! Trace instrumentation and replay normalization helpers.
//!
//! This module exposes graph exports, event formatting, and trace normalization
//! utilities used by replay, diagnostics, and CI evidence generation.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::execution_plan::ExecutionPlan;
use crate::trace::TraceEvent;

mod graph;
mod trace_formatting;
mod trace_normalization;

pub use graph::{
    export_graph, export_graph_dot, export_graph_json, GraphEdge, GraphExport, GraphNode,
};
pub use trace_formatting::format_normalized_event;
pub use trace_normalization::normalize_trace_events;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
/// Stable, normalized event shape consumed by replay and diff tooling.
pub enum TraceEventNormalized {
    LifecyclePhaseEntered {
        phase: String,
    },
    ExecutionBoundaryCrossed {
        boundary: String,
        state: String,
    },
    GovernedProposalObserved {
        proposal_id: String,
        tool_name: String,
        redacted_arguments_ref: String,
    },
    GovernedProposalNormalized {
        proposal_id: String,
        normalized_proposal_ref: String,
        redacted_arguments_ref: String,
    },
    GovernedAccConstructed {
        proposal_id: String,
        acc_contract_id: String,
        replay_posture: String,
    },
    GovernedPolicyInjected {
        proposal_id: String,
        policy_evidence_ref: String,
        outcome: String,
    },
    GovernedVisibilityResolved {
        proposal_id: String,
        actor_view: String,
        operator_view: String,
        reviewer_view: String,
        public_report_view: String,
        observatory_projection: String,
    },
    GovernedFreedomGateDecided {
        proposal_id: String,
        candidate_id: String,
        decision: String,
        reason_code: String,
        boundary: String,
        redaction_summary: String,
    },
    GovernedActionSelected {
        proposal_id: String,
        action_id: String,
        tool_name: String,
        adapter_id: String,
        evidence_refs: Vec<String>,
    },
    GovernedActionRejected {
        proposal_id: String,
        action_id: String,
        tool_name: String,
        adapter_id: String,
        reason_code: String,
        evidence_refs: Vec<String>,
    },
    GovernedExecutionResultRecorded {
        proposal_id: String,
        action_id: String,
        adapter_id: String,
        result_ref: String,
        evidence_refs: Vec<String>,
    },
    GovernedRefusalRecorded {
        proposal_id: String,
        action_id: String,
        reason_code: String,
        evidence_refs: Vec<String>,
    },
    GovernedRedactionDecisionRecorded {
        proposal_id: String,
        audience: String,
        surfaces: Vec<String>,
        outcome: String,
        detail: Option<String>,
    },
    SchedulerPolicy {
        max_concurrency: usize,
        source: String,
    },
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
    DelegationRequested {
        delegation_id: String,
        step_id: String,
        action_kind: String,
        target_id: String,
    },
    DelegationPolicyEvaluated {
        delegation_id: String,
        step_id: String,
        action_kind: String,
        target_id: String,
        decision: String,
        rule_id: Option<String>,
    },
    DelegationApproved {
        delegation_id: String,
        step_id: String,
    },
    DelegationDenied {
        delegation_id: String,
        step_id: String,
        action_kind: String,
        target_id: String,
        rule_id: Option<String>,
    },
    DelegationDispatched {
        delegation_id: String,
        step_id: String,
        action_kind: String,
        target_id: String,
    },
    DelegationResultReceived {
        delegation_id: String,
        step_id: String,
        success: bool,
        output_bytes: usize,
    },
    DelegationCompleted {
        delegation_id: String,
        step_id: String,
        outcome: String,
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
/// Replay projection of step lifecycle ordering.
pub struct TraceReplay {
    pub step_started_order: Vec<String>,
    pub step_finished_order: Vec<String>,
    pub step_output_chunk_order: Vec<String>,
    pub events: Vec<TraceEventNormalized>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Diff of two execution plan or trace graphs.
pub struct PlanDiff {
    pub added_nodes: Vec<String>,
    pub removed_nodes: Vec<String>,
    pub changed_dependencies: Vec<String>,
    pub changed_save_as: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Diff of two normalized trace traces.
pub struct TraceDiff {
    pub changed_indices: Vec<usize>,
    pub left_only: Vec<String>,
    pub right_only: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Stable replay-surface schema/contract failures.
pub struct ReplayInvariantError {
    pub code: &'static str,
    pub message: String,
}

impl ReplayInvariantError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            code: "REPLAY_INVARIANT_VIOLATION",
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ReplayInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ReplayInvariantError {}

pub const ACTIVATION_LOG_VERSION_V1: u32 = 1;
pub const ACTIVATION_LOG_VERSION_V2: u32 = 2;

fn normalized_event_requires_activation_log_v2(event: &TraceEventNormalized) -> bool {
    matches!(
        event,
        TraceEventNormalized::GovernedProposalObserved { .. }
            | TraceEventNormalized::GovernedProposalNormalized { .. }
            | TraceEventNormalized::GovernedAccConstructed { .. }
            | TraceEventNormalized::GovernedPolicyInjected { .. }
            | TraceEventNormalized::GovernedVisibilityResolved { .. }
            | TraceEventNormalized::GovernedFreedomGateDecided { .. }
            | TraceEventNormalized::GovernedActionSelected { .. }
            | TraceEventNormalized::GovernedActionRejected { .. }
            | TraceEventNormalized::GovernedExecutionResultRecorded { .. }
            | TraceEventNormalized::GovernedRefusalRecorded { .. }
            | TraceEventNormalized::GovernedRedactionDecisionRecorded { .. }
    )
}

fn activation_log_version_for_events(events: &[TraceEventNormalized]) -> u32 {
    if events
        .iter()
        .any(normalized_event_requires_activation_log_v2)
    {
        ACTIVATION_LOG_VERSION_V2
    } else {
        ACTIVATION_LOG_VERSION_V1
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct ActivationLogStableIds {
    step_id: String,
    delegation_id: String,
    run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct ActivationLogArtifact {
    activation_log_version: u32,
    ordering: String,
    stable_ids: ActivationLogStableIds,
    events: Vec<TraceEventNormalized>,
}

/// Serialize a raw trace into a canonical activation-log artifact on disk.
pub fn write_trace_artifact(path: &Path, events: &[TraceEvent]) -> Result<()> {
    let normalized = normalize_trace_events(events);
    let activation_log_version = activation_log_version_for_events(&normalized);
    let artifact = ActivationLogArtifact {
        activation_log_version,
        ordering: "append_only_emission_order".to_string(),
        stable_ids: ActivationLogStableIds {
            step_id: "stable within resolved execution plan".to_string(),
            delegation_id: "deterministic per run: del-<counter>".to_string(),
            run_id: "run-scoped identifier; not replay-stable across independent runs".to_string(),
        },
        events: normalized,
    };
    let body = serde_json::to_vec_pretty(&artifact).context("serialize trace artifact")?;
    fs::write(path, body)
        .with_context(|| format!("failed writing trace artifact '{}'", path.display()))
}

/// Classify a trace invariant error into a replay-facing failure code.
pub fn stable_failure_kind(err: &anyhow::Error) -> Option<&'static str> {
    for cause in err.chain() {
        if cause.downcast_ref::<ReplayInvariantError>().is_some() {
            return Some("replay_invariant_violation");
        }
    }
    None
}

/// Load a normalized activation-log trace artifact for replay or validation.
pub fn load_trace_artifact(path: &Path) -> Result<Vec<TraceEventNormalized>> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed reading trace artifact '{}'", path.display()))?;

    let parsed: serde_json::Value = serde_json::from_str(&raw).map_err(|err| {
        ReplayInvariantError::new(format!(
            "failed parsing trace artifact '{}' as json: {err}",
            path.display()
        ))
    })?;

    match parsed {
        // Canonical v0.75+ wrapper object.
        serde_json::Value::Object(obj) => {
            let artifact: ActivationLogArtifact = serde_json::from_value(serde_json::Value::Object(obj))
                .map_err(|err| {
                    ReplayInvariantError::new(format!(
                        "failed parsing trace artifact '{}' as activation log wrapper: {err}",
                        path.display()
                    ))
                })?;
            if artifact.activation_log_version != ACTIVATION_LOG_VERSION_V1
                && artifact.activation_log_version != ACTIVATION_LOG_VERSION_V2
            {
                return Err(ReplayInvariantError::new(format!(
                    "unsupported activation_log_version {} in '{}'; expected {} or {}",
                    artifact.activation_log_version,
                    path.display(),
                    ACTIVATION_LOG_VERSION_V1,
                    ACTIVATION_LOG_VERSION_V2
                ))
                .into());
            }
            if artifact.ordering != "append_only_emission_order" {
                return Err(ReplayInvariantError::new(format!(
                    "unsupported activation log ordering '{}' in '{}'",
                    artifact.ordering,
                    path.display()
                ))
                .into());
            }
            if artifact.activation_log_version == ACTIVATION_LOG_VERSION_V1
                && artifact
                    .events
                    .iter()
                    .any(normalized_event_requires_activation_log_v2)
            {
                return Err(ReplayInvariantError::new(format!(
                    "activation_log_version {} in '{}' does not support governed normalized events",
                    artifact.activation_log_version,
                    path.display()
                ))
                .into());
            }
            Ok(artifact.events)
        }
        // Backward compatibility: pre-v0.75 artifacts stored a bare normalized-event array.
        serde_json::Value::Array(arr) => {
            serde_json::from_value(serde_json::Value::Array(arr)).map_err(|err| {
                ReplayInvariantError::new(format!(
                    "failed parsing trace artifact '{}' as legacy activation log event array: {err}",
                    path.display()
                ))
                .into()
            })
        }
        _ => Err(ReplayInvariantError::new(format!(
            "failed parsing trace artifact '{}': expected activation log wrapper object or legacy event array",
            path.display()
        ))
        .into()),
    }
}

/// Derive execution-order summaries from normalized event streams.
pub fn replay_trace(events: &[TraceEventNormalized]) -> TraceReplay {
    let mut step_started_order = Vec::new();
    let mut step_finished_order = Vec::new();
    let mut step_output_chunk_order = Vec::new();

    for ev in events {
        match ev {
            TraceEventNormalized::LifecyclePhaseEntered { .. }
            | TraceEventNormalized::ExecutionBoundaryCrossed { .. }
            | TraceEventNormalized::GovernedProposalObserved { .. }
            | TraceEventNormalized::GovernedProposalNormalized { .. }
            | TraceEventNormalized::GovernedAccConstructed { .. }
            | TraceEventNormalized::GovernedPolicyInjected { .. }
            | TraceEventNormalized::GovernedVisibilityResolved { .. }
            | TraceEventNormalized::GovernedFreedomGateDecided { .. }
            | TraceEventNormalized::GovernedActionSelected { .. }
            | TraceEventNormalized::GovernedActionRejected { .. }
            | TraceEventNormalized::GovernedExecutionResultRecorded { .. }
            | TraceEventNormalized::GovernedRefusalRecorded { .. }
            | TraceEventNormalized::GovernedRedactionDecisionRecorded { .. } => {}
            TraceEventNormalized::StepStarted { step_id, .. } => {
                step_started_order.push(step_id.clone())
            }
            TraceEventNormalized::StepFinished { step_id, .. } => {
                step_finished_order.push(step_id.clone())
            }
            TraceEventNormalized::StepOutputChunk { step_id, .. } => {
                step_output_chunk_order.push(step_id.clone())
            }
            TraceEventNormalized::DelegationRequested { .. }
            | TraceEventNormalized::DelegationPolicyEvaluated { .. }
            | TraceEventNormalized::DelegationApproved { .. }
            | TraceEventNormalized::DelegationDenied { .. }
            | TraceEventNormalized::DelegationDispatched { .. }
            | TraceEventNormalized::DelegationResultReceived { .. }
            | TraceEventNormalized::DelegationCompleted { .. }
            | TraceEventNormalized::SchedulerPolicy { .. }
            | TraceEventNormalized::RunFailed { .. }
            | TraceEventNormalized::RunFinished { .. }
            | TraceEventNormalized::PromptAssembled { .. }
            | TraceEventNormalized::CallEntered { .. }
            | TraceEventNormalized::CallExited { .. } => {}
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
            "adl-trace-artifact-{now}-{}.json",
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
    fn write_trace_artifact_emits_frozen_activation_log_wrapper() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "adl-trace-wrapper-{now}-{}.json",
            std::process::id()
        ));
        let events = vec![TraceEvent::RunFinished {
            ts_ms: 10,
            elapsed_ms: 10,
            success: true,
        }];
        write_trace_artifact(&path, &events).expect("write artifact");
        let raw = std::fs::read_to_string(&path).expect("read artifact");
        let parsed: serde_json::Value = serde_json::from_str(&raw).expect("parse artifact");
        assert_eq!(
            parsed["activation_log_version"].as_u64(),
            Some(ACTIVATION_LOG_VERSION_V1 as u64)
        );
        assert_eq!(
            parsed["ordering"].as_str(),
            Some("append_only_emission_order")
        );
        assert!(parsed["stable_ids"].is_object());
        assert!(parsed["events"].is_array());
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn load_trace_artifact_rejects_missing_required_wrapper_fields() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "adl-trace-missing-required-{now}-{}.json",
            std::process::id()
        ));
        let body = serde_json::json!({
            "activation_log_version": ACTIVATION_LOG_VERSION_V1,
            "stable_ids": {
                "step_id": "stable within resolved execution plan",
                "delegation_id": "deterministic per run: del-<counter>",
                "run_id": "run-scoped identifier; not replay-stable across independent runs"
            },
            "events": []
        });
        std::fs::write(
            &path,
            serde_json::to_vec_pretty(&body).expect("serialize missing ordering"),
        )
        .expect("write artifact");
        let err = load_trace_artifact(&path).expect_err("missing ordering should fail");
        assert!(
            err.to_string().contains("failed parsing trace artifact"),
            "unexpected error: {err:#}"
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn load_trace_artifact_rejects_version_mismatch() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "adl-trace-version-mismatch-{now}-{}.json",
            std::process::id()
        ));
        let body = serde_json::json!({
            "activation_log_version": ACTIVATION_LOG_VERSION_V2 + 1,
            "ordering": "append_only_emission_order",
            "stable_ids": {
                "step_id": "stable within resolved execution plan",
                "delegation_id": "deterministic per run: del-<counter>",
                "run_id": "run-scoped identifier; not replay-stable across independent runs"
            },
            "events": []
        });
        std::fs::write(
            &path,
            serde_json::to_vec_pretty(&body).expect("serialize mismatched version"),
        )
        .expect("write artifact");
        let err = load_trace_artifact(&path).expect_err("version mismatch should fail");
        assert!(
            err.to_string()
                .contains("unsupported activation_log_version"),
            "unexpected error: {err:#}"
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn load_trace_artifact_rejects_ordering_mismatch() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "adl-trace-ordering-mismatch-{now}-{}.json",
            std::process::id()
        ));
        let body = serde_json::json!({
            "activation_log_version": ACTIVATION_LOG_VERSION_V1,
            "ordering": "unordered",
            "stable_ids": {
                "step_id": "stable within resolved execution plan",
                "delegation_id": "deterministic per run: del-<counter>",
                "run_id": "run-scoped identifier; not replay-stable across independent runs"
            },
            "events": []
        });
        std::fs::write(
            &path,
            serde_json::to_vec_pretty(&body).expect("serialize mismatched ordering"),
        )
        .expect("write artifact");
        let err = load_trace_artifact(&path).expect_err("ordering mismatch should fail");
        assert!(
            err.to_string()
                .contains("unsupported activation log ordering"),
            "unexpected error: {err:#}"
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn load_trace_artifact_preserves_append_ordering_in_events() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "adl-trace-order-preserved-{now}-{}.json",
            std::process::id()
        ));
        let events = vec![
            TraceEvent::StepFinished {
                ts_ms: 20,
                elapsed_ms: 20,
                step_id: "z-step".to_string(),
                success: true,
                duration_ms: 0,
            },
            TraceEvent::StepFinished {
                ts_ms: 21,
                elapsed_ms: 21,
                step_id: "a-step".to_string(),
                success: false,
                duration_ms: 0,
            },
            TraceEvent::RunFinished {
                ts_ms: 22,
                elapsed_ms: 22,
                success: false,
            },
        ];
        write_trace_artifact(&path, &events).expect("write artifact");
        let loaded = load_trace_artifact(&path).expect("load artifact");
        assert_eq!(
            loaded,
            normalize_trace_events(&events),
            "activation log must preserve append order (no implicit re-sorting)"
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn write_trace_artifact_uses_v2_for_governed_events() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "adl-trace-governed-wrapper-{now}-{}.json",
            std::process::id()
        ));
        let events = vec![TraceEvent::GovernedProposalObserved {
            ts_ms: 10,
            elapsed_ms: 10,
            proposal_id: "proposal.safe-read".to_string(),
            tool_name: "fixture.safe_read".to_string(),
            redacted_arguments_ref: "artifacts/run/governed/proposal_arguments.redacted.json"
                .to_string(),
        }];
        write_trace_artifact(&path, &events).expect("write artifact");
        let raw = std::fs::read_to_string(&path).expect("read artifact");
        let parsed: serde_json::Value = serde_json::from_str(&raw).expect("parse artifact");
        assert_eq!(
            parsed["activation_log_version"].as_u64(),
            Some(ACTIVATION_LOG_VERSION_V2 as u64)
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn load_trace_artifact_rejects_governed_events_under_v1_wrapper() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "adl-trace-governed-v1-{now}-{}.json",
            std::process::id()
        ));
        let body = serde_json::json!({
            "activation_log_version": ACTIVATION_LOG_VERSION_V1,
            "ordering": "append_only_emission_order",
            "stable_ids": {
                "step_id": "stable within resolved execution plan",
                "delegation_id": "deterministic per run: del-<counter>",
                "run_id": "run-scoped identifier; not replay-stable across independent runs"
            },
            "events": [{
                "kind": "GovernedProposalObserved",
                "proposal_id": "proposal.safe-read",
                "tool_name": "fixture.safe_read",
                "redacted_arguments_ref": "artifacts/run/governed/proposal_arguments.redacted.json"
            }]
        });
        std::fs::write(
            &path,
            serde_json::to_vec_pretty(&body).expect("serialize governed v1 body"),
        )
        .expect("write artifact");
        let err = load_trace_artifact(&path).expect_err("governed v1 wrapper should fail");
        assert!(
            err.to_string()
                .contains("does not support governed normalized events"),
            "unexpected error: {err:#}"
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn load_trace_artifact_invalid_json_returns_context_error() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "adl-trace-invalid-{now}-{}.json",
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
    fn normalize_delegation_events_preserves_optional_rule_id_and_order() {
        let events = vec![
            TraceEvent::DelegationRequested {
                ts_ms: 1,
                elapsed_ms: 1,
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
                action_kind: "provider_call".to_string(),
                target_id: "local".to_string(),
            },
            TraceEvent::DelegationPolicyEvaluated {
                ts_ms: 2,
                elapsed_ms: 2,
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
                action_kind: "provider_call".to_string(),
                target_id: "local".to_string(),
                decision: "allowed".to_string(),
                rule_id: None,
            },
            TraceEvent::DelegationDispatched {
                ts_ms: 3,
                elapsed_ms: 3,
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
                action_kind: "provider_call".to_string(),
                target_id: "local".to_string(),
            },
        ];
        let normalized = normalize_trace_events(&events);
        let json = serde_json::to_string_pretty(&normalized).expect("serialize");
        assert!(json.contains("\"kind\": \"DelegationRequested\""));
        assert!(json.contains("\"delegation_id\": \"del-1\""));
        assert!(json.contains("\"rule_id\": null"));
        assert_eq!(
            normalized.iter().map(format_normalized_event).collect::<Vec<_>>(),
            vec![
                "DelegationRequested delegation_id=del-1 step=s1 action=provider_call target=local".to_string(),
                "DelegationPolicyEvaluated delegation_id=del-1 step=s1 action=provider_call target=local decision=allowed".to_string(),
                "DelegationDispatched delegation_id=del-1 step=s1 action=provider_call target=local".to_string(),
            ]
        );
    }

    #[test]
    fn format_normalized_event_covers_variants() {
        let messages = [
            format_normalized_event(&TraceEventNormalized::DelegationPolicyEvaluated {
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
                action_kind: "provider_call".to_string(),
                target_id: "local".to_string(),
                decision: "denied".to_string(),
                rule_id: Some("deny-local".to_string()),
            }),
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
        assert!(messages
            .iter()
            .any(|m| m.contains("DelegationPolicyEvaluated")));
        assert!(messages.iter().any(|m| m.contains("delegation=")));
        assert!(messages.iter().any(|m| m.contains("CallExited")));
    }
}
