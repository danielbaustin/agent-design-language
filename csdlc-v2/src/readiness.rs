use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

use crate::error::{ErrorCode, Result, V2Error};

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
