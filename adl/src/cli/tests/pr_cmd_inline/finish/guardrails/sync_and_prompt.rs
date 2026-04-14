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
