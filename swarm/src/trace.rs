use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Trace {
    pub run_id: String,
    pub workflow_id: String,
    pub version: String,
    pub events: Vec<TraceEvent>,
    run_started_ms: u128,
    step_started_ms: HashMap<String, u128>,
}

#[derive(Debug, Clone)]
pub enum TraceEvent {
    RunFailed {
        ts_ms: u128,
        message: String,
    },
    RunFinished {
        ts_ms: u128,
        success: bool,
        elapsed_ms: u128,
    },
    StepStarted {
        ts_ms: u128,
        step_id: String,
        agent_id: String,
        provider_id: String,
        task_id: String,
    },
    PromptAssembled {
        ts_ms: u128,
        step_id: String,
        prompt_hash: String,
    },
    StepFinished {
        ts_ms: u128,
        step_id: String,
        success: bool,
        elapsed_ms: u128,
    },
}

impl TraceEvent {
    pub fn summarize(&self, enhanced: bool) -> String {
        match self {
            TraceEvent::RunFailed { ts_ms, message } => {
                if enhanced {
                    let ts = format_ts_ms(*ts_ms);
                    format!("{ts_ms} RunFailed ts={ts} message={message}")
                } else {
                    format!("{ts_ms} RunFailed message={message}")
                }
            }
            TraceEvent::RunFinished {
                ts_ms,
                success,
                elapsed_ms,
            } => {
                if enhanced {
                    let ts = format_ts_ms(*ts_ms);
                    let elapsed = format_elapsed_ms(*elapsed_ms);
                    format!(
                        "{ts_ms} RunFinished ts={ts} success={success} elapsed_ms={elapsed_ms} elapsed={elapsed}"
                    )
                } else {
                    format!("{ts_ms} RunFinished success={success}")
                }
            }
            TraceEvent::StepStarted {
                ts_ms,
                step_id,
                agent_id,
                provider_id,
                task_id,
            } => {
                if enhanced {
                    let ts = format_ts_ms(*ts_ms);
                    format!(
                        "{ts_ms} StepStarted ts={ts} step={step_id} agent={agent_id} provider={provider_id} task={task_id}"
                    )
                } else {
                    format!(
                        "{ts_ms} StepStarted step={step_id} agent={agent_id} provider={provider_id} task={task_id}"
                    )
                }
            }
            TraceEvent::PromptAssembled {
                ts_ms,
                step_id,
                prompt_hash,
            } => {
                if enhanced {
                    let ts = format_ts_ms(*ts_ms);
                    format!("{ts_ms} PromptAssembled ts={ts} step={step_id} hash={prompt_hash}")
                } else {
                    format!("{ts_ms} PromptAssembled step={step_id} hash={prompt_hash}")
                }
            }
            TraceEvent::StepFinished {
                ts_ms,
                step_id,
                success,
                elapsed_ms,
            } => {
                if enhanced {
                    let ts = format_ts_ms(*ts_ms);
                    let elapsed = format_elapsed_ms(*elapsed_ms);
                    format!(
                        "{ts_ms} StepFinished ts={ts} step={step_id} success={success} elapsed_ms={elapsed_ms} elapsed={elapsed}"
                    )
                } else {
                    format!("{ts_ms} StepFinished step={step_id} success={success}")
                }
            }
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
    ) {
        let ts_ms = Self::now_ms();
        self.events.push(TraceEvent::StepStarted {
            ts_ms,
            step_id: step_id.to_string(),
            agent_id: agent_id.to_string(),
            provider_id: provider_id.to_string(),
            task_id: task_id.to_string(),
        });
        self.step_started_ms.insert(step_id.to_string(), ts_ms);
    }

    pub fn run_failed(&mut self, message: &str) {
        self.events.push(TraceEvent::RunFailed {
            ts_ms: Self::now_ms(),
            message: message.to_string(),
        });
    }

    pub fn prompt_assembled(&mut self, step_id: &str, prompt_hash: &str) {
        self.events.push(TraceEvent::PromptAssembled {
            ts_ms: Self::now_ms(),
            step_id: step_id.to_string(),
            prompt_hash: prompt_hash.to_string(),
        });
    }

    pub fn step_finished(&mut self, step_id: &str, success: bool) {
        let ts_ms = Self::now_ms();
        let elapsed_ms = self
            .step_started_ms
            .remove(step_id)
            .map(|started| ts_ms.saturating_sub(started))
            .unwrap_or(0);
        self.events.push(TraceEvent::StepFinished {
            ts_ms,
            step_id: step_id.to_string(),
            success,
            elapsed_ms,
        });
    }

    pub fn run_finished(&mut self, success: bool) {
        let ts_ms = Self::now_ms();
        let elapsed_ms = ts_ms.saturating_sub(self.run_started_ms);
        self.events.push(TraceEvent::RunFinished {
            ts_ms,
            success,
            elapsed_ms,
        });
    }
}

/// Print a human-readable trace to stdout (stable + diff-friendly).
pub fn print_trace(tr: &Trace) {
    let enhanced = tr.version.trim() == "0.2";
    println!(
        "TRACE run_id={} workflow_id={} version={}",
        tr.run_id, tr.workflow_id, tr.version
    );
    for ev in &tr.events {
        println!("{}", ev.summarize(enhanced));
    }
}

fn format_ts_ms(ts_ms: u128) -> String {
    // Human-readable UTC-like timestamp without extra dependencies.
    // Kept deterministic and cross-platform for tests/log parsing.
    let secs = ts_ms / 1000;
    let millis = ts_ms % 1000;
    format!("{secs}.{millis:03}Z")
}

fn format_elapsed_ms(elapsed_ms: u128) -> String {
    let secs = elapsed_ms as f64 / 1000.0;
    format!("{secs:.2}s")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_records_step_lifecycle_events_in_order() {
        let mut tr = Trace::new("run-1", "workflow-1", "0.1");

        tr.step_started("step-1", "agent-1", "provider-1", "task-1");
        tr.prompt_assembled("step-1", "hash-123");
        tr.step_finished("step-1", true);

        assert_eq!(tr.events.len(), 3);

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
            TraceEvent::StepFinished {
                success,
                elapsed_ms,
                ..
            } => {
                assert!(*success);
                assert!(*elapsed_ms <= 1_000);
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

        tr.step_started("step-a", "agent-a", "provider-a", "task-a");
        tr.step_finished("step-a", true);

        tr.step_started("step-b", "agent-b", "provider-b", "task-b");
        tr.step_finished("step-b", false);

        assert_eq!(tr.events.len(), 4);
    }
}
