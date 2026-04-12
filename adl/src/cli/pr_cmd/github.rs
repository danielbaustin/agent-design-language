use super::*;
use crate::cli::pr_cmd_prompt::infer_workflow_queue;
use ::adl::control_plane::resolve_primary_checkout_root;
use serde::Deserialize;
use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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

pub(super) fn current_pr_url(repo: &str, branch: &str) -> Result<Option<String>> {
    let out = run_capture_allow_failure(
        "gh",
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
    let out = run_capture_allow_failure(
        "gh",
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
        serde_json::from_str(&out).with_context(|| "failed to parse gh pr list json")?;
    let version_tag = format!("[{version}]");
    Ok(prs
        .into_iter()
        .filter(|pr| pr.base_ref_name == "main")
        .filter(|pr| pr.title.contains(&version_tag))
        .map(|mut pr| {
            pr.queue = infer_workflow_queue(&pr.title, "", None).map(str::to_string);
            pr
        })
        .filter(|pr| {
            exclude_branch
                .map(|branch| pr.head_ref_name != branch)
                .unwrap_or(true)
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
    let linked = run_capture_allow_failure(
        "gh",
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
    if linked
        .as_deref()
        .unwrap_or_default()
        .lines()
        .any(|line| line.trim() == issue.to_string())
    {
        return Ok(true);
    }
    let body = run_capture_allow_failure(
        "gh",
        &[
            "pr", "view", "-R", repo, pr_ref, "--json", "body", "--jq", ".body",
        ],
    )?
    .unwrap_or_default();
    Ok(body.contains(&format!("Closes #{issue}")))
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
    run_status(
        "gh",
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

    let command_path = std::env::var("ADL_POST_MERGE_CLOSEOUT_CMD")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| {
            helper_command_path(repo_root, "adl/tools/attach_post_merge_closeout.sh")
        });
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
    let labels = run_capture_allow_failure(
        "gh",
        &[
            "issue",
            "view",
            &issue.to_string(),
            "-R",
            repo,
            "--json",
            "labels",
            "-q",
            ".labels[].name",
        ],
    )?;
    if let Some(labels) = labels {
        for line in labels.lines() {
            if let Some(version) = line.strip_prefix("version:") {
                return Ok(Some(version.trim().to_string()));
            }
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
    let mut cmd = Command::new("gh");
    cmd.arg("issue")
        .arg("create")
        .arg("-R")
        .arg(repo)
        .arg("--title")
        .arg(title)
        .arg("--body")
        .arg(body);
    for label in labels_csv
        .split(',')
        .map(str::trim)
        .filter(|label| !label.is_empty())
    {
        cmd.arg("--label").arg(label);
    }
    let output = cmd
        .output()
        .with_context(|| "failed to spawn gh issue create")?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "init: gh issue create failed{}",
            if stderr.trim().is_empty() {
                String::new()
            } else {
                format!(": {}", stderr.trim())
            }
        );
    }
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        bail!("init: gh issue create returned empty output");
    }
    Ok(stdout)
}

fn issue_label_names(issue: u32, repo: &str) -> Result<Vec<String>> {
    let labels = run_capture_allow_failure(
        "gh",
        &[
            "issue",
            "view",
            &issue.to_string(),
            "-R",
            repo,
            "--json",
            "labels",
            "-q",
            ".labels[].name",
        ],
    )?
    .unwrap_or_default();
    Ok(labels
        .lines()
        .map(str::trim)
        .filter(|label| !label.is_empty())
        .map(ToString::to_string)
        .collect())
}

fn gh_issue_edit_title(repo: &str, issue: u32, title: &str) -> Result<()> {
    run_status(
        "gh",
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
    .with_context(|| format!("create: gh issue edit title failed for issue #{issue}"))
}

fn gh_issue_add_labels(repo: &str, issue: u32, labels: &[String]) -> Result<()> {
    if labels.is_empty() {
        return Ok(());
    }
    let issue_s = issue.to_string();
    let mut args = vec!["issue", "edit", issue_s.as_str(), "-R", repo];
    for label in labels {
        args.push("--add-label");
        args.push(label);
    }
    run_status("gh", &args)
        .with_context(|| format!("create: gh issue add labels failed for issue #{issue}"))
}

fn gh_issue_remove_labels(repo: &str, issue: u32, labels: &[String]) -> Result<()> {
    if labels.is_empty() {
        return Ok(());
    }
    let issue_s = issue.to_string();
    let mut args = vec!["issue", "edit", issue_s.as_str(), "-R", repo];
    for label in labels {
        args.push("--remove-label");
        args.push(label);
    }
    run_status("gh", &args)
        .with_context(|| format!("create: gh issue remove labels failed for issue #{issue}"))
}

pub(super) fn ensure_issue_metadata_parity(
    repo: &str,
    issue: u32,
    expected_title: &str,
    labels_csv: &str,
) -> Result<()> {
    let expected: BTreeSet<String> = labels_csv
        .split(',')
        .map(str::trim)
        .filter(|label| !label.is_empty())
        .map(ToString::to_string)
        .collect();
    if expected.is_empty() {
        bail!("create: expected at least one label for tracked issue creation");
    }

    let current_title = gh_issue_title(issue, repo)?.unwrap_or_default();
    if current_title != expected_title {
        gh_issue_edit_title(repo, issue, expected_title)?;
    }

    let actual: BTreeSet<String> = issue_label_names(issue, repo)?.into_iter().collect();
    let missing: Vec<String> = expected.difference(&actual).cloned().collect();
    let stale_versions: Vec<String> = actual
        .iter()
        .filter(|label| label.starts_with("version:") && !expected.contains(*label))
        .cloned()
        .collect();

    if !missing.is_empty() {
        gh_issue_add_labels(repo, issue, &missing)?;
    }
    if !stale_versions.is_empty() {
        gh_issue_remove_labels(repo, issue, &stale_versions)?;
    }

    let final_title = gh_issue_title(issue, repo)?.unwrap_or_default();
    if final_title != expected_title {
        bail!(
            "create: issue #{} title mismatch after metadata parity enforcement: expected '{}', got '{}'",
            issue,
            expected_title,
            final_title
        );
    }
    let final_labels: BTreeSet<String> = issue_label_names(issue, repo)?.into_iter().collect();
    let final_missing: Vec<String> = expected.difference(&final_labels).cloned().collect();
    let final_stale_versions: Vec<String> = final_labels
        .iter()
        .filter(|label| label.starts_with("version:") && !expected.contains(*label))
        .cloned()
        .collect();
    if !final_missing.is_empty() || !final_stale_versions.is_empty() {
        let mut problems = Vec::new();
        if !final_missing.is_empty() {
            problems.push(format!("missing labels: {}", final_missing.join(", ")));
        }
        if !final_stale_versions.is_empty() {
            problems.push(format!(
                "unexpected version labels: {}",
                final_stale_versions.join(", ")
            ));
        }
        bail!(
            "create: issue #{} metadata drift remains after parity enforcement: {}",
            issue,
            problems.join("; ")
        );
    }
    Ok(())
}

pub(super) fn gh_issue_edit_body(repo: &str, issue: u32, body: &str) -> Result<()> {
    let body_file = write_temp_markdown("issue_body", body)?;
    run_status(
        "gh",
        &[
            "issue",
            "edit",
            &issue.to_string(),
            "-R",
            repo,
            "--body-file",
            path_str(&body_file)?,
        ],
    )
    .with_context(|| format!("create: gh issue edit failed for issue #{issue}"))
}

pub(super) fn gh_issue_title(issue: u32, repo: &str) -> Result<Option<String>> {
    let out = run_capture_allow_failure(
        "gh",
        &[
            "issue",
            "view",
            &issue.to_string(),
            "-R",
            repo,
            "--json",
            "title",
            "-q",
            ".title",
        ],
    )?;
    Ok(out
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty()))
}
