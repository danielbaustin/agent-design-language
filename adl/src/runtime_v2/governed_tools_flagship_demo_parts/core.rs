//! Runtime-v2 governed-tools flagship demo proof bundle.
//!
//! This module composes existing fixture-backed governed-tools surfaces into one
//! reviewer-facing D11 packet without claiming arbitrary production execution.

use super::*;

mod cases;
mod constants;
mod helpers;
mod models;
mod reports;
mod trace_support;

pub(crate) use cases::*;
pub use constants::*;
pub use models::*;
pub(crate) use reports::*;
pub(crate) use trace_support::*;
