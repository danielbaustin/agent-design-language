//! Runtime-v2 cultivating-intelligence contract.
//!
//! WP-14 turns formation evidence into a bounded, reviewable runtime surface.
//! The packet must stay trace-linked, operational, and explicit about the
//! adjacent v0.91.1 capability/intelligence/memory/ToM boundary.

use super::*;
#[path = "cultivating_intelligence_parts/models.rs"]
mod cultivating_intelligence_models;

#[path = "cultivating_intelligence_parts/builder.rs"]
mod cultivating_intelligence_builder;

#[path = "cultivating_intelligence_parts/validation.rs"]
mod cultivating_intelligence_validation;

pub use cultivating_intelligence_models::*;
pub use cultivating_intelligence_builder::*;
pub use cultivating_intelligence_validation::*;
pub(crate) use cultivating_intelligence_validation::{
    cultivation_assessment,
    cultivation_criterion,
    cultivation_dimension,
    ordered_outcome_refs,
    ordered_trace_refs,
};
