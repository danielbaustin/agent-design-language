use super::TraceEventNormalized;
use crate::trace::TraceEvent;

pub fn normalize_trace_events(events: &[TraceEvent]) -> Vec<TraceEventNormalized> {
    events
        .iter()
        .map(|ev| match ev {
            TraceEvent::LifecyclePhaseEntered { phase, .. } => {
                TraceEventNormalized::LifecyclePhaseEntered {
                    phase: phase.as_str().to_string(),
                }
            }
            TraceEvent::ExecutionBoundaryCrossed {
                boundary, state, ..
            } => TraceEventNormalized::ExecutionBoundaryCrossed {
                boundary: boundary.as_str().to_string(),
                state: state.clone(),
            },
            TraceEvent::SchedulerPolicy {
                max_concurrency,
                source,
                ..
            } => TraceEventNormalized::SchedulerPolicy {
                max_concurrency: *max_concurrency,
                source: source.clone(),
            },
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
            TraceEvent::DelegationRequested {
                delegation_id,
                step_id,
                action_kind,
                target_id,
                ..
            } => TraceEventNormalized::DelegationRequested {
                delegation_id: delegation_id.clone(),
                step_id: step_id.clone(),
                action_kind: action_kind.clone(),
                target_id: target_id.clone(),
            },
            TraceEvent::DelegationPolicyEvaluated {
                delegation_id,
                step_id,
                action_kind,
                target_id,
                decision,
                rule_id,
                ..
            } => TraceEventNormalized::DelegationPolicyEvaluated {
                delegation_id: delegation_id.clone(),
                step_id: step_id.clone(),
                action_kind: action_kind.clone(),
                target_id: target_id.clone(),
                decision: decision.clone(),
                rule_id: rule_id.clone(),
            },
            TraceEvent::DelegationApproved {
                delegation_id,
                step_id,
                ..
            } => TraceEventNormalized::DelegationApproved {
                delegation_id: delegation_id.clone(),
                step_id: step_id.clone(),
            },
            TraceEvent::DelegationDenied {
                delegation_id,
                step_id,
                action_kind,
                target_id,
                rule_id,
                ..
            } => TraceEventNormalized::DelegationDenied {
                delegation_id: delegation_id.clone(),
                step_id: step_id.clone(),
                action_kind: action_kind.clone(),
                target_id: target_id.clone(),
                rule_id: rule_id.clone(),
            },
            TraceEvent::DelegationDispatched {
                delegation_id,
                step_id,
                action_kind,
                target_id,
                ..
            } => TraceEventNormalized::DelegationDispatched {
                delegation_id: delegation_id.clone(),
                step_id: step_id.clone(),
                action_kind: action_kind.clone(),
                target_id: target_id.clone(),
            },
            TraceEvent::DelegationResultReceived {
                delegation_id,
                step_id,
                success,
                output_bytes,
                ..
            } => TraceEventNormalized::DelegationResultReceived {
                delegation_id: delegation_id.clone(),
                step_id: step_id.clone(),
                success: *success,
                output_bytes: *output_bytes,
            },
            TraceEvent::DelegationCompleted {
                delegation_id,
                step_id,
                outcome,
                ..
            } => TraceEventNormalized::DelegationCompleted {
                delegation_id: delegation_id.clone(),
                step_id: step_id.clone(),
                outcome: outcome.clone(),
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
