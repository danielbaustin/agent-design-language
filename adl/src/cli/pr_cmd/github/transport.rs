use super::{
    block_on_octocrab, parse_pr_number, parse_repo, pr_validation_projection_status, with_octocrab,
    OpenPullRequest, PrValidationCheckReport, PrValidationReport,
};
#[cfg(test)]
use super::{run_gh_status_shell, test_gh_fixture_fallback_allowed};
use crate::cli::observability;
use ::adl::control_plane::sanitize_slug;
use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

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
pub(super) struct PrValidationCheckSnapshot {
    pub(super) name: String,
    pub(super) status: String,
    pub(super) conclusion: String,
    pub(super) job_run_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PrValidationSnapshot {
    pub(super) pr_number: u64,
    pub(super) commit_sha: String,
    pub(super) state: String,
    pub(super) is_draft: bool,
    pub(super) checks: Vec<PrValidationCheckSnapshot>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PrValidationDisposition {
    Pending,
    Success,
    Failed,
    Cancelled,
    Skipped,
    TimedOut,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ToolingAnomalyPacket {
    schema: String,
    anomaly_type: String,
    command_class: String,
    repo: String,
    pr_number: u64,
    linked_issue_numbers: Vec<u32>,
    commit_sha: String,
    pr_state: String,
    is_draft: bool,
    disposition: String,
    wait_reason: String,
    elapsed_ms: u128,
    poll_count: usize,
    observation_count: usize,
    observed_check_names: Vec<String>,
    pending_checks: Vec<PrValidationCheckReport>,
    failed_checks: Vec<PrValidationCheckReport>,
    checks: Vec<PrValidationCheckReport>,
    report_relpath: String,
    issue_body_relpath: String,
    suggested_issue_title: String,
    remediation_route: String,
    first_observed_at_epoch_ms: u128,
    last_observed_at_epoch_ms: u128,
}

pub(super) fn current_pr_url_octocrab(repo: &str, branch: &str) -> Result<Option<String>> {
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

pub(super) fn list_prs_octocrab(repo: &str) -> Result<Vec<OpenPullRequest>> {
    let repo_parts = parse_repo(repo)?;
    with_octocrab("pr.list.wave", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let mut page = block_on_octocrab(runtime, "pr.list.wave", || async {
            octo.pulls(&owner, &name)
                .list()
                .state(octocrab::params::State::Open)
                .per_page(100)
                .send()
                .await
        })?;
        let mut prs = Vec::new();
        let mut seen_next_pages = HashSet::new();
        loop {
            prs.extend(page.items.into_iter().map(|pr| {
                OpenPullRequest {
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
                    state: pr
                        .state
                        .map(|state| format!("{state:?}").to_uppercase())
                        .unwrap_or_else(|| "OPEN".to_string()),
                    queue: None,
                }
            }));
            let Some(next) = page.next.clone() else {
                break;
            };
            let next_url = next.to_string();
            if !seen_next_pages.insert(next_url.clone()) {
                observability::emit_event(
                    "adl",
                    "github_octocrab",
                    "failed",
                    &[
                        ("operation", "pr.list.wave"),
                        ("reason", "pagination_repeated_next"),
                    ],
                );
                return Err(anyhow!(
                    "github_client.pagination_loop: operation 'pr.list.wave' received repeated next-page URL '{}' after page {}",
                    next_url,
                    seen_next_pages.len() + 1
                ));
            }
            page = block_on_octocrab(runtime, "pr.list.wave", || async {
                octo.get_page::<octocrab::models::pulls::PullRequest>(&Some(next.clone()))
                    .await
            })?
            .ok_or_else(|| anyhow!("GitHub advertised a next PR page but did not return it"))?;
        }
        Ok(prs)
    })
}

pub(super) fn list_prs_by_head_ref_octocrab(
    repo: &str,
    head_ref_name: &str,
) -> Result<Vec<OpenPullRequest>> {
    let repo_parts = parse_repo(repo)?;
    let head = format!("{}:{head_ref_name}", repo_parts.owner);
    with_octocrab("pr.list.head_ref", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let page = block_on_octocrab(runtime, "pr.list.head_ref", || async {
            octo.pulls(&owner, &name)
                .list()
                .state(octocrab::params::State::All)
                .head(head.clone())
                .per_page(20)
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
                state: pr
                    .state
                    .map(|state| format!("{state:?}").to_uppercase())
                    .unwrap_or_else(|| "OPEN".to_string()),
                queue: None,
            })
            .collect())
    })
}

pub(super) fn pr_metadata_octocrab(repo: &str, pr_ref: &str) -> Result<OpenPullRequest> {
    let repo_parts = parse_repo(repo)?;
    let number = parse_pr_number(pr_ref)? as u64;
    with_octocrab("pr.view.metadata", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let pr = block_on_octocrab(runtime, "pr.view.metadata", || async {
            octo.pulls(&owner, &name).get(number).await
        })?;
        Ok(OpenPullRequest {
            number: pr.number.unwrap_or(number) as u32,
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
            state: pr
                .state
                .map(|state| format!("{state:?}").to_uppercase())
                .unwrap_or_else(|| "OPEN".to_string()),
            queue: None,
        })
    })
}

pub(super) fn pr_body_octocrab(repo: &str, pr_ref: &str) -> Result<String> {
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

pub(super) fn issue_comment_octocrab(repo: &str, issue: u32, body: &str) -> Result<()> {
    #[derive(Serialize)]
    struct IssueCommentPayload<'a> {
        body: &'a str,
    }

    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.comment", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let route = format!("/repos/{owner}/{name}/issues/{issue}/comments");
        let payload = IssueCommentPayload { body };
        let _: serde_json::Value = block_on_octocrab(runtime, "issue.comment", || async {
            octo.post(route.as_str(), Some(&payload)).await
        })?;
        Ok(())
    })
    .with_context(|| format!("github_client.octocrab_transport: issue comment failed for #{issue}"))
}

pub(super) fn issue_close_octocrab(
    repo: &str,
    issue: u32,
    reason: octocrab::models::issues::IssueStateReason,
) -> Result<()> {
    #[derive(Serialize)]
    struct IssueClosePayload<'a> {
        state: &'a str,
        state_reason: &'a str,
    }

    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.close", |runtime, octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let route = format!("/repos/{owner}/{name}/issues/{issue}");
        let state_reason = match reason {
            octocrab::models::issues::IssueStateReason::Completed => "completed",
            octocrab::models::issues::IssueStateReason::NotPlanned => "not_planned",
            _ => "completed",
        };
        let payload = IssueClosePayload {
            state: "closed",
            state_reason,
        };
        block_on_octocrab(runtime, "issue.close", || async {
            let _: serde_json::Value = octo.patch(route.as_str(), Some(&payload)).await?;
            Ok(())
        })?;
        Ok(())
    })
    .with_context(|| format!("github_client.octocrab_transport: issue close failed for #{issue}"))
}

pub(super) fn pr_closing_issue_numbers_octocrab(repo: &str, pr_ref: &str) -> Result<Vec<u32>> {
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

pub(super) fn pr_base_ref_octocrab(repo: &str, pr_ref: &str) -> Result<String> {
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

pub(super) fn create_pr_octocrab(
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

pub(super) fn update_pr_body_octocrab(repo: &str, pr_ref: &str, body: &str) -> Result<()> {
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

pub(super) fn update_pr_title_body_octocrab(
    repo: &str,
    pr_ref: &str,
    title: &str,
    body: &str,
) -> Result<()> {
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

pub(super) fn mark_pr_ready_octocrab(repo: &str, pr_ref: &str) -> Result<()> {
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

pub(super) fn merge_pr_octocrab(repo: &str, pr_ref: &str) -> Result<()> {
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
        let wait_terminal = pr_validation_wait_disposition_is_terminal(&snapshot, disposition);
        let next_delay = if wait_terminal {
            Duration::ZERO
        } else {
            poll_delay
        };
        emit_pr_validation_wait_snapshot(&snapshot, disposition, started, poll_count, next_delay);
        capture_pr_validation_wait_anomaly_best_effort(
            repo,
            &snapshot,
            disposition,
            started,
            poll_count,
        );

        if wait_terminal {
            return Ok(pr_validation_report_from_snapshot_with_disposition(
                &snapshot,
                disposition,
            ));
        }

        if started.elapsed() >= timeout {
            emit_pr_validation_wait_timeout(&snapshot, started, poll_count, Duration::ZERO);
            capture_pr_validation_wait_anomaly_best_effort(
                repo,
                &snapshot,
                PrValidationDisposition::TimedOut,
                started,
                poll_count,
            );
            return Ok(pr_validation_report_from_snapshot_with_disposition(
                &snapshot,
                PrValidationDisposition::TimedOut,
            ));
        }
        std::thread::sleep(poll_delay);
    }
}

pub(super) fn pr_validation_report(repo: &str, pr_ref: &str) -> Result<PrValidationReport> {
    let snapshot = pr_validation_status_octocrab(repo, pr_ref)?;
    Ok(pr_validation_report_from_snapshot_with_disposition(
        &snapshot,
        classify_pr_validation_snapshot(&snapshot),
    ))
}

pub(super) fn pr_validation_status_octocrab(
    repo: &str,
    pr_ref: &str,
) -> Result<PrValidationSnapshot> {
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

pub(super) fn pr_ready_with_optional_fixture_fallback(
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

fn capture_pr_validation_wait_anomaly(
    repo: &str,
    snapshot: &PrValidationSnapshot,
    disposition: PrValidationDisposition,
    started: Instant,
    poll_count: usize,
) -> Result<()> {
    if !tooling_anomaly_reporting_enabled() {
        return Ok(());
    }

    let elapsed = started.elapsed();
    let threshold = pr_validation_anomaly_threshold();
    let anomaly_type = match disposition {
        PrValidationDisposition::TimedOut => Some("pr_validation_timeout"),
        PrValidationDisposition::Pending if !snapshot.is_draft && elapsed >= threshold => {
            Some("pr_validation_slow")
        }
        PrValidationDisposition::Success
        | PrValidationDisposition::Failed
        | PrValidationDisposition::Cancelled
        | PrValidationDisposition::Skipped
            if elapsed >= threshold =>
        {
            Some("pr_validation_slow")
        }
        _ => None,
    };
    let Some(anomaly_type) = anomaly_type else {
        return Ok(());
    };

    let report = pr_validation_report_from_snapshot_with_disposition(snapshot, disposition);
    let wait_reason = pr_validation_wait_reason(snapshot, disposition, "aggregate").to_string();
    let linked_issue_numbers =
        pr_closing_issue_numbers_octocrab(repo, &snapshot.pr_number.to_string())
            .unwrap_or_default();
    let repo_root = tooling_anomaly_checkout_root()?;
    let report_dir = tooling_anomaly_report_dir(&repo_root)?;
    fs::create_dir_all(&report_dir)
        .with_context(|| format!("create tooling anomaly report dir {}", report_dir.display()))?;

    let fingerprint = pr_validation_anomaly_fingerprint(
        anomaly_type,
        repo,
        snapshot,
        &report.checks,
        &wait_reason,
    );
    let report_path = report_dir.join(format!("{fingerprint}.json"));
    let issue_body_path = report_dir.join(format!("{fingerprint}.md"));
    let now_ms = unix_time_ms();
    let report_relpath = repo_relative_display(&repo_root, &report_path);
    let issue_body_relpath = repo_relative_display(&repo_root, &issue_body_path);

    let mut packet = read_existing_tooling_anomaly_packet(&report_path)?.unwrap_or_else(|| {
        ToolingAnomalyPacket {
            schema: "adl.tooling_anomaly.v1".to_string(),
            anomaly_type: anomaly_type.to_string(),
            command_class: "pr.validation.wait".to_string(),
            repo: repo.to_string(),
            pr_number: snapshot.pr_number,
            linked_issue_numbers: linked_issue_numbers.clone(),
            commit_sha: snapshot.commit_sha.clone(),
            pr_state: snapshot.state.clone(),
            is_draft: snapshot.is_draft,
            disposition: disposition.as_event_result().to_string(),
            wait_reason: wait_reason.clone(),
            elapsed_ms: elapsed.as_millis(),
            poll_count,
            observation_count: 0,
            observed_check_names: Vec::new(),
            pending_checks: Vec::new(),
            failed_checks: Vec::new(),
            checks: Vec::new(),
            report_relpath: report_relpath.clone(),
            issue_body_relpath: issue_body_relpath.clone(),
            suggested_issue_title: format!(
                "[tools][observed-bug] {} on PR #{}",
                anomaly_type.replace('_', "-"),
                snapshot.pr_number
            ),
            remediation_route: format!(
                "Review {report_relpath} and create or update a follow-on with a safe body file, for example: adl pr issue create --title '<title>' --body-file {issue_body_relpath}"
            ),
            first_observed_at_epoch_ms: now_ms,
            last_observed_at_epoch_ms: now_ms,
        }
    });

    packet.linked_issue_numbers = linked_issue_numbers;
    packet.commit_sha = snapshot.commit_sha.clone();
    packet.pr_state = snapshot.state.clone();
    packet.is_draft = snapshot.is_draft;
    packet.disposition = disposition.as_event_result().to_string();
    packet.wait_reason = wait_reason;
    packet.elapsed_ms = elapsed.as_millis();
    packet.poll_count = poll_count;
    packet.observation_count += 1;
    packet.pending_checks = report.pending_checks.clone();
    packet.failed_checks = report.failed_checks.clone();
    packet.checks = report.checks.clone();
    packet.observed_check_names = packet
        .checks
        .iter()
        .map(|check| check.name.clone())
        .collect();
    packet.last_observed_at_epoch_ms = now_ms;

    let issue_body = render_pr_validation_anomaly_issue_body(&packet);
    fs::write(&issue_body_path, issue_body).with_context(|| {
        format!(
            "write tooling anomaly issue body {}",
            issue_body_path.display()
        )
    })?;
    fs::write(&report_path, serde_json::to_string_pretty(&packet)?)
        .with_context(|| format!("write tooling anomaly packet {}", report_path.display()))?;
    Ok(())
}

fn capture_pr_validation_wait_anomaly_best_effort(
    repo: &str,
    snapshot: &PrValidationSnapshot,
    disposition: PrValidationDisposition,
    started: Instant,
    poll_count: usize,
) {
    if let Err(err) =
        capture_pr_validation_wait_anomaly(repo, snapshot, disposition, started, poll_count)
    {
        emit_pr_validation_wait_anomaly_capture_failure(repo, snapshot, disposition, &err);
    }
}

fn emit_pr_validation_wait_anomaly_capture_failure(
    repo: &str,
    snapshot: &PrValidationSnapshot,
    disposition: PrValidationDisposition,
    err: &anyhow::Error,
) {
    let pr_number = snapshot.pr_number.to_string();
    let is_draft = snapshot.is_draft.to_string();
    let mut fields = vec![
        ("repo".to_string(), repo.to_string()),
        ("pr_number".to_string(), pr_number),
        ("commit_sha".to_string(), snapshot.commit_sha.clone()),
        ("pr_state".to_string(), snapshot.state.clone()),
        ("is_draft".to_string(), is_draft),
        (
            "disposition".to_string(),
            disposition.as_event_result().to_string(),
        ),
        ("detail".to_string(), err.to_string()),
    ];
    if let Some(path) = std::env::var_os("ADL_TOOLING_ANOMALY_REPORT_DIR") {
        fields.push((
            "report_dir".to_string(),
            PathBuf::from(path).display().to_string(),
        ));
    }
    let borrowed = fields
        .iter()
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect::<Vec<_>>();
    observability::emit_event(
        "adl",
        "pr.validation.wait.anomaly_capture",
        "failed",
        &borrowed,
    );
}

fn tooling_anomaly_reporting_enabled() -> bool {
    matches!(
        std::env::var("ADL_REPORT_TOOLING_ANOMALIES")
            .ok()
            .as_deref(),
        Some("1" | "true" | "TRUE" | "yes" | "on")
    )
}

fn pr_validation_anomaly_threshold() -> Duration {
    duration_env_ms("ADL_PR_VALIDATION_ANOMALY_THRESHOLD_MS", 60 * 1000)
}

fn tooling_anomaly_checkout_root() -> Result<PathBuf> {
    let mut cursor = std::env::current_dir().context("read current dir for anomaly reporting")?;
    loop {
        if cursor.join(".git").exists() {
            return Ok(cursor);
        }
        if !cursor.pop() {
            bail!("could not locate checkout root for tooling anomaly reporting");
        }
    }
}

fn tooling_anomaly_report_dir(repo_root: &Path) -> Result<PathBuf> {
    if let Some(explicit) = std::env::var_os("ADL_TOOLING_ANOMALY_REPORT_DIR") {
        let explicit = PathBuf::from(explicit);
        return Ok(if explicit.is_absolute() {
            explicit
        } else {
            repo_root.join(explicit)
        });
    }
    Ok(repo_root
        .join(".adl")
        .join("reports")
        .join("tooling-anomalies")
        .join("pr-validation"))
}

fn pr_validation_anomaly_fingerprint(
    anomaly_type: &str,
    repo: &str,
    snapshot: &PrValidationSnapshot,
    checks: &[PrValidationCheckReport],
    wait_reason: &str,
) -> String {
    let mut raw = format!(
        "{anomaly_type}|{repo}|{}|{}|{}",
        snapshot.pr_number, snapshot.commit_sha, wait_reason
    );
    for check in checks {
        raw.push('|');
        raw.push_str(&check.name);
    }
    let digest = fnv1a64(raw.as_bytes());
    let prefix = sanitize_slug(format!(
        "{anomaly_type}-pr-{}-{}",
        snapshot.pr_number,
        &snapshot.commit_sha[..snapshot.commit_sha.len().min(12)]
    ));
    format!("{prefix}-{digest:016x}")
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn unix_time_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time")
        .as_millis()
}

fn repo_relative_display(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn read_existing_tooling_anomaly_packet(path: &Path) -> Result<Option<ToolingAnomalyPacket>> {
    if !path.exists() {
        return Ok(None);
    }
    let contents = fs::read_to_string(path)
        .with_context(|| format!("read tooling anomaly packet {}", path.display()))?;
    Ok(Some(serde_json::from_str(&contents).with_context(
        || format!("parse tooling anomaly packet {}", path.display()),
    )?))
}

fn render_pr_validation_anomaly_issue_body(packet: &ToolingAnomalyPacket) -> String {
    let linked_issues = if packet.linked_issue_numbers.is_empty() {
        "none inferred".to_string()
    } else {
        packet
            .linked_issue_numbers
            .iter()
            .map(|issue| format!("#{issue}"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let checks = if packet.checks.is_empty() {
        "- no checks were reported yet".to_string()
    } else {
        packet
            .checks
            .iter()
            .map(|check| {
                format!(
                    "- `{}`: status=`{}` conclusion=`{}` job_run_id=`{}`",
                    check.name, check.status, check.conclusion, check.job_run_id
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    format!(
        "## Summary\n\nObserved `{}` while waiting on repo-native PR validation.\n\n## Evidence\n\n- Repo: `{}`\n- PR: `#{}`\n- Linked issues: {}\n- Commit: `{}`\n- PR state: `{}`\n- Draft: `{}`\n- Disposition: `{}`\n- Wait reason: `{}`\n- Elapsed: `{}` ms\n- Poll count: `{}`\n- Observation count: `{}`\n- Machine-readable packet: `{}`\n\n## Checks\n\n{}\n\n## Remediation\n\n- Reproduce with repo-native `adl/tools/pr.sh validation <pr>` or the owning lifecycle command.\n- Create or update a bounded follow-on using a safe body file rather than inline shell body text.\n- Example publication path: `adl pr issue create --title \"<title>\" --body-file {}`\n- Keep the generated packet and body file together so duplicate observations fold into the same artifact.\n",
        packet.anomaly_type,
        packet.repo,
        packet.pr_number,
        linked_issues,
        packet.commit_sha,
        packet.pr_state,
        packet.is_draft,
        packet.disposition,
        packet.wait_reason,
        packet.elapsed_ms,
        packet.poll_count,
        packet.observation_count,
        packet.report_relpath,
        checks,
        packet.issue_body_relpath
    )
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

pub(super) fn classify_pr_validation_snapshot(
    snapshot: &PrValidationSnapshot,
) -> PrValidationDisposition {
    if snapshot.state == "CLOSED" {
        return PrValidationDisposition::Cancelled;
    }
    if snapshot.checks.is_empty() {
        return PrValidationDisposition::Pending;
    }
    let effective_checks = effective_pr_validation_checks(&snapshot.checks);
    if effective_checks
        .iter()
        .any(|check| validation_conclusion_is_cancelled(&check.conclusion))
    {
        return PrValidationDisposition::Cancelled;
    }
    if effective_checks.iter().any(|check| {
        validation_conclusion_is_failed(&check.conclusion)
            || status_context_failure_status(&check.status)
    }) {
        return PrValidationDisposition::Failed;
    }
    if effective_checks
        .iter()
        .any(|check| validation_check_is_pending(&check.status, &check.conclusion))
    {
        return PrValidationDisposition::Pending;
    }
    if effective_checks
        .iter()
        .all(|check| validation_conclusion_is_skipped(&check.conclusion))
    {
        return PrValidationDisposition::Skipped;
    }
    PrValidationDisposition::Success
}

pub(super) fn pr_validation_wait_disposition_is_terminal(
    snapshot: &PrValidationSnapshot,
    disposition: PrValidationDisposition,
) -> bool {
    match disposition {
        PrValidationDisposition::Pending => false,
        PrValidationDisposition::Success | PrValidationDisposition::Skipped => !snapshot.is_draft,
        PrValidationDisposition::Failed
        | PrValidationDisposition::Cancelled
        | PrValidationDisposition::TimedOut => true,
    }
}

fn effective_pr_validation_checks(
    checks: &[PrValidationCheckSnapshot],
) -> Vec<&PrValidationCheckSnapshot> {
    let mut effective = Vec::new();
    for check in checks {
        if let Some(existing) = effective
            .iter()
            .position(|candidate: &&PrValidationCheckSnapshot| candidate.name == check.name)
        {
            if validation_check_is_newer(check, effective[existing]) {
                effective[existing] = check;
            }
        } else {
            effective.push(check);
        }
    }
    effective
}

fn validation_check_is_newer(
    candidate: &PrValidationCheckSnapshot,
    current: &PrValidationCheckSnapshot,
) -> bool {
    match (
        candidate.job_run_id.parse::<u64>(),
        current.job_run_id.parse::<u64>(),
    ) {
        (Ok(candidate_id), Ok(current_id)) => candidate_id >= current_id,
        (Ok(_), Err(_)) => true,
        (Err(_), Ok(_)) => false,
        (Err(_), Err(_)) => true,
    }
}

pub(super) fn pr_validation_report_from_snapshot_with_disposition(
    snapshot: &PrValidationSnapshot,
    disposition: PrValidationDisposition,
) -> PrValidationReport {
    let effective_checks = effective_pr_validation_checks(&snapshot.checks);
    let checks = snapshot
        .checks
        .iter()
        .map(pr_validation_check_report)
        .collect::<Vec<_>>();
    let failed_checks = effective_checks
        .iter()
        .filter(|check| {
            validation_conclusion_is_failed(&check.conclusion)
                || status_context_failure_status(&check.status)
        })
        .map(|check| pr_validation_check_report(check))
        .collect::<Vec<_>>();
    let pending_checks = effective_checks
        .iter()
        .filter(|check| validation_check_is_pending(&check.status, &check.conclusion))
        .map(|check| pr_validation_check_report(check))
        .collect::<Vec<_>>();
    PrValidationReport {
        pr_number: snapshot.pr_number,
        commit_sha: snapshot.commit_sha.clone(),
        pr_state: snapshot.state.clone(),
        is_draft: snapshot.is_draft,
        disposition: disposition.as_event_result().to_string(),
        projection_status: pr_validation_projection_status(
            &snapshot.state,
            snapshot.is_draft,
            disposition.as_event_result(),
        )
        .to_string(),
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

pub(super) fn emit_pr_validation_wait_snapshot(
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

pub(super) fn emit_pr_validation_wait_timeout(
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
