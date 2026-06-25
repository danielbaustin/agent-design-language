use super::*;
use adl::session_ledger::{
    default_ledger_path, save_ledger, ClaimInput, ClaimMode, GithubRef, ResourceRef, SessionLedger,
    DEFAULT_TTL_SECS,
};

fn assert_nonempty_materialized_file(path: impl AsRef<std::path::Path>) {
    let path = path.as_ref();
    let metadata = fs::metadata(path).unwrap_or_else(|err| {
        panic!(
            "expected materialized nonempty file at {}: {err}",
            path.display()
        )
    });
    assert!(
        metadata.is_file() && metadata.len() > 0,
        "expected materialized nonempty file at {}",
        path.display()
    );
}

fn write_session_claim(
    repo: &std::path::Path,
    issue: u64,
    session_id: &str,
    branch: &str,
    worktree_path: &std::path::Path,
) {
    let mut ledger = SessionLedger::empty(chrono::Utc::now());
    ledger
        .claim(
            ClaimInput {
                session_id: session_id.to_string(),
                owner: "codex".to_string(),
                resource: ResourceRef {
                    kind: "csdlc_issue".to_string(),
                    id: issue.to_string(),
                },
                purpose: "test ownership".to_string(),
                mode: ClaimMode::Active,
                lifecycle_phase: Some("pr_run".to_string()),
                policy_ref: Some("AGENTS.md".to_string()),
                github: GithubRef {
                    issue: Some(issue),
                    pull_request: None,
                    repository: Some("owner/repo".to_string()),
                    last_state: Some("open".to_string()),
                },
                branch: Some(branch.to_string()),
                worktree_path: Some(
                    worktree_path
                        .strip_prefix(repo)
                        .unwrap_or(worktree_path)
                        .display()
                        .to_string(),
                ),
                do_not_touch_paths: Vec::new(),
                blockers: Vec::new(),
                ttl_secs: DEFAULT_TTL_SECS,
            },
            chrono::Utc::now(),
        )
        .expect("claim");
    save_ledger(&default_ledger_path(repo), &ledger).expect("save ledger");
}

#[test]
fn init_doctor_and_start_record_readiness_prep_goal_metric_stages() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-start-readiness-prep-metrics");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    let home = temp.join("home");
    fs::create_dir_all(&repo).expect("repo dir");
    fs::create_dir_all(&home).expect("home dir");
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
    fs::write(repo.join("README.md"), "readiness prep metrics\n").expect("readme");
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

    seed_codex_goal_transcript(
        &home,
        1154,
        "thread-1154-prep",
        4442,
        88,
        1782230400,
        1782230488,
    );
    let old_home = env::var_os("HOME");
    let old_thread_id = env::var_os("CODEX_THREAD_ID");
    unsafe {
        env::set_var("HOME", &home);
        env::set_var("CODEX_THREAD_ID", "thread-1154-prep");
    }

    let issue_ref = IssueRef::new(1154, "v0.86", "readiness-prep-metrics").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Readiness prep metrics");

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    real_pr(&[
        "init".to_string(),
        "1154".to_string(),
        "--slug".to_string(),
        "readiness-prep-metrics".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Readiness prep metrics".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr init");

    let doctor_block = real_pr(&[
        "doctor".to_string(),
        "1154".to_string(),
        "--slug".to_string(),
        "readiness-prep-metrics".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--mode".to_string(),
        "ready".to_string(),
    ]);
    doctor_block.expect("doctor should report BLOCK without failing the command");

    write_design_time_ready_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Readiness prep metrics",
        "not bound yet",
    );

    real_pr(&[
        "doctor".to_string(),
        "1154".to_string(),
        "--slug".to_string(),
        "readiness-prep-metrics".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--mode".to_string(),
        "ready".to_string(),
    ])
    .expect("real_pr doctor ready");

    write_session_claim(
        &repo,
        1154,
        "thread-self",
        "codex/1154-readiness-prep-metrics",
        &issue_ref.default_worktree_path(&repo, None),
    );

    real_pr(&[
        "start".to_string(),
        "1154".to_string(),
        "--slug".to_string(),
        "readiness-prep-metrics".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Readiness prep metrics".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr start");
    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        match old_home {
            Some(value) => env::set_var("HOME", value),
            None => env::remove_var("HOME"),
        }
        match old_thread_id {
            Some(value) => env::set_var("CODEX_THREAD_ID", value),
            None => env::remove_var("CODEX_THREAD_ID"),
        }
    }

    let summary_path = issue_ref
        .task_bundle_dir_path(&repo)
        .join("artifacts/goal_metrics/issue-1154-goal-metrics-summary.json");
    let summary: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(summary_path).expect("read goal summary"))
            .expect("parse goal summary");
    assert_eq!(
        summary["phases_recorded"],
        serde_json::json!([
            "card_repair",
            "doctor_readiness",
            "execution_ready",
            "issue_init"
        ])
    );
    assert_eq!(
        summary["segments_recorded"],
        serde_json::json!(["readiness_prep"])
    );
    assert_eq!(summary["selected_stage"], "execution_ready");
    assert_eq!(summary["selected_segment"], "readiness_prep");
}

#[test]
fn real_pr_start_failure_leaves_doctor_readiness_as_selected_prep_stage() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-start-readiness-prep-failure");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    let home = temp.join("home");
    fs::create_dir_all(&repo).expect("repo dir");
    fs::create_dir_all(&home).expect("home dir");
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
    fs::write(repo.join("README.md"), "readiness prep metrics failure\n").expect("readme");
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

    seed_codex_goal_transcript(
        &home,
        1155,
        "thread-1155-prep",
        5555,
        77,
        1782230400,
        1782230477,
    );
    let old_home = env::var_os("HOME");
    let old_env_session = env::var_os("CODEX_SESSION_ID");
    let old_thread_id = env::var_os("CODEX_THREAD_ID");
    unsafe {
        env::set_var("HOME", &home);
        env::set_var("CODEX_SESSION_ID", "thread-1155-prep");
        env::set_var("CODEX_THREAD_ID", "thread-1155-prep");
    }

    let issue_ref =
        IssueRef::new(1155, "v0.86", "readiness-prep-start-failure").expect("issue ref");
    write_authored_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.86][tools] Readiness prep start failure",
    );

    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("CODEX_SESSION_ID", "thread-self");
    }
    env::set_current_dir(&repo).expect("chdir");
    real_pr(&[
        "init".to_string(),
        "1155".to_string(),
        "--slug".to_string(),
        "readiness-prep-start-failure".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Readiness prep start failure".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr init");

    write_design_time_ready_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Readiness prep start failure",
        "not bound yet",
    );
    write_session_claim(
        &repo,
        1155,
        "thread-self",
        "codex/1155-readiness-prep-start-failure",
        &issue_ref.default_worktree_path(&repo, None),
    );

    real_pr(&[
        "doctor".to_string(),
        "1155".to_string(),
        "--slug".to_string(),
        "readiness-prep-start-failure".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--mode".to_string(),
        "ready".to_string(),
    ])
    .expect("real_pr doctor ready");

    let conflicting_branch = "codex/1155-readiness-prep-start-failure";
    assert!(Command::new("git")
        .args(["branch", conflicting_branch, "origin/main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    let conflicting_worktree = repo.join(".worktrees/conflicting-start");
    assert!(Command::new("git")
        .args([
            "worktree",
            "add",
            path_str(&conflicting_worktree).expect("conflicting worktree path"),
            conflicting_branch,
        ])
        .current_dir(&repo)
        .status()
        .expect("git worktree add")
        .success());

    let err = real_pr(&[
        "start".to_string(),
        "1155".to_string(),
        "--slug".to_string(),
        "readiness-prep-start-failure".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Readiness prep start failure".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect_err("real_pr start should fail when the issue branch is already checked out elsewhere");
    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        match old_home {
            Some(value) => env::set_var("HOME", value),
            None => env::remove_var("HOME"),
        }
        match old_env_session {
            Some(value) => env::set_var("CODEX_SESSION_ID", value),
            None => env::remove_var("CODEX_SESSION_ID"),
        }
        match old_thread_id {
            Some(value) => env::set_var("CODEX_THREAD_ID", value),
            None => env::remove_var("CODEX_THREAD_ID"),
        }
    }
    assert!(err.to_string().contains("already checked out in worktree"));

    let summary_path = issue_ref
        .task_bundle_dir_path(&repo)
        .join("artifacts/goal_metrics/issue-1155-goal-metrics-summary.json");
    let summary: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(summary_path).expect("read goal summary"))
            .expect("parse goal summary");
    assert_eq!(
        summary["phases_recorded"],
        serde_json::json!(["doctor_readiness", "issue_init"])
    );
    assert_eq!(
        summary["segments_recorded"],
        serde_json::json!(["readiness_prep"])
    );
    assert_eq!(summary["selected_stage"], "doctor_readiness");
    assert_eq!(summary["selected_segment"], "readiness_prep");
}

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
    let old_session = env::var_os("CODEX_SESSION_ID");
    unsafe {
        env::set_var("CODEX_SESSION_ID", "thread-self");
    }
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(1152, "v0.86", "rust-start-ready-test").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust start ready test");
    write_design_time_ready_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Rust start ready test",
        "codex/1152-rust-start-ready-test",
    );
    write_session_claim(
        &repo,
        1152,
        "thread-self",
        "codex/1152-rust-start-ready-test",
        &issue_ref.default_worktree_path(&repo, None),
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
    match old_session {
        Some(value) => unsafe { env::set_var("CODEX_SESSION_ID", value) },
        None => unsafe { env::remove_var("CODEX_SESSION_ID") },
    }
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
    assert_nonempty_materialized_file(issue_ref.issue_prompt_path(&worktree));
    assert_nonempty_materialized_file(issue_ref.task_bundle_stp_path(&worktree));
    assert_nonempty_materialized_file(issue_ref.task_bundle_input_path(&worktree));
    assert_nonempty_materialized_file(issue_ref.task_bundle_output_path(&worktree));
    assert_nonempty_materialized_file(issue_ref.task_bundle_plan_path(&worktree));
    assert_nonempty_materialized_file(issue_ref.task_bundle_review_policy_path(&worktree));
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
fn real_pr_start_requires_an_active_self_claim_before_binding_worktree() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-start-requires-self-claim");
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
    fs::write(repo.join("README.md"), "missing self claim\n").expect("write readme");
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

    let issue_ref = IssueRef::new(1157, "v0.86", "requires-self-claim").expect("issue ref");
    let branch = "codex/1157-requires-self-claim";
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Requires self claim");
    write_design_time_ready_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Requires self claim",
        branch,
    );

    let prev_dir = env::current_dir().expect("cwd");
    let old_session = env::var("CODEX_SESSION_ID").ok();
    unsafe {
        env::set_var("CODEX_SESSION_ID", "thread-self");
    }
    env::set_current_dir(&repo).expect("chdir");
    let err = real_pr(&[
        "start".to_string(),
        "1157".to_string(),
        "--slug".to_string(),
        "requires-self-claim".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Requires self claim".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect_err("missing self claim should block run binding");
    env::set_current_dir(prev_dir).expect("restore cwd");
    match old_session {
        Some(value) => unsafe { env::set_var("CODEX_SESSION_ID", value) },
        None => unsafe { env::remove_var("CODEX_SESSION_ID") },
    }

    let err_text = err.to_string();
    assert!(err_text.contains("missing active self-claim"));
    assert!(err_text.contains("adl session claim"));
    assert!(!issue_ref.default_worktree_path(&repo, None).exists());
}

#[test]
fn real_pr_start_blocks_when_another_session_claims_the_issue() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-start-session-ledger-conflict");
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
    fs::write(repo.join("README.md"), "session ledger conflict\n").expect("write readme");
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
        1155,
        "v0.86".to_string(),
        "session-ledger-conflict".to_string(),
    )
    .expect("issue ref");
    let branch = "codex/1155-session-ledger-conflict";
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Session ledger conflict");
    write_design_time_ready_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Session ledger conflict",
        branch,
    );
    write_session_claim(
        &repo,
        1155,
        "thread-other",
        branch,
        &issue_ref.default_worktree_path(&repo, None),
    );

    let prev_dir = env::current_dir().expect("cwd");
    let old_session = env::var("CODEX_SESSION_ID").ok();
    unsafe {
        env::set_var("CODEX_SESSION_ID", "thread-self");
    }
    env::set_current_dir(&repo).expect("chdir");
    let err = real_pr(&[
        "start".to_string(),
        "1155".to_string(),
        "--slug".to_string(),
        "session-ledger-conflict".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Session ledger conflict".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect_err("active claim should block run binding");
    env::set_current_dir(prev_dir).expect("restore cwd");
    match old_session {
        Some(value) => unsafe { env::set_var("CODEX_SESSION_ID", value) },
        None => unsafe { env::remove_var("CODEX_SESSION_ID") },
    }

    let err_text = err.to_string();
    assert!(err_text.contains("session ledger active conflict"));
    assert!(err_text.contains("thread-other"));
    assert!(
        !issue_ref.default_worktree_path(&repo, None).exists(),
        "conflict should block before worktree creation"
    );
}

#[test]
fn real_pr_start_allows_current_session_claim_and_stale_claim_history() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-start-session-ledger-self-claim");
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
    fs::write(repo.join("README.md"), "session ledger self claim\n").expect("write readme");
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
        1156,
        "v0.86".to_string(),
        "session-ledger-self-claim".to_string(),
    )
    .expect("issue ref");
    let branch = "codex/1156-session-ledger-self-claim";
    write_authored_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.86][tools] Session ledger self claim",
    );
    write_design_time_ready_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Session ledger self claim",
        branch,
    );
    write_session_claim(
        &repo,
        1156,
        "thread-self",
        branch,
        &issue_ref.default_worktree_path(&repo, None),
    );

    let prev_dir = env::current_dir().expect("cwd");
    let old_session = env::var("CODEX_SESSION_ID").ok();
    unsafe {
        env::set_var("CODEX_SESSION_ID", "thread-self");
    }
    env::set_current_dir(&repo).expect("chdir");
    real_pr(&[
        "start".to_string(),
        "1156".to_string(),
        "--slug".to_string(),
        "session-ledger-self-claim".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Session ledger self claim".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("self claim should allow run binding");
    env::set_current_dir(prev_dir).expect("restore cwd");
    match old_session {
        Some(value) => unsafe { env::set_var("CODEX_SESSION_ID", value) },
        None => unsafe { env::remove_var("CODEX_SESSION_ID") },
    }

    assert!(issue_ref.default_worktree_path(&repo, None).is_dir());
}

#[test]
fn real_pr_start_rejects_invocation_from_existing_issue_worktree() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-start-nested-worktree-guard");
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
    fs::write(repo.join("README.md"), "nested guard placeholder\n").expect("write readme");
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

    let issue_ref = IssueRef::new(3819, "v0.91.5", "nested-worktree-guard").expect("issue ref");
    let branch = "codex/3819-v0-91-5-tools-nested-worktree-guard";
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.91.5][tools] Nested worktree guard");
    write_design_time_ready_cards(
        &repo,
        &issue_ref,
        "[v0.91.5][tools] Nested worktree guard",
        branch,
    );
    write_session_claim(
        &repo,
        3819,
        "thread-self",
        branch,
        &issue_ref.default_worktree_path(&repo, None),
    );

    let prev_dir = env::current_dir().expect("cwd");
    let old_session = env::var_os("CODEX_SESSION_ID");
    unsafe {
        env::set_var("CODEX_SESSION_ID", "thread-self");
    }
    env::set_current_dir(&repo).expect("chdir repo");
    real_pr(&[
        "start".to_string(),
        "3819".to_string(),
        "--slug".to_string(),
        "nested-worktree-guard".to_string(),
        "--title".to_string(),
        "[v0.91.5][tools] Nested worktree guard".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.91.5".to_string(),
    ])
    .expect("initial start");

    let worktree = issue_ref.default_worktree_path(&repo, None);
    env::set_current_dir(&worktree).expect("chdir worktree");
    let err = real_pr(&[
        "start".to_string(),
        "3819".to_string(),
        "--slug".to_string(),
        "nested-worktree-guard".to_string(),
        "--title".to_string(),
        "[v0.91.5][tools] Nested worktree guard".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.91.5".to_string(),
    ])
    .expect_err("issue worktree invocation should fail clearly");
    env::set_current_dir(prev_dir).expect("restore cwd");
    match old_session {
        Some(value) => unsafe { env::set_var("CODEX_SESSION_ID", value) },
        None => unsafe { env::remove_var("CODEX_SESSION_ID") },
    }

    let err = err.to_string();
    assert!(err.contains("must be invoked from the primary checkout"));
    assert!(err.contains(&repo.display().to_string()));
    assert!(err.contains(&worktree.display().to_string()));
    assert!(
        !worktree.join(".worktrees").exists(),
        "nested worktree root should not be created inside the issue worktree"
    );
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
    let old_session = env::var_os("CODEX_SESSION_ID");
    unsafe {
        env::set_var("CODEX_SESSION_ID", "thread-self");
    }
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
    write_session_claim(
        &repo,
        1153,
        "thread-self",
        branch,
        &issue_ref.default_worktree_path(&repo, None),
    );
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
    match old_session {
        Some(value) => unsafe { env::set_var("CODEX_SESSION_ID", value) },
        None => unsafe { env::remove_var("CODEX_SESSION_ID") },
    }

    assert_nonempty_materialized_file(issue_ref.issue_prompt_path(&worktree));
    assert_nonempty_materialized_file(issue_ref.worktree_task_bundle_stp_path(&worktree));
    assert_nonempty_materialized_file(issue_ref.worktree_task_bundle_input_path(&worktree));
    assert_nonempty_materialized_file(issue_ref.worktree_task_bundle_output_path(&worktree));
    assert_nonempty_materialized_file(issue_ref.worktree_task_bundle_plan_path(&worktree));
    assert_nonempty_materialized_file(issue_ref.worktree_task_bundle_review_policy_path(&worktree));
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
    let old_session = env::var_os("CODEX_SESSION_ID");
    unsafe {
        env::set_var("CODEX_SESSION_ID", "thread-self");
    }
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
    write_session_claim(
        &repo,
        1154,
        "thread-self",
        branch,
        &issue_ref.default_worktree_path(&repo, None),
    );
    write_session_claim(
        &repo,
        1154,
        "thread-self",
        branch,
        &issue_ref.default_worktree_path(&repo, None),
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
    match old_session {
        Some(value) => unsafe { env::set_var("CODEX_SESSION_ID", value) },
        None => unsafe { env::remove_var("CODEX_SESSION_ID") },
    }

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
    let old_session = env::var_os("CODEX_SESSION_ID");
    unsafe {
        env::set_var("CODEX_SESSION_ID", "thread-self");
    }
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
    write_session_claim(
        &repo,
        1917,
        "thread-self",
        "codex/1917-ready-invalid-worktree-srp",
        &issue_ref.default_worktree_path(&repo, None),
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
    match old_session {
        Some(value) => unsafe { env::set_var("CODEX_SESSION_ID", value) },
        None => unsafe { env::remove_var("CODEX_SESSION_ID") },
    }
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
    let old_session = env::var_os("CODEX_SESSION_ID");
    unsafe {
        env::set_var("CODEX_SESSION_ID", "thread-self");
    }
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
    write_session_claim(
        &repo,
        1199,
        "thread-self",
        "codex/1199-start-rewrites-unbound",
        &issue_ref.default_worktree_path(&repo, None),
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
    match old_session {
        Some(value) => unsafe { env::set_var("CODEX_SESSION_ID", value) },
        None => unsafe { env::remove_var("CODEX_SESSION_ID") },
    }
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
    let old_session = env::var_os("CODEX_SESSION_ID");
    unsafe {
        env::set_var("CODEX_SESSION_ID", "thread-self");
    }
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
    write_session_claim(
        &repo,
        1288,
        "thread-self",
        "codex/1288-canonical-bind-slug",
        &issue_ref.default_worktree_path(&repo, None),
    );

    let old_session = env::var_os("CODEX_SESSION_ID");
    unsafe {
        env::set_var("CODEX_SESSION_ID", "thread-self");
    }
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
    match old_session {
        Some(value) => unsafe { env::set_var("CODEX_SESSION_ID", value) },
        None => unsafe { env::remove_var("CODEX_SESSION_ID") },
    }

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
    let old_session = env::var_os("CODEX_SESSION_ID");
    unsafe {
        env::set_var("CODEX_SESSION_ID", "thread-self");
    }
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
    let old_session = env::var_os("CODEX_SESSION_ID");
    unsafe {
        env::set_var("CODEX_SESSION_ID", "thread-self");
    }
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(1198, "v0.86", "ready-worktree-cwd").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Ready worktree cwd");
    write_design_time_ready_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Ready worktree cwd",
        "codex/1198-ready-worktree-cwd",
    );
    write_session_claim(
        &repo,
        1198,
        "thread-self",
        "codex/1198-ready-worktree-cwd",
        &issue_ref.default_worktree_path(&repo, None),
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
    match old_session {
        Some(value) => unsafe { env::set_var("CODEX_SESSION_ID", value) },
        None => unsafe { env::remove_var("CODEX_SESSION_ID") },
    }
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
    let old_github_client = env::var_os("ADL_GITHUB_CLIENT");
    let old_github_token = env::var_os("GITHUB_TOKEN");
    let old_gh_token = env::var_os("GH_TOKEN");
    let old_token_file = env::var_os("ADL_GITHUB_TOKEN_FILE");
    let old_keychain_service = env::var_os("ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE");
    let old_keychain_account = env::var_os("ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT");
    let old_disable_default_token_file = env::var_os("ADL_TEST_DISABLE_DEFAULT_GITHUB_TOKEN_FILE");
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::remove_var("ADL_GITHUB_CLIENT");
        env::set_var("ADL_TEST_DISABLE_DEFAULT_GITHUB_TOKEN_FILE", "1");
        env::remove_var("GITHUB_TOKEN");
        env::remove_var("GH_TOKEN");
        env::remove_var("ADL_GITHUB_TOKEN_FILE");
        env::remove_var("ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE");
        env::remove_var("ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT");
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
        match old_github_client {
            Some(value) => env::set_var("ADL_GITHUB_CLIENT", value),
            None => env::remove_var("ADL_GITHUB_CLIENT"),
        }
        match old_github_token {
            Some(value) => env::set_var("GITHUB_TOKEN", value),
            None => env::remove_var("GITHUB_TOKEN"),
        }
        match old_gh_token {
            Some(value) => env::set_var("GH_TOKEN", value),
            None => env::remove_var("GH_TOKEN"),
        }
        match old_token_file {
            Some(value) => env::set_var("ADL_GITHUB_TOKEN_FILE", value),
            None => env::remove_var("ADL_GITHUB_TOKEN_FILE"),
        }
        match old_keychain_service {
            Some(value) => env::set_var("ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE", value),
            None => env::remove_var("ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE"),
        }
        match old_keychain_account {
            Some(value) => env::set_var("ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT", value),
            None => env::remove_var("ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT"),
        }
        match old_disable_default_token_file {
            Some(value) => env::set_var("ADL_TEST_DISABLE_DEFAULT_GITHUB_TOKEN_FILE", value),
            None => env::remove_var("ADL_TEST_DISABLE_DEFAULT_GITHUB_TOKEN_FILE"),
        }
    }
    assert!(err
        .to_string()
        .contains("start: unresolved open PR queue detected for v0.86 [tools:inferred]"));
    assert!(err.to_string().contains("#1169 [draft]"));
}

#[test]
fn real_pr_start_allow_open_pr_wave_skips_wave_scan_before_binding() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-start-open-wave-override-skips-scan");
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
    fs::write(repo.join("README.md"), "override branch placeholder\n").expect("write readme");
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
        1174,
        "v0.86".to_string(),
        "v0-86-tools-preflight-override".to_string(),
    )
    .expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Preflight override");
    write_session_claim(
        &repo,
        1174,
        "thread-self",
        "codex/1174-v0-86-tools-preflight-override",
        &issue_ref.default_worktree_path(&repo, None),
    );

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    let pr_list_marker = repo.join("pr-list-called.marker");
    write_executable(
        &gh_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"pr list\" ]]; then\n  printf called > '{}'\n  printf 'pr list must not be called when --allow-open-pr-wave is explicit\\n' >&2\n  exit 42\nfi\nexit 0\n",
            pr_list_marker.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let old_session = env::var_os("CODEX_SESSION_ID");
    let old_disable_default_token_file = env::var_os("ADL_TEST_DISABLE_DEFAULT_GITHUB_TOKEN_FILE");
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("CODEX_SESSION_ID", "thread-self");
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("ADL_TEST_DISABLE_DEFAULT_GITHUB_TOKEN_FILE", "1");
    }
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "start".to_string(),
        "1174".to_string(),
        "--slug".to_string(),
        "v0-86-tools-preflight-override".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Preflight override".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--no-fetch-issue".to_string(),
        "--allow-open-pr-wave".to_string(),
    ])
    .expect_err("fixture cards may still block after the wave override is honored");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        match old_session {
            Some(value) => env::set_var("CODEX_SESSION_ID", value),
            None => env::remove_var("CODEX_SESSION_ID"),
        }
        env::set_var("PATH", old_path);
        match old_disable_default_token_file {
            Some(value) => env::set_var("ADL_TEST_DISABLE_DEFAULT_GITHUB_TOKEN_FILE", value),
            None => env::remove_var("ADL_TEST_DISABLE_DEFAULT_GITHUB_TOKEN_FILE"),
        }
    }
    assert!(err
        .to_string()
        .contains("design-time card completion gate failed"));
    assert!(
        !pr_list_marker.exists(),
        "explicit --allow-open-pr-wave must skip pr list before binding/card gates"
    );
}

#[test]
fn real_pr_doctor_allow_open_pr_wave_skips_preflight_wave_scan() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-doctor-open-wave-override-skips-scan");
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
    fs::write(repo.join("README.md"), "doctor override placeholder\n").expect("write readme");
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
    let issue_ref = IssueRef::new(
        1175,
        "v0.86".to_string(),
        "v0-86-tools-doctor-override".to_string(),
    )
    .expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Doctor override");

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    let pr_list_marker = repo.join("doctor-pr-list-called.marker");
    write_executable(
        &gh_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"pr list\" ]]; then\n  printf called > '{}'\n  printf 'doctor preflight must not call pr list when --allow-open-pr-wave is explicit\\n' >&2\n  exit 42\nfi\nexit 0\n",
            pr_list_marker.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let old_disable_default_token_file = env::var_os("ADL_TEST_DISABLE_DEFAULT_GITHUB_TOKEN_FILE");
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("ADL_TEST_DISABLE_DEFAULT_GITHUB_TOKEN_FILE", "1");
    }
    env::set_current_dir(&repo).expect("chdir");

    real_pr(&[
        "doctor".to_string(),
        "1175".to_string(),
        "--slug".to_string(),
        "v0-86-tools-doctor-override".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--mode".to_string(),
        "preflight".to_string(),
        "--allow-open-pr-wave".to_string(),
        "--json".to_string(),
    ])
    .expect("doctor preflight should honor explicit wave override");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
        match old_disable_default_token_file {
            Some(value) => env::set_var("ADL_TEST_DISABLE_DEFAULT_GITHUB_TOKEN_FILE", value),
            None => env::remove_var("ADL_TEST_DISABLE_DEFAULT_GITHUB_TOKEN_FILE"),
        }
    }
    assert!(
        !pr_list_marker.exists(),
        "explicit doctor --allow-open-pr-wave must skip pr list during preflight"
    );
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
