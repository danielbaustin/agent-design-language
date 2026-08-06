use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString};

use crate::cards::{CardKind, CardValues, FindingDisposition, FindingSeverity};
use crate::error::{ErrorCode, Result, V2Error};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum LifecyclePhase {
    Initialized,
    Ready,
    Bound,
    Implemented,
    Reviewed,
    Published,
    MergeReady,
    Merged,
    ClosedOut,
}

impl LifecyclePhase {
    pub fn allows(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Initialized, Self::Ready)
                | (Self::Ready, Self::Bound)
                | (Self::Bound, Self::Implemented)
                | (Self::Implemented, Self::Reviewed)
                | (Self::Reviewed, Self::Implemented)
                | (Self::Reviewed, Self::Published)
                | (Self::Published, Self::Implemented)
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewAssignment {
    pub reviewer: String,
    pub assigned_by: String,
    pub revision: String,
    pub scope: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewFindingEvidence {
    pub id: String,
    pub severity: FindingSeverity,
    pub summary: String,
    pub actionable: bool,
    pub in_scope: bool,
    pub disposition: FindingDisposition,
    pub fix_revision: Option<String>,
    pub route: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct NonSubstantiveProof {
    pub policy: String,
    pub from_revision: String,
    pub to_revision: String,
    pub from_commit: String,
    pub to_commit: String,
    pub changed_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewEvidence {
    pub reviewer: String,
    pub scope: Vec<String>,
    pub reviewed_revision: String,
    pub findings: Vec<ReviewFindingEvidence>,
    pub residual_risks: Vec<String>,
    pub completed: bool,
    pub non_substantive_proof: Option<NonSubstantiveProof>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PublicationEvidence {
    pub repository: String,
    pub issue: u64,
    pub pull_request: u64,
    pub url: String,
    pub base: String,
    pub head: String,
    pub revision: String,
    pub draft: bool,
    pub observed_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReadinessEvidence {
    pub pull_request: u64,
    pub head_sha: String,
    pub checks: Vec<crate::readiness::CheckObservation>,
    pub review_state: crate::readiness::RemoteReviewState,
    pub conflict_state: crate::readiness::ConflictState,
    pub post_publication_findings: Vec<crate::readiness::PostPublicationFinding>,
    pub ready: bool,
    pub blockers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TerminalEvidence {
    pub pull_request: Option<u64>,
    pub disposition: crate::readiness::TerminalDisposition,
    pub observed_sha: Option<String>,
    pub observed_state: String,
    pub receipt_path: String,
    pub branch: Option<String>,
    pub worktree: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TerminalReceipt {
    pub schema: String,
    pub issue: u64,
    pub repository: String,
    pub initialization_digest: String,
    pub receipt_ref: String,
    pub authored_artifacts: BTreeMap<String, String>,
    pub record: IssueRecord,
    pub cards: BTreeMap<CardKind, CardValues>,
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct MigrationEvidence {
    pub schema: String,
    pub imported_unix_seconds: u64,
    pub sunset_unix_seconds: u64,
    pub source_digest: String,
    pub authored_sources: BTreeMap<String, String>,
    pub authored_sections: BTreeMap<String, BTreeMap<String, String>>,
    pub compatibility_view: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesignReview {
    Pending,
    Approved { reviewer: String, revision: String },
    ChangesRequired { reviewer: String, reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CardProjection {
    pub values_digest: String,
    pub rendered_digest: String,
    pub ast_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TransitionEvent {
    pub sequence: u64,
    pub from: LifecyclePhase,
    pub to: LifecyclePhase,
    pub actor: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct AuditEvent {
    pub sequence: u64,
    pub generation: u64,
    pub actor: String,
    pub reason: String,
    pub operation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct IssueRecord {
    pub schema: String,
    pub issue: u64,
    pub repository: String,
    pub initialization_digest: String,
    pub phase: LifecyclePhase,
    pub generation: u64,
    pub digest: String,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub worktree: Option<String>,
    pub review_assignment: Option<ReviewAssignment>,
    pub review: Option<ReviewEvidence>,
    #[serde(default)]
    pub publication: Option<PublicationEvidence>,
    #[serde(default)]
    pub readiness: Option<ReadinessEvidence>,
    #[serde(default)]
    pub terminal: Option<TerminalEvidence>,
    #[serde(default)]
    pub migration: Option<MigrationEvidence>,
    pub design_path: String,
    pub diagram_path: String,
    pub design_review: DesignReview,
    pub cards: BTreeMap<CardKind, CardProjection>,
    pub transitions: Vec<TransitionEvent>,
    pub audit: Vec<AuditEvent>,
}

impl IssueRecord {
    pub fn advance(&mut self, next: LifecyclePhase, actor: String, reason: String) -> Result<()> {
        if !self.phase.allows(next) {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                format!("transition {} -> {} is not allowed", self.phase, next),
            ));
        }
        let from = self.phase;
        self.phase = next;
        self.transitions.push(TransitionEvent {
            sequence: self.transitions.len() as u64 + 1,
            from,
            to: next,
            actor,
            reason,
        });
        Ok(())
    }
}
