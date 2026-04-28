//! Deterministic formatting utilities for normalized instrumentation events.

use super::TraceEventNormalized;

/// Render one normalized event into a stable single-line string.
pub fn format_normalized_event(ev: &TraceEventNormalized) -> String {
    match ev {
        TraceEventNormalized::LifecyclePhaseEntered { phase } => {
            format!("LifecyclePhaseEntered phase={phase}")
        }
        TraceEventNormalized::ExecutionBoundaryCrossed { boundary, state } => {
            format!("ExecutionBoundaryCrossed boundary={boundary} state={state}")
        }
        TraceEventNormalized::GovernedProposalObserved {
            proposal_id,
            tool_name,
            redacted_arguments_ref,
        } => {
            format!(
                "GovernedProposalObserved proposal_id={proposal_id} tool={tool_name} arguments_ref={redacted_arguments_ref}"
            )
        }
        TraceEventNormalized::GovernedProposalNormalized {
            proposal_id,
            normalized_proposal_ref,
            redacted_arguments_ref,
        } => {
            format!(
                "GovernedProposalNormalized proposal_id={proposal_id} normalized_ref={normalized_proposal_ref} arguments_ref={redacted_arguments_ref}"
            )
        }
        TraceEventNormalized::GovernedAccConstructed {
            proposal_id,
            acc_contract_id,
            replay_posture,
        } => {
            format!(
                "GovernedAccConstructed proposal_id={proposal_id} acc={acc_contract_id} replay_posture={replay_posture}"
            )
        }
        TraceEventNormalized::GovernedPolicyInjected {
            proposal_id,
            policy_evidence_ref,
            outcome,
        } => {
            format!(
                "GovernedPolicyInjected proposal_id={proposal_id} policy_ref={policy_evidence_ref} outcome={outcome}"
            )
        }
        TraceEventNormalized::GovernedVisibilityResolved {
            proposal_id,
            actor_view,
            operator_view,
            reviewer_view,
            public_report_view,
            observatory_projection,
        } => {
            format!(
                "GovernedVisibilityResolved proposal_id={proposal_id} actor_view={actor_view} operator_view={operator_view} reviewer_view={reviewer_view} public_view={public_report_view} observatory={observatory_projection}"
            )
        }
        TraceEventNormalized::GovernedFreedomGateDecided {
            proposal_id,
            candidate_id,
            decision,
            reason_code,
            boundary,
            redaction_summary,
        } => {
            format!(
                "GovernedFreedomGateDecided proposal_id={proposal_id} candidate={candidate_id} decision={decision} reason={reason_code} boundary={boundary} redaction={redaction_summary}"
            )
        }
        TraceEventNormalized::GovernedActionSelected {
            proposal_id,
            action_id,
            tool_name,
            adapter_id,
            evidence_refs,
        } => {
            format!(
                "GovernedActionSelected proposal_id={proposal_id} action_id={action_id} tool={tool_name} adapter={adapter_id} evidence_count={}",
                evidence_refs.len()
            )
        }
        TraceEventNormalized::GovernedActionRejected {
            proposal_id,
            action_id,
            tool_name,
            adapter_id,
            reason_code,
            evidence_refs,
        } => {
            format!(
                "GovernedActionRejected proposal_id={proposal_id} action_id={action_id} tool={tool_name} adapter={adapter_id} reason={reason_code} evidence_count={}",
                evidence_refs.len()
            )
        }
        TraceEventNormalized::GovernedExecutionResultRecorded {
            proposal_id,
            action_id,
            adapter_id,
            result_ref,
            evidence_refs,
        } => {
            format!(
                "GovernedExecutionResultRecorded proposal_id={proposal_id} action_id={action_id} adapter={adapter_id} result_ref={result_ref} evidence_count={}",
                evidence_refs.len()
            )
        }
        TraceEventNormalized::GovernedRefusalRecorded {
            proposal_id,
            action_id,
            reason_code,
            evidence_refs,
        } => {
            format!(
                "GovernedRefusalRecorded proposal_id={proposal_id} action_id={action_id} reason={reason_code} evidence_count={}",
                evidence_refs.len()
            )
        }
        TraceEventNormalized::GovernedRedactionDecisionRecorded {
            proposal_id,
            audience,
            surfaces,
            outcome,
            detail,
        } => {
            let surfaces = surfaces.join(",");
            let base = format!(
                "GovernedRedactionDecisionRecorded proposal_id={proposal_id} audience={audience} surfaces={surfaces} outcome={outcome}"
            );
            if let Some(detail) = detail {
                format!("{base} detail={detail}")
            } else {
                base
            }
        }
        TraceEventNormalized::SchedulerPolicy {
            max_concurrency,
            source,
        } => format!("SchedulerPolicy max_concurrency={max_concurrency} source={source}"),
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
        TraceEventNormalized::DelegationRequested {
            delegation_id,
            step_id,
            action_kind,
            target_id,
        } => format!(
            "DelegationRequested delegation_id={delegation_id} step={step_id} action={action_kind} target={target_id}"
        ),
        TraceEventNormalized::DelegationPolicyEvaluated {
            delegation_id,
            step_id,
            action_kind,
            target_id,
            decision,
            rule_id,
        } => {
            let base = format!(
                "DelegationPolicyEvaluated delegation_id={delegation_id} step={step_id} action={action_kind} target={target_id} decision={decision}"
            );
            if let Some(rule_id) = rule_id {
                format!("{base} rule_id={rule_id}")
            } else {
                base
            }
        }
        TraceEventNormalized::DelegationApproved {
            delegation_id,
            step_id,
        } => format!("DelegationApproved delegation_id={delegation_id} step={step_id}"),
        TraceEventNormalized::DelegationDenied {
            delegation_id,
            step_id,
            action_kind,
            target_id,
            rule_id,
        } => {
            let base = format!(
                "DelegationDenied delegation_id={delegation_id} step={step_id} action={action_kind} target={target_id}"
            );
            if let Some(rule_id) = rule_id {
                format!("{base} rule_id={rule_id}")
            } else {
                base
            }
        }
        TraceEventNormalized::DelegationDispatched {
            delegation_id,
            step_id,
            action_kind,
            target_id,
        } => format!(
            "DelegationDispatched delegation_id={delegation_id} step={step_id} action={action_kind} target={target_id}"
        ),
        TraceEventNormalized::DelegationResultReceived {
            delegation_id,
            step_id,
            success,
            output_bytes,
        } => format!(
            "DelegationResultReceived delegation_id={delegation_id} step={step_id} success={success} bytes={output_bytes}"
        ),
        TraceEventNormalized::DelegationCompleted {
            delegation_id,
            step_id,
            outcome,
        } => format!(
            "DelegationCompleted delegation_id={delegation_id} step={step_id} outcome={outcome}"
        ),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_normalized_event_covers_all_formatting_branches() {
        let cases = vec![
            (
                TraceEventNormalized::LifecyclePhaseEntered {
                    phase: "plan".to_string(),
                },
                "LifecyclePhaseEntered phase=plan",
            ),
            (
                TraceEventNormalized::ExecutionBoundaryCrossed {
                    boundary: "scheduler".to_string(),
                    state: "entered".to_string(),
                },
                "ExecutionBoundaryCrossed boundary=scheduler state=entered",
            ),
            (
                TraceEventNormalized::GovernedProposalObserved {
                    proposal_id: "proposal.safe-read".to_string(),
                    tool_name: "fixture.safe_read".to_string(),
                    redacted_arguments_ref: "artifacts/run/governed/proposal_arguments.redacted.json"
                        .to_string(),
                },
                "GovernedProposalObserved proposal_id=proposal.safe-read tool=fixture.safe_read arguments_ref=artifacts/run/governed/proposal_arguments.redacted.json",
            ),
            (
                TraceEventNormalized::GovernedProposalNormalized {
                    proposal_id: "proposal.safe-read".to_string(),
                    normalized_proposal_ref: "normalized.proposal.safe-read".to_string(),
                    redacted_arguments_ref: "artifacts/run/governed/proposal_arguments.redacted.json"
                        .to_string(),
                },
                "GovernedProposalNormalized proposal_id=proposal.safe-read normalized_ref=normalized.proposal.safe-read arguments_ref=artifacts/run/governed/proposal_arguments.redacted.json",
            ),
            (
                TraceEventNormalized::SchedulerPolicy {
                    max_concurrency: 2,
                    source: "workflow".to_string(),
                },
                "SchedulerPolicy max_concurrency=2 source=workflow",
            ),
            (
                TraceEventNormalized::RunFailed {
                    message: "boom".to_string(),
                },
                "RunFailed message=boom",
            ),
            (
                TraceEventNormalized::RunFinished { success: true },
                "RunFinished success=true",
            ),
            (
                TraceEventNormalized::StepStarted {
                    step_id: "s1".to_string(),
                    agent_id: "a".to_string(),
                    provider_id: "p".to_string(),
                    task_id: "t".to_string(),
                    delegation_json: None,
                },
                "StepStarted step=s1 agent=a provider=p task=t",
            ),
            (
                TraceEventNormalized::StepStarted {
                    step_id: "s1".to_string(),
                    agent_id: "a".to_string(),
                    provider_id: "p".to_string(),
                    task_id: "t".to_string(),
                    delegation_json: Some("{\"role\":\"r\"}".to_string()),
                },
                "StepStarted step=s1 agent=a provider=p task=t delegation={\"role\":\"r\"}",
            ),
            (
                TraceEventNormalized::PromptAssembled {
                    step_id: "s1".to_string(),
                    prompt_hash: "abc".to_string(),
                },
                "PromptAssembled step=s1 hash=abc",
            ),
            (
                TraceEventNormalized::StepOutputChunk {
                    step_id: "s1".to_string(),
                    chunk_bytes: 3,
                },
                "StepOutputChunk step=s1 bytes=3",
            ),
            (
                TraceEventNormalized::DelegationRequested {
                    delegation_id: "del-1".to_string(),
                    step_id: "s1".to_string(),
                    action_kind: "provider_call".to_string(),
                    target_id: "local".to_string(),
                },
                "DelegationRequested delegation_id=del-1 step=s1 action=provider_call target=local",
            ),
            (
                TraceEventNormalized::DelegationPolicyEvaluated {
                    delegation_id: "del-1".to_string(),
                    step_id: "s1".to_string(),
                    action_kind: "provider_call".to_string(),
                    target_id: "local".to_string(),
                    decision: "allowed".to_string(),
                    rule_id: None,
                },
                "DelegationPolicyEvaluated delegation_id=del-1 step=s1 action=provider_call target=local decision=allowed",
            ),
            (
                TraceEventNormalized::DelegationPolicyEvaluated {
                    delegation_id: "del-1".to_string(),
                    step_id: "s1".to_string(),
                    action_kind: "provider_call".to_string(),
                    target_id: "local".to_string(),
                    decision: "denied".to_string(),
                    rule_id: Some("deny-local".to_string()),
                },
                "DelegationPolicyEvaluated delegation_id=del-1 step=s1 action=provider_call target=local decision=denied rule_id=deny-local",
            ),
            (
                TraceEventNormalized::DelegationApproved {
                    delegation_id: "del-1".to_string(),
                    step_id: "s1".to_string(),
                },
                "DelegationApproved delegation_id=del-1 step=s1",
            ),
            (
                TraceEventNormalized::DelegationDenied {
                    delegation_id: "del-1".to_string(),
                    step_id: "s1".to_string(),
                    action_kind: "provider_call".to_string(),
                    target_id: "local".to_string(),
                    rule_id: None,
                },
                "DelegationDenied delegation_id=del-1 step=s1 action=provider_call target=local",
            ),
            (
                TraceEventNormalized::DelegationDenied {
                    delegation_id: "del-1".to_string(),
                    step_id: "s1".to_string(),
                    action_kind: "provider_call".to_string(),
                    target_id: "local".to_string(),
                    rule_id: Some("deny-local".to_string()),
                },
                "DelegationDenied delegation_id=del-1 step=s1 action=provider_call target=local rule_id=deny-local",
            ),
            (
                TraceEventNormalized::DelegationDispatched {
                    delegation_id: "del-1".to_string(),
                    step_id: "s1".to_string(),
                    action_kind: "provider_call".to_string(),
                    target_id: "local".to_string(),
                },
                "DelegationDispatched delegation_id=del-1 step=s1 action=provider_call target=local",
            ),
            (
                TraceEventNormalized::DelegationResultReceived {
                    delegation_id: "del-1".to_string(),
                    step_id: "s1".to_string(),
                    success: true,
                    output_bytes: 7,
                },
                "DelegationResultReceived delegation_id=del-1 step=s1 success=true bytes=7",
            ),
            (
                TraceEventNormalized::DelegationCompleted {
                    delegation_id: "del-1".to_string(),
                    step_id: "s1".to_string(),
                    outcome: "success".to_string(),
                },
                "DelegationCompleted delegation_id=del-1 step=s1 outcome=success",
            ),
            (
                TraceEventNormalized::StepFinished {
                    step_id: "s1".to_string(),
                    success: false,
                },
                "StepFinished step=s1 success=false",
            ),
            (
                TraceEventNormalized::CallEntered {
                    caller_step_id: "caller".to_string(),
                    callee_workflow_id: "wf".to_string(),
                    namespace: "ns".to_string(),
                },
                "CallEntered caller_step=caller callee_workflow=wf namespace=ns",
            ),
            (
                TraceEventNormalized::CallExited {
                    caller_step_id: "caller".to_string(),
                    status: "success".to_string(),
                    namespace: "ns".to_string(),
                },
                "CallExited caller_step=caller status=success namespace=ns",
            ),
        ];

        for (event, expected) in cases {
            assert_eq!(format_normalized_event(&event), expected);
        }
    }
}
