use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

use crate::error::{ErrorCode, Result, V2Error};
use crate::git::clean_commit_revision;
use crate::github::PrStatePacket;
use crate::model::{IssueRecord, LifecyclePhase};

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
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MergeMethod {
    Merge,
    Squash,
    Rebase,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct MergeRequest {
    pub schema: String,
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub claim_id: String,
    pub actor: String,
    pub repository: String,
    pub pull_request: u64,
    pub base: String,
    pub head: String,
    pub expected_head_sha: String,
    pub merge_method: MergeMethod,
    #[serde(default)]
    pub required_checks: Vec<String>,
    #[serde(default)]
    pub require_review: bool,
    pub token_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct MergeResult {
    pub schema: String,
    pub issue: u64,
    pub pull_request: u64,
    pub repository: String,
    pub base: String,
    pub head: String,
    pub head_sha: String,
    pub merge_method: MergeMethod,
    pub merge_sha: String,
    pub already_merged: bool,
}

pub fn build_result(
    request: &MergeRequest,
    merge_sha: String,
    already_merged: bool,
) -> MergeResult {
    MergeResult {
        schema: "csdlc.merge_result.v1".into(),
        issue: request.issue,
        pull_request: request.pull_request,
        repository: request.repository.clone(),
        base: request.base.clone(),
        head: request.head.clone(),
        head_sha: request.expected_head_sha.clone(),
        merge_method: request.merge_method,
        merge_sha,
        already_merged,
    }
}

pub fn validate_request(request: &MergeRequest) -> Result<()> {
    if request.schema != "csdlc.merge_request.v1"
        || request.issue == 0
        || request.pull_request == 0
        || request.repository.split('/').count() != 2
        || request.base.trim().is_empty()
        || request.head.trim().is_empty()
        || request.expected_head_sha.trim().is_empty()
        || request.claim_id.trim().is_empty()
        || request.actor.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "merge request identity is incomplete",
        ));
    }
    Ok(())
}

pub fn validate_canonical(record: &IssueRecord, request: &MergeRequest, now: u64) -> Result<()> {
    validate_request(request)?;
    if record.issue != request.issue
        || record.repository != request.repository
        || record.generation != request.expected_generation
        || record.digest != request.expected_digest
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "merge request does not match canonical issue identity or digest",
        ));
    }
    if record.phase != LifecyclePhase::MergeReady {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "merge requires canonical merge_ready phase",
        ));
    }
    let claim = record
        .claim
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "merge claim is missing"))?;
    claim.validate(&request.claim_id, now)?;
    if claim.owner != request.actor {
        return Err(V2Error::new(
            ErrorCode::InvalidClaim,
            "merge actor does not own the active claim",
        ));
    }
    let publication = record.publication.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "publication evidence is missing",
        )
    })?;
    if publication.repository != request.repository
        || publication.issue != request.issue
        || publication.pull_request != request.pull_request
        || publication.base != request.base
        || publication.head != request.head
        || publication.draft
        || publication.observed_state != "open"
        || publication.revision != clean_commit_revision(&request.expected_head_sha)
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "canonical publication does not match the exact merge request",
        ));
    }
    let readiness = record.readiness.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "readiness evidence is missing",
        )
    })?;
    if !readiness.ready
        || readiness.pull_request != request.pull_request
        || readiness.head_sha != request.expected_head_sha
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "current readiness evidence does not match the exact merge request",
        ));
    }
    Ok(())
}

pub fn validate_remote(packet: &PrStatePacket, request: &MergeRequest) -> Result<()> {
    if packet.repository != request.repository
        || packet.pull_request != request.pull_request
        || packet.draft
        || packet.merge_state != "clean"
        || packet.base_ref.as_deref() != Some(request.base.as_str())
        || packet.head_sha != request.expected_head_sha
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "remote PR is not the exact clean merge target",
        ));
    }
    if packet.classification != "ready" {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!("remote merge gate is {}", packet.classification),
        ));
    }
    for required in &request.required_checks {
        let check = packet
            .checks
            .iter()
            .find(|check| &check.name == required)
            .ok_or_else(|| {
                V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    format!("required check {required} is missing"),
                )
            })?;
        if check.conclusion != "success" {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!("required check {required} is {}", check.conclusion),
            ));
        }
    }
    if request.require_review && packet.review_decision != "approved" {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "required review approval is missing",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::github::PrCheck;

    fn request() -> MergeRequest {
        MergeRequest {
            schema: "csdlc.merge_request.v1".into(),
            issue: 7,
            expected_generation: 4,
            expected_digest: "digest".into(),
            claim_id: "claim".into(),
            actor: "agent".into(),
            repository: "owner/repo".into(),
            pull_request: 12,
            base: "main".into(),
            head: "codex/7".into(),
            expected_head_sha: "abc123".into(),
            merge_method: MergeMethod::Squash,
            required_checks: vec!["ci".into()],
            require_review: true,
            token_file: None,
        }
    }

    fn packet() -> PrStatePacket {
        PrStatePacket {
            schema: "csdlc.github_pr_state.v1".into(),
            repository: "owner/repo".into(),
            pull_request: 12,
            linked_issue: Some(7),
            draft: false,
            merge_state: "clean".into(),
            review_decision: "approved".into(),
            base_ref: Some("main".into()),
            head_sha: "abc123".into(),
            checks: vec![PrCheck {
                name: "ci".into(),
                required: true,
                conclusion: "success".into(),
                details_url: None,
            }],
            required_check_names: vec!["ci".into()],
            classification: "ready".into(),
        }
    }

    #[test]
    fn accepts_exact_green_remote_gate() {
        validate_remote(&packet(), &request()).expect("gate should pass");
    }

    #[test]
    fn accepts_has_hooks_after_github_normalization_to_clean() {
        let mut value = packet();
        value.merge_state = crate::github::normalize_mergeable_state(Some(
            octocrab::models::pulls::MergeableState::HasHooks,
        ))
        .into();
        assert_eq!(value.merge_state, "clean");
        value.classification = crate::github::classify_pr_state(&value, true).into();
        validate_remote(&value, &request()).expect("normalized has-hooks state should merge");
    }

    #[test]
    fn rejects_head_drift() {
        let mut value = packet();
        value.head_sha = "different".into();
        assert_eq!(
            validate_remote(&value, &request()).unwrap_err().code,
            ErrorCode::ReconciliationRequired
        );
    }

    #[test]
    fn rejects_pending_required_check() {
        let mut value = packet();
        value.checks[0].conclusion = "pending".into();
        value.classification = "waiting".into();
        assert_eq!(
            validate_remote(&value, &request()).unwrap_err().code,
            ErrorCode::ReconciliationRequired
        );
    }

    #[test]
    fn rejects_incomplete_request_identity() {
        let mut value = request();
        value.repository.clear();
        assert_eq!(
            validate_request(&value).unwrap_err().code,
            ErrorCode::InvalidInput
        );
    }

    #[test]
    fn builds_distinguishable_merge_and_already_merged_results() {
        let merged = build_result(&request(), "merge-sha".into(), false);
        assert_eq!(merged.schema, "csdlc.merge_result.v1");
        assert_eq!(merged.head_sha, "abc123");
        assert_eq!(merged.merge_sha, "merge-sha");
        assert!(!merged.already_merged);

        let replay = build_result(&request(), "merge-sha".into(), true);
        assert_eq!(
            replay,
            MergeResult {
                already_merged: true,
                ..merged
            }
        );
    }

    #[test]
    fn serializes_each_supported_merge_method() {
        for (method, expected) in [
            (MergeMethod::Merge, "merge"),
            (MergeMethod::Squash, "squash"),
            (MergeMethod::Rebase, "rebase"),
        ] {
            assert_eq!(serde_json::to_value(method).expect("method JSON"), expected);
        }
    }
}
