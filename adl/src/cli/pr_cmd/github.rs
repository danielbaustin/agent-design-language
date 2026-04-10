use super::*;
use serde::Deserialize;
use std::collections::BTreeSet;
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
        .filter(|pr| {
            exclude_branch
                .map(|branch| pr.head_ref_name != branch)
                .unwrap_or(true)
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
                pr.title,
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

pub(super) fn ensure_issue_labels(repo: &str, issue: u32, labels_csv: &str) -> Result<()> {
    let expected: BTreeSet<String> = labels_csv
        .split(',')
        .map(str::trim)
        .filter(|label| !label.is_empty())
        .map(ToString::to_string)
        .collect();
    if expected.is_empty() {
        bail!("create: expected at least one label for tracked issue creation");
    }

    let actual: BTreeSet<String> = issue_label_names(issue, repo)?.into_iter().collect();
    let missing: Vec<String> = expected.difference(&actual).cloned().collect();
    if !missing.is_empty() {
        bail!(
            "create: issue #{} is missing expected labels after gh issue create: {}",
            issue,
            missing.join(", ")
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
