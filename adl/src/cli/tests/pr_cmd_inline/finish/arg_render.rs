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
