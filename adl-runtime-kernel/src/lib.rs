//! Independent, additive runtime-kernel proof for ADL issue #5170.

pub mod channel;
pub mod component;
pub mod config;
pub mod continuity;
pub mod contract;
pub mod control;
pub mod governance;
pub mod operations;
pub mod parity;
pub mod proof;
pub mod reasoning;
pub mod supervisor;
pub mod telemetry;
pub mod topology;
pub mod weather;

pub use channel::{channel, BoundedReceiver, BoundedSender, ChannelFullPolicy, SendError};
pub use component::{
    Component, ComponentContext, ComponentError, ComponentFactory, ComponentId, ComponentSpec,
    FailurePolicy, PortSpec, RunningState,
};
pub use config::*;
pub use continuity::*;
pub use contract::*;
pub use control::*;
pub use governance::*;
pub use operations::*;
pub use parity::*;
pub use reasoning::*;
pub use supervisor::{Kernel, KernelControl, KernelError, KernelExit, KernelHandle};
pub use telemetry::*;
pub use topology::{
    ComponentRegistry, ConfiguredTopology, FactoryRegistration, FactoryRegistry, TopologyError,
    ValidatedTopology,
};
pub use weather::*;
