use super::*;
use crate::cli::pr_cmd::finish_support::{
    ensure_finish_branch_not_behind_origin_main, ensure_finish_task_bundle_surfaces,
    normalize_docs_only_sor_text, open_pr_url_nonblocking, open_pr_url_nonblocking_with_timeout,
    real_pr_finish, resolve_finish_issue_scope_and_slug, select_finish_validation_plan_for_finish,
    FinishValidationMode, FinishValidationPlan,
};
use crate::cli::pr_cmd::git_support::commits_behind_origin_main;

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
fn local_pr_url_opener_failure_is_non_blocking_warning() {
    let temp = unique_temp_dir("adl-pr-url-opener-warning");
    let opener = temp.join("open");
    write_executable(
        &opener,
        "#!/usr/bin/env bash\nset -euo pipefail\necho 'No application knows how to open URL' >&2\nexit 42\n",
    );

    let result = open_pr_url_nonblocking(
        path_str(&opener).expect("opener path"),
        "https://github.com/danielbaustin/agent-design-language/pull/3830",
    );

    assert!(!result.success);
    assert!(result
        .warning
        .contains("warning: local PR URL opener failed"));
    assert!(result.warning.contains("PR publication already succeeded"));
    assert!(result.warning.contains(
        "Open manually: https://github.com/danielbaustin/agent-design-language/pull/3830"
    ));
    assert!(result
        .warning
        .contains("No application knows how to open URL"));
}

#[test]
fn local_pr_url_opener_spawn_failure_is_non_blocking_warning() {
    let result = open_pr_url_nonblocking(
        "/definitely/missing/adl-pr-url-opener",
        "https://github.com/danielbaustin/agent-design-language/pull/3830",
    );

    assert!(!result.success);
    assert!(result
        .warning
        .contains("warning: local PR URL opener could not start"));
    assert!(result.warning.contains("PR publication already succeeded"));
    assert!(result.warning.contains(
        "Open manually: https://github.com/danielbaustin/agent-design-language/pull/3830"
    ));
}

#[test]
fn local_pr_url_opener_timeout_is_non_blocking_warning() {
    let temp = unique_temp_dir("adl-pr-url-opener-timeout");
    let opener = temp.join("open");
    write_executable(&opener, "#!/usr/bin/env bash\nset -euo pipefail\nsleep 5\n");

    let result = open_pr_url_nonblocking_with_timeout(
        path_str(&opener).expect("opener path"),
        "https://github.com/danielbaustin/agent-design-language/pull/3830",
        std::time::Duration::from_millis(100),
    );

    assert!(!result.success);
    assert!(result
        .warning
        .contains("warning: local PR URL opener timed out"));
    assert!(result.warning.contains("PR publication already succeeded"));
    assert!(result.warning.contains(
        "Open manually: https://github.com/danielbaustin/agent-design-language/pull/3830"
    ));
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
            mode: FinishValidationMode::LargerBinaryFocused,
            commands: vec![
                "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
                "cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string(),
                "cargo test --manifest-path adl/Cargo.toml --bin adl cli::pr_cmd".to_string(),
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
            mode: FinishValidationMode::LargerBinaryFocused,
            commands: vec![
                "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
                "cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string(),
                "cargo test --manifest-path adl/Cargo.toml --bin adl cli::pr_cmd".to_string(),
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
fn docs_only_sor_normalization_repairs_aliases_and_ingests_validation_evidence() {
    let input = r#"# issue-3738

Task ID: issue-3738
Run ID: issue-3738
Version: v0.91.5
Title: Example
Branch: codex/example
Card Status: ready
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5
- Provider: OpenAI
- Start Time: 2026-06-16T00:00:00Z
- End Time: 2026-06-16T00:00:01Z

## Summary

done

## Artifacts produced
- docs/example.md

## Actions taken
- did the thing

## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
  - `docs/example.md`
- Worktree-only paths remaining: none
- Worktree prune result: not_run
- Integration state: open_pr
- Verification scope: main-repo
- Integration method used: manual
- Verification performed:
  - `python3 - <<'PY' ...`
    Existing docs-only proof.
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `python3 - <<'PY' ...`
    Existing docs-only proof.
- Results:
  - PASS

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "python3 - <<'PY' ..."
  determinism:
    status: NOT_RUN
```

## Determinism Evidence
- not_run

## Security / Privacy Checks
- ok

## Replay Artifacts
- not_applicable

## Artifact Verification
- docs/example.md

## Decisions / Deviations
- none

## Follow-ups / Deferred work
- none
"#;

    let plan = FinishValidationPlan {
        mode: FinishValidationMode::DocsOnly,
        commands: vec![
            "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
            "git diff --check".to_string(),
        ],
    };

    let normalized = normalize_docs_only_sor_text(input, &plan.commands);

    assert!(normalized.contains("- Integration state: pr_open"));
    assert!(normalized.contains("- Verification scope: main_repo"));
    assert!(normalized.contains("`bash adl/tools/check_no_tracked_adl_issue_record_residue.sh`"));
    assert!(normalized.contains("`git diff --check`"));
    assert!(normalized.contains("\"bash adl/tools/check_no_tracked_adl_issue_record_residue.sh\""));
    assert!(normalized.contains("\"git diff --check\""));
}

#[test]
fn docs_only_sor_normalization_is_idempotent_for_existing_entries() {
    let input = r#"## Validation
- Validation commands and their purpose:
  - `git diff --check`
    Verified whitespace and patch hygiene on the docs-only changed surfaces.
- Results:
  - PASS

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "git diff --check"
  determinism:
    status: NOT_RUN
```
"#;

    let commands = vec!["git diff --check".to_string()];
    let normalized = normalize_docs_only_sor_text(input, &commands);
    assert_eq!(normalized.matches("git diff --check").count(), 2);
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

    let err = select_finish_validation_plan("adl,docs").expect_err("unclassified plan should fail");
    unsafe {
        env::set_var("PATH", old_path);
    }

    assert!(err
        .to_string()
        .contains("not classified into docs-only, small-binary focused, or larger-binary focused"));
}

#[test]
fn finish_guard_blocks_branch_behind_origin_main_before_validation() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-stale-base-guard");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    init_git_repo(&repo);
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&repo)
        .status()
        .expect("git config");
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .status()
        .expect("git config");
    fs::write(repo.join("README.md"), "base\n").expect("readme");
    Command::new("git")
        .args(["add", "README.md"])
        .current_dir(&repo)
        .status()
        .expect("git add");
    Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit");
    Command::new("git")
        .args([
            "init",
            "--bare",
            "-q",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git init bare");
    Command::new("git")
        .args([
            "remote",
            "set-url",
            "origin",
            path_str(&origin).expect("origin path"),
        ])
        .current_dir(&repo)
        .status()
        .expect("git remote set-url");
    Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch");
    Command::new("git")
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push");
    Command::new("git")
        .args(["symbolic-ref", "HEAD", "refs/heads/main"])
        .current_dir(&origin)
        .status()
        .expect("git origin head");
    ensure_finish_branch_not_behind_origin_main(&repo).expect("fresh branch");

    let upstream = temp.join("upstream");
    Command::new("git")
        .args([
            "clone",
            "-q",
            path_str(&origin).expect("origin path"),
            path_str(&upstream).expect("upstream path"),
        ])
        .status()
        .expect("git clone");
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(&upstream)
        .status()
        .expect("git config");
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&upstream)
        .status()
        .expect("git config");
    fs::write(upstream.join("README.md"), "upstream\n").expect("upstream readme");
    Command::new("git")
        .args(["add", "README.md"])
        .current_dir(&upstream)
        .status()
        .expect("git add");
    Command::new("git")
        .args(["commit", "-q", "-m", "upstream"])
        .current_dir(&upstream)
        .status()
        .expect("git commit");
    Command::new("git")
        .args(["push", "-q", "origin", "main"])
        .current_dir(&upstream)
        .status()
        .expect("git push");
    Command::new("git")
        .args(["fetch", "-q", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git fetch");

    assert_eq!(commits_behind_origin_main(&repo).expect("behind count"), 1);
    let err = ensure_finish_branch_not_behind_origin_main(&repo).expect_err("stale branch");
    let message = err.to_string();
    assert!(message.contains("finish: branch is behind origin/main by 1 commit(s)"));
    assert!(message.contains("rebase before publication"));
    assert!(message.contains("coverage-impact false positives"));
    assert!(message.contains("git fetch origin main && git rebase origin/main --autostash"));
}

#[test]
fn finish_unclassified_paths_fail_closed_instead_of_widening_to_repo_wide_rust_validation() {
    let err = select_finish_validation_plan("adl,docs").expect_err("unclassified plan should fail");
    assert!(err
        .to_string()
        .contains("not classified into docs-only, small-binary focused, or larger-binary focused"));
}

#[test]
fn finish_validation_sanitizes_live_github_transport_env() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-sanitized-github-env");
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
    let cargo_log = temp.join("cargo-env.log");
    write_executable(
        &bin_dir.join("cargo"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf 'args=%s ADL_GITHUB_CLIENT=%s ADL_GITHUB_DISABLE_GH_FALLBACK=%s ADL_GITHUB_OCTOCRAB_BASE_URI=%s GITHUB_TOKEN=%s GH_TOKEN=%s\\n' \"$*\" \"${{ADL_GITHUB_CLIENT-}}\" \"${{ADL_GITHUB_DISABLE_GH_FALLBACK-}}\" \"${{ADL_GITHUB_OCTOCRAB_BASE_URI-}}\" \"${{GITHUB_TOKEN-}}\" \"${{GH_TOKEN-}}\" >> '{}'\nexit 0\n",
            cargo_log.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let github_envs = [
        "ADL_GITHUB_CLIENT",
        "ADL_GITHUB_DISABLE_GH_FALLBACK",
        "ADL_GITHUB_OCTOCRAB_BASE_URI",
        "GITHUB_TOKEN",
        "GH_TOKEN",
    ];
    let old_github_envs = github_envs
        .iter()
        .map(|key| (*key, env::var(key).ok()))
        .collect::<Vec<_>>();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("ADL_GITHUB_CLIENT", "octocrab");
        env::set_var("ADL_GITHUB_DISABLE_GH_FALLBACK", "1");
        env::set_var("ADL_GITHUB_OCTOCRAB_BASE_URI", "http://127.0.0.1:9");
        env::set_var("GITHUB_TOKEN", "github-secret-token");
        env::set_var("GH_TOKEN", "gh-secret-token");
    }

    run_finish_validation_rust(
        &repo,
        &select_finish_validation_plan("adl/src/cli/pr_cmd/doctor.rs")
            .expect("larger binary focused plan"),
    )
    .expect("focused validation should not inherit live GitHub transport env");

    unsafe {
        env::set_var("PATH", old_path);
        for (key, value) in old_github_envs {
            match value {
                Some(value) => env::set_var(key, value),
                None => env::remove_var(key),
            }
        }
    }

    let cargo_env = fs::read_to_string(&cargo_log).expect("cargo env log");
    assert!(cargo_env.contains("args=test --manifest-path"));
    assert!(!cargo_env.contains("nextest run --manifest-path"));
    assert!(!cargo_env.contains("octocrab"));
    assert!(!cargo_env.contains("github-secret-token"));
    assert!(!cargo_env.contains("gh-secret-token"));
    assert!(!cargo_env.contains("127.0.0.1:9"));
    assert!(cargo_env
        .lines()
        .all(|line| line.contains("ADL_GITHUB_CLIENT= ADL_GITHUB_DISABLE_GH_FALLBACK=")));
}

#[test]
fn finish_validation_plan_supports_focused_local_ci_gated_mode() {
    let plan = select_finish_validation_plan(
        "adl/src/cli/pr_cmd/doctor.rs,adl/src/cli/pr_cmd/lifecycle/tests.rs,adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs,.github/workflows/ci.yaml,adl/tools/check_coverage_impact.sh,adl/tools/ci_path_policy.sh,docs/tooling/merge_readiness_gate_policy_v0.91.4.md,docs/milestones/v0.91.4/DEMO_MATRIX_v0.91.4.md,docs/milestones/v0.91.4/FEATURE_PROOF_COVERAGE_v0.91.4.md,docs/milestones/v0.91.4/features/MERGE_READINESS_AND_PR_GATE_HARDENING.md,docs/milestones/v0.91.4/review/merge_readiness/ct_demo_001_merge_gate_profile_report.md",
    )
    .expect("focused plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string()));
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(plan
        .commands
        .contains(&"cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string()));
    assert!(plan
        .commands
        .contains(&"cargo test --manifest-path adl/Cargo.toml --bin adl cli::pr_cmd".to_string()));
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
fn finish_validation_plan_classifies_owner_validation_lanes() {
    let csdlc_plan = select_finish_validation_plan(
        "adl/tools/run_owner_validation_lane.sh,docs/milestones/v0.91.5/LOCAL_VS_CI_VALIDATION_POLICY_3607.md",
    )
    .expect("owner lane plan");
    assert_eq!(csdlc_plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(csdlc_plan
        .commands
        .contains(&"bash adl/tools/test_owner_validation_lane.sh".to_string()));
    assert!(csdlc_plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh csdlc".to_string()));
    assert!(csdlc_plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh all --build".to_string()));
    assert!(!csdlc_plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));
    let csdlc_rendered_validation = render_default_finish_validation(&csdlc_plan);
    assert!(!csdlc_rendered_validation.contains("cargo nextest run"));
    assert!(csdlc_rendered_validation.contains("larger owner-binary focused build/test only"));
    assert!(csdlc_rendered_validation.contains("CI integration proof"));

    let runtime_plan = select_finish_validation_plan("adl/tools/test_adl_runtime_compatibility.sh")
        .expect("runtime owner lane plan");
    assert_eq!(runtime_plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(runtime_plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh runtime --build".to_string()));
    assert!(!runtime_plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));

    let review_plan = select_finish_validation_plan("adl/tools/test_adl_review_compatibility.sh")
        .expect("review owner lane plan");
    assert_eq!(review_plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(review_plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh review --build".to_string()));
    assert!(!review_plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));
}

#[test]
fn finish_validation_plan_classifies_rust_refactor_slices() {
    let prompt_editor_plan = select_finish_validation_plan(
        "adl/src/csdlc_prompt_editor.rs,adl/src/csdlc_prompt_editor/structure.rs",
    )
    .expect("prompt editor refactor plan");
    assert_eq!(
        prompt_editor_plan.mode,
        FinishValidationMode::LargerBinaryFocused
    );
    assert!(prompt_editor_plan
        .commands
        .contains(&"cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string()));
    assert!(prompt_editor_plan
        .commands
        .contains(&"cargo test --manifest-path adl/Cargo.toml --bin adl".to_string()));

    let run_artifacts_plan = select_finish_validation_plan(
        "adl/src/cli/run_artifacts_types.rs,adl/src/cli/run_artifacts_types/state.rs",
    )
    .expect("run artifacts refactor plan");
    assert_eq!(
        run_artifacts_plan.mode,
        FinishValidationMode::LargerBinaryFocused
    );
    assert!(run_artifacts_plan
        .commands
        .contains(&"cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string()));
    assert!(run_artifacts_plan
        .commands
        .contains(&"cargo test --manifest-path adl/Cargo.toml --bin adl".to_string()));
}

#[test]
fn finish_validation_profile_uses_actual_changed_paths_not_broad_stage_request() {
    let docs_plan = select_finish_validation_plan_for_finish(
        ".",
        &["docs/milestones/v0.91.3/review/example.md".to_string()],
    )
    .expect("docs-only actual path plan");
    assert_eq!(docs_plan.mode, FinishValidationMode::DocsOnly);
    assert!(!docs_plan
        .commands
        .iter()
        .any(|command: &String| command.contains("cargo nextest")));

    let focused_plan = select_finish_validation_plan_for_finish(
        ".",
        &["adl/src/cli/pr_cmd/doctor.rs".to_string()],
    )
    .expect("focused actual path plan");
    assert_eq!(focused_plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(focused_plan
        .commands
        .contains(&"cargo test --manifest-path adl/Cargo.toml --bin adl cli::pr_cmd".to_string()));
    assert!(!focused_plan
        .commands
        .iter()
        .any(|command: &String| command.contains("cargo clippy")));
}

#[test]
fn finish_validation_profile_treats_issue_records_and_skill_docs_as_docs_only() {
    let plan = select_finish_validation_plan_for_finish(
        ".",
        &[
            "adl/tools/skills/docs/CI_RUNTIME_POLICY_GUIDE.md".to_string(),
            "adl/tools/skills/pr-finish/SKILL.md".to_string(),
        ],
    )
    .expect("docs-only review artifact plan");

    assert_eq!(plan.mode, FinishValidationMode::DocsOnly);
    assert_eq!(
        plan.commands,
        vec![
            "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
            "git diff --check".to_string(),
        ]
    );
}

#[test]
fn finish_validation_profile_does_not_treat_behavioral_tooling_as_docs_only() {
    let plan = select_finish_validation_plan_for_finish(
        ".",
        &[
            "adl/tools/pr.sh".to_string(),
            "docs/milestones/v0.91.5/DOCS_ONLY_VALIDATION_BUNDLE_3736.md".to_string(),
        ],
    )
    .expect("behavioral tooling plan");

    assert_eq!(plan.mode, FinishValidationMode::SmallBinaryFocused);
}

#[test]
fn finish_validation_profile_keeps_public_prompt_packet_changes_focused() {
    let plan = select_finish_validation_plan_for_finish(
        ".",
        &[
            "adl/src/cli/tooling_cmd/public_prompt_packet.rs".to_string(),
            "adl/src/cli/tooling_cmd/tests/public_prompt_packet.rs".to_string(),
            "docs/milestones/v0.91.5/features/PUBLIC_PROMPT_RECORDS_v0.91.5.md".to_string(),
        ],
    )
    .expect("public prompt packet plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string()));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-csdlc public_prompt_packet"
            .to_string()
    ));
    assert!(!plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));
    assert!(!plan
        .commands
        .iter()
        .any(|command| command.contains("cargo nextest")));
    assert!(!plan
        .commands
        .iter()
        .any(|command| command.contains("--doc --all-features")));
}

#[test]
fn finish_validation_profile_keeps_finish_support_changes_narrow() {
    let plan = select_finish_validation_plan_for_finish(
        ".",
        &[
            "adl/src/cli/pr_cmd/finish_support.rs".to_string(),
            "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs".to_string(),
            "docs/default_workflow.md".to_string(),
        ],
    )
    .expect("finish support plan");

    assert_eq!(plan.mode, FinishValidationMode::SmallBinaryFocused);
    assert!(plan
        .commands
        .contains(&"cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string()));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation"
            .to_string()
    ));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_helper_paths_run_focused_local_ci_gated_validation"
            .to_string()
    ));
    assert!(!plan
        .commands
        .contains(&"cargo test --manifest-path adl/Cargo.toml --bin adl cli::pr_cmd".to_string()));
    assert!(!plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));
    assert!(!plan
        .commands
        .iter()
        .any(|command| command.contains("cargo nextest")));
}

#[test]
fn finish_validation_profile_keeps_small_binary_delegation_proof_focused() {
    let plan = select_finish_validation_plan_for_finish(
        ".",
        &[
            "adl/tools/test_pr_small_binary_delegation.sh".to_string(),
            "docs/milestones/v0.91.5/review/tooling_adoption/PR_LIFECYCLE_SMALL_BINARIES_PROOF_3838.md"
                .to_string(),
        ],
    )
    .expect("small-binary delegation proof plan");

    assert_eq!(plan.mode, FinishValidationMode::SmallBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_pr_small_binary_delegation.sh".to_string()));
    assert!(!plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));
    assert!(!plan
        .commands
        .iter()
        .any(|command| command.contains("cargo nextest")));
}

#[test]
fn finish_restores_missing_canonical_cards_from_slug_drifted_issue_bundle() {
    let repo = unique_temp_dir("adl-pr-finish-slug-drift");
    let issue_ref = IssueRef::new(
        3766,
        "v0.91.5".to_string(),
        "canonical-finish-slug".to_string(),
    )
    .expect("issue ref");
    let tasks_dir = repo.join(".adl").join("v0.91.5").join("tasks");
    let drifted_dir = tasks_dir.join("issue-3766__v0-91-5-tools-title-derived-slug");
    fs::create_dir_all(&drifted_dir).expect("drifted bundle dir");
    for file_name in ["stp.md", "sip.md", "sor.md", "spp.md", "srp.md"] {
        fs::write(
            drifted_dir.join(file_name),
            format!("{file_name} restored from title-derived slug\n"),
        )
        .expect("write drifted card");
    }

    ensure_finish_task_bundle_surfaces(&repo, &issue_ref).expect("restore canonical cards");

    let canonical_dir = issue_ref.task_bundle_dir_path(&repo);
    for file_name in ["stp.md", "sip.md", "sor.md", "spp.md", "srp.md"] {
        let restored = fs::read_to_string(canonical_dir.join(file_name)).expect("read restored");
        assert_eq!(
            restored,
            format!("{file_name} restored from title-derived slug\n")
        );
    }
}

#[test]
fn finish_identity_resolution_prefers_bound_worktree_local_bundle() {
    let primary = unique_temp_dir("adl-pr-finish-identity-primary");
    let worktree = unique_temp_dir("adl-pr-finish-identity-worktree");
    fs::create_dir_all(worktree.join(".adl/v0.91.5/tasks/issue-3766__worktree-local-finish-slug"))
        .expect("worktree bundle");

    let identity =
        resolve_finish_issue_scope_and_slug(&worktree, &primary, 3766).expect("identity");

    assert_eq!(
        identity,
        (
            "v0.91.5".to_string(),
            "worktree-local-finish-slug".to_string()
        )
    );
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
        "adl/src/cli/pr_cmd/doctor.rs,adl/src/cli/pr_cmd/lifecycle/tests.rs,adl/src/cli/tooling_cmd/public_prompt_packet.rs,.github/workflows/ci.yaml,adl/tools/check_coverage_impact.sh,adl/tools/ci_path_policy.sh,docs/tooling/merge_readiness_gate_policy_v0.91.4.md,docs/milestones/v0.91.4/review/merge_readiness/ct_demo_001_merge_gate_profile_report.md",
    )
    .expect("focused plan");
    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
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
    assert!(cargo_calls.contains("--bin adl-csdlc public_prompt_packet"));
    assert!(!cargo_calls.contains("clippy --manifest-path"));

    let focused_calls = fs::read_to_string(&focused_log).expect("focused log");
    assert!(focused_calls.contains("coverage"));
    assert!(focused_calls.contains("path-policy"));
}

#[test]
fn finish_helper_paths_run_narrow_finish_focused_validation() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-narrow-focused-validation");
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
    write_executable(
        &bin_dir.join("cargo"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nexit 0\n",
            cargo_log.display()
        ),
    );
    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let plan = select_finish_validation_plan(
        "adl/src/cli/pr_cmd/finish_support.rs,adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs,docs/default_workflow.md",
    )
    .expect("narrow finish-focused plan");
    assert_eq!(plan.mode, FinishValidationMode::SmallBinaryFocused);
    run_finish_validation_rust(&repo, &plan).expect("narrow focused validation");

    unsafe {
        env::set_var("PATH", old_path);
    }

    let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
    assert!(cargo_calls.contains("fmt --manifest-path"));
    assert!(cargo_calls
        .contains("--bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation"));
    assert!(cargo_calls.contains(
        "--bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_helper_paths_run_focused_local_ci_gated_validation"
    ));
    assert!(!cargo_calls.contains(" cli::pr_cmd\n"));
    assert!(!cargo_calls.contains("clippy --manifest-path"));
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

    fs::write(repo.join("unstaged.rs"), "pub fn unrelated() {}\n").expect("write unrelated");
    assert_eq!(
        finish_changed_paths(&repo, true).expect("staged paths with unrelated unstaged edit"),
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

#[test]
fn real_pr_finish_happy_path_is_covered_in_default_lane() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-default-lane");
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
        .args([
            "checkout",
            "-q",
            "-b",
            "codex/1153-rust-finish-default-lane"
        ])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let issue_ref = IssueRef::new(
        1153,
        "v0.86".to_string(),
        "rust-finish-default-lane".to_string(),
    )
    .expect("issue ref");
    fs::create_dir_all(issue_ref.task_bundle_dir_path(&repo)).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    let plan = issue_ref.task_bundle_plan_path(&repo);
    let review_policy = issue_ref.task_bundle_review_policy_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust finish default lane");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Rust finish default lane",
        "codex/1153-rust-finish-default-lane",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_spp(
        &plan,
        &issue_ref,
        "[v0.86][tools] Rust finish default lane",
        "codex/1153-rust-finish-default-lane",
        &repo,
    );
    write_authored_srp(
        &review_policy,
        &issue_ref,
        "[v0.86][tools] Rust finish default lane",
        "codex/1153-rust-finish-default-lane",
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1153-rust-finish-default-lane");
    fs::write(
        repo.join("adl/src/lib.rs"),
        "pub fn placeholder() {}\npub fn changed() {}\n",
    )
    .expect("write change");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let janitor_log = temp.join("janitor.log");
    let closeout_log = temp.join("closeout.log");
    let gh_path = bin_dir.join("gh");
    let janitor_path = bin_dir.join("janitor");
    let closeout_path = bin_dir.join("closeout");
    write_executable(
        &gh_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr create' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1159\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1153\\n'\n  else\n    printf 'Closes #1153\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
            gh_log.display()
        ),
    );
    write_executable(
        &janitor_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\n",
            janitor_log.display()
        ),
    );
    write_executable(
        &closeout_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\n",
            closeout_log.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let old_janitor_cmd = env::var("ADL_PR_JANITOR_CMD").ok();
    let old_janitor_disable = env::var("ADL_PR_JANITOR_DISABLE").ok();
    let old_closeout_cmd = env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    let old_closeout_disable = env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("ADL_PR_JANITOR_DISABLE", "0");
        env::set_var("ADL_PR_JANITOR_CMD", &janitor_path);
        env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
        env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", &closeout_path);
    }
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr_finish(&[
        "1153".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Rust finish default lane".to_string(),
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
        if let Some(value) = old_janitor_cmd {
            env::set_var("ADL_PR_JANITOR_CMD", value);
        } else {
            env::remove_var("ADL_PR_JANITOR_CMD");
        }
        if let Some(value) = old_janitor_disable {
            env::set_var("ADL_PR_JANITOR_DISABLE", value);
        } else {
            env::remove_var("ADL_PR_JANITOR_DISABLE");
        }
        if let Some(value) = old_closeout_cmd {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
        }
        if let Some(value) = old_closeout_disable {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_DISABLE");
        }
    }

    result.expect("real_pr_finish success");

    let output_text = fs::read_to_string(&output).expect("read output card");
    assert!(
        !output_text.contains("bash adl/tools/check_no_tracked_adl_issue_record_residue.sh"),
        "--no-checks finish should not inject unrun validation commands into SOR"
    );
    assert!(
        !output_text.contains("git diff --check"),
        "--no-checks finish should not inject docs-only validation evidence into SOR"
    );

    let head_subject = run_capture(
        "git",
        &[
            "-C",
            path_str(&repo).expect("repo"),
            "log",
            "-1",
            "--format=%s",
        ],
    )
    .expect("head subject");
    assert!(head_subject.contains("[v0.86][tools] Rust finish default lane (Closes #1153)"));
    let gh_log = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_log.contains("pr create"));
    let janitor_log = fs::read_to_string(&janitor_log).expect("janitor log");
    let closeout_log = fs::read_to_string(&closeout_log).expect("closeout log");
    assert!(janitor_log.contains("--issue 1153"));
    assert!(closeout_log.contains("--issue 1153"));
}
