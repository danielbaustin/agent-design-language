use std::{
    collections::{BTreeMap, VecDeque},
    sync::{Arc, Mutex, RwLock},
    time::Instant,
};

use serde::{Deserialize, Serialize};

use crate::{ComponentId, RunningState};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ClockAuthority {
    Degraded { reason: String },
    Authoritative { source: String, unix_millis: u64 },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BootstrapEvent {
    pub sequence: u64,
    pub monotonic_millis: u64,
    pub component: Option<ComponentId>,
    pub event: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeSnapshot {
    pub components: BTreeMap<ComponentId, RunningState>,
    pub clock: ClockAuthority,
    pub event_count: usize,
    pub observability_ready: bool,
}

#[derive(Debug)]
struct RecorderState {
    next_sequence: u64,
    startup: VecDeque<BootstrapEvent>,
    retained: Vec<BootstrapEvent>,
    promoted: bool,
}

#[derive(Clone, Debug)]
pub struct RuntimeRecorder {
    started: Instant,
    capacity: usize,
    events: Arc<Mutex<RecorderState>>,
    components: Arc<RwLock<BTreeMap<ComponentId, RunningState>>>,
    clock: Arc<RwLock<ClockAuthority>>,
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
            events: Arc::new(Mutex::new(RecorderState {
                next_sequence: 0,
                startup: VecDeque::with_capacity(startup_capacity),
                retained: Vec::new(),
                promoted: false,
            })),
            components: Arc::new(RwLock::new(BTreeMap::new())),
            clock: Arc::new(RwLock::new(ClockAuthority::Degraded {
                reason: "wall_clock_unsynchronized".to_owned(),
            })),
        }
    }

    pub fn emit(&self, component: Option<ComponentId>, event: impl Into<String>) {
        let event = {
            let mut state = self.events.lock().expect("recorder event mutex poisoned");
            let event = BootstrapEvent {
                sequence: state.next_sequence,
                monotonic_millis: self
                    .started
                    .elapsed()
                    .as_millis()
                    .try_into()
                    .unwrap_or(u64::MAX),
                component,
                event: event.into(),
            };
            state.next_sequence += 1;
            if state.promoted {
                state.retained.push(event.clone());
            } else {
                if state.startup.len() == self.capacity {
                    state.startup.pop_front();
                }
                state.startup.push_back(event.clone());
            }
            event
        };
        tracing::info!(
            target: "adl_runtime_kernel",
            sequence = event.sequence,
            monotonic_millis = event.monotonic_millis,
            component = ?event.component,
            event = %event.event,
            "runtime event"
        );
        eprintln!(
            "adl_kernel_event {}",
            serde_json::to_string(&event).expect("bootstrap event serialization cannot fail")
        );
    }

    pub fn promote_observability(&self) -> Vec<BootstrapEvent> {
        let mut state = self.events.lock().expect("recorder event mutex poisoned");
        if state.promoted {
            return Vec::new();
        }
        let buffered = state.startup.drain(..).collect::<Vec<_>>();
        state.retained.extend(buffered.iter().cloned());
        state.promoted = true;
        buffered
    }

    pub fn set_component_state(&self, id: ComponentId, state: RunningState) {
        self.components
            .write()
            .expect("component state lock poisoned")
            .insert(id.clone(), state);
        self.emit(Some(id), format!("state:{state:?}"));
    }

    pub fn set_clock_authority(&self, authority: ClockAuthority) {
        *self.clock.write().expect("clock state lock poisoned") = authority;
        self.emit(None, "clock_authority_updated");
    }

    pub fn events(&self) -> Vec<BootstrapEvent> {
        let state = self.events.lock().expect("recorder event mutex poisoned");
        if state.promoted {
            state.retained.clone()
        } else {
            state.startup.iter().cloned().collect()
        }
    }

    pub fn snapshot(&self) -> RuntimeSnapshot {
        let events = self.events.lock().expect("recorder event mutex poisoned");
        RuntimeSnapshot {
            components: self
                .components
                .read()
                .expect("component state lock poisoned")
                .clone(),
            clock: self
                .clock
                .read()
                .expect("clock state lock poisoned")
                .clone(),
            event_count: if events.promoted {
                events.retained.len()
            } else {
                events.startup.len()
            },
            observability_ready: events.promoted,
        }
    }
}
