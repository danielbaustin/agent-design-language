use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::cards::{FindingDisposition, ReviewResult};
use crate::model::{LifecyclePhase, ReviewAssignment, ReviewEvidence};
use crate::store::{ReviewCommit, Store};
use crate::{ErrorCode, IssueRecord, Result, V2Error};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReviewAssignmentRequest {
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub reviewer: String,
    pub assigned_by: String,
    pub scope: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReviewRecordRequest {
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub actor: String,
    pub evidence: ReviewEvidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReviewRecoveryRequest {
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub actor: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PublicationReviewReport {
    pub schema: String,
    pub ready: bool,
    pub blocker_codes: Vec<String>,
    pub reviewed_revision: Option<String>,
    pub accepted_revision: String,
}

pub fn assign_review(store: &Store, request: ReviewAssignmentRequest) -> Result<IssueRecord> {
    let record = store.load_record(request.issue)?;
    require_cas(
        &record,
        request.expected_generation,
        &request.expected_digest,
    )?;
    if record.phase != LifecyclePhase::Implemented {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "review assignment requires implemented phase",
        ));
    }
    if request.reviewer.trim().is_empty()
        || request.assigned_by.trim().is_empty()
        || request.scope.is_empty()
        || request.scope.iter().any(|s| s.trim().is_empty())
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "review assignment is incomplete",
        ));
    }
    let revision = crate::git::substantive_revision(store.root(), &request.scope)?;
    let head = crate::git::run(store.root(), &["rev-parse", "HEAD"])?.stdout;
    if revision != crate::git::clean_commit_revision(&head) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "review assignment requires a clean substantive commit",
        ));
    }
    let assignment = ReviewAssignment {
        reviewer: request.reviewer,
        assigned_by: request.assigned_by.clone(),
        revision,
        scope: request.scope,
    };
    store.commit_review_assignment(request.issue, &request.expected_digest, assignment)
}

pub fn recover_review(store: &Store, request: ReviewRecoveryRequest) -> Result<IssueRecord> {
    if request.reason.trim().is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "review recovery reason is required",
        ));
    }
    store.commit_review_recovery(
        request.issue,
        request.expected_generation,
        &request.expected_digest,
        request.actor,
        request.reason,
    )
}

pub fn record_review(store: &Store, request: ReviewRecordRequest) -> Result<IssueRecord> {
    let record = store.load_record(request.issue)?;
    require_cas(
        &record,
        request.expected_generation,
        &request.expected_digest,
    )?;
    if record.phase != LifecyclePhase::Implemented {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "review recording requires implemented phase",
        ));
    }
    if let Some(assignment) = record.review_assignment.as_ref() {
        validate_evidence(assignment, &request.evidence)?;
    } else {
        validate_direct_evidence(store, &request.evidence)?;
    }
    let result = if request.evidence.completed
        && request.evidence.findings.iter().all(|f| {
            !f.actionable
                || !f.in_scope
                || matches!(
                    f.disposition,
                    FindingDisposition::Fixed | FindingDisposition::AcceptedRisk
                )
        }) {
        ReviewResult::Pass
    } else {
        ReviewResult::ChangesRequired
    };
    store.commit_review(ReviewCommit {
        issue: request.issue,
        expected_digest: record.digest,
        actor: request.actor,
        evidence: request.evidence,
        result,
        advance_reviewed: true,
    })
}

fn validate_direct_evidence(store: &Store, evidence: &ReviewEvidence) -> Result<()> {
    if evidence.reviewer.trim().is_empty()
        || evidence.scope.is_empty()
        || evidence.scope.iter().any(|path| path.trim().is_empty())
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "direct review evidence requires reviewer and non-empty scope",
        ));
    }
    let revision = crate::git::substantive_revision(store.root(), &evidence.scope)?;
    let head = crate::git::run(store.root(), &["rev-parse", "HEAD"])?.stdout;
    if revision != crate::git::clean_commit_revision(&head)
        || evidence.reviewed_revision != revision
    {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "direct review evidence must match the clean exact scoped commit",
        ));
    }
    validate_findings(evidence)
}

fn require_cas(record: &IssueRecord, generation: u64, digest: &str) -> Result<()> {
    if record.generation != generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "review generation is stale",
        ));
    }
    if record.digest != digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "review digest is stale",
        ));
    }
    Ok(())
}

fn validate_evidence(assignment: &ReviewAssignment, evidence: &ReviewEvidence) -> Result<()> {
    if evidence.reviewer != assignment.reviewer
        || evidence.scope != assignment.scope
        || evidence.reviewed_revision != assignment.revision
        || evidence.reviewer.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "review evidence does not match assignment",
        ));
    }
    validate_findings(evidence)
}

fn validate_findings(evidence: &ReviewEvidence) -> Result<()> {
    let mut ids = std::collections::BTreeSet::new();
    for f in &evidence.findings {
        if f.id.trim().is_empty() || f.summary.trim().is_empty() || !ids.insert(&f.id) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "review finding identity is invalid",
            ));
        }
        if f.actionable
            && f.in_scope
            && f.disposition == FindingDisposition::Fixed
            && f.fix_revision.as_deref() != Some(evidence.reviewed_revision.as_str())
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "fixed finding must name the reviewed revision",
            ));
        }
        if f.actionable
            && f.in_scope
            && f.disposition == FindingDisposition::AcceptedRisk
            && evidence.residual_risks.is_empty()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "accepted actionable risk requires residual-risk evidence",
            ));
        }
        if !f.in_scope
            && (f.disposition != FindingDisposition::OutOfScope
                || f.route.as_deref().unwrap_or_default().is_empty())
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "out-of-scope finding requires route",
            ));
        }
    }
    Ok(())
}

pub fn evaluate_publication_review(
    evidence: Option<&ReviewEvidence>,
    current_revision: &str,
) -> PublicationReviewReport {
    evaluate(evidence, current_revision, false)
}

pub fn evaluate_publication_review_in_repo(
    root: &std::path::Path,
    evidence: Option<&ReviewEvidence>,
    current_revision: &str,
) -> PublicationReviewReport {
    let proof_ok = match evidence {
        Some(e) if e.non_substantive_proof.is_some() => e
            .non_substantive_proof
            .as_ref()
            .is_some_and(|proof| verify_non_substantive(root, e, current_revision, proof)),
        Some(e) => verify_automatic_metadata_only(root, e, current_revision),
        None => false,
    };
    evaluate(evidence, current_revision, proof_ok)
}

fn evaluate(
    evidence: Option<&ReviewEvidence>,
    current_revision: &str,
    proof_ok: bool,
) -> PublicationReviewReport {
    let mut blockers = Vec::new();
    let reviewed = evidence.map(|e| e.reviewed_revision.clone());
    match evidence {
        None => blockers.push("review_missing".into()),
        Some(e) => {
            if !e.completed {
                blockers.push("review_incomplete".into());
            }
            if e.reviewer.trim().is_empty() || e.scope.is_empty() {
                blockers.push("review_identity_or_scope_missing".into());
            }
            if e.findings.iter().any(|f| {
                (f.disposition == FindingDisposition::Fixed
                    && f.fix_revision.as_deref() != Some(e.reviewed_revision.as_str()))
                    || (f.disposition == FindingDisposition::AcceptedRisk
                        && e.residual_risks.is_empty())
            }) {
                blockers.push("review_evidence_invalid".into());
            }
            if e.reviewed_revision != current_revision && !proof_ok {
                blockers.push("review_stale".into());
            }
            if e.findings.iter().any(|f| {
                f.actionable
                    && f.in_scope
                    && !matches!(
                        f.disposition,
                        FindingDisposition::Fixed | FindingDisposition::AcceptedRisk
                    )
            }) {
                blockers.push("actionable_finding_unresolved".into());
            }
            if e.findings.iter().any(|f| {
                !f.in_scope
                    && (f.disposition != FindingDisposition::OutOfScope
                        || f.route.as_deref().unwrap_or_default().is_empty())
            }) {
                blockers.push("out_of_scope_finding_unrouted".into());
            }
        }
    }
    PublicationReviewReport {
        schema: "csdlc.publication_review.v1".into(),
        ready: blockers.is_empty(),
        blocker_codes: blockers,
        reviewed_revision: reviewed,
        accepted_revision: current_revision.into(),
    }
}

fn verify_non_substantive(
    root: &std::path::Path,
    e: &ReviewEvidence,
    current: &str,
    p: &crate::NonSubstantiveProof,
) -> bool {
    if p.policy != "review_metadata_only_v1"
        || p.from_revision != e.reviewed_revision
        || p.to_revision != current
        || p.from_revision != crate::git::clean_commit_revision(&p.from_commit)
        || p.to_revision != crate::git::clean_commit_revision(&p.to_commit)
    {
        return false;
    }
    crate::git::metadata_only_changed_paths(root, &p.from_commit, &p.to_commit)
        .is_ok_and(|paths| !paths.is_empty() && paths == p.changed_paths)
}

fn verify_automatic_metadata_only(
    root: &std::path::Path,
    evidence: &ReviewEvidence,
    current: &str,
) -> bool {
    let Some(from_commit) = evidence
        .reviewed_revision
        .strip_prefix("git-blake3:")
        .and_then(|value| value.split(':').next())
    else {
        return false;
    };
    let Some(to_commit) = current
        .strip_prefix("git-blake3:")
        .and_then(|value| value.split(':').next())
    else {
        return false;
    };
    if from_commit == to_commit
        || crate::git::clean_commit_revision(from_commit) != evidence.reviewed_revision
        || crate::git::clean_commit_revision(to_commit) != current
    {
        return false;
    }
    crate::git::metadata_only_changed_paths(root, from_commit, to_commit)
        .is_ok_and(|paths| !paths.is_empty())
}
