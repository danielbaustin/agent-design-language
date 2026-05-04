use super::{Trace, TraceEvent};

use crate::adl::DelegationSpec;

impl TraceEvent {
    pub fn summarize(&self) -> String {
        match self {
            TraceEvent::LifecyclePhaseEntered {
                ts_ms,
                elapsed_ms,
                phase,
            } => format!(
                "{} (+{}ms) LifecyclePhaseEntered phase={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                phase.as_str()
            ),
            TraceEvent::ExecutionBoundaryCrossed {
                ts_ms,
                elapsed_ms,
                boundary,
                state,
            } => format!(
                "{} (+{}ms) ExecutionBoundaryCrossed boundary={} state={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                boundary.as_str(),
                state
            ),
            TraceEvent::GovernedProposalObserved {
                ts_ms,
                elapsed_ms,
                proposal_id,
                tool_name,
                redacted_arguments_ref,
            } => format!(
                "{} (+{}ms) GovernedProposalObserved proposal_id={} tool={} arguments_ref={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                proposal_id,
                tool_name,
                redacted_arguments_ref
            ),
            TraceEvent::GovernedProposalNormalized {
                ts_ms,
                elapsed_ms,
                proposal_id,
                normalized_proposal_ref,
                redacted_arguments_ref,
            } => format!(
                "{} (+{}ms) GovernedProposalNormalized proposal_id={} normalized_ref={} arguments_ref={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                proposal_id,
                normalized_proposal_ref,
                redacted_arguments_ref
            ),
            TraceEvent::GovernedAccConstructed {
                ts_ms,
                elapsed_ms,
                proposal_id,
                acc_contract_id,
                replay_posture,
            } => format!(
                "{} (+{}ms) GovernedAccConstructed proposal_id={} acc={} replay_posture={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                proposal_id,
                acc_contract_id,
                replay_posture
            ),
            TraceEvent::GovernedPolicyInjected {
                ts_ms,
                elapsed_ms,
                proposal_id,
                policy_evidence_ref,
                outcome,
            } => format!(
                "{} (+{}ms) GovernedPolicyInjected proposal_id={} policy_ref={} outcome={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                proposal_id,
                policy_evidence_ref,
                outcome
            ),
            TraceEvent::GovernedVisibilityResolved {
                ts_ms,
                elapsed_ms,
                proposal_id,
                actor_view,
                operator_view,
                reviewer_view,
                public_report_view,
                observatory_projection,
            } => format!(
                "{} (+{}ms) GovernedVisibilityResolved proposal_id={} actor_view={} operator_view={} reviewer_view={} public_view={} observatory={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                proposal_id,
                actor_view,
                operator_view,
                reviewer_view,
                public_report_view,
                observatory_projection
            ),
            TraceEvent::GovernedFreedomGateDecided {
                ts_ms,
                elapsed_ms,
                proposal_id,
                candidate_id,
                decision,
                reason_code,
                boundary,
                redaction_summary,
            } => format!(
                "{} (+{}ms) GovernedFreedomGateDecided proposal_id={} candidate={} decision={} reason={} boundary={} redaction={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                proposal_id,
                candidate_id,
                decision,
                reason_code,
                boundary,
                redaction_summary
            ),
            TraceEvent::GovernedActionSelected {
                ts_ms,
                elapsed_ms,
                proposal_id,
                action_id,
                tool_name,
                adapter_id,
                evidence_refs,
            } => format!(
                "{} (+{}ms) GovernedActionSelected proposal_id={} action_id={} tool={} adapter={} evidence_count={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                proposal_id,
                action_id,
                tool_name,
                adapter_id,
                evidence_refs.len()
            ),
            TraceEvent::GovernedActionRejected {
                ts_ms,
                elapsed_ms,
                proposal_id,
                action_id,
                tool_name,
                adapter_id,
                reason_code,
                evidence_refs,
            } => format!(
                "{} (+{}ms) GovernedActionRejected proposal_id={} action_id={} tool={} adapter={} reason={} evidence_count={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                proposal_id,
                action_id,
                tool_name,
                adapter_id,
                reason_code,
                evidence_refs.len()
            ),
            TraceEvent::GovernedExecutionResultRecorded {
                ts_ms,
                elapsed_ms,
                proposal_id,
                action_id,
                adapter_id,
                result_ref,
                evidence_refs,
            } => format!(
                "{} (+{}ms) GovernedExecutionResultRecorded proposal_id={} action_id={} adapter={} result_ref={} evidence_count={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                proposal_id,
                action_id,
                adapter_id,
                result_ref,
                evidence_refs.len()
            ),
            TraceEvent::GovernedRefusalRecorded {
                ts_ms,
                elapsed_ms,
                proposal_id,
                action_id,
                reason_code,
                evidence_refs,
            } => format!(
                "{} (+{}ms) GovernedRefusalRecorded proposal_id={} action_id={} reason={} evidence_count={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                proposal_id,
                action_id,
                reason_code,
                evidence_refs.len()
            ),
            TraceEvent::GovernedRedactionDecisionRecorded {
                ts_ms,
                elapsed_ms,
                proposal_id,
                audience,
                surfaces,
                outcome,
                detail,
            } => {
                let surfaces = surfaces.join(",");
                let base = format!(
                    "{} (+{}ms) GovernedRedactionDecisionRecorded proposal_id={} audience={} surfaces={} outcome={}",
                    format_ts_ms(*ts_ms),
                    elapsed_ms,
                    proposal_id,
                    audience,
                    surfaces,
                    outcome
                );
                if let Some(detail) = detail {
                    format!("{base} detail={detail}")
                } else {
                    base
                }
            }
            TraceEvent::SchedulerPolicy {
                ts_ms,
                elapsed_ms,
                max_concurrency,
                source,
            } => format!(
                "{} (+{}ms) SchedulerPolicy max_concurrency={} source={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                max_concurrency,
                source
            ),
            TraceEvent::RunFailed {
                ts_ms,
                elapsed_ms,
                message,
            } => format!(
                "{} (+{}ms) RunFailed message={message}",
                format_ts_ms(*ts_ms),
                elapsed_ms
            ),
            TraceEvent::RunFinished {
                ts_ms,
                elapsed_ms,
                success,
            } => format!(
                "{} (+{}ms) RunFinished success={success}",
                format_ts_ms(*ts_ms),
                elapsed_ms
            ),
            TraceEvent::StepStarted {
                ts_ms,
                elapsed_ms,
                step_id,
                agent_id,
                provider_id,
                task_id,
                delegation,
            } => {
                let prefix = format!(
                    "{} (+{}ms) StepStarted step={step_id} agent={agent_id} provider={provider_id} task={task_id}",
                    format_ts_ms(*ts_ms),
                    elapsed_ms
                );
                if let Some(json) = delegation_json(delegation.as_ref()) {
                    format!("{prefix} delegation={json}")
                } else {
                    prefix
                }
            }
            TraceEvent::PromptAssembled {
                ts_ms,
                elapsed_ms,
                step_id,
                prompt_hash,
            } => format!(
                "{} (+{}ms) PromptAssembled step={step_id} hash={prompt_hash}",
                format_ts_ms(*ts_ms),
                elapsed_ms
            ),
            TraceEvent::StepOutputChunk {
                ts_ms,
                elapsed_ms,
                step_id,
                chunk_bytes,
            } => format!(
                "{} (+{}ms) StepOutputChunk step={step_id} bytes={chunk_bytes}",
                format_ts_ms(*ts_ms),
                elapsed_ms
            ),
            TraceEvent::DelegationRequested {
                ts_ms,
                elapsed_ms,
                delegation_id,
                step_id,
                action_kind,
                target_id,
            } => format!(
                "{} (+{}ms) DelegationRequested delegation_id={} step={} action={} target={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                delegation_id,
                step_id,
                action_kind,
                target_id
            ),
            TraceEvent::DelegationPolicyEvaluated {
                ts_ms,
                elapsed_ms,
                delegation_id,
                step_id,
                action_kind,
                target_id,
                decision,
                rule_id,
            } => {
                let base = format!(
                    "{} (+{}ms) DelegationPolicyEvaluated delegation_id={} step={} action={} target={} decision={}",
                    format_ts_ms(*ts_ms),
                    elapsed_ms,
                    delegation_id,
                    step_id,
                    action_kind,
                    target_id,
                    decision
                );
                if let Some(rule_id) = rule_id {
                    format!("{base} rule_id={rule_id}")
                } else {
                    base
                }
            }
            TraceEvent::DelegationApproved {
                ts_ms,
                elapsed_ms,
                delegation_id,
                step_id,
            } => format!(
                "{} (+{}ms) DelegationApproved delegation_id={} step={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                delegation_id,
                step_id
            ),
            TraceEvent::DelegationDenied {
                ts_ms,
                elapsed_ms,
                delegation_id,
                step_id,
                action_kind,
                target_id,
                rule_id,
            } => {
                let base = format!(
                    "{} (+{}ms) DelegationDenied delegation_id={} step={} action={} target={}",
                    format_ts_ms(*ts_ms),
                    elapsed_ms,
                    delegation_id,
                    step_id,
                    action_kind,
                    target_id
                );
                if let Some(rule_id) = rule_id {
                    format!("{base} rule_id={rule_id}")
                } else {
                    base
                }
            }
            TraceEvent::DelegationDispatched {
                ts_ms,
                elapsed_ms,
                delegation_id,
                step_id,
                action_kind,
                target_id,
            } => format!(
                "{} (+{}ms) DelegationDispatched delegation_id={} step={} action={} target={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                delegation_id,
                step_id,
                action_kind,
                target_id
            ),
            TraceEvent::DelegationResultReceived {
                ts_ms,
                elapsed_ms,
                delegation_id,
                step_id,
                success,
                output_bytes,
            } => format!(
                "{} (+{}ms) DelegationResultReceived delegation_id={} step={} success={} bytes={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                delegation_id,
                step_id,
                success,
                output_bytes
            ),
            TraceEvent::DelegationCompleted {
                ts_ms,
                elapsed_ms,
                delegation_id,
                step_id,
                outcome,
            } => format!(
                "{} (+{}ms) DelegationCompleted delegation_id={} step={} outcome={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                delegation_id,
                step_id,
                outcome
            ),
            TraceEvent::StepFinished {
                ts_ms,
                elapsed_ms,
                step_id,
                success,
                duration_ms,
            } => format!(
                "{} (+{}ms) StepFinished step={step_id} success={success} duration={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                format_duration_secs(*duration_ms)
            ),
            TraceEvent::CallEntered {
                ts_ms,
                elapsed_ms,
                caller_step_id,
                callee_workflow_id,
                namespace,
            } => format!(
                "{} (+{}ms) CallEntered caller_step={} callee_workflow={} namespace={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                caller_step_id,
                callee_workflow_id,
                namespace
            ),
            TraceEvent::CallExited {
                ts_ms,
                elapsed_ms,
                caller_step_id,
                status,
                namespace,
            } => format!(
                "{} (+{}ms) CallExited caller_step={} status={} namespace={}",
                format_ts_ms(*ts_ms),
                elapsed_ms,
                caller_step_id,
                status,
                namespace
            ),
        }
    }
}

/// Print a human-readable trace to stdout (stable + diff-friendly).
pub fn print_trace(tr: &Trace) {
    println!(
        "TRACE run_id={} workflow_id={} version={}",
        tr.run_id, tr.workflow_id, tr.version
    );
    for ev in &tr.events {
        println!("{}", ev.summarize());
    }
}

fn delegation_json(delegation: Option<&DelegationSpec>) -> Option<String> {
    let d = delegation?;
    if d.is_effectively_empty() {
        return None;
    }
    serde_json::to_string(&d.canonicalized()).ok()
}

pub(crate) fn sanitize_governed_text(value: &str) -> String {
    if value.contains("/Users/")
        || value.contains("/home/")
        || value.contains("sk-")
        || value.contains("gho_")
        || value.contains("BEGIN PRIVATE KEY")
    {
        return "[redacted-sensitive-text]".to_string();
    }
    if value.contains('{') || value.contains('}') {
        return "[redacted-structured-text]".to_string();
    }
    value.to_string()
}

pub fn format_iso_utc_ms(ts_ms: u128) -> String {
    format_ts_ms(ts_ms)
}

fn format_ts_ms(ts_ms: u128) -> String {
    // Convert unix epoch millis to UTC without external dependencies.
    // Algorithm adapted from civil calendar conversion by Howard Hinnant.
    let total_secs = i128::try_from(ts_ms / 1000).unwrap_or(i128::MAX);
    let millis = ts_ms % 1000;

    let days = total_secs.div_euclid(86_400);
    let secs_of_day = total_secs.rem_euclid(86_400);

    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 }.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096).div_euclid(365);
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2).div_euclid(153);
    let d = doy - (153 * mp + 2).div_euclid(5) + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };

    let hour = secs_of_day.div_euclid(3_600);
    let minute = secs_of_day.rem_euclid(3_600).div_euclid(60);
    let second = secs_of_day.rem_euclid(60);

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        year, m, d, hour, minute, second, millis
    )
}

fn format_duration_secs(duration_ms: u128) -> String {
    let secs = duration_ms as f64 / 1000.0;
    format!("{secs:.3}s")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adl::DelegationSpec;
    use crate::execute::{ExecutionBoundary, RuntimeLifecyclePhase};

    #[test]
    fn summarize_covers_all_variants_and_optional_paths() {
        let ts_ms = 1_712_345_678_901;

        let event_samples = vec![
            TraceEvent::LifecyclePhaseEntered {
                ts_ms,
                elapsed_ms: 0,
                phase: RuntimeLifecyclePhase::Init,
            },
            TraceEvent::ExecutionBoundaryCrossed {
                ts_ms,
                elapsed_ms: 1,
                boundary: ExecutionBoundary::RuntimeInit,
                state: "fresh_start".to_string(),
            },
            TraceEvent::GovernedProposalObserved {
                ts_ms,
                elapsed_ms: 2,
                proposal_id: "p1".to_string(),
                tool_name: "tool".to_string(),
                redacted_arguments_ref: "redacted-a".to_string(),
            },
            TraceEvent::GovernedProposalNormalized {
                ts_ms,
                elapsed_ms: 3,
                proposal_id: "p1".to_string(),
                normalized_proposal_ref: "norm-p1".to_string(),
                redacted_arguments_ref: "redacted-a".to_string(),
            },
            TraceEvent::GovernedAccConstructed {
                ts_ms,
                elapsed_ms: 4,
                proposal_id: "p1".to_string(),
                acc_contract_id: "acc".to_string(),
                replay_posture: "strict".to_string(),
            },
            TraceEvent::GovernedPolicyInjected {
                ts_ms,
                elapsed_ms: 5,
                proposal_id: "p1".to_string(),
                policy_evidence_ref: "policy".to_string(),
                outcome: "allowed".to_string(),
            },
            TraceEvent::GovernedVisibilityResolved {
                ts_ms,
                elapsed_ms: 6,
                proposal_id: "p1".to_string(),
                actor_view: "actor".to_string(),
                operator_view: "operator".to_string(),
                reviewer_view: "{reviewer:public}".to_string(),
                public_report_view: "/Users/secret/path".to_string(),
                observatory_projection: "sk-foo".to_string(),
            },
            TraceEvent::GovernedFreedomGateDecided {
                ts_ms,
                elapsed_ms: 7,
                proposal_id: "p1".to_string(),
                candidate_id: "c1".to_string(),
                decision: "deny".to_string(),
                reason_code: "policy".to_string(),
                boundary: "run".to_string(),
                redaction_summary: "no-redactions".to_string(),
            },
            TraceEvent::GovernedActionSelected {
                ts_ms,
                elapsed_ms: 8,
                proposal_id: "p1".to_string(),
                action_id: "a1".to_string(),
                tool_name: "tool".to_string(),
                adapter_id: "adapter".to_string(),
                evidence_refs: vec!["e1".to_string(), "e2".to_string()],
            },
            TraceEvent::GovernedActionRejected {
                ts_ms,
                elapsed_ms: 9,
                proposal_id: "p1".to_string(),
                action_id: "a1".to_string(),
                tool_name: "tool".to_string(),
                adapter_id: "adapter".to_string(),
                reason_code: "denied".to_string(),
                evidence_refs: vec![],
            },
            TraceEvent::GovernedExecutionResultRecorded {
                ts_ms,
                elapsed_ms: 10,
                proposal_id: "p1".to_string(),
                action_id: "a1".to_string(),
                adapter_id: "adapter".to_string(),
                result_ref: "result".to_string(),
                evidence_refs: vec!["r1".to_string()],
            },
            TraceEvent::GovernedRefusalRecorded {
                ts_ms,
                elapsed_ms: 11,
                proposal_id: "p1".to_string(),
                action_id: "a1".to_string(),
                reason_code: "reject".to_string(),
                evidence_refs: vec!["r2".to_string()],
            },
            TraceEvent::GovernedRedactionDecisionRecorded {
                ts_ms,
                elapsed_ms: 12,
                proposal_id: "p1".to_string(),
                audience: "aud".to_string(),
                surfaces: vec!["results".to_string()],
                outcome: "redacted".to_string(),
                detail: Some("digest".to_string()),
            },
            TraceEvent::GovernedRedactionDecisionRecorded {
                ts_ms,
                elapsed_ms: 13,
                proposal_id: "p2".to_string(),
                audience: "aud2".to_string(),
                surfaces: vec!["arguments".to_string()],
                outcome: "redacted".to_string(),
                detail: None,
            },
            TraceEvent::SchedulerPolicy {
                ts_ms,
                elapsed_ms: 14,
                max_concurrency: 2,
                source: "planner".to_string(),
            },
            TraceEvent::RunFailed {
                ts_ms,
                elapsed_ms: 15,
                message: "error".to_string(),
            },
            TraceEvent::RunFinished {
                ts_ms,
                elapsed_ms: 16,
                success: false,
            },
            TraceEvent::StepStarted {
                ts_ms,
                elapsed_ms: 17,
                step_id: "s1".to_string(),
                agent_id: "agent".to_string(),
                provider_id: "provider".to_string(),
                task_id: "task".to_string(),
                delegation: Some(DelegationSpec {
                    role: Some("planner".to_string()),
                    requires_verification: Some(false),
                    escalation_target: Some("human".to_string()),
                    tags: vec!["tag-a".to_string(), "tag-b".to_string()],
                }),
            },
            TraceEvent::PromptAssembled {
                ts_ms,
                elapsed_ms: 18,
                step_id: "s1".to_string(),
                prompt_hash: "phash".to_string(),
            },
            TraceEvent::StepOutputChunk {
                ts_ms,
                elapsed_ms: 19,
                step_id: "s1".to_string(),
                chunk_bytes: 8,
            },
            TraceEvent::DelegationRequested {
                ts_ms,
                elapsed_ms: 20,
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
                action_kind: "provider_call".to_string(),
                target_id: "provider".to_string(),
            },
            TraceEvent::DelegationPolicyEvaluated {
                ts_ms,
                elapsed_ms: 21,
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
                action_kind: "provider_call".to_string(),
                target_id: "provider".to_string(),
                decision: "allowed".to_string(),
                rule_id: None,
            },
            TraceEvent::DelegationPolicyEvaluated {
                ts_ms,
                elapsed_ms: 22,
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
                action_kind: "provider_call".to_string(),
                target_id: "provider".to_string(),
                decision: "denied".to_string(),
                rule_id: Some("rule-1".to_string()),
            },
            TraceEvent::DelegationApproved {
                ts_ms,
                elapsed_ms: 23,
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
            },
            TraceEvent::DelegationDenied {
                ts_ms,
                elapsed_ms: 24,
                delegation_id: "del-2".to_string(),
                step_id: "s1".to_string(),
                action_kind: "provider_call".to_string(),
                target_id: "provider".to_string(),
                rule_id: Some("deny-1".to_string()),
            },
            TraceEvent::DelegationDenied {
                ts_ms,
                elapsed_ms: 25,
                delegation_id: "del-2".to_string(),
                step_id: "s1".to_string(),
                action_kind: "provider_call".to_string(),
                target_id: "provider".to_string(),
                rule_id: None,
            },
            TraceEvent::DelegationDispatched {
                ts_ms,
                elapsed_ms: 26,
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
                action_kind: "provider_call".to_string(),
                target_id: "provider".to_string(),
            },
            TraceEvent::DelegationResultReceived {
                ts_ms,
                elapsed_ms: 27,
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
                success: true,
                output_bytes: 128,
            },
            TraceEvent::DelegationCompleted {
                ts_ms,
                elapsed_ms: 28,
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
                outcome: "applied".to_string(),
            },
            TraceEvent::StepFinished {
                ts_ms,
                elapsed_ms: 29,
                step_id: "s1".to_string(),
                success: true,
                duration_ms: 3_000,
            },
            TraceEvent::CallEntered {
                ts_ms,
                elapsed_ms: 30,
                caller_step_id: "s1".to_string(),
                callee_workflow_id: "wf-2".to_string(),
                namespace: "ns".to_string(),
            },
            TraceEvent::CallExited {
                ts_ms,
                elapsed_ms: 31,
                caller_step_id: "s1".to_string(),
                status: "ok".to_string(),
                namespace: "ns".to_string(),
            },
        ];

        let lines: Vec<String> = event_samples.iter().map(TraceEvent::summarize).collect();
        assert!(!lines.is_empty());
        assert!(lines
            .iter()
            .any(|line| line.contains("GovernedVisibilityResolved")));
        assert!(lines
            .iter()
            .any(|line| line.contains("GovernedRedactionDecisionRecorded")));
        assert!(lines
            .iter()
            .any(|line| line.contains("DelegationPolicyEvaluated")));
        assert!(lines.iter().any(|line| line.contains("rule_id=rule-1")));
        assert!(lines.iter().any(|line| line.contains("CallExited")));
    }

    #[test]
    fn trace_report_helpers_preserve_expected_shapes() {
        assert_eq!(
            format_iso_utc_ms(1_609_459_200_000),
            "2021-01-01T00:00:00.000Z"
        );
        assert_eq!(format_duration_secs(1_500), "1.500s");
        assert_eq!(format_duration_secs(0), "0.000s");

        assert_eq!(
            sanitize_governed_text("path=/Users/test/private-key"),
            "[redacted-sensitive-text]"
        );
        assert_eq!(
            sanitize_governed_text("{\"secret\":\"yes\"}"),
            "[redacted-structured-text]"
        );
        assert_eq!(sanitize_governed_text("plain"), "plain");
    }

    #[test]
    fn trace_print_trace_hits_output_path() {
        let mut trace = crate::trace::Trace::new("run-print", "workflow-print", "0.90.5");
        trace.lifecycle_phase_entered(RuntimeLifecyclePhase::Init);
        print_trace(&trace);
        assert_eq!(trace.events.len(), 1);
    }
}
