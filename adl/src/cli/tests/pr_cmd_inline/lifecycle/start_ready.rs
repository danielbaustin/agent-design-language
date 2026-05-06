use super::*;

#[test]
fn real_pr_start_rejects_tracked_dirty_primary_main_before_binding_worktree() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-start-dirty-main-guard");
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
    fs::write(repo.join(".gitignore"), ".adl/\n").expect("gitignore");
    fs::write(repo.join("README.md"), "clean main\n").expect("readme");
    assert!(Command::new("git")
        .args(["add", ".gitignore", "README.md"])
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

    fs::create_dir_all(repo.join(".adl/local-notes")).expect("local notes dir");
    fs::write(repo.join(".adl/local-notes/ignored.md"), "local only\n").expect("ignored adl");
    fs::write(repo.join("README.md"), "tracked drift on main\n").expect("dirty readme");

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let err = real_pr(&[
        "start".to_string(),
        "1777".to_string(),
        "--slug".to_string(),
        "dirty-main-guard".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Dirty main guard".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect_err("tracked dirty primary main should block issue-mode run");
    env::set_current_dir(prev_dir).expect("restore cwd");

    let err = err.to_string();
    assert!(err.contains("unsafe_root_checkout_execution"));
    assert!(err.contains("README.md"));
    assert!(!err.contains("ignored.md"));
}

#[test]
fn real_pr_start_bootstraps_worktree_and_ready_passes() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-start-ready");
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
    fs::write(repo.join("README.md"), "ready branch placeholder\n").expect("write readme");
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
            path_str(&origin).expect("origin path")
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
        .args(["fetch", "-q", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git fetch")
        .success());

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(1152, "v0.86", "rust-start-ready-test").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust start ready test");
    let local_templates = repo.join("docs/templates");
    fs::create_dir_all(local_templates.join("nested")).expect("create local templates");
    fs::write(
        local_templates.join("README_TEMPLATE.md"),
        "# canonical readme template\n",
    )
    .expect("write readme template");
    fs::write(
        local_templates.join("nested/FEATURE_DOC_TEMPLATE.md"),
        "# canonical feature doc template\n",
    )
    .expect("write nested feature template");

    real_pr(&[
        "start".to_string(),
        "1152".to_string(),
        "--slug".to_string(),
        "rust-start-ready-test".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Rust start ready test".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr start");

    let root_sip = issue_ref.task_bundle_input_path(&repo);
    let worktree = issue_ref.default_worktree_path(&repo, None);
    let wt_sip = issue_ref.task_bundle_input_path(&worktree);
    let source_path = issue_ref.issue_prompt_path(&repo);
    let branch = "codex/1152-rust-start-ready-test";
    write_authored_sip(
        &root_sip,
        &issue_ref,
        "[v0.86][tools] Rust start ready test",
        branch,
        &source_path,
        &repo,
    );
    write_authored_sip(
        &wt_sip,
        &issue_ref,
        "[v0.86][tools] Rust start ready test",
        branch,
        &issue_ref.issue_prompt_path(&worktree),
        &worktree,
    );

    let ready = real_pr(&[
        "ready".to_string(),
        "1152".to_string(),
        "--slug".to_string(),
        "rust-start-ready-test".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    ready.expect("real_pr ready");

    assert!(worktree.is_dir());
    assert_eq!(
        run_capture(
            "git",
            &[
                "-C",
                path_str(&worktree).expect("wt path"),
                "rev-parse",
                "--abbrev-ref",
                "HEAD"
            ]
        )
        .expect("branch")
        .trim(),
        "codex/1152-rust-start-ready-test"
    );
    assert!(issue_ref.task_bundle_stp_path(&repo).is_file());
    assert!(issue_ref.task_bundle_input_path(&repo).is_file());
    assert!(issue_ref.task_bundle_output_path(&repo).is_file());
    assert!(issue_ref.task_bundle_plan_path(&repo).is_file());
    assert!(issue_ref.task_bundle_review_policy_path(&repo).is_file());
    assert!(issue_ref.task_bundle_stp_path(&worktree).is_file());
    assert!(issue_ref.task_bundle_input_path(&worktree).is_file());
    assert!(issue_ref.task_bundle_output_path(&worktree).is_file());
    assert!(issue_ref.task_bundle_plan_path(&worktree).is_file());
    assert!(issue_ref
        .task_bundle_review_policy_path(&worktree)
        .is_file());
    assert_eq!(
        fs::read_to_string(worktree.join("docs/templates/README_TEMPLATE.md"))
            .expect("read mirrored template"),
        "# canonical readme template\n"
    );
    assert_eq!(
        fs::read_to_string(worktree.join("docs/templates/nested/FEATURE_DOC_TEMPLATE.md"))
            .expect("read mirrored nested template"),
        "# canonical feature doc template\n"
    );
    let root_cards = resolve_cards_root(&repo, None);
    assert!(card_stp_path(&root_cards, 1152).symlink_metadata().is_ok());
    assert!(card_input_path(&root_cards, 1152)
        .symlink_metadata()
        .is_ok());
    assert!(card_output_path(&root_cards, 1152)
        .symlink_metadata()
        .is_ok());
    assert!(card_plan_path(&root_cards, 1152).symlink_metadata().is_ok());
    assert!(card_review_policy_path(&root_cards, 1152)
        .symlink_metadata()
        .is_ok());
}

#[test]
fn real_pr_start_rewrites_unbound_root_input_card_branch() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-start-rewrites-unbound");
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
    fs::write(repo.join("README.md"), "start rewrites unbound root card\n").expect("seed file");
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
        .args(["fetch", "-q", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git fetch")
        .success());

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(1199, "v0.86", "start-rewrites-unbound").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Start rewrites unbound");
    real_pr(&[
        "init".to_string(),
        "1199".to_string(),
        "--slug".to_string(),
        "start-rewrites-unbound".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Start rewrites unbound".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr init");

    let root_sip = issue_ref.task_bundle_input_path(&repo);
    assert_eq!(
        field_line_value(&root_sip, "Branch").expect("root branch before start"),
        "not bound yet"
    );

    let start = real_pr(&[
        "start".to_string(),
        "1199".to_string(),
        "--slug".to_string(),
        "start-rewrites-unbound".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Start rewrites unbound".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    start.expect("real_pr start after unbound root card");
    assert_eq!(
        field_line_value(&root_sip, "Branch").expect("root branch"),
        "codex/1199-start-rewrites-unbound"
    );
    assert!(issue_ref
        .task_bundle_input_path(&issue_ref.default_worktree_path(&repo, None))
        .is_file());
}

#[test]
fn real_pr_ready_succeeds_when_invoked_from_started_worktree() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-ready-worktree-cwd");
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
    fs::write(repo.join("README.md"), "ready from worktree\n").expect("seed file");
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
        .args(["fetch", "-q", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git fetch")
        .success());

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(1198, "v0.86", "ready-worktree-cwd").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Ready worktree cwd");
    real_pr(&[
        "start".to_string(),
        "1198".to_string(),
        "--slug".to_string(),
        "ready-worktree-cwd".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Ready worktree cwd".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr start");
    let worktree = issue_ref.default_worktree_path(&repo, None);
    let root_sip = issue_ref.task_bundle_input_path(&repo);
    let wt_sip = issue_ref.task_bundle_input_path(&worktree);
    write_authored_sip(
        &root_sip,
        &issue_ref,
        "[v0.86][tools] Ready worktree cwd",
        "codex/1198-ready-worktree-cwd",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_sip(
        &wt_sip,
        &issue_ref,
        "[v0.86][tools] Ready worktree cwd",
        "codex/1198-ready-worktree-cwd",
        &issue_ref.issue_prompt_path(&worktree),
        &worktree,
    );
    env::set_current_dir(&worktree).expect("chdir worktree");

    let ready = real_pr(&[
        "ready".to_string(),
        "1198".to_string(),
        "--slug".to_string(),
        "ready-worktree-cwd".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    ready.expect("ready from worktree");
}

#[test]
fn real_pr_start_blocks_when_open_milestone_pr_wave_exists() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-start-blocks-wave");
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
    fs::write(repo.join("README.md"), "ready branch placeholder\n").expect("write readme");
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
            path_str(&origin).expect("origin path")
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
        .args(["fetch", "-q", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git fetch")
        .success());
    let issue_ref = IssueRef::new(
        1173,
        "v0.86".to_string(),
        "v0-86-tools-preflight-guard".to_string(),
    )
    .expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Preflight guard");

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
            &gh_path,
            "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"pr list\" ]]; then\n  cat <<'JSON'\n[{\"number\":1169,\"title\":\"[v0.86][tools] Keep tools queue busy\",\"url\":\"https://example.test/pr/1169\",\"headRefName\":\"codex/1161-v0-86-tools-keep-tools-queue-busy\",\"baseRefName\":\"main\",\"isDraft\":true}]\nJSON\n  exit 0\nfi\nexit 1\n",
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "start".to_string(),
        "1173".to_string(),
        "--slug".to_string(),
        "v0-86-tools-preflight-guard".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Preflight guard".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--no-fetch-issue".to_string(),
    ])
    .expect_err("start should block on same-queue open PR");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    assert!(err
        .to_string()
        .contains("start: unresolved open PR queue detected for v0.86 [tools:inferred]"));
    assert!(err.to_string().contains("#1169 [draft]"));
}

#[test]
fn real_pr_ready_requires_slug_when_local_state_missing() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-ready-missing-slug");
    init_git_repo(&repo);
    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let err = real_pr(&["ready".to_string(), "1152".to_string()]).expect_err("ready should fail");
    env::set_current_dir(prev_dir).expect("restore cwd");
    assert!(err
        .to_string()
        .contains("ready: could not infer slug; pass --slug or run start first"));
}
