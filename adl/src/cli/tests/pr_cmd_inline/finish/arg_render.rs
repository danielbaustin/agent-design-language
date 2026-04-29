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
        Some(&render_default_finish_validation(&FinishValidationPlan {
            mode: FinishValidationMode::FullRust,
            commands: vec![
                "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
                "bash adl/tools/check_coverage_impact.sh --base origin/main --include-working-tree --summary adl/target/coverage-impact-summary.json --require-summary-for-risk".to_string(),
                "cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string(),
                "cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings".to_string(),
                "cargo nextest run --manifest-path adl/Cargo.toml --all-features (fallback: cargo test --manifest-path adl/Cargo.toml --all-features when cargo-nextest is unavailable locally)".to_string(),
                "cargo test --manifest-path adl/Cargo.toml --doc --all-features".to_string(),
            ],
        })),
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
        Some(&render_default_finish_validation(&FinishValidationPlan {
            mode: FinishValidationMode::FullRust,
            commands: vec![
                "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
                "bash adl/tools/check_coverage_impact.sh --base origin/main --include-working-tree --summary adl/target/coverage-impact-summary.json --require-summary-for-risk".to_string(),
                "cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string(),
                "cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings".to_string(),
                "cargo nextest run --manifest-path adl/Cargo.toml --all-features (fallback: cargo test --manifest-path adl/Cargo.toml --all-features when cargo-nextest is unavailable locally)".to_string(),
                "cargo test --manifest-path adl/Cargo.toml --doc --all-features".to_string(),
            ],
        })),
        "fp-123",
        &temp,
    )
    .expect_err("issue template text should be rejected");
    assert!(err.to_string().contains("issue-template/prompt text"));
}

#[test]
fn render_pr_body_defaults_docs_only_validation_when_needed() {
    let temp = unique_temp_dir("adl-pr-render-body-docs-only");
    fs::create_dir_all(&temp).expect("temp dir");
    let input = temp.join("input.md");
    let output = temp.join("output.md");
    fs::write(&input, "# input\n").expect("write input");
    fs::write(
        &output,
        "# rust-finish-test\n\n## Summary\nsummary text\n\n## Artifacts produced\n- docs/milestones/v0.89/README.md\n",
    )
    .expect("write output");

    let body = render_pr_body(
        Some("Closes #1153"),
        &input,
        &output,
        None,
        Some(&render_default_finish_validation(&FinishValidationPlan {
            mode: FinishValidationMode::DocsOnly,
            commands: vec![
                "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
                "git diff --check".to_string(),
            ],
        })),
        "fp-123",
        &temp,
    )
    .expect("render body");

    assert!(body.contains("bash adl/tools/check_no_tracked_adl_issue_record_residue.sh"));
    assert!(body.contains("git diff --check"));
    assert!(!body.contains("cargo clippy --all-targets -- -D warnings"));
    assert!(!body.contains("cargo nextest run"));
    assert!(!body.contains("cargo test"));
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
fn finish_helper_paths_cover_ahead_count_and_validation_modes() {
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
    assert_eq!(
        select_finish_validation_plan("docs,README.md")
            .expect("docs-only plan")
            .mode,
        FinishValidationMode::DocsOnly
    );
    run_finish_validation_rust(
        &repo,
        &select_finish_validation_plan("docs,README.md").expect("docs-only plan"),
    )
    .expect("docs-only validation");
    assert!(
        !cargo_log.exists(),
        "docs-only validation should not invoke cargo"
    );

    assert_eq!(
        select_finish_validation_plan("adl,docs")
            .expect("full-rust plan")
            .mode,
        FinishValidationMode::FullRust
    );
    run_finish_validation_rust(
        &repo,
        &select_finish_validation_plan("adl,docs").expect("full-rust plan"),
    )
    .expect("full validation");
    unsafe {
        env::set_var("PATH", old_path);
    }

    let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
    assert!(cargo_calls.contains("fmt --manifest-path"));
    assert!(cargo_calls.contains("clippy --manifest-path"));
    assert!(cargo_calls.contains("nextest --version"));
    assert!(cargo_calls.contains("nextest run --manifest-path"));
    assert!(cargo_calls.contains("test --manifest-path"));
    assert!(cargo_calls.contains("--doc --all-features"));
}

#[test]
fn finish_full_rust_validation_falls_back_when_nextest_is_unavailable() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-nextest-fallback");
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

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let cargo_log = temp.join("cargo.log");
    let cargo_path = bin_dir.join("cargo");
    write_executable(
        &cargo_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"${{1:-}}\" = 'nextest' ] && [ \"${{2:-}}\" = '--version' ]; then\n  exit 1\nfi\nexit 0\n",
            cargo_log.display()
        ),
    );
    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    run_finish_validation_rust(
        &repo,
        &select_finish_validation_plan("adl,docs").expect("full-rust plan"),
    )
    .expect("full validation");
    unsafe {
        env::set_var("PATH", old_path);
    }

    let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
    assert!(cargo_calls.contains("nextest --version"));
    assert!(!cargo_calls.contains("nextest run --manifest-path"));
    assert!(cargo_calls.contains("test --manifest-path"));
    assert!(cargo_calls.contains("--all-features"));
    assert!(cargo_calls.contains("--doc --all-features"));
}

#[test]
fn finish_validation_plan_supports_focused_local_ci_gated_mode() {
    let plan = select_finish_validation_plan(
        "adl/src/cli/pr_cmd/finish_support.rs,adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs,.github/workflows/ci.yaml,adl/tools/check_coverage_impact.sh,adl/tools/ci_path_policy.sh",
    )
    .expect("focused plan");

    assert_eq!(plan.mode, FinishValidationMode::FocusedLocalCiGated);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string()));
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(plan
        .commands
        .contains(&"cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string()));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml cli::pr_cmd::tests::finish".to_string()
    ));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_check_coverage_impact.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_ci_path_policy.sh".to_string()));
    assert!(!plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));
}

#[test]
fn finish_helper_paths_run_focused_local_ci_gated_validation() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-focused-validation");
    let repo = temp.join("repo");
    fs::create_dir_all(repo.join("adl/tools")).expect("adl tools dir");
    fs::create_dir_all(repo.join("adl/src/cli/tests/pr_cmd_inline/finish"))
        .expect("finish tests dir");
    fs::write(
        repo.join("adl/Cargo.toml"),
        "[package]\nname='adl'\nversion='0.1.0'\n",
    )
    .expect("cargo toml");
    write_executable(
        &repo.join("adl/tools/check_no_tracked_adl_issue_record_residue.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 0\n",
    );
    write_executable(
        &repo.join("adl/tools/test_check_coverage_impact.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' coverage >> \"$FOCUSED_LOG\"\n",
    );
    write_executable(
        &repo.join("adl/tools/test_ci_path_policy.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' path-policy >> \"$FOCUSED_LOG\"\n",
    );
    init_git_repo(&repo);

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let cargo_log = temp.join("cargo.log");
    let focused_log = temp.join("focused.log");
    let cargo_path = bin_dir.join("cargo");
    write_executable(
        &cargo_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nexit 0\n",
            cargo_log.display()
        ),
    );
    let old_path = env::var("PATH").unwrap_or_default();
    let old_focused_log = env::var("FOCUSED_LOG").ok();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("FOCUSED_LOG", &focused_log);
    }

    let plan = select_finish_validation_plan(
        "adl/src/cli/pr_cmd/finish_support.rs,.github/workflows/ci.yaml,adl/tools/check_coverage_impact.sh,adl/tools/ci_path_policy.sh",
    )
    .expect("focused plan");
    assert_eq!(plan.mode, FinishValidationMode::FocusedLocalCiGated);
    run_finish_validation_rust(&repo, &plan).expect("focused validation");

    unsafe {
        env::set_var("PATH", old_path);
    }
    match old_focused_log {
        Some(value) => unsafe { env::set_var("FOCUSED_LOG", value) },
        None => unsafe { env::remove_var("FOCUSED_LOG") },
    }

    let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
    assert!(cargo_calls.contains("fmt --manifest-path"));
    assert!(cargo_calls.contains("test --manifest-path"));
    assert!(!cargo_calls.contains("clippy --manifest-path"));

    let focused_calls = fs::read_to_string(&focused_log).expect("focused log");
    assert!(focused_calls.contains("coverage"));
    assert!(focused_calls.contains("path-policy"));
}

#[test]
fn finish_output_card_guards_cover_not_started_and_completed_validation_failures() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-finish-output-guards");
    let tools_dir = repo.join("adl/tools");
    fs::create_dir_all(&tools_dir).expect("tools dir");
    let validator_log = repo.join("validator.log");
    write_executable(
        &tools_dir.join("validate_structured_prompt.sh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" > '{}'\nif [ \"${{VALIDATOR_MODE:-pass}}\" = 'fail' ]; then\n  exit 1\nfi\n",
            validator_log.display()
        ),
    );

    let output = repo.join("sor.md");
    fs::write(&output, "Status: NOT_STARTED\n").expect("write bootstrap sor");
    let err = ensure_output_card_is_started(&output).expect_err("bootstrap sor should fail");
    assert!(err
        .to_string()
        .contains("output card is still bootstrap state"));

    fs::write(&output, "Status: DONE\n").expect("write completed sor");
    validate_completed_sor(&repo, &output).expect("completed sor should validate");
    let validator_call = fs::read_to_string(&validator_log).expect("validator log");
    assert!(validator_call.contains("--type"));
    assert!(validator_call.contains("sor"));
    assert!(validator_call.contains("--phase"));
    assert!(validator_call.contains("completed"));
    assert!(validator_call.contains(&output.display().to_string()));

    unsafe {
        env::set_var("VALIDATOR_MODE", "fail");
    }
    let err = validate_completed_sor(&repo, &output).expect_err("validator failure should bubble");
    unsafe {
        env::remove_var("VALIDATOR_MODE");
    }
    assert!(err
        .to_string()
        .contains("output card failed completed-phase validation"));
    assert!(err.to_string().contains(&output.display().to_string()));
}

#[test]
fn finish_path_tracking_covers_staged_vs_head_changes_and_local_only_issue_surfaces() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-path-tracking");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
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
    fs::write(repo.join("tracked.txt"), "base\n").expect("base file");
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

    fs::write(repo.join("tracked.txt"), "staged change\n").expect("modify tracked");
    stage_selected_paths_rust(&repo, "tracked.txt").expect("stage tracked");
    assert_eq!(
        finish_changed_paths(&repo, true).expect("staged paths"),
        vec!["tracked.txt".to_string()]
    );

    fs::write(repo.join("ahead.txt"), "ahead\n").expect("ahead file");
    assert!(Command::new("git")
        .args(["add", "ahead.txt", "tracked.txt"])
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
    let ahead_paths = finish_changed_paths(&repo, false).expect("ahead paths");
    assert!(ahead_paths.contains(&"ahead.txt".to_string()));
    assert!(ahead_paths.contains(&"tracked.txt".to_string()));

    let issue_ref = IssueRef::new(1153, "v0.86".to_string(), "rust-finish-test".to_string())
        .expect("issue ref");
    let source_path = issue_ref.issue_prompt_path(&repo);
    let input_path = issue_ref.task_bundle_input_path(&repo);
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("source parent");
    fs::create_dir_all(input_path.parent().expect("input parent")).expect("input parent");
    fs::write(&source_path, "---\nissue_card_schema: adl.issue.v1\n---\n").expect("source prompt");
    fs::write(&input_path, "# input\n").expect("input card");
    assert!(Command::new("git")
        .args([
            "add",
            path_str(&source_path).expect("source path"),
            path_str(&input_path).expect("input path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git add issue surfaces")
        .success());

    let tracked = tracked_issue_surface_paths(&repo, &repo, &issue_ref, &source_path)
        .expect("tracked issue surfaces");
    let expected_source = path_relative_to_repo(&repo, &source_path);
    let expected_input = path_relative_to_repo(&repo, &input_path);
    assert_eq!(
        tracked,
        vec![expected_source.clone(), expected_input.clone()]
    );

    let err = ensure_issue_surfaces_are_local_only(&repo, &repo, &issue_ref, &source_path)
        .expect_err("tracked issue surfaces should fail");
    assert!(err
        .to_string()
        .contains("canonical .adl issue surfaces must remain local-only"));
    assert!(err.to_string().contains(&expected_source));
    assert!(err.to_string().contains(&expected_input));
}

#[test]
fn finish_misc_helpers_cover_section_parsing_fingerprint_and_create_outcomes() {
    let temp = unique_temp_dir("adl-pr-finish-misc-helpers");
    let markdown = temp.join("sections.md");
    fs::write(
        &markdown,
        "# title\n\n## Summary\nline one\nline two\n\n## Validation\n- cargo test\n",
    )
    .expect("markdown");
    assert_eq!(
        extract_markdown_section(&markdown, "Summary").expect("summary"),
        "line one\nline two"
    );
    assert_eq!(
        extract_markdown_section(&markdown, "Missing").expect("missing"),
        ""
    );

    assert!(extra_pr_body_looks_like_issue_template("wp: tools"));
    assert!(extra_pr_body_looks_like_issue_template(
        "## Goal\nDo a thing"
    ));
    assert!(!extra_pr_body_looks_like_issue_template(
        "plain implementation note"
    ));

    assert_eq!(
        issue_bundle_issue_number_from_repo_relative(".adl/v0.89/tasks/issue-1847__slug/sor.md"),
        Some(1847)
    );
    assert_eq!(
        issue_bundle_issue_number_from_repo_relative("docs/README.md"),
        None
    );

    let fingerprint = finish_inputs_fingerprint(
        "[v0.89][tests] Add coverage",
        "adl/src/cli/pr_cmd.rs,docs/README.md",
        ".adl/v0.89/tasks/issue-1847__slug/sip.md",
        ".adl/v0.89/tasks/issue-1847__slug/sor.md",
    );
    assert_eq!(
        fingerprint,
        finish_inputs_fingerprint(
            "[v0.89][tests] Add coverage",
            "adl/src/cli/pr_cmd.rs,docs/README.md",
            ".adl/v0.89/tasks/issue-1847__slug/sip.md",
            ".adl/v0.89/tasks/issue-1847__slug/sor.md",
        )
    );
    assert!(!fingerprint.contains('|'));
    assert!(!fingerprint.contains('/'));

    let temp_markdown = write_temp_markdown("adl-pr-finish", "hello world").expect("temp file");
    assert_eq!(
        fs::read_to_string(&temp_markdown).expect("temp contents"),
        "hello world"
    );

    assert_eq!(
        infer_required_outcome_type_for_create("track:roadmap,type:docs", "[v0.89][docs] Refresh"),
        "docs"
    );
    assert_eq!(
        infer_required_outcome_type_for_create(
            "track:roadmap,area:tests",
            "[v0.89] Improve coverage"
        ),
        "tests"
    );
    assert_eq!(
        infer_required_outcome_type_for_create("track:roadmap", "[demo] Show the workflow"),
        "demo"
    );
    assert_eq!(
        infer_required_outcome_type_for_create("track:roadmap,type:task", "[v0.89] Ship code"),
        "code"
    );
}

fn restore_env(key: &str, value: Option<String>) {
    unsafe {
        if let Some(value) = value {
            env::set_var(key, value);
        } else {
            env::remove_var(key);
        }
    }
}

#[test]
fn finish_local_helpers_cover_checkout_and_file_truth_edges() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-finish-support-local-helpers");
    let left = temp.join("left");
    fs::create_dir_all(&left).expect("left dir");
    assert!(same_checkout_root(&left, &left).expect("same path"));

    let linked_target = temp.join("linked-target");
    let linked_alias = temp.join("linked-alias");
    fs::create_dir_all(&linked_target).expect("linked target");
    std::os::unix::fs::symlink(&linked_target, &linked_alias).expect("symlink");
    assert!(same_checkout_root(&linked_target, &linked_alias).expect("canonical match"));

    let missing = temp.join("missing.txt");
    assert!(!ensure_nonempty_file_path(&missing).expect("missing path should be false"));

    let whitespace = temp.join("whitespace.txt");
    fs::write(&whitespace, " \n\t").expect("whitespace file");
    assert!(!ensure_nonempty_file_path(&whitespace).expect("whitespace file"));

    let text = temp.join("text.txt");
    fs::write(&text, "ready\n").expect("text file");
    assert!(ensure_nonempty_file_path(&text).expect("nonempty file"));

    let started = temp.join("started.md");
    fs::write(&started, "Status: IN_PROGRESS\n").expect("started sor");
    ensure_output_card_is_started(&started).expect("started sor accepted");

    let bootstrap = temp.join("bootstrap.md");
    fs::write(&bootstrap, "Status: NOT_STARTED\n").expect("bootstrap sor");
    let err = ensure_output_card_is_started(&bootstrap).expect_err("bootstrap sor rejected");
    assert!(err.to_string().contains("Status: NOT_STARTED"));
}

#[test]
fn finish_validation_plan_and_runner_cover_docs_focused_full_and_error_paths() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-finish-support-validation");
    let repo = temp.join("repo");
    let adl_dir = repo.join("adl");
    let tools_dir = adl_dir.join("tools");
    let target_dir = adl_dir.join("target");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&tools_dir).expect("tools dir");
    fs::create_dir_all(&target_dir).expect("target dir");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    fs::write(
        adl_dir.join("Cargo.toml"),
        "[package]\nname='adl'\nversion='0.1.0'\n",
    )
    .expect("manifest");

    let guard_log = temp.join("guard.log");
    let diff_log = temp.join("diff.log");
    let cargo_log = temp.join("cargo.log");
    let coverage_log = temp.join("coverage.log");
    let ci_log = temp.join("ci.log");
    let validator_log = temp.join("validator.log");

    write_executable(
        &tools_dir.join("check_no_tracked_adl_issue_record_residue.sh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf 'guard\\n' >> '{}'\n",
            guard_log.display()
        ),
    );
    write_executable(
        &tools_dir.join("test_check_coverage_impact.sh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\n",
            coverage_log.display()
        ),
    );
    write_executable(
        &tools_dir.join("test_ci_path_policy.sh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\n",
            ci_log.display()
        ),
    );
    write_executable(
        &tools_dir.join("check_coverage_impact.sh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\n",
            coverage_log.display()
        ),
    );
    write_executable(
        &tools_dir.join("validate_structured_prompt.sh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\ncase \"$*\" in *broken*) exit 8 ;; *) exit 0 ;; esac\n",
            validator_log.display()
        ),
    );
    write_executable(
        &bin_dir.join("git"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nexit 0\n",
            diff_log.display()
        ),
    );
    write_executable(
        &bin_dir.join("cargo"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2\" = 'nextest --version' ]; then\n  exit 1\nfi\nexit 0\n",
            cargo_log.display()
        ),
    );

    let old_path = env::var("PATH").ok();
    let mut path_entries = vec![bin_dir.clone()];
    path_entries.extend(env::split_paths(old_path.as_deref().unwrap_or("")));
    unsafe {
        env::set_var("PATH", env::join_paths(path_entries).expect("join PATH"));
    }

    let docs_plan = select_finish_validation_plan("docs,README.md").expect("docs plan");
    assert_eq!(docs_plan.mode, FinishValidationMode::DocsOnly);
    run_finish_validation_rust(&repo, &docs_plan).expect("docs plan runs");

    let focused_plan = select_finish_validation_plan(
        "adl/src/cli/pr_cmd/finish_support.rs,.github/workflows/ci.yaml,adl/tools/ci_path_policy.sh",
    )
    .expect("focused plan");
    assert_eq!(focused_plan.mode, FinishValidationMode::FocusedLocalCiGated);
    run_finish_validation_rust(&repo, &focused_plan).expect("focused plan runs");

    let unsupported = FinishValidationPlan {
        mode: FinishValidationMode::FocusedLocalCiGated,
        commands: vec![
            "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
            "git diff --check".to_string(),
            "unsupported".to_string(),
        ],
    };
    let err = run_finish_validation_rust(&repo, &unsupported).expect_err("unsupported command");
    assert!(err.to_string().contains("unsupported focused validation command"));

    let full_plan = select_finish_validation_plan("adl/src/lib.rs").expect("full rust plan");
    assert_eq!(full_plan.mode, FinishValidationMode::FullRust);
    run_finish_validation_rust(&repo, &full_plan).expect("full plan runs");

    let completed = temp.join("completed.sor.md");
    fs::write(&completed, "Status: COMPLETED\n").expect("completed sor");
    validate_completed_sor(&repo, &completed).expect("validator success");

    let broken = temp.join("broken.sor.md");
    fs::write(&broken, "Status: COMPLETED\n").expect("broken sor");
    let err = validate_completed_sor(&repo, &broken).expect_err("validator failure");
    assert!(err
        .to_string()
        .contains("output card failed completed-phase validation"));

    restore_env("PATH", old_path);

    assert!(fs::read_to_string(&guard_log)
        .expect("guard log")
        .contains("guard"));
    assert!(fs::read_to_string(&cargo_log)
        .expect("cargo log")
        .contains("test --manifest-path"));
    assert!(fs::read_to_string(&coverage_log)
        .expect("coverage log")
        .contains("--require-summary-for-risk"));
    assert!(fs::metadata(&ci_log).expect("ci log metadata").len() > 0);
    assert!(fs::read_to_string(&validator_log)
        .expect("validator log")
        .contains("--phase"));
}

#[test]
fn finish_surface_helpers_cover_tracking_stage_and_body_rendering() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-finish-support-surfaces");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
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
    fs::create_dir_all(repo.join(".adl/v0.86/tasks/issue-1153__rust-finish-test"))
        .expect("task dir");
    fs::create_dir_all(repo.join("docs")).expect("docs dir");
    fs::write(repo.join("docs/README.md"), "hello\n").expect("doc");
    assert!(Command::new("git")
        .args(["add", "docs/README.md"])
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

    let issue_ref =
        IssueRef::new(1153, "v0.86".to_string(), "rust-finish-test".to_string()).expect("issue ref");
    let source = issue_ref.issue_prompt_path(&repo);
    let root_stp = issue_ref.task_bundle_stp_path(&repo);
    let root_input = issue_ref.task_bundle_input_path(&repo);
    let root_output = issue_ref.task_bundle_output_path(&repo);
    fs::create_dir_all(source.parent().expect("source parent")).expect("source parent dir");
    fs::write(&source, "prompt\n").expect("source");
    fs::write(&root_stp, "stp\n").expect("stp");
    fs::write(&root_input, "sip\n").expect("sip");
    fs::write(
        &root_output,
        "Status: COMPLETED\n\n## Summary\nDone\n\n## Artifacts produced\n- a\n",
    )
    .expect("sor");

    assert!(stage_selected_paths_rust(&repo, "docs/README.md").is_ok());
    let tracked =
        tracked_issue_surface_paths(&repo, &repo, &issue_ref, &source).expect("tracked paths");
    assert!(tracked.is_empty());
    assert!(ensure_issue_surfaces_are_local_only(&repo, &repo, &issue_ref, &source).is_ok());

    assert!(Command::new("git")
        .args([
            "add",
            path_relative_to_repo(&repo, &source).as_str(),
            path_relative_to_repo(&repo, &root_stp).as_str(),
            path_relative_to_repo(&repo, &root_input).as_str(),
            path_relative_to_repo(&repo, &root_output).as_str(),
        ])
        .current_dir(&repo)
        .status()
        .expect("git add local surfaces")
        .success());
    let err = ensure_issue_surfaces_are_local_only(&repo, &repo, &issue_ref, &source)
        .expect_err("tracked surfaces should fail");
    assert!(err
        .to_string()
        .contains("canonical .adl issue surfaces must remain local-only"));

    let input = temp.join("input.md");
    fs::write(&input, "Status: ACTIVE\n").expect("input");
    let output = temp.join("output.md");
    fs::write(
        &output,
        "Status: COMPLETED\n\n## Summary\nSummary text\n\n## Validation\ncargo test\n",
    )
    .expect("output");
    let body = render_pr_body(
        Some("Closes #1153"),
        &input,
        &output,
        Some("Extra notes"),
        Some("- fallback validation"),
        "fingerprint-123",
        &repo,
    )
    .expect("pr body");
    assert!(body.contains("## Summary"));
    assert!(body.contains("## Validation"));
    assert!(body.contains("## Notes"));
    assert!(body.contains("Idempotency-Key: fingerprint-123"));
    assert_eq!(
        finish_inputs_fingerprint("Title", "adl", "input.md", "output.md"),
        finish_inputs_fingerprint("Title", "adl", "input.md", "output.md")
    );

    let temp_md = write_temp_markdown("finish-support-test", "body").expect("temp markdown");
    assert_eq!(fs::read_to_string(temp_md).expect("temp body"), "body");
}
