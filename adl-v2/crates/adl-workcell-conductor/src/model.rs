use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

pub const CONDUCTOR_CONTRACT_VERSION: &str = "adl.workcell-conductor.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardKind {
    Sip,
    Stp,
    Spp,
    Vpp,
    Srp,
    Sor,
}

impl CardKind {
    pub(crate) fn required() -> BTreeSet<Self> {
        [
            Self::Sip,
            Self::Stp,
            Self::Spp,
            Self::Vpp,
            Self::Srp,
            Self::Sor,
        ]
        .into_iter()
        .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimSnapshot {
    pub id: String,
    pub owner: String,
    pub branch: String,
    pub worktree: String,
    pub purpose: String,
    pub expires_unix_seconds: u64,
    pub protected_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationLane {
    pub name: String,
    pub argv: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IssueSnapshot {
    pub issue: u64,
    pub source_revision: String,
    pub ready: bool,
    pub cards: BTreeSet<CardKind>,
    pub claim: Option<ClaimSnapshot>,
    pub dependencies: Vec<u64>,
    pub write_paths: Vec<String>,
    pub validation_lanes: Vec<ValidationLane>,
    pub expected_outputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionPlanSnapshot {
    pub contract: String,
    pub source_digest: String,
    pub node_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConductorInput {
    pub contract: String,
    pub source_revision: String,
    pub observed_unix_seconds: u64,
    pub correlation_seed: String,
    pub max_writable_assignments: usize,
    pub active_writable_assignments: usize,
    pub known_validation_lanes: BTreeSet<String>,
    pub resolved_dependencies: BTreeSet<u64>,
    pub execution_plan: ExecutionPlanSnapshot,
    pub issues: Vec<IssueSnapshot>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Lane {
    Serial,
    Parallel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SerializedGate {
    Review,
    Publication,
    Merge,
    PostMergeValidation,
    Closeout,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskAssignment {
    pub issue: u64,
    pub claim_id: String,
    pub branch: String,
    pub worktree: String,
    pub source_revision: String,
    pub execution_plan_digest: String,
    pub dependencies: Vec<u64>,
    pub protected_paths: Vec<String>,
    pub write_paths: Vec<String>,
    pub validation_lanes: Vec<ValidationLane>,
    pub expected_outputs: Vec<String>,
    pub lane: Lane,
    pub wave: usize,
    pub correlation_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConductorPlan {
    pub contract: String,
    pub source_revision: String,
    pub execution_plan_digest: String,
    pub assignments: Vec<TaskAssignment>,
    pub serialized_gates: Vec<SerializedGate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConductorDecision {
    pub plan: ConductorPlan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefusalCode {
    InvalidContract,
    InvalidInput,
    MissingCards,
    MissingClaim,
    StaleClaim,
    NotReady,
    DuplicateIssue,
    UnresolvedDependency,
    DependencyCycle,
    UnknownValidationLane,
    InvalidPath,
    PathCollision,
    WipOverflow,
    AmbiguousAuthority,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{code:?} at {path}: {message}")]
#[serde(deny_unknown_fields)]
pub struct ConductorRefusal {
    pub code: RefusalCode,
    pub path: String,
    pub message: String,
    pub evidence_refs: Vec<String>,
}

impl ConductorRefusal {
    pub(crate) fn new(
        code: RefusalCode,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            path: path.into(),
            message: message.into(),
            evidence_refs: Vec::new(),
        }
    }

    pub(crate) fn for_issue(
        code: RefusalCode,
        issue: u64,
        path: impl Into<String>,
        message: impl Into<String>,
        revision: &str,
    ) -> Self {
        Self {
            code,
            path: path.into(),
            message: message.into(),
            evidence_refs: vec![format!("issue:{issue}@{revision}")],
        }
    }
}
