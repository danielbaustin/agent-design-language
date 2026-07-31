//! Independent, additive runtime-kernel proof for ADL issue #5170.

pub mod acip;
pub mod assembly;
pub mod channel;
pub mod cognition;
pub mod component;
pub mod config;
pub mod continuity;
pub mod contract;
pub mod control;
pub mod durable_state;
pub mod governance;
pub mod identity_memory;
pub mod ingress;
pub mod live_continuity;
pub mod operations;
pub mod parity;
pub mod parity_b;
pub mod private_state;
pub mod proof;
pub mod protocol_adapters;
pub mod reasoning;
pub mod supervisor;
pub mod telemetry;
pub mod time;
pub mod topology;
pub mod weather;

pub use acip::*;
pub use assembly::*;
pub use channel::{channel, BoundedReceiver, BoundedSender, ChannelFullPolicy, SendError};
pub use cognition::*;
pub use component::{
    Component, ComponentContext, ComponentError, ComponentFactory, ComponentId, ComponentSpec,
    FailurePolicy, PortSpec, RunningState,
};
pub use config::*;
pub use continuity::*;
pub use contract::*;
pub use control::*;
pub use durable_state::*;
pub use governance::*;
pub use identity_memory::*;
pub use ingress::*;
pub use live_continuity::*;
pub use operations::*;
pub use parity::*;
pub use parity_b::*;
pub use private_state::*;
pub use protocol_adapters::{
    build_production_operation_executors as build_protocol_production_operation_executors,
    protocol_operation_executors, protocol_operation_executors_from_env, ProtocolAdapter,
    ProtocolBuildError, ProtocolEndpoint, ProtocolFrame, ProtocolResponse, ProtocolSecret,
    ProtocolSecurity, ProtocolStatus, MAX_PROTOCOL_FRAME_FRESHNESS_MILLIS,
    MAX_PROTOCOL_RESPONSE_BYTES, PROTOCOL_FRAME_SCHEMA, PROTOCOL_RESPONSE_SCHEMA,
};
pub use reasoning::*;
pub use supervisor::{Kernel, KernelControl, KernelError, KernelExit, KernelHandle};
pub use telemetry::*;
pub use time::*;
pub use topology::{
    ComponentRegistry, ConfiguredTopology, FactoryRegistration, FactoryRegistry, TopologyError,
    ValidatedTopology,
};
pub use weather::*;
