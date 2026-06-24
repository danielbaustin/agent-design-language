use super::*;
use crate::cli::observability;
#[cfg(test)]
use crate::cli::pr_cmd::github_client::GithubClientMode;
use crate::cli::pr_cmd::github_client::{
    body_contains_closing_linkage, issue_labels_from_csv, issue_labels_from_csv_in_order,
    issue_metadata_drift, linked_issue_numbers_from_lines, linked_issue_numbers_include,
    plan_issue_metadata_parity, pr_matches_main_version_wave, redact_for_diagnostics,
    AdlGithubClient, GithubClientBackend, IssueMetadataSnapshot, PullRequestMetadataSnapshot,
};
use crate::cli::pr_cmd_args::IssueCloseReason;
use crate::cli::pr_cmd_args::IssueStateFilter;
use crate::cli::pr_cmd_prompt::infer_workflow_queue;
use crate::cli::tokio_runtime::with_current_thread_runtime;
use ::adl::control_plane::resolve_primary_checkout_root;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

#[cfg(test)]
mod test_support;
mod transport;

#[cfg(test)]
use self::test_support::{
    run_gh_capture_shell, run_gh_capture_shell_allow_failure, run_gh_status_shell,
    run_gh_status_shell_allow_failure, test_gh_fixture_fallback_allowed,
};
#[cfg(test)]
use self::transport::{
    classify_pr_validation_snapshot, emit_pr_validation_wait_snapshot,
    emit_pr_validation_wait_timeout, pr_validation_status_octocrab, PrValidationCheckSnapshot,
    PrValidationDisposition, PrValidationSnapshot,
};
use self::transport::{
    create_pr_octocrab, current_pr_url_octocrab, issue_close_octocrab, issue_comment_octocrab,
    list_prs_by_head_ref_octocrab, list_prs_octocrab, mark_pr_ready_octocrab, merge_pr_octocrab,
    pr_base_ref_octocrab, pr_body_octocrab, pr_closing_issue_numbers_octocrab,
    update_pr_body_octocrab, update_pr_title_body_octocrab,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct RepoParts {
    owner: String,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(super) struct OpenPullRequest {
    pub(super) number: u32,
    pub(super) title: String,
    pub(super) url: String,
    #[serde(rename = "headRefName")]
    pub(super) head_ref_name: String,
    #[serde(rename = "baseRefName")]
    pub(super) base_ref_name: String,
    #[serde(rename = "isDraft")]
    pub(super) is_draft: bool,
    #[serde(default = "default_open_pr_state")]
    pub(super) state: String,
    #[serde(skip)]
    pub(super) queue: Option<String>,
}

fn default_open_pr_state() -> String {
    "OPEN".to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct IssueRecord {
    pub(crate) number: u32,
    pub(crate) title: String,
    pub(crate) state: String,
    pub(crate) url: String,
    #[serde(rename = "closedAt")]
    pub(crate) closed_at: Option<String>,
    pub(crate) body: Option<String>,
    pub(crate) labels: Vec<String>,
    pub(crate) milestone: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct RestIssueLabel {
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct RepoLabelRecord {
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct RestIssueMilestone {
    title: String,
}

#[derive(Debug, Clone, Deserialize)]
struct RestIssueRecord {
    number: u64,
    title: Option<String>,
    state: Option<String>,
    html_url: Option<String>,
    closed_at: Option<String>,
    body: Option<String>,
    labels: Option<Vec<RestIssueLabel>>,
    milestone: Option<RestIssueMilestone>,
    pull_request: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
struct RestIssueSearchResult {
    items: Vec<RestIssueRecord>,
}

impl RestIssueRecord {
    fn into_issue_record(self) -> Option<IssueRecord> {
        if self.pull_request.is_some() {
            return None;
        }
        Some(IssueRecord {
            number: self.number as u32,
            title: self.title?,
            state: self.state?,
            url: self.html_url?,
            closed_at: self.closed_at,
            body: self.body,
            labels: self
                .labels
                .unwrap_or_default()
                .into_iter()
                .map(|label| label.name)
                .collect(),
            milestone: self.milestone.map(|milestone| milestone.title),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct PrValidationCheckReport {
    pub(super) name: String,
    pub(super) status: String,
    pub(super) conclusion: String,
    pub(super) job_run_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct PrValidationReport {
    pub(super) pr_number: u64,
    pub(super) commit_sha: String,
    pub(super) pr_state: String,
    pub(super) is_draft: bool,
    pub(super) disposition: String,
    pub(super) projection_status: String,
    pub(super) checks: Vec<PrValidationCheckReport>,
    pub(super) failed_checks: Vec<PrValidationCheckReport>,
    pub(super) pending_checks: Vec<PrValidationCheckReport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct IssueWatchLinkedPrReport {
    pub(crate) number: u32,
    pub(crate) url: String,
    pub(crate) head_ref_name: String,
    pub(crate) base_ref_name: String,
    pub(crate) is_draft: bool,
    pub(crate) state: String,
    pub(crate) validation: PrValidationReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct IssueWatchLocalReadinessReport {
    pub(crate) status: String,
    pub(crate) pr_run_readiness: String,
    pub(crate) reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct IssueWatchReport {
    pub(crate) schema: &'static str,
    pub(crate) issue: u32,
    pub(crate) issue_state: String,
    pub(crate) authoritative_classifier: &'static str,
    pub(crate) advisory_agent_mode: &'static str,
    pub(crate) classification: String,
    pub(crate) next_skill: String,
    pub(crate) continuation: String,
    pub(crate) reason: String,
    pub(crate) local_readiness: IssueWatchLocalReadinessReport,
    pub(crate) linked_pr: Option<IssueWatchLinkedPrReport>,
}

pub(super) fn wait_for_pr_validation_finish(repo: &str, pr_ref: &str) -> Result<()> {
    transport::wait_for_pr_validation_finish(repo, pr_ref)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ClosingLinkageLiveStatus {
    Unavailable(&'static str),
    Failed(&'static str),
    Closing,
    NonClosing,
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct ClosingLinkageEventPayload {
    repository: Option<ClosingLinkageEventRepository>,
    pull_request: Option<ClosingLinkageEventPullRequest>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct ClosingLinkageEventRepository {
    full_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct ClosingLinkageEventPullRequest {
    body: Option<String>,
    number: Option<u64>,
}

fn body_declares_non_closing_lifecycle_pr(body: &str) -> bool {
    body.to_ascii_lowercase()
        .contains("non-closing lifecycle pr")
}

fn issue_number_from_codex_branch(head_ref: &str) -> Option<u32> {
    for (idx, _) in head_ref.match_indices("codex/") {
        if idx > 0 && !head_ref[..idx].ends_with('/') {
            continue;
        }
        let rest = &head_ref[idx + "codex/".len()..];
        let digit_len = rest.chars().take_while(|ch| ch.is_ascii_digit()).count();
        if digit_len == 0 {
            continue;
        }
        if rest.as_bytes().get(digit_len) != Some(&b'-') {
            continue;
        }
        if let Ok(issue) = rest[..digit_len].parse::<u32>() {
            return Some(issue);
        }
    }
    None
}

fn live_closing_linkage_status(repo: &str, pr_number: u64, issue: u32) -> ClosingLinkageLiveStatus {
    let Ok(client) = AdlGithubClient::from_env() else {
        return ClosingLinkageLiveStatus::Unavailable(
            "live PR metadata was unavailable because repo, PR number, or token context was missing",
        );
    };
    if client.backend() != GithubClientBackend::Octocrab {
        return ClosingLinkageLiveStatus::Unavailable(
            "live PR metadata was unavailable because repo, PR number, or token context was missing",
        );
    }
    let pr_ref = pr_number.to_string();
    let linked = run_gh_capture_allow_failure(
        "pr.view.closing_issues",
        &[
            "pr",
            "view",
            "-R",
            repo,
            &pr_ref,
            "--json",
            "closingIssuesReferences",
            "--jq",
            ".closingIssuesReferences[]?.number",
        ],
    );
    let Ok(linked) = linked else {
        return ClosingLinkageLiveStatus::Failed("live PR metadata fetch failed");
    };
    let linked_issue_numbers =
        linked_issue_numbers_from_lines(linked.as_deref().unwrap_or_default());
    if linked_issue_numbers_include(&linked_issue_numbers, issue) {
        return ClosingLinkageLiveStatus::Closing;
    }
    let body = run_gh_capture_allow_failure(
        "pr.view.body",
        &[
            "pr", "view", "-R", repo, &pr_ref, "--json", "body", "--jq", ".body",
        ],
    );
    let Ok(body) = body else {
        return ClosingLinkageLiveStatus::Failed("live PR metadata fetch failed");
    };
    let body = body.unwrap_or_default();
    if body_declares_non_closing_lifecycle_pr(&body) {
        return ClosingLinkageLiveStatus::NonClosing;
    }
    if body_contains_closing_linkage(&body, issue) {
        return ClosingLinkageLiveStatus::Closing;
    }
    ClosingLinkageLiveStatus::Missing
}

pub(super) fn check_pr_closing_linkage_guard(
    event_name_arg: Option<&str>,
    event_path_arg: Option<&Path>,
    head_ref_arg: Option<&str>,
    repo_arg: Option<&str>,
) -> Result<()> {
    let event_name = event_name_arg
        .map(str::to_string)
        .or_else(|| std::env::var("GITHUB_EVENT_NAME").ok())
        .unwrap_or_default();
    let head_ref = head_ref_arg
        .map(str::to_string)
        .or_else(|| std::env::var("GITHUB_HEAD_REF").ok())
        .or_else(|| std::env::var("GITHUB_REF_NAME").ok())
        .unwrap_or_default();

    if event_name != "pull_request" {
        println!("check_pr_closing_linkage: skipped for event '{event_name}'");
        return Ok(());
    }

    let event_path = event_path_arg
        .map(Path::to_path_buf)
        .or_else(|| std::env::var("GITHUB_EVENT_PATH").ok().map(Into::into))
        .ok_or_else(|| {
            anyhow!("check_pr_closing_linkage: missing GitHub pull_request event payload")
        })?;
    if !event_path.is_file() {
        bail!("check_pr_closing_linkage: missing GitHub pull_request event payload");
    }

    let issue = match issue_number_from_codex_branch(&head_ref) {
        Some(issue) => issue,
        None => {
            println!("check_pr_closing_linkage: skipped for non-issue branch '{head_ref}'");
            return Ok(());
        }
    };

    let payload_text = fs::read_to_string(&event_path).with_context(|| {
        format!(
            "check_pr_closing_linkage: failed to read pull_request event payload '{}'",
            event_path.display()
        )
    })?;
    let payload: ClosingLinkageEventPayload = serde_json::from_str(&payload_text)
        .context("check_pr_closing_linkage: invalid GitHub pull_request event payload JSON")?;
    let repo = repo_arg
        .map(str::to_string)
        .or_else(|| std::env::var("GITHUB_REPOSITORY").ok())
        .or_else(|| payload.repository.and_then(|repo| repo.full_name))
        .unwrap_or_default();
    let pr_number = payload.pull_request.as_ref().and_then(|pr| pr.number);
    let event_body = payload
        .pull_request
        .as_ref()
        .and_then(|pr| pr.body.clone())
        .unwrap_or_default();
    let live_status = match (repo.trim().is_empty(), pr_number) {
        (false, Some(pr_number)) => live_closing_linkage_status(&repo, pr_number, issue),
        _ => ClosingLinkageLiveStatus::Unavailable(
            "live PR metadata was unavailable because repo, PR number, or token context was missing",
        ),
    };
    let pr_number_display = pr_number
        .map(|number| number.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    match live_status {
        ClosingLinkageLiveStatus::Closing => {
            println!(
                "check_pr_closing_linkage: PR #{pr_number_display} closes issue #{issue} (live PR metadata)"
            );
            Ok(())
        }
        ClosingLinkageLiveStatus::NonClosing => {
            println!(
                "check_pr_closing_linkage: PR #{pr_number_display} declares non-closing lifecycle work for issue #{issue} (live PR metadata)"
            );
            Ok(())
        }
        ClosingLinkageLiveStatus::Missing => {
            bail!(
                "check_pr_closing_linkage: live PR body for PR #{pr_number_display} is missing closing linkage to issue #{issue}; update the PR body with a closing keyword such as 'Closes #{issue}' or declare a non-closing lifecycle PR"
            );
        }
        ClosingLinkageLiveStatus::Unavailable(note) | ClosingLinkageLiveStatus::Failed(note) => {
            if body_declares_non_closing_lifecycle_pr(&event_body) {
                println!(
                    "check_pr_closing_linkage: PR #{pr_number_display} declares non-closing lifecycle work for issue #{issue} (event payload)"
                );
                return Ok(());
            }
            if body_contains_closing_linkage(&event_body, issue) {
                println!(
                    "check_pr_closing_linkage: PR #{pr_number_display} closes issue #{issue} (event payload)"
                );
                return Ok(());
            }
            bail!(
                "check_pr_closing_linkage: PR #{pr_number_display} is missing closing linkage to issue #{issue}; include a closing keyword such as 'Closes #{issue}' or declare a non-closing lifecycle PR. {note}. If this is a rerun after a PR-body-only repair, GitHub may be reusing a stale pull_request event payload; rerun with token/repo context for live metadata validation or push a fresh commit to refresh the event payload."
            );
        }
    }
}

pub(super) fn wait_for_pr_validation_report(
    repo: &str,
    pr_ref: &str,
) -> Result<PrValidationReport> {
    transport::wait_for_pr_validation_report(repo, pr_ref)
}

pub(super) fn pr_validation_report(repo: &str, pr_ref: &str) -> Result<PrValidationReport> {
    transport::pr_validation_report(repo, pr_ref)
}

pub(super) fn pr_validation_projection_status(
    pr_state: &str,
    is_draft: bool,
    disposition: &str,
) -> &'static str {
    if pr_state.eq_ignore_ascii_case("MERGED") {
        return "merged";
    }
    match disposition {
        "pending" => "checks_pending",
        "failed" | "cancelled" | "timed_out" => "checks_failed",
        "success" | "skipped" if is_draft => "checks_green_but_draft",
        "success" | "skipped" => "ready_to_merge_or_review",
        _ if is_draft => "checks_pending",
        _ => "unknown",
    }
}

pub(crate) fn pr_metadata_for_watch(repo: &str, pr_ref: &str) -> Result<OpenPullRequest> {
    transport::pr_metadata_octocrab(repo, pr_ref)
}

pub(crate) fn issue_number_for_pr_watch(repo: &str, pr: &OpenPullRequest) -> Result<u32> {
    let linked = pr_closing_issue_numbers_octocrab(repo, &pr.number.to_string())?;
    issue_number_from_pr_metadata_for_watch(pr, &linked)
}

pub(crate) fn issue_number_from_pr_metadata_for_watch(
    pr: &OpenPullRequest,
    linked: &[u32],
) -> Result<u32> {
    if linked.len() == 1 {
        return Ok(linked[0]);
    }
    if linked.len() > 1 {
        bail!(
            "watch: PR #{} closes multiple issues {}; pass the issue number explicitly",
            pr.number,
            linked
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    issue_number_from_codex_branch(&pr.head_ref_name).ok_or_else(|| {
        anyhow!(
            "watch: PR #{} has no closing issue metadata and head branch '{}' does not start with a codex issue number",
            pr.number,
            pr.head_ref_name
        )
    })
}

pub(crate) fn linked_prs_for_issue(
    repo: &str,
    issue: u32,
    branch_hint: Option<&str>,
) -> Result<Vec<OpenPullRequest>> {
    let mut linked = Vec::new();
    let prs = if let Some(branch_hint) = branch_hint {
        let prs = list_prs_by_head_ref_octocrab(repo, branch_hint)?;
        if prs.is_empty() {
            list_prs_octocrab(repo)?
        } else {
            prs
        }
    } else {
        list_prs_octocrab(repo)?
    };
    for pr in prs {
        let pr_ref = pr.number.to_string();
        let linked_issues =
            pr_closing_issue_numbers_octocrab(repo, &pr_ref).with_context(|| {
                format!(
                    "watch: failed to inspect closing issues for PR #{}",
                    pr.number
                )
            })?;
        if linked_issues.contains(&issue)
            || issue_number_from_codex_branch(&pr.head_ref_name) == Some(issue)
        {
            linked.push(pr);
        }
    }
    Ok(linked)
}

pub(crate) fn build_issue_watch_report(
    issue: &IssueRecord,
    closed_completed: bool,
    local_readiness: IssueWatchLocalReadinessReport,
    linked_pr: Option<(OpenPullRequest, PrValidationReport)>,
) -> IssueWatchReport {
    if closed_completed {
        return IssueWatchReport {
            schema: "adl.pr.watch.v1",
            issue: issue.number,
            issue_state: issue.state.to_uppercase(),
            authoritative_classifier: "adl",
            advisory_agent_mode: "local_agent_advisory_only",
            classification: "closeout_needed".to_string(),
            next_skill: "pr-closeout".to_string(),
            continuation: "action_required".to_string(),
            reason: "issue_closed_completed".to_string(),
            local_readiness,
            linked_pr: linked_pr.map(|(pr, validation)| IssueWatchLinkedPrReport {
                number: pr.number,
                url: pr.url,
                head_ref_name: pr.head_ref_name,
                base_ref_name: pr.base_ref_name,
                is_draft: pr.is_draft,
                state: pr.state,
                validation,
            }),
        };
    }

    if !issue.state.eq_ignore_ascii_case("open") {
        return IssueWatchReport {
            schema: "adl.pr.watch.v1",
            issue: issue.number,
            issue_state: issue.state.to_uppercase(),
            authoritative_classifier: "adl",
            advisory_agent_mode: "local_agent_advisory_only",
            classification: "closed".to_string(),
            next_skill: "human_review".to_string(),
            continuation: "ask_operator".to_string(),
            reason: "issue_closed_without_completed_reason".to_string(),
            local_readiness,
            linked_pr: linked_pr.map(|(pr, validation)| IssueWatchLinkedPrReport {
                number: pr.number,
                url: pr.url,
                head_ref_name: pr.head_ref_name,
                base_ref_name: pr.base_ref_name,
                is_draft: pr.is_draft,
                state: pr.state,
                validation,
            }),
        };
    }

    let Some((pr, validation)) = linked_pr else {
        let (classification, next_skill, continuation, reason) =
            if local_readiness.pr_run_readiness == "ready" {
                (
                    "ready_for_run",
                    "pr-run",
                    "continue",
                    "issue_ready_without_linked_pr",
                )
            } else if local_readiness.status == "failed" {
                (
                    "blocked",
                    "pr-ready",
                    "action_required",
                    "issue_local_readiness_failed",
                )
            } else {
                (
                    "unknown",
                    "issue-watcher",
                    "ask_operator",
                    "issue_local_readiness_unknown",
                )
            };
        return IssueWatchReport {
            schema: "adl.pr.watch.v1",
            issue: issue.number,
            issue_state: issue.state.to_uppercase(),
            authoritative_classifier: "adl",
            advisory_agent_mode: "local_agent_advisory_only",
            classification: classification.to_string(),
            next_skill: next_skill.to_string(),
            continuation: continuation.to_string(),
            reason: reason.to_string(),
            local_readiness,
            linked_pr: None,
        };
    };

    let (classification, next_skill, continuation, reason) = if validation.pr_state == "MERGED" {
        (
            "merged_pending_closeout",
            "pr-closeout",
            "action_required",
            "linked_pr_merged_closeout_pending",
        )
    } else if validation.projection_status == "checks_green_but_draft" {
        (
            "checks_green_but_draft",
            "pr-janitor",
            "action_required",
            "linked_pr_checks_green_but_draft",
        )
    } else if validation.is_draft {
        ("pr_open", "issue-watcher", "continue", "linked_pr_draft")
    } else {
        match validation.disposition.as_str() {
            "pending" => (
                "checks_running",
                "issue-watcher",
                "continue",
                "linked_pr_checks_pending",
            ),
            "failed" | "cancelled" | "timed_out" => (
                "checks_failed",
                "pr-janitor",
                "action_required",
                "linked_pr_checks_failed",
            ),
            "success" | "skipped" => (
                "checks_green",
                "human_review",
                "ask_operator",
                "linked_pr_checks_green",
            ),
            _ => (
                "blocked",
                "issue-watcher",
                "ask_operator",
                "linked_pr_validation_ambiguous",
            ),
        }
    };

    IssueWatchReport {
        schema: "adl.pr.watch.v1",
        issue: issue.number,
        issue_state: issue.state.to_uppercase(),
        authoritative_classifier: "adl",
        advisory_agent_mode: "local_agent_advisory_only",
        classification: classification.to_string(),
        next_skill: next_skill.to_string(),
        continuation: continuation.to_string(),
        reason: reason.to_string(),
        local_readiness,
        linked_pr: Some(IssueWatchLinkedPrReport {
            number: pr.number,
            url: pr.url,
            head_ref_name: pr.head_ref_name,
            base_ref_name: pr.base_ref_name,
            is_draft: pr.is_draft,
            state: pr.state,
            validation,
        }),
    }
}

fn github_client(operation: &str) -> Result<AdlGithubClient> {
    let client = AdlGithubClient::from_env()
        .with_context(|| format!("github client policy rejected operation '{operation}'"))?;
    let config = client.config();
    match config.backend {
        GithubClientBackend::Octocrab => Ok(client),
        GithubClientBackend::GhFallback => bail!(
            "github_client.gh_fallback_removed: operation '{}' requires octocrab transport; {}; credential values are never printed",
            operation,
            github_credential_preflight_hint(config)
        ),
    }
}

fn github_credential_preflight_hint(
    config: &crate::cli::pr_cmd::github_client::AdlGithubClientConfig,
) -> String {
    match config.token_source {
        None => "credential_status=missing_token; configure GITHUB_TOKEN, GH_TOKEN, ADL_GITHUB_TOKEN_FILE, or ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE before live C-SDLC GitHub operations, or provide the approved default token file at $HOME/keys/github.token; load the token from an operator-approved secret source and pass it only to the ADL command environment without echoing it; do not fall back to direct gh commands".to_string(),
        Some(source) => format!(
            "credential_status=token_present source={}; ADL_GITHUB_CLIENT=gh is not a supported read or mutation fallback for covered operations; use ADL_GITHUB_CLIENT=auto or ADL_GITHUB_CLIENT=octocrab",
            source.env_name()
        ),
    }
}

#[cfg(test)]
pub(super) fn run_gh_capture(operation: &str, args: &[&str]) -> Result<String> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed(operation)? {
        return run_gh_capture_shell(operation, args);
    }
    run_octocrab_capture(operation, args)
}

pub(crate) fn run_gh_capture_allow_failure(
    operation: &str,
    args: &[&str],
) -> Result<Option<String>> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed(operation)? {
        return run_gh_capture_shell_allow_failure(operation, args);
    }
    run_octocrab_capture(operation, args).map(Some)
}

pub(super) fn run_gh_status(operation: &str, args: &[&str]) -> Result<()> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed(operation)? {
        return run_gh_status_shell(operation, args);
    }
    run_octocrab_status(operation, args)
}

pub(super) fn pr_view_base_ref_finish_existing(repo: &str, pr_ref: &str) -> Result<String> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("pr.view.base_ref.finish_existing")? {
        return run_gh_capture_shell(
            "pr.view.base_ref.finish_existing",
            &[
                "pr",
                "view",
                "-R",
                repo,
                pr_ref,
                "--json",
                "baseRefName",
                "--jq",
                ".baseRefName",
            ],
        )
        .map(|value: String| value.trim().to_string());
    }
    pr_base_ref_octocrab(repo, pr_ref)
}

pub(super) fn pr_edit_finish_existing(
    repo: &str,
    pr_ref: &str,
    title: &str,
    body_file: &Path,
) -> Result<()> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("pr.edit.finish_existing")? {
        return run_gh_status_shell(
            "pr.edit.finish_existing",
            &[
                "pr",
                "edit",
                "-R",
                repo,
                pr_ref,
                "--title",
                title,
                "--body-file",
                path_str(body_file)?,
            ],
        );
    }
    let body = std::fs::read_to_string(body_file)
        .with_context(|| format!("failed to read PR body file '{}'", body_file.display()))?;
    update_pr_title_body_octocrab(repo, pr_ref, title, &body)
}

pub(super) fn pr_create_finish(
    repo: &str,
    title: &str,
    head: &str,
    base: &str,
    body_file: &Path,
    draft: bool,
) -> Result<String> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("pr.create.finish")? {
        return run_gh_capture_shell(
            "pr.create.finish",
            &[
                "pr",
                "create",
                "-R",
                repo,
                "--base",
                base,
                "--head",
                head,
                "--title",
                title,
                "--body-file",
                path_str(body_file)?,
                "--draft",
            ],
        )
        .map(|value: String| value.trim().to_string());
    }
    let body = std::fs::read_to_string(body_file)
        .with_context(|| format!("failed to read PR body file '{}'", body_file.display()))?;
    create_pr_octocrab(repo, title, head, base, &body, draft)
}

pub(super) fn pr_ready_finish(repo: &str, pr_ref: &str) -> Result<()> {
    transport::pr_ready_with_optional_fixture_fallback("pr.ready.finish", repo, pr_ref)
}

pub(super) fn pr_ready_finish_merge(repo: &str, pr_ref: &str) -> Result<()> {
    transport::pr_ready_with_optional_fixture_fallback("pr.ready.finish_merge", repo, pr_ref)
}

pub(super) fn pr_ready_finish_allow_failure(repo: &str, pr_ref: &str) -> Result<bool> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("pr.ready.finish")? {
        return run_gh_status_shell_allow_failure(
            "pr.ready.finish",
            &["pr", "ready", "-R", repo, pr_ref],
        );
    }
    pr_ready_finish(repo, pr_ref).map(|_| true)
}

pub(super) fn pr_ready_finish_merge_allow_failure(repo: &str, pr_ref: &str) -> Result<bool> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("pr.ready.finish_merge")? {
        return run_gh_status_shell_allow_failure(
            "pr.ready.finish_merge",
            &["pr", "ready", "-R", repo, pr_ref],
        );
    }
    pr_ready_finish_merge(repo, pr_ref).map(|_| true)
}

pub(super) fn pr_merge_finish(repo: &str, pr_ref: &str) -> Result<()> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("pr.merge.finish")? {
        return run_gh_status_shell(
            "pr.merge.finish",
            &["pr", "merge", "-R", repo, "--squash", pr_ref],
        );
    }
    merge_pr_octocrab(repo, pr_ref)
}

fn parse_repo(repo: &str) -> Result<RepoParts> {
    let (owner, name) = repo
        .split_once('/')
        .ok_or_else(|| anyhow!("github repo must be owner/name, got '{repo}'"))?;
    if owner.trim().is_empty() || name.trim().is_empty() {
        bail!("github repo must be owner/name, got '{repo}'");
    }
    Ok(RepoParts {
        owner: owner.to_string(),
        name: name.to_string(),
    })
}

fn parse_pr_number(pr_ref: &str) -> Result<u64> {
    let trimmed = pr_ref.trim();
    if let Ok(number) = trimmed.parse::<u64>() {
        return Ok(number);
    }
    let marker = "/pull/";
    if let Some((_, number)) = trimmed.rsplit_once(marker) {
        let number = number
            .split(['/', '?', '#'])
            .next()
            .unwrap_or_default()
            .parse::<u64>()
            .with_context(|| format!("failed to parse pull request number from '{pr_ref}'"))?;
        return Ok(number);
    }
    bail!("failed to parse pull request number from '{pr_ref}'")
}

fn block_on_octocrab<T, Fut>(
    runtime: &tokio::runtime::Runtime,
    operation: &str,
    mut make_future: impl FnMut() -> Fut,
) -> Result<T>
where
    Fut: std::future::Future<Output = ::octocrab::Result<T>>,
{
    observability::emit_event(
        "adl",
        "github_octocrab",
        "started",
        &[("operation", operation)],
    );
    let _runtime_guard = runtime.enter();
    let mut attempt = 1usize;
    let max_attempts = octocrab_max_attempts();
    let result = loop {
        let call_result = if let Some(request_timeout) = octocrab_request_timeout(operation) {
            match runtime.block_on(tokio::time::timeout(request_timeout, make_future())) {
                Ok(result) => result,
                Err(_elapsed) => {
                    let attempts_text = attempt.to_string();
                    let timeout_secs_text = request_timeout.as_secs().to_string();
                    observability::emit_event(
                        "adl",
                        "github_octocrab",
                        "failed",
                        &[
                            ("operation", operation),
                            ("attempts", attempts_text.as_str()),
                            ("reason", "timeout"),
                            ("timeout_secs", timeout_secs_text.as_str()),
                        ],
                    );
                    return Err(anyhow!(
                        "github_client.timeout: operation '{}' exceeded the {}s per-attempt timeout after {} attempt(s)",
                        operation,
                        request_timeout.as_secs(),
                        attempt
                    ));
                }
            }
        } else {
            runtime.block_on(make_future())
        };
        match call_result {
            Ok(value) => break value,
            Err(err)
                if attempt < max_attempts
                    && octocrab_operation_allows_retry(operation)
                    && octocrab_error_is_retryable(&err) =>
            {
                let attempt_text = attempt.to_string();
                let max_attempts_text = max_attempts.to_string();
                observability::emit_event(
                    "adl",
                    "github_octocrab",
                    "retry",
                    &[
                        ("operation", operation),
                        ("attempt", attempt_text.as_str()),
                        ("max_attempts", max_attempts_text.as_str()),
                    ],
                );
                std::thread::sleep(octocrab_retry_delay(attempt));
                attempt += 1;
            }
            Err(err) => {
                let attempts_text = attempt.to_string();
                observability::emit_event(
                    "adl",
                    "github_octocrab",
                    "failed",
                    &[
                        ("operation", operation),
                        ("attempts", attempts_text.as_str()),
                    ],
                );
                return Err(anyhow!(
                    "github_client.octocrab_transport: operation '{}' failed after {} attempt(s): {}",
                    operation,
                    attempt,
                    format_octocrab_failure(&err)
                ));
            }
        }
    };
    observability::emit_event(
        "adl",
        "github_octocrab",
        "completed",
        &[("operation", operation)],
    );
    Ok(result)
}

fn format_octocrab_failure(err: &octocrab::Error) -> String {
    match err {
        octocrab::Error::GitHub { source, .. } => match source.status_code.as_u16() {
            401 => "github_client.auth: GitHub authentication failed; token is missing, invalid, expired, or not accepted for this endpoint".to_string(),
            403 => "github_client.auth: GitHub authorization failed; token may lack required repo/workflow permission or the API refused the operation".to_string(),
            429 => "github_client.rate_limit: GitHub rate limit or secondary throttling refused the operation".to_string(),
            404 => {
                "github_client.not_found: GitHub resource was not found or token cannot see it"
                    .to_string()
            }
            _ => format!("github_client.transport: {err}"),
        },
        _ => err.to_string(),
    }
}

fn octocrab_max_attempts() -> usize {
    std::env::var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|attempts| (1..=10).contains(attempts))
        .unwrap_or(3)
}

fn octocrab_request_timeout(operation: &str) -> Option<Duration> {
    if operation != "pr.list.wave" {
        return None;
    }
    Some(Duration::from_secs(
        std::env::var("ADL_GITHUB_OCTOCRAB_TIMEOUT_SECS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .filter(|secs| (1..=120).contains(secs))
            .unwrap_or(10),
    ))
}

fn octocrab_retry_delay(attempt: usize) -> Duration {
    Duration::from_millis(match attempt {
        0 | 1 => 50,
        2 => 150,
        _ => 300,
    })
}

fn octocrab_error_is_retryable(err: &octocrab::Error) -> bool {
    match err {
        octocrab::Error::GitHub { source, .. } => matches!(
            source.status_code.as_u16(),
            408 | 409 | 425 | 429 | 500..=599
        ),
        octocrab::Error::Http { .. }
        | octocrab::Error::Hyper { .. }
        | octocrab::Error::Service { .. }
        | octocrab::Error::Other { .. } => true,
        _ => false,
    }
}

fn octocrab_operation_allows_retry(operation: &str) -> bool {
    matches!(
        operation,
        "pr.list.current_branch"
            | "pr.list.open_wave"
            | "pr.view.body"
            | "pr.view.closing_issues"
            | "pr.view.base_ref.finish_existing"
            | "pr.validation.status"
            | "pr.edit.body_file"
            | "pr.edit.finish_existing"
            | "pr.ready"
            | "issue.list"
            | "issue.search"
            | "issue.view.labels"
            | "issue.edit.title"
            | "issue.edit.body"
            | "issue.view.title"
            | "issue.view.body"
            | "issue.view.full"
            | "issue.view.state"
            | "issue.view.repo_labels"
            | "issue.edit.labels"
            | "issue.close"
    )
}

fn with_octocrab<T>(
    operation: &str,
    f: impl FnOnce(&tokio::runtime::Runtime, octocrab::Octocrab) -> Result<T>,
) -> Result<T> {
    with_current_thread_runtime(
        &format!("github_client.octocrab_runtime: failed to build runtime for {operation}"),
        |runtime| {
            let client = github_client(operation)?;
            let octo = client
                .octocrab()
                .map_err(|err| anyhow!("github_client.octocrab_build: {err}"))?;
            f(runtime, octo)
        },
    )
}

fn run_octocrab_capture(operation: &str, args: &[&str]) -> Result<String> {
    let _client = github_client(operation)?;
    match operation {
        "pr.list.current_branch" => {
            let repo = arg_after(args, "-R")?;
            let branch = arg_after(args, "--head")?;
            current_pr_url_octocrab(repo, branch).map(|url| url.unwrap_or_default())
        }
        "pr.list.open_wave" | "pr.list.wave" => {
            let repo = arg_after(args, "-R")?;
            let prs = list_prs_octocrab(repo)?;
            serde_json::to_string(&prs).context("failed to serialize octocrab PR list")
        }
        "pr.view.body" => {
            let repo = arg_after(args, "-R")?;
            let pr_ref = positional_after(args, "view")?;
            pr_body_octocrab(repo, pr_ref)
        }
        "pr.view.closing_issues" => {
            let repo = arg_after(args, "-R")?;
            let pr_ref = positional_after(args, "view")?;
            pr_closing_issue_numbers_octocrab(repo, pr_ref).map(|numbers| {
                numbers
                    .into_iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            })
        }
        "pr.create.finish" => {
            let repo = arg_after(args, "-R")?;
            let base = arg_after(args, "--base")?;
            let head = arg_after(args, "--head")?;
            let title = arg_after(args, "--title")?;
            let body_file = arg_after(args, "--body-file")?;
            let body = std::fs::read_to_string(body_file)
                .with_context(|| format!("failed to read PR body file '{body_file}'"))?;
            create_pr_octocrab(repo, title, head, base, &body, args.contains(&"--draft"))
        }
        "pr.view.base_ref.finish_existing" => {
            let repo = arg_after(args, "-R")?;
            let pr_ref = positional_after(args, "view")?;
            pr_base_ref_octocrab(repo, pr_ref)
        }
        other => bail!("github_client.unsupported_capture_operation: {other}"),
    }
}

fn run_octocrab_status(operation: &str, args: &[&str]) -> Result<()> {
    let _client = github_client(operation)?;
    match operation {
        "pr.edit.body_file" => {
            let repo = arg_after(args, "-R")?;
            let pr_ref = positional_after(args, "edit")?;
            let body_file = arg_after(args, "--body-file")?;
            let body = std::fs::read_to_string(body_file)
                .with_context(|| format!("failed to read PR body file '{body_file}'"))?;
            update_pr_body_octocrab(repo, pr_ref, &body)
        }
        "pr.edit.finish_existing" => {
            let repo = arg_after(args, "-R")?;
            let pr_ref = positional_after(args, "edit")?;
            let title = arg_after(args, "--title")?;
            let body_file = arg_after(args, "--body-file")?;
            let body = std::fs::read_to_string(body_file)
                .with_context(|| format!("failed to read PR body file '{body_file}'"))?;
            update_pr_title_body_octocrab(repo, pr_ref, title, &body)
        }
        "pr.ready.finish" | "pr.ready.finish_merge" => {
            let repo = arg_after(args, "-R")?;
            let pr_ref = positional_after(args, "ready")?;
            mark_pr_ready_octocrab(repo, pr_ref)
        }
        "pr.merge.finish" => {
            let repo = arg_after(args, "-R")?;
            let pr_ref = args
                .last()
                .copied()
                .ok_or_else(|| anyhow!("pr.merge.finish missing PR reference"))?;
            merge_pr_octocrab(repo, pr_ref)
        }
        "issue.comment" => {
            let repo = arg_after(args, "-R")?;
            let issue = positional_after(args, "comment")?
                .parse::<u32>()
                .context("failed to parse issue number for issue.comment")?;
            let body_file = arg_after(args, "--body-file")?;
            let body = std::fs::read_to_string(body_file)
                .with_context(|| format!("failed to read issue comment body file '{body_file}'"))?;
            issue_comment_octocrab(repo, issue, &body)
        }
        "issue.close" => {
            let repo = arg_after(args, "-R")?;
            let issue = positional_after(args, "close")?
                .parse::<u32>()
                .context("failed to parse issue number for issue.close")?;
            let reason = issue_close_reason_from_args(args)?;
            issue_close_octocrab(repo, issue, reason)
        }
        other => bail!("github_client.unsupported_status_operation: {other}"),
    }
}

fn issue_close_reason_from_args(
    args: &[&str],
) -> Result<octocrab::models::issues::IssueStateReason> {
    let raw = arg_after(args, "--reason")
        .or_else(|_| arg_after(args, "--state-reason"))
        .unwrap_or("completed");
    match raw {
        "completed" => Ok(octocrab::models::issues::IssueStateReason::Completed),
        "not_planned" | "not-planned" => {
            Ok(octocrab::models::issues::IssueStateReason::NotPlanned)
        }
        other => bail!(
            "github_client.issue_close: unsupported state reason '{other}'; expected completed or not_planned"
        ),
    }
}

fn arg_after<'a>(args: &'a [&str], flag: &str) -> Result<&'a str> {
    args.windows(2)
        .find_map(|window| (window[0] == flag).then_some(window[1]))
        .ok_or_else(|| anyhow!("missing required argument '{flag}' in GitHub operation"))
}

fn positional_after<'a>(args: &'a [&str], command: &str) -> Result<&'a str> {
    let Some(command_index) = args.iter().position(|arg| *arg == command) else {
        bail!("missing GitHub command '{command}' in operation arguments");
    };
    let mut index = command_index + 1;
    while index < args.len() {
        match args[index] {
            "-R" | "--repo" | "--json" | "--jq" | "--body-file" | "--title" | "--base"
            | "--head" => {
                index += 2;
            }
            "--draft" | "--squash" | "--delete-branch" | "--web" => {
                index += 1;
            }
            other if other.starts_with('-') => {
                index += 1;
            }
            other => return Ok(other),
        }
    }
    bail!("missing positional argument after '{command}' in GitHub operation")
}

pub(super) fn current_pr_url(repo: &str, branch: &str) -> Result<Option<String>> {
    let out = run_gh_capture_allow_failure(
        "pr.list.current_branch",
        &[
            "pr", "list", "-R", repo, "--head", branch, "--state", "open", "--json", "url", "--jq",
            ".[0].url",
        ],
    )?;
    Ok(out
        .and_then(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() || trimmed == "null" {
                None
            } else {
                trimmed.lines().next().map(ToString::to_string)
            }
        })
        .filter(|url| !url.is_empty()))
}

pub(super) fn unresolved_milestone_pr_wave(
    repo: &str,
    version: &str,
    target_queue: &str,
    exclude_branch: Option<&str>,
) -> Result<Vec<OpenPullRequest>> {
    let out = run_gh_capture_allow_failure(
        "pr.list.open_wave",
        &[
            "pr",
            "list",
            "-R",
            repo,
            "--state",
            "open",
            "--json",
            "number,title,url,headRefName,baseRefName,isDraft",
        ],
    )?
    .unwrap_or_else(|| "[]".to_string());
    let prs: Vec<OpenPullRequest> =
        serde_json::from_str(&out).with_context(|| "failed to parse GitHub PR list JSON")?;
    prs.into_iter()
        .filter(|pr| {
            pr_matches_main_version_wave(
                &PullRequestMetadataSnapshot::new(
                    &pr.title,
                    &pr.head_ref_name,
                    &pr.base_ref_name,
                    pr.is_draft,
                ),
                version,
                exclude_branch,
            )
        })
        .map(|mut pr| {
            pr.queue = infer_workflow_queue(&pr.title, "", None).map(str::to_string);
            pr
        })
        .filter(|pr| pr.queue.as_deref() == Some(target_queue))
        .filter_map(|pr| match pr_has_any_closing_linkage(repo, &pr.url) {
            Ok(true) => Some(Ok(pr)),
            Ok(false) => None,
            Err(err) => Some(Err(err)),
        })
        .collect::<Result<Vec<_>>>()
}

fn pr_has_any_closing_linkage(repo: &str, pr_ref: &str) -> Result<bool> {
    let linked = run_gh_capture_allow_failure(
        "pr.view.closing_issues",
        &[
            "pr",
            "view",
            "-R",
            repo,
            pr_ref,
            "--json",
            "closingIssuesReferences",
            "--jq",
            ".closingIssuesReferences[]?.number",
        ],
    )?;
    Ok(
        linked_issue_numbers_from_lines(linked.as_deref().unwrap_or_default())
            .into_iter()
            .next()
            .is_some(),
    )
}

pub(super) fn format_open_pr_wave(prs: &[OpenPullRequest]) -> String {
    prs.iter()
        .map(|pr| {
            format!(
                "- #{} [{}] {} ({})",
                pr.number,
                if pr.is_draft { "draft" } else { "ready" },
                match pr.queue.as_deref() {
                    Some(queue) => format!("[queue={queue}] {}", pr.title),
                    None => format!("[queue=unknown] {}", pr.title),
                },
                pr.url
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(super) fn pr_has_closing_linkage(repo: &str, pr_ref: &str, issue: u32) -> Result<bool> {
    let linked = run_gh_capture_allow_failure(
        "pr.view.closing_issues",
        &[
            "pr",
            "view",
            "-R",
            repo,
            pr_ref,
            "--json",
            "closingIssuesReferences",
            "--jq",
            ".closingIssuesReferences[]?.number",
        ],
    )?;
    let linked_issue_numbers =
        linked_issue_numbers_from_lines(linked.as_deref().unwrap_or_default());
    if linked_issue_numbers_include(&linked_issue_numbers, issue) {
        return Ok(true);
    }
    let body = run_gh_capture_allow_failure(
        "pr.view.body",
        &[
            "pr", "view", "-R", repo, pr_ref, "--json", "body", "--jq", ".body",
        ],
    )?
    .unwrap_or_default();
    Ok(body_contains_closing_linkage(&body, issue))
}

pub(super) fn ensure_pr_closing_linkage(
    repo: &str,
    pr_ref: &str,
    issue: u32,
    no_close: bool,
) -> Result<()> {
    if no_close {
        return Ok(());
    }
    if !pr_has_closing_linkage(repo, pr_ref, issue)? {
        bail!(
            "finish: PR '{}' is missing closing linkage to issue #{}. Include a closing keyword such as 'Closes #{}' in the PR body.",
            pr_ref,
            issue,
            issue
        );
    }
    Ok(())
}

pub(super) fn ensure_or_repair_pr_closing_linkage(
    repo: &str,
    pr_ref: &str,
    issue: u32,
    no_close: bool,
    desired_body_file: &Path,
) -> Result<bool> {
    if no_close {
        return Ok(false);
    }
    if pr_has_closing_linkage(repo, pr_ref, issue)? {
        return Ok(false);
    }
    run_gh_status(
        "pr.edit.body_file",
        &[
            "pr",
            "edit",
            "-R",
            repo,
            pr_ref,
            "--body-file",
            path_str(desired_body_file)?,
        ],
    )?;
    ensure_pr_closing_linkage(repo, pr_ref, issue, no_close)?;
    Ok(true)
}

pub(super) fn attach_pr_janitor(
    repo_root: &Path,
    repo: &str,
    issue: u32,
    branch: &str,
    pr_url: &str,
    expected_pr_state: &str,
) -> Result<()> {
    if std::env::var("ADL_PR_JANITOR_DISABLE").ok().as_deref() == Some("1") {
        return Ok(());
    }

    let command_path = std::env::var("ADL_PR_JANITOR_CMD")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| helper_command_path(repo_root, "adl/tools/attach_pr_janitor.sh"));
    let mut command = helper_command_with_github_context(&command_path);
    let output = command
        .arg("--repo-root")
        .arg(repo_root)
        .arg("--repo")
        .arg(repo)
        .arg("--issue")
        .arg(issue.to_string())
        .arg("--branch")
        .arg(branch)
        .arg("--pr-url")
        .arg(pr_url)
        .arg("--expected-pr-state")
        .arg(expected_pr_state)
        .output()
        .with_context(|| format!("finish: failed to spawn PR janitor command '{command_path}'"))?;
    if !output.status.success() {
        let stderr = redact_for_diagnostics(&String::from_utf8_lossy(&output.stderr));
        let stdout = redact_for_diagnostics(&String::from_utf8_lossy(&output.stdout));
        bail!(
            "finish: PR janitor auto-attach failed for issue #{} and PR '{}': {}{}",
            issue,
            pr_url,
            stderr.trim(),
            if stdout.trim().is_empty() {
                String::new()
            } else {
                format!(" (stdout: {})", stdout.trim())
            }
        );
    }
    Ok(())
}

pub(super) fn attach_post_merge_closeout(
    repo_root: &Path,
    repo: &str,
    issue: u32,
    branch: &str,
    pr_url: &str,
) -> Result<()> {
    if std::env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE")
        .ok()
        .as_deref()
        == Some("1")
    {
        return Ok(());
    }

    let Some(command_path) = std::env::var("ADL_POST_MERGE_CLOSEOUT_CMD")
        .ok()
        .filter(|value| !value.trim().is_empty())
    else {
        observability::emit_event(
            "adl",
            "github_octocrab",
            "skipped",
            &[
                ("operation", "post_merge_closeout.attach"),
                ("reason", "rust_closeout_no_background_watcher"),
            ],
        );
        return Ok(());
    };
    let mut command = helper_command_with_github_context(&command_path);
    let output = command
        .arg("--repo-root")
        .arg(repo_root)
        .arg("--repo")
        .arg(repo)
        .arg("--issue")
        .arg(issue.to_string())
        .arg("--branch")
        .arg(branch)
        .arg("--pr-url")
        .arg(pr_url)
        .output()
        .with_context(|| {
            format!("finish: failed to spawn post-merge closeout command '{command_path}'")
        })?;
    if !output.status.success() {
        let stderr = redact_for_diagnostics(&String::from_utf8_lossy(&output.stderr));
        let stdout = redact_for_diagnostics(&String::from_utf8_lossy(&output.stdout));
        bail!(
            "finish: post-merge closeout auto-attach failed for issue #{} and PR '{}': {}{}",
            issue,
            pr_url,
            stderr.trim(),
            if stdout.trim().is_empty() {
                String::new()
            } else {
                format!(" (stdout: {})", stdout.trim())
            }
        );
    }
    Ok(())
}

const GITHUB_CONTEXT_ENVS: &[&str] = &[
    "ADL_GITHUB_CLIENT",
    "ADL_GITHUB_DISABLE_GH_FALLBACK",
    "ADL_GITHUB_OCTOCRAB_BASE_URI",
    "ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS",
    "GITHUB_TOKEN",
    "GH_TOKEN",
    "ADL_GITHUB_TOKEN_FILE",
    "ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE",
    "ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT",
];

fn helper_command_with_github_context(command_path: &str) -> Command {
    let mut command = Command::new(command_path);
    for key in GITHUB_CONTEXT_ENVS {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        } else {
            command.env_remove(key);
        }
    }
    command
}

fn helper_command_path(repo_root: &Path, relative: &str) -> String {
    let direct = repo_root.join(relative);
    if direct.is_file() {
        return direct.display().to_string();
    }
    let primary_root = resolve_primary_checkout_root(repo_root, None);
    let fallback = primary_root.join(relative);
    if fallback.is_file() {
        return fallback.display().to_string();
    }
    direct.display().to_string()
}

pub(super) fn issue_version(issue: u32, repo: &str) -> Result<Option<String>> {
    let label_versions = gh_issue_label_names(issue, repo)?
        .into_iter()
        .filter_map(|label| {
            label
                .strip_prefix("version:")
                .map(|version| version.trim().to_string())
                .filter(|version| !version.is_empty())
        })
        .collect::<BTreeSet<_>>();
    if label_versions.len() > 1 {
        bail!(
            "issue_version: conflicting version labels for issue #{}: {}",
            issue,
            label_versions.into_iter().collect::<Vec<_>>().join(", ")
        );
    }

    let title_version = gh_issue_title(issue, repo)?.and_then(|title| version_from_title(&title));
    let body_version = match gh_issue_body(issue, repo)? {
        Some(body) => explicit_issue_body_version(&body)?,
        None => None,
    };

    let mut evidence = BTreeMap::new();
    if let Some(version) = label_versions.iter().next().cloned() {
        evidence.insert("label", version);
    }
    if let Some(version) = title_version {
        evidence.insert("title", version);
    }
    if let Some(version) = body_version {
        evidence.insert("body", version);
    }

    let unique_versions = evidence.values().cloned().collect::<BTreeSet<_>>();
    if unique_versions.len() > 1 {
        let sources = evidence
            .into_iter()
            .map(|(source, version)| format!("{source}={version}"))
            .collect::<Vec<_>>()
            .join(", ");
        bail!(
            "issue_version: conflicting version evidence for issue #{}: {}",
            issue,
            sources
        );
    }

    Ok(unique_versions.into_iter().next())
}

fn explicit_issue_body_version(body: &str) -> Result<Option<String>> {
    let mut versions = std::collections::BTreeSet::new();
    for line in body.lines() {
        let trimmed = line.trim();
        let value = trimmed
            .strip_prefix("Version:")
            .or_else(|| trimmed.strip_prefix("version:"))
            .map(str::trim);
        if let Some(version) = value.filter(|value| !value.is_empty()) {
            versions.insert(version.to_string());
        }
    }
    if versions.len() > 1 {
        bail!(
            "issue_version: conflicting explicit body version evidence: {}",
            versions.into_iter().collect::<Vec<_>>().join(", ")
        );
    }
    Ok(versions.into_iter().next())
}

pub(super) fn gh_issue_create(
    repo: &str,
    title: &str,
    body: &str,
    labels_csv: &str,
) -> Result<String> {
    #[derive(Serialize)]
    struct IssueCreatePayload<'a> {
        title: &'a str,
        body: &'a str,
        labels: Vec<String>,
    }

    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("issue.create")? {
        return run_gh_capture_shell(
            "issue.create",
            &[
                "issue", "create", "-R", repo, "--title", title, "--body", body, "--label",
                labels_csv,
            ],
        )
        .map(|url: String| url.trim().to_string());
    }
    let repo_parts = parse_repo(repo)?;
    let labels = issue_labels_from_csv_in_order(labels_csv);
    with_octocrab("issue.create", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let route = format!("/repos/{owner}/{name}/issues");
        let payload = IssueCreatePayload {
            title,
            body,
            labels: labels.clone(),
        };
        let issue: RestIssueRecord = block_on_octocrab(runtime, "issue.create", || async {
            octo.post(route.as_str(), Some(&payload)).await
        })?;
        issue
            .into_issue_record()
            .map(|issue| issue.url)
            .ok_or_else(|| {
                anyhow!("issue create: GitHub returned a pull request or malformed issue")
            })
    })
}

pub(crate) fn gh_issue_list(
    repo: &str,
    state: IssueStateFilter,
    limit: usize,
) -> Result<Vec<IssueRecord>> {
    #[derive(Serialize)]
    struct IssueListQuery<'a> {
        state: &'a str,
        per_page: usize,
        page: usize,
    }

    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.list", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let mut page = 1usize;
        let mut collected = Vec::new();
        while collected.len() < limit {
            let route = format!("/repos/{owner}/{name}/issues");
            let query = IssueListQuery {
                state: state.as_str(),
                per_page: 100,
                page,
            };
            let batch: Vec<RestIssueRecord> = block_on_octocrab(runtime, "issue.list", || async {
                octo.get(route.as_str(), Some(&query)).await
            })?;
            let batch_len = batch.len();
            for issue in batch {
                if let Some(issue) = issue.into_issue_record() {
                    collected.push(issue);
                    if collected.len() == limit {
                        break;
                    }
                }
            }
            if batch_len < 100 {
                break;
            }
            page += 1;
        }
        Ok(collected)
    })
}

pub(crate) fn gh_issue_search(
    repo: &str,
    query: &str,
    state: IssueStateFilter,
    limit: usize,
) -> Result<Vec<IssueRecord>> {
    #[derive(Serialize)]
    struct IssueSearchQuery {
        q: String,
        per_page: usize,
        page: usize,
    }

    let repo_parts = parse_repo(repo)?;
    let state_term = match state {
        IssueStateFilter::Open => " state:open",
        IssueStateFilter::Closed => " state:closed",
        IssueStateFilter::All => "",
    };
    let composed_query = format!(
        "repo:{}/{} is:issue{} {}",
        repo_parts.owner,
        repo_parts.name,
        state_term,
        query.trim()
    );
    with_octocrab("issue.search", |runtime, octo| {
        let mut page = 1usize;
        let mut collected = Vec::new();
        while collected.len() < limit {
            let query = IssueSearchQuery {
                q: composed_query.clone(),
                per_page: 100,
                page,
            };
            let response: RestIssueSearchResult =
                block_on_octocrab(runtime, "issue.search", || async {
                    octo.get("/search/issues", Some(&query)).await
                })?;
            let batch_len = response.items.len();
            for issue in response.items {
                if let Some(issue) = issue.into_issue_record() {
                    collected.push(issue);
                    if collected.len() == limit {
                        break;
                    }
                }
            }
            if batch_len < 100 {
                break;
            }
            page += 1;
        }
        Ok(collected)
    })
}

pub(crate) fn gh_issue_view(repo: &str, issue: u32) -> Result<IssueRecord> {
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.view.full", |runtime, octo| {
        let route = format!(
            "/repos/{}/{}/issues/{}",
            repo_parts.owner, repo_parts.name, issue
        );
        let issue: RestIssueRecord = block_on_octocrab(runtime, "issue.view.full", || async {
            octo.get(route.as_str(), None::<&()>).await
        })?;
        issue.into_issue_record().ok_or_else(|| {
            anyhow!("issue view: GitHub returned a pull request instead of an issue")
        })
    })
}

pub(crate) fn gh_issue_label_names(issue: u32, repo: &str) -> Result<Vec<String>> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("issue.view.labels")? {
        let Some(out) = run_gh_capture_shell_allow_failure(
            "issue.view.labels",
            &[
                "issue",
                "view",
                &issue.to_string(),
                "-R",
                repo,
                "--json",
                "labels",
                "--jq",
                ".labels[].name",
            ],
        )?
        else {
            return Ok(Vec::new());
        };
        return Ok(out
            .lines()
            .map(str::trim)
            .filter(|label: &&str| !label.is_empty())
            .map(ToString::to_string)
            .collect());
    }
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.view.labels", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let issue = block_on_octocrab(runtime, "issue.view.labels", || async {
            octo.issues(&owner, &name).get(issue as u64).await
        })?;
        Ok(issue.labels.into_iter().map(|label| label.name).collect())
    })
}

fn gh_repo_label_names(repo: &str) -> Result<BTreeSet<String>> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("issue.view.repo_labels")? {
        let out = run_gh_capture_shell(
            "issue.view.repo_labels",
            &[
                "label", "list", "-R", repo, "--json", "name", "--jq", ".[].name",
            ],
        )?;
        return Ok(out
            .lines()
            .map(str::trim)
            .filter(|label| !label.is_empty())
            .map(str::to_string)
            .collect());
    }

    #[derive(Serialize)]
    struct RepoLabelsQuery {
        per_page: usize,
        page: usize,
    }

    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.view.repo_labels", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let route = format!("/repos/{owner}/{name}/labels");
        let mut page = 1usize;
        let mut labels = BTreeSet::new();
        loop {
            let query = RepoLabelsQuery {
                per_page: 100,
                page,
            };
            let batch: Vec<RepoLabelRecord> =
                block_on_octocrab(runtime, "issue.view.repo_labels", || async {
                    octo.get(route.as_str(), Some(&query)).await
                })?;
            if batch.is_empty() {
                break;
            }
            let count = batch.len();
            labels.extend(
                batch
                    .into_iter()
                    .map(|label| label.name.trim().to_string())
                    .filter(|label| !label.is_empty()),
            );
            if count < 100 {
                break;
            }
            page += 1;
        }
        Ok(labels)
    })
}

pub(super) fn ensure_repo_labels_exist(
    repo: &str,
    labels: &BTreeSet<String>,
    operation: &str,
) -> Result<()> {
    if labels.is_empty() {
        return Ok(());
    }
    let available = gh_repo_label_names(repo)?;
    let missing = labels.difference(&available).cloned().collect::<Vec<_>>();
    if missing.is_empty() {
        return Ok(());
    }
    bail!(
        "{operation}: repo is missing required GitHub labels: {}. Create them through the approved ADL GitHub label path before retrying so issue metadata does not mutate partially.",
        missing.join(", ")
    );
}

pub(super) fn gh_issue_edit_title(repo: &str, issue: u32, title: &str) -> Result<()> {
    #[derive(Serialize)]
    struct IssueTitlePayload<'a> {
        title: &'a str,
    }

    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("issue.edit.title")? {
        return run_gh_status_shell(
            "issue.edit.title",
            &[
                "issue",
                "edit",
                &issue.to_string(),
                "-R",
                repo,
                "--title",
                title,
            ],
        )
        .with_context(|| format!("create: gh fixture issue edit title failed for issue #{issue}"));
    }
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.edit.title", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let route = format!("/repos/{owner}/{name}/issues/{issue}");
        let payload = IssueTitlePayload { title };
        let _: RestIssueRecord = block_on_octocrab(runtime, "issue.edit.title", || async {
            octo.patch(route.as_str(), Some(&payload)).await
        })?;
        Ok(())
    })
    .with_context(|| format!("create: octocrab issue edit title failed for issue #{issue}"))
}

pub(super) fn ensure_issue_metadata_parity(
    repo: &str,
    issue: u32,
    expected_title: &str,
    labels_csv: &str,
) -> Result<()> {
    let expected = issue_labels_from_csv(labels_csv);
    if expected.is_empty() {
        bail!("create: expected at least one label for tracked issue creation");
    }
    ensure_repo_labels_exist(repo, &expected, "create")?;

    let current_title = gh_issue_title(issue, repo)?.unwrap_or_default();
    let actual = IssueMetadataSnapshot::new(current_title, gh_issue_label_names(issue, repo)?);
    let plan = plan_issue_metadata_parity(expected_title, &expected, &actual);

    let mut final_labels = actual.labels.clone();
    for label in &plan.labels_to_add {
        final_labels.insert(label.clone());
    }
    for label in &plan.version_labels_to_remove {
        final_labels.remove(label);
    }
    if let Some(title) = plan.title_update {
        gh_issue_edit_title(repo, issue, &title)?;
    }
    if !plan.labels_to_add.is_empty() || !plan.version_labels_to_remove.is_empty() {
        gh_issue_set_labels(repo, issue, &final_labels.into_iter().collect::<Vec<_>>())?;
    }

    let final_title = gh_issue_title(issue, repo)?.unwrap_or_default();
    let final_actual = IssueMetadataSnapshot::new(final_title, gh_issue_label_names(issue, repo)?);
    if final_actual.title != expected_title {
        bail!(
            "create: issue #{} title mismatch after metadata parity enforcement: expected '{}', got '{}'",
            issue,
            expected_title,
            final_actual.title
        );
    }
    let problems = issue_metadata_drift(expected_title, &expected, &final_actual);
    if !problems.is_empty() {
        bail!(
            "create: issue #{} metadata drift remains after parity enforcement: {}",
            issue,
            problems.join("; ")
        );
    }
    Ok(())
}

pub(super) fn gh_issue_edit_body(repo: &str, issue: u32, body: &str) -> Result<()> {
    #[derive(Serialize)]
    struct IssueBodyPayload<'a> {
        body: &'a str,
    }

    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("issue.edit.body")? {
        return run_gh_status_shell(
            "issue.edit.body",
            &[
                "issue",
                "edit",
                &issue.to_string(),
                "-R",
                repo,
                "--body",
                body,
            ],
        )
        .with_context(|| format!("create: gh fixture issue edit failed for issue #{issue}"));
    }
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.edit.body", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let route = format!("/repos/{owner}/{name}/issues/{issue}");
        let payload = IssueBodyPayload { body };
        let _: RestIssueRecord = block_on_octocrab(runtime, "issue.edit.body", || async {
            octo.patch(route.as_str(), Some(&payload)).await
        })?;
        Ok(())
    })
    .with_context(|| format!("create: octocrab issue edit failed for issue #{issue}"))
}

pub(super) fn gh_issue_title(issue: u32, repo: &str) -> Result<Option<String>> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("issue.view.title")? {
        let out = run_gh_capture_shell_allow_failure(
            "issue.view.title",
            &[
                "issue",
                "view",
                &issue.to_string(),
                "-R",
                repo,
                "--json",
                "title",
                "--jq",
                ".title",
            ],
        )?;
        return Ok(out.and_then(|title: String| {
            Some(title.trim().to_string()).filter(|title: &String| !title.is_empty())
        }));
    }
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.view.title", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let issue = block_on_octocrab(runtime, "issue.view.title", || async {
            octo.issues(&owner, &name).get(issue as u64).await
        })?;
        Ok(Some(issue.title).filter(|title| !title.trim().is_empty()))
    })
}

pub(crate) fn gh_issue_body(issue: u32, repo: &str) -> Result<Option<String>> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("issue.view.body")? {
        let out = run_gh_capture_shell_allow_failure(
            "issue.view.body",
            &[
                "issue",
                "view",
                &issue.to_string(),
                "-R",
                repo,
                "--json",
                "body",
                "--jq",
                ".body",
            ],
        )?;
        return Ok(out.and_then(|body: String| {
            Some(body.trim().to_string()).filter(|body: &String| !body.is_empty())
        }));
    }
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.view.body", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let issue = block_on_octocrab(runtime, "issue.view.body", || async {
            octo.issues(&owner, &name).get(issue as u64).await
        })?;
        Ok(issue.body.filter(|body| !body.trim().is_empty()))
    })
}

pub(crate) fn gh_issue_is_closed_completed(issue: u32, repo: &str) -> Result<bool> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("issue.view.state")? {
        #[derive(Deserialize)]
        struct IssueStateFixture {
            state: Option<String>,
            #[serde(rename = "stateReason")]
            state_reason: Option<String>,
        }

        let Some(out) = run_gh_capture_shell_allow_failure(
            "issue.view.state",
            &[
                "issue",
                "view",
                &issue.to_string(),
                "-R",
                repo,
                "--json",
                "state,stateReason",
            ],
        )?
        else {
            return Ok(false);
        };
        let state: IssueStateFixture =
            serde_json::from_str(&out).context("failed to parse GitHub issue state JSON")?;
        return Ok(state.state.as_deref() == Some("CLOSED")
            && state.state_reason.as_deref() == Some("COMPLETED"));
    }
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.view.state", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let issue = block_on_octocrab(runtime, "issue.view.state", || async {
            octo.issues(&owner, &name).get(issue as u64).await
        })?;
        Ok(issue.state == octocrab::models::IssueState::Closed
            && issue.state_reason == Some(octocrab::models::issues::IssueStateReason::Completed))
    })
}

pub(super) fn gh_issue_set_labels(repo: &str, issue: u32, labels: &[String]) -> Result<()> {
    #[derive(Serialize)]
    struct IssueLabelsPayload<'a> {
        labels: &'a [String],
    }

    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("issue.edit.labels")? {
        return run_gh_status_shell(
            "issue.edit.labels",
            &[
                "issue",
                "edit",
                &issue.to_string(),
                "-R",
                repo,
                "--add-label",
                &labels.join(","),
            ],
        )
        .with_context(|| {
            format!("create: gh fixture issue label update failed for issue #{issue}")
        });
    }
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.edit.labels", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let route = format!("/repos/{owner}/{name}/issues/{issue}");
        let payload = IssueLabelsPayload { labels };
        let _: RestIssueRecord = block_on_octocrab(runtime, "issue.edit.labels", || async {
            octo.patch(route.as_str(), Some(&payload)).await
        })?;
        Ok(())
    })
    .with_context(|| format!("create: octocrab issue label update failed for issue #{issue}"))
}

pub(super) fn gh_issue_comment(repo: &str, issue: u32, body: &str) -> Result<()> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("issue.comment")? {
        return run_gh_status_shell(
            "issue.comment",
            &[
                "issue",
                "comment",
                &issue.to_string(),
                "-R",
                repo,
                "--body",
                body,
            ],
        )
        .with_context(|| format!("issue comment: gh fixture comment failed for issue #{issue}"));
    }
    issue_comment_octocrab(repo, issue, body)
}

pub(super) fn gh_issue_close(repo: &str, issue: u32, reason: IssueCloseReason) -> Result<()> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("issue.close")? {
        return run_gh_status_shell(
            "issue.close",
            &[
                "issue",
                "close",
                &issue.to_string(),
                "-R",
                repo,
                "--reason",
                reason.as_str(),
            ],
        )
        .with_context(|| format!("issue close: gh fixture close failed for issue #{issue}"));
    }
    issue_close_octocrab(repo, issue, issue_close_reason(reason))
}

fn issue_close_reason(reason: IssueCloseReason) -> octocrab::models::issues::IssueStateReason {
    match reason {
        IssueCloseReason::Completed => octocrab::models::issues::IssueStateReason::Completed,
        IssueCloseReason::NotPlanned => octocrab::models::issues::IssueStateReason::NotPlanned,
    }
}

#[cfg(test)]
mod tests;
