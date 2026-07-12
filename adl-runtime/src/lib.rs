//! CSM runtime crate boundary.
//!
//! This crate is intentionally limited to runtime-owned contracts that can be
//! built without ADL compiler or C-SDLC control-plane crates.

pub mod backpressure;
pub mod continuity_history;
pub mod curiosity;
pub mod determinism;
pub mod networking;
pub mod observability;
pub mod resident_agent;
pub mod runtime_api;
pub mod runtime_api_auth;
pub mod supervision;
pub mod topology;

pub const CSM_RUNTIME_OWNER: &str = "csm";
pub const ADL_TOOLING_ROLE: &str = "tooling_control_plane";
