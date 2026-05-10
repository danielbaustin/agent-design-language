use super::super::*;
use serde::Deserialize;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct GithubIssueLifecycleState {
    state: String,
    #[serde(rename = "stateReason")]
    state_reason: Option<String>,
}

pub(crate) fn issue_is_closed_and_completed(issue: u32, repo: &str) -> Result<bool> {
    let Some(raw) = run_capture_allow_failure(
        "gh",
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
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(false);
    }
    let state: GithubIssueLifecycleState =
        serde_json::from_str(trimmed).context("failed to parse gh issue state json")?;
    Ok(state.state == "CLOSED" && state.state_reason.as_deref() == Some("COMPLETED"))
}

pub(crate) fn ensure_issue_closed_completed_for_closeout(issue: u32, repo: &str) -> Result<()> {
    if !issue_is_closed_and_completed(issue, repo)? {
        bail!(
            "closeout: issue #{} is not closed with COMPLETED state yet; refusing automatic closeout",
            issue
        );
    }
    Ok(())
}

pub(crate) fn wait_for_issue_closed_and_completed(issue: u32, repo: &str) -> Result<()> {
    for _ in 0..10 {
        if issue_is_closed_and_completed(issue, repo)? {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(500));
    }
    bail!(
        "finish: issue #{} did not reach CLOSED/COMPLETED state after merge; closeout cannot proceed automatically",
        issue
    );
}
