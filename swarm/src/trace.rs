use std::collections::HashMap;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::adl::DelegationSpec;

#[derive(Debug, Clone)]
pub struct Trace {
    pub run_id: String,
    pub workflow_id: String,
    pub version: String,
    pub events: Vec<TraceEvent>,
    run_started_ms: u128,
    run_started_instant: Instant,
    step_started_ms: HashMap<String, u128>,
    delegation_ids: HashMap<String, String>,
    next_delegation_counter: u64,
}

#[derive(Debug, Clone)]
pub enum TraceEvent {
    SchedulerPolicy {
        ts_ms: u128,
        elapsed_ms: u128,
        max_concurrency: usize,
        source: String,
    },
    RunFailed {
        ts_ms: u128,
        elapsed_ms: u128,
        message: String,
    },
    RunFinished {
        ts_ms: u128,
        elapsed_ms: u128,
        success: bool,
    },
    StepStarted {
        ts_ms: u128,
        elapsed_ms: u128,
        step_id: String,
        agent_id: String,
        provider_id: String,
        task_id: String,
        delegation: Option<DelegationSpec>,
    },
    PromptAssembled {
        ts_ms: u128,
        elapsed_ms: u128,
        step_id: String,
        prompt_hash: String,
    },
    StepOutputChunk {
        ts_ms: u128,
        elapsed_ms: u128,
        step_id: String,
        chunk_bytes: usize,
    },
    DelegationRequested {
        ts_ms: u128,
        elapsed_ms: u128,
        delegation_id: String,
        step_id: String,
        action_kind: String,
        target_id: String,
    },
    DelegationPolicyEvaluated {
        ts_ms: u128,
        elapsed_ms: u128,
        delegation_id: String,
        step_id: String,
        action_kind: String,
        target_id: String,
        decision: String,
        rule_id: Option<String>,
    },
    DelegationApproved {
        ts_ms: u128,
        elapsed_ms: u128,
        delegation_id: String,
        step_id: String,
    },
    DelegationDenied {
        ts_ms: u128,
        elapsed_ms: u128,
        delegation_id: String,
        step_id: String,
        action_kind: String,
        target_id: String,
        rule_id: Option<String>,
    },
    DelegationDispatched {
        ts_ms: u128,
        elapsed_ms: u128,
        delegation_id: String,
        step_id: String,
        action_kind: String,
        target_id: String,
    },
    DelegationResultReceived {
        ts_ms: u128,
        elapsed_ms: u128,
        delegation_id: String,
        step_id: String,
        success: bool,
        output_bytes: usize,
    },
    DelegationCompleted {
        ts_ms: u128,
        elapsed_ms: u128,
        delegation_id: String,
        step_id: String,
        outcome: String,
    },
    StepFinished {
        ts_ms: u128,
        elapsed_ms: u128,
        step_id: String,
        success: bool,
        duration_ms: u128,
    },
    CallEntered {
        ts_ms: u128,
        elapsed_ms: u128,
        caller_step_id: String,
        callee_workflow_id: String,
        namespace: String,
    },
    CallExited {
        ts_ms: u128,
        elapsed_ms: u128,
        caller_step_id: String,
        status: String,
        namespace: String,
    },
}

impl TraceEvent {
    pub fn summarize(&self) -> String {
        match self {
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

impl Trace {
    pub fn new(
        run_id: impl Into<String>,
        workflow_id: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        let started = Self::now_ms();
        Self {
            run_id: run_id.into(),
            workflow_id: workflow_id.into(),
            version: version.into(),
            events: Vec::new(),
            run_started_ms: started,
            run_started_instant: Instant::now(),
            step_started_ms: HashMap::new(),
            delegation_ids: HashMap::new(),
            next_delegation_counter: 0,
        }
    }

    fn now_ms() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0)
    }

    pub fn step_started(
        &mut self,
        step_id: &str,
        agent_id: &str,
        provider_id: &str,
        task_id: &str,
        delegation: Option<&DelegationSpec>,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::StepStarted {
            ts_ms,
            elapsed_ms,
            step_id: step_id.to_string(),
            agent_id: agent_id.to_string(),
            provider_id: provider_id.to_string(),
            task_id: task_id.to_string(),
            delegation: delegation.cloned(),
        });
        self.step_started_ms.insert(step_id.to_string(), elapsed_ms);
    }

    pub fn run_failed(&mut self, message: &str) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::RunFailed {
            ts_ms,
            elapsed_ms,
            message: message.to_string(),
        });
    }

    pub fn scheduler_policy(&mut self, max_concurrency: usize, source: &str) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::SchedulerPolicy {
            ts_ms,
            elapsed_ms,
            max_concurrency,
            source: source.to_string(),
        });
    }

    pub fn prompt_assembled(&mut self, step_id: &str, prompt_hash: &str) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::PromptAssembled {
            ts_ms,
            elapsed_ms,
            step_id: step_id.to_string(),
            prompt_hash: prompt_hash.to_string(),
        });
    }

    pub fn step_output_chunk(&mut self, step_id: &str, chunk_bytes: usize) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::StepOutputChunk {
            ts_ms,
            elapsed_ms,
            step_id: step_id.to_string(),
            chunk_bytes,
        });
    }

    fn delegation_id_for_step(&mut self, step_id: &str) -> String {
        if let Some(existing) = self.delegation_ids.get(step_id) {
            return existing.clone();
        }
        self.next_delegation_counter = self.next_delegation_counter.saturating_add(1);
        let delegation_id = format!("del-{}", self.next_delegation_counter);
        self.delegation_ids
            .insert(step_id.to_string(), delegation_id.clone());
        delegation_id
    }

    pub fn delegation_requested(&mut self, step_id: &str, action_kind: &str, target_id: &str) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        let delegation_id = self.delegation_id_for_step(step_id);
        self.events.push(TraceEvent::DelegationRequested {
            ts_ms,
            elapsed_ms,
            delegation_id,
            step_id: step_id.to_string(),
            action_kind: action_kind.to_string(),
            target_id: target_id.to_string(),
        });
    }

    pub fn delegation_policy_evaluated(
        &mut self,
        step_id: &str,
        action_kind: &str,
        target_id: &str,
        decision: &str,
        rule_id: Option<&str>,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        let delegation_id = self.delegation_id_for_step(step_id);
        self.events.push(TraceEvent::DelegationPolicyEvaluated {
            ts_ms,
            elapsed_ms,
            delegation_id,
            step_id: step_id.to_string(),
            action_kind: action_kind.to_string(),
            target_id: target_id.to_string(),
            decision: decision.to_string(),
            rule_id: rule_id.map(|v| v.to_string()),
        });
    }

    pub fn delegation_approved(&mut self, step_id: &str) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        let delegation_id = self.delegation_id_for_step(step_id);
        self.events.push(TraceEvent::DelegationApproved {
            ts_ms,
            elapsed_ms,
            delegation_id,
            step_id: step_id.to_string(),
        });
    }

    pub fn delegation_denied(
        &mut self,
        step_id: &str,
        action_kind: &str,
        target_id: &str,
        rule_id: Option<&str>,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        let delegation_id = self.delegation_id_for_step(step_id);
        self.events.push(TraceEvent::DelegationDenied {
            ts_ms,
            elapsed_ms,
            delegation_id,
            step_id: step_id.to_string(),
            action_kind: action_kind.to_string(),
            target_id: target_id.to_string(),
            rule_id: rule_id.map(|v| v.to_string()),
        });
    }

    pub fn delegation_dispatched(&mut self, step_id: &str, action_kind: &str, target_id: &str) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        let delegation_id = self.delegation_id_for_step(step_id);
        self.events.push(TraceEvent::DelegationDispatched {
            ts_ms,
            elapsed_ms,
            delegation_id,
            step_id: step_id.to_string(),
            action_kind: action_kind.to_string(),
            target_id: target_id.to_string(),
        });
    }

    pub fn delegation_result_received(
        &mut self,
        step_id: &str,
        success: bool,
        output_bytes: usize,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        let delegation_id = self.delegation_id_for_step(step_id);
        self.events.push(TraceEvent::DelegationResultReceived {
            ts_ms,
            elapsed_ms,
            delegation_id,
            step_id: step_id.to_string(),
            success,
            output_bytes,
        });
    }

    pub fn delegation_completed(&mut self, step_id: &str, outcome: &str) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        let delegation_id = self.delegation_id_for_step(step_id);
        self.events.push(TraceEvent::DelegationCompleted {
            ts_ms,
            elapsed_ms,
            delegation_id,
            step_id: step_id.to_string(),
            outcome: outcome.to_string(),
        });
    }

    pub fn step_finished(&mut self, step_id: &str, success: bool) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        let duration_ms = self
            .step_started_ms
            .remove(step_id)
            .map(|started| elapsed_ms.saturating_sub(started))
            .unwrap_or(0);
        self.events.push(TraceEvent::StepFinished {
            ts_ms,
            elapsed_ms,
            step_id: step_id.to_string(),
            success,
            duration_ms,
        });
    }

    pub fn call_entered(
        &mut self,
        caller_step_id: &str,
        callee_workflow_id: &str,
        namespace: &str,
    ) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::CallEntered {
            ts_ms,
            elapsed_ms,
            caller_step_id: caller_step_id.to_string(),
            callee_workflow_id: callee_workflow_id.to_string(),
            namespace: namespace.to_string(),
        });
    }

    pub fn call_exited(&mut self, caller_step_id: &str, status: &str, namespace: &str) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::CallExited {
            ts_ms,
            elapsed_ms,
            caller_step_id: caller_step_id.to_string(),
            status: status.to_string(),
            namespace: namespace.to_string(),
        });
    }

    pub fn run_finished(&mut self, success: bool) {
        let elapsed_ms = self.run_started_instant.elapsed().as_millis();
        let ts_ms = self.run_started_ms.saturating_add(elapsed_ms);
        self.events.push(TraceEvent::RunFinished {
            ts_ms,
            elapsed_ms,
            success,
        });
    }

    pub fn current_elapsed_ms(&self) -> u128 {
        self.run_started_instant.elapsed().as_millis()
    }

    pub fn current_ts_ms(&self) -> u128 {
        self.run_started_ms
            .saturating_add(self.run_started_instant.elapsed().as_millis())
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

    #[test]
    fn trace_records_step_lifecycle_events_in_order() {
        let mut tr = Trace::new("run-1", "workflow-1", "0.1");

        tr.step_started("step-1", "agent-1", "provider-1", "task-1", None);
        tr.prompt_assembled("step-1", "hash-123");
        tr.step_output_chunk("step-1", 5);
        tr.step_finished("step-1", true);

        assert_eq!(tr.events.len(), 4);

        match &tr.events[0] {
            TraceEvent::StepStarted { step_id, .. } => {
                assert_eq!(step_id, "step-1");
            }
            _ => panic!("expected StepStarted event"),
        }

        match &tr.events[1] {
            TraceEvent::PromptAssembled { prompt_hash, .. } => {
                assert_eq!(prompt_hash, "hash-123");
            }
            _ => panic!("expected PromptAssembled event"),
        }

        match &tr.events[2] {
            TraceEvent::StepOutputChunk { chunk_bytes, .. } => {
                assert_eq!(*chunk_bytes, 5);
            }
            _ => panic!("expected StepOutputChunk event"),
        }

        match &tr.events[3] {
            TraceEvent::StepFinished {
                success,
                duration_ms,
                ..
            } => {
                assert!(*success);
                assert!(*duration_ms <= 1_000);
            }
            _ => panic!("expected StepFinished event"),
        }
    }

    #[test]
    fn trace_records_delegation_denied_event_with_optional_rule_id() {
        let mut tr = Trace::new("run-1", "wf-1", "0.7");
        tr.delegation_denied("s1", "provider_call", "local", Some("deny-local"));
        tr.delegation_denied("s2", "provider_call", "backup", None);

        let line0 = tr.events[0].summarize();
        let line1 = tr.events[1].summarize();
        assert!(line0.contains("DelegationDenied"));
        assert!(line0.contains("action=provider_call"));
        assert!(line0.contains("target=local"));
        assert!(line0.contains("rule_id=deny-local"));
        assert!(line1.contains("DelegationDenied"));
        assert!(line1.contains("action=provider_call"));
        assert!(line1.contains("target=backup"));
        assert!(!line1.contains("rule_id="));
    }

    #[test]
    fn trace_records_scheduler_policy_event() {
        let mut tr = Trace::new("run-1", "wf-1", "0.7");
        tr.scheduler_policy(4, "engine_default");
        assert_eq!(tr.events.len(), 1);
        match &tr.events[0] {
            TraceEvent::SchedulerPolicy {
                max_concurrency,
                source,
                ..
            } => {
                assert_eq!(*max_concurrency, 4);
                assert_eq!(source, "engine_default");
                let summary = tr.events[0].summarize();
                assert!(summary.contains("SchedulerPolicy"));
                assert!(summary.contains("max_concurrency=4"));
                assert!(summary.contains("source=engine_default"));
            }
            _ => panic!("expected SchedulerPolicy event"),
        }
    }

    #[test]
    fn trace_records_delegation_policy_event_with_optional_rule_id() {
        let mut tr = Trace::new("run-del-policy", "wf-del-policy", "0.7");
        tr.delegation_policy_evaluated(
            "s1",
            "provider_call",
            "local",
            "denied",
            Some("deny-local"),
        );
        tr.delegation_policy_evaluated("s2", "provider_call", "backup", "allowed", None);
        assert_eq!(tr.events.len(), 2);

        let line0 = tr.events[0].summarize();
        assert!(line0.contains("DelegationPolicyEvaluated"));
        assert!(line0.contains("action=provider_call"));
        assert!(line0.contains("target=local"));
        assert!(line0.contains("decision=denied"));
        assert!(line0.contains("rule_id=deny-local"));

        let line1 = tr.events[1].summarize();
        assert!(line1.contains("decision=allowed"));
        assert!(!line1.contains("rule_id="));
    }

    #[test]
    fn trace_preserves_run_and_workflow_ids() {
        let tr = Trace::new("run-x", "workflow-y", "0.1");

        assert_eq!(tr.run_id, "run-x");
        assert_eq!(tr.workflow_id, "workflow-y");
    }

    #[test]
    fn trace_allows_multiple_steps() {
        let mut tr = Trace::new("run-2", "workflow-2", "0.1");

        tr.step_started("step-a", "agent-a", "provider-a", "task-a", None);
        tr.step_finished("step-a", true);

        tr.step_started("step-b", "agent-b", "provider-b", "task-b", None);
        tr.step_finished("step-b", false);

        assert_eq!(tr.events.len(), 4);
    }

    #[test]
    fn trace_records_call_events() {
        let mut tr = Trace::new("run-3", "workflow-3", "0.5");
        tr.call_entered("parent", "child", "ns");
        tr.call_exited("parent", "success", "ns");
        assert_eq!(tr.events.len(), 2);
    }

    #[test]
    fn delegation_ids_are_stable_per_step_and_increment_in_order() {
        let mut tr = Trace::new("run-del", "workflow-del", "0.7");
        tr.delegation_requested("s1", "provider_call", "local");
        tr.delegation_policy_evaluated("s1", "provider_call", "local", "allowed", None);
        tr.delegation_requested("s2", "remote_exec", "remote");

        let ids: Vec<String> = tr
            .events
            .iter()
            .filter_map(|event| match event {
                TraceEvent::DelegationRequested { delegation_id, .. } => {
                    Some(delegation_id.clone())
                }
                TraceEvent::DelegationPolicyEvaluated { delegation_id, .. } => {
                    Some(delegation_id.clone())
                }
                _ => None,
            })
            .collect();
        assert_eq!(ids, vec!["del-1", "del-1", "del-2"]);
    }

    #[test]
    fn delegation_event_summaries_are_stable_and_safe() {
        let mut tr = Trace::new("run-del", "workflow-del", "0.7");
        tr.delegation_requested("s1", "provider_call", "local");
        tr.delegation_policy_evaluated("s1", "provider_call", "local", "allowed", None);
        tr.delegation_dispatched("s1", "provider_call", "local");
        tr.delegation_result_received("s1", true, 12);
        tr.delegation_completed("s1", "success");

        let lines: Vec<String> = tr.events.iter().map(TraceEvent::summarize).collect();
        assert!(lines.iter().any(|line| line.contains(
            "DelegationRequested delegation_id=del-1 step=s1 action=provider_call target=local"
        )));
        assert!(lines.iter().any(|line| line.contains(
            "DelegationPolicyEvaluated delegation_id=del-1 step=s1 action=provider_call target=local decision=allowed"
        )));
        assert!(lines.iter().any(|line| line.contains(
            "DelegationDispatched delegation_id=del-1 step=s1 action=provider_call target=local"
        )));
        assert!(lines.iter().any(|line| line.contains(
            "DelegationResultReceived delegation_id=del-1 step=s1 success=true bytes=12"
        )));
        assert!(lines
            .iter()
            .any(|line| line
                .contains("DelegationCompleted delegation_id=del-1 step=s1 outcome=success")));
    }

    #[test]
    fn trace_step_started_includes_delegation_when_present() {
        let mut tr = Trace::new("run-del", "workflow-del", "0.6");
        tr.step_started(
            "step-del",
            "agent-del",
            "provider-del",
            "task-del",
            Some(&DelegationSpec {
                role: Some("reviewer".to_string()),
                requires_verification: Some(true),
                escalation_target: Some("human".to_string()),
                tags: vec!["safety".to_string(), "compliance".to_string()],
            }),
        );
        let line = tr.events[0].summarize();
        assert!(
            line.contains("delegation={\"role\":\"reviewer\",\"requires_verification\":true,\"escalation_target\":\"human\",\"tags\":[\"compliance\",\"safety\"]}"),
            "line was:\n{line}"
        );
    }
}
