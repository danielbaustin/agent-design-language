use octocrab::models::pulls::{MergeableState, ReviewState};
use octocrab::params::repos::Commitish;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Debug, Clone, Deserialize)]
pub struct PrStateRequest {
    pub repository: String,
    pub pull_request: u64,
    pub required_checks: Vec<String>,
    pub require_review: bool,
    pub token_file: Option<String>,
    pub linked_issue: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrCheck {
    pub name: String,
    pub required: bool,
    pub conclusion: String,
    pub details_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrStatePacket {
    pub schema: String,
    pub repository: String,
    pub pull_request: u64,
    pub linked_issue: Option<u64>,
    pub draft: bool,
    pub merge_state: String,
    pub review_decision: String,
    pub base_ref: Option<String>,
    pub head_sha: String,
    pub checks: Vec<PrCheck>,
    pub required_check_names: Vec<String>,
    pub classification: String,
}

pub fn classify_pr_state(packet: &PrStatePacket, require_review: bool) -> &'static str {
    if packet.draft {
        return "waiting";
    }
    if packet
        .checks
        .iter()
        .any(|c| c.required && matches!(c.conclusion.as_str(), "failure" | "cancelled"))
    {
        return "failed";
    }
    if packet.merge_state == "behind" || packet.merge_state == "dirty" {
        return "stale_base";
    }
    if packet.merge_state == "unknown" {
        return "waiting";
    }
    if packet
        .required_check_names
        .iter()
        .any(|name| !packet.checks.iter().any(|c| &c.name == name))
    {
        return "waiting";
    }
    if packet.checks.iter().any(|c| c.conclusion == "unknown") {
        return "waiting";
    }
    if packet
        .checks
        .iter()
        .any(|c| c.required && c.conclusion == "pending")
    {
        return "waiting";
    }
    if require_review && packet.review_decision != "approved" {
        return "operator_review";
    }
    "ready"
}

pub async fn collect_pr_state(request: &PrStateRequest) -> crate::Result<PrStatePacket> {
    let (owner, repo) = request.repository.split_once('/').ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "repository must be owner/name",
        )
    })?;
    let token = resolve_token(request.token_file.as_deref())?;
    let crab = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .map_err(remote)?;
    let pr = crab
        .pulls(owner, repo)
        .get(request.pull_request)
        .await
        .map_err(remote)?;
    let head = pr.head.as_ref().ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "PR head is absent",
        )
    })?;
    let page = crab
        .checks(owner, repo)
        .list_check_runs_for_git_ref(Commitish(head.sha.clone()))
        .per_page(100)
        .send()
        .await
        .map_err(remote)?;
    let mut latest = BTreeMap::new();
    for run in page.check_runs {
        latest.insert(run.name.clone(), run);
    }
    let checks = latest
        .into_values()
        .map(|run| PrCheck {
            required: request.required_checks.contains(&run.name),
            name: run.name,
            conclusion: conclusion(run.conclusion.as_deref()).into(),
            details_url: run.details_url,
        })
        .collect::<Vec<_>>();
    let reviews = crab
        .all_pages(
            crab.pulls(owner, repo)
                .list_reviews(request.pull_request)
                .per_page(100)
                .send()
                .await
                .map_err(remote)?,
        )
        .await
        .map_err(remote)?;
    let review_decision = if reviews
        .iter()
        .any(|r| r.state == Some(ReviewState::ChangesRequested))
    {
        "changes_requested"
    } else if reviews
        .iter()
        .any(|r| r.state == Some(ReviewState::Approved))
    {
        "approved"
    } else {
        "pending"
    };
    let merge_state = match pr.mergeable_state {
        Some(MergeableState::Dirty) => "dirty",
        Some(MergeableState::Unknown) | None => "unknown",
        Some(MergeableState::Clean | MergeableState::HasHooks) => "clean",
        _ => "behind",
    };
    let mut packet = PrStatePacket {
        schema: "csdlc.github_pr_state.v1".into(),
        repository: request.repository.clone(),
        pull_request: request.pull_request,
        linked_issue: request.linked_issue,
        draft: pr.draft.unwrap_or(false),
        merge_state: merge_state.into(),
        review_decision: review_decision.into(),
        base_ref: pr.base.as_ref().map(|b| b.ref_field.clone()),
        head_sha: head.sha.clone(),
        checks,
        required_check_names: request.required_checks.clone(),
        classification: String::new(),
    };
    packet.classification = classify_pr_state(&packet, request.require_review).into();
    Ok(packet)
}

fn resolve_token(path: Option<&str>) -> crate::Result<String> {
    for key in ["ADL_GITHUB_TOKEN", "GITHUB_TOKEN", "GH_TOKEN"] {
        if let Ok(v) = std::env::var(key) {
            if !v.trim().is_empty() {
                return Ok(v);
            }
        }
    }
    let path = path
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("ADL_GITHUB_TOKEN_FILE").map(std::path::PathBuf::from))
        .or_else(|| {
            std::env::var_os("HOME")
                .map(|home| std::path::PathBuf::from(home).join("keys/github.token"))
        })
        .ok_or_else(|| {
            crate::V2Error::new(
                crate::ErrorCode::InvalidInput,
                "GitHub token source is unavailable",
            )
        })?;
    let value = fs::read_to_string(Path::new(&path)).map_err(|_| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "GitHub token source is unavailable",
        )
    })?;
    if value.trim().is_empty() {
        return Err(crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "GitHub token source is empty",
        ));
    }
    Ok(value.trim().into())
}
fn conclusion(value: Option<&str>) -> &'static str {
    match value {
        Some("success") => "success",
        Some("failure" | "timed_out" | "action_required" | "startup_failure") => "failure",
        Some("cancelled") => "cancelled",
        Some("skipped") => "skipped",
        Some("neutral") => "neutral",
        None => "pending",
        _ => "unknown",
    }
}
fn remote(error: octocrab::Error) -> crate::V2Error {
    crate::V2Error::new(
        crate::ErrorCode::RemoteFailure,
        format!("GitHub observation failed: {error}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn packet(state: &str) -> PrStatePacket {
        PrStatePacket {
            schema: "x".into(),
            repository: "o/r".into(),
            pull_request: 1,
            linked_issue: Some(2),
            draft: false,
            merge_state: "clean".into(),
            review_decision: "approved".into(),
            base_ref: Some("main".into()),
            head_sha: "abc".into(),
            checks: vec![PrCheck {
                name: "ci".into(),
                required: true,
                conclusion: state.into(),
                details_url: None,
            }],
            required_check_names: vec!["ci".into()],
            classification: String::new(),
        }
    }
    #[test]
    fn classifies_common_tail_states() {
        assert_eq!(classify_pr_state(&packet("success"), true), "ready");
        assert_eq!(classify_pr_state(&packet("pending"), false), "waiting");
        assert_eq!(classify_pr_state(&packet("failure"), false), "failed");
    }
}
