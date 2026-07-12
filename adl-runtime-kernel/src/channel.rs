use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelFullPolicy {
    Block,
    Reject,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum SendError {
    #[error("bounded channel is full")]
    Full,
    #[error("bounded channel is closed")]
    Closed,
}

#[derive(Clone, Debug)]
pub struct ChannelMetrics {
    capacity: usize,
    state: Arc<Mutex<ChannelMetricState>>,
}

#[derive(Clone, Copy, Debug, Default)]
struct ChannelMetricState {
    generation: u64,
    sent: u64,
    rejected: u64,
    depth: u64,
    high_water: u64,
}

impl ChannelMetrics {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            state: Arc::new(Mutex::new(ChannelMetricState::default())),
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn sent(&self) -> u64 {
        self.state
            .lock()
            .expect("channel metrics mutex poisoned")
            .sent
    }

    pub fn rejected(&self) -> u64 {
        self.state
            .lock()
            .expect("channel metrics mutex poisoned")
            .rejected
    }

    pub fn depth(&self) -> u64 {
        self.state
            .lock()
            .expect("channel metrics mutex poisoned")
            .depth
    }

    pub fn high_water(&self) -> u64 {
        self.state
            .lock()
            .expect("channel metrics mutex poisoned")
            .high_water
    }

    pub fn snapshot(&self) -> (u64, usize, u64, u64, u64, u64) {
        let state = *self.state.lock().expect("channel metrics mutex poisoned");
        (
            state.generation,
            self.capacity,
            state.depth,
            state.high_water,
            state.sent,
            state.rejected,
        )
    }

    fn record_rejected(&self) {
        let mut state = self.state.lock().expect("channel metrics mutex poisoned");
        state.generation += 1;
        state.rejected += 1;
    }

    fn record_enqueue(&self) {
        let mut state = self.state.lock().expect("channel metrics mutex poisoned");
        state.generation += 1;
        state.sent += 1;
        state.depth += 1;
        state.high_water = state.high_water.max(state.depth);
    }

    fn record_dequeue(&self) {
        let mut state = self.state.lock().expect("channel metrics mutex poisoned");
        state.generation += 1;
        state.depth = state.depth.saturating_sub(1);
    }
}

#[derive(Debug)]
pub struct BoundedSender<T> {
    tx: mpsc::Sender<T>,
    policy: ChannelFullPolicy,
    metrics: ChannelMetrics,
}

impl<T> Clone for BoundedSender<T> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            policy: self.policy,
            metrics: self.metrics.clone(),
        }
    }
}

impl<T> BoundedSender<T> {
    pub async fn send(&self, value: T) -> Result<(), SendError> {
        let result = match self.policy {
            ChannelFullPolicy::Block => {
                self.tx
                    .reserve()
                    .await
                    .map_err(|_| SendError::Closed)
                    .map(|permit| {
                        self.record_enqueue();
                        permit.send(value);
                    })
            }
            ChannelFullPolicy::Reject => self
                .tx
                .try_reserve()
                .map_err(|error| match error {
                    mpsc::error::TrySendError::Full(_) => SendError::Full,
                    mpsc::error::TrySendError::Closed(_) => SendError::Closed,
                })
                .map(|permit| {
                    self.record_enqueue();
                    permit.send(value);
                }),
        };
        match result {
            Ok(()) => Ok(()),
            Err(error) => {
                if error == SendError::Full {
                    self.metrics.record_rejected();
                }
                Err(error)
            }
        }
    }

    pub fn metrics(&self) -> ChannelMetrics {
        self.metrics.clone()
    }

    fn record_enqueue(&self) {
        self.metrics.record_enqueue();
    }
}

#[derive(Debug)]
pub struct BoundedReceiver<T> {
    rx: mpsc::Receiver<T>,
    metrics: ChannelMetrics,
}

impl<T> BoundedReceiver<T> {
    pub async fn recv(&mut self) -> Option<T> {
        let value = self.rx.recv().await;
        if value.is_some() {
            self.metrics.record_dequeue();
        }
        value
    }

    pub fn try_recv(&mut self) -> Result<T, mpsc::error::TryRecvError> {
        let value = self.rx.try_recv()?;
        self.metrics.record_dequeue();
        Ok(value)
    }
}

pub fn channel<T>(
    capacity: usize,
    policy: ChannelFullPolicy,
) -> (BoundedSender<T>, BoundedReceiver<T>) {
    assert!(capacity > 0, "bounded channel capacity must be non-zero");
    let (tx, rx) = mpsc::channel(capacity);
    let metrics = ChannelMetrics::new(capacity);
    (
        BoundedSender {
            tx,
            policy,
            metrics: metrics.clone(),
        },
        BoundedReceiver { rx, metrics },
    )
}
