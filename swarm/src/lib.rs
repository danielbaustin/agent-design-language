//! ADL `swarm` runtime library.
//!
//! This crate provides the language model (`adl`), resolution/planning (`resolve`,
//! `execution_plan`), deterministic execution (`execute`), and trust/verification
//! boundaries (`signing`, `remote_exec`) used by the `swarm` CLI.
//!
//! v0.5 invariants:
//! - deterministic execution order for ready steps
//! - bounded concurrency for concurrent execution plans
//! - optional signature verification with strict enforcement on `--run`
//! - remote execution MVP where scheduling remains local

pub mod adl;
pub mod bounded_executor;
pub mod demo;
pub mod execute;
pub mod execution_plan;
pub mod instrumentation;
pub mod plan;
pub mod prompt;
pub mod provider;
pub mod remote_exec;
pub mod resolve;
pub mod schema;
pub mod signing;
pub mod trace;
