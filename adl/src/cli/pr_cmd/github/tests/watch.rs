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
        created_at: None,
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
        updated_at: None,
        mergeable: None,
        queue: None,
    }
}

fn linked_pr_with_head(number: u32, head_ref_name: &str) -> OpenPullRequest {
    OpenPullRequest {
        head_ref_name: head_ref_name.to_string(),
        ..linked_pr(number, false)
    }
}

#[test]
fn open_pr_wave_format_includes_known_and_unknown_queue_truth() {
    let mut queued = linked_pr(4705, true);
    queued.title = "[v0.91.7][WP-02] Add repo-native PR inventory command".to_string();
    queued.queue = Some("wp".to_string());
    let unclassified = linked_pr(4706, false);

    let rendered = format_open_pr_wave(&[queued, unclassified]);

    assert!(rendered.contains("#4705 [draft] [queue=wp]"));
    assert!(rendered.contains("#4706 [ready] [queue=unknown]"));
}

#[test]
fn non_closing_lifecycle_marker_is_case_insensitive() {
    assert!(body_declares_non_closing_lifecycle_pr(
        "This is a NON-CLOSING LIFECYCLE PR for review only."
    ));
    assert!(!body_declares_non_closing_lifecycle_pr(
        "This PR closes the tracked implementation issue."
    ));
}

#[test]
fn pr_metadata_helpers_parse_expected_success_and_error_shapes() {
    let repo = parse_repo("danielbaustin/agent-design-language").expect("repo parses");
    assert_eq!(repo.owner, "danielbaustin");
    assert_eq!(repo.name, "agent-design-language");
    assert!(parse_repo("missing-slash").is_err());
    assert!(parse_repo("/missing-owner").is_err());

    assert_eq!(parse_pr_number("4705").expect("numeric PR"), 4705);
    assert_eq!(
        parse_pr_number("https://github.com/danielbaustin/agent-design-language/pull/4705/files")
            .expect("URL PR"),
        4705
    );
    assert!(parse_pr_number("not-a-pr").is_err());
}

#[test]
fn issue_record_conversion_filters_pull_requests_and_preserves_issue_fields() {
    let issue: RestIssueRecord = serde_json::from_value(serde_json::json!({
        "number": 4661,
        "title": "Consume closeout truth",
        "state": "open",
        "html_url": "https://github.com/owner/repo/issues/4661",
        "created_at": "2026-06-30T17:30:12Z",
        "closed_at": null,
        "body": "body",
        "labels": [{"name": "version:v0.91.7"}],
        "milestone": {"title": "v0.91.7"}
    }))
    .expect("issue fixture");
    let converted = issue.into_issue_record().expect("issue record");
    assert_eq!(converted.number, 4661);
    assert_eq!(converted.labels, vec!["version:v0.91.7"]);
    assert_eq!(converted.milestone.as_deref(), Some("v0.91.7"));

    let pull_request: RestIssueRecord = serde_json::from_value(serde_json::json!({
        "number": 4705,
        "title": "PR",
        "state": "open",
        "html_url": "https://github.com/owner/repo/pull/4705",
        "pull_request": {}
    }))
    .expect("PR fixture");
    assert!(pull_request.into_issue_record().is_none());

    let missing_title: RestIssueRecord = serde_json::from_value(serde_json::json!({
        "number": 4662,
        "state": "open",
        "html_url": "https://github.com/owner/repo/issues/4662"
    }))
    .expect("missing-title fixture");
    assert!(missing_title.into_issue_record().is_none());

    let missing_state: RestIssueRecord = serde_json::from_value(serde_json::json!({
        "number": 4663,
        "title": "Missing state",
        "html_url": "https://github.com/owner/repo/issues/4663"
    }))
    .expect("missing-state fixture");
    assert!(missing_state.into_issue_record().is_none());

    let missing_url: RestIssueRecord = serde_json::from_value(serde_json::json!({
        "number": 4664,
        "title": "Missing URL",
        "state": "open"
    }))
    .expect("missing-url fixture");
    assert!(missing_url.into_issue_record().is_none());
}

#[test]
fn github_argument_helpers_cover_expected_success_and_error_paths() {
    assert_eq!(
        arg_after(&["pr", "view", "-R", "owner/repo", "4705"], "-R").expect("repo flag"),
        "owner/repo"
    );
    assert!(arg_after(&["pr", "view", "4705"], "-R")
        .expect_err("missing repo flag")
        .to_string()
        .contains("missing required argument '-R'"));

    assert_eq!(
        positional_after(
            &[
                "pr",
                "create",
                "-R",
                "owner/repo",
                "--base",
                "main",
                "--head",
                "codex/4622-pr-inventory",
                "--draft",
                "4705",
            ],
            "create"
        )
        .expect("positional after flags"),
        "4705"
    );
    assert!(
        positional_after(&["pr", "view", "-R", "owner/repo"], "view")
            .expect_err("missing positional")
            .to_string()
            .contains("missing positional argument after 'view'")
    );
    assert!(positional_after(&["pr", "view", "4705"], "edit")
        .expect_err("missing command")
        .to_string()
        .contains("missing GitHub command 'edit'"));
}

#[test]
fn issue_close_reason_and_body_version_helpers_are_strict() {
    assert!(matches!(
        issue_close_reason_from_args(&["issue", "close", "4622"]).expect("default close reason"),
        octocrab::models::issues::IssueStateReason::Completed
    ));
    assert!(matches!(
        issue_close_reason_from_args(&["issue", "close", "4622", "--reason", "not-planned"])
            .expect("hyphenated not planned"),
        octocrab::models::issues::IssueStateReason::NotPlanned
    ));
    assert!(matches!(
        issue_close_reason_from_args(&["issue", "close", "4622", "--state-reason", "not_planned",])
            .expect("underscored not planned"),
        octocrab::models::issues::IssueStateReason::NotPlanned
    ));
    assert!(
        issue_close_reason_from_args(&["issue", "close", "4622", "--reason", "duplicate"])
            .expect_err("unsupported close reason")
            .to_string()
            .contains("unsupported state reason 'duplicate'")
    );

    assert_eq!(
        explicit_issue_body_version("Title\nVersion: v0.91.7\n").expect("body version"),
        Some("v0.91.7".to_string())
    );
    assert_eq!(
        explicit_issue_body_version("No explicit version").expect("no body version"),
        None
    );
    assert!(
        explicit_issue_body_version("Version: v0.91.6\nversion: v0.91.7")
            .expect_err("conflicting versions")
            .to_string()
            .contains("conflicting explicit body version evidence")
    );
}

#[test]
fn open_pr_state_default_and_retry_policy_helpers_are_deterministic() {
    let pr: OpenPullRequest = serde_json::from_value(serde_json::json!({
        "number": 4705,
        "title": "PR",
        "url": "https://github.com/owner/repo/pull/4705",
        "headRefName": "codex/4622-pr-inventory",
        "baseRefName": "main",
        "isDraft": true
    }))
    .expect("PR with default state");
    assert_eq!(pr.state, default_open_pr_state());
    assert!(octocrab_operation_allows_retry("pr.validation.status"));
    assert!(octocrab_operation_allows_retry("issue.close"));
    assert!(!octocrab_operation_allows_retry("pr.create.finish"));
    assert_eq!(
        octocrab_retry_delay(0),
        std::time::Duration::from_millis(50)
    );
    assert_eq!(
        octocrab_retry_delay(2),
        std::time::Duration::from_millis(150)
    );
    assert_eq!(
        octocrab_retry_delay(3),
        std::time::Duration::from_millis(300)
    );
}

#[test]
fn projection_status_and_codex_branch_parsing_cover_edge_cases() {
    assert_eq!(
        pr_validation_projection_status("MERGED", true, "failed"),
        "merged"
    );
    assert_eq!(
        pr_validation_projection_status("OPEN", true, "success"),
        "checks_green_but_draft"
    );
    assert_eq!(
        pr_validation_projection_status("OPEN", false, "skipped"),
        "ready_to_merge_or_review"
    );
    assert_eq!(
        pr_validation_projection_status("OPEN", false, "timed_out"),
        "checks_failed"
    );
    assert_eq!(
        pr_validation_projection_status("OPEN", true, "unknown"),
        "checks_pending"
    );
    assert_eq!(
        pr_validation_projection_status("OPEN", false, "unknown"),
        "unknown"
    );

    assert_eq!(
        issue_number_from_codex_branch("refs/heads/codex/4622-pr-inventory"),
        Some(4622)
    );
    assert_eq!(
        issue_number_from_codex_branch("feature/codex/4622-pr-inventory"),
        Some(4622)
    );
    assert_eq!(
        issue_number_from_codex_branch("feature/not-codex/4622-pr-inventory"),
        None
    );
    assert_eq!(issue_number_from_codex_branch("codex/no-number"), None);
    assert_eq!(issue_number_from_codex_branch("codex/4622"), None);
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
        projection_status: pr_validation_projection_status("OPEN", is_draft, disposition)
            .to_string(),
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
        projection_status: "merged".to_string(),
        checks: vec![],
        failed_checks: vec![],
        pending_checks: vec![],
    }
}

fn shepherd_report(
    classification: &str,
    tail_owner: &str,
    shepherd_state: &str,
    next_skill: &str,
    continuation: &str,
    reason: &str,
    local_readiness: IssueWatchLocalReadinessReport,
) -> IssueWatchReport {
    IssueWatchReport {
        schema: "adl.pr.watch.v1",
        issue: 4630,
        issue_state: "OPEN".to_string(),
        authoritative_classifier: "adl",
        advisory_agent_mode: "local_agent_advisory_only",
        classification: classification.to_string(),
        tail_owner: tail_owner.to_string(),
        shepherd_state: shepherd_state.to_string(),
        next_skill: next_skill.to_string(),
        continuation: continuation.to_string(),
        reason: reason.to_string(),
        local_readiness,
        linked_pr: None,
    }
}

#[test]
fn issue_watch_routes_ready_issue_without_pr_to_pr_run() {
    let report = build_issue_watch_report(&open_issue(4397), false, readiness_ready(), None);
    assert_eq!(report.authoritative_classifier, "adl");
    assert_eq!(report.advisory_agent_mode, "local_agent_advisory_only");
    assert_eq!(report.classification, "ready_for_run");
    assert_eq!(report.tail_owner, "pr-run");
    assert_eq!(report.shepherd_state, "ready_without_pr");
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
    assert_eq!(report.tail_owner, "issue-watcher");
    assert_eq!(report.shepherd_state, "watcher_owned_pr_open");
    assert_eq!(report.next_skill, "issue-watcher");
}

#[test]
fn issue_watch_routes_all_green_draft_pr_to_janitor() {
    let pr = linked_pr(77, true);
    let report = build_issue_watch_report(
        &open_issue(4397),
        false,
        readiness_ready(),
        Some((pr, validation_report("success", true))),
    );
    assert_eq!(report.classification, "checks_green_but_draft");
    assert_eq!(report.tail_owner, "pr-janitor");
    assert_eq!(
        report.shepherd_state,
        "green_draft_requires_publication_action"
    );
    assert_eq!(report.next_skill, "pr-janitor");
    assert_eq!(report.continuation, "action_required");
    assert_eq!(report.reason, "linked_pr_checks_green_but_draft");
    assert_eq!(
        report
            .linked_pr
            .as_ref()
            .expect("linked PR")
            .validation
            .projection_status,
        "checks_green_but_draft"
    );
}

#[test]
fn issue_watch_pr_number_resolution_routes_missing_closing_linkage_by_codex_branch() {
    let pr = linked_pr_with_head(4495, "codex/4487-worktree-safe-truth");
    assert_eq!(
        issue_number_from_pr_metadata_for_watch(&pr, &[]).expect("codex branch issue fallback"),
        4487
    );

    let missing = linked_pr_with_head(4496, "feature/no-issue-prefix");
    let err = issue_number_from_pr_metadata_for_watch(&missing, &[])
        .expect_err("missing closing refs and non-codex branch should fail");
    assert!(err.to_string().contains("has no closing issue metadata"));

    let ambiguous = issue_number_from_pr_metadata_for_watch(&pr, &[4487, 4488])
        .expect_err("multiple closing issues should require explicit issue");
    assert!(ambiguous
        .to_string()
        .contains("closes multiple issues 4487, 4488"));
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
    assert_eq!(report.tail_owner, "pr-janitor");
    assert_eq!(report.shepherd_state, "janitor_owned_checks_failed");
    assert_eq!(report.next_skill, "pr-janitor");
}

#[test]
fn issue_watch_routes_failed_draft_checks_to_pr_janitor() {
    let pr = linked_pr(77, true);
    let report = build_issue_watch_report(
        &open_issue(4397),
        false,
        readiness_ready(),
        Some((pr, validation_report("failed", true))),
    );
    assert_eq!(report.classification, "checks_failed");
    assert_eq!(report.tail_owner, "pr-janitor");
    assert_eq!(report.shepherd_state, "janitor_owned_checks_failed");
    assert_eq!(report.next_skill, "pr-janitor");
    assert_eq!(report.continuation, "action_required");
    assert_eq!(report.reason, "linked_pr_checks_failed");
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
    assert_eq!(report.tail_owner, "issue-watcher");
    assert_eq!(report.shepherd_state, "watcher_owned_checks_running");
    assert_eq!(report.next_skill, "issue-watcher");
    assert_eq!(report.continuation, "continue");
}

#[test]
fn issue_watch_routes_green_checks_to_issue_watcher() {
    let pr = linked_pr(77, false);
    let report = build_issue_watch_report(
        &open_issue(4397),
        false,
        readiness_ready(),
        Some((pr, validation_report("success", false))),
    );
    assert_eq!(report.classification, "checks_green");
    assert_eq!(report.tail_owner, "issue-watcher");
    assert_eq!(report.shepherd_state, "watcher_owned_waiting_for_review");
    assert_eq!(report.next_skill, "human_review");
    assert_eq!(report.continuation, "ask_operator");
    assert_eq!(report.reason, "linked_pr_checks_green_waiting_review");
}

#[test]
fn issue_watch_routes_skipped_checks_to_issue_watcher_owned_review_handoff() {
    let pr = linked_pr(77, false);
    let report = build_issue_watch_report(
        &open_issue(4397),
        false,
        readiness_ready(),
        Some((pr, validation_report("skipped", false))),
    );
    assert_eq!(report.classification, "checks_green");
    assert_eq!(report.tail_owner, "issue-watcher");
    assert_eq!(report.shepherd_state, "watcher_owned_waiting_for_review");
    assert_eq!(report.next_skill, "human_review");
    assert_eq!(report.continuation, "ask_operator");
    assert_eq!(report.reason, "linked_pr_checks_green_waiting_review");
}

#[test]
fn issue_watch_routes_closed_completed_issue_to_closeout() {
    let mut issue = open_issue(4397);
    issue.state = "closed".to_string();
    issue.closed_at = Some("2026-06-22T00:00:00Z".to_string());
    let report = build_issue_watch_report(&issue, true, readiness_ready(), None);
    assert_eq!(report.classification, "closeout_needed");
    assert_eq!(report.tail_owner, "pr-closeout");
    assert_eq!(report.shepherd_state, "closeout_required");
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
    assert_eq!(report.tail_owner, "pr-closeout");
    assert_eq!(report.shepherd_state, "merged_pending_closeout");
    assert_eq!(report.next_skill, "pr-closeout");
    assert_eq!(report.continuation, "action_required");
}

#[test]
fn issue_watch_routes_failed_local_readiness_without_pr_to_blocked() {
    let report = build_issue_watch_report(&open_issue(4397), false, readiness_failed(), None);
    assert_eq!(report.classification, "blocked");
    assert_eq!(report.tail_owner, "pr-ready");
    assert_eq!(report.shepherd_state, "local_readiness_failed");
    assert_eq!(report.next_skill, "pr-ready");
    assert_eq!(
        report.local_readiness.reason,
        "doctor: sor failed validation"
    );
}

#[test]
fn lifecycle_shepherd_maps_ready_without_pr_to_pre_run() {
    let watch = shepherd_report(
        "ready_for_run",
        "pr-run",
        "ready_without_pr",
        "pr-run",
        "continue",
        "issue_ready_without_linked_pr",
        readiness_ready(),
    );
    let report = build_issue_lifecycle_shepherd_report(&watch, "pre_run", "blocked");
    assert!(report.lifecycle_shepherd.active);
    assert_eq!(report.lifecycle_shepherd.state, "pre_run");
    assert_eq!(report.lifecycle_shepherd.owner_skill, "pr-ready");
    assert_eq!(report.lifecycle_shepherd.next_skill, "pr-run");
    assert!(report.lifecycle_shepherd.closeout_required);
    assert!(
        report
            .lifecycle_shepherd
            .authority_boundary
            .merge_authority_human_only
    );
}

#[test]
fn lifecycle_shepherd_maps_ready_run_bound_issue_to_execution_bound() {
    let watch = shepherd_report(
        "ready_for_run",
        "pr-run",
        "ready_without_pr",
        "pr-run",
        "continue",
        "issue_ready_without_linked_pr",
        readiness_ready(),
    );
    let report = build_issue_lifecycle_shepherd_report(&watch, "run_bound", "blocked");
    assert_eq!(report.lifecycle_shepherd.state, "execution_bound");
    assert_eq!(report.lifecycle_shepherd.owner_skill, "pr-run");
    assert_eq!(report.lifecycle_shepherd.next_skill, "pr-run");
}

#[test]
fn lifecycle_shepherd_maps_finish_ready_run_bound_issue_to_publication_ready() {
    let watch = shepherd_report(
        "ready_for_run",
        "pr-run",
        "ready_without_pr",
        "pr-run",
        "continue",
        "issue_ready_without_linked_pr",
        readiness_ready(),
    );
    let report = build_issue_lifecycle_shepherd_report(&watch, "run_bound", "ready");
    assert_eq!(report.lifecycle_shepherd.state, "publication_ready");
    assert_eq!(report.lifecycle_shepherd.owner_skill, "pr-finish");
    assert_eq!(report.lifecycle_shepherd.next_skill, "pr-finish");
}

#[test]
fn lifecycle_shepherd_maps_green_wait_state_to_pr_waiting() {
    let watch = shepherd_report(
        "checks_green",
        "issue-watcher",
        "watcher_owned_waiting_for_review",
        "human_review",
        "ask_operator",
        "linked_pr_checks_green_waiting_review",
        readiness_ready(),
    );
    let report = build_issue_lifecycle_shepherd_report(&watch, "run_bound", "blocked");
    assert_eq!(report.lifecycle_shepherd.state, "pr_waiting");
    assert_eq!(report.lifecycle_shepherd.owner_skill, "issue-watcher");
    assert_eq!(report.lifecycle_shepherd.next_skill, "human_review");
}

#[test]
fn lifecycle_shepherd_maps_failed_checks_to_janitor_active() {
    let watch = shepherd_report(
        "checks_failed",
        "pr-janitor",
        "janitor_owned_checks_failed",
        "pr-janitor",
        "action_required",
        "linked_pr_checks_failed",
        readiness_ready(),
    );
    let report = build_issue_lifecycle_shepherd_report(&watch, "run_bound", "blocked");
    assert_eq!(report.lifecycle_shepherd.state, "janitor_active");
    assert_eq!(report.lifecycle_shepherd.owner_skill, "pr-janitor");
    assert_eq!(report.lifecycle_shepherd.next_skill, "pr-janitor");
}

#[test]
fn lifecycle_shepherd_maps_closeout_needed_to_closed_no_pr() {
    let watch = shepherd_report(
        "closeout_needed",
        "pr-closeout",
        "closeout_required",
        "pr-closeout",
        "action_required",
        "issue_closed_completed",
        readiness_ready(),
    );
    let report = build_issue_lifecycle_shepherd_report(&watch, "run_bound", "blocked");
    assert_eq!(report.lifecycle_shepherd.state, "closed_no_pr");
    assert_eq!(report.lifecycle_shepherd.owner_skill, "pr-closeout");
    assert_eq!(report.lifecycle_shepherd.next_skill, "pr-closeout");
    assert!(report.lifecycle_shepherd.closeout_required);
}

#[test]
fn lifecycle_shepherd_keeps_closed_ready_issue_in_closed_no_pr_until_closeout() {
    let watch = IssueWatchReport {
        issue_state: "CLOSED".to_string(),
        ..shepherd_report(
            "closeout_needed",
            "pr-closeout",
            "closeout_required",
            "pr-closeout",
            "action_required",
            "issue_closed_completed",
            readiness_ready(),
        )
    };
    let report = build_issue_lifecycle_shepherd_report(&watch, "closed", "blocked");
    assert!(report.lifecycle_shepherd.active);
    assert_eq!(report.lifecycle_shepherd.state, "closed_no_pr");
    assert_eq!(report.lifecycle_shepherd.owner_skill, "pr-closeout");
    assert_eq!(report.lifecycle_shepherd.next_skill, "pr-closeout");
    assert!(report.lifecycle_shepherd.closeout_required);
}

#[test]
fn lifecycle_shepherd_preserves_blocked_state_for_failed_local_readiness() {
    let watch = shepherd_report(
        "blocked",
        "pr-ready",
        "local_readiness_failed",
        "pr-ready",
        "action_required",
        "issue_local_readiness_failed",
        readiness_failed(),
    );
    let report = build_issue_lifecycle_shepherd_report(&watch, "unknown", "unknown");
    assert!(report.lifecycle_shepherd.active);
    assert_eq!(report.lifecycle_shepherd.state, "blocked");
    assert_eq!(report.lifecycle_shepherd.owner_skill, "pr-ready");
    assert_eq!(report.lifecycle_shepherd.next_skill, "pr-ready");
}
