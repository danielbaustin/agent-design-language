use super::super::*;
use std::thread;
use std::time::Duration;

pub(crate) fn issue_is_closed_and_completed(issue: u32, repo: &str) -> Result<bool> {
    gh_issue_is_closed_completed(issue, repo)
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
