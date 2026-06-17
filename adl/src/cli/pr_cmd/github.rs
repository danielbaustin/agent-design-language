use super::*;
use crate::cli::observability;
#[cfg(test)]
use crate::cli::pr_cmd::github_client::GithubClientMode;
use crate::cli::pr_cmd::github_client::{
    body_contains_closing_linkage, issue_labels_from_csv, issue_labels_from_csv_in_order,
    issue_metadata_drift, linked_issue_numbers_from_lines, linked_issue_numbers_include,
    plan_issue_metadata_parity, pr_matches_main_version_wave, AdlGithubClient, GithubClientBackend,
    IssueMetadataSnapshot, PullRequestMetadataSnapshot,
};
use crate::cli::pr_cmd_args::IssueStateFilter;
use crate::cli::pr_cmd_prompt::infer_workflow_queue;
use ::adl::control_plane::resolve_primary_checkout_root;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

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
    #[serde(skip)]
    pub(super) queue: Option<String>,
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

#[derive(Debug, Clone, Deserialize)]
struct PullRequestClosingIssuesResponse {
    repository: Option<PullRequestClosingIssuesRepository>,
}

#[derive(Debug, Clone, Deserialize)]
struct PullRequestClosingIssuesRepository {
    #[serde(rename = "pullRequest")]
    pull_request: Option<PullRequestClosingIssuesPullRequest>,
}

#[derive(Debug, Clone, Deserialize)]
struct PullRequestClosingIssuesPullRequest {
    #[serde(rename = "closingIssuesReferences")]
    closing_issues_references: PullRequestClosingIssuesConnection,
}

#[derive(Debug, Clone, Deserialize)]
struct PullRequestClosingIssuesConnection {
    nodes: Option<Vec<Option<PullRequestClosingIssueNode>>>,
}

#[derive(Debug, Clone, Deserialize)]
struct PullRequestClosingIssueNode {
    number: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct PullRequestNodeIdResponse {
    repository: Option<PullRequestNodeIdRepository>,
}

#[derive(Debug, Clone, Deserialize)]
struct PullRequestNodeIdRepository {
    #[serde(rename = "pullRequest")]
    pull_request: Option<PullRequestNodeIdPullRequest>,
}

#[derive(Debug, Clone, Deserialize)]
struct PullRequestNodeIdPullRequest {
    id: String,
}

#[derive(Debug, Clone, Deserialize)]
struct MarkPullRequestReadyResponse {
    #[serde(rename = "markPullRequestReadyForReview")]
    _mark_pull_request_ready_for_review: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
struct PullRequestValidationStatusResponse {
    repository: Option<PullRequestValidationRepository>,
}

#[derive(Debug, Clone, Deserialize)]
struct PullRequestValidationRepository {
    #[serde(rename = "pullRequest")]
    pull_request: Option<PullRequestValidationPullRequest>,
}

#[derive(Debug, Clone, Deserialize)]
struct PullRequestValidationPullRequest {
    number: u64,
    #[serde(rename = "headRefOid")]
    head_ref_oid: String,
    state: String,
    #[serde(rename = "isDraft")]
    is_draft: bool,
    #[serde(rename = "statusCheckRollup")]
    status_check_rollup: Option<PullRequestValidationRollup>,
}

#[derive(Debug, Clone, Deserialize)]
struct PullRequestValidationRollup {
    contexts: Option<PullRequestValidationContextConnection>,
}

#[derive(Debug, Clone, Deserialize)]
struct PullRequestValidationContextConnection {
    nodes: Option<Vec<Option<PullRequestValidationContextNode>>>,
    #[serde(rename = "pageInfo")]
    page_info: PullRequestValidationPageInfo,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct PullRequestValidationPageInfo {
    #[serde(rename = "hasNextPage")]
    has_next_page: bool,
    #[serde(rename = "endCursor")]
    end_cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "__typename")]
enum PullRequestValidationContextNode {
    CheckRun {
        name: String,
        status: Option<String>,
        conclusion: Option<String>,
        #[serde(rename = "databaseId")]
        database_id: Option<i64>,
        #[serde(rename = "checkSuite")]
        check_suite: Option<PullRequestValidationCheckSuite>,
    },
    StatusContext {
        context: String,
        state: Option<String>,
    },
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Deserialize)]
struct PullRequestValidationCheckSuite {
    #[serde(rename = "workflowRun")]
    workflow_run: Option<PullRequestValidationWorkflowRun>,
}

#[derive(Debug, Clone, Deserialize)]
struct PullRequestValidationWorkflowRun {
    #[serde(rename = "databaseId")]
    database_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PrValidationCheckSnapshot {
    name: String,
    status: String,
    conclusion: String,
    job_run_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PrValidationSnapshot {
    pr_number: u64,
    commit_sha: String,
    state: String,
    is_draft: bool,
    checks: Vec<PrValidationCheckSnapshot>,
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
    pub(super) checks: Vec<PrValidationCheckReport>,
    pub(super) failed_checks: Vec<PrValidationCheckReport>,
    pub(super) pending_checks: Vec<PrValidationCheckReport>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PrValidationDisposition {
    Pending,
    Success,
    Failed,
    Cancelled,
    Skipped,
    TimedOut,
}

fn current_pr_url_octocrab(repo: &str, branch: &str) -> Result<Option<String>> {
    let repo_parts = parse_repo(repo)?;
    let head = format!("{}:{branch}", repo_parts.owner);
    with_octocrab("pr.list.current_branch", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let page = block_on_octocrab(runtime, "pr.list.current_branch", || async {
            octo.pulls(&owner, &name)
                .list()
                .state(octocrab::params::State::Open)
                .head(head.clone())
                .per_page(10)
                .send()
                .await
        })?;
        Ok(page
            .items
            .into_iter()
            .find_map(|pr| pr.html_url.map(|url| url.to_string())))
    })
}

fn list_open_prs_octocrab(repo: &str) -> Result<Vec<OpenPullRequest>> {
    let repo_parts = parse_repo(repo)?;
    with_octocrab("pr.list.open_wave", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let page = block_on_octocrab(runtime, "pr.list.open_wave", || async {
            octo.pulls(&owner, &name)
                .list()
                .state(octocrab::params::State::Open)
                .per_page(100)
                .send()
                .await
        })?;
        Ok(page
            .items
            .into_iter()
            .map(|pr| OpenPullRequest {
                number: pr.number.unwrap_or_default() as u32,
                title: pr.title.unwrap_or_default(),
                url: pr.html_url.map(|url| url.to_string()).unwrap_or_default(),
                head_ref_name: pr
                    .head
                    .as_ref()
                    .map(|head| head.ref_field.clone())
                    .unwrap_or_default(),
                base_ref_name: pr
                    .base
                    .as_ref()
                    .map(|base| base.ref_field.clone())
                    .unwrap_or_default(),
                is_draft: pr.draft.unwrap_or(false),
                queue: None,
            })
            .collect())
    })
}

fn pr_body_octocrab(repo: &str, pr_ref: &str) -> Result<String> {
    let repo_parts = parse_repo(repo)?;
    let number = parse_pr_number(pr_ref)?;
    with_octocrab("pr.view.body", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let pr = block_on_octocrab(runtime, "pr.view.body", || async {
            octo.pulls(&owner, &name).get(number).await
        })?;
        Ok(pr.body.unwrap_or_default())
    })
}

fn pr_closing_issue_numbers_octocrab(repo: &str, pr_ref: &str) -> Result<Vec<u32>> {
    let repo_parts = parse_repo(repo)?;
    let number = parse_pr_number(pr_ref)? as i64;
    with_octocrab("pr.view.closing_issues", |runtime, octo| {
        let payload = serde_json::json!({
            "query": r#"
                query($owner: String!, $name: String!, $number: Int!) {
                  repository(owner: $owner, name: $name) {
                    pullRequest(number: $number) {
                      closingIssuesReferences(first: 100) {
                        nodes {
                          number
                        }
                      }
                    }
                  }
                }
            "#,
            "variables": {
                "owner": repo_parts.owner,
                "name": repo_parts.name,
                "number": number,
            }
        });
        let response: PullRequestClosingIssuesResponse =
            block_on_octocrab(runtime, "pr.view.closing_issues", || async {
                octo.graphql::<PullRequestClosingIssuesResponse>(&payload)
                    .await
            })?;
        let numbers = response
            .repository
            .and_then(|repo| repo.pull_request)
            .map(|pr| {
                pr.closing_issues_references
                    .nodes
                    .unwrap_or_default()
                    .into_iter()
                    .flatten()
                    .map(|node| node.number)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        Ok(numbers)
    })
}

fn pr_base_ref_octocrab(repo: &str, pr_ref: &str) -> Result<String> {
    let repo_parts = parse_repo(repo)?;
    let number = parse_pr_number(pr_ref)?;
    with_octocrab("pr.view.base_ref.finish_existing", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let pr = block_on_octocrab(runtime, "pr.view.base_ref.finish_existing", || async {
            octo.pulls(&owner, &name).get(number).await
        })?;
        Ok(pr
            .base
            .map(|base| base.ref_field)
            .filter(|base| !base.is_empty())
            .unwrap_or_default())
    })
}

fn create_pr_octocrab(
    repo: &str,
    title: &str,
    head: &str,
    base: &str,
    body: &str,
    draft: bool,
) -> Result<String> {
    let repo_parts = parse_repo(repo)?;
    with_octocrab("pr.create.finish", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let pr = block_on_octocrab(runtime, "pr.create.finish", || async {
            octo.pulls(&owner, &name)
                .create(title, head, base)
                .body(body.to_string())
                .draft(draft)
                .send()
                .await
        })?;
        pr.html_url
            .map(|url| url.to_string())
            .ok_or_else(|| anyhow!("github_client.octocrab_transport: PR create returned no url"))
    })
}

fn update_pr_body_octocrab(repo: &str, pr_ref: &str, body: &str) -> Result<()> {
    let repo_parts = parse_repo(repo)?;
    let number = parse_pr_number(pr_ref)?;
    with_octocrab("pr.edit.body_file", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        block_on_octocrab(runtime, "pr.edit.body_file", || async {
            octo.pulls(&owner, &name)
                .update(number)
                .body(body.to_string())
                .send()
                .await
        })?;
        Ok(())
    })
}

fn update_pr_title_body_octocrab(repo: &str, pr_ref: &str, title: &str, body: &str) -> Result<()> {
    let repo_parts = parse_repo(repo)?;
    let number = parse_pr_number(pr_ref)?;
    with_octocrab("pr.edit.finish_existing", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        block_on_octocrab(runtime, "pr.edit.finish_existing", || async {
            octo.pulls(&owner, &name)
                .update(number)
                .title(title.to_string())
                .body(body.to_string())
                .send()
                .await
        })?;
        Ok(())
    })
}

fn mark_pr_ready_octocrab(repo: &str, pr_ref: &str) -> Result<()> {
    let repo_parts = parse_repo(repo)?;
    let number = parse_pr_number(pr_ref)? as i64;
    with_octocrab("pr.ready", |runtime, octo| {
        let id_payload = serde_json::json!({
            "query": r#"
                query($owner: String!, $name: String!, $number: Int!) {
                  repository(owner: $owner, name: $name) {
                    pullRequest(number: $number) {
                      id
                    }
                  }
                }
            "#,
            "variables": {
                "owner": repo_parts.owner,
                "name": repo_parts.name,
                "number": number,
            }
        });
        let id_response: PullRequestNodeIdResponse =
            block_on_octocrab(runtime, "pr.ready", || async {
                octo.graphql::<PullRequestNodeIdResponse>(&id_payload).await
            })?;
        let pull_request_id = id_response
            .repository
            .and_then(|repo| repo.pull_request)
            .map(|pr| pr.id)
            .filter(|id| !id.trim().is_empty())
            .ok_or_else(|| {
                anyhow!("GitHub did not return a pull request node id for PR {pr_ref}")
            })?;
        let ready_payload = serde_json::json!({
            "query": r#"
                mutation($pullRequestId: ID!) {
                  markPullRequestReadyForReview(input: { pullRequestId: $pullRequestId }) {
                    pullRequest {
                      id
                      isDraft
                    }
                  }
                }
            "#,
            "variables": {
                "pullRequestId": pull_request_id,
            }
        });
        let _: MarkPullRequestReadyResponse = block_on_octocrab(runtime, "pr.ready", || async {
            octo.graphql::<MarkPullRequestReadyResponse>(&ready_payload)
                .await
        })?;
        Ok(())
    })
}

fn merge_pr_octocrab(repo: &str, pr_ref: &str) -> Result<()> {
    let repo_parts = parse_repo(repo)?;
    let number = parse_pr_number(pr_ref)?;
    with_octocrab("pr.merge.finish", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        block_on_octocrab(runtime, "pr.merge.finish", || async {
            octo.pulls(&owner, &name)
                .merge(number)
                .method(octocrab::params::pulls::MergeMethod::Squash)
                .send()
                .await
        })?;
        Ok(())
    })
}

pub(super) fn wait_for_pr_validation_finish(repo: &str, pr_ref: &str) -> Result<()> {
    let report = wait_for_pr_validation_report(repo, pr_ref)?;
    match report.disposition.as_str() {
        "success" | "skipped" => Ok(()),
        "failed" => {
            bail!(
                "pr validation failed for PR #{pr}: at least one check failed",
                pr = report.pr_number
            )
        }
        "cancelled" => {
            bail!(
                "pr validation cancelled for PR #{pr}: at least one check was cancelled",
                pr = report.pr_number
            )
        }
        "timed_out" => bail!("pr validation timed out for PR #{}", report.pr_number),
        other => bail!(
            "pr validation ended with unsupported disposition for PR #{}: {}",
            report.pr_number,
            other
        ),
    }
}

pub(super) fn wait_for_pr_validation_report(
    repo: &str,
    pr_ref: &str,
) -> Result<PrValidationReport> {
    let timeout = pr_validation_wait_timeout();
    let poll_delay = pr_validation_wait_poll_delay();
    let started = Instant::now();
    let mut poll_count = 0usize;

    loop {
        poll_count += 1;
        let snapshot = pr_validation_status_octocrab(repo, pr_ref)?;
        let disposition = classify_pr_validation_snapshot(&snapshot);
        let next_delay = if disposition == PrValidationDisposition::Pending {
            poll_delay
        } else {
            Duration::ZERO
        };
        emit_pr_validation_wait_snapshot(&snapshot, disposition, started, poll_count, next_delay);

        match disposition {
            PrValidationDisposition::Success
            | PrValidationDisposition::Skipped
            | PrValidationDisposition::Failed
            | PrValidationDisposition::Cancelled
            | PrValidationDisposition::TimedOut => {
                return Ok(pr_validation_report_from_snapshot_with_disposition(
                    &snapshot,
                    disposition,
                ))
            }
            PrValidationDisposition::Pending => {
                if started.elapsed() >= timeout {
                    emit_pr_validation_wait_timeout(&snapshot, started, poll_count, Duration::ZERO);
                    return Ok(pr_validation_report_from_snapshot_with_disposition(
                        &snapshot,
                        PrValidationDisposition::TimedOut,
                    ));
                }
                std::thread::sleep(poll_delay);
            }
        }
    }
}

pub(super) fn pr_validation_report(repo: &str, pr_ref: &str) -> Result<PrValidationReport> {
    let snapshot = pr_validation_status_octocrab(repo, pr_ref)?;
    Ok(pr_validation_report_from_snapshot_with_disposition(
        &snapshot,
        classify_pr_validation_snapshot(&snapshot),
    ))
}

fn pr_validation_status_octocrab(repo: &str, pr_ref: &str) -> Result<PrValidationSnapshot> {
    let repo_parts = parse_repo(repo)?;
    let number = parse_pr_number(pr_ref)? as i64;
    with_octocrab("pr.validation.status", |runtime, octo| {
        let mut after: Option<String> = None;
        let mut snapshot: Option<PrValidationSnapshot> = None;
        loop {
            let payload = serde_json::json!({
                "query": r#"
                query($owner: String!, $name: String!, $number: Int!, $after: String) {
                  repository(owner: $owner, name: $name) {
                    pullRequest(number: $number) {
                      number
                      headRefOid
                      state
                      isDraft
                      statusCheckRollup {
                        contexts(first: 100, after: $after) {
                          nodes {
                            __typename
                            ... on CheckRun {
                              name
                              status
                              conclusion
                              databaseId
                              checkSuite {
                                workflowRun {
                                  databaseId
                                }
                              }
                            }
                            ... on StatusContext {
                              context
                              state
                            }
                          }
                          pageInfo {
                            hasNextPage
                            endCursor
                          }
                        }
                      }
                    }
                  }
                }
            "#,
                "variables": {
                    "owner": repo_parts.owner,
                    "name": repo_parts.name,
                    "number": number,
                    "after": after,
                }
            });
            let response: PullRequestValidationStatusResponse =
                block_on_octocrab(runtime, "pr.validation.status", || async {
                    octo.graphql::<PullRequestValidationStatusResponse>(&payload)
                        .await
                })?;
            let pr = response
                .repository
                .and_then(|repo| repo.pull_request)
                .ok_or_else(|| {
                    anyhow!("GitHub did not return validation status for PR {pr_ref}")
                })?;
            let (mut page_snapshot, page_info) = pr_validation_snapshot_from_response(pr);
            if let Some(current) = snapshot.as_mut() {
                current.checks.append(&mut page_snapshot.checks);
            } else {
                snapshot = Some(page_snapshot);
            }
            if !page_info.has_next_page {
                return snapshot.ok_or_else(|| {
                    anyhow!("GitHub did not return validation status for PR {pr_ref}")
                });
            }
            after = page_info.end_cursor;
            if after.as_deref().unwrap_or_default().trim().is_empty() {
                bail!(
                    "GitHub validation status for PR {pr_ref} is paginated but did not return an end cursor"
                );
            }
        }
    })
}

fn pr_validation_snapshot_from_response(
    pr: PullRequestValidationPullRequest,
) -> (PrValidationSnapshot, PullRequestValidationPageInfo) {
    let contexts = pr.status_check_rollup.and_then(|rollup| rollup.contexts);
    let page_info = contexts
        .as_ref()
        .map(|contexts| contexts.page_info.clone())
        .unwrap_or_default();
    let checks = contexts
        .and_then(|contexts| contexts.nodes)
        .unwrap_or_default()
        .into_iter()
        .flatten()
        .filter_map(|node| match node {
            PullRequestValidationContextNode::CheckRun {
                name,
                status,
                conclusion,
                database_id,
                check_suite,
            } => {
                let job_run_id = check_suite
                    .and_then(|suite| suite.workflow_run)
                    .and_then(|run| run.database_id)
                    .or(database_id)
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                Some(PrValidationCheckSnapshot {
                    name,
                    status: status.unwrap_or_else(|| "UNKNOWN".to_string()),
                    conclusion: conclusion.unwrap_or_else(|| "UNKNOWN".to_string()),
                    job_run_id,
                })
            }
            PullRequestValidationContextNode::StatusContext { context, state } => {
                let state = state.unwrap_or_else(|| "UNKNOWN".to_string());
                Some(PrValidationCheckSnapshot {
                    name: context,
                    status: status_context_state_to_status(&state).to_string(),
                    conclusion: status_context_state_to_conclusion(&state).to_string(),
                    job_run_id: "unknown".to_string(),
                })
            }
            PullRequestValidationContextNode::Other => None,
        })
        .collect();
    (
        PrValidationSnapshot {
            pr_number: pr.number,
            commit_sha: pr.head_ref_oid,
            state: pr.state,
            is_draft: pr.is_draft,
            checks,
        },
        page_info,
    )
}

fn classify_pr_validation_snapshot(snapshot: &PrValidationSnapshot) -> PrValidationDisposition {
    if snapshot.state == "CLOSED" {
        return PrValidationDisposition::Cancelled;
    }
    if snapshot.is_draft {
        return PrValidationDisposition::Pending;
    }
    if snapshot.checks.is_empty() {
        return PrValidationDisposition::Pending;
    }
    if snapshot
        .checks
        .iter()
        .any(|check| validation_conclusion_is_cancelled(&check.conclusion))
    {
        return PrValidationDisposition::Cancelled;
    }
    if snapshot.checks.iter().any(|check| {
        validation_conclusion_is_failed(&check.conclusion)
            || status_context_failure_status(&check.status)
    }) {
        return PrValidationDisposition::Failed;
    }
    if snapshot
        .checks
        .iter()
        .any(|check| validation_check_is_pending(&check.status, &check.conclusion))
    {
        return PrValidationDisposition::Pending;
    }
    if snapshot
        .checks
        .iter()
        .all(|check| validation_conclusion_is_skipped(&check.conclusion))
    {
        return PrValidationDisposition::Skipped;
    }
    PrValidationDisposition::Success
}

fn pr_validation_report_from_snapshot_with_disposition(
    snapshot: &PrValidationSnapshot,
    disposition: PrValidationDisposition,
) -> PrValidationReport {
    let checks = snapshot
        .checks
        .iter()
        .map(pr_validation_check_report)
        .collect::<Vec<_>>();
    let failed_checks = snapshot
        .checks
        .iter()
        .filter(|check| {
            validation_conclusion_is_failed(&check.conclusion)
                || status_context_failure_status(&check.status)
        })
        .map(pr_validation_check_report)
        .collect::<Vec<_>>();
    let pending_checks = snapshot
        .checks
        .iter()
        .filter(|check| validation_check_is_pending(&check.status, &check.conclusion))
        .map(pr_validation_check_report)
        .collect::<Vec<_>>();
    PrValidationReport {
        pr_number: snapshot.pr_number,
        commit_sha: snapshot.commit_sha.clone(),
        pr_state: snapshot.state.clone(),
        is_draft: snapshot.is_draft,
        disposition: disposition.as_event_result().to_string(),
        checks,
        failed_checks,
        pending_checks,
    }
}

fn pr_validation_check_report(check: &PrValidationCheckSnapshot) -> PrValidationCheckReport {
    PrValidationCheckReport {
        name: check.name.clone(),
        status: check.status.clone(),
        conclusion: check.conclusion.clone(),
        job_run_id: check.job_run_id.clone(),
    }
}

fn emit_pr_validation_wait_snapshot(
    snapshot: &PrValidationSnapshot,
    disposition: PrValidationDisposition,
    started: Instant,
    poll_count: usize,
    next_poll_delay: Duration,
) {
    if snapshot.checks.is_empty() {
        let (status, conclusion) = match disposition {
            PrValidationDisposition::Skipped => ("COMPLETED", "SKIPPED"),
            PrValidationDisposition::TimedOut => ("TIMED_OUT", "TIMED_OUT"),
            _ => ("PENDING", "UNKNOWN"),
        };
        emit_pr_validation_wait_event(
            snapshot,
            disposition,
            "aggregate",
            status,
            conclusion,
            "unknown",
            started,
            poll_count,
            next_poll_delay,
        );
        return;
    }
    for check in &snapshot.checks {
        let check_disposition = classify_pr_validation_check(check);
        emit_pr_validation_wait_event(
            snapshot,
            check_disposition,
            &check.name,
            &check.status,
            &check.conclusion,
            &check.job_run_id,
            started,
            poll_count,
            next_poll_delay,
        );
    }
    emit_pr_validation_wait_event(
        snapshot,
        disposition,
        "aggregate",
        "COMPLETED",
        disposition.as_conclusion(),
        "unknown",
        started,
        poll_count,
        next_poll_delay,
    );
}

fn emit_pr_validation_wait_timeout(
    snapshot: &PrValidationSnapshot,
    started: Instant,
    poll_count: usize,
    next_poll_delay: Duration,
) {
    emit_pr_validation_wait_event(
        snapshot,
        PrValidationDisposition::TimedOut,
        "aggregate",
        "TIMED_OUT",
        "TIMED_OUT",
        "unknown",
        started,
        poll_count,
        next_poll_delay,
    );
}

#[allow(clippy::too_many_arguments)]
fn emit_pr_validation_wait_event(
    snapshot: &PrValidationSnapshot,
    disposition: PrValidationDisposition,
    check_name: &str,
    status: &str,
    conclusion: &str,
    job_run_id: &str,
    started: Instant,
    poll_count: usize,
    next_poll_delay: Duration,
) {
    let pr_number = snapshot.pr_number.to_string();
    let elapsed_ms = started.elapsed().as_millis().to_string();
    let poll_count = poll_count.to_string();
    let next_poll_delay_ms = next_poll_delay.as_millis().to_string();
    let is_draft = snapshot.is_draft.to_string();
    let wait_reason = pr_validation_wait_reason(snapshot, disposition, check_name);
    observability::emit_event(
        "adl",
        "pr.validation.wait",
        disposition.as_event_result(),
        &[
            ("pr_number", pr_number.as_str()),
            ("commit_sha", snapshot.commit_sha.as_str()),
            ("check_name", check_name),
            ("job_run_id", job_run_id),
            ("pr_state", snapshot.state.as_str()),
            ("is_draft", is_draft.as_str()),
            ("wait_reason", wait_reason),
            ("status", status),
            ("conclusion", conclusion),
            ("elapsed_ms", elapsed_ms.as_str()),
            ("poll_count", poll_count.as_str()),
            ("next_poll_delay_ms", next_poll_delay_ms.as_str()),
        ],
    );
}

fn pr_validation_wait_reason(
    snapshot: &PrValidationSnapshot,
    disposition: PrValidationDisposition,
    check_name: &str,
) -> &'static str {
    if snapshot.is_draft && disposition == PrValidationDisposition::Pending {
        "pr_draft"
    } else if snapshot.checks.is_empty() && disposition == PrValidationDisposition::Pending {
        "checks_not_reported"
    } else if check_name == "aggregate" {
        "aggregate"
    } else {
        "check_state"
    }
}

fn classify_pr_validation_check(check: &PrValidationCheckSnapshot) -> PrValidationDisposition {
    if validation_conclusion_is_cancelled(&check.conclusion) {
        PrValidationDisposition::Cancelled
    } else if validation_conclusion_is_failed(&check.conclusion)
        || status_context_failure_status(&check.status)
    {
        PrValidationDisposition::Failed
    } else if validation_check_is_pending(&check.status, &check.conclusion) {
        PrValidationDisposition::Pending
    } else if validation_conclusion_is_skipped(&check.conclusion) {
        PrValidationDisposition::Skipped
    } else {
        PrValidationDisposition::Success
    }
}

impl PrValidationDisposition {
    fn as_event_result(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Success => "success",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Skipped => "skipped",
            Self::TimedOut => "timed_out",
        }
    }

    fn as_conclusion(self) -> &'static str {
        match self {
            Self::Pending => "PENDING",
            Self::Success => "SUCCESS",
            Self::Failed => "FAILURE",
            Self::Cancelled => "CANCELLED",
            Self::Skipped => "SKIPPED",
            Self::TimedOut => "TIMED_OUT",
        }
    }
}

fn validation_check_is_pending(status: &str, conclusion: &str) -> bool {
    matches!(
        status,
        "QUEUED" | "REQUESTED" | "WAITING" | "PENDING" | "IN_PROGRESS" | "EXPECTED"
    ) || conclusion == "UNKNOWN"
        || conclusion.is_empty()
}

fn validation_conclusion_is_failed(conclusion: &str) -> bool {
    matches!(
        conclusion,
        "FAILURE" | "TIMED_OUT" | "ACTION_REQUIRED" | "STARTUP_FAILURE" | "ERROR"
    )
}

fn validation_conclusion_is_cancelled(conclusion: &str) -> bool {
    conclusion == "CANCELLED"
}

fn validation_conclusion_is_skipped(conclusion: &str) -> bool {
    matches!(conclusion, "SKIPPED" | "NEUTRAL")
}

fn status_context_failure_status(status: &str) -> bool {
    matches!(status, "FAILURE" | "ERROR")
}

fn status_context_state_to_status(state: &str) -> &'static str {
    match state {
        "PENDING" => "PENDING",
        "EXPECTED" => "EXPECTED",
        "ERROR" => "ERROR",
        "FAILURE" => "FAILURE",
        "SUCCESS" => "SUCCESS",
        _ => "UNKNOWN",
    }
}

fn status_context_state_to_conclusion(state: &str) -> &'static str {
    match state {
        "SUCCESS" => "SUCCESS",
        "FAILURE" | "ERROR" => "FAILURE",
        "PENDING" | "EXPECTED" => "UNKNOWN",
        _ => "UNKNOWN",
    }
}

fn pr_validation_wait_timeout() -> Duration {
    duration_env_ms("ADL_PR_VALIDATION_WAIT_TIMEOUT_MS", 15 * 60 * 1000)
}

fn pr_validation_wait_poll_delay() -> Duration {
    duration_env_ms("ADL_PR_VALIDATION_WAIT_POLL_MS", 10 * 1000)
}

fn duration_env_ms(key: &str, default_ms: u64) -> Duration {
    Duration::from_millis(
        std::env::var(key)
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(default_ms),
    )
}

fn issue_comment_octocrab(repo: &str, issue: u32, body: &str) -> Result<()> {
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.comment", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        block_on_octocrab(runtime, "issue.comment", || async {
            octo.issues(&owner, &name)
                .create_comment(issue as u64, body.to_string())
                .await
        })?;
        Ok(())
    })
    .with_context(|| format!("github_client.octocrab_transport: issue comment failed for #{issue}"))
}

fn issue_close_octocrab(
    repo: &str,
    issue: u32,
    reason: octocrab::models::issues::IssueStateReason,
) -> Result<()> {
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.close", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        block_on_octocrab(runtime, "issue.close", || async {
            octo.issues(&owner, &name)
                .update(issue as u64)
                .state(octocrab::models::IssueState::Closed)
                .state_reason(reason.clone())
                .send()
                .await
        })?;
        Ok(())
    })
    .with_context(|| format!("github_client.octocrab_transport: issue close failed for #{issue}"))
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
        None => "credential_status=missing_token; set GITHUB_TOKEN or GH_TOKEN before live C-SDLC GitHub operations; load the token from an operator-approved secret source and pass it only to the ADL command environment without echoing it; do not fall back to direct gh commands".to_string(),
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
        .map(|value| value.trim().to_string());
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
        .map(|value| value.trim().to_string());
    }
    let body = std::fs::read_to_string(body_file)
        .with_context(|| format!("failed to read PR body file '{}'", body_file.display()))?;
    create_pr_octocrab(repo, title, head, base, &body, draft)
}

fn pr_ready_with_optional_fixture_fallback(
    _operation: &str,
    repo: &str,
    pr_ref: &str,
) -> Result<()> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed(_operation)? {
        return run_gh_status_shell(_operation, &["pr", "ready", "-R", repo, pr_ref]);
    }
    mark_pr_ready_octocrab(repo, pr_ref)
}

pub(super) fn pr_ready_finish(repo: &str, pr_ref: &str) -> Result<()> {
    pr_ready_with_optional_fixture_fallback("pr.ready.finish", repo, pr_ref)
}

pub(super) fn pr_ready_finish_merge(repo: &str, pr_ref: &str) -> Result<()> {
    pr_ready_with_optional_fixture_fallback("pr.ready.finish_merge", repo, pr_ref)
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

#[cfg(test)]
fn test_gh_fixture_fallback_allowed(operation: &str) -> Result<bool> {
    let client = AdlGithubClient::from_env()
        .with_context(|| format!("github client policy rejected operation '{operation}'"))?;
    let config = client.config();
    Ok(config.requested_mode == GithubClientMode::Auto && config.gh_fallback_allowed)
}

#[cfg(test)]
fn test_github_cli_fixture_command(operation: &str) -> Result<std::path::PathBuf> {
    if let Some(value) = std::env::var_os("ADL_TEST_GITHUB_CLI_FIXTURE") {
        let path = std::path::PathBuf::from(value);
        if path.as_os_str().is_empty() {
            bail!(
                "github_client.test_fixture: operation '{}' has empty ADL_TEST_GITHUB_CLI_FIXTURE",
                operation
            );
        }
        return Ok(path);
    }

    let temp_root = std::env::temp_dir()
        .canonicalize()
        .unwrap_or_else(|_| std::env::temp_dir());
    if let Some(path) = std::env::var_os("PATH")
        .into_iter()
        .flat_map(|paths| std::env::split_paths(&paths).collect::<Vec<_>>())
        .map(|dir| dir.join("gh"))
        .find(|candidate| {
            candidate.is_file()
                && candidate
                    .canonicalize()
                    .map(|path| path.starts_with(&temp_root))
                    .unwrap_or(false)
                && std::fs::read_to_string(candidate)
                    .map(|text| text.contains("ADL_GITHUB_TEST_FIXTURE"))
                    .unwrap_or(false)
        })
    {
        return Ok(path);
    }

    let path = temp_root.join(format!("adl-github-cli-fixture-{}", std::process::id()));
    std::fs::write(
        &path,
        "#!/usr/bin/env bash\n# ADL_GITHUB_TEST_FIXTURE\nset -euo pipefail\nif [[ \"$1 $2\" == \"issue view\" ]]; then\n  if printf '%s\\n' \"$*\" | grep -q -- '--json title'; then\n    printf '[v0.86][tools] Init test\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json labels'; then\n    printf 'track:roadmap\\ntype:task\\narea:tools\\nversion:v0.86\\n'\n    exit 0\n  fi\n  if printf '%s\\n' \"$*\" | grep -q -- '--json body'; then\n    printf '\\n'\n    exit 0\n  fi\n  printf '{\"state\":\"OPEN\",\"stateReason\":null}\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue create\" ]]; then\n  printf 'https://github.com/example/repo/issues/1\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"pr list\" ]]; then\n  printf '[]\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"pr view\" ]]; then\n  printf '{}\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"pr edit\" ]]; then\n  exit 0\nfi\nif [[ \"$1 $2\" == \"pr create\" ]]; then\n  printf 'https://github.com/example/repo/pull/1\\n'\n  exit 0\nfi\nexit 1\n",
    )
    .with_context(|| {
        format!(
            "github_client.test_fixture: failed to write default fixture for {operation}"
        )
    })?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&path)
            .with_context(|| {
                format!(
                    "github_client.test_fixture: failed to stat default fixture for {operation}"
                )
            })?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&path, perms).with_context(|| {
            format!("github_client.test_fixture: failed to chmod default fixture for {operation}")
        })?;
    }
    Ok(path)
}

#[cfg(test)]
fn run_gh_capture_shell(operation: &str, args: &[&str]) -> Result<String> {
    let fixture = test_github_cli_fixture_command(operation)?;
    let output = Command::new(&fixture)
        .args(args)
        .output()
        .with_context(|| {
            format!(
                "github_client.test_fixture: failed to spawn fixture command '{}' for {operation}",
                fixture.display()
            )
        })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        bail!(
            "github_client.test_fixture: operation '{}' failed: {}{}",
            operation,
            stderr.trim(),
            if stdout.trim().is_empty() {
                String::new()
            } else {
                format!(" (stdout: {})", stdout.trim())
            }
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
fn run_gh_capture_shell_allow_failure(operation: &str, args: &[&str]) -> Result<Option<String>> {
    let fixture = test_github_cli_fixture_command(operation)?;
    let output = Command::new(&fixture)
        .args(args)
        .output()
        .with_context(|| {
            format!(
                "github_client.test_fixture: failed to spawn fixture command '{}' for {operation}",
                fixture.display()
            )
        })?;
    if !output.status.success() {
        return Ok(None);
    }
    Ok(Some(String::from_utf8_lossy(&output.stdout).to_string()))
}

#[cfg(test)]
fn run_gh_status_shell(operation: &str, args: &[&str]) -> Result<()> {
    let fixture = test_github_cli_fixture_command(operation)?;
    let output = Command::new(&fixture)
        .args(args)
        .output()
        .with_context(|| {
            format!(
                "github_client.test_fixture: failed to spawn fixture command '{}' for {operation}",
                fixture.display()
            )
        })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        bail!(
            "github_client.test_fixture: operation '{}' failed: {}{}",
            operation,
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

#[cfg(test)]
fn run_gh_status_shell_allow_failure(operation: &str, args: &[&str]) -> Result<bool> {
    let fixture = test_github_cli_fixture_command(operation)?;
    let output = Command::new(&fixture)
        .args(args)
        .output()
        .with_context(|| {
            format!(
                "github_client.test_fixture: failed to spawn fixture command '{}' for {operation}",
                fixture.display()
            )
        })?;
    Ok(output.status.success())
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
    Fut: std::future::Future<Output = octocrab::Result<T>>,
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
        match runtime.block_on(make_future()) {
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
            | "issue.edit.labels"
            | "issue.close"
    )
}

fn with_octocrab<T>(
    operation: &str,
    f: impl FnOnce(&tokio::runtime::Runtime, octocrab::Octocrab) -> Result<T>,
) -> Result<T> {
    let runtime = build_octocrab_runtime(operation)?;
    let _runtime_guard = runtime.enter();
    let client = github_client(operation)?;
    let octo = client
        .octocrab()
        .map_err(|err| anyhow!("github_client.octocrab_build: {err}"))?;
    f(&runtime, octo)
}

fn build_octocrab_runtime(operation: &str) -> Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .with_context(|| {
            format!("github_client.octocrab_runtime: failed to build runtime for {operation}")
        })
}

fn run_octocrab_capture(operation: &str, args: &[&str]) -> Result<String> {
    match operation {
        "pr.list.current_branch" => {
            let repo = arg_after(args, "-R")?;
            let branch = arg_after(args, "--head")?;
            current_pr_url_octocrab(repo, branch).map(|url| url.unwrap_or_default())
        }
        "pr.list.open_wave" => {
            let repo = arg_after(args, "-R")?;
            let prs = list_open_prs_octocrab(repo)?;
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
    let output = Command::new(&command_path)
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
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
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
    let output = Command::new(&command_path)
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
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
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
    for label in gh_issue_label_names(issue, repo)? {
        if let Some(version) = label.strip_prefix("version:") {
            return Ok(Some(version.trim().to_string()));
        }
    }

    let title = gh_issue_title(issue, repo)?;
    Ok(title.and_then(|title| version_from_title(&title)))
}

pub(super) fn gh_issue_create(
    repo: &str,
    title: &str,
    body: &str,
    labels_csv: &str,
) -> Result<String> {
    #[cfg(test)]
    if test_gh_fixture_fallback_allowed("issue.create")? {
        return run_gh_capture_shell(
            "issue.create",
            &[
                "issue", "create", "-R", repo, "--title", title, "--body", body, "--label",
                labels_csv,
            ],
        )
        .map(|url| url.trim().to_string());
    }
    let repo_parts = parse_repo(repo)?;
    let labels = issue_labels_from_csv_in_order(labels_csv);
    with_octocrab("issue.create", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let issue = block_on_octocrab(runtime, "issue.create", || async {
            octo.issues(&owner, &name)
                .create(title)
                .body(body.to_string())
                .labels(labels.clone())
                .send()
                .await
        })?;
        Ok(issue.html_url.to_string())
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
            .filter(|label| !label.is_empty())
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

fn gh_issue_edit_title(repo: &str, issue: u32, title: &str) -> Result<()> {
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
        block_on_octocrab(runtime, "issue.edit.title", || async {
            octo.issues(&owner, &name)
                .update(issue as u64)
                .title(title)
                .send()
                .await
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
        block_on_octocrab(runtime, "issue.edit.body", || async {
            octo.issues(&owner, &name)
                .update(issue as u64)
                .body(body)
                .send()
                .await
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
        return Ok(
            out.and_then(|title| Some(title.trim().to_string()).filter(|title| !title.is_empty()))
        );
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
        return Ok(
            out.and_then(|body| Some(body.trim().to_string()).filter(|body| !body.is_empty()))
        );
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

fn gh_issue_set_labels(repo: &str, issue: u32, labels: &[String]) -> Result<()> {
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
        block_on_octocrab(runtime, "issue.edit.labels", || async {
            octo.issues(&owner, &name)
                .update(issue as u64)
                .labels(labels)
                .send()
                .await
        })?;
        Ok(())
    })
    .with_context(|| format!("create: octocrab issue label update failed for issue #{issue}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::tests::env_lock as cli_env_lock;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::{Path, PathBuf};
    use std::thread;
    use std::time::Duration;
    use tiny_http::{Header, Response, Server};

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        cli_env_lock()
    }

    fn unique_temp_dir(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        path.push(format!("{name}-{nanos}-{}", std::process::id()));
        fs::create_dir_all(&path).expect("temp dir");
        path
    }

    fn write_executable(path: &Path, body: &str) {
        let body = if path.file_name().and_then(|name| name.to_str()) == Some("gh")
            && !body.contains("ADL_GITHUB_TEST_FIXTURE")
        {
            body.replacen(
                "#!/usr/bin/env bash\n",
                "#!/usr/bin/env bash\n# ADL_GITHUB_TEST_FIXTURE\n",
                1,
            )
        } else {
            body.to_string()
        };
        fs::write(path, body).expect("write executable");
        let mut perms = fs::metadata(path).expect("metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms).expect("chmod");
    }

    fn restore_env(key: &str, value: Option<String>) {
        unsafe {
            if let Some(value) = value {
                std::env::set_var(key, value);
            } else {
                std::env::remove_var(key);
            }
        }
    }

    fn clear_github_policy_env() -> Vec<(&'static str, Option<String>)> {
        let keys = [
            "ADL_GITHUB_CLIENT",
            "ADL_GITHUB_DISABLE_GH_FALLBACK",
            "ADL_GITHUB_OCTOCRAB_BASE_URI",
            "ADL_TEST_GITHUB_CLI_FIXTURE",
            "GITHUB_TOKEN",
            "GH_TOKEN",
        ];
        let saved = keys
            .into_iter()
            .map(|key| (key, std::env::var(key).ok()))
            .collect::<Vec<_>>();
        unsafe {
            for (key, _) in &saved {
                std::env::remove_var(key);
            }
        }
        saved
    }

    fn restore_github_policy_env(saved: Vec<(&'static str, Option<String>)>) {
        for (key, value) in saved {
            restore_env(key, value);
        }
    }

    fn reserve_local_port() -> u16 {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind local port");
        let port = listener.local_addr().expect("local addr").port();
        drop(listener);
        port
    }

    fn json_response(body: impl Into<String>) -> Response<std::io::Cursor<Vec<u8>>> {
        let mut response = Response::from_string(body.into()).with_status_code(200);
        if let Ok(header) = Header::from_bytes("Content-Type", "application/json") {
            response = response.with_header(header);
        }
        response
    }

    fn pr_fixture(number: u64, title: &str, body: &str, head: &str, base: &str) -> String {
        serde_json::json!({
            "url": format!("https://api.github.test/repos/owner/repo/pulls/{number}"),
            "html_url": format!("https://github.com/owner/repo/pull/{number}"),
            "number": number,
            "title": title,
            "body": body,
            "draft": number.is_multiple_of(2),
            "head": {
                "ref": head,
                "sha": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            },
            "base": {
                "ref": base,
                "sha": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
            }
        })
        .to_string()
    }

    fn label_fixture(name: &str) -> serde_json::Value {
        serde_json::json!({
            "id": 1,
            "node_id": format!("LABEL_{name}"),
            "url": format!("https://api.github.test/labels/{name}"),
            "name": name,
            "description": null,
            "color": "ededed",
            "default": false
        })
    }

    fn author_fixture() -> serde_json::Value {
        serde_json::json!({
            "login": "octo-test",
            "id": 1,
            "node_id": "USER_1",
            "avatar_url": "https://github.test/avatar.png",
            "gravatar_id": "",
            "url": "https://api.github.test/users/octo-test",
            "html_url": "https://github.com/octo-test",
            "followers_url": "https://api.github.test/users/octo-test/followers",
            "following_url": "https://api.github.test/users/octo-test/following{/other_user}",
            "gists_url": "https://api.github.test/users/octo-test/gists{/gist_id}",
            "starred_url": "https://api.github.test/users/octo-test/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.test/users/octo-test/subscriptions",
            "organizations_url": "https://api.github.test/users/octo-test/orgs",
            "repos_url": "https://api.github.test/users/octo-test/repos",
            "events_url": "https://api.github.test/users/octo-test/events{/privacy}",
            "received_events_url": "https://api.github.test/users/octo-test/received_events",
            "type": "User",
            "site_admin": false,
            "name": null,
            "patch_url": null
        })
    }

    fn issue_fixture(number: u32, title: &str, body: Option<&str>, labels: &[&str]) -> String {
        serde_json::json!({
            "id": number,
            "node_id": format!("ISSUE_{number}"),
            "url": format!("https://api.github.test/repos/owner/repo/issues/{number}"),
            "repository_url": "https://api.github.test/repos/owner/repo",
            "labels_url": format!("https://api.github.test/repos/owner/repo/issues/{number}/labels{{/name}}"),
            "comments_url": format!("https://api.github.test/repos/owner/repo/issues/{number}/comments"),
            "events_url": format!("https://api.github.test/repos/owner/repo/issues/{number}/events"),
            "html_url": format!("https://github.com/owner/repo/issues/{number}"),
            "number": number,
            "state": "closed",
            "state_reason": "completed",
            "title": title,
            "body": body,
            "user": author_fixture(),
            "labels": labels.iter().map(|label| label_fixture(label)).collect::<Vec<_>>(),
            "assignees": [],
            "locked": false,
            "comments": 0,
            "created_at": "2026-06-14T00:00:00Z",
            "updated_at": "2026-06-14T00:00:00Z"
        })
        .to_string()
    }

    fn issue_summary_fixture(
        number: u32,
        title: &str,
        state: &str,
        closed_at: Option<&str>,
        milestone: Option<&str>,
        labels: &[&str],
    ) -> serde_json::Value {
        let mut issue = serde_json::json!({
            "id": number,
            "node_id": format!("ISSUE_{number}"),
            "url": format!("https://api.github.test/repos/owner/repo/issues/{number}"),
            "repository_url": "https://api.github.test/repos/owner/repo",
            "labels_url": format!("https://api.github.test/repos/owner/repo/issues/{number}/labels{{/name}}"),
            "comments_url": format!("https://api.github.test/repos/owner/repo/issues/{number}/comments"),
            "events_url": format!("https://api.github.test/repos/owner/repo/issues/{number}/events"),
            "html_url": format!("https://github.com/owner/repo/issues/{number}"),
            "number": number,
            "state": state,
            "closed_at": closed_at,
            "title": title,
            "body": format!("body for {number}"),
            "user": author_fixture(),
            "labels": labels.iter().map(|label| label_fixture(label)).collect::<Vec<_>>(),
            "assignees": [],
            "locked": false,
            "comments": 0,
            "created_at": "2026-06-14T00:00:00Z",
            "updated_at": "2026-06-14T00:00:00Z"
        });
        issue["milestone"] = milestone
            .map(|value| serde_json::json!({"title": value}))
            .unwrap_or(serde_json::Value::Null);
        issue
    }

    fn issue_comment_fixture(issue: u32, body: &str) -> String {
        serde_json::json!({
            "id": 9900 + issue,
            "node_id": format!("ISSUE_COMMENT_{issue}"),
            "url": format!("https://api.github.test/repos/owner/repo/issues/comments/{}", 9900 + issue),
            "html_url": format!("https://github.com/owner/repo/issues/{issue}#issuecomment-{}", 9900 + issue),
            "issue_url": format!("https://api.github.test/repos/owner/repo/issues/{issue}"),
            "body": body,
            "user": author_fixture(),
            "created_at": "2026-06-14T00:00:00Z",
            "updated_at": "2026-06-14T00:00:00Z"
        })
        .to_string()
    }

    fn spawn_octocrab_test_server(
        expected_requests: usize,
    ) -> (String, thread::JoinHandle<Vec<String>>) {
        let port = reserve_local_port();
        let bind_addr = format!("127.0.0.1:{port}");
        let server = Server::http(&bind_addr).expect("bind octocrab test server");
        let handle = thread::spawn(move || {
            let mut seen = Vec::new();
            for _ in 0..expected_requests {
                let Some(mut request) = server
                    .recv_timeout(Duration::from_secs(5))
                    .expect("octocrab test server receive")
                else {
                    break;
                };
                let method = request.method().as_str().to_string();
                let url = request.url().to_string();
                let mut body = String::new();
                let _ = request.as_reader().read_to_string(&mut body);
                seen.push(format!("{method} {url} {body}"));
                let path = url.split('?').next().unwrap_or(url.as_str());
                let response_body = match (method.as_str(), path) {
                    ("GET", "/repos/owner/repo/pulls") => {
                        if url.contains("per_page=100") {
                            format!(
                                "[{}]",
                                pr_fixture(
                                    1160,
                                    "[v0.91.5][Sprint 1][tools] Open wave",
                                    "Closes #3698",
                                    "codex/3698-next",
                                    "main"
                                )
                            )
                        } else {
                            format!(
                                "[{}]",
                                pr_fixture(
                                    1159,
                                    "[v0.91.5][tools] Current branch",
                                    "Closes #3697",
                                    "codex/3697-octocrab-operational-transport",
                                    "main"
                                )
                            )
                        }
                    }
                    ("GET", "/repos/owner/repo/pulls/1159") => pr_fixture(
                        1159,
                        "[v0.91.5][tools] Existing PR",
                        "Existing body\n\nCloses #3697\n",
                        "codex/3697-octocrab-operational-transport",
                        "main",
                    ),
                    ("PATCH", "/repos/owner/repo/pulls/1159") => pr_fixture(
                        1159,
                        "[v0.91.5][tools] Updated PR",
                        "Updated body\n\nCloses #3697\n",
                        "codex/3697-octocrab-operational-transport",
                        "main",
                    ),
                    ("POST", "/repos/owner/repo/pulls") => pr_fixture(
                        1162,
                        "[v0.91.5][tools] New PR",
                        "New body\n\nCloses #3697\n",
                        "codex/3697-octocrab-operational-transport",
                        "main",
                    ),
                    ("PUT", "/repos/owner/repo/pulls/1159/merge") => {
                        r#"{"sha":"cccccccccccccccccccccccccccccccccccccccc","merged":true,"message":"merged"}"#
                            .to_string()
                    }
                    ("POST", "/graphql") => {
                        if body.contains("markPullRequestReadyForReview") {
                            serde_json::json!({
                                "data": {
                                    "markPullRequestReadyForReview": {
                                        "pullRequest": {
                                            "id": "PR_kwDOready",
                                            "isDraft": false
                                        }
                                    }
                                }
                            })
                            .to_string()
                        } else if body.contains("pullRequest(number: $number)") && body.contains("id") && !body.contains("closingIssuesReferences") {
                            serde_json::json!({
                                "data": {
                                    "repository": {
                                        "pullRequest": {
                                            "id": "PR_kwDOready"
                                        }
                                    }
                                }
                            })
                            .to_string()
                        } else {
                            serde_json::json!({
                                "data": {
                                    "repository": {
                                        "pullRequest": {
                                            "closingIssuesReferences": {
                                                "nodes": [{"number": 3697}, null, {"number": 3698}]
                                            }
                                        }
                                    }
                                }
                            })
                            .to_string()
                        }
                    }
                    ("POST", "/repos/owner/repo/issues") => issue_fixture(
                        77,
                        "[v0.91.5][tools] Created issue",
                        Some("created"),
                        &["version:v0.91.5"],
                    ),
                    ("GET", "/repos/owner/repo/issues/77") => issue_fixture(
                        77,
                        "[v0.91.5][tools] Issue title",
                        Some("issue body"),
                        &["version:v0.91.5", "area:tools"],
                    ),
                    ("GET", "/repos/owner/repo/issues/92") => serde_json::json!({
                        "number": 92,
                        "pull_request": {"url": "https://api.github.test/repos/owner/repo/pulls/92"}
                    })
                    .to_string(),
                    ("GET", "/repos/owner/repo/issues") => serde_json::json!([
                        issue_summary_fixture(
                            91,
                            "[v0.91.5][docs] Deferred-work ledger",
                            "open",
                            None,
                            Some("v0.91.5"),
                            &["version:v0.91.5", "area:docs"]
                        ),
                        serde_json::json!({
                            "number": 92,
                            "pull_request": {"url": "https://api.github.test/repos/owner/repo/pulls/92"}
                        }),
                        issue_summary_fixture(
                            93,
                            "[v0.91.5][quality] Reviewer checklist",
                            "closed",
                            Some("2026-06-15T00:00:00Z"),
                            Some("v0.91.5"),
                            &["version:v0.91.5", "area:quality"]
                        )
                    ])
                    .to_string(),
                    ("PATCH", "/repos/owner/repo/issues/77") => issue_fixture(
                        77,
                        "[v0.91.5][tools] Updated issue",
                        Some("updated body"),
                        &["version:v0.91.5", "area:tools", "type:task"],
                    ),
                    ("POST", "/repos/owner/repo/issues/77/comments") => {
                        issue_comment_fixture(77, "closeout line 1\n\ncloseout line 3\n")
                    }
                    ("GET", "/search/issues") => serde_json::json!({
                        "items": [
                            issue_summary_fixture(
                                94,
                                "[v0.91.5][multi-agent][docs] Compare single-agent vs multi-agent overhead on one docs audit",
                                "open",
                                None,
                                Some("v0.91.5"),
                                &["version:v0.91.5", "area:docs", "track:backlog"]
                            )
                        ]
                    })
                    .to_string(),
                    _ => serde_json::json!({
                        "message": format!("unexpected request {method} {url}")
                    })
                    .to_string(),
                };
                let _ = request.respond(json_response(response_body));
            }
            seen
        });
        (format!("http://{bind_addr}"), handle)
    }

    fn spawn_transient_octocrab_test_server() -> (String, thread::JoinHandle<Vec<String>>) {
        let port = reserve_local_port();
        let bind_addr = format!("127.0.0.1:{port}");
        let server = Server::http(&bind_addr).expect("bind transient octocrab test server");
        let handle = thread::spawn(move || {
            let mut seen = Vec::new();
            for attempt in 1..=2 {
                let Some(request) = server
                    .recv_timeout(Duration::from_secs(5))
                    .expect("transient octocrab test server receive")
                else {
                    break;
                };
                let method = request.method().as_str().to_string();
                let url = request.url().to_string();
                seen.push(format!("{method} {url}"));
                if attempt == 1 {
                    let _ = request.respond(
                        json_response(r#"{"message":"intermittent upstream fault"}"#)
                            .with_status_code(502),
                    );
                } else {
                    let _ = request.respond(json_response(issue_fixture(
                        88,
                        "[v0.91.5][tools] Retry succeeds",
                        Some("retry body"),
                        &["version:v0.91.5"],
                    )));
                }
            }
            seen
        });
        (format!("http://{bind_addr}"), handle)
    }

    fn pr_validation_graphql_response(
        status: &str,
        conclusion: Option<&str>,
        check_name: &str,
    ) -> String {
        pr_validation_graphql_response_page(status, conclusion, check_name, false, None)
    }

    fn pr_validation_graphql_response_page(
        status: &str,
        conclusion: Option<&str>,
        check_name: &str,
        has_next_page: bool,
        end_cursor: Option<&str>,
    ) -> String {
        serde_json::json!({
            "data": {
                "repository": {
                    "pullRequest": {
                        "number": 1159,
                        "headRefOid": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                        "state": "OPEN",
                        "isDraft": false,
                        "statusCheckRollup": {
                            "contexts": {
                                "nodes": [
                                    {
                                        "__typename": "CheckRun",
                                        "name": check_name,
                                        "status": status,
                                        "conclusion": conclusion,
                                        "databaseId": 991,
                                        "checkSuite": {
                                            "workflowRun": {
                                                "databaseId": 8801
                                            }
                                        }
                                    }
                                ],
                                "pageInfo": {
                                    "hasNextPage": has_next_page,
                                    "endCursor": end_cursor
                                }
                            }
                        }
                    }
                }
            }
        })
        .to_string()
    }

    fn spawn_validation_status_paginated_server() -> (String, thread::JoinHandle<Vec<String>>) {
        let port = reserve_local_port();
        let bind_addr = format!("127.0.0.1:{port}");
        let server = Server::http(&bind_addr).expect("bind validation status pagination server");
        let handle = thread::spawn(move || {
            let mut seen = Vec::new();
            for page in 1..=2 {
                let Some(mut request) = server
                    .recv_timeout(Duration::from_secs(5))
                    .expect("validation pagination server receive")
                else {
                    break;
                };
                let method = request.method().as_str().to_string();
                let url = request.url().to_string();
                let mut body = String::new();
                let _ = request.as_reader().read_to_string(&mut body);
                seen.push(format!("{method} {url} {body}"));
                let response = if page == 1 {
                    pr_validation_graphql_response_page(
                        "COMPLETED",
                        Some("SUCCESS"),
                        "adl-ci",
                        true,
                        Some("cursor-1"),
                    )
                } else {
                    pr_validation_graphql_response_page(
                        "COMPLETED",
                        Some("SUCCESS"),
                        "adl-coverage",
                        false,
                        None,
                    )
                };
                let _ = request.respond(json_response(response));
            }
            seen
        });
        (format!("http://{bind_addr}"), handle)
    }

    fn spawn_validation_status_once_server(
        status: &'static str,
        conclusion: Option<&'static str>,
        check_name: &'static str,
    ) -> (String, thread::JoinHandle<Vec<String>>) {
        let port = reserve_local_port();
        let bind_addr = format!("127.0.0.1:{port}");
        let server = Server::http(&bind_addr).expect("bind validation status single server");
        let handle = thread::spawn(move || {
            let mut seen = Vec::new();
            let Some(mut request) = server
                .recv_timeout(Duration::from_secs(5))
                .expect("validation single server receive")
            else {
                return seen;
            };
            let method = request.method().as_str().to_string();
            let url = request.url().to_string();
            let mut body = String::new();
            let _ = request.as_reader().read_to_string(&mut body);
            seen.push(format!("{method} {url} {body}"));
            let _ = request.respond(json_response(pr_validation_graphql_response(
                status, conclusion, check_name,
            )));
            seen
        });
        (format!("http://{bind_addr}"), handle)
    }

    fn spawn_validation_status_transient_server() -> (String, thread::JoinHandle<Vec<String>>) {
        let port = reserve_local_port();
        let bind_addr = format!("127.0.0.1:{port}");
        let server = Server::http(&bind_addr).expect("bind validation status server");
        let handle = thread::spawn(move || {
            let mut seen = Vec::new();
            for attempt in 1..=2 {
                let Some(mut request) = server
                    .recv_timeout(Duration::from_secs(5))
                    .expect("validation status server receive")
                else {
                    break;
                };
                let method = request.method().as_str().to_string();
                let url = request.url().to_string();
                let mut body = String::new();
                let _ = request.as_reader().read_to_string(&mut body);
                seen.push(format!("{method} {url} {body}"));
                if attempt == 1 {
                    let _ = request.respond(
                        json_response(r#"{"message":"intermittent graphql fault"}"#)
                            .with_status_code(502),
                    );
                } else {
                    let _ = request.respond(json_response(pr_validation_graphql_response(
                        "COMPLETED",
                        Some("SUCCESS"),
                        "adl-ci",
                    )));
                }
            }
            seen
        });
        (format!("http://{bind_addr}"), handle)
    }

    #[test]
    fn pr_validation_wait_classifies_pending_failed_successful_and_skipped_states() {
        let snapshot = |checks: Vec<PrValidationCheckSnapshot>| PrValidationSnapshot {
            pr_number: 1159,
            commit_sha: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            state: "OPEN".to_string(),
            is_draft: false,
            checks,
        };

        assert_eq!(
            classify_pr_validation_snapshot(&snapshot(vec![PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "IN_PROGRESS".to_string(),
                conclusion: "UNKNOWN".to_string(),
                job_run_id: "8801".to_string(),
            }])),
            PrValidationDisposition::Pending
        );
        assert_eq!(
            classify_pr_validation_snapshot(&snapshot(vec![PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "FAILURE".to_string(),
                job_run_id: "8801".to_string(),
            }])),
            PrValidationDisposition::Failed
        );
        assert_eq!(
            classify_pr_validation_snapshot(&snapshot(vec![PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "SUCCESS".to_string(),
                job_run_id: "8801".to_string(),
            }])),
            PrValidationDisposition::Success
        );
        assert_eq!(
            classify_pr_validation_snapshot(&snapshot(Vec::new())),
            PrValidationDisposition::Pending
        );
        assert_eq!(
            classify_pr_validation_snapshot(&snapshot(vec![PrValidationCheckSnapshot {
                name: "optional-lane".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "SKIPPED".to_string(),
                job_run_id: "8803".to_string(),
            }])),
            PrValidationDisposition::Skipped
        );
        assert_eq!(
            classify_pr_validation_snapshot(&snapshot(vec![PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "CANCELLED".to_string(),
                job_run_id: "8801".to_string(),
            }])),
            PrValidationDisposition::Cancelled
        );

        let merged_success = PrValidationSnapshot {
            state: "MERGED".to_string(),
            checks: vec![PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "SUCCESS".to_string(),
                job_run_id: "8804".to_string(),
            }],
            ..snapshot(Vec::new())
        };
        assert_eq!(
            classify_pr_validation_snapshot(&merged_success),
            PrValidationDisposition::Success
        );
    }

    #[test]
    fn pr_validation_wait_emits_tail_friendly_events_for_terminal_and_pending_states() {
        let _guard = env_lock();
        let temp = unique_temp_dir("adl-pr-validation-wait-log");
        let log_path = temp.join("events.log");
        unsafe {
            std::env::set_var("ADL_OBSERVABILITY_STDERR", "0");
            std::env::set_var(
                "ADL_OBSERVABILITY_LOG",
                log_path.to_str().expect("log path utf8"),
            );
        }
        let snapshot = PrValidationSnapshot {
            pr_number: 1159,
            commit_sha: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            state: "OPEN".to_string(),
            is_draft: false,
            checks: vec![
                PrValidationCheckSnapshot {
                    name: "adl-ci".to_string(),
                    status: "IN_PROGRESS".to_string(),
                    conclusion: "UNKNOWN".to_string(),
                    job_run_id: "8801".to_string(),
                },
                PrValidationCheckSnapshot {
                    name: "adl-coverage".to_string(),
                    status: "COMPLETED".to_string(),
                    conclusion: "FAILURE".to_string(),
                    job_run_id: "8802".to_string(),
                },
            ],
        };

        emit_pr_validation_wait_snapshot(
            &snapshot,
            PrValidationDisposition::Failed,
            Instant::now(),
            3,
            Duration::from_millis(250),
        );
        let success = PrValidationSnapshot {
            checks: vec![PrValidationCheckSnapshot {
                name: "adl-ci".to_string(),
                status: "COMPLETED".to_string(),
                conclusion: "SUCCESS".to_string(),
                job_run_id: "8801".to_string(),
            }],
            ..snapshot.clone()
        };
        emit_pr_validation_wait_snapshot(
            &success,
            PrValidationDisposition::Success,
            Instant::now(),
            5,
            Duration::ZERO,
        );
        emit_pr_validation_wait_timeout(&snapshot, Instant::now(), 4, Duration::ZERO);
        let no_checks_pending = PrValidationSnapshot {
            checks: Vec::new(),
            ..snapshot.clone()
        };
        emit_pr_validation_wait_snapshot(
            &no_checks_pending,
            PrValidationDisposition::Pending,
            Instant::now(),
            1,
            Duration::from_millis(250),
        );
        let draft_no_checks = PrValidationSnapshot {
            is_draft: true,
            checks: Vec::new(),
            ..snapshot.clone()
        };
        emit_pr_validation_wait_snapshot(
            &draft_no_checks,
            PrValidationDisposition::Pending,
            Instant::now(),
            2,
            Duration::from_millis(250),
        );

        let log = fs::read_to_string(&log_path).expect("read validation wait log");
        assert!(log.contains("stage=pr.validation.wait"));
        assert!(log.contains("result=pending"));
        assert!(log.contains("result=success"));
        assert!(log.contains("result=failed"));
        assert!(log.contains("result=timed_out"));
        assert!(log.contains("pr_number=1159"));
        assert!(log.contains("commit_sha=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
        assert!(log.contains("check_name=adl-ci"));
        assert!(log.contains("job_run_id=8801"));
        assert!(log.contains("pr_state=OPEN"));
        assert!(log.contains("is_draft=true"));
        assert!(log.contains("wait_reason=pr_draft"));
        assert!(log.contains("wait_reason=checks_not_reported"));
        assert!(log.contains("poll_count=3"));
        assert!(log.contains("next_poll_delay_ms=250"));

        unsafe {
            std::env::remove_var("ADL_OBSERVABILITY_STDERR");
            std::env::remove_var("ADL_OBSERVABILITY_LOG");
        }
    }

    #[test]
    fn pr_validation_status_query_retries_transient_transport_and_returns_success_snapshot() {
        let _guard = env_lock();
        let policy_env = clear_github_policy_env();
        let temp = unique_temp_dir("adl-pr-validation-status-retry");
        let log_path = temp.join("events.log");
        let (base_uri, server) = spawn_validation_status_transient_server();
        unsafe {
            std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
            std::env::set_var("GITHUB_TOKEN", "test-token");
            std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
            std::env::set_var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS", "2");
            std::env::set_var("ADL_OBSERVABILITY_STDERR", "0");
            std::env::set_var(
                "ADL_OBSERVABILITY_LOG",
                log_path.to_str().expect("log path utf8"),
            );
        }

        let snapshot = pr_validation_status_octocrab("owner/repo", "1159")
            .expect("validation status snapshot after retry");
        assert_eq!(snapshot.pr_number, 1159);
        assert_eq!(
            snapshot.commit_sha,
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        );
        assert_eq!(snapshot.checks.len(), 1);
        assert_eq!(snapshot.checks[0].name, "adl-ci");
        assert_eq!(snapshot.checks[0].status, "COMPLETED");
        assert_eq!(snapshot.checks[0].conclusion, "SUCCESS");
        assert_eq!(snapshot.checks[0].job_run_id, "8801");

        let log = fs::read_to_string(&log_path).expect("read validation status log");
        assert!(log.contains("stage=github_octocrab"));
        assert!(log.contains("operation=pr.validation.status"));
        assert!(log.contains("result=retry"));
        assert!(log.contains("result=completed"));
        let seen = server.join().expect("server join");
        assert_eq!(
            seen.len(),
            2,
            "unexpected validation status calls: {seen:#?}"
        );
        assert!(seen.iter().all(|call| call.starts_with("POST /graphql ")));

        unsafe {
            std::env::remove_var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS");
            std::env::remove_var("ADL_OBSERVABILITY_STDERR");
            std::env::remove_var("ADL_OBSERVABILITY_LOG");
        }
        restore_github_policy_env(policy_env);
    }

    #[test]
    fn octocrab_transport_covers_pr_and_issue_operations_against_mock_github() {
        let _guard = env_lock();
        let policy_env = clear_github_policy_env();
        let temp = unique_temp_dir("adl-octocrab-transport");
        let (base_uri, server) = spawn_octocrab_test_server(27);
        unsafe {
            std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
            std::env::set_var("GITHUB_TOKEN", "test-token");
            std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
        }

        assert_eq!(
            current_pr_url("owner/repo", "codex/3697-octocrab-operational-transport")
                .expect("current PR URL")
                .as_deref(),
            Some("https://github.com/owner/repo/pull/1159")
        );
        let wave = unresolved_milestone_pr_wave("owner/repo", "v0.91.5", "tools", None)
            .expect("open PR wave");
        assert_eq!(wave.len(), 1);
        assert_eq!(wave[0].number, 1160);
        assert_eq!(wave[0].queue.as_deref(), Some("tools"));

        let body = run_gh_capture(
            "pr.view.body",
            &[
                "pr",
                "view",
                "-R",
                "owner/repo",
                "1159",
                "--json",
                "body",
                "--jq",
                ".body",
            ],
        )
        .expect("PR body");
        assert!(body.contains("Closes #3697"));
        let closing = run_gh_capture(
            "pr.view.closing_issues",
            &[
                "pr",
                "view",
                "-R",
                "owner/repo",
                "1159",
                "--json",
                "closingIssuesReferences",
                "--jq",
                ".closingIssuesReferences[]?.number",
            ],
        )
        .expect("closing issues");
        assert_eq!(closing.lines().collect::<Vec<_>>(), vec!["3697", "3698"]);
        let base = run_gh_capture(
            "pr.view.base_ref.finish_existing",
            &[
                "pr",
                "view",
                "-R",
                "owner/repo",
                "1159",
                "--json",
                "baseRefName",
                "--jq",
                ".baseRefName",
            ],
        )
        .expect("base ref");
        assert_eq!(base, "main");

        let pr_body_file = temp.join("pr-body.md");
        fs::write(&pr_body_file, "New body\n\nCloses #3697\n").expect("write PR body");
        let created_pr = run_gh_capture(
            "pr.create.finish",
            &[
                "pr",
                "create",
                "-R",
                "owner/repo",
                "--title",
                "[v0.91.5][tools] New PR",
                "--head",
                "codex/3697-octocrab-operational-transport",
                "--base",
                "main",
                "--body-file",
                path_str(&pr_body_file).expect("body path"),
                "--draft",
            ],
        )
        .expect("create PR");
        assert_eq!(created_pr, "https://github.com/owner/repo/pull/1162");

        run_gh_status(
            "pr.edit.body_file",
            &[
                "pr",
                "edit",
                "-R",
                "owner/repo",
                "1159",
                "--body-file",
                path_str(&pr_body_file).expect("body path"),
            ],
        )
        .expect("edit PR body");
        run_gh_status(
            "pr.edit.finish_existing",
            &[
                "pr",
                "edit",
                "-R",
                "owner/repo",
                "1159",
                "--title",
                "[v0.91.5][tools] Updated PR",
                "--body-file",
                path_str(&pr_body_file).expect("body path"),
            ],
        )
        .expect("edit PR title/body");
        run_gh_status(
            "pr.ready.finish",
            &["pr", "ready", "-R", "owner/repo", "1159"],
        )
        .expect("mark ready");
        run_gh_status(
            "pr.merge.finish",
            &["pr", "merge", "-R", "owner/repo", "1159"],
        )
        .expect("merge PR");

        assert_eq!(
            gh_issue_create(
                "owner/repo",
                "[v0.91.5][tools] Created issue",
                "created",
                "version:v0.91.5"
            )
            .expect("issue create"),
            "https://github.com/owner/repo/issues/77"
        );
        assert_eq!(
            gh_issue_label_names(77, "owner/repo").expect("issue labels"),
            vec!["version:v0.91.5".to_string(), "area:tools".to_string()]
        );
        let listed = gh_issue_list("owner/repo", IssueStateFilter::All, 10).expect("issue list");
        assert_eq!(listed.len(), 2);
        assert_eq!(listed[0].number, 91);
        assert_eq!(listed[0].milestone.as_deref(), Some("v0.91.5"));
        assert_eq!(listed[1].number, 93);
        assert_eq!(
            gh_issue_search("owner/repo", "docs audit", IssueStateFilter::Open, 10)
                .expect("issue search")[0]
                .number,
            94
        );
        gh_issue_edit_title("owner/repo", 77, "[v0.91.5][tools] Updated issue")
            .expect("edit issue title");
        gh_issue_edit_body("owner/repo", 77, "updated body").expect("edit issue body");
        assert_eq!(
            gh_issue_title(77, "owner/repo")
                .expect("issue title")
                .as_deref(),
            Some("[v0.91.5][tools] Issue title")
        );
        assert_eq!(
            gh_issue_body(77, "owner/repo")
                .expect("issue body")
                .as_deref(),
            Some("issue body")
        );
        let viewed = gh_issue_view("owner/repo", 77).expect("issue view");
        assert_eq!(viewed.number, 77);
        assert_eq!(viewed.state, "closed");
        assert_eq!(viewed.body.as_deref(), Some("issue body"));
        assert!(viewed.labels.iter().any(|label| label == "area:tools"));
        let pr_like = gh_issue_view("owner/repo", 92).expect_err("pr-like issue view should fail");
        assert!(pr_like
            .to_string()
            .contains("GitHub returned a pull request instead of an issue"));
        assert!(gh_issue_is_closed_completed(77, "owner/repo").expect("issue state"));
        gh_issue_set_labels(
            "owner/repo",
            77,
            &[
                "version:v0.91.5".to_string(),
                "area:tools".to_string(),
                "type:task".to_string(),
            ],
        )
        .expect("set labels");
        let issue_comment_file = temp.join("issue-comment.md");
        fs::write(&issue_comment_file, "closeout line 1\n\ncloseout line 3\n")
            .expect("write issue comment");
        run_gh_status(
            "issue.comment",
            &[
                "issue",
                "comment",
                "77",
                "-R",
                "owner/repo",
                "--body-file",
                path_str(&issue_comment_file).expect("issue comment body path"),
            ],
        )
        .expect("issue comment");
        run_gh_status(
            "issue.close",
            &[
                "issue",
                "close",
                "77",
                "-R",
                "owner/repo",
                "--reason",
                "completed",
            ],
        )
        .expect("issue close completed");
        run_gh_status(
            "issue.close",
            &[
                "issue",
                "close",
                "77",
                "-R",
                "owner/repo",
                "--reason",
                "not_planned",
            ],
        )
        .expect("issue close not planned");

        let seen = server.join().expect("server join");
        assert_eq!(seen.len(), 27, "unexpected mock GitHub calls: {seen:#?}");
        assert!(seen
            .iter()
            .any(|call| call.starts_with("POST /repos/owner/repo/pulls ")));
        assert!(seen.iter().any(|call| call.contains("\"draft\":true")));
        assert!(seen
            .iter()
            .any(|call| call.starts_with("PUT /repos/owner/repo/pulls/1159/merge ")));
        assert!(seen
            .iter()
            .any(|call| call.contains("\"labels\":[\"version:v0.91.5\"")));
        assert!(seen.iter().any(|call| {
            call.starts_with("POST /repos/owner/repo/issues/77/comments ")
                && call.contains("closeout line 1\\n\\ncloseout line 3\\n")
        }));
        assert!(seen.iter().any(|call| {
            call.starts_with("PATCH /repos/owner/repo/issues/77 ")
                && call.contains("\"state\":\"closed\"")
                && call.contains("\"state_reason\":\"completed\"")
        }));
        assert!(seen.iter().any(|call| {
            call.starts_with("PATCH /repos/owner/repo/issues/77 ")
                && call.contains("\"state\":\"closed\"")
                && call.contains("\"state_reason\":\"not_planned\"")
        }));
        restore_github_policy_env(policy_env);
    }

    #[test]
    fn unresolved_wave_ignores_non_closing_stale_pr_residue() {
        let _guard = env_lock();
        let policy_env = clear_github_policy_env();
        let temp = unique_temp_dir("adl-open-wave-closing-filter");
        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let github_cli_fixture = bin_dir.join("gh");
        write_executable(
            &github_cli_fixture,
            r#"#!/usr/bin/env bash
set -euo pipefail
if [ "$1 $2" = 'pr list' ]; then
  cat <<'JSON'
[
  {
    "number": 2001,
    "title": "[v0.91.5][tools] Stale residue",
    "url": "https://github.com/owner/repo/pull/2001",
    "headRefName": "codex/stale-residue",
    "baseRefName": "main",
    "isDraft": true
  },
  {
    "number": 2002,
    "title": "[v0.91.5][tools] Real active blocker",
    "url": "https://github.com/owner/repo/pull/2002",
    "headRefName": "codex/real-active-blocker",
    "baseRefName": "main",
    "isDraft": true
  },
  {
    "number": 2003,
    "title": "[v0.91.5] Queue-less closing PR",
    "url": "https://github.com/owner/repo/pull/2003",
    "headRefName": "codex/queue-less-closing-pr",
    "baseRefName": "main",
    "isDraft": true
  }
]
JSON
  exit 0
fi
if [ "$1 $2" = 'pr view' ]; then
  if printf '%s ' "$@" | grep -q 'pull/2001'; then
    exit 0
  fi
  if printf '%s ' "$@" | grep -q 'pull/2002'; then
    printf '3790\n'
    exit 0
  fi
  if printf '%s ' "$@" | grep -q 'pull/2003'; then
    printf '3841\n'
    exit 0
  fi
fi
exit 1
"#,
        );

        let old_path = std::env::var("PATH").ok();
        let mut path_entries = vec![bin_dir.clone()];
        path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
        unsafe {
            std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", &github_cli_fixture);
            std::env::set_var(
                "PATH",
                std::env::join_paths(path_entries).expect("join PATH"),
            );
        }

        let wave = unresolved_milestone_pr_wave("owner/repo", "v0.91.5", "tools", None)
            .expect("open PR wave");
        assert_eq!(wave.len(), 1);
        assert_eq!(wave[0].number, 2002);
        assert_eq!(wave[0].queue.as_deref(), Some("tools"));

        restore_env("PATH", old_path);
        restore_github_policy_env(policy_env);
    }

    #[test]
    fn octocrab_transport_retries_transient_github_failures() {
        let _guard = env_lock();
        let policy_env = clear_github_policy_env();
        let (base_uri, server) = spawn_transient_octocrab_test_server();
        unsafe {
            std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
            std::env::set_var("GITHUB_TOKEN", "test-token");
            std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
            std::env::set_var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS", "2");
        }

        assert_eq!(
            gh_issue_title(88, "owner/repo")
                .expect("transient issue title")
                .as_deref(),
            Some("[v0.91.5][tools] Retry succeeds")
        );
        let seen = server.join().expect("server join");
        assert_eq!(
            seen,
            vec![
                "GET /repos/owner/repo/issues/88".to_string(),
                "GET /repos/owner/repo/issues/88".to_string()
            ]
        );
        unsafe {
            std::env::remove_var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS");
        }
        restore_github_policy_env(policy_env);
    }

    #[test]
    fn octocrab_transport_honors_quiet_stderr_compatibility_log() {
        let _guard = env_lock();
        let policy_env = clear_github_policy_env();
        let temp = unique_temp_dir("adl-octocrab-observability-log");
        let log_path = temp.join("events.log");
        let (base_uri, server) = spawn_transient_octocrab_test_server();
        unsafe {
            std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
            std::env::set_var("GITHUB_TOKEN", "test-token");
            std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
            std::env::set_var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS", "2");
            std::env::set_var("ADL_OBSERVABILITY_STDERR", "0");
            std::env::set_var(
                "ADL_OBSERVABILITY_LOG",
                log_path.to_str().expect("log path utf8"),
            );
        }

        assert_eq!(
            gh_issue_title(88, "owner/repo")
                .expect("transient issue title")
                .as_deref(),
            Some("[v0.91.5][tools] Retry succeeds")
        );

        let log = fs::read_to_string(&log_path).expect("read observability log");
        assert!(log.contains("stage=github_octocrab"));
        assert!(log.contains("result=started"));
        assert!(log.contains("result=retry"));
        assert!(log.contains("result=completed"));
        assert!(log.contains("operation=issue.view.title"));

        let seen = server.join().expect("server join");
        assert_eq!(
            seen,
            vec![
                "GET /repos/owner/repo/issues/88".to_string(),
                "GET /repos/owner/repo/issues/88".to_string()
            ]
        );
        unsafe {
            std::env::remove_var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS");
            std::env::remove_var("ADL_OBSERVABILITY_STDERR");
            std::env::remove_var("ADL_OBSERVABILITY_LOG");
        }
        restore_github_policy_env(policy_env);
    }

    #[test]
    fn pr_validation_status_query_paginates_status_rollup_contexts() {
        let _guard = env_lock();
        let policy_env = clear_github_policy_env();
        let (base_uri, server) = spawn_validation_status_paginated_server();
        unsafe {
            std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
            std::env::set_var("GITHUB_TOKEN", "test-token");
            std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
        }

        let snapshot =
            pr_validation_status_octocrab("owner/repo", "1159").expect("paginated snapshot");
        assert_eq!(snapshot.checks.len(), 2);
        assert_eq!(snapshot.checks[0].name, "adl-ci");
        assert_eq!(snapshot.checks[1].name, "adl-coverage");

        let seen = server.join().expect("server join");
        assert_eq!(seen.len(), 2, "unexpected pagination calls: {seen:#?}");
        assert!(seen[0].contains(r#""after":null"#));
        assert!(seen[1].contains(r#""after":"cursor-1""#));

        unsafe {
            std::env::remove_var("ADL_GITHUB_OCTOCRAB_BASE_URI");
        }
        restore_github_policy_env(policy_env);
    }

    #[test]
    fn pr_validation_watch_returns_failed_report_without_second_fetch() {
        let _guard = env_lock();
        let policy_env = clear_github_policy_env();
        let (base_uri, server) =
            spawn_validation_status_once_server("COMPLETED", Some("FAILURE"), "adl-coverage");
        unsafe {
            std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
            std::env::set_var("GITHUB_TOKEN", "test-token");
            std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
        }

        let report =
            wait_for_pr_validation_report("owner/repo", "1159").expect("failed report returned");
        assert_eq!(report.disposition, "failed");
        assert_eq!(report.failed_checks.len(), 1);
        assert_eq!(report.failed_checks[0].name, "adl-coverage");
        assert_eq!(report.checks.len(), 1);

        let seen = server.join().expect("server join");
        assert_eq!(
            seen.len(),
            1,
            "watch should not refetch after terminal state"
        );

        unsafe {
            std::env::remove_var("ADL_GITHUB_OCTOCRAB_BASE_URI");
        }
        restore_github_policy_env(policy_env);
    }

    #[test]
    fn octocrab_retry_policy_blocks_non_idempotent_mutations() {
        assert!(octocrab_operation_allows_retry("pr.list.current_branch"));
        assert!(octocrab_operation_allows_retry("pr.validation.status"));
        assert!(octocrab_operation_allows_retry("issue.view.title"));
        assert!(octocrab_operation_allows_retry("issue.close"));
        assert!(!octocrab_operation_allows_retry("issue.comment"));
        assert!(!octocrab_operation_allows_retry("issue.create"));
        assert!(!octocrab_operation_allows_retry("pr.create.finish"));
        assert!(!octocrab_operation_allows_retry("pr.merge.finish"));
    }

    #[test]
    fn closing_linkage_helpers_cover_reference_body_repair_and_error_paths() {
        let _guard = env_lock();
        let policy_env = clear_github_policy_env();
        let temp = unique_temp_dir("adl-github-closing-linkage");
        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let gh_log = temp.join("gh.log");
        let state_dir = temp.join("state");
        fs::create_dir_all(&state_dir).expect("state dir");

        let linked_ref = state_dir.join("linked_ref.txt");
        let linked_body = state_dir.join("linked_body.txt");
        let unlinked_ref = state_dir.join("unlinked_ref.txt");
        let unlinked_body = state_dir.join("unlinked_body.txt");
        let repair_ref = state_dir.join("repair_ref.txt");
        let repair_body = state_dir.join("repair_body.txt");
        fs::write(&linked_ref, "1153\n").expect("linked refs");
        fs::write(&linked_body, "Refs #1153\n").expect("linked body");
        fs::write(&unlinked_ref, "").expect("unlinked refs");
        fs::write(&unlinked_body, "Refs #9999\n").expect("unlinked body");
        fs::write(&repair_ref, "").expect("repair refs");
        fs::write(&repair_body, "Refs #1153\n").expect("repair body");

        let github_cli_fixture = bin_dir.join("github-cli-fixture");
        write_executable(
            &github_cli_fixture,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2\" = 'pr list' ]; then\n  printf 'https://github.com/owner/repo/pull/1159\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  pr_ref=''\n  for arg in \"$@\"; do\n    case \"$arg\" in\n      https://github.com/owner/repo/pull/1159|https://github.com/owner/repo/pull/1160|https://github.com/owner/repo/pull/1161)\n        pr_ref=\"$arg\"\n        ;;\n    esac\n  done\n  case \"$pr_ref\" in\n    https://github.com/owner/repo/pull/1159)\n      refs='{}'\n      body='{}'\n      ;;\n    https://github.com/owner/repo/pull/1160)\n      refs='{}'\n      body='{}'\n      ;;\n    https://github.com/owner/repo/pull/1161)\n      refs='{}'\n      body='{}'\n      ;;\n    *)\n      exit 13\n      ;;\n  esac\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    cat \"$refs\"\n    exit 0\n  fi\n  if printf '%s ' \"$@\" | grep -q ' --json body '; then\n    cat \"$body\"\n    exit 0\n  fi\n  exit 14\nfi\nif [ \"$1 $2\" = 'pr edit' ]; then\n  pr_ref=''\n  body_file=''\n  while [ $# -gt 0 ]; do\n    case \"$1\" in\n      https://github.com/owner/repo/pull/1161)\n        pr_ref=\"$1\"\n        shift\n        ;;\n      --body-file)\n        body_file=\"$2\"\n        shift 2\n        ;;\n      *)\n        shift\n        ;;\n    esac\n  done\n  [ \"$pr_ref\" = 'https://github.com/owner/repo/pull/1161' ] || exit 15\n  cp \"$body_file\" '{}'\n  printf '1153\\n' > '{}'\n  exit 0\nfi\nexit 16\n",
                gh_log.display(),
                linked_ref.display(),
                linked_body.display(),
                unlinked_ref.display(),
                unlinked_body.display(),
                repair_ref.display(),
                repair_body.display(),
                repair_body.display(),
                repair_ref.display()
            ),
        );

        let desired_body = temp.join("desired.md");
        fs::write(&desired_body, "Closes #1153\n\n## Summary\nrepaired\n").expect("desired body");

        let old_path = std::env::var("PATH").ok();
        let mut path_entries = vec![bin_dir.clone()];
        path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
        unsafe {
            std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", &github_cli_fixture);
            std::env::set_var(
                "PATH",
                std::env::join_paths(path_entries).expect("join PATH"),
            );
        }

        assert_eq!(
            current_pr_url("owner/repo", "codex/1153-branch")
                .expect("current pr")
                .as_deref(),
            Some("https://github.com/owner/repo/pull/1159")
        );
        assert!(pr_has_closing_linkage(
            "owner/repo",
            "https://github.com/owner/repo/pull/1159",
            1153
        )
        .expect("linked ref"));
        assert!(!pr_has_closing_linkage(
            "owner/repo",
            "https://github.com/owner/repo/pull/1160",
            1153
        )
        .expect("unlinked"));
        ensure_pr_closing_linkage(
            "owner/repo",
            "https://github.com/owner/repo/pull/1159",
            1153,
            true,
        )
        .expect("no-close skip");
        let err = ensure_pr_closing_linkage(
            "owner/repo",
            "https://github.com/owner/repo/pull/1160",
            1153,
            false,
        )
        .expect_err("missing linkage should fail");
        assert!(err
            .to_string()
            .contains("missing closing linkage to issue #1153"));

        let repaired = ensure_or_repair_pr_closing_linkage(
            "owner/repo",
            "https://github.com/owner/repo/pull/1161",
            1153,
            false,
            &desired_body,
        )
        .expect("repair should succeed");
        assert!(repaired);
        assert!(pr_has_closing_linkage(
            "owner/repo",
            "https://github.com/owner/repo/pull/1161",
            1153
        )
        .expect("linked after repair"));

        restore_env("PATH", old_path);

        let gh_calls = fs::read_to_string(&gh_log).expect("gh log");
        assert!(gh_calls
            .contains("pr edit -R owner/repo https://github.com/owner/repo/pull/1161 --body-file"));
        restore_github_policy_env(policy_env);
    }

    #[test]
    fn helper_attach_commands_cover_disabled_success_failure_and_fallback_paths() {
        let _guard = env_lock();
        let temp = unique_temp_dir("adl-github-attach-helpers");
        let repo = temp.join("repo");
        let tools_dir = repo.join("adl/tools");
        fs::create_dir_all(&tools_dir).expect("repo tools");

        let janitor_success = temp.join("janitor-success.log");
        let closeout_success = temp.join("closeout-success.log");

        write_executable(
            &tools_dir.join("attach_pr_janitor.sh"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\n",
                janitor_success.display()
            ),
        );
        write_executable(
            &tools_dir.join("attach_post_merge_closeout.sh"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\n",
                closeout_success.display()
            ),
        );
        let failing = temp.join("failing-helper.sh");
        write_executable(
            &failing,
            "#!/usr/bin/env bash\nset -euo pipefail\necho 'helper stdout'\necho 'helper stderr' >&2\nexit 9\n",
        );

        let old_janitor_disable = std::env::var("ADL_PR_JANITOR_DISABLE").ok();
        let old_janitor_cmd = std::env::var("ADL_PR_JANITOR_CMD").ok();
        let old_closeout_disable = std::env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
        let old_closeout_cmd = std::env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();

        unsafe {
            std::env::set_var("ADL_PR_JANITOR_DISABLE", "1");
            std::env::remove_var("ADL_PR_JANITOR_CMD");
        }
        attach_pr_janitor(
            &repo,
            "owner/repo",
            1153,
            "codex/1153-branch",
            "https://github.com/owner/repo/pull/1159",
            "draft",
        )
        .expect("disabled janitor should skip");

        unsafe {
            std::env::set_var("ADL_PR_JANITOR_DISABLE", "0");
            std::env::remove_var("ADL_PR_JANITOR_CMD");
        }
        attach_pr_janitor(
            &repo,
            "owner/repo",
            1153,
            "codex/1153-branch",
            "https://github.com/owner/repo/pull/1159",
            "draft",
        )
        .expect("repo helper janitor");

        unsafe {
            std::env::set_var("ADL_PR_JANITOR_CMD", "   ");
        }
        attach_pr_janitor(
            &repo,
            "owner/repo",
            1153,
            "codex/1153-branch",
            "https://github.com/owner/repo/pull/1159",
            "ready",
        )
        .expect("blank override janitor fallback");

        unsafe {
            std::env::set_var("ADL_PR_JANITOR_CMD", &failing);
        }
        let err = attach_pr_janitor(
            &repo,
            "owner/repo",
            1153,
            "codex/1153-branch",
            "https://github.com/owner/repo/pull/1159",
            "draft",
        )
        .expect_err("failing janitor should bubble");
        assert!(err.to_string().contains("PR janitor auto-attach failed"));
        assert!(err.to_string().contains("helper stderr"));
        assert!(err.to_string().contains("stdout: helper stdout"));

        unsafe {
            std::env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "1");
            std::env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
        }
        attach_post_merge_closeout(
            &repo,
            "owner/repo",
            1153,
            "codex/1153-branch",
            "https://github.com/owner/repo/pull/1159",
        )
        .expect("disabled closeout should skip");

        unsafe {
            std::env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
            std::env::set_var(
                "ADL_POST_MERGE_CLOSEOUT_CMD",
                tools_dir.join("attach_post_merge_closeout.sh"),
            );
        }
        attach_post_merge_closeout(
            &repo,
            "owner/repo",
            1153,
            "codex/1153-branch",
            "https://github.com/owner/repo/pull/1159",
        )
        .expect("repo helper closeout");

        unsafe {
            std::env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", "   ");
        }
        attach_post_merge_closeout(
            &repo,
            "owner/repo",
            1153,
            "codex/1153-branch",
            "https://github.com/owner/repo/pull/1159",
        )
        .expect("blank override closeout skip");

        unsafe {
            std::env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", &failing);
        }
        let err = attach_post_merge_closeout(
            &repo,
            "owner/repo",
            1153,
            "codex/1153-branch",
            "https://github.com/owner/repo/pull/1159",
        )
        .expect_err("failing closeout should bubble");
        assert!(err
            .to_string()
            .contains("post-merge closeout auto-attach failed"));
        assert!(err.to_string().contains("helper stderr"));
        assert!(err.to_string().contains("stdout: helper stdout"));

        restore_env("ADL_PR_JANITOR_DISABLE", old_janitor_disable);
        restore_env("ADL_PR_JANITOR_CMD", old_janitor_cmd);
        restore_env("ADL_POST_MERGE_CLOSEOUT_DISABLE", old_closeout_disable);
        restore_env("ADL_POST_MERGE_CLOSEOUT_CMD", old_closeout_cmd);

        let janitor_calls = fs::read_to_string(&janitor_success).expect("janitor success log");
        assert!(janitor_calls.contains("--expected-pr-state draft"));
        assert!(janitor_calls.contains("--expected-pr-state ready"));
        assert!(fs::read_to_string(&closeout_success)
            .expect("closeout success log")
            .contains("--pr-url https://github.com/owner/repo/pull/1159"));
    }

    #[test]
    fn github_helpers_cover_fallback_and_spawn_failure_paths() {
        let _guard = env_lock();
        let policy_env = clear_github_policy_env();
        let temp = unique_temp_dir("adl-github-helper-fallbacks");
        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let body_ref = temp.join("body-ref.txt");
        let body_text = temp.join("body.txt");
        fs::write(&body_ref, "").expect("empty refs");
        fs::write(&body_text, "Closes #1153\n").expect("body text");
        let github_cli_fixture = bin_dir.join("github-cli-fixture");
        write_executable(
            &github_cli_fixture,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    cat '{}'\n    exit 0\n  fi\n  if printf '%s ' \"$@\" | grep -q ' --json body '; then\n    cat '{}'\n    exit 0\n  fi\nfi\nif [ \"$1 $2\" = 'issue view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'labels'; then\n    printf 'track:roadmap\\n'\n  else\n    printf 'Tracking issue without version\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
                body_ref.display(),
                body_text.display()
            ),
        );

        let old_path = std::env::var("PATH").ok();
        let mut path_entries = vec![bin_dir.clone()];
        path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
        unsafe {
            std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", &github_cli_fixture);
            std::env::set_var(
                "PATH",
                std::env::join_paths(path_entries).expect("join PATH"),
            );
        }

        assert!(pr_has_closing_linkage(
            "owner/repo",
            "https://github.com/owner/repo/pull/1159",
            1153
        )
        .expect("body fallback should count"));
        assert_eq!(
            issue_version(1153, "owner/repo").expect("no inferred version"),
            None
        );

        restore_env("PATH", old_path);

        let missing = temp.join("missing-helper.sh");
        unsafe {
            std::env::set_var("ADL_PR_JANITOR_CMD", &missing);
            std::env::set_var("ADL_PR_JANITOR_DISABLE", "0");
        }
        let err = attach_pr_janitor(
            temp.as_path(),
            "owner/repo",
            1153,
            "codex/1153-branch",
            "https://github.com/owner/repo/pull/1159",
            "draft",
        )
        .expect_err("missing janitor helper should surface spawn failure");
        assert!(err
            .to_string()
            .contains("failed to spawn PR janitor command"));

        unsafe {
            std::env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", &missing);
            std::env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
        }
        let err = attach_post_merge_closeout(
            temp.as_path(),
            "owner/repo",
            1153,
            "codex/1153-branch",
            "https://github.com/owner/repo/pull/1159",
        )
        .expect_err("missing closeout helper should surface spawn failure");
        assert!(err
            .to_string()
            .contains("failed to spawn post-merge closeout command"));

        unsafe {
            std::env::remove_var("ADL_PR_JANITOR_CMD");
            std::env::remove_var("ADL_PR_JANITOR_DISABLE");
            std::env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
            std::env::remove_var("ADL_POST_MERGE_CLOSEOUT_DISABLE");
        }
        restore_github_policy_env(policy_env);
    }

    #[test]
    fn issue_metadata_helpers_preserve_create_body_title_and_label_parity() {
        let _guard = env_lock();
        let policy_env = clear_github_policy_env();
        let temp = unique_temp_dir("adl-github-issue-metadata");
        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let title_file = temp.join("title.txt");
        let labels_file = temp.join("labels.txt");
        let body_file = temp.join("body.md");
        let log_file = temp.join("gh.log");
        fs::write(&title_file, "[v0.91.4][tools] Old title\n").expect("title");
        fs::write(&labels_file, "track:roadmap\nversion:v0.91.4\n").expect("labels");

        let github_cli_fixture = bin_dir.join("github-cli-fixture");
        write_executable(
            &github_cli_fixture,
            &format!(
                r#"#!/usr/bin/env python3
import pathlib
import shutil
import sys

title = pathlib.Path({title:?})
labels = pathlib.Path({labels:?})
body = pathlib.Path({body:?})
log = pathlib.Path({log:?})
args = sys.argv[1:]
with log.open("a", encoding="utf-8") as fh:
    fh.write(repr(args) + "\n")

if args[:2] == ["issue", "create"]:
    print("https://github.com/owner/repo/issues/77")
    sys.exit(0)

if args[:2] == ["issue", "view"]:
    if "labels" in args:
        print(labels.read_text(encoding="utf-8"), end="")
        sys.exit(0)
    if "title" in args:
        print(title.read_text(encoding="utf-8"), end="")
        sys.exit(0)
    sys.exit(2)

if args[:2] == ["issue", "edit"]:
    current_labels = [
        line.strip()
        for line in labels.read_text(encoding="utf-8").splitlines()
        if line.strip()
    ]
    i = 2
    while i < len(args):
        if args[i] == "--title":
            title.write_text(args[i + 1] + "\n", encoding="utf-8")
            i += 2
        elif args[i] == "--add-label":
            requested_labels = [
                label.strip()
                for label in args[i + 1].split(",")
                if label.strip()
            ]
            if "," in args[i + 1]:
                current_labels = []
            for label in requested_labels:
                label = label.strip()
                if label and label not in current_labels:
                    current_labels.append(label)
            i += 2
        elif args[i] == "--remove-label":
            current_labels = [label for label in current_labels if label != args[i + 1]]
            i += 2
        elif args[i] == "--body":
            body.write_text(args[i + 1], encoding="utf-8")
            i += 2
        elif args[i] == "--body-file":
            shutil.copyfile(args[i + 1], body)
            i += 2
        else:
            i += 1
    labels.write_text("\n".join(current_labels) + "\n", encoding="utf-8")
    sys.exit(0)

sys.exit(9)
"#,
                title = title_file.display().to_string(),
                labels = labels_file.display().to_string(),
                body = body_file.display().to_string(),
                log = log_file.display().to_string(),
            ),
        );

        let old_path = std::env::var("PATH").ok();
        let mut path_entries = vec![bin_dir.clone()];
        path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
        unsafe {
            std::env::set_var("ADL_TEST_GITHUB_CLI_FIXTURE", &github_cli_fixture);
            std::env::set_var(
                "PATH",
                std::env::join_paths(path_entries).expect("join PATH"),
            );
        }

        let created = gh_issue_create(
            "owner/repo",
            "[v0.91.5][tools] New title",
            "issue body",
            " version:v0.91.5, area:tools,,type:task ",
        )
        .expect("create issue");
        assert_eq!(created, "https://github.com/owner/repo/issues/77");

        gh_issue_edit_body("owner/repo", 77, "updated body").expect("edit body");
        assert_eq!(
            fs::read_to_string(&body_file).expect("body file"),
            "updated body"
        );

        ensure_issue_metadata_parity(
            "owner/repo",
            77,
            "[v0.91.5][tools] New title",
            "track:roadmap,area:tools,version:v0.91.5",
        )
        .expect("metadata parity");

        assert_eq!(
            fs::read_to_string(&title_file).expect("title"),
            "[v0.91.5][tools] New title\n"
        );
        assert_eq!(
            fs::read_to_string(&labels_file).expect("labels"),
            "area:tools\ntrack:roadmap\nversion:v0.91.5\n"
        );

        restore_env("PATH", old_path);

        let calls = fs::read_to_string(&log_file).expect("gh log");
        assert!(calls.contains("'--label', ' version:v0.91.5, area:tools,,type:task '"));
        assert!(calls.contains("'--title', '[v0.91.5][tools] New title'"));
        assert!(calls.contains("'--add-label', 'area:tools,track:roadmap,version:v0.91.5'"));
        restore_github_policy_env(policy_env);
    }

    #[test]
    fn live_gh_policy_guard_blocks_disabled_fallback_before_spawn() {
        let _guard = env_lock();
        let policy_env = clear_github_policy_env();
        let temp = unique_temp_dir("adl-github-disabled-fallback");
        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let gh_log = temp.join("gh.log");
        write_executable(
            &bin_dir.join("gh"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nprintf 'unexpected gh spawn\\n'\n",
                gh_log.display()
            ),
        );
        let old_path = std::env::var("PATH").ok();
        let mut path_entries = vec![bin_dir.clone()];
        path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
        unsafe {
            std::env::set_var(
                "PATH",
                std::env::join_paths(path_entries).expect("join PATH"),
            );
            std::env::set_var("ADL_GITHUB_DISABLE_GH_FALLBACK", "1");
        }

        let err = current_pr_url("owner/repo", "codex/3672-branch")
            .expect_err("fallback-disabled current_pr_url should fail closed");
        let err_debug = format!("{err:?}");
        assert!(err_debug.contains("pr.list.current_branch"));
        assert!(err_debug.contains("github_client.fallback_disabled"));
        let err = gh_issue_edit_body("owner/repo", 3672, "body")
            .expect_err("fallback-disabled issue edit should fail closed");
        let err_debug = format!("{err:?}");
        assert!(err_debug.contains("issue.edit.body"));
        assert!(err_debug.contains("github_client.fallback_disabled"));
        assert!(
            !gh_log.exists(),
            "policy guard should reject before spawning gh"
        );

        restore_env("PATH", old_path);
        restore_github_policy_env(policy_env);
    }

    #[test]
    fn live_github_policy_blocks_explicit_gh_fallback_before_spawn() {
        let _guard = env_lock();
        let policy_env = clear_github_policy_env();
        let temp = unique_temp_dir("adl-github-explicit-gh-fallback");
        let bin_dir = temp.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir");
        let gh_log = temp.join("gh.log");
        write_executable(
            &bin_dir.join("gh"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nprintf 'unexpected gh spawn\\n'\n",
                gh_log.display()
            ),
        );
        let old_path = std::env::var("PATH").ok();
        let mut path_entries = vec![bin_dir.clone()];
        path_entries.extend(std::env::split_paths(old_path.as_deref().unwrap_or("")));
        unsafe {
            std::env::set_var(
                "PATH",
                std::env::join_paths(path_entries).expect("join PATH"),
            );
            std::env::set_var("ADL_GITHUB_CLIENT", "gh");
            std::env::set_var("GITHUB_TOKEN", "test-token");
        }

        let err = current_pr_url("owner/repo", "codex/3672-branch")
            .expect_err("explicit gh fallback current_pr_url should fail closed");
        let err_debug = format!("{err:?}");
        assert!(err_debug.contains("pr.list.current_branch"));
        assert!(err_debug.contains("github_client.gh_fallback_removed"));
        assert!(err_debug.contains("credential_status=token_present"));
        assert!(err_debug.contains("read or mutation fallback"));
        assert!(err_debug.contains("source=GITHUB_TOKEN"));
        assert!(!err_debug.contains("test-token"));
        let err = gh_issue_edit_body("owner/repo", 3672, "body")
            .expect_err("explicit gh fallback issue edit should fail closed");
        let err_debug = format!("{err:?}");
        assert!(err_debug.contains("github_client.gh_fallback_removed"));
        assert!(err_debug.contains("credential_status=token_present"));
        assert!(err_debug.contains("read or mutation fallback"));
        assert!(!err_debug.contains("test-token"));
        assert!(
            !gh_log.exists(),
            "fallback removal should reject before spawning gh"
        );

        restore_env("PATH", old_path);
        restore_github_policy_env(policy_env);
    }

    #[test]
    fn live_github_policy_explains_missing_token_before_spawn() {
        let _guard = env_lock();
        let policy_env = clear_github_policy_env();
        unsafe {
            std::env::set_var("ADL_GITHUB_CLIENT", "gh");
        }

        let err = current_pr_url("owner/repo", "codex/3805-branch")
            .expect_err("explicit gh fallback without token should explain credential preflight");
        let err_debug = format!("{err:?}");
        assert!(err_debug.contains("github_client.gh_fallback_removed"));
        assert!(err_debug.contains("credential_status=missing_token"));
        assert!(err_debug.contains("set GITHUB_TOKEN or GH_TOKEN"));
        assert!(err_debug.contains("operator-approved secret source"));
        assert!(err_debug.contains("do not fall back to direct gh commands"));
        assert!(err_debug.contains("credential values are never printed"));

        restore_github_policy_env(policy_env);
    }
}
