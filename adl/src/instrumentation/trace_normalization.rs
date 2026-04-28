//! Mapping from runtime events to normalized replay events.

use super::TraceEventNormalized;
use crate::trace::TraceEvent;

/// Normalize raw runtime trace events into replay-safe normalized events.
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
            TraceEvent::GovernedProposalObserved {
                proposal_id,
                tool_name,
                redacted_arguments_ref,
                ..
            } => TraceEventNormalized::GovernedProposalObserved {
                proposal_id: proposal_id.clone(),
                tool_name: tool_name.clone(),
                redacted_arguments_ref: redacted_arguments_ref.clone(),
            },
            TraceEvent::GovernedProposalNormalized {
                proposal_id,
                normalized_proposal_ref,
                redacted_arguments_ref,
                ..
            } => TraceEventNormalized::GovernedProposalNormalized {
                proposal_id: proposal_id.clone(),
                normalized_proposal_ref: normalized_proposal_ref.clone(),
                redacted_arguments_ref: redacted_arguments_ref.clone(),
            },
            TraceEvent::GovernedAccConstructed {
                proposal_id,
                acc_contract_id,
                replay_posture,
                ..
            } => TraceEventNormalized::GovernedAccConstructed {
                proposal_id: proposal_id.clone(),
                acc_contract_id: acc_contract_id.clone(),
                replay_posture: replay_posture.clone(),
            },
            TraceEvent::GovernedPolicyInjected {
                proposal_id,
                policy_evidence_ref,
                outcome,
                ..
            } => TraceEventNormalized::GovernedPolicyInjected {
                proposal_id: proposal_id.clone(),
                policy_evidence_ref: policy_evidence_ref.clone(),
                outcome: outcome.clone(),
            },
            TraceEvent::GovernedVisibilityResolved {
                proposal_id,
                actor_view,
                operator_view,
                reviewer_view,
                public_report_view,
                observatory_projection,
                ..
            } => TraceEventNormalized::GovernedVisibilityResolved {
                proposal_id: proposal_id.clone(),
                actor_view: actor_view.clone(),
                operator_view: operator_view.clone(),
                reviewer_view: reviewer_view.clone(),
                public_report_view: public_report_view.clone(),
                observatory_projection: observatory_projection.clone(),
            },
            TraceEvent::GovernedFreedomGateDecided {
                proposal_id,
                candidate_id,
                decision,
                reason_code,
                boundary,
                redaction_summary,
                ..
            } => TraceEventNormalized::GovernedFreedomGateDecided {
                proposal_id: proposal_id.clone(),
                candidate_id: candidate_id.clone(),
                decision: decision.clone(),
                reason_code: reason_code.clone(),
                boundary: boundary.clone(),
                redaction_summary: redaction_summary.clone(),
            },
            TraceEvent::GovernedActionSelected {
                proposal_id,
                action_id,
                tool_name,
                adapter_id,
                evidence_refs,
                ..
            } => TraceEventNormalized::GovernedActionSelected {
                proposal_id: proposal_id.clone(),
                action_id: action_id.clone(),
                tool_name: tool_name.clone(),
                adapter_id: adapter_id.clone(),
                evidence_refs: evidence_refs.clone(),
            },
            TraceEvent::GovernedActionRejected {
                proposal_id,
                action_id,
                tool_name,
                adapter_id,
                reason_code,
                evidence_refs,
                ..
            } => TraceEventNormalized::GovernedActionRejected {
                proposal_id: proposal_id.clone(),
                action_id: action_id.clone(),
                tool_name: tool_name.clone(),
                adapter_id: adapter_id.clone(),
                reason_code: reason_code.clone(),
                evidence_refs: evidence_refs.clone(),
            },
            TraceEvent::GovernedExecutionResultRecorded {
                proposal_id,
                action_id,
                adapter_id,
                result_ref,
                evidence_refs,
                ..
            } => TraceEventNormalized::GovernedExecutionResultRecorded {
                proposal_id: proposal_id.clone(),
                action_id: action_id.clone(),
                adapter_id: adapter_id.clone(),
                result_ref: result_ref.clone(),
                evidence_refs: evidence_refs.clone(),
            },
            TraceEvent::GovernedRefusalRecorded {
                proposal_id,
                action_id,
                reason_code,
                evidence_refs,
                ..
            } => TraceEventNormalized::GovernedRefusalRecorded {
                proposal_id: proposal_id.clone(),
                action_id: action_id.clone(),
                reason_code: reason_code.clone(),
                evidence_refs: evidence_refs.clone(),
            },
            TraceEvent::GovernedRedactionDecisionRecorded {
                proposal_id,
                audience,
                surfaces,
                outcome,
                detail,
                ..
            } => TraceEventNormalized::GovernedRedactionDecisionRecorded {
                proposal_id: proposal_id.clone(),
                audience: audience.clone(),
                surfaces: surfaces.clone(),
                outcome: outcome.clone(),
                detail: detail.clone(),
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
