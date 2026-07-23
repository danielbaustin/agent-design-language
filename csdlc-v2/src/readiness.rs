use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::{Component, Path, PathBuf};
use strum::{AsRefStr, Display, EnumString};

use crate::error::{ErrorCode, Result, V2Error};
use crate::model::{IssueRecord, LifecyclePhase, ReadinessEvidence, TerminalEvidence};
use crate::Store;

macro_rules! closed_enum {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Display, EnumString, AsRefStr)]
        #[serde(rename_all = "snake_case")]
        #[strum(serialize_all = "snake_case")]
        pub enum $name { $($variant),+ }
    };
}

closed_enum!(CheckRequirement { Required, Optional });
closed_enum!(CheckConclusion {
    Pending,
    Success,
    Failure,
    Cancelled,
    Skipped,
    Neutral,
    Unknown
});
closed_enum!(RemoteReviewState {
    Pending,
    Approved,
    ChangesRequested,
    NotRequired,
    Unknown
});
closed_enum!(ConflictState {
    Clean,
    Conflicted,
    Pending,
    Unknown
});
closed_enum!(TerminalDisposition {
    Merged,
    ClosedUnmerged,
    ClosedNoPr
});

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CheckObservation {
    pub name: String,
    pub requirement: CheckRequirement,
    pub conclusion: CheckConclusion,
    pub details_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PostPublicationFinding {
    pub id: String,
    pub reviewer: String,
    pub summary: String,
    pub changes_requested: bool,
    pub active: bool,
    pub route: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReadinessRequest {
    pub schema: String,
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub claim_id: String,
    pub actor: String,
    pub pull_request: u64,
    pub head_sha: String,
    pub required_checks: Vec<String>,
    pub require_review: bool,
    pub checks: Vec<CheckObservation>,
    pub review_state: RemoteReviewState,
    pub conflict_state: ConflictState,
    pub post_publication_findings: Vec<PostPublicationFinding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReadinessReport {
    pub schema: String,
    pub ready: bool,
    pub blockers: Vec<String>,
    pub required_success: Vec<String>,
    pub required_pending: Vec<String>,
    pub required_failed: Vec<String>,
    pub optional_non_success: Vec<String>,
}

pub fn classify_readiness(request: &ReadinessRequest) -> Result<ReadinessReport> {
    if request.schema != "csdlc.readiness_request.v1" || request.head_sha.is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "readiness request identity is invalid",
        ));
    }
    let mut required_success = Vec::new();
    let mut required_pending = Vec::new();
    let mut required_failed = Vec::new();
    let mut optional_non_success = Vec::new();
    for required in &request.required_checks {
        match request.checks.iter().find(|check| check.name == *required) {
            Some(check) if check.requirement != CheckRequirement::Required => {
                required_failed.push(format!("{required}:misclassified"))
            }
            Some(check) => match check.conclusion {
                CheckConclusion::Success => required_success.push(required.clone()),
                CheckConclusion::Pending | CheckConclusion::Unknown => {
                    required_pending.push(required.clone())
                }
                _ => required_failed.push(required.clone()),
            },
            None => required_pending.push(format!("{required}:unobserved")),
        }
    }
    for check in request.checks.iter().filter(|check| {
        check.requirement == CheckRequirement::Optional
            && check.conclusion != CheckConclusion::Success
    }) {
        optional_non_success.push(check.name.clone());
    }
    let mut blockers = Vec::new();
    if !required_pending.is_empty() {
        blockers.push("required_checks_pending".into());
    }
    if !required_failed.is_empty() {
        blockers.push("required_checks_failed".into());
    }
    if request.require_review && request.review_state != RemoteReviewState::Approved {
        blockers.push("required_review_not_approved".into());
    }
    if request.conflict_state != ConflictState::Clean {
        blockers.push("conflict_state_not_clean".into());
    }
    if request
        .post_publication_findings
        .iter()
        .any(|finding| finding.changes_requested && finding.active)
    {
        blockers.push("post_publication_changes_requested".into());
    }
    Ok(ReadinessReport {
        schema: "csdlc.readiness_report.v1".into(),
        ready: blockers.is_empty(),
        blockers,
        required_success,
        required_pending,
        required_failed,
        optional_non_success,
    })
}

pub fn record_readiness(store: &Store, request: ReadinessRequest) -> Result<IssueRecord> {
    let report = classify_readiness(&request)?;
    let evidence = ReadinessEvidence {
        pull_request: request.pull_request,
        head_sha: request.head_sha.clone(),
        checks: request.checks.clone(),
        review_state: request.review_state,
        conflict_state: request.conflict_state,
        post_publication_findings: request.post_publication_findings.clone(),
        ready: report.ready,
        blockers: report.blockers.clone(),
    };
    store.commit_readiness(request, evidence)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TerminalObservation {
    pub schema: String,
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub claim_id: String,
    pub actor: String,
    pub pull_request: Option<u64>,
    pub disposition: TerminalDisposition,
    pub observed_sha: Option<String>,
    pub observed_state: String,
    pub approved_no_pr_reason: Option<String>,
    pub receipt_path: String,
}

pub fn closeout_issue(store: &Store, observation: TerminalObservation) -> Result<IssueRecord> {
    if observation.schema != "csdlc.terminal_observation.v1"
        || observation.receipt_path.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "terminal observation is incomplete",
        ));
    }
    match observation.disposition {
        TerminalDisposition::Merged
            if observation.pull_request.is_none()
                || observation.observed_sha.is_none()
                || observation.observed_state != "merged" =>
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "merged closeout requires observed PR, SHA, and merged state",
            ))
        }
        TerminalDisposition::ClosedUnmerged
            if observation.pull_request.is_none()
                || observation.observed_sha.is_none()
                || observation.observed_state != "closed" =>
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "closed-unmerged closeout requires an observed closed PR",
            ))
        }
        TerminalDisposition::ClosedNoPr
            if observation.pull_request.is_some()
                || observation
                    .approved_no_pr_reason
                    .as_deref()
                    .unwrap_or_default()
                    .trim()
                    .is_empty() =>
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "no-PR closeout requires explicit approval reason and no PR",
            ))
        }
        _ => {}
    }
    let evidence = TerminalEvidence {
        pull_request: observation.pull_request,
        disposition: observation.disposition,
        observed_sha: observation.observed_sha.clone(),
        observed_state: observation.observed_state.clone(),
        receipt_path: observation.receipt_path.clone(),
        released_branch: String::new(),
        released_worktree: String::new(),
        released_protected_paths: Vec::new(),
    };
    store.commit_terminal(observation, evidence)
}

pub fn validate_prune_surface(
    root: &std::path::Path,
    expected_branch: &str,
    expected_worktree: &str,
) -> Result<()> {
    let branch = crate::git::current_branch(root)?;
    let canonical = root.canonicalize().map_err(|_| unsafe_checkout())?;
    let expected = resolve_terminal_worktree(root, expected_worktree)?;
    let topology = crate::git::worktrees(root)?;
    let observed = topology.iter().any(|(candidate_branch, candidate_path)| {
        candidate_branch == expected_branch
            && Path::new(candidate_path).canonicalize().ok().as_ref() == Some(&canonical)
    });
    if branch != expected_branch || canonical != expected || !observed {
        return Err(unsafe_checkout());
    }
    if !crate::git::run(root, &["status", "--porcelain", "--untracked-files=all"])?
        .stdout
        .is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "dirty worktree cannot be pruned",
        ));
    }
    Ok(())
}

fn resolve_terminal_worktree(root: &Path, expected_worktree: &str) -> Result<PathBuf> {
    if expected_worktree == "." {
        return root.canonicalize().map_err(|_| unsafe_checkout());
    }
    let expected = Path::new(expected_worktree);
    if expected_worktree.is_empty()
        || expected
            .components()
            .any(|component| matches!(component, Component::CurDir | Component::ParentDir))
    {
        return Err(unsafe_checkout());
    }
    let candidate = if expected.is_absolute() {
        expected.to_path_buf()
    } else {
        if !expected
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
        {
            return Err(unsafe_checkout());
        }
        let common = crate::git::run(
            root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?;
        PathBuf::from(common.stdout)
            .parent()
            .ok_or_else(unsafe_checkout)?
            .join(expected)
    };
    candidate.canonicalize().map_err(|_| unsafe_checkout())
}

fn unsafe_checkout() -> V2Error {
    V2Error::new(
        ErrorCode::UnsafeCheckout,
        "prune target does not match terminal claim topology",
    )
}

pub(crate) fn terminal_phase_allowed(
    phase: LifecyclePhase,
    disposition: TerminalDisposition,
) -> bool {
    match disposition {
        TerminalDisposition::Merged => phase == LifecyclePhase::MergeReady,
        TerminalDisposition::ClosedUnmerged => matches!(
            phase,
            LifecyclePhase::Published | LifecyclePhase::MergeReady
        ),
        TerminalDisposition::ClosedNoPr => phase == LifecyclePhase::Reviewed,
    }
}
