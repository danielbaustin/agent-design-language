//! Runtime-v2 wellbeing metrics contract.
//!
//! WP-09 consumes the prior moral-governance review surfaces and turns them
//! into a bounded wellbeing diagnostic. The result must stay decomposed,
//! evidence-backed, privacy-governed, and explicitly non-scalar.

use super::*;
#[path = "wellbeing_metrics_parts/models.rs"]
mod wellbeing_models;

#[path = "wellbeing_metrics_parts/builder.rs"]
mod wellbeing_builder;

#[path = "wellbeing_metrics_parts/validation.rs"]
mod wellbeing_validation;

pub use wellbeing_models::*;
pub use wellbeing_builder::*;
pub use wellbeing_validation::*;
pub(crate) use wellbeing_validation::{
    build_wellbeing_fixture,
    dimension_signal,
    ordered_outcome_refs,
    ordered_trace_refs,
};
