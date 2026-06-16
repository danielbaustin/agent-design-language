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
    write_design_time_ready_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Rust start ready test",
        "codex/1152-rust-start-ready-test",
    );
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
    assert!(issue_ref.issue_prompt_path(&worktree).is_file());
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
fn real_pr_start_repairs_preexisting_worktree_missing_task_bundle() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-start-repair-worktree-bundle");
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
    fs::write(repo.join("README.md"), "repair worktree bundle test\n").expect("write readme");
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
    let issue_ref = IssueRef::new(1153, "v0.86", "repair-worktree-bundle").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Repair worktree bundle");
    write_design_time_ready_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Repair worktree bundle",
        "codex/1153-repair-worktree-bundle",
    );
    let branch = "codex/1153-repair-worktree-bundle";
    let worktree = issue_ref.default_worktree_path(&repo, None);
    assert!(Command::new("git")
        .args([
            "worktree",
            "add",
            "-b",
            branch,
            path_str(&worktree).expect("wt path"),
            "origin/main",
        ])
        .current_dir(&repo)
        .status()
        .expect("git worktree add")
        .success());
    assert!(!issue_ref.worktree_task_bundle_dir_path(&worktree).exists());

    real_pr(&[
        "start".to_string(),
        "1153".to_string(),
        "--slug".to_string(),
        "repair-worktree-bundle".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Repair worktree bundle".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr start");

    env::set_current_dir(prev_dir).expect("restore cwd");

    assert!(issue_ref.issue_prompt_path(&worktree).is_file());
    assert!(issue_ref.worktree_task_bundle_stp_path(&worktree).is_file());
    assert!(issue_ref
        .worktree_task_bundle_input_path(&worktree)
        .is_file());
    assert!(issue_ref
        .worktree_task_bundle_output_path(&worktree)
        .is_file());
    assert!(issue_ref
        .worktree_task_bundle_plan_path(&worktree)
        .is_file());
    assert!(issue_ref
        .worktree_task_bundle_review_policy_path(&worktree)
        .is_file());
}

#[test]
fn real_pr_start_updates_existing_root_spp_and_srp_branch() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-start-updates-root-plan-review-branch");
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
    fs::write(
        repo.join("README.md"),
        "update root plan/review branch test\n",
    )
    .expect("write readme");
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
    let issue_ref =
        IssueRef::new(1154, "v0.86", "update-root-plan-review-branch").expect("issue ref");
    write_authored_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.86][tools] Update root plan and review branch",
    );

    real_pr(&[
        "init".to_string(),
        "1154".to_string(),
        "--slug".to_string(),
        "update-root-plan-review-branch".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Update root plan and review branch".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr init");

    let branch = "codex/1154-update-root-plan-review-branch";
    assert!(fs::read_to_string(issue_ref.task_bundle_plan_path(&repo))
        .expect("read root spp")
        .contains("branch: \"not bound yet\""));
    assert!(
        fs::read_to_string(issue_ref.task_bundle_review_policy_path(&repo))
            .expect("read root srp")
            .contains("branch: \"not bound yet\"")
    );
    write_design_time_ready_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Update root plan and review branch",
        "not bound yet",
    );

    real_pr(&[
        "start".to_string(),
        "1154".to_string(),
        "--slug".to_string(),
        "update-root-plan-review-branch".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Update root plan and review branch".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr start");

    let worktree = issue_ref.default_worktree_path(&repo, None);
    let root_sip = issue_ref.task_bundle_input_path(&repo);
    let wt_sip = issue_ref.task_bundle_input_path(&worktree);
    let source_path = issue_ref.issue_prompt_path(&repo);
    write_authored_sip(
        &root_sip,
        &issue_ref,
        "[v0.86][tools] Update root plan and review branch",
        branch,
        &source_path,
        &repo,
    );
    write_authored_sip(
        &wt_sip,
        &issue_ref,
        "[v0.86][tools] Update root plan and review branch",
        branch,
        &issue_ref.issue_prompt_path(&worktree),
        &worktree,
    );

    let ready = real_pr(&[
        "ready".to_string(),
        "1154".to_string(),
        "--slug".to_string(),
        "update-root-plan-review-branch".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");

    ready.expect("real_pr ready");
    assert!(fs::read_to_string(issue_ref.task_bundle_plan_path(&repo))
        .expect("read updated root spp")
        .contains(&format!("branch: \"{branch}\"")));
    assert!(
        fs::read_to_string(issue_ref.task_bundle_review_policy_path(&repo))
            .expect("read updated root srp")
            .contains(&format!("branch: \"{branch}\""))
    );
    assert!(
        fs::read_to_string(issue_ref.task_bundle_plan_path(&worktree))
            .expect("read worktree spp")
            .contains(&format!("branch: \"{branch}\""))
    );
    assert!(
        fs::read_to_string(issue_ref.task_bundle_review_policy_path(&worktree))
            .expect("read worktree srp")
            .contains(&format!("branch: \"{branch}\""))
    );
}

#[test]
fn real_pr_ready_blocks_invalid_worktree_srp() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-ready-invalid-worktree-srp");
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
    fs::write(repo.join("README.md"), "ready invalid srp\n").expect("write readme");
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
    let issue_ref = IssueRef::new(1917, "v0.86", "ready-invalid-worktree-srp").expect("issue ref");
    write_authored_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.86][tools] Ready invalid worktree srp",
    );
    write_design_time_ready_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Ready invalid worktree srp",
        "codex/1917-ready-invalid-worktree-srp",
    );

    real_pr(&[
        "start".to_string(),
        "1917".to_string(),
        "--slug".to_string(),
        "ready-invalid-worktree-srp".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Ready invalid worktree srp".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr start");

    let root_sip = issue_ref.task_bundle_input_path(&repo);
    let worktree = issue_ref.default_worktree_path(&repo, None);
    let wt_sip = issue_ref.task_bundle_input_path(&worktree);
    let source_path = issue_ref.issue_prompt_path(&repo);
    let branch = "codex/1917-ready-invalid-worktree-srp";
    write_authored_sip(
        &root_sip,
        &issue_ref,
        "[v0.86][tools] Ready invalid worktree srp",
        branch,
        &source_path,
        &repo,
    );
    write_authored_sip(
        &wt_sip,
        &issue_ref,
        "[v0.86][tools] Ready invalid worktree srp",
        branch,
        &issue_ref.issue_prompt_path(&worktree),
        &worktree,
    );

    let wt_srp = issue_ref.task_bundle_review_policy_path(&worktree);
    let invalid_srp = fs::read_to_string(&wt_srp)
        .expect("read worktree srp")
        .replace("status: \"draft\"", "status: \"queued\"");
    fs::write(&wt_srp, invalid_srp).expect("write invalid worktree srp");

    let err = real_pr(&[
        "ready".to_string(),
        "1917".to_string(),
        "--slug".to_string(),
        "ready-invalid-worktree-srp".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect_err("ready should reject invalid worktree srp");

    env::set_current_dir(prev_dir).expect("restore cwd");
    let err = err.to_string();
    assert!(err.contains("ready: srp failed validation"));
    assert!(err.contains("status must be one of: draft, ready, approved"));
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
    write_design_time_ready_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Start rewrites unbound",
        "not bound yet",
    );

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
fn real_pr_start_uses_canonical_local_slug_when_title_slug_drift_exists() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-start-canonical-local-slug");
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
    fs::write(repo.join("README.md"), "canonical local slug\n").expect("seed file");
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
    let issue_ref =
        IssueRef::new(1288, "v0.86", "canonical-bind-slug").expect("canonical issue ref");
    write_authored_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.86][tools] Canonical bind slug issue",
    );
    real_pr(&[
        "init".to_string(),
        "1288".to_string(),
        "--slug".to_string(),
        "canonical-bind-slug".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Canonical bind slug issue".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("init canonical bundle");
    write_design_time_ready_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Canonical bind slug issue",
        "not bound yet",
    );

    real_pr(&[
        "start".to_string(),
        "1288".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Title changed after bundle bootstrap".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("start should honor canonical local slug");

    env::set_current_dir(prev_dir).expect("restore cwd");

    let worktree = issue_ref.default_worktree_path(&repo, None);
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
        "codex/1288-canonical-bind-slug"
    );
    assert!(issue_ref.task_bundle_stp_path(&repo).is_file());
    assert!(issue_ref.task_bundle_stp_path(&worktree).is_file());
}

#[test]
fn real_pr_start_still_fails_closed_when_real_duplicate_bundle_exists() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-start-real-duplicate-bundle");
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
    fs::write(repo.join("README.md"), "duplicate bind test\n").expect("seed file");
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
    let issue_ref = IssueRef::new(1289, "v0.86", "canonical-duplicate-bind").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Canonical duplicate bind");
    real_pr(&[
        "init".to_string(),
        "1289".to_string(),
        "--slug".to_string(),
        "canonical-duplicate-bind".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Canonical duplicate bind".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("init canonical bundle");
    let duplicate_task = repo.join(".adl/v0.86/tasks/issue-1289__legacy-duplicate-bind");
    fs::create_dir_all(&duplicate_task).expect("duplicate task dir");

    let err = real_pr(&[
        "start".to_string(),
        "1289".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Canonical duplicate bind".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect_err("real duplicate should still fail closed");

    env::set_current_dir(prev_dir).expect("restore cwd");

    let text = err.to_string();
    assert!(text.contains("duplicate local task-bundle identities detected"));
    assert!(text.contains("legacy-duplicate-bind"));
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
    write_design_time_ready_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Ready worktree cwd",
        "codex/1198-ready-worktree-cwd",
    );
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
            "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"pr list\" ]]; then\n  cat <<'JSON'\n[{\"number\":1169,\"title\":\"[v0.86][tools] Keep tools queue busy\",\"url\":\"https://example.test/pr/1169\",\"headRefName\":\"codex/1161-v0-86-tools-keep-tools-queue-busy\",\"baseRefName\":\"main\",\"isDraft\":true}]\nJSON\n  exit 0\nfi\nif [[ \"$1 $2\" == \"pr view\" ]]; then\n  printf '1161\\n'\n  exit 0\nfi\nexit 1\n",
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
