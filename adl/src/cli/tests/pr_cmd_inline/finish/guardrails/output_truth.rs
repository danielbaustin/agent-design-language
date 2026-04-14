use super::*;

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
