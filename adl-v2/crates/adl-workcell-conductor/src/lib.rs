//! Pure deterministic planning from typed lifecycle snapshots to work assignments.

mod model;
mod planner;

pub use model::{
    CardKind, ClaimSnapshot, ConductorDecision, ConductorInput, ConductorPlan, ConductorRefusal,
    ExecutionPlanSnapshot, IssueSnapshot, Lane, RefusalCode, SerializedGate, TaskAssignment,
    ValidationLane, CONDUCTOR_CONTRACT_VERSION,
};
pub use planner::plan;
