use super::*;

fn readiness_ready() -> IssueWatchLocalReadinessReport {
    IssueWatchLocalReadinessReport {
        status: "ready".to_string(),
        pr_run_readiness: "ready".to_string(),
        reason: "doctor_ready_pass".to_string(),
    }
}

fn readiness_failed() -> IssueWatchLocalReadinessReport {
    IssueWatchLocalReadinessReport {
        status: "failed".to_string(),
        pr_run_readiness: "unknown".to_string(),
        reason: "doctor: sor failed validation".to_string(),
    }
}

fn open_issue(number: u32) -> IssueRecord {
    IssueRecord {
        number,
        title: format!("Issue {number}"),
        state: "open".to_string(),
        url: format!("https://github.com/owner/repo/issues/{number}"),
        closed_at: None,
        body: None,
        labels: vec![],
        milestone: None,
    }
}

fn linked_pr(number: u32, is_draft: bool) -> OpenPullRequest {
    OpenPullRequest {
        number,
        title: format!("PR {number}"),
        url: format!("https://github.com/owner/repo/pull/{number}"),
        head_ref_name: format!("codex/{}-fixture", number + 1000),
        base_ref_name: "main".to_string(),
        is_draft,
        state: "OPEN".to_string(),
        queue: None,
    }
}

fn validation_report(disposition: &str, is_draft: bool) -> PrValidationReport {
    let failed_checks = if disposition == "failed" {
        vec![PrValidationCheckReport {
            name: "adl-ci".to_string(),
            status: "COMPLETED".to_string(),
            conclusion: "FAILURE".to_string(),
            job_run_id: "1".to_string(),
        }]
    } else {
        vec![]
    };
    let pending_checks = if disposition == "pending" && !is_draft {
        vec![PrValidationCheckReport {
            name: "adl-ci".to_string(),
            status: "IN_PROGRESS".to_string(),
            conclusion: "PENDING".to_string(),
            job_run_id: "2".to_string(),
        }]
    } else {
        vec![]
    };
    PrValidationReport {
        pr_number: 77,
        commit_sha: "abc".to_string(),
        pr_state: "OPEN".to_string(),
        is_draft,
        disposition: disposition.to_string(),
        checks: failed_checks
            .iter()
            .chain(pending_checks.iter())
            .cloned()
            .collect(),
        failed_checks,
        pending_checks,
    }
}

fn merged_validation_report() -> PrValidationReport {
    PrValidationReport {
        pr_number: 77,
        commit_sha: "abc".to_string(),
        pr_state: "MERGED".to_string(),
        is_draft: false,
        disposition: "success".to_string(),
        checks: vec![],
        failed_checks: vec![],
        pending_checks: vec![],
    }
}

#[test]
fn issue_watch_routes_ready_issue_without_pr_to_pr_run() {
    let report = build_issue_watch_report(&open_issue(4397), false, readiness_ready(), None);
    assert_eq!(report.authoritative_classifier, "adl");
    assert_eq!(report.advisory_agent_mode, "local_agent_advisory_only");
    assert_eq!(report.classification, "ready_for_run");
    assert_eq!(report.next_skill, "pr-run");
    assert_eq!(report.continuation, "continue");
}

#[test]
fn issue_watch_routes_draft_pr_to_issue_watcher() {
    let pr = linked_pr(77, true);
    let report = build_issue_watch_report(
        &open_issue(4397),
        false,
        readiness_ready(),
        Some((pr, validation_report("pending", true))),
    );
    assert_eq!(report.classification, "pr_open");
    assert_eq!(report.next_skill, "issue-watcher");
}

#[test]
fn issue_watch_routes_failed_checks_to_pr_janitor() {
    let pr = linked_pr(77, false);
    let report = build_issue_watch_report(
        &open_issue(4397),
        false,
        readiness_ready(),
        Some((pr, validation_report("failed", false))),
    );
    assert_eq!(report.classification, "checks_failed");
    assert_eq!(report.next_skill, "pr-janitor");
}

#[test]
fn issue_watch_routes_pending_checks_to_issue_watcher() {
    let pr = linked_pr(77, false);
    let report = build_issue_watch_report(
        &open_issue(4397),
        false,
        readiness_ready(),
        Some((pr, validation_report("pending", false))),
    );
    assert_eq!(report.classification, "checks_running");
    assert_eq!(report.next_skill, "issue-watcher");
    assert_eq!(report.continuation, "continue");
}

#[test]
fn issue_watch_routes_green_checks_to_human_review() {
    let pr = linked_pr(77, false);
    let report = build_issue_watch_report(
        &open_issue(4397),
        false,
        readiness_ready(),
        Some((pr, validation_report("success", false))),
    );
    assert_eq!(report.classification, "checks_green");
    assert_eq!(report.next_skill, "human_review");
}

#[test]
fn issue_watch_routes_closed_completed_issue_to_closeout() {
    let mut issue = open_issue(4397);
    issue.state = "closed".to_string();
    issue.closed_at = Some("2026-06-22T00:00:00Z".to_string());
    let report = build_issue_watch_report(&issue, true, readiness_ready(), None);
    assert_eq!(report.classification, "closeout_needed");
    assert_eq!(report.next_skill, "pr-closeout");
}

#[test]
fn issue_watch_routes_merged_pr_to_merged_pending_closeout() {
    let mut pr = linked_pr(77, false);
    pr.state = "MERGED".to_string();
    let report = build_issue_watch_report(
        &open_issue(4397),
        false,
        readiness_ready(),
        Some((pr, merged_validation_report())),
    );
    assert_eq!(report.classification, "merged_pending_closeout");
    assert_eq!(report.next_skill, "pr-closeout");
    assert_eq!(report.continuation, "action_required");
}

#[test]
fn issue_watch_routes_failed_local_readiness_without_pr_to_blocked() {
    let report = build_issue_watch_report(&open_issue(4397), false, readiness_failed(), None);
    assert_eq!(report.classification, "blocked");
    assert_eq!(report.next_skill, "pr-ready");
    assert_eq!(report.local_readiness.reason, "doctor: sor failed validation");
}
