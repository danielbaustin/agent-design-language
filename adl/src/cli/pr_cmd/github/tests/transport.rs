use super::*;

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
    let wave =
        unresolved_milestone_pr_wave("owner/repo", "v0.91.5", "tools", None).expect("open PR wave");
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
    assert_eq!(viewed.created_at.as_deref(), Some("2026-06-14T00:00:00Z"));
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

    let wave =
        unresolved_milestone_pr_wave("owner/repo", "v0.91.5", "tools", None).expect("open PR wave");
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

    let snapshot = pr_validation_status_octocrab("owner/repo", "1159").expect("paginated snapshot");
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
fn list_prs_octocrab_paginates_rest_results() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let (base_uri, server) = spawn_open_prs_paginated_server();
    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        std::env::set_var("GITHUB_TOKEN", "test-token");
        std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
    }

    let prs = list_prs_octocrab("owner/repo").expect("paginated PRs");
    assert_eq!(prs.len(), 2);
    assert_eq!(prs[0].number, 2101);
    assert_eq!(prs[1].number, 2102);

    let seen = server.join().expect("server join");
    assert_eq!(seen.len(), 2, "unexpected pagination calls: {seen:#?}");
    assert!(seen[0].contains("/repos/owner/repo/pulls?"));
    assert!(
        seen[0].contains("state=open"),
        "first PR-wave list must request only open PRs: {seen:#?}"
    );
    assert!(seen[1].contains("/repos/owner/repo/pulls?page=2"));

    unsafe {
        std::env::remove_var("ADL_GITHUB_OCTOCRAB_BASE_URI");
    }
    restore_github_policy_env(policy_env);
}

#[test]
fn list_prs_octocrab_fails_closed_on_repeated_next_page_urls() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let (base_uri, server) = spawn_open_prs_repeated_next_server();
    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        std::env::set_var("GITHUB_TOKEN", "test-token");
        std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
    }

    let err = list_prs_octocrab("owner/repo").expect_err("repeated next URL should fail closed");
    assert!(err.to_string().contains("github_client.pagination_loop"));
    assert!(err.to_string().contains("pr.list.wave"));

    let seen = server.join().expect("server join");
    assert_eq!(seen.len(), 2, "unexpected pagination calls: {seen:#?}");
    assert!(seen[0].contains("/repos/owner/repo/pulls?"));
    assert!(
        seen[0].contains("state=open"),
        "repeated-page PR-wave list must request only open PRs: {seen:#?}"
    );
    assert!(seen[1].contains("/repos/owner/repo/pulls?page=2"));

    unsafe {
        std::env::remove_var("ADL_GITHUB_OCTOCRAB_BASE_URI");
    }
    restore_github_policy_env(policy_env);
}

#[test]
fn list_prs_octocrab_times_out_promptly_when_github_stalls() {
    let _guard = env_lock();
    let policy_env = clear_github_policy_env();
    let (base_uri, server) = spawn_open_prs_slow_server(Duration::from_secs(2));
    unsafe {
        std::env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        std::env::set_var("GITHUB_TOKEN", "test-token");
        std::env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", &base_uri);
        std::env::set_var("ADL_GITHUB_OCTOCRAB_TIMEOUT_SECS", "1");
        std::env::set_var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS", "1");
    }

    let err = list_prs_octocrab("owner/repo").expect_err("slow GitHub should time out");
    assert!(err.to_string().contains("github_client.timeout"));
    assert!(err.to_string().contains("pr.list.wave"));

    let seen = server.join().expect("server join");
    assert_eq!(seen.len(), 1, "unexpected slow-server calls: {seen:#?}");
    assert!(
        seen[0].contains("state=open"),
        "slow PR-wave list must request only open PRs: {seen:#?}"
    );

    unsafe {
        std::env::remove_var("ADL_GITHUB_OCTOCRAB_BASE_URI");
        std::env::remove_var("ADL_GITHUB_OCTOCRAB_TIMEOUT_SECS");
        std::env::remove_var("ADL_GITHUB_OCTOCRAB_MAX_ATTEMPTS");
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
    assert!(!octocrab_operation_allows_retry("pr.list.wave"));
    assert!(!octocrab_operation_allows_retry("issue.comment"));
    assert!(!octocrab_operation_allows_retry("issue.create"));
    assert!(!octocrab_operation_allows_retry("pr.create.finish"));
    assert!(!octocrab_operation_allows_retry("pr.merge.finish"));
}
