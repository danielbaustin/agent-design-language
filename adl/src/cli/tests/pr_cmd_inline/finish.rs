use super::*;

#[test]
fn parse_finish_args_requires_title_and_accepts_finish_flags() {
    let err = parse_finish_args(&["1153".to_string()]).expect_err("missing title");
    assert!(err.to_string().contains("--title is required"));

    let parsed = parse_finish_args(&[
        "1153".to_string(),
        "--title".to_string(),
        "Example".to_string(),
        "--paths".to_string(),
        "adl,docs".to_string(),
        "--no-checks".to_string(),
        "--ready".to_string(),
        "--allow-gitignore".to_string(),
        "--no-open".to_string(),
    ])
    .expect("parse finish");
    assert_eq!(parsed.issue, 1153);
    assert_eq!(parsed.title, "Example");
    assert_eq!(parsed.paths, "adl,docs");
    assert!(parsed.no_checks);
    assert!(parsed.ready);
    assert!(parsed.allow_gitignore);
    assert!(parsed.no_open);
}

#[test]
fn render_pr_body_uses_output_sections_and_rejects_issue_template_text() {
    let temp = unique_temp_dir("adl-pr-render-body");
    fs::create_dir_all(&temp).expect("temp dir");
    let input = temp.join("input.md");
    let output = temp.join("output.md");
    fs::write(&input, "# input\n").expect("write input");
    fs::write(
            &output,
            "# rust-finish-test\n\n## Summary\nsummary text\n\n## Artifacts produced\n- adl/src/cli/pr_cmd.rs\n\n## Validation\n- cargo test\n",
        )
        .expect("write output");

    let body = render_pr_body(
        Some("Closes #1153"),
        &input,
        &output,
        Some("extra notes"),
        false,
        "fp-123",
        &temp,
    )
    .expect("render body");
    assert!(body.contains("Closes #1153"));
    assert!(body.contains("## Summary"));
    assert!(body.contains("summary text"));
    assert!(body.contains("## Artifacts"));
    assert!(body.contains("adl/src/cli/pr_cmd.rs"));
    assert!(body.contains("## Validation"));
    assert!(body.contains("## Notes"));
    assert!(body.contains("Idempotency-Key: fp-123"));

    let err = render_pr_body(
        None,
        &input,
        &output,
        Some("issue_card_schema: adl.issue.v1"),
        false,
        "fp-123",
        &temp,
    )
    .expect_err("issue template text should be rejected");
    assert!(err.to_string().contains("issue-template/prompt text"));
}

#[test]
fn real_pr_finish_creates_draft_pr_and_commits_branch_changes() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-create");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::write(repo.join(".gitignore"), ".adl/\n").expect("seed gitignore");
    fs::create_dir_all(repo.join("adl/src")).expect("adl src");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
    assert!(Command::new("git")
        .args(["add", ".gitignore", "adl/src/lib.rs"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    assert!(Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git init bare")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());
    assert!(Command::new("git")
        .args(["checkout", "-q", "-b", "codex/1153-rust-finish-test",])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let issue_ref =
        IssueRef::new(1153, "v0.86".to_string(), "rust-finish-test".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust finish test");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Rust finish test",
        "codex/1153-rust-finish-test",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1153-rust-finish-test");

    fs::write(
        repo.join("adl/src/lib.rs"),
        "pub fn placeholder() {}\npub fn changed() {}\n",
    )
    .expect("write change");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let janitor_log = temp.join("janitor.log");
    let closeout_log = temp.join("closeout.log");
    let gh_path = bin_dir.join("gh");
    let janitor_path = bin_dir.join("janitor");
    let closeout_path = bin_dir.join("closeout");
    write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr create' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1159\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1153\\n'\n  else\n    printf 'Closes #1153\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
                gh_log.display()
            ),
        );
    write_executable(
        &janitor_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\n",
            janitor_log.display()
        ),
    );
    write_executable(
        &closeout_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\n",
            closeout_log.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let old_janitor_cmd = env::var("ADL_PR_JANITOR_CMD").ok();
    let old_janitor_disable = env::var("ADL_PR_JANITOR_DISABLE").ok();
    let old_closeout_cmd = env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    let old_closeout_disable = env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("ADL_PR_JANITOR_DISABLE", "0");
        env::set_var("ADL_PR_JANITOR_CMD", &janitor_path);
        env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
        env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", &closeout_path);
    }
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
        "finish".to_string(),
        "1153".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Rust finish test".to_string(),
        "--paths".to_string(),
        "adl".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &input),
        "--output".to_string(),
        path_relative_to_repo(&repo, &output),
        "--no-checks".to_string(),
        "--no-open".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
        if let Some(value) = old_janitor_cmd {
            env::set_var("ADL_PR_JANITOR_CMD", value);
        } else {
            env::remove_var("ADL_PR_JANITOR_CMD");
        }
        if let Some(value) = old_janitor_disable {
            env::set_var("ADL_PR_JANITOR_DISABLE", value);
        } else {
            env::remove_var("ADL_PR_JANITOR_DISABLE");
        }
        if let Some(value) = old_closeout_cmd {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
        }
        if let Some(value) = old_closeout_disable {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_DISABLE");
        }
    }
    result.expect("real_pr finish");

    let head_subject = run_capture(
        "git",
        &[
            "-C",
            path_str(&repo).expect("repo"),
            "log",
            "-1",
            "--format=%s",
        ],
    )
    .expect("head subject");
    assert!(head_subject.contains("[v0.86][tools] Rust finish test (Closes #1153)"));
    let head_files = run_capture(
        "git",
        &[
            "-C",
            path_str(&repo).expect("repo"),
            "ls-tree",
            "-r",
            "--name-only",
            "HEAD",
        ],
    )
    .expect("head files");
    assert!(!head_files.contains(".adl/v0.86/bodies/issue-1153-rust-finish-test.md"));
    assert!(!head_files.contains(".adl/v0.86/tasks/issue-1153__rust-finish-test/stp.md"));
    assert!(!head_files.contains(".adl/v0.86/tasks/issue-1153__rust-finish-test/sip.md"));
    assert!(!head_files.contains(".adl/v0.86/tasks/issue-1153__rust-finish-test/sor.md"));
    assert!(Command::new("git")
        .args([
            "--git-dir",
            path_str(&origin).expect("origin"),
            "rev-parse",
            "--verify",
            "refs/heads/codex/1153-rust-finish-test",
        ])
        .status()
        .expect("verify pushed branch")
        .success());
    let gh_calls = fs::read_to_string(&gh_log).expect("read gh log");
    let janitor_calls = fs::read_to_string(&janitor_log).expect("read janitor log");
    let closeout_calls = fs::read_to_string(&closeout_log).expect("read closeout log");
    assert!(gh_calls.contains("pr create"));
    assert!(gh_calls.contains("pr view -R danielbaustin/agent-design-language https://github.com/danielbaustin/agent-design-language/pull/1159 --json closingIssuesReferences --jq .closingIssuesReferences[]?.number"));
    assert!(janitor_calls.contains("--issue 1153"));
    assert!(janitor_calls.contains("--branch codex/1153-rust-finish-test"));
    assert!(janitor_calls
        .contains("--pr-url https://github.com/danielbaustin/agent-design-language/pull/1159"));
    assert!(janitor_calls.contains("--expected-pr-state draft"));
    assert!(closeout_calls.contains("--issue 1153"));
    assert!(closeout_calls.contains("--branch codex/1153-rust-finish-test"));
    assert!(closeout_calls
        .contains("--pr-url https://github.com/danielbaustin/agent-design-language/pull/1159"));
}

#[test]
fn real_pr_finish_fails_when_pr_janitor_auto_attach_fails() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-janitor-fail");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::write(repo.join(".gitignore"), ".adl/\n").expect("seed gitignore");
    fs::create_dir_all(repo.join("adl/src")).expect("adl src");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
    assert!(Command::new("git")
        .args(["add", ".gitignore", "adl/src/lib.rs"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    assert!(Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git init bare")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());
    assert!(Command::new("git")
        .args(["checkout", "-q", "-b", "codex/1154-rust-finish-test",])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let issue_ref =
        IssueRef::new(1154, "v0.86".to_string(), "rust-finish-test".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust finish test");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Rust finish test",
        "codex/1154-rust-finish-test",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1154-rust-finish-test");
    fs::write(
        repo.join("adl/src/lib.rs"),
        "pub fn placeholder() {}\npub fn changed() {}\n",
    )
    .expect("write change");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    let janitor_path = bin_dir.join("janitor");
    let closeout_path = bin_dir.join("closeout");
    write_executable(
        &gh_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr create' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1160\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1154\\n'\n  else\n    printf 'Closes #1154\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
    );
    write_executable(
        &janitor_path,
        "#!/usr/bin/env bash\nset -euo pipefail\necho 'janitor attach failed' >&2\nexit 9\n",
    );
    write_executable(
        &closeout_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 0\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let old_janitor_cmd = env::var("ADL_PR_JANITOR_CMD").ok();
    let old_janitor_disable = env::var("ADL_PR_JANITOR_DISABLE").ok();
    let old_closeout_cmd = env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    let old_closeout_disable = env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("ADL_PR_JANITOR_DISABLE", "0");
        env::set_var("ADL_PR_JANITOR_CMD", &janitor_path);
        env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
        env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", &closeout_path);
    }
    env::set_current_dir(&repo).expect("chdir");
    let result = real_pr(&[
        "finish".to_string(),
        "1154".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Rust finish test".to_string(),
        "--paths".to_string(),
        "adl".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &input),
        "--output".to_string(),
        path_relative_to_repo(&repo, &output),
        "--no-checks".to_string(),
        "--no-open".to_string(),
    ]);
    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
        if let Some(value) = old_janitor_cmd {
            env::set_var("ADL_PR_JANITOR_CMD", value);
        } else {
            env::remove_var("ADL_PR_JANITOR_CMD");
        }
        if let Some(value) = old_janitor_disable {
            env::set_var("ADL_PR_JANITOR_DISABLE", value);
        } else {
            env::remove_var("ADL_PR_JANITOR_DISABLE");
        }
        if let Some(value) = old_closeout_cmd {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
        }
        if let Some(value) = old_closeout_disable {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_DISABLE");
        }
    }

    let err = result.expect_err("finish should fail when janitor attach fails");
    assert!(err
        .to_string()
        .contains("finish: PR janitor auto-attach failed"));
}

#[test]
fn real_pr_finish_fails_when_post_merge_closeout_auto_attach_fails() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-closeout-fail");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::create_dir_all(repo.join("adl/src")).expect("adl src");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    assert!(Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git init bare")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());
    assert!(Command::new("git")
        .args(["checkout", "-q", "-b", "codex/1161-rust-finish-test"])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let issue_ref =
        IssueRef::new(1161, "v0.86".to_string(), "rust-finish-test".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust finish test");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Rust finish test",
        "codex/1161-rust-finish-test",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1161-rust-finish-test");
    fs::write(
        repo.join("adl/src/lib.rs"),
        "pub fn placeholder() {}\npub fn changed() {}\n",
    )
    .expect("write change");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    let janitor_path = bin_dir.join("janitor");
    let closeout_path = bin_dir.join("closeout");
    write_executable(
        &gh_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr create' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1161\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1161\\n'\n  else\n    printf 'Closes #1161\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
    );
    write_executable(
        &janitor_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 0\n",
    );
    write_executable(
        &closeout_path,
        "#!/usr/bin/env bash\nset -euo pipefail\necho 'closeout attach failed' >&2\nexit 9\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let old_janitor_cmd = env::var("ADL_PR_JANITOR_CMD").ok();
    let old_janitor_disable = env::var("ADL_PR_JANITOR_DISABLE").ok();
    let old_closeout_cmd = env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    let old_closeout_disable = env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("ADL_PR_JANITOR_DISABLE", "0");
        env::set_var("ADL_PR_JANITOR_CMD", &janitor_path);
        env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
        env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", &closeout_path);
    }
    env::set_current_dir(&repo).expect("chdir");
    let result = real_pr(&[
        "finish".to_string(),
        "1161".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Rust finish test".to_string(),
        "--paths".to_string(),
        "adl".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &input),
        "--output".to_string(),
        path_relative_to_repo(&repo, &output),
        "--no-checks".to_string(),
        "--no-open".to_string(),
    ]);
    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
        if let Some(value) = old_janitor_cmd {
            env::set_var("ADL_PR_JANITOR_CMD", value);
        } else {
            env::remove_var("ADL_PR_JANITOR_CMD");
        }
        if let Some(value) = old_janitor_disable {
            env::set_var("ADL_PR_JANITOR_DISABLE", value);
        } else {
            env::remove_var("ADL_PR_JANITOR_DISABLE");
        }
        if let Some(value) = old_closeout_cmd {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
        }
        if let Some(value) = old_closeout_disable {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_DISABLE");
        }
    }

    let err = result.expect_err("finish should fail when post-merge closeout auto-attach fails");
    assert!(err
        .to_string()
        .contains("finish: post-merge closeout auto-attach failed"));
}

#[test]
fn real_pr_finish_syncs_completed_output_to_root_bundle_and_cards_surface() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-sync-output");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    let worktree = temp.join("worktree");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::create_dir_all(repo.join("adl/src")).expect("adl src");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    assert!(Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git init bare")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());
    assert!(Command::new("git")
        .args([
            "worktree",
            "add",
            "-b",
            "codex/1153-rust-finish-sync",
            path_str(&worktree).expect("worktree"),
            "origin/main",
        ])
        .current_dir(&repo)
        .status()
        .expect("git worktree add")
        .success());

    let issue_ref =
        IssueRef::new(1153, "v0.86".to_string(), "rust-finish-sync".to_string()).expect("ref");
    let root_bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    let wt_bundle_dir = issue_ref.task_bundle_dir_path(&worktree);
    fs::create_dir_all(&root_bundle_dir).expect("root bundle dir");
    fs::create_dir_all(&wt_bundle_dir).expect("wt bundle dir");

    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust finish sync");
    let wt_issue_prompt = issue_ref.issue_prompt_path(&worktree);
    fs::create_dir_all(wt_issue_prompt.parent().expect("wt prompt parent")).expect("wt prompt dir");
    fs::copy(issue_ref.issue_prompt_path(&repo), &wt_issue_prompt).expect("copy issue prompt");

    let root_stp = issue_ref.task_bundle_stp_path(&repo);
    let wt_stp = issue_ref.task_bundle_stp_path(&worktree);
    fs::copy(issue_ref.issue_prompt_path(&repo), &root_stp).expect("seed root stp");
    fs::copy(&wt_issue_prompt, &wt_stp).expect("seed wt stp");

    let root_input = issue_ref.task_bundle_input_path(&repo);
    let wt_input = issue_ref.task_bundle_input_path(&worktree);
    write_authored_sip(
        &root_input,
        &issue_ref,
        "[v0.86][tools] Rust finish sync",
        "codex/1153-rust-finish-sync",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_sip(
        &wt_input,
        &issue_ref,
        "[v0.86][tools] Rust finish sync",
        "codex/1153-rust-finish-sync",
        &wt_issue_prompt,
        &worktree,
    );

    let root_output = issue_ref.task_bundle_output_path(&repo);
    write_output_card(
        &repo,
        &root_output,
        &issue_ref,
        "[v0.86][tools] Rust finish sync",
        "codex/1153-rust-finish-sync",
    )
    .expect("root output");
    let wt_output = issue_ref.task_bundle_output_path(&worktree);
    write_completed_sor_fixture(&wt_output, "codex/1153-rust-finish-sync");

    let cards_root = resolve_cards_root(&repo, None);
    let compat_output = card_output_path(&cards_root, 1153);
    ensure_symlink(&compat_output, &root_output).expect("compat symlink");

    fs::write(
        worktree.join("adl/src/lib.rs"),
        "pub fn placeholder() {}\npub fn changed() {}\n",
    )
    .expect("write change");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let gh_path = bin_dir.join("gh");
    write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr create' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1159\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1153\\n'\n  else\n    printf 'Closes #1153\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
                gh_log.display()
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&worktree).expect("chdir");

    let result = real_pr(&[
        "finish".to_string(),
        "1153".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Rust finish sync".to_string(),
        "--paths".to_string(),
        "adl".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&worktree, &wt_input),
        "--output".to_string(),
        path_relative_to_repo(&worktree, &wt_output),
        "--no-checks".to_string(),
        "--no-open".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    result.expect("real_pr finish");

    let root_text = fs::read_to_string(&root_output).expect("root output text");
    let compat_text = fs::read_to_string(&compat_output).expect("compat output text");
    assert!(root_text.contains("Status: DONE"));
    assert!(root_text.contains("codex/1153-rust-finish-sync"));
    assert_eq!(root_text, compat_text);
}

#[test]
fn real_pr_finish_refuses_to_publish_local_only_bundle_without_tracked_changes() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-ignored-bundle-only");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::create_dir_all(repo.join("adl/src")).expect("adl src");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    assert!(Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git init bare")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());
    assert!(Command::new("git")
        .args([
            "checkout",
            "-q",
            "-b",
            "codex/1157-rust-finish-ignored-bundle"
        ])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let issue_ref = IssueRef::new(
        1157,
        "v0.86".to_string(),
        "rust-finish-ignored-bundle".to_string(),
    )
    .expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    write_authored_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.86][tools] Rust finish ignored bundle",
    );
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Rust finish ignored bundle",
        "codex/1157-rust-finish-ignored-bundle",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1157-rust-finish-ignored-bundle");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr create' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1160\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1157\\n'\n  else\n    printf 'Closes #1157\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
            gh_log.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
        "finish".to_string(),
        "1157".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Rust finish ignored bundle".to_string(),
        "--paths".to_string(),
        "adl".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &input),
        "--output".to_string(),
        path_relative_to_repo(&repo, &output),
        "--no-checks".to_string(),
        "--no-open".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    let err = result
        .expect_err("finish should reject local-only bundle publication without tracked changes");
    assert!(err
        .to_string()
        .contains("No changes detected and branch has no commits ahead of origin/main"));
    let head_files = run_capture(
        "git",
        &[
            "-C",
            path_str(&repo).expect("repo"),
            "ls-tree",
            "-r",
            "--name-only",
            "HEAD",
        ],
    )
    .expect("head files");
    assert!(!head_files.contains(".adl/v0.86/bodies/issue-1157-rust-finish-ignored-bundle.md"));
    assert!(!head_files.contains(".adl/v0.86/tasks/issue-1157__rust-finish-ignored-bundle/stp.md"));
    assert!(!head_files.contains(".adl/v0.86/tasks/issue-1157__rust-finish-ignored-bundle/sip.md"));
    assert!(!head_files.contains(".adl/v0.86/tasks/issue-1157__rust-finish-ignored-bundle/sor.md"));
    let gh_calls = fs::read_to_string(&gh_log).expect("read gh log");
    assert!(!gh_calls.contains("pr create"));
}

#[test]
fn real_pr_finish_refuses_when_canonical_issue_surfaces_are_tracked() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-tracked-canonical-surfaces");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::create_dir_all(repo.join("adl/src")).expect("adl src");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    assert!(Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git init bare")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());
    assert!(Command::new("git")
        .args([
            "checkout",
            "-q",
            "-b",
            "codex/1158-rust-finish-tracked-bundle"
        ])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let issue_ref =
        IssueRef::new(1158, "v0.86".to_string(), "rust-finish-tracked-bundle").expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    write_authored_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.86][tools] Rust finish tracked bundle",
    );
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Rust finish tracked bundle",
        "codex/1158-rust-finish-tracked-bundle",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1158-rust-finish-tracked-bundle");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add tracked bundle")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "track local-only bundle"])
        .current_dir(&repo)
        .status()
        .expect("git commit tracked bundle")
        .success());

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  exit 0\nfi\nexit 1\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
        "finish".to_string(),
        "1158".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Rust finish tracked bundle".to_string(),
        "--paths".to_string(),
        "adl".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &input),
        "--output".to_string(),
        path_relative_to_repo(&repo, &output),
        "--no-checks".to_string(),
        "--no-open".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    let err = result.expect_err("finish should refuse tracked canonical issue surfaces");
    assert!(err
        .to_string()
        .contains("canonical .adl issue surfaces must remain local-only"));
}

#[test]
fn real_pr_finish_accepts_primary_checkout_issue_prompt_without_worktree_local_copy() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-primary-prompt");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    let worktree = temp.join("worktree");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::create_dir_all(repo.join("adl/src")).expect("adl src");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    assert!(Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git init bare")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());
    assert!(Command::new("git")
        .args([
            "worktree",
            "add",
            "-b",
            "codex/1241-finish-primary-prompt",
            path_str(&worktree).expect("worktree"),
            "origin/main",
        ])
        .current_dir(&repo)
        .status()
        .expect("git worktree add")
        .success());

    let issue_ref = IssueRef::new(
        1241,
        "v0.86".to_string(),
        "finish-primary-prompt".to_string(),
    )
    .expect("ref");
    let root_bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    let wt_bundle_dir = issue_ref.task_bundle_dir_path(&worktree);
    fs::create_dir_all(&root_bundle_dir).expect("root bundle dir");
    fs::create_dir_all(&wt_bundle_dir).expect("wt bundle dir");

    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Finish primary prompt");

    let root_stp = issue_ref.task_bundle_stp_path(&repo);
    let wt_stp = issue_ref.task_bundle_stp_path(&worktree);
    fs::copy(issue_ref.issue_prompt_path(&repo), &root_stp).expect("seed root stp");
    fs::copy(&root_stp, &wt_stp).expect("seed wt stp");

    let root_input = issue_ref.task_bundle_input_path(&repo);
    let wt_input = issue_ref.task_bundle_input_path(&worktree);
    write_authored_sip(
        &root_input,
        &issue_ref,
        "[v0.86][tools] Finish primary prompt",
        "codex/1241-finish-primary-prompt",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_sip(
        &wt_input,
        &issue_ref,
        "[v0.86][tools] Finish primary prompt",
        "codex/1241-finish-primary-prompt",
        &issue_ref.issue_prompt_path(&repo),
        &worktree,
    );

    let root_output = issue_ref.task_bundle_output_path(&repo);
    write_output_card(
        &repo,
        &root_output,
        &issue_ref,
        "[v0.86][tools] Finish primary prompt",
        "codex/1241-finish-primary-prompt",
    )
    .expect("root output");
    let wt_output = issue_ref.task_bundle_output_path(&worktree);
    write_completed_sor_fixture(&wt_output, "codex/1241-finish-primary-prompt");

    let cards_root = resolve_cards_root(&repo, None);
    let compat_output = card_output_path(&cards_root, 1241);
    ensure_symlink(&compat_output, &root_output).expect("compat symlink");

    fs::write(
        worktree.join("adl/src/lib.rs"),
        "pub fn placeholder() {}\npub fn changed() {}\n",
    )
    .expect("write change");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let gh_path = bin_dir.join("gh");
    write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr create' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1241\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1241\\n'\n  else\n    printf 'Closes #1241\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
                gh_log.display()
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&worktree).expect("chdir");

    let result = real_pr(&[
        "finish".to_string(),
        "1241".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Finish primary prompt".to_string(),
        "--paths".to_string(),
        "adl".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&worktree, &wt_input),
        "--output".to_string(),
        path_relative_to_repo(&worktree, &wt_output),
        "--no-checks".to_string(),
        "--no-open".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    result.expect("real_pr finish");

    let gh_calls = fs::read_to_string(&gh_log).expect("read gh log");
    assert!(gh_calls.contains("pr create"));
    assert!(!issue_ref.issue_prompt_path(&worktree).exists());
    let root_text = fs::read_to_string(&root_output).expect("root output text");
    assert!(root_text.contains("Status: DONE"));
}

#[test]
fn real_pr_finish_updates_existing_pr_and_marks_ready() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-edit");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::create_dir_all(repo.join("adl/src")).expect("adl src");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    assert!(Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git init bare")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());
    assert!(Command::new("git")
        .args(["checkout", "-q", "-b", "codex/1153-rust-finish-test-edit",])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let issue_ref = IssueRef::new(
        1153,
        "v0.86".to_string(),
        "rust-finish-test-edit".to_string(),
    )
    .expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust finish test edit");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Rust finish test edit",
        "codex/1153-rust-finish-test-edit",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1153-rust-finish-test-edit");

    fs::write(
        repo.join("adl/src/lib.rs"),
        "pub fn placeholder() -> usize { 1 }\n",
    )
    .expect("write change");
    assert!(Command::new("git")
        .args(["add", "adl/src/lib.rs"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "existing branch commit"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let gh_path = bin_dir.join("gh");
    write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1160\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr edit' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr ready' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1153\\n'\n  else\n    printf 'Closes #1153\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
                gh_log.display()
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
        "finish".to_string(),
        "1153".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Rust finish test edit".to_string(),
        "--paths".to_string(),
        "adl".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &input),
        "--output".to_string(),
        path_relative_to_repo(&repo, &output),
        "--no-checks".to_string(),
        "--ready".to_string(),
        "--no-open".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    result.expect("real_pr finish edit");

    let gh_calls = fs::read_to_string(&gh_log).expect("read gh log");
    assert!(gh_calls.contains("pr edit -R danielbaustin/agent-design-language https://github.com/danielbaustin/agent-design-language/pull/1160 --title [v0.86][tools] Rust finish test edit --body-file"));
    assert!(gh_calls.contains("pr ready -R danielbaustin/agent-design-language https://github.com/danielbaustin/agent-design-language/pull/1160"));
}

#[test]
fn finish_helper_paths_cover_nonempty_and_staged_checks() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-finish-helpers");
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::write(repo.join("tracked.txt"), "base\n").expect("write base");
    assert!(Command::new("git")
        .args(["add", "tracked.txt"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());

    let missing = repo.join("missing.md");
    let empty = repo.join("empty.md");
    let filled = repo.join("filled.md");
    fs::write(&empty, " \n").expect("write empty");
    fs::write(&filled, "content\n").expect("write filled");
    assert!(!ensure_nonempty_file_path(&missing).expect("missing ok"));
    assert!(!ensure_nonempty_file_path(&empty).expect("empty ok"));
    assert!(ensure_nonempty_file_path(&filled).expect("filled ok"));

    assert!(!has_uncommitted_changes(&repo).expect("clean"));
    fs::write(repo.join("tracked.txt"), "changed\n").expect("modify tracked");
    assert!(has_uncommitted_changes(&repo).expect("dirty"));

    stage_selected_paths_rust(&repo, "tracked.txt").expect("stage");
    assert!(!staged_diff_is_empty(&repo).expect("staged diff"));
    assert!(!staged_gitignore_change_present(&repo).expect("no gitignore"));

    fs::write(repo.join(".gitignore"), "target\n").expect("write gitignore");
    stage_selected_paths_rust(&repo, ".gitignore").expect("stage gitignore");
    assert!(staged_gitignore_change_present(&repo).expect("gitignore change"));

    let ignored_dir = repo.join(".adl").join("v0.86").join("tasks");
    fs::create_dir_all(&ignored_dir).expect("ignored dir");
    let ignored_file = ignored_dir
        .join("issue-1153__rust-finish-test")
        .join("sor.md");
    fs::create_dir_all(ignored_file.parent().expect("ignored file parent"))
        .expect("ignored parent");
    fs::write(&ignored_file, "ignored output\n").expect("ignored output");
    fs::write(repo.join(".gitignore"), ".adl/\ntarget\n").expect("write ignore rules");
    stage_selected_paths_rust(&repo, "tracked.txt").expect("stage tracked file only");
    let staged_name_only = run_capture(
        "git",
        &[
            "-C",
            path_str(&repo).expect("repo"),
            "diff",
            "--cached",
            "--name-only",
        ],
    )
    .expect("cached names");
    assert!(!staged_name_only.contains(".adl/v0.86/tasks/issue-1153__rust-finish-test/sor.md"));
    assert!(staged_name_only.contains("tracked.txt"));
}

#[test]
fn finish_helper_paths_cover_ahead_count_and_batch_checks() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-batch-checks");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(repo.join("adl/tools")).expect("adl tools dir");
    fs::write(
        repo.join("adl/Cargo.toml"),
        "[package]\nname='adl'\nversion='0.1.0'\n",
    )
    .expect("cargo toml");
    write_executable(
        &repo.join("adl/tools/check_no_tracked_adl_issue_record_residue.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 0\n",
    );
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::write(repo.join("README.md"), "base\n").expect("readme");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git init bare")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());
    assert_eq!(commits_ahead_of_origin_main(&repo).expect("ahead count"), 0);

    fs::write(repo.join("README.md"), "ahead\n").expect("modify");
    assert!(Command::new("git")
        .args(["add", "README.md"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "ahead"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert_eq!(commits_ahead_of_origin_main(&repo).expect("ahead count"), 1);

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let cargo_log = temp.join("cargo.log");
    let cargo_path = bin_dir.join("cargo");
    write_executable(
        &cargo_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nexit 0\n",
            cargo_log.display()
        ),
    );
    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    run_batched_checks_rust(&repo).expect("batch checks");
    unsafe {
        env::set_var("PATH", old_path);
    }

    let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
    assert!(cargo_calls.contains("fmt --manifest-path"));
    assert!(cargo_calls.contains("clippy --manifest-path"));
    assert!(cargo_calls.contains("test --manifest-path"));
}

#[test]
fn finish_helper_paths_cover_pr_lookup_and_closing_linkage() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-gh-helpers");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let gh_path = bin_dir.join("gh");
    write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2\" = 'pr list' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1159\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1153\\n'\n  else\n    printf 'Closes #1153\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
                gh_log.display()
            ),
        );
    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let pr = current_pr_url(
        "danielbaustin/agent-design-language",
        "codex/1153-rust-finish-test",
    )
    .expect("pr url");
    assert_eq!(
        pr.as_deref(),
        Some("https://github.com/danielbaustin/agent-design-language/pull/1159")
    );
    assert!(pr_has_closing_linkage(
        "danielbaustin/agent-design-language",
        "https://github.com/danielbaustin/agent-design-language/pull/1159",
        1153
    )
    .expect("closing linkage"));
    ensure_pr_closing_linkage(
        "danielbaustin/agent-design-language",
        "https://github.com/danielbaustin/agent-design-language/pull/1159",
        1153,
        false,
    )
    .expect("ensure linkage");
    ensure_pr_closing_linkage(
        "danielbaustin/agent-design-language",
        "https://github.com/danielbaustin/agent-design-language/pull/1159",
        1153,
        true,
    )
    .expect("no close skips");

    unsafe {
        env::set_var("PATH", old_path);
    }
    let gh_calls = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_calls.contains("pr list -R danielbaustin/agent-design-language --head codex/1153-rust-finish-test --state open --json url --jq .[0].url"));
}

#[test]
fn real_pr_finish_rejects_main_and_reports_no_pr_when_only_local_bundle_sync_changes_exist() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-errors");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::write(repo.join(".gitignore"), ".adl/\n").expect("seed gitignore");
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    fs::create_dir_all(repo.join("adl/src")).expect("adl src");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
    let issue_ref =
        IssueRef::new(1153, "v0.86".to_string(), "rust-finish-test".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    fs::write(issue_ref.task_bundle_input_path(&repo), "# input\n").expect("input");
    write_completed_sor_fixture(
        &issue_ref.task_bundle_output_path(&repo),
        "codex/1153-rust-finish-test",
    );
    assert!(Command::new("git")
        .args(["add", ".gitignore", "adl/src/lib.rs"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init on main"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git init bare")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());
    let issue_ref =
        IssueRef::new(1153, "v0.86".to_string(), "rust-finish-test".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "Example");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "Example",
        "codex/1153-rust-finish-test",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1153-rust-finish-test");
    let cards_root = resolve_cards_root(&repo, None);
    let compat_output = card_output_path(&cards_root, 1153);
    ensure_symlink(&compat_output, &output).expect("compat symlink");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "seed finish bundle"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let main_err = real_pr(&[
        "finish".to_string(),
        "1153".to_string(),
        "--title".to_string(),
        "Example".to_string(),
        "--no-checks".to_string(),
        "--no-open".to_string(),
    ])
    .expect_err("main should be rejected");
    assert!(main_err.to_string().contains("refusing to run on main"));
    env::set_current_dir(prev_dir).expect("restore cwd");

    assert!(Command::new("git")
        .args(["checkout", "-q", "-b", "codex/1153-rust-finish-test"])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());
    let mut touched_output = fs::read_to_string(&output).expect("read output");
    touched_output.push('\n');
    fs::write(&output, touched_output).expect("touch output");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        "#!/usr/bin/env bash\nset -euo pipefail\ncmd=\"$(printf '%s ' \"$@\")\"\nif printf '%s' \"$cmd\" | grep -q 'repo view'; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif printf '%s' \"$cmd\" | grep -q 'pr list'; then\n  exit 0\nfi\nif printf '%s' \"$cmd\" | grep -q 'pr view'; then\n  if printf '%s' \"$cmd\" | grep -q 'closingIssuesReferences'; then\n    printf '1153\\n'\n  else\n    printf 'Closes #1153\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
    );
    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");
    let bundle_sync_err = real_pr(&[
        "finish".to_string(),
        "1153".to_string(),
        "--title".to_string(),
        "Example".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &input),
        "--output".to_string(),
        path_relative_to_repo(&repo, &output),
        "--no-checks".to_string(),
        "--no-open".to_string(),
    ])
    .expect_err("expected downstream gh fixture failure after bundle sync work is detected");
    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    assert!(bundle_sync_err.to_string().contains("Nothing to PR."));
}

#[test]
fn ensure_or_repair_pr_closing_linkage_repairs_live_pr_body() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-repair-linkage");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let state_body = temp.join("pr_body.txt");
    fs::write(&state_body, "Refs #1153\n").expect("seed body");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    if grep -q 'Closes #1153' '{}'; then\n      printf '1153\\n'\n    fi\n    exit 0\n  fi\n  if printf '%s ' \"$@\" | grep -q ' --json body '; then\n    cat '{}'\n    exit 0\n  fi\nfi\nif [ \"$1 $2\" = 'pr edit' ]; then\n  body_file=''\n  while [ $# -gt 0 ]; do\n    if [ \"$1\" = '--body-file' ]; then\n      body_file=\"$2\"\n      shift 2\n    else\n      shift\n    fi\n  done\n  cp \"$body_file\" '{}'\n  exit 0\nfi\nexit 1\n",
            gh_log.display(),
            state_body.display(),
            state_body.display(),
            state_body.display()
        ),
    );

    let body_file = temp.join("desired.md");
    fs::write(&body_file, "Closes #1153\n\n## Summary\nrepaired\n").expect("desired body");

    let old_path = env::var("PATH").unwrap_or_default();
    let old_entries = env::split_paths(&old_path).collect::<Vec<_>>();
    let mut new_entries = vec![bin_dir.clone()];
    new_entries.extend(old_entries);
    unsafe {
        env::set_var("PATH", env::join_paths(new_entries).expect("join PATH"));
    }

    ensure_or_repair_pr_closing_linkage(
        "danielbaustin/agent-design-language",
        "https://github.com/danielbaustin/agent-design-language/pull/1159",
        1153,
        false,
        &body_file,
    )
    .expect("repair should succeed");

    unsafe {
        env::set_var("PATH", old_path);
    }

    let repaired = fs::read_to_string(&state_body).expect("read repaired body");
    assert!(repaired.contains("Closes #1153"));
    let gh_calls = fs::read_to_string(&gh_log).expect("read gh log");
    assert!(gh_calls.contains("pr edit -R danielbaustin/agent-design-language https://github.com/danielbaustin/agent-design-language/pull/1159 --body-file"));
}

#[test]
fn real_pr_finish_rejects_staged_gitignore_changes_without_allow_flag() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-gitignore-guard");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::write(repo.join(".gitignore"), ".adl/\n").expect("seed gitignore");
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    fs::create_dir_all(repo.join("adl/src")).expect("adl src");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
    let issue_ref =
        IssueRef::new(1153, "v0.86".to_string(), "rust-finish-test".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    write_authored_issue_prompt(&repo, &issue_ref, "Example");
    fs::copy(
        issue_ref.issue_prompt_path(&repo),
        issue_ref.task_bundle_stp_path(&repo),
    )
    .expect("seed stp");
    write_authored_sip(
        &issue_ref.task_bundle_input_path(&repo),
        &issue_ref,
        "Example",
        "codex/1153-rust-finish-test",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_completed_sor_fixture(
        &issue_ref.task_bundle_output_path(&repo),
        "codex/1153-rust-finish-test",
    );
    let cards_root = resolve_cards_root(&repo, None);
    let compat_output = card_output_path(&cards_root, 1153);
    ensure_symlink(&compat_output, &issue_ref.task_bundle_output_path(&repo))
        .expect("compat symlink");
    assert!(Command::new("git")
        .args(["add", ".gitignore", "adl/src/lib.rs"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git init bare")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());
    assert!(Command::new("git")
        .args(["checkout", "-q", "-b", "codex/1153-rust-finish-test"])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());
    fs::write(repo.join(".gitignore"), ".adl/\ntarget\n").expect("write gitignore");
    assert!(Command::new("git")
        .args(["add", ".gitignore"])
        .current_dir(&repo)
        .status()
        .expect("git add gitignore")
        .success());

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let err = real_pr(&[
        "finish".to_string(),
        "1153".to_string(),
        "--title".to_string(),
        "Example".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &issue_ref.task_bundle_input_path(&repo)),
        "--output".to_string(),
        path_relative_to_repo(&repo, &issue_ref.task_bundle_output_path(&repo)),
        "--no-checks".to_string(),
        "--no-open".to_string(),
    ])
    .expect_err("gitignore guard should block finish");
    env::set_current_dir(prev_dir).expect("restore cwd");
    assert!(err
        .to_string()
        .contains("staged .gitignore or adl/.gitignore changes detected"));
    assert!(err
        .to_string()
        .contains("Canonical issue bundle files are staged automatically"));
}

#[test]
fn real_pr_finish_rejects_not_started_output_card_before_publication() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-not-started");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::create_dir_all(repo.join("adl/src")).expect("adl src");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    assert!(Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git init bare")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());
    assert!(Command::new("git")
        .args(["checkout", "-q", "-b", "codex/1156-output-card-guard"])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let issue_ref =
        IssueRef::new(1156, "v0.86".to_string(), "output-card-guard".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Output card guard");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Output card guard",
        "codex/1156-output-card-guard",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    fs::write(
        &output,
        r#"# output-card-guard

Task ID: issue-1156
Run ID: issue-1156
Version: v0.86
Title: output-card-guard
Branch: codex/1156-output-card-guard
Status: NOT_STARTED
"#,
    )
    .expect("write output");
    fs::write(
        repo.join("adl/src/lib.rs"),
        "pub fn placeholder() {}\npub fn changed() {}\n",
    )
    .expect("write change");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nexit 0\n",
            gh_log.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "finish".to_string(),
        "1156".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Output card guard".to_string(),
        "--paths".to_string(),
        "adl".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &input),
        "--output".to_string(),
        path_relative_to_repo(&repo, &output),
        "--no-checks".to_string(),
        "--no-open".to_string(),
    ])
    .expect_err("NOT_STARTED output card should be rejected");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    assert!(err
        .to_string()
        .contains("output card is still bootstrap state (Status: NOT_STARTED)"));
    let gh_calls = fs::read_to_string(&gh_log).unwrap_or_default();
    assert!(
        !gh_calls.contains("pr create") && !gh_calls.contains("pr edit"),
        "finish should fail before any PR publication call"
    );
}

#[test]
fn real_pr_finish_rejects_closed_issue_with_stale_canonical_sor_truth() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-closed-stale-sor");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_bootstrap_support_files(&repo);
    init_git_repo(&repo);
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config")
        .success());
    fs::create_dir_all(repo.join("adl/src")).expect("adl src");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    assert!(Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git init bare")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());
    assert!(Command::new("git")
        .args([
            "checkout",
            "-q",
            "-b",
            "codex/1158-rust-finish-closed-stale"
        ])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let issue_ref = IssueRef::new(
        1158,
        "v0.86".to_string(),
        "rust-finish-closed-stale".to_string(),
    )
    .expect("ref");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust finish closed stale");
    if let Some(parent) = stp.parent() {
        fs::create_dir_all(parent).expect("bundle dir");
    }
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Rust finish closed stale",
        "codex/1158-rust-finish-closed-stale",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1158-rust-finish-closed-stale");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    write_executable(
        &bin_dir.join("gh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$1 $2 $3 $4\" == \"issue view 1158 -R\" ]]; then\n  printf '{{\"state\":\"CLOSED\",\"stateReason\":\"COMPLETED\"}}\\n'\n  exit 0\nfi\nexit 1\n",
            gh_log.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "finish".to_string(),
        "1158".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Rust finish closed stale".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &input),
        "--output".to_string(),
        path_relative_to_repo(&repo, &output),
        "--no-checks".to_string(),
        "--no-open".to_string(),
    ])
    .expect_err("finish should reject stale closed issue sor truth");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    let rendered = err.to_string();
    assert!(rendered.contains("finish: closed issue #1158 has stale canonical sor truth"));
    let gh_calls = fs::read_to_string(&gh_log).expect("gh log");
    assert!(
        !gh_calls.contains("pr create") && !gh_calls.contains("pr edit"),
        "finish should fail before any PR publication call"
    );
}
