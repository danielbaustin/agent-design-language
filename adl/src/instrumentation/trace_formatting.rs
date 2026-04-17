use super::TraceEventNormalized;

pub fn format_normalized_event(ev: &TraceEventNormalized) -> String {
    match ev {
        TraceEventNormalized::LifecyclePhaseEntered { phase } => {
            format!("LifecyclePhaseEntered phase={phase}")
        }
        TraceEventNormalized::ExecutionBoundaryCrossed { boundary, state } => {
            format!("ExecutionBoundaryCrossed boundary={boundary} state={state}")
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
