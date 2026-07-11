use std::{fmt, sync::Arc, time::Duration};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;

use crate::telemetry::RuntimeRecorder;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ComponentId(String);

impl ComponentId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ComponentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl From<&str> for ComponentId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PortSpec {
    pub name: String,
    pub message_type: String,
}

impl PortSpec {
    pub fn typed<T: 'static>(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            message_type: std::any::type_name::<T>().to_owned(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FailurePolicy {
    Fatal,
    Degrade,
    Restart {
        max_restarts: u32,
        backoff_millis: u64,
    },
}

impl FailurePolicy {
    pub fn restart(max_restarts: u32, backoff: Duration) -> Self {
        Self::Restart {
            max_restarts,
            backoff_millis: backoff.as_millis().try_into().unwrap_or(u64::MAX),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ComponentSpec {
    pub id: ComponentId,
    pub dependencies: Vec<ComponentId>,
    pub inputs: Vec<PortSpec>,
    pub outputs: Vec<PortSpec>,
    pub failure_policy: FailurePolicy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunningState {
    Starting,
    Ready,
    Running,
    Restarting,
    Degraded,
    Stopping,
    Stopped,
    Failed,
}

#[derive(Debug, Error)]
#[error("{message}")]
pub struct ComponentError {
    message: String,
}

impl ComponentError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

pub struct ComponentContext {
    pub id: ComponentId,
    pub cancellation: CancellationToken,
    pub recorder: RuntimeRecorder,
    ready: Option<oneshot::Sender<()>>,
}

impl ComponentContext {
    pub(crate) fn new(
        id: ComponentId,
        cancellation: CancellationToken,
        recorder: RuntimeRecorder,
        ready: oneshot::Sender<()>,
    ) -> Self {
        Self {
            id,
            cancellation,
            recorder,
            ready: Some(ready),
        }
    }

    pub fn ready(&mut self) {
        if let Some(ready) = self.ready.take() {
            let _ = ready.send(());
        }
    }
}

#[async_trait]
pub trait Component: Send + 'static {
    async fn run(self: Box<Self>, context: ComponentContext) -> Result<(), ComponentError>;
}

pub trait ComponentFactory: Send + Sync + 'static {
    fn spec(&self) -> ComponentSpec;
    fn build(&self) -> Box<dyn Component>;
}

impl<T> ComponentFactory for Arc<T>
where
    T: ComponentFactory + ?Sized,
{
    fn spec(&self) -> ComponentSpec {
        (**self).spec()
    }

    fn build(&self) -> Box<dyn Component> {
        (**self).build()
    }
}
