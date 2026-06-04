//! Deterministic runtime action-log projection.
//!
//! The action log is a reviewer/operator-facing JSONL view over the canonical
//! runtime trace. It does not replace trace/replay evidence; it makes material
//! runtime decisions inspectable without exposing prompts, secrets, or raw tool
//! arguments.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{Context, Result};
use serde::Serialize;

use crate::trace::TraceEvent;

pub const ACTION_LOG_SCHEMA_VERSION: &str = "adl.runtime_action_log.v1";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeActionLogEvent {
    pub schema_version: &'static str,
    pub sequence: usize,
    pub stage: String,
    pub component: String,
    pub result: String,
    pub step_id: Option<String>,
    pub actor: Option<String>,
    pub provider_ref: Option<String>,
    pub action_ref: Option<String>,
    pub input_refs: Vec<String>,
    pub output_refs: Vec<String>,
    pub reason_code: Option<String>,
    pub elapsed_ms: u128,
}

impl RuntimeActionLogEvent {
    fn new(
        sequence: usize,
        stage: impl Into<String>,
        component: impl Into<String>,
        result: impl Into<String>,
        elapsed_ms: u128,
    ) -> Self {
        Self {
            schema_version: ACTION_LOG_SCHEMA_VERSION,
            sequence,
            stage: stage.into(),
            component: component.into(),
            result: result.into(),
            step_id: None,
            actor: None,
            provider_ref: None,
            action_ref: None,
            input_refs: Vec::new(),
            output_refs: Vec::new(),
            reason_code: None,
            elapsed_ms,
        }
    }
}

pub fn runtime_action_log_events(events: &[TraceEvent]) -> Vec<RuntimeActionLogEvent> {
    events
        .iter()
        .enumerate()
        .map(|(sequence, event)| action_log_event(sequence, event))
        .collect()
}

pub fn runtime_action_log_events_with_artifacts(
    events: &[TraceEvent],
    artifact_refs: &[String],
) -> Vec<RuntimeActionLogEvent> {
    let mut log_events = runtime_action_log_events(events);
    let mut artifact_refs = artifact_refs.to_vec();
    artifact_refs.sort();
    artifact_refs.dedup();
    for artifact_ref in artifact_refs {
        let mut ev = RuntimeActionLogEvent::new(
            log_events.len(),
            "artifact_write",
            "runtime",
            "wrote",
            events_elapsed_ms(events),
        );
        ev.output_refs.push(artifact_ref);
        ev.reason_code = Some("generated_run_artifact".to_string());
        log_events.push(ev);
    }
    log_events
}

pub fn write_action_log_artifact(path: &Path, events: &[TraceEvent]) -> Result<()> {
    write_action_log_events(path, &runtime_action_log_events(events))
}

pub fn write_action_log_artifact_with_artifacts(
    path: &Path,
    events: &[TraceEvent],
    artifact_refs: &[String],
) -> Result<()> {
    write_action_log_events(
        path,
        &runtime_action_log_events_with_artifacts(events, artifact_refs),
    )
}

fn write_action_log_events(path: &Path, events: &[RuntimeActionLogEvent]) -> Result<()> {
    let file = File::create(path)
        .with_context(|| format!("failed creating runtime action log '{}'", path.display()))?;
    let mut writer = BufWriter::new(file);
    for event in events {
        serde_json::to_writer(&mut writer, &event).context("serialize runtime action log event")?;
        writer
            .write_all(b"\n")
            .context("write runtime action log newline")?;
    }
    writer
        .flush()
        .with_context(|| format!("flush runtime action log '{}'", path.display()))?;
    Ok(())
}

fn events_elapsed_ms(events: &[TraceEvent]) -> u128 {
    events
        .iter()
        .map(|event| match event {
            TraceEvent::LifecyclePhaseEntered { elapsed_ms, .. }
            | TraceEvent::ExecutionBoundaryCrossed { elapsed_ms, .. }
            | TraceEvent::GovernedProposalObserved { elapsed_ms, .. }
            | TraceEvent::GovernedProposalNormalized { elapsed_ms, .. }
            | TraceEvent::GovernedAccConstructed { elapsed_ms, .. }
            | TraceEvent::GovernedPolicyInjected { elapsed_ms, .. }
            | TraceEvent::GovernedVisibilityResolved { elapsed_ms, .. }
            | TraceEvent::GovernedFreedomGateDecided { elapsed_ms, .. }
            | TraceEvent::GovernedActionSelected { elapsed_ms, .. }
            | TraceEvent::GovernedActionRejected { elapsed_ms, .. }
            | TraceEvent::GovernedExecutionResultRecorded { elapsed_ms, .. }
            | TraceEvent::GovernedRefusalRecorded { elapsed_ms, .. }
            | TraceEvent::GovernedRedactionDecisionRecorded { elapsed_ms, .. }
            | TraceEvent::SchedulerPolicy { elapsed_ms, .. }
            | TraceEvent::RunFailed { elapsed_ms, .. }
            | TraceEvent::RunFinished { elapsed_ms, .. }
            | TraceEvent::StepStarted { elapsed_ms, .. }
            | TraceEvent::PromptAssembled { elapsed_ms, .. }
            | TraceEvent::StepOutputChunk { elapsed_ms, .. }
            | TraceEvent::DelegationRequested { elapsed_ms, .. }
            | TraceEvent::DelegationPolicyEvaluated { elapsed_ms, .. }
            | TraceEvent::DelegationApproved { elapsed_ms, .. }
            | TraceEvent::DelegationDenied { elapsed_ms, .. }
            | TraceEvent::DelegationDispatched { elapsed_ms, .. }
            | TraceEvent::DelegationResultReceived { elapsed_ms, .. }
            | TraceEvent::DelegationCompleted { elapsed_ms, .. }
            | TraceEvent::StepFinished { elapsed_ms, .. }
            | TraceEvent::CallEntered { elapsed_ms, .. }
            | TraceEvent::CallExited { elapsed_ms, .. } => *elapsed_ms,
        })
        .max()
        .unwrap_or(0)
}

fn action_log_event(sequence: usize, event: &TraceEvent) -> RuntimeActionLogEvent {
    match event {
        TraceEvent::LifecyclePhaseEntered {
            elapsed_ms, phase, ..
        } => {
            let mut ev = RuntimeActionLogEvent::new(
                sequence,
                "lifecycle",
                "runtime",
                "entered",
                *elapsed_ms,
            );
            ev.reason_code = Some(phase.as_str().to_string());
            ev
        }
        TraceEvent::ExecutionBoundaryCrossed {
            elapsed_ms,
            boundary,
            state,
            ..
        } => {
            let mut ev =
                RuntimeActionLogEvent::new(sequence, "boundary", "runtime", state, *elapsed_ms);
            ev.reason_code = Some(boundary.as_str().to_string());
            ev
        }
        TraceEvent::SchedulerPolicy {
            elapsed_ms,
            max_concurrency,
            source,
            ..
        } => {
            let mut ev = RuntimeActionLogEvent::new(
                sequence,
                "scheduler",
                "runtime",
                "selected",
                *elapsed_ms,
            );
            ev.reason_code = Some(source.clone());
            ev.output_refs
                .push(format!("max_concurrency:{max_concurrency}"));
            ev
        }
        TraceEvent::RunFailed { elapsed_ms, .. } => {
            let mut ev =
                RuntimeActionLogEvent::new(sequence, "run", "runtime", "failed", *elapsed_ms);
            ev.reason_code = Some("runtime_failure".to_string());
            ev
        }
        TraceEvent::RunFinished {
            elapsed_ms,
            success,
            ..
        } => RuntimeActionLogEvent::new(
            sequence,
            "run",
            "runtime",
            if *success { "success" } else { "failure" },
            *elapsed_ms,
        ),
        TraceEvent::StepStarted {
            elapsed_ms,
            step_id,
            agent_id,
            provider_id,
            task_id,
            ..
        } => {
            let mut ev =
                RuntimeActionLogEvent::new(sequence, "step", "runtime", "started", *elapsed_ms);
            ev.step_id = Some(step_id.clone());
            ev.actor = Some(agent_id.clone());
            ev.provider_ref = Some(provider_id.clone());
            ev.input_refs.push(format!("task:{task_id}"));
            ev
        }
        TraceEvent::PromptAssembled {
            elapsed_ms,
            step_id,
            prompt_hash,
            ..
        } => {
            let mut ev = RuntimeActionLogEvent::new(
                sequence,
                "validate_inputs",
                "runtime",
                "ok",
                *elapsed_ms,
            );
            ev.step_id = Some(step_id.clone());
            ev.input_refs.push(format!("prompt_hash:{prompt_hash}"));
            ev.reason_code = Some("prompt_assembled".to_string());
            ev
        }
        TraceEvent::StepOutputChunk {
            elapsed_ms,
            step_id,
            chunk_bytes,
            ..
        } => {
            let mut ev = RuntimeActionLogEvent::new(
                sequence,
                "provider_output",
                "runtime",
                "observed",
                *elapsed_ms,
            );
            ev.step_id = Some(step_id.clone());
            ev.output_refs.push(format!("chunk_bytes:{chunk_bytes}"));
            ev
        }
        TraceEvent::StepFinished {
            elapsed_ms,
            step_id,
            success,
            ..
        } => {
            let mut ev = RuntimeActionLogEvent::new(
                sequence,
                "step",
                "runtime",
                result(*success),
                *elapsed_ms,
            );
            ev.step_id = Some(step_id.clone());
            ev
        }
        TraceEvent::DelegationRequested {
            elapsed_ms,
            delegation_id,
            step_id,
            action_kind,
            target_id,
            ..
        } => {
            let mut ev = RuntimeActionLogEvent::new(
                sequence,
                "policy",
                "delegation",
                "requested",
                *elapsed_ms,
            );
            ev.step_id = Some(step_id.clone());
            ev.action_ref = Some(format!("{action_kind}:{target_id}"));
            ev.input_refs.push(format!("delegation:{delegation_id}"));
            ev
        }
        TraceEvent::DelegationPolicyEvaluated {
            elapsed_ms,
            delegation_id,
            step_id,
            action_kind,
            target_id,
            decision,
            rule_id,
            ..
        } => {
            let mut ev =
                RuntimeActionLogEvent::new(sequence, "policy", "delegation", decision, *elapsed_ms);
            ev.step_id = Some(step_id.clone());
            ev.action_ref = Some(format!("{action_kind}:{target_id}"));
            ev.input_refs.push(format!("delegation:{delegation_id}"));
            ev.reason_code = rule_id.clone();
            ev
        }
        TraceEvent::DelegationApproved {
            elapsed_ms,
            delegation_id,
            step_id,
            ..
        } => {
            let mut ev = RuntimeActionLogEvent::new(
                sequence,
                "policy",
                "delegation",
                "approved",
                *elapsed_ms,
            );
            ev.step_id = Some(step_id.clone());
            ev.input_refs.push(format!("delegation:{delegation_id}"));
            ev
        }
        TraceEvent::DelegationDenied {
            elapsed_ms,
            delegation_id,
            step_id,
            action_kind,
            target_id,
            rule_id,
            ..
        } => {
            let mut ev =
                RuntimeActionLogEvent::new(sequence, "policy", "delegation", "denied", *elapsed_ms);
            ev.step_id = Some(step_id.clone());
            ev.action_ref = Some(format!("{action_kind}:{target_id}"));
            ev.input_refs.push(format!("delegation:{delegation_id}"));
            ev.reason_code = rule_id
                .clone()
                .or_else(|| Some("policy_denied".to_string()));
            ev
        }
        TraceEvent::DelegationDispatched {
            elapsed_ms,
            delegation_id,
            step_id,
            action_kind,
            target_id,
            ..
        } => {
            let mut ev = RuntimeActionLogEvent::new(
                sequence,
                "provider_route",
                "delegation",
                "dispatched",
                *elapsed_ms,
            );
            ev.step_id = Some(step_id.clone());
            ev.action_ref = Some(format!("{action_kind}:{target_id}"));
            ev.input_refs.push(format!("delegation:{delegation_id}"));
            ev.provider_ref = Some(target_id.clone());
            ev
        }
        TraceEvent::DelegationResultReceived {
            elapsed_ms,
            delegation_id,
            step_id,
            success,
            output_bytes,
            ..
        } => {
            let mut ev = RuntimeActionLogEvent::new(
                sequence,
                "provider_call",
                "delegation",
                result(*success),
                *elapsed_ms,
            );
            ev.step_id = Some(step_id.clone());
            ev.input_refs.push(format!("delegation:{delegation_id}"));
            ev.output_refs.push(format!("output_bytes:{output_bytes}"));
            ev
        }
        TraceEvent::DelegationCompleted {
            elapsed_ms,
            delegation_id,
            step_id,
            outcome,
            ..
        } => {
            let mut ev =
                RuntimeActionLogEvent::new(sequence, "delegation", "runtime", outcome, *elapsed_ms);
            ev.step_id = Some(step_id.clone());
            ev.input_refs.push(format!("delegation:{delegation_id}"));
            ev
        }
        TraceEvent::CallEntered {
            elapsed_ms,
            caller_step_id,
            callee_workflow_id,
            namespace,
            ..
        } => {
            let mut ev = RuntimeActionLogEvent::new(
                sequence,
                "workflow_call",
                "runtime",
                "entered",
                *elapsed_ms,
            );
            ev.step_id = Some(caller_step_id.clone());
            ev.action_ref = Some(format!("workflow:{callee_workflow_id}"));
            ev.input_refs.push(format!("namespace:{namespace}"));
            ev
        }
        TraceEvent::CallExited {
            elapsed_ms,
            caller_step_id,
            status,
            namespace,
            ..
        } => {
            let mut ev = RuntimeActionLogEvent::new(
                sequence,
                "workflow_call",
                "runtime",
                status,
                *elapsed_ms,
            );
            ev.step_id = Some(caller_step_id.clone());
            ev.input_refs.push(format!("namespace:{namespace}"));
            ev
        }
        TraceEvent::GovernedProposalObserved {
            elapsed_ms,
            proposal_id,
            tool_name,
            redacted_arguments_ref,
            ..
        } => {
            let mut ev =
                RuntimeActionLogEvent::new(sequence, "proposal", "polis", "observed", *elapsed_ms);
            ev.action_ref = Some(format!("proposal:{proposal_id}"));
            ev.provider_ref = Some(tool_name.clone());
            ev.input_refs.push(redacted_arguments_ref.clone());
            ev
        }
        TraceEvent::GovernedProposalNormalized {
            elapsed_ms,
            proposal_id,
            normalized_proposal_ref,
            redacted_arguments_ref,
            ..
        } => {
            let mut ev = RuntimeActionLogEvent::new(
                sequence,
                "proposal",
                "polis",
                "normalized",
                *elapsed_ms,
            );
            ev.action_ref = Some(format!("proposal:{proposal_id}"));
            ev.input_refs.push(redacted_arguments_ref.clone());
            ev.output_refs.push(normalized_proposal_ref.clone());
            ev
        }
        TraceEvent::GovernedAccConstructed {
            elapsed_ms,
            proposal_id,
            acc_contract_id,
            replay_posture,
            ..
        } => {
            let mut ev = RuntimeActionLogEvent::new(
                sequence,
                "authority_contract",
                "polis",
                "constructed",
                *elapsed_ms,
            );
            ev.action_ref = Some(format!("proposal:{proposal_id}"));
            ev.output_refs.push(acc_contract_id.clone());
            ev.reason_code = Some(replay_posture.clone());
            ev
        }
        TraceEvent::GovernedPolicyInjected {
            elapsed_ms,
            proposal_id,
            policy_evidence_ref,
            outcome,
            ..
        } => {
            let mut ev =
                RuntimeActionLogEvent::new(sequence, "policy", "polis", outcome, *elapsed_ms);
            ev.action_ref = Some(format!("proposal:{proposal_id}"));
            ev.input_refs.push(policy_evidence_ref.clone());
            ev
        }
        TraceEvent::GovernedVisibilityResolved {
            elapsed_ms,
            proposal_id,
            ..
        } => {
            let mut ev = RuntimeActionLogEvent::new(
                sequence,
                "visibility",
                "polis",
                "resolved",
                *elapsed_ms,
            );
            ev.action_ref = Some(format!("proposal:{proposal_id}"));
            ev
        }
        TraceEvent::GovernedFreedomGateDecided {
            elapsed_ms,
            proposal_id,
            candidate_id,
            decision,
            reason_code,
            ..
        } => {
            let mut ev = RuntimeActionLogEvent::new(
                sequence,
                "freedom_gate",
                "polis",
                decision,
                *elapsed_ms,
            );
            ev.action_ref = Some(format!("proposal:{proposal_id}"));
            ev.input_refs.push(format!("candidate:{candidate_id}"));
            ev.reason_code = Some(reason_code.clone());
            ev
        }
        TraceEvent::GovernedActionSelected {
            elapsed_ms,
            proposal_id,
            action_id,
            tool_name,
            adapter_id,
            evidence_refs,
            ..
        } => governed_action(
            sequence,
            *elapsed_ms,
            proposal_id,
            action_id,
            tool_name,
            adapter_id,
            evidence_refs,
            "selected",
            None,
        ),
        TraceEvent::GovernedActionRejected {
            elapsed_ms,
            proposal_id,
            action_id,
            tool_name,
            adapter_id,
            reason_code,
            evidence_refs,
            ..
        } => governed_action(
            sequence,
            *elapsed_ms,
            proposal_id,
            action_id,
            tool_name,
            adapter_id,
            evidence_refs,
            "rejected",
            Some(reason_code.clone()),
        ),
        TraceEvent::GovernedExecutionResultRecorded {
            elapsed_ms,
            proposal_id,
            action_id,
            adapter_id,
            result_ref,
            evidence_refs,
            ..
        } => {
            let mut ev = RuntimeActionLogEvent::new(
                sequence,
                "tool_result",
                "polis",
                "recorded",
                *elapsed_ms,
            );
            ev.action_ref = Some(format!("proposal:{proposal_id}/action:{action_id}"));
            ev.provider_ref = Some(adapter_id.clone());
            ev.output_refs.push(result_ref.clone());
            ev.output_refs.extend(evidence_refs.clone());
            ev
        }
        TraceEvent::GovernedRefusalRecorded {
            elapsed_ms,
            proposal_id,
            action_id,
            reason_code,
            evidence_refs,
            ..
        } => {
            let mut ev =
                RuntimeActionLogEvent::new(sequence, "refusal", "polis", "recorded", *elapsed_ms);
            ev.action_ref = Some(format!("proposal:{proposal_id}/action:{action_id}"));
            ev.reason_code = Some(reason_code.clone());
            ev.output_refs.extend(evidence_refs.clone());
            ev
        }
        TraceEvent::GovernedRedactionDecisionRecorded {
            elapsed_ms,
            proposal_id,
            audience,
            outcome,
            ..
        } => {
            let mut ev =
                RuntimeActionLogEvent::new(sequence, "redaction", "polis", outcome, *elapsed_ms);
            ev.action_ref = Some(format!("proposal:{proposal_id}"));
            ev.reason_code = Some(audience.clone());
            ev
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn governed_action(
    sequence: usize,
    elapsed_ms: u128,
    proposal_id: &str,
    action_id: &str,
    tool_name: &str,
    adapter_id: &str,
    evidence_refs: &[String],
    result: &str,
    reason_code: Option<String>,
) -> RuntimeActionLogEvent {
    let mut ev = RuntimeActionLogEvent::new(sequence, "tool_action", "polis", result, elapsed_ms);
    ev.action_ref = Some(format!("proposal:{proposal_id}/action:{action_id}"));
    ev.provider_ref = Some(format!("{tool_name}:{adapter_id}"));
    ev.output_refs.extend(evidence_refs.iter().cloned());
    ev.reason_code = reason_code;
    ev
}

fn result(success: bool) -> &'static str {
    if success {
        "success"
    } else {
        "failure"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execute::{ExecutionBoundary, RuntimeLifecyclePhase};

    #[test]
    fn runtime_action_log_projects_policy_and_provider_events_without_raw_messages() {
        let events = vec![
            TraceEvent::StepStarted {
                ts_ms: 1,
                elapsed_ms: 1,
                step_id: "s1".to_string(),
                agent_id: "agent".to_string(),
                provider_id: "provider".to_string(),
                task_id: "task".to_string(),
                delegation: None,
            },
            TraceEvent::PromptAssembled {
                ts_ms: 2,
                elapsed_ms: 2,
                step_id: "s1".to_string(),
                prompt_hash: "sha256:abc".to_string(),
            },
            TraceEvent::DelegationPolicyEvaluated {
                ts_ms: 3,
                elapsed_ms: 3,
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
                action_kind: "provider_call".to_string(),
                target_id: "provider".to_string(),
                decision: "denied".to_string(),
                rule_id: Some("deny-rule".to_string()),
            },
            TraceEvent::RunFailed {
                ts_ms: 4,
                elapsed_ms: 4,
                message: "/Users/daniel/private/token should not appear".to_string(),
            },
        ];

        let log = runtime_action_log_events(&events);
        assert_eq!(log.len(), 4);
        assert_eq!(log[0].sequence, 0);
        assert_eq!(log[0].stage, "step");
        assert_eq!(log[0].actor.as_deref(), Some("agent"));
        assert_eq!(log[1].input_refs, vec!["prompt_hash:sha256:abc"]);
        assert_eq!(log[2].stage, "policy");
        assert_eq!(log[2].result, "denied");
        assert_eq!(log[2].reason_code.as_deref(), Some("deny-rule"));
        assert_eq!(log[3].reason_code.as_deref(), Some("runtime_failure"));

        let json = serde_json::to_string(&log).expect("serialize log");
        assert!(!json.contains("/Users/daniel"));
        assert!(!json.contains("token"));
    }

    #[test]
    fn runtime_action_log_jsonl_is_deterministic() {
        let events = vec![
            TraceEvent::RunFinished {
                ts_ms: 10,
                elapsed_ms: 9,
                success: true,
            },
            TraceEvent::GovernedFreedomGateDecided {
                ts_ms: 11,
                elapsed_ms: 10,
                proposal_id: "p1".to_string(),
                candidate_id: "c1".to_string(),
                decision: "allow".to_string(),
                reason_code: "bounded_low_risk".to_string(),
                boundary: "models_propose_runtime_decides".to_string(),
                redaction_summary: "public-safe".to_string(),
            },
        ];
        let a = runtime_action_log_events(&events);
        let b = runtime_action_log_events(&events);
        assert_eq!(a, b);
        assert_eq!(a[1].component, "polis");
        assert_eq!(a[1].stage, "freedom_gate");
        assert_eq!(a[1].reason_code.as_deref(), Some("bounded_low_risk"));
    }

    #[test]
    fn runtime_action_log_writer_emits_jsonl_lines() {
        let path = std::env::temp_dir().join(format!(
            "adl-action-log-test-{}-{}.jsonl",
            std::process::id(),
            1
        ));
        let events = vec![TraceEvent::RunFinished {
            ts_ms: 10,
            elapsed_ms: 9,
            success: true,
        }];

        write_action_log_artifact(&path, &events).expect("write action log");
        let raw = std::fs::read_to_string(&path).expect("read action log");
        std::fs::remove_file(&path).ok();

        let lines: Vec<&str> = raw.lines().collect();
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("\"schema_version\":\"adl.runtime_action_log.v1\""));
        assert!(lines[0].contains("\"stage\":\"run\""));
        assert!(lines[0].contains("\"result\":\"success\""));
    }

    #[test]
    fn runtime_action_log_adds_sorted_artifact_write_events() {
        let events = vec![TraceEvent::RunFinished {
            ts_ms: 10,
            elapsed_ms: 9,
            success: true,
        }];
        let artifact_refs = vec![
            "run_summary.json".to_string(),
            "logs/trace_v1.json".to_string(),
            "run_summary.json".to_string(),
        ];

        let log = runtime_action_log_events_with_artifacts(&events, &artifact_refs);
        assert_eq!(log.len(), 3);
        assert_eq!(log[1].stage, "artifact_write");
        assert_eq!(log[1].result, "wrote");
        assert_eq!(log[1].output_refs, vec!["logs/trace_v1.json"]);
        assert_eq!(log[2].output_refs, vec!["run_summary.json"]);
        assert_eq!(
            log[2].reason_code.as_deref(),
            Some("generated_run_artifact")
        );
    }

    #[test]
    fn runtime_action_log_covers_all_runtime_event_families() {
        let events = vec![
            TraceEvent::LifecyclePhaseEntered {
                ts_ms: 1,
                elapsed_ms: 1,
                phase: RuntimeLifecyclePhase::Init,
            },
            TraceEvent::ExecutionBoundaryCrossed {
                ts_ms: 2,
                elapsed_ms: 2,
                boundary: ExecutionBoundary::RuntimeInit,
                state: "fresh_start".to_string(),
            },
            TraceEvent::SchedulerPolicy {
                ts_ms: 3,
                elapsed_ms: 3,
                max_concurrency: 2,
                source: "workflow_override".to_string(),
            },
            TraceEvent::RunFinished {
                ts_ms: 4,
                elapsed_ms: 4,
                success: false,
            },
            TraceEvent::StepOutputChunk {
                ts_ms: 5,
                elapsed_ms: 5,
                step_id: "s1".to_string(),
                chunk_bytes: 42,
            },
            TraceEvent::StepFinished {
                ts_ms: 6,
                elapsed_ms: 6,
                step_id: "s1".to_string(),
                success: false,
                duration_ms: 1,
            },
            TraceEvent::DelegationRequested {
                ts_ms: 7,
                elapsed_ms: 7,
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
                action_kind: "remote_exec".to_string(),
                target_id: "node-a".to_string(),
            },
            TraceEvent::DelegationApproved {
                ts_ms: 8,
                elapsed_ms: 8,
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
            },
            TraceEvent::DelegationDenied {
                ts_ms: 9,
                elapsed_ms: 9,
                delegation_id: "del-2".to_string(),
                step_id: "s2".to_string(),
                action_kind: "filesystem_write".to_string(),
                target_id: "workspace".to_string(),
                rule_id: None,
            },
            TraceEvent::DelegationDispatched {
                ts_ms: 10,
                elapsed_ms: 10,
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
                action_kind: "remote_exec".to_string(),
                target_id: "node-a".to_string(),
            },
            TraceEvent::DelegationResultReceived {
                ts_ms: 11,
                elapsed_ms: 11,
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
                success: true,
                output_bytes: 512,
            },
            TraceEvent::DelegationCompleted {
                ts_ms: 12,
                elapsed_ms: 12,
                delegation_id: "del-1".to_string(),
                step_id: "s1".to_string(),
                outcome: "completed".to_string(),
            },
            TraceEvent::CallEntered {
                ts_ms: 13,
                elapsed_ms: 13,
                caller_step_id: "s1".to_string(),
                callee_workflow_id: "child-flow".to_string(),
                namespace: "child".to_string(),
            },
            TraceEvent::CallExited {
                ts_ms: 14,
                elapsed_ms: 14,
                caller_step_id: "s1".to_string(),
                status: "success".to_string(),
                namespace: "child".to_string(),
            },
            TraceEvent::GovernedProposalObserved {
                ts_ms: 15,
                elapsed_ms: 15,
                proposal_id: "proposal-1".to_string(),
                tool_name: "safe_tool".to_string(),
                redacted_arguments_ref: "governed/args.redacted.json".to_string(),
            },
            TraceEvent::GovernedProposalNormalized {
                ts_ms: 16,
                elapsed_ms: 16,
                proposal_id: "proposal-1".to_string(),
                normalized_proposal_ref: "governed/proposal.normalized.json".to_string(),
                redacted_arguments_ref: "governed/args.redacted.json".to_string(),
            },
            TraceEvent::GovernedAccConstructed {
                ts_ms: 17,
                elapsed_ms: 17,
                proposal_id: "proposal-1".to_string(),
                acc_contract_id: "acc-1".to_string(),
                replay_posture: "reviewable".to_string(),
            },
            TraceEvent::GovernedPolicyInjected {
                ts_ms: 18,
                elapsed_ms: 18,
                proposal_id: "proposal-1".to_string(),
                policy_evidence_ref: "governed/policy.json".to_string(),
                outcome: "allowed".to_string(),
            },
            TraceEvent::GovernedVisibilityResolved {
                ts_ms: 19,
                elapsed_ms: 19,
                proposal_id: "proposal-1".to_string(),
                actor_view: "actor".to_string(),
                operator_view: "operator".to_string(),
                reviewer_view: "reviewer".to_string(),
                public_report_view: "public".to_string(),
                observatory_projection: "observatory".to_string(),
            },
            TraceEvent::GovernedActionSelected {
                ts_ms: 20,
                elapsed_ms: 20,
                proposal_id: "proposal-1".to_string(),
                action_id: "action-1".to_string(),
                tool_name: "safe_tool".to_string(),
                adapter_id: "adapter-a".to_string(),
                evidence_refs: vec!["evidence/a.json".to_string()],
            },
            TraceEvent::GovernedActionRejected {
                ts_ms: 21,
                elapsed_ms: 21,
                proposal_id: "proposal-1".to_string(),
                action_id: "action-2".to_string(),
                tool_name: "safe_tool".to_string(),
                adapter_id: "adapter-a".to_string(),
                reason_code: "policy_denied".to_string(),
                evidence_refs: vec!["evidence/b.json".to_string()],
            },
            TraceEvent::GovernedExecutionResultRecorded {
                ts_ms: 22,
                elapsed_ms: 22,
                proposal_id: "proposal-1".to_string(),
                action_id: "action-1".to_string(),
                adapter_id: "adapter-a".to_string(),
                result_ref: "governed/result.json".to_string(),
                evidence_refs: vec!["evidence/c.json".to_string()],
            },
            TraceEvent::GovernedRefusalRecorded {
                ts_ms: 23,
                elapsed_ms: 23,
                proposal_id: "proposal-1".to_string(),
                action_id: "action-3".to_string(),
                reason_code: "unsafe_request".to_string(),
                evidence_refs: vec!["evidence/refusal.json".to_string()],
            },
            TraceEvent::GovernedRedactionDecisionRecorded {
                ts_ms: 24,
                elapsed_ms: 24,
                proposal_id: "proposal-1".to_string(),
                audience: "public".to_string(),
                surfaces: vec!["report".to_string()],
                outcome: "redacted".to_string(),
                detail: None,
            },
        ];

        let log = runtime_action_log_events(&events);
        assert_eq!(log.len(), events.len());
        assert_eq!(log[0].reason_code.as_deref(), Some("init"));
        assert_eq!(log[1].stage, "boundary");
        assert_eq!(log[2].output_refs, vec!["max_concurrency:2"]);
        assert_eq!(log[3].result, "failure");
        assert_eq!(log[8].reason_code.as_deref(), Some("policy_denied"));
        assert_eq!(log[12].action_ref.as_deref(), Some("workflow:child-flow"));
        assert_eq!(log[14].component, "polis");
        assert_eq!(log[18].stage, "visibility");
        assert_eq!(log[19].provider_ref.as_deref(), Some("safe_tool:adapter-a"));
        assert_eq!(log[20].reason_code.as_deref(), Some("policy_denied"));
        assert_eq!(
            log[21].output_refs,
            vec!["governed/result.json", "evidence/c.json"]
        );
        assert_eq!(log[22].reason_code.as_deref(), Some("unsafe_request"));
        assert_eq!(log[23].reason_code.as_deref(), Some("public"));
    }
}
