use super::*;

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
        .args(["add", "."])
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
    let plan = issue_ref.task_bundle_plan_path(&repo);
    let review_policy = issue_ref.task_bundle_review_policy_path(&repo);
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
    write_authored_spp(
        &plan,
        &issue_ref,
        "[v0.86][tools] Rust finish test",
        "codex/1153-rust-finish-test",
        &repo,
    );
    write_authored_srp(
        &review_policy,
        &issue_ref,
        "[v0.86][tools] Rust finish test",
        "codex/1153-rust-finish-test",
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
    let watcher_log = temp.join("watcher.log");
    let closeout_log = temp.join("closeout.log");
    let open_log = temp.join("open.log");
    let gh_path = bin_dir.join("gh");
    let janitor_path = bin_dir.join("janitor");
    let watcher_path = bin_dir.join("watcher");
    let closeout_path = bin_dir.join("closeout");
    let open_path = bin_dir.join("open");
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
        &watcher_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\n",
            watcher_log.display()
        ),
    );
    write_executable(
        &closeout_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\n",
            closeout_log.display()
        ),
    );
    write_executable(
        &open_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\necho 'No application knows how to open URL' >&2\nexit 42\n",
            open_log.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let old_janitor_cmd = env::var("ADL_PR_JANITOR_CMD").ok();
    let old_janitor_disable = env::var("ADL_PR_JANITOR_DISABLE").ok();
    let old_watcher_cmd = env::var("ADL_ISSUE_WATCHER_CMD").ok();
    let old_watcher_disable = env::var("ADL_ISSUE_WATCHER_DISABLE").ok();
    let old_closeout_cmd = env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    let old_closeout_disable = env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("ADL_PR_JANITOR_DISABLE", "0");
        env::set_var("ADL_PR_JANITOR_CMD", &janitor_path);
        env::set_var("ADL_ISSUE_WATCHER_DISABLE", "0");
        env::set_var("ADL_ISSUE_WATCHER_CMD", &watcher_path);
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
        if let Some(value) = old_watcher_cmd {
            env::set_var("ADL_ISSUE_WATCHER_CMD", value);
        } else {
            env::remove_var("ADL_ISSUE_WATCHER_CMD");
        }
        if let Some(value) = old_watcher_disable {
            env::set_var("ADL_ISSUE_WATCHER_DISABLE", value);
        } else {
            env::remove_var("ADL_ISSUE_WATCHER_DISABLE");
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
    let watcher_calls = fs::read_to_string(&watcher_log).expect("read watcher log");
    let closeout_calls = fs::read_to_string(&closeout_log).expect("read closeout log");
    let open_calls = fs::read_to_string(&open_log).expect("read open log");
    assert!(gh_calls.contains("pr create"));
    assert!(gh_calls.contains("pr view"));
    assert!(gh_calls.contains("danielbaustin/agent-design-language"));
    assert!(gh_calls.contains("https://github.com/danielbaustin/agent-design-language/pull/1159"));
    assert!(gh_calls.contains("closingIssuesReferences"));
    assert!(janitor_calls.contains("--issue 1153"));
    assert!(janitor_calls.contains("--branch codex/1153-rust-finish-test"));
    assert!(janitor_calls
        .contains("--pr-url https://github.com/danielbaustin/agent-design-language/pull/1159"));
    assert!(janitor_calls.contains("--expected-pr-state draft"));
    assert!(watcher_calls.contains("--issue 1153"));
    assert!(watcher_calls.contains("--branch codex/1153-rust-finish-test"));
    assert!(watcher_calls
        .contains("--pr-url https://github.com/danielbaustin/agent-design-language/pull/1159"));
    assert!(watcher_calls.contains("--expected-pr-state draft"));
    assert!(watcher_calls.contains("--classification pr_open"));
    assert!(watcher_calls.contains("--tail-owner issue-watcher"));
    assert!(watcher_calls.contains("--shepherd-state watcher_owned_pr_open"));
    assert!(closeout_calls.contains("--issue 1153"));
    assert!(closeout_calls.contains("--branch codex/1153-rust-finish-test"));
    assert!(open_calls.contains("https://github.com/danielbaustin/agent-design-language/pull/1159"));
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
        .args(["add", "."])
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
    let plan = issue_ref.task_bundle_plan_path(&repo);
    let review_policy = issue_ref.task_bundle_review_policy_path(&repo);
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
    write_authored_spp(
        &plan,
        &issue_ref,
        "[v0.86][tools] Rust finish test",
        "codex/1154-rust-finish-test",
        &repo,
    );
    write_authored_srp(
        &review_policy,
        &issue_ref,
        "[v0.86][tools] Rust finish test",
        "codex/1154-rust-finish-test",
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
    let watcher_path = bin_dir.join("watcher");
    let closeout_path = bin_dir.join("closeout");
    let watcher_log = temp.join("watcher.log");
    let pr_body_capture = temp.join("pr-body.md");
    write_executable(
        &gh_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr create' ]; then\n  while [ \"$#\" -gt 0 ]; do\n    if [ \"$1\" = '--body-file' ]; then\n      cp \"$2\" '{}'\n      break\n    fi\n    shift\n  done\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1160\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1154\\n'\n  else\n    printf 'Closes #1154\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
            pr_body_capture.display()
        ),
    );
    write_executable(
        &janitor_path,
        "#!/usr/bin/env bash\nset -euo pipefail\necho 'janitor attach failed' >&2\nexit 9\n",
    );
    write_executable(
        &watcher_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\n",
            watcher_log.display()
        ),
    );
    write_executable(
        &closeout_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 0\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let old_janitor_cmd = env::var("ADL_PR_JANITOR_CMD").ok();
    let old_janitor_disable = env::var("ADL_PR_JANITOR_DISABLE").ok();
    let old_watcher_cmd = env::var("ADL_ISSUE_WATCHER_CMD").ok();
    let old_watcher_disable = env::var("ADL_ISSUE_WATCHER_DISABLE").ok();
    let old_closeout_cmd = env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    let old_closeout_disable = env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("ADL_PR_JANITOR_DISABLE", "0");
        env::set_var("ADL_PR_JANITOR_CMD", &janitor_path);
        env::set_var("ADL_ISSUE_WATCHER_DISABLE", "0");
        env::set_var("ADL_ISSUE_WATCHER_CMD", &watcher_path);
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
        "--no-close".to_string(),
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
        if let Some(value) = old_watcher_cmd {
            env::set_var("ADL_ISSUE_WATCHER_CMD", value);
        } else {
            env::remove_var("ADL_ISSUE_WATCHER_CMD");
        }
        if let Some(value) = old_watcher_disable {
            env::set_var("ADL_ISSUE_WATCHER_DISABLE", value);
        } else {
            env::remove_var("ADL_ISSUE_WATCHER_DISABLE");
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
    let captured_body = fs::read_to_string(&pr_body_capture).expect("captured PR body");
    assert!(captured_body.contains("Non-closing lifecycle PR"));
    assert!(captured_body.contains("issue #1154 remains open"));
    assert!(!captured_body.contains("Closes #1154"));
    assert!(
        !watcher_log.exists(),
        "watcher should not launch when janitor attach fails first"
    );
}

#[test]
fn real_pr_finish_warns_but_succeeds_when_issue_watcher_auto_attach_fails() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-watcher-fail");
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
        .args(["add", "."])
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
        .args(["checkout", "-q", "-b", "codex/1162-rust-finish-test"])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let issue_ref =
        IssueRef::new(1162, "v0.86".to_string(), "rust-finish-test".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    let plan = issue_ref.task_bundle_plan_path(&repo);
    let review_policy = issue_ref.task_bundle_review_policy_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust finish test");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Rust finish test",
        "codex/1162-rust-finish-test",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_spp(
        &plan,
        &issue_ref,
        "[v0.86][tools] Rust finish test",
        "codex/1162-rust-finish-test",
        &repo,
    );
    write_authored_srp(
        &review_policy,
        &issue_ref,
        "[v0.86][tools] Rust finish test",
        "codex/1162-rust-finish-test",
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1162-rust-finish-test");
    fs::write(
        repo.join("adl/src/lib.rs"),
        "pub fn placeholder() {}\npub fn changed() {}\n",
    )
    .expect("write change");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let gh_path = bin_dir.join("gh");
    let janitor_path = bin_dir.join("janitor");
    let watcher_path = bin_dir.join("watcher");
    let closeout_path = bin_dir.join("closeout");
    write_executable(
        &gh_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr create' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1162\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1162\\n'\n  else\n    printf 'Closes #1162\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
            gh_log.display()
        ),
    );
    write_executable(
        &janitor_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 0\n",
    );
    write_executable(
        &watcher_path,
        "#!/usr/bin/env bash\nset -euo pipefail\necho 'watcher attach failed' >&2\nexit 9\n",
    );
    write_executable(
        &closeout_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 0\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let old_janitor_cmd = env::var("ADL_PR_JANITOR_CMD").ok();
    let old_janitor_disable = env::var("ADL_PR_JANITOR_DISABLE").ok();
    let old_watcher_cmd = env::var("ADL_ISSUE_WATCHER_CMD").ok();
    let old_watcher_disable = env::var("ADL_ISSUE_WATCHER_DISABLE").ok();
    let old_closeout_cmd = env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    let old_closeout_disable = env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("ADL_PR_JANITOR_DISABLE", "0");
        env::set_var("ADL_PR_JANITOR_CMD", &janitor_path);
        env::set_var("ADL_ISSUE_WATCHER_DISABLE", "0");
        env::set_var("ADL_ISSUE_WATCHER_CMD", &watcher_path);
        env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
        env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", &closeout_path);
    }
    env::set_current_dir(&repo).expect("chdir");
    let result = real_pr(&[
        "finish".to_string(),
        "1162".to_string(),
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
        if let Some(value) = old_watcher_cmd {
            env::set_var("ADL_ISSUE_WATCHER_CMD", value);
        } else {
            env::remove_var("ADL_ISSUE_WATCHER_CMD");
        }
        if let Some(value) = old_watcher_disable {
            env::set_var("ADL_ISSUE_WATCHER_DISABLE", value);
        } else {
            env::remove_var("ADL_ISSUE_WATCHER_DISABLE");
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

    result.expect("watcher attach failure should not block finish");
    let gh_calls = fs::read_to_string(&gh_log).expect("read gh log");
    assert!(gh_calls.contains("pr create"));
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
    let plan = issue_ref.task_bundle_plan_path(&repo);
    let review_policy = issue_ref.task_bundle_review_policy_path(&repo);
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
    write_authored_spp(
        &plan,
        &issue_ref,
        "[v0.86][tools] Rust finish test",
        "codex/1161-rust-finish-test",
        &repo,
    );
    write_authored_srp(
        &review_policy,
        &issue_ref,
        "[v0.86][tools] Rust finish test",
        "codex/1161-rust-finish-test",
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
    let plan = issue_ref.task_bundle_plan_path(&repo);
    let review_policy = issue_ref.task_bundle_review_policy_path(&repo);
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
    write_authored_spp(
        &plan,
        &issue_ref,
        "[v0.86][tools] Rust finish test edit",
        "codex/1153-rust-finish-test-edit",
        &repo,
    );
    write_authored_srp(
        &review_policy,
        &issue_ref,
        "[v0.86][tools] Rust finish test edit",
        "codex/1153-rust-finish-test-edit",
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
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1160\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr edit' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr ready' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'baseRefName'; then\n    printf 'main\\n'\n  elif printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1153\\n'\n  else\n    printf 'Closes #1153\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
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
    assert!(gh_calls.contains("pr edit"));
    assert!(gh_calls.contains("danielbaustin/agent-design-language"));
    assert!(gh_calls.contains("https://github.com/danielbaustin/agent-design-language/pull/1160"));
    assert!(gh_calls.contains("--title [v0.86][tools] Rust finish test edit"));
    assert!(gh_calls.contains("--body-file"));
    assert!(gh_calls.contains("pr ready"));
}
