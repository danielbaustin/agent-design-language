use std::{
    collections::{BTreeMap, VecDeque},
    sync::{Arc, Mutex},
    time::Instant,
};

use serde::{Deserialize, Serialize};

use crate::{channel::ChannelMetrics, ComponentId, RunningState};

pub const RUNTIME_SNAPSHOT_SCHEMA: &str = "adl.runtime.control_snapshot.v1";
pub const RUNTIME_MASTER_LOG_RECORD_SCHEMA: &str = "adl.runtime.master_log_record.v1";
pub const RUNTIME_MASTER_LOG_AUDIT_SCHEMA: &str = "adl.runtime.master_log_audit.v1";
pub const RUNTIME_OBSERVABILITY_PIPELINE_SCHEMA: &str = "adl.runtime_v3.observability.pipeline.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ClockAuthority {
    Degraded { reason: String },
    Authoritative { source: String, unix_millis: u64 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservabilityDegradation {
    ExporterUnavailable,
    ExporterRejected,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ObservabilityHealth {
    Pending,
    Ready,
    Degraded { reason: ObservabilityDegradation },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservabilityPipelineSnapshot {
    pub schema: String,
    pub health: ObservabilityHealth,
    pub runtime_instance_id: String,
    pub vector_pid: Option<u32>,
    pub vector_version: String,
    pub master_log_ref: String,
    pub log_audit_ref: String,
    pub otlp_endpoint: Option<String>,
    pub otlp_timeout_millis: u64,
    pub sequence_next: u64,
    pub drain_complete: bool,
    pub last_failure: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleState {
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeEvent {
    KernelBootstrapStarted,
    StartupEventsFlushed(usize),
    ComponentDegraded,
    ComponentState(RunningState),
    ClockAuthorityUpdated,
    ControlCommandCompleted,
    DomainWorkCompleted,
    KernelStarting,
    ComponentsReady,
}

impl RuntimeEvent {
    fn code(self) -> String {
        match self {
            Self::KernelBootstrapStarted => "kernel_bootstrap_started".to_owned(),
            Self::StartupEventsFlushed(count) => format!("startup_events_flushed:{count}"),
            Self::ComponentDegraded => "component_degraded".to_owned(),
            Self::ComponentState(state) => format!("state:{state:?}"),
            Self::ClockAuthorityUpdated => "clock_authority_updated".to_owned(),
            Self::ControlCommandCompleted => "control_command_completed".to_owned(),
            Self::DomainWorkCompleted => "domain_work_completed".to_owned(),
            Self::KernelStarting => "kernel_starting".to_owned(),
            Self::ComponentsReady => "components_ready".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct QueueHealth {
    pub generation: u64,
    pub capacity: usize,
    pub depth: u64,
    pub high_water: u64,
    pub sent: u64,
    pub rejected: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContinuityHead {
    pub generation: u64,
    pub accepted_through: u64,
    pub topology_hash: String,
    pub config_hash: String,
    pub integrity: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BootstrapEvent {
    pub sequence: u64,
    pub monotonic_millis: u64,
    pub component: Option<ComponentId>,
    pub event: String,
    pub correlation_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeSnapshot {
    pub schema: String,
    pub revision: u64,
    pub topology_generation: u64,
    pub components: BTreeMap<ComponentId, RunningState>,
    pub restart_counts: BTreeMap<ComponentId, u32>,
    pub queues: BTreeMap<String, QueueHealth>,
    pub clock: ClockAuthority,
    pub continuity_head: Option<ContinuityHead>,
    pub lifecycle: LifecycleState,
    pub event_count: usize,
    pub observability: ObservabilityHealth,
    pub observability_ready: bool,
    pub observability_pipeline: Option<ObservabilityPipelineSnapshot>,
}

#[derive(Debug)]
struct RecorderState {
    revision: u64,
    topology_generation: u64,
    next_sequence: u64,
    startup: VecDeque<BootstrapEvent>,
    retained: Vec<BootstrapEvent>,
    components: BTreeMap<ComponentId, RunningState>,
    restart_counts: BTreeMap<ComponentId, u32>,
    queues: BTreeMap<String, ChannelMetrics>,
    clock: ClockAuthority,
    continuity_head: Option<ContinuityHead>,
    observability: ObservabilityHealth,
    observability_pipeline: Option<ObservabilityPipelineSnapshot>,
    lifecycle: LifecycleState,
}

#[derive(Clone, Debug)]
pub struct RuntimeRecorder {
    started: Instant,
    capacity: usize,
    state: Arc<Mutex<RecorderState>>,
}

impl RuntimeRecorder {
    pub fn new(startup_capacity: usize) -> Self {
        assert!(
            startup_capacity > 0,
            "startup event capacity must be non-zero"
        );
        Self {
            started: Instant::now(),
            capacity: startup_capacity,
            state: Arc::new(Mutex::new(RecorderState {
                revision: 0,
                topology_generation: 0,
                next_sequence: 0,
                startup: VecDeque::with_capacity(startup_capacity),
                retained: Vec::new(),
                components: BTreeMap::new(),
                restart_counts: BTreeMap::new(),
                queues: BTreeMap::new(),
                clock: ClockAuthority::Degraded {
                    reason: "wall_clock_unsynchronized".to_owned(),
                },
                continuity_head: None,
                observability: ObservabilityHealth::Pending,
                observability_pipeline: None,
                lifecycle: LifecycleState::Starting,
            })),
        }
    }

    pub fn emit(&self, component: Option<ComponentId>, event: RuntimeEvent) {
        self.emit_correlated(component, event, None);
    }

    pub fn emit_correlated(
        &self,
        component: Option<ComponentId>,
        event: RuntimeEvent,
        correlation_id: Option<&str>,
    ) {
        let event = {
            let mut state = self.state.lock().expect("recorder state mutex poisoned");
            let event = BootstrapEvent {
                sequence: state.next_sequence,
                monotonic_millis: self
                    .started
                    .elapsed()
                    .as_millis()
                    .try_into()
                    .unwrap_or(u64::MAX),
                component: component.map(|id| ComponentId::new(safe_field(id.as_str()))),
                event: event.code(),
                correlation_id: correlation_id.map(safe_correlation),
            };
            state.next_sequence += 1;
            state.revision += 1;
            if matches!(state.observability, ObservabilityHealth::Pending) {
                if state.startup.len() == self.capacity {
                    state.startup.pop_front();
                }
                state.startup.push_back(event.clone());
            } else {
                state.retained.push(event.clone());
            }
            event
        };
        trace_event(&event, false);
        eprintln!(
            "adl_event schema=adl.runtime.event.v1 sequence={} component={} event={} correlation_id={}",
            event.sequence,
            event
                .component
                .as_ref()
                .map(ComponentId::as_str)
                .unwrap_or("none"),
            event.event,
            event.correlation_id.as_deref().unwrap_or("none")
        );
    }

    pub fn initialize_observability(&self, health: ObservabilityHealth) -> Vec<BootstrapEvent> {
        assert!(
            !matches!(health, ObservabilityHealth::Pending),
            "observability initialization requires a terminal readiness classification"
        );
        let mut state = self.state.lock().expect("recorder state mutex poisoned");
        if !matches!(state.observability, ObservabilityHealth::Pending) {
            return Vec::new();
        }
        let buffered = state.startup.drain(..).collect::<Vec<_>>();
        for event in &buffered {
            trace_event(event, true);
        }
        state.retained.extend(buffered.iter().cloned());
        state.observability = health;
        state.revision += 1;
        buffered
    }

    pub fn promote_observability(&self) -> Vec<BootstrapEvent> {
        self.initialize_observability(ObservabilityHealth::Ready)
    }

    pub fn set_observability_pipeline(&self, pipeline: ObservabilityPipelineSnapshot) {
        let mut state = self.state.lock().expect("recorder state mutex poisoned");
        if matches!(state.observability, ObservabilityHealth::Pending) {
            let buffered = state.startup.drain(..).collect::<Vec<_>>();
            for event in &buffered {
                trace_event(event, true);
            }
            state.retained.extend(buffered);
        }
        state.observability = pipeline.health.clone();
        state.observability_pipeline = Some(pipeline);
        state.revision += 1;
    }

    pub fn set_component_state(&self, id: ComponentId, running: RunningState) {
        {
            let mut state = self.state.lock().expect("recorder state mutex poisoned");
            state.components.insert(id.clone(), running);
            state.revision += 1;
        }
        self.emit(Some(id), RuntimeEvent::ComponentState(running));
    }

    pub fn set_restart_count(&self, id: ComponentId, count: u32) {
        let mut state = self.state.lock().expect("recorder state mutex poisoned");
        state.restart_counts.insert(id, count);
        state.revision += 1;
    }

    pub fn set_clock_authority(&self, authority: ClockAuthority) {
        {
            let mut state = self.state.lock().expect("recorder state mutex poisoned");
            state.clock = authority;
            state.revision += 1;
        }
        self.emit(None, RuntimeEvent::ClockAuthorityUpdated);
    }

    pub fn set_topology_generation(&self, generation: u64) {
        let mut state = self.state.lock().expect("recorder state mutex poisoned");
        state.topology_generation = generation;
        state.revision += 1;
    }

    pub fn set_queue_health(&self, name: impl Into<String>, metrics: &ChannelMetrics) {
        let mut state = self.state.lock().expect("recorder state mutex poisoned");
        state
            .queues
            .insert(safe_field(&name.into()), metrics.clone());
        state.revision += 1;
    }

    pub fn set_continuity_head(&self, head: ContinuityHead) {
        let mut state = self.state.lock().expect("recorder state mutex poisoned");
        state.continuity_head = Some(head);
        state.revision += 1;
    }

    pub fn set_lifecycle(&self, lifecycle: LifecycleState) {
        let mut state = self.state.lock().expect("recorder state mutex poisoned");
        state.lifecycle = lifecycle;
        state.revision += 1;
    }

    pub fn events(&self) -> Vec<BootstrapEvent> {
        let state = self.state.lock().expect("recorder state mutex poisoned");
        if matches!(state.observability, ObservabilityHealth::Pending) {
            state.startup.iter().cloned().collect()
        } else {
            state.retained.clone()
        }
    }

    pub fn snapshot(&self) -> RuntimeSnapshot {
        let state = self.state.lock().expect("recorder state mutex poisoned");
        RuntimeSnapshot {
            schema: RUNTIME_SNAPSHOT_SCHEMA.to_owned(),
            revision: state.revision,
            topology_generation: state.topology_generation,
            components: state.components.clone(),
            restart_counts: state.restart_counts.clone(),
            queues: state
                .queues
                .iter()
                .map(|(name, metrics)| {
                    let (generation, capacity, depth, high_water, sent, rejected) =
                        metrics.snapshot();
                    (
                        name.clone(),
                        QueueHealth {
                            generation,
                            capacity,
                            depth,
                            high_water,
                            sent,
                            rejected,
                        },
                    )
                })
                .collect(),
            clock: state.clock.clone(),
            continuity_head: state.continuity_head.clone(),
            lifecycle: state.lifecycle,
            event_count: if matches!(state.observability, ObservabilityHealth::Pending) {
                state.startup.len()
            } else {
                state.retained.len()
            },
            observability_ready: !matches!(state.observability, ObservabilityHealth::Pending),
            observability: state.observability.clone(),
            observability_pipeline: state.observability_pipeline.clone(),
        }
    }
}

fn safe_field(value: &str) -> String {
    if !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b':' | b'_' | b'-'))
    {
        value.to_owned()
    } else {
        "redacted".to_owned()
    }
}

fn safe_correlation(value: &str) -> String {
    if value.len() == 32
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        value.to_owned()
    } else {
        "redacted".to_owned()
    }
}

fn trace_event(event: &BootstrapEvent, promoted: bool) {
    tracing::info!(
        target: "adl_runtime_kernel",
        sequence = event.sequence,
        monotonic_millis = event.monotonic_millis,
        component = ?event.component,
        event = %event.event,
        correlation_id = ?event.correlation_id,
        promoted,
        "runtime event"
    );
}
