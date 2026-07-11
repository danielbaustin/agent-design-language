//! Independent, additive runtime-kernel proof for ADL issue #5170.

pub mod channel;
pub mod component;
pub mod contract;
pub mod proof;
pub mod supervisor;
pub mod telemetry;
pub mod topology;

pub use channel::{channel, BoundedReceiver, BoundedSender, ChannelFullPolicy};
pub use component::{
    Component, ComponentContext, ComponentError, ComponentFactory, ComponentId, ComponentSpec,
    FailurePolicy, PortSpec, RunningState,
};
pub use contract::*;
pub use supervisor::{Kernel, KernelError, KernelExit, KernelHandle};
pub use telemetry::{BootstrapEvent, ClockAuthority, RuntimeRecorder};
pub use topology::{ComponentRegistry, TopologyError, ValidatedTopology};
