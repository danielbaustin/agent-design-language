use super::*;

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
fn real_pr_finish_rejects_staged_foreign_issue_bundle_mutations() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-foreign-bundle");
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
        .args([
            "checkout",
            "-q",
            "-b",
            "codex/1161-rust-finish-foreign-bundle"
        ])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let issue_ref = IssueRef::new(
        1161,
        "v0.86".to_string(),
        "rust-finish-foreign-bundle".to_string(),
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
        "[v0.86][tools] Rust finish foreign bundle",
    );
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Rust finish foreign bundle",
        "codex/1161-rust-finish-foreign-bundle",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1161-rust-finish-foreign-bundle");

    let foreign_issue_ref =
        IssueRef::new(7777, "v0.86".to_string(), "foreign-drift".to_string()).expect("foreign");
    let foreign_bundle = foreign_issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&foreign_bundle).expect("foreign bundle dir");
    fs::write(foreign_bundle.join("sor.md"), "# stray\n").expect("foreign sor");
    assert!(Command::new("git")
        .args([
            "add",
            "-f",
            &path_relative_to_repo(&repo, &foreign_bundle.join("sor.md")),
        ])
        .current_dir(&repo)
        .status()
        .expect("git add foreign sor")
        .success());

    fs::write(
        repo.join("adl/src/lib.rs"),
        "pub fn placeholder() {}\npub fn changed() {}\n",
    )
    .expect("write change");

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
        "1161".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Rust finish foreign bundle".to_string(),
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
    let err = result.expect_err("finish should refuse foreign staged bundle mutation");
    assert!(err
        .to_string()
        .contains("staged .adl task-bundle changes for non-active issues detected"));
    assert!(err.to_string().contains("issue-7777__foreign-drift/sor.md"));
}

#[test]
fn real_pr_finish_allows_deletion_only_cleanup_for_foreign_issue_bundle_residue() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-foreign-bundle-delete");
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
            "codex/1161-rust-finish-foreign-bundle-delete"
        ])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let issue_ref =
        IssueRef::new(1161, "v0.86", "rust-finish-foreign-bundle-delete").expect("issue ref");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    let source = issue_ref.issue_prompt_path(&repo);
    write_authored_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.86][tools] Rust finish foreign bundle delete",
    );
    fs::create_dir_all(stp.parent().expect("stp parent")).expect("stp parent");
    fs::copy(&source, &stp).expect("write stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Rust finish foreign bundle delete",
        "codex/1161-rust-finish-foreign-bundle-delete",
        &source,
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1161-rust-finish-foreign-bundle-delete");
    assert!(Command::new("git")
        .args(["add", "adl/src/lib.rs"])
        .current_dir(&repo)
        .status()
        .expect("git add lib")
        .success());

    let foreign = repo.join(".adl/v0.86/tasks/issue-7777__foreign-drift/sor.md");
    fs::create_dir_all(foreign.parent().expect("foreign parent")).expect("foreign parent");
    fs::write(&foreign, "foreign residue\n").expect("write foreign sor");
    assert!(Command::new("git")
        .args(["add", path_str(&foreign).expect("foreign path")])
        .current_dir(&repo)
        .status()
        .expect("git add foreign sor")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "seed foreign residue"])
        .current_dir(&repo)
        .status()
        .expect("git commit foreign residue")
        .success());
    assert!(Command::new("git")
        .args(["rm", "--cached", path_str(&foreign).expect("foreign path")])
        .current_dir(&repo)
        .status()
        .expect("git rm cached foreign sor")
        .success());

    fs::write(
        repo.join("adl/src/lib.rs"),
        "pub fn placeholder() {}\npub fn changed() {}\n",
    )
    .expect("write change");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == 'repo view' ]]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == 'pr list' ]]; then\n  exit 0\nfi\nif [[ \"$1 $2\" == 'pr create' ]]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/9999\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == 'pr view' ]]; then\n  printf '{\"body\":\"Closes #1161\"}\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == 'pr edit' ]]; then\n  exit 0\nfi\nif [[ \"$1 $2\" == 'issue view' ]]; then\n  printf '{\"state\":\"OPEN\",\"stateReason\":null}\\n'\n  exit 0\nfi\nexit 0\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
        "finish".to_string(),
        "1161".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Rust finish foreign bundle delete".to_string(),
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
    result.expect("finish should allow deletion-only foreign bundle cleanup");
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
        .contains("Canonical .adl issue bundles are local-only and must not be staged"));
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
