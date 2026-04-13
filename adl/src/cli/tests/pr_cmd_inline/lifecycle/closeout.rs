use super::*;

#[test]
fn real_pr_closeout_reconciles_closed_completed_issue_bundle() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-closeout-success");
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
    fs::write(repo.join("README.md"), "closeout success\n").expect("seed file");
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
    let issue_ref = IssueRef::new(
        1596,
        "v0.87.1",
        "v0-87-1-tools-make-closeout-automatic-after-merge-closure",
    )
    .expect("issue ref");
    write_authored_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.87.1][tools] Make closeout automatic after merge/closure",
    );
    real_pr(&[
        "init".to_string(),
        "1596".to_string(),
        "--slug".to_string(),
        issue_ref.slug().to_string(),
        "--title".to_string(),
        "[v0.87.1][tools] Make closeout automatic after merge/closure".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.87.1".to_string(),
    ])
    .expect("real_pr init");

    let sip_path = issue_ref.task_bundle_input_path(&repo);
    write_authored_sip(
        &sip_path,
        &issue_ref,
        "[v0.87.1][tools] Make closeout automatic after merge/closure",
        "codex/1596-v0-87-1-tools-make-closeout-automatic-after-merge-closure",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    let sor_path = issue_ref.task_bundle_output_path(&repo);
    write_completed_sor_fixture(
        &sor_path,
        "codex/1596-v0-87-1-tools-make-closeout-automatic-after-merge-closure",
    );

    let worktree = issue_ref.default_worktree_path(&repo, None);
    assert!(Command::new("git")
        .args([
            "worktree",
            "add",
            "-q",
            "-b",
            "codex/1596-v0-87-1-tools-make-closeout-automatic-after-merge-closure",
            path_str(&worktree).expect("worktree path"),
            "origin/main",
        ])
        .current_dir(&repo)
        .status()
        .expect("git worktree add")
        .success());
    assert!(worktree.is_dir(), "closeout fixture worktree should exist");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2 $3 $4\" == \"issue view 1596 -R\" ]]; then\n  echo '{\"state\":\"CLOSED\",\"stateReason\":\"COMPLETED\"}'\n  exit 0\nfi\nexit 1\n",
    );
    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let closeout = real_pr(&[
        "closeout".to_string(),
        "1596".to_string(),
        "--slug".to_string(),
        issue_ref.slug().to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.87.1".to_string(),
    ]);

    unsafe {
        env::set_var("PATH", old_path);
    }
    env::set_current_dir(prev_dir).expect("restore cwd");
    closeout.expect("closeout closed reconcile");

    let canonical_text = fs::read_to_string(&sor_path).expect("read canonical sor");
    assert!(canonical_text.contains("Status: DONE"));
    assert!(canonical_text.contains("- Integration state: merged"));
    assert!(canonical_text.contains("- Verification scope: main_repo"));
    assert!(canonical_text.contains("- Worktree-only paths remaining: none"));
    assert!(!worktree.exists());
}

#[test]
fn real_pr_closeout_refuses_issue_that_is_not_completed() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-closeout-refuse");
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
    fs::write(repo.join("README.md"), "closeout refusal\n").expect("seed file");
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

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(
        1596,
        "v0.87.1",
        "v0-87-1-tools-make-closeout-automatic-after-merge-closure",
    )
    .expect("issue ref");
    write_authored_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.87.1][tools] Make closeout automatic after merge/closure",
    );
    real_pr(&[
        "init".to_string(),
        "1596".to_string(),
        "--slug".to_string(),
        issue_ref.slug().to_string(),
        "--title".to_string(),
        "[v0.87.1][tools] Make closeout automatic after merge/closure".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.87.1".to_string(),
    ])
    .expect("real_pr init");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2 $3 $4\" == \"issue view 1596 -R\" ]]; then\n  echo '{\"state\":\"OPEN\",\"stateReason\":\"REOPENED\"}'\n  exit 0\nfi\nexit 1\n",
    );
    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let closeout = real_pr(&[
        "closeout".to_string(),
        "1596".to_string(),
        "--slug".to_string(),
        issue_ref.slug().to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.87.1".to_string(),
    ]);

    unsafe {
        env::set_var("PATH", old_path);
    }
    env::set_current_dir(prev_dir).expect("restore cwd");

    let err = closeout.expect_err("closeout should refuse unfinished issue");
    assert!(err.to_string().contains("refusing automatic closeout"));
}
