use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

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

#[derive(Clone, Debug, Default)]
pub struct ChannelMetrics {
    sent: Arc<AtomicU64>,
    rejected: Arc<AtomicU64>,
}

impl ChannelMetrics {
    pub fn sent(&self) -> u64 {
        self.sent.load(Ordering::Relaxed)
    }

    pub fn rejected(&self) -> u64 {
        self.rejected.load(Ordering::Relaxed)
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
            ChannelFullPolicy::Block => self.tx.send(value).await.map_err(|_| SendError::Closed),
            ChannelFullPolicy::Reject => self.tx.try_send(value).map_err(|error| match error {
                mpsc::error::TrySendError::Full(_) => SendError::Full,
                mpsc::error::TrySendError::Closed(_) => SendError::Closed,
            }),
        };
        match result {
            Ok(()) => {
                self.metrics.sent.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
            Err(error) => {
                if error == SendError::Full {
                    self.metrics.rejected.fetch_add(1, Ordering::Relaxed);
                }
                Err(error)
            }
        }
    }

    pub fn metrics(&self) -> ChannelMetrics {
        self.metrics.clone()
    }
}

#[derive(Debug)]
pub struct BoundedReceiver<T>(mpsc::Receiver<T>);

impl<T> BoundedReceiver<T> {
    pub async fn recv(&mut self) -> Option<T> {
        self.0.recv().await
    }
}

pub fn channel<T>(
    capacity: usize,
    policy: ChannelFullPolicy,
) -> (BoundedSender<T>, BoundedReceiver<T>) {
    assert!(capacity > 0, "bounded channel capacity must be non-zero");
    let (tx, rx) = mpsc::channel(capacity);
    let metrics = ChannelMetrics::default();
    (
        BoundedSender {
            tx,
            policy,
            metrics,
        },
        BoundedReceiver(rx),
    )
}
