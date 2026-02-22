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
}

#[derive(Debug, Clone)]
pub enum TraceEvent {
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
