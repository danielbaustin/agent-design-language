//! Runtime-v2 flagship observatory contract and report artifacts.
//!
//! Captures flagship observatory surfaces used as a reduced proof set for
//! review and cross-boundary integration checkpoints.

use super::*;

mod builders;
mod report;
mod types;
mod validation;

pub use types::*;

use report::{render_observatory_flagship_operator_report, validate_flagship_operator_report};
use validation::{
    validate_actor_roster, validate_feature_demo_coverage, validate_flagship_walkthrough,
    validate_relative_path_list, validate_required_flagship_refs,
};
