use adl_workcell_conductor::TaskAssignment;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const CONVERGENCE_CONTRACT_VERSION: &str = "adl.workcell-convergence.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConvergenceInput {
    pub contract: String,
    pub source_revision: String,
    pub correlation_seed: String,
    pub authority: ConvergenceAuthority,
    pub assignments: Vec<TaskAssignment>,
    pub outputs: Vec<TaskOutput>,
    pub active_claims: Vec<ActiveClaim>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConvergenceAuthority {
    pub subject: String,
    pub may_decide: bool,
    pub may_create_task: bool,
    pub may_mutate_github: bool,
    pub may_write_filesystem: bool,
    pub may_mutate_lifecycle: bool,
    pub declared_integration_authority: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActiveClaim {
    pub issue: u64,
    pub claim_id: String,
    pub protected_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskOutput {
    pub issue: u64,
    pub claim_id: String,
    pub branch: String,
    pub worktree: String,
    pub source_revision: String,
    pub assignment_digest: String,
    pub protected_paths: Vec<String>,
    pub write_paths: Vec<String>,
    pub artifacts: Vec<ArtifactRef>,
    pub validation_refs: Vec<String>,
    pub review_refs: Vec<String>,
    pub status: OutputStatus,
    pub changed_assumptions: Vec<ChangedAssumption>,
    pub blockers: Vec<Blocker>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactRef {
    pub path: String,
    pub digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputStatus {
    Succeeded,
    Partial,
    Blocked,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChangedAssumption {
    pub key: String,
    pub expected: String,
    pub observed: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Blocker {
    pub code: BlockerCode,
    pub issue: u64,
    pub message: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerCode {
    MissingOutput,
    StaleOutput,
    ForgedBinding,
    DuplicateOutput,
    PathOverlap,
    InvalidPath,
    OutOfScopeArtifact,
    MissingArtifact,
    AmbiguousReview,
    HiddenMutationAuthority,
    ResidualBlocker,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConvergenceEnvelope {
    pub contract: String,
    pub decision_id: String,
    pub decision: ConvergenceDecision,
    pub projection: ReadOnlyProjection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConvergenceDecision {
    Integrate(IntegrationPlan),
    Replan(ReplanRecord),
    Blocked(BlockedRecord),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrationPlan {
    pub source_revision: String,
    pub authority: String,
    pub steps: Vec<IntegrationStep>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrationStep {
    pub issue: u64,
    pub claim_id: String,
    pub branch: String,
    pub source_revision: String,
    pub artifacts: Vec<ArtifactRef>,
    pub validation_refs: Vec<String>,
    pub review_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplanRecord {
    pub source_revision: String,
    pub changed_assumptions: Vec<ChangedAssumption>,
    pub admissible_remaining_work: Vec<u64>,
    pub integrated_issues: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BlockedRecord {
    pub source_revision: String,
    pub blockers: Vec<Blocker>,
    pub integrated_issues: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadOnlyProjection {
    pub source_revision: String,
    pub integrated: Vec<IntegrationStep>,
    pub partial_successes: Vec<IntegrationStep>,
    pub residual_blockers: Vec<Blocker>,
    pub remaining_issues: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConvergenceErrorCode {
    InvalidContract,
    InvalidInput,
    InvalidPath,
    Serialization,
}

#[derive(Debug, Clone, PartialEq, Eq, Error, Serialize, Deserialize)]
#[error("convergence error: {code:?}")]
#[serde(deny_unknown_fields)]
pub struct ConvergenceError {
    pub code: ConvergenceErrorCode,
    pub message: String,
}

impl ConvergenceError {
    pub(crate) fn new(code: ConvergenceErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}
