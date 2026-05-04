use super::*;

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
    assert!(
        !repo
            .join(".adl/logs/post-merge-closeout/issue-1161")
            .exists(),
        "broad finish tests should not spawn post-merge closeout watcher artifacts by default"
    );
}
