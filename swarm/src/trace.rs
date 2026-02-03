use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Trace {
    pub run_id: String,
    pub workflow_id: String,
    pub events: Vec<TraceEvent>,
}

#[derive(Debug, Clone)]
pub enum TraceEvent {
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
    },
}

impl TraceEvent {
    pub fn summarize(&self) -> String {
        match self {
            TraceEvent::StepStarted {
                ts_ms,
                step_id,
                agent_id,
                provider_id,
                task_id,
            } => format!(
                "{ts_ms} StepStarted step={step_id} agent={agent_id} provider={provider_id} task={task_id}"
            ),
            TraceEvent::PromptAssembled {
                ts_ms,
                step_id,
                prompt_hash,
            } => format!("{ts_ms} PromptAssembled step={step_id} hash={prompt_hash}"),
            TraceEvent::StepFinished {
                ts_ms,
                step_id,
                success,
            } => format!("{ts_ms} StepFinished step={step_id} success={success}"),
        }
    }
}

impl Trace {
    pub fn new(run_id: impl Into<String>, workflow_id: impl Into<String>) -> Self {
        Self {
            run_id: run_id.into(),
            workflow_id: workflow_id.into(),
            events: Vec::new(),
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
        self.events.push(TraceEvent::StepStarted {
            ts_ms: Self::now_ms(),
            step_id: step_id.to_string(),
            agent_id: agent_id.to_string(),
            provider_id: provider_id.to_string(),
            task_id: task_id.to_string(),
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
        self.events.push(TraceEvent::StepFinished {
            ts_ms: Self::now_ms(),
            step_id: step_id.to_string(),
            success,
        });
    }
}

/// Print a human-readable trace to stdout (stable + diff-friendly).
pub fn print_trace(tr: &Trace) {
    println!("TRACE run_id={} workflow_id={}", tr.run_id, tr.workflow_id);
    for ev in &tr.events {
        println!("{}", ev.summarize());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_records_step_lifecycle_events_in_order() {
        let mut tr = Trace::new("run-1", "workflow-1");

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
            TraceEvent::StepFinished { success, .. } => {
                assert!(*success);
            }
            _ => panic!("expected StepFinished event"),
        }
    }

    #[test]
    fn trace_preserves_run_and_workflow_ids() {
        let tr = Trace::new("run-x", "workflow-y");

        assert_eq!(tr.run_id, "run-x");
        assert_eq!(tr.workflow_id, "workflow-y");
    }

    #[test]
    fn trace_allows_multiple_steps() {
        let mut tr = Trace::new("run-2", "workflow-2");

        tr.step_started("step-a", "agent-a", "provider-a", "task-a");
        tr.step_finished("step-a", true);

        tr.step_started("step-b", "agent-b", "provider-b", "task-b");
        tr.step_finished("step-b", false);

        assert_eq!(tr.events.len(), 4);
    }
}
