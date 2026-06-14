use super::*;
use crate::cli::pr_cmd::github_client::{
    body_contains_closing_linkage, issue_labels_from_csv, issue_labels_from_csv_in_order,
    issue_metadata_drift, linked_issue_numbers_from_lines, linked_issue_numbers_include,
    plan_issue_metadata_parity, pr_matches_main_version_wave, AdlGithubClient, GithubClientBackend,
    IssueMetadataSnapshot, PullRequestMetadataSnapshot,
};
use crate::cli::pr_cmd_prompt::infer_workflow_queue;
use ::adl::control_plane::resolve_primary_checkout_root;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
struct RepoParts {
    owner: String,
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ReadyForReviewResponse {}

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

fn current_pr_url_octocrab(repo: &str, branch: &str) -> Result<Option<String>> {
    let repo_parts = parse_repo(repo)?;
    let head = format!("{}:{branch}", repo_parts.owner);
    with_octocrab("pr.list.current_branch", |octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let page = block_on_octocrab(
            "pr.list.current_branch",
            octo.pulls(&owner, &name)
                .list()
                .state(octocrab::params::State::Open)
                .head(head)
                .per_page(10)
                .send(),
        )?;
        Ok(page
            .items
            .into_iter()
            .find_map(|pr| pr.html_url.map(|url| url.to_string())))
    })
}

fn list_open_prs_octocrab(repo: &str) -> Result<Vec<OpenPullRequest>> {
    let repo_parts = parse_repo(repo)?;
    with_octocrab("pr.list.open_wave", |octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let page = block_on_octocrab(
            "pr.list.open_wave",
            octo.pulls(&owner, &name)
                .list()
                .state(octocrab::params::State::Open)
                .per_page(100)
                .send(),
        )?;
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
    with_octocrab("pr.view.body", |octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let pr = block_on_octocrab("pr.view.body", octo.pulls(&owner, &name).get(number))?;
        Ok(pr.body.unwrap_or_default())
    })
}

fn pr_closing_issue_numbers_octocrab(repo: &str, pr_ref: &str) -> Result<Vec<u32>> {
    let repo_parts = parse_repo(repo)?;
    let number = parse_pr_number(pr_ref)? as i64;
    with_octocrab("pr.view.closing_issues", |octo| {
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
            block_on_octocrab("pr.view.closing_issues", octo.graphql(&payload))?;
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
    with_octocrab("pr.view.base_ref.finish_existing", |octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let pr = block_on_octocrab(
            "pr.view.base_ref.finish_existing",
            octo.pulls(&owner, &name).get(number),
        )?;
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
    with_octocrab("pr.create.finish", |octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let pr = block_on_octocrab(
            "pr.create.finish",
            octo.pulls(&owner, &name)
                .create(title, head, base)
                .body(body.to_string())
                .draft(draft)
                .send(),
        )?;
        Ok(pr.html_url.map(|url| url.to_string()).ok_or_else(|| {
            anyhow!("github_client.octocrab_transport: PR create returned no url")
        })?)
    })
}

fn update_pr_body_octocrab(repo: &str, pr_ref: &str, body: &str) -> Result<()> {
    let repo_parts = parse_repo(repo)?;
    let number = parse_pr_number(pr_ref)?;
    with_octocrab("pr.edit.body_file", |octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        block_on_octocrab(
            "pr.edit.body_file",
            octo.pulls(&owner, &name)
                .update(number)
                .body(body.to_string())
                .send(),
        )?;
        Ok(())
    })
}

fn update_pr_title_body_octocrab(repo: &str, pr_ref: &str, title: &str, body: &str) -> Result<()> {
    let repo_parts = parse_repo(repo)?;
    let number = parse_pr_number(pr_ref)?;
    with_octocrab("pr.edit.finish_existing", |octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        block_on_octocrab(
            "pr.edit.finish_existing",
            octo.pulls(&owner, &name)
                .update(number)
                .title(title.to_string())
                .body(body.to_string())
                .send(),
        )?;
        Ok(())
    })
}

fn mark_pr_ready_octocrab(repo: &str, pr_ref: &str) -> Result<()> {
    let repo_parts = parse_repo(repo)?;
    let number = parse_pr_number(pr_ref)?;
    with_octocrab("pr.ready", |octo| {
        let route = format!(
            "/repos/{}/{}/pulls/{}/ready_for_review",
            repo_parts.owner, repo_parts.name, number
        );
        let _: ReadyForReviewResponse =
            block_on_octocrab("pr.ready", octo.post(route, None::<&()>))?;
        Ok(())
    })
}

fn merge_pr_octocrab(repo: &str, pr_ref: &str) -> Result<()> {
    let repo_parts = parse_repo(repo)?;
    let number = parse_pr_number(pr_ref)?;
    with_octocrab("pr.merge.finish", |octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        block_on_octocrab(
            "pr.merge.finish",
            octo.pulls(&owner, &name)
                .merge(number)
                .method(octocrab::params::pulls::MergeMethod::Squash)
                .send(),
        )?;
        Ok(())
    })
}

fn github_client(operation: &str) -> Result<AdlGithubClient> {
    let client = AdlGithubClient::from_env()
        .with_context(|| format!("github client policy rejected operation '{operation}'"))?;
    let config = client.config();
    match config.backend {
        GithubClientBackend::Octocrab => Ok(client),
        GithubClientBackend::GhFallback => bail!(
            "github_client.gh_fallback_removed: operation '{}' requires octocrab transport; set GITHUB_TOKEN or GH_TOKEN and do not rely on gh fallback",
            operation
        ),
    }
}

pub(super) fn run_gh_capture(operation: &str, args: &[&str]) -> Result<String> {
    run_octocrab_capture(operation, args)
}

pub(crate) fn run_gh_capture_allow_failure(
    operation: &str,
    args: &[&str],
) -> Result<Option<String>> {
    run_octocrab_capture(operation, args).map(Some)
}

pub(super) fn run_gh_status(operation: &str, args: &[&str]) -> Result<()> {
    run_octocrab_status(operation, args)
}

pub(super) fn run_gh_status_allow_failure(operation: &str, args: &[&str]) -> Result<bool> {
    run_octocrab_status(operation, args).map(|_| true)
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

fn block_on_octocrab<T>(
    operation: &str,
    f: impl std::future::Future<Output = octocrab::Result<T>>,
) -> Result<T> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .with_context(|| {
            format!("github_client.octocrab_runtime: failed to build runtime for {operation}")
        })?;
    runtime.block_on(f).map_err(|err| {
        anyhow!(
            "github_client.octocrab_transport: operation '{}' failed: {}",
            operation,
            err
        )
    })
}

fn with_octocrab<T>(operation: &str, f: impl FnOnce(octocrab::Octocrab) -> Result<T>) -> Result<T> {
    let client = github_client(operation)?;
    let octo = client
        .octocrab()
        .map_err(|err| anyhow!("github_client.octocrab_build: {err}"))?;
    f(octo)
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
        other => bail!("github_client.unsupported_status_operation: {other}"),
    }
}

fn arg_after<'a>(args: &'a [&str], flag: &str) -> Result<&'a str> {
    args.windows(2)
        .find_map(|window| (window[0] == flag).then_some(window[1]))
        .ok_or_else(|| anyhow!("missing required argument '{flag}' in GitHub operation"))
}

fn positional_after<'a>(args: &'a [&str], command: &str) -> Result<&'a str> {
    args.windows(2)
        .find_map(|window| (window[0] == command).then_some(window[1]))
        .ok_or_else(|| anyhow!("missing positional argument after '{command}' in GitHub operation"))
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
    Ok(prs
        .into_iter()
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
        .filter(|pr| {
            pr.queue
                .as_deref()
                .is_none_or(|queue| queue == target_queue)
        })
        .collect())
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
    let _ = (repo_root, repo, issue, branch, pr_url);
    eprintln!(
        "• Post-merge closeout watcher is not auto-attached: legacy shell watcher was retired from the no-gh workflow. Run `adl/tools/pr.sh closeout <issue>` after GitHub closes the issue."
    );
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
    let repo_parts = parse_repo(repo)?;
    let labels = issue_labels_from_csv_in_order(labels_csv);
    with_octocrab("issue.create", |octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let issue = block_on_octocrab(
            "issue.create",
            octo.issues(&owner, &name)
                .create(title)
                .body(body.to_string())
                .labels(labels)
                .send(),
        )?;
        Ok(issue.html_url.to_string())
    })
}

pub(crate) fn gh_issue_label_names(issue: u32, repo: &str) -> Result<Vec<String>> {
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.view.labels", |octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let issue = block_on_octocrab(
            "issue.view.labels",
            octo.issues(&owner, &name).get(issue as u64),
        )?;
        Ok(issue.labels.into_iter().map(|label| label.name).collect())
    })
}

fn gh_issue_edit_title(repo: &str, issue: u32, title: &str) -> Result<()> {
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.edit.title", |octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        block_on_octocrab(
            "issue.edit.title",
            octo.issues(&owner, &name)
                .update(issue as u64)
                .title(title)
                .send(),
        )?;
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
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.edit.body", |octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        block_on_octocrab(
            "issue.edit.body",
            octo.issues(&owner, &name)
                .update(issue as u64)
                .body(body)
                .send(),
        )?;
        Ok(())
    })
    .with_context(|| format!("create: octocrab issue edit failed for issue #{issue}"))
}

pub(super) fn gh_issue_title(issue: u32, repo: &str) -> Result<Option<String>> {
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.view.title", |octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let issue = block_on_octocrab(
            "issue.view.title",
            octo.issues(&owner, &name).get(issue as u64),
        )?;
        Ok(Some(issue.title).filter(|title| !title.trim().is_empty()))
    })
}

pub(crate) fn gh_issue_body(issue: u32, repo: &str) -> Result<Option<String>> {
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.view.body", |octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let issue = block_on_octocrab(
            "issue.view.body",
            octo.issues(&owner, &name).get(issue as u64),
        )?;
        Ok(issue.body.filter(|body| !body.trim().is_empty()))
    })
}

pub(crate) fn gh_issue_is_closed_completed(issue: u32, repo: &str) -> Result<bool> {
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.view.state", |octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        let issue = block_on_octocrab(
            "issue.view.state",
            octo.issues(&owner, &name).get(issue as u64),
        )?;
        Ok(issue.state == octocrab::models::IssueState::Closed
            && issue.state_reason == Some(octocrab::models::issues::IssueStateReason::Completed))
    })
}

fn gh_issue_set_labels(repo: &str, issue: u32, labels: &[String]) -> Result<()> {
    let repo_parts = parse_repo(repo)?;
    with_octocrab("issue.edit.labels", |octo| {
        let owner = repo_parts.owner.clone();
        let name = repo_parts.name.clone();
        block_on_octocrab(
            "issue.edit.labels",
            octo.issues(&owner, &name)
                .update(issue as u64)
                .labels(labels)
                .send(),
        )?;
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

        write_executable(
            &bin_dir.join("gh"),
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
            std::env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
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
        .expect("blank override closeout fallback");

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
        write_executable(
            &bin_dir.join("gh"),
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

        write_executable(
            &bin_dir.join("gh"),
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
            if args[i + 1] not in current_labels:
                current_labels.append(args[i + 1])
            i += 2
        elif args[i] == "--remove-label":
            current_labels = [label for label in current_labels if label != args[i + 1]]
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
            "track:roadmap\narea:tools\nversion:v0.91.5\n"
        );

        restore_env("PATH", old_path);

        let calls = fs::read_to_string(&log_file).expect("gh log");
        assert!(calls.contains("'--label', 'version:v0.91.5'"));
        assert!(calls.contains("'--label', 'area:tools'"));
        assert!(calls.contains("'--label', 'type:task'"));
        assert!(calls.contains("'--title', '[v0.91.5][tools] New title'"));
        assert!(calls.contains("'--add-label', 'area:tools'"));
        assert!(calls.contains("'--add-label', 'version:v0.91.5'"));
        assert!(calls.contains("'--remove-label', 'version:v0.91.4'"));
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
        let err = gh_issue_edit_body("owner/repo", 3672, "body")
            .expect_err("explicit gh fallback issue edit should fail closed");
        let err_debug = format!("{err:?}");
        assert!(err_debug.contains("github_client.gh_fallback_removed"));
        assert!(
            !gh_log.exists(),
            "fallback removal should reject before spawning gh"
        );

        restore_env("PATH", old_path);
        restore_github_policy_env(policy_env);
    }
}
