use super::*;
use crate::cli::github_token::{GithubTokenSource, ResolvedGithubToken};
use crate::cli::pr_cmd::finish_support::{
    ensure_finish_branch_not_behind_origin_main, ensure_finish_task_bundle_surfaces,
    ensure_finish_validation_profile_is_runnable, ensure_no_staged_issue_bundle_mutations,
    extra_pr_body_looks_like_issue_template, extract_markdown_section,
    finish_declared_paths_for_validation, finish_inputs_fingerprint,
    issue_bundle_issue_number_from_repo_relative, load_finish_validation_profile,
    load_finish_validation_profile_for_execution, non_closing_lifecycle_line,
    normalize_docs_only_sor_text, normalize_sor_emitted_facts_fixture, open_pr_url_nonblocking,
    open_pr_url_nonblocking_with_timeout, push_finish_branch_with_git, real_pr_finish,
    reject_local_issue_bundle_paths_in_finish_paths, render_default_finish_validation,
    resolve_finish_issue_scope_and_slug, restage_finish_output_truth_paths,
    run_finish_validation_status, select_finish_validation_plan_for_finish, FinishValidationMode,
    FinishValidationPlan, FinishValidationProfile, FinishValidationProfileEscalation,
    FinishValidationProfileEscalationReason, FinishValidationProfileRunItem,
    FinishValidationProfileSurfaceItem, FinishValidationVppRecord, SorFactEmissionContext,
};
use crate::cli::pr_cmd::git_support::commits_behind_origin_main;
use std::os::unix::fs::PermissionsExt;

#[test]
fn finish_declared_paths_for_validation_splits_operator_surface() {
    assert_eq!(
        finish_declared_paths_for_validation("docs, adl/src , ,README.md"),
        vec!["docs", "adl/src", "README.md"]
    );
}

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

    let merge_parsed = parse_finish_args(&[
        "1153".to_string(),
        "--title".to_string(),
        "Example".to_string(),
        "--merge".to_string(),
    ])
    .expect("parse finish merge");
    assert!(merge_parsed.merge_mode);
    assert!(
        merge_parsed.ready,
        "--merge should imply ready so native finish merge does not stall on draft-only state"
    );

    let auto_merge_parsed = parse_finish_args(&[
        "1153".to_string(),
        "--title".to_string(),
        "Example".to_string(),
        "--auto-merge".to_string(),
    ])
    .expect("parse finish auto-merge");
    assert!(auto_merge_parsed.merge_mode);
    assert!(
        auto_merge_parsed.ready,
        "--auto-merge should imply ready so native finish merge does not stall on draft-only state"
    );
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
fn finish_push_uses_token_askpass_without_leaking_token_in_argv() {
    let temp = unique_temp_dir("adl-pr-finish-token-aware-push");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    let log = temp.join("git-push.log");
    let git = temp.join("git");
    write_executable(
        &git,
        &format!(
            r#"#!/usr/bin/env bash
set -euo pipefail
log='{log}'
printf 'argv:%s\n' "$*" >"$log"
printf 'prompt:%s\n' "${{GIT_TERMINAL_PROMPT:-}}" >>"$log"
printf 'askpass:%s\n' "${{GIT_ASKPASS:-}}" >>"$log"
printf 'token_file:%s\n' "${{ADL_GIT_ASKPASS_TOKEN_FILE:-}}" >>"$log"
printf 'github_token_env:%s\n' "${{GITHUB_TOKEN:-}}" >>"$log"
printf 'gh_token_env:%s\n' "${{GH_TOKEN:-}}" >>"$log"
printf 'token_file_env:%s\n' "${{ADL_GITHUB_TOKEN_FILE:-}}" >>"$log"
if [[ -n "${{GIT_ASKPASS:-}}" ]]; then
  printf 'user:%s\n' "$("$GIT_ASKPASS" 'Username for https://github.com')" >>"$log"
  printf 'pass:%s\n' "$("$GIT_ASKPASS" 'Password for https://x-access-token@github.com')" >>"$log"
fi
"#,
            log = path_str(&log).expect("log path")
        ),
    );
    let token = ResolvedGithubToken::new("test-token-4598", GithubTokenSource::GithubToken)
        .expect("fake token");

    push_finish_branch_with_git(
        path_str(&git).expect("git path"),
        &repo,
        "codex/4598-example",
        Some(&token),
    )
    .expect("push succeeds");

    let log_text = fs::read_to_string(&log).expect("read push log");
    assert!(log_text.contains("argv:-C "));
    assert!(log_text.contains(" push origin codex/4598-example"));
    assert!(log_text.contains("prompt:0"));
    assert!(log_text.contains("askpass:"));
    assert!(log_text.contains("token_file:"));
    assert!(log_text.contains("github_token_env:\n"));
    assert!(log_text.contains("gh_token_env:\n"));
    assert!(log_text.contains("token_file_env:\n"));
    assert!(log_text.contains("user:x-access-token"));
    assert!(log_text.contains("pass:test-token-4598"));
    let token_file_line = log_text
        .lines()
        .find_map(|line| line.strip_prefix("token_file:"))
        .expect("token file line");
    assert!(
        !Path::new(token_file_line).exists(),
        "temporary git askpass token file must be removed after push"
    );
    let argv_line = log_text
        .lines()
        .find(|line| line.starts_with("argv:"))
        .expect("argv line");
    assert!(
        !argv_line.contains("test-token-4598"),
        "token must not be passed on the git command line"
    );
}

fn copy_finish_bootstrap_support_files(repo: &Path) {
    copy_bootstrap_support_files(repo);
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf();
    let tools_dir = repo.join("adl/tools");
    fs::copy(
        workspace_root.join("adl/tools/owner_binary_resolution.sh"),
        tools_dir.join("owner_binary_resolution.sh"),
    )
    .expect("copy owner binary resolution helper");
    #[cfg(unix)]
    {
        let helper = tools_dir.join("owner_binary_resolution.sh");
        let mut perms = fs::metadata(&helper).expect("metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&helper, perms).expect("chmod owner binary helper");
    }
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
        Some(&render_default_finish_validation(
            &FinishValidationPlan {
                mode: FinishValidationMode::LargerBinaryFocused,
                commands: vec![
                    "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
                    "cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string(),
                    "cargo test --manifest-path adl/Cargo.toml --bin adl cli::pr_cmd".to_string(),
                ],
            },
            None,
        )),
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
        Some(&render_default_finish_validation(
            &FinishValidationPlan {
                mode: FinishValidationMode::LargerBinaryFocused,
                commands: vec![
                    "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
                    "cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string(),
                    "cargo test --manifest-path adl/Cargo.toml --bin adl cli::pr_cmd".to_string(),
                ],
            },
            None,
        )),
        "fp-123",
        &temp,
    )
    .expect_err("issue template text should be rejected");
    assert!(err.to_string().contains("issue-template/prompt text"));
}

#[test]
fn render_pr_body_can_declare_non_closing_lifecycle_pr() {
    let temp = unique_temp_dir("adl-pr-render-body-no-close");
    fs::create_dir_all(&temp).expect("temp dir");
    let input = temp.join("input.md");
    let output = temp.join("output.md");
    fs::write(&input, "# input\n").expect("write input");
    fs::write(
        &output,
        "# no-close\n\n## Summary\nsummary text\n\n## Artifacts produced\n- docs/example.md\n",
    )
    .expect("write output");

    let body = render_pr_body(
        Some(&non_closing_lifecycle_line(1153)),
        &input,
        &output,
        None,
        None,
        "fp-123",
        &temp,
    )
    .expect("render no-close body");

    assert!(body.contains("Non-closing lifecycle PR"));
    assert!(body.contains("issue #1153 remains open"));
    assert!(!body.contains("Closes #1153"));
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
        Some(&render_default_finish_validation(
            &FinishValidationPlan {
                mode: FinishValidationMode::DocsOnly,
                commands: vec![
                    "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
                    "git diff --check".to_string(),
                ],
            },
            None,
        )),
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

## PVF Lane Truth
- Initial PVF lane: `docs_only`
- Planned PVF lane: `docs_only`
- Final PVF lane: `docs_only`
- Lane change reason: `no_lane_change`

## Issue Metrics Truth
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

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
fn sor_emitted_facts_merge_review_validation_and_pr_publication_truth() {
    let output = r#"## Verification Summary

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
    let review = r#"---
review_results:
  findings_status: "findings_present"
  recommended_outcome: "needs_followup"
---

# Structured Review Prompt

## Findings

- Missing focused regression test for fact merge.

## Dispositions

- Added focused SOR fact merge test coverage.
"#;

    let normalized = normalize_sor_emitted_facts_fixture(
        output,
        &[
            "adl/src/cli/pr_cmd/finish_support.rs".to_string(),
            "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs".to_string(),
        ],
        &[
            "git diff --check".to_string(),
            "cargo test --manifest-path adl/Cargo.toml sor_emitted_facts_merge_review_validation_and_pr_publication_truth".to_string(),
        ],
        review,
        SorFactEmissionContext {
            validation_status: "PASS",
            pr_url: Some("https://github.com/danielbaustin/agent-design-language/pull/9999"),
            integration_state: "pr_open",
            closing_linkage_repaired: true,
        },
    )
    .expect("normalize sor emitted facts");

    assert!(normalized.contains("sor_facts:"));
    assert!(normalized.contains("schema_version: adl.sor_facts.v1"));
    assert!(normalized.contains("findings_status: findings_present"));
    assert!(normalized.contains("recommended_outcome: needs_followup"));
    assert!(normalized.contains("Missing focused regression test for fact merge."));
    assert!(normalized.contains("Added focused SOR fact merge test coverage."));
    assert!(normalized
        .contains("pr_url: https://github.com/danielbaustin/agent-design-language/pull/9999"));
    assert!(normalized.contains("state: pr_open"));
    assert!(normalized.contains("repaired missing PR closing linkage"));
}

#[test]
fn sor_emitted_facts_merge_is_idempotent() {
    let output = r#"## Verification Summary

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
    let review = r#"---
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
---

# Structured Review Prompt

## Findings

- No material findings.

## Dispositions

- No fixes required.
"#;
    let changed_paths = vec!["docs/example.md".to_string()];
    let commands = vec!["git diff --check".to_string()];

    let first = normalize_sor_emitted_facts_fixture(
        output,
        &changed_paths,
        &commands,
        review,
        SorFactEmissionContext {
            validation_status: "PASS",
            pr_url: Some("https://example.test/pr/1"),
            integration_state: "pr_open",
            closing_linkage_repaired: false,
        },
    )
    .expect("first normalize");
    let second = normalize_sor_emitted_facts_fixture(
        &first,
        &changed_paths,
        &commands,
        review,
        SorFactEmissionContext {
            validation_status: "PASS",
            pr_url: Some("https://example.test/pr/1"),
            integration_state: "pr_open",
            closing_linkage_repaired: false,
        },
    )
    .expect("second normalize");

    assert_eq!(first, second);
}

#[test]
fn sor_emitted_facts_record_not_run_validation_truth_when_checks_are_skipped() {
    let output = r#"## Verification Summary

```yaml
verification_summary:
  validation:
    status: NOT_RUN
```
"#;
    let review = r#"---
review_results:
  findings_status: "not_run"
  recommended_outcome: "not_run"
---

# Structured Review Prompt
"#;

    let normalized = normalize_sor_emitted_facts_fixture(
        output,
        &["docs/example.md".to_string()],
        &["git diff --check".to_string()],
        review,
        SorFactEmissionContext {
            validation_status: "NOT_RUN",
            pr_url: None,
            integration_state: "worktree_only",
            closing_linkage_repaired: false,
        },
    )
    .expect("normalize no-checks sor emitted facts");

    assert!(normalized.contains("status: NOT_RUN"));
    assert!(normalized.contains("pr_url: null"));
}

#[test]
fn sor_emitted_facts_fallback_preserves_non_yaml_summary_and_appends_machine_readable_block() {
    let output = r#"## Verification Summary

plain-text verification summary that should not become a finish-time parse failure
"#;
    let review = r#"---
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
---

# Structured Review Prompt

## Findings

- No material findings.

## Dispositions

- No fixes required.
"#;

    let normalized = normalize_sor_emitted_facts_fixture(
        output,
        &["docs/example.md".to_string()],
        &["git diff --check".to_string()],
        review,
        SorFactEmissionContext {
            validation_status: "PASS",
            pr_url: Some("https://example.test/pr/2"),
            integration_state: "pr_open",
            closing_linkage_repaired: false,
        },
    )
    .expect("normalize with fallback");

    assert!(normalized.contains("plain-text verification summary"));
    assert!(normalized.contains("Machine-readable SOR facts:"));
    assert!(normalized.contains("sor_facts:"));
}

#[test]
fn sor_emitted_facts_creates_verification_summary_when_missing() {
    let output = r#"## Summary

Execution summary without a verification section yet.
"#;
    let review = r#"---
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
---

# Structured Review Prompt

## Findings

- No material findings.

## Dispositions

- No fixes required.
"#;

    let normalized = normalize_sor_emitted_facts_fixture(
        output,
        &["docs/example.md".to_string()],
        &["git diff --check".to_string()],
        review,
        SorFactEmissionContext {
            validation_status: "PASS",
            pr_url: Some("https://example.test/pr/2"),
            integration_state: "pr_open",
            closing_linkage_repaired: false,
        },
    )
    .expect("normalize with missing verification summary");

    assert!(normalized.contains("## Summary"));
    assert!(normalized.contains("## Verification Summary"));
    assert!(normalized.contains("Machine-readable SOR facts:"));
    assert!(normalized.contains("schema_version: adl.sor_facts.v1"));
}

#[test]
fn sor_emitted_facts_fallback_replaces_existing_machine_readable_block_instead_of_appending() {
    let output = r#"## Verification Summary

plain-text verification summary that should not become a finish-time parse failure
"#;
    let review = r#"---
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
---

# Structured Review Prompt

## Findings

- No material findings.

## Dispositions

- No fixes required.
"#;

    let first = normalize_sor_emitted_facts_fixture(
        output,
        &["docs/example.md".to_string()],
        &["git diff --check".to_string()],
        review,
        SorFactEmissionContext {
            validation_status: "PASS",
            pr_url: Some("https://example.test/pr/2"),
            integration_state: "worktree_only",
            closing_linkage_repaired: false,
        },
    )
    .expect("first fallback normalize");
    let second = normalize_sor_emitted_facts_fixture(
        &first,
        &["docs/example.md".to_string()],
        &["git diff --check".to_string()],
        review,
        SorFactEmissionContext {
            validation_status: "PASS",
            pr_url: Some("https://example.test/pr/2"),
            integration_state: "pr_open",
            closing_linkage_repaired: false,
        },
    )
    .expect("second fallback normalize");

    assert_eq!(second.matches("Machine-readable SOR facts:").count(), 1);
    assert!(second.contains("state: pr_open"));
    assert!(!second.contains("state: worktree_only\n\nMachine-readable SOR facts:"));
}

#[test]
fn sor_emitted_facts_fallback_is_idempotent_for_initially_empty_summary_body() {
    let output = "## Verification Summary\n";
    let review = r#"---
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
---

# Structured Review Prompt

## Findings

- No material findings.
"#;

    let first = normalize_sor_emitted_facts_fixture(
        output,
        &["docs/example.md".to_string()],
        &["git diff --check".to_string()],
        review,
        SorFactEmissionContext {
            validation_status: "PASS",
            pr_url: Some("https://example.test/pr/3"),
            integration_state: "worktree_only",
            closing_linkage_repaired: false,
        },
    )
    .expect("first empty-body fallback normalize");
    let second = normalize_sor_emitted_facts_fixture(
        &first,
        &["docs/example.md".to_string()],
        &["git diff --check".to_string()],
        review,
        SorFactEmissionContext {
            validation_status: "PASS",
            pr_url: Some("https://example.test/pr/3"),
            integration_state: "pr_open",
            closing_linkage_repaired: false,
        },
    )
    .expect("second empty-body fallback normalize");

    assert_eq!(second.matches("Machine-readable SOR facts:").count(), 1);
    assert!(second.contains("state: pr_open"));
}

#[test]
fn sor_emitted_facts_sanitize_manager_backed_changed_files_path() {
    let output = "## Verification Summary\n";
    let review = r#"---
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
---

# Structured Review Prompt

## Findings

- No material findings.
"#;

    let normalized = normalize_sor_emitted_facts_fixture(
        output,
        &["adl/src/cli/pr_cmd/github.rs".to_string()],
        &["bash adl/tools/run_pr_fast_test_lane.sh --changed-files /private/tmp/finish-validation-profile-123.txt".to_string()],
        review,
        SorFactEmissionContext {
            validation_status: "PASS",
            pr_url: None,
            integration_state: "worktree_only",
            closing_linkage_repaired: false,
        },
    )
    .expect("normalize with sanitized changed-files command");

    assert!(normalized
        .contains("bash adl/tools/run_pr_fast_test_lane.sh --changed-files <changed-files>"));
    assert!(!normalized.contains("/private/tmp/finish-validation-profile-123.txt"));
}

#[test]
fn sor_emitted_facts_default_review_truth_when_srp_front_matter_is_absent() {
    let output = r#"## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
```
"#;
    let review = r#"# Structured Review Prompt

## Findings

- Review packet not finalized yet.
"#;

    let normalized = normalize_sor_emitted_facts_fixture(
        output,
        &["docs/example.md".to_string()],
        &["git diff --check".to_string()],
        review,
        SorFactEmissionContext {
            validation_status: "PASS",
            pr_url: None,
            integration_state: "worktree_only",
            closing_linkage_repaired: false,
        },
    )
    .expect("normalize without srp front matter");

    assert!(normalized.contains("findings_status: not_run"));
    assert!(normalized.contains("recommended_outcome: not_run"));
    assert!(normalized.contains("Review packet not finalized yet."));
}

#[test]
fn sor_emitted_facts_parse_review_truth_from_crlf_front_matter() {
    let output = "## Verification Summary\n\n```yaml\nverification_summary:\n  validation:\n    status: PASS\n```\n";
    let review = "---\r\nreview_results:\r\n  findings_status: \"no_findings\"\r\n  recommended_outcome: \"pass\"\r\n---\r\n\r\n# Structured Review Prompt\r\n\r\n## Findings\r\n\r\n- No material findings.\r\n";

    let normalized = normalize_sor_emitted_facts_fixture(
        output,
        &["docs/example.md".to_string()],
        &["git diff --check".to_string()],
        review,
        SorFactEmissionContext {
            validation_status: "PASS",
            pr_url: None,
            integration_state: "worktree_only",
            closing_linkage_repaired: false,
        },
    )
    .expect("normalize crlf srp front matter");

    assert!(normalized.contains("findings_status: no_findings"));
    assert!(normalized.contains("recommended_outcome: pass"));
}

#[test]
fn sor_emitted_facts_capture_numbered_review_findings_and_dispositions() {
    let output = "## Verification Summary\n\n```yaml\nverification_summary:\n  validation:\n    status: PASS\n```\n";
    let review = r#"---
review_results:
  findings_status: "findings_present"
  recommended_outcome: "needs_followup"
---

# Structured Review Prompt

## Findings

1. Numbered finding survives into machine-readable SOR facts.
2) Alternate ordered-list syntax also survives.
   - nested evidence should not become a separate finding.

## Dispositions

1. Added focused regression coverage for numbered findings.
2) Deferred larger PR inventory work into a retained route packet.
   - nested disposition detail should not become a separate disposition.
"#;

    let normalized = normalize_sor_emitted_facts_fixture(
        output,
        &["adl/src/cli/pr_cmd/finish_support.rs".to_string()],
        &["git diff --check".to_string()],
        review,
        SorFactEmissionContext {
            validation_status: "PASS",
            pr_url: None,
            integration_state: "worktree_only",
            closing_linkage_repaired: false,
        },
    )
    .expect("normalize numbered finding review evidence");

    assert!(normalized.contains("findings_status: findings_present"));
    assert!(normalized.contains("recommended_outcome: needs_followup"));
    assert!(normalized.contains("Numbered finding survives into machine-readable SOR facts."));
    assert!(normalized.contains("Alternate ordered-list syntax also survives."));
    assert!(normalized.contains("Added focused regression coverage for numbered findings."));
    assert!(normalized.contains("Deferred larger PR inventory work into a retained route packet."));
    assert!(!normalized.contains("nested evidence should not become a separate finding."));
    assert!(
        !normalized.contains("nested disposition detail should not become a separate disposition.")
    );
}

#[test]
fn render_default_finish_validation_includes_profile_truth_and_sanitizes_changed_files() {
    let plan = FinishValidationPlan {
        mode: FinishValidationMode::SmallBinaryFocused,
        commands: vec![
            "git diff --check".to_string(),
            "cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish profile_test"
                .to_string(),
        ],
    };
    let profile = FinishValidationProfile {
        selected_profile: "selected_2_lane_profile".to_string(),
        status: "ready_to_run".to_string(),
        pr_publication_sufficient: true,
        run: vec![
            FinishValidationProfileRunItem {
                lane_id: "csdlc_owner_lane".to_string(),
                command: "bash adl/tools/run_owner_validation_lane.sh csdlc".to_string(),
                reason: "csdlc_owner_surface_requires_csdlc_owner_lane".to_string(),
                matched_paths: vec!["adl/src/cli/pr_cmd/doctor.rs".to_string()],
                vpp_record: Some(FinishValidationVppRecord {
                    contract_version: "vpp.lane.v1".to_string(),
                    artifacts: vec!["working_tree_diff_hygiene".to_string()],
                    expected_runtime_class: "tiny".to_string(),
                    parallel_group: "docs_hygiene".to_string(),
                    cache_equivalence_group: "git_diff_check".to_string(),
                    failure_semantics: "fail_closed".to_string(),
                }),
            },
            FinishValidationProfileRunItem {
                lane_id: "rust_pr_fast".to_string(),
                command: "bash adl/tools/run_pr_fast_test_lane.sh --changed-files /private/tmp/changed-files.txt".to_string(),
                reason: "bounded_rust_surface_runs_focused_nextest".to_string(),
                matched_paths: vec!["adl/src/cli/pr_cmd/doctor.rs".to_string()],
                vpp_record: None,
            },
        ],
        not_run: vec![FinishValidationProfileSurfaceItem {
            surface: "coverage_release_gate".to_string(),
            reason: "reserved for coverage or release policy selection".to_string(),
        }],
        deferred: vec![FinishValidationProfileSurfaceItem {
            surface: "ci_integration".to_string(),
            reason: "deferred to GitHub checks for merge-context validation".to_string(),
        }],
        escalation: FinishValidationProfileEscalation {
            required: false,
            reasons: vec![FinishValidationProfileEscalationReason {
                lane_id: "none".to_string(),
                status: "not_applicable".to_string(),
                reason: "not used".to_string(),
                matched_paths: vec![],
                manifest_rule: None,
                remediation_hint: None,
            }],
        },
    };

    let rendered = render_default_finish_validation(&plan, Some(&profile));

    assert!(rendered.contains("Selected validation profile: `selected_2_lane_profile`"));
    assert!(rendered.contains("Profile-selected run lanes:"));
    assert!(rendered
        .contains("`csdlc_owner_lane` via `bash adl/tools/run_owner_validation_lane.sh csdlc`"));
    assert!(rendered.contains(
        "vpp: contract=vpp.lane.v1 runtime_class=tiny parallel_group=docs_hygiene cache_equivalence_group=git_diff_check failure_semantics=fail_closed"
    ));
    assert!(rendered.contains("artifacts: working_tree_diff_hygiene"));
    assert!(rendered.contains("`rust_pr_fast` via `bash adl/tools/run_pr_fast_test_lane.sh --changed-files <changed-files>`"));
    assert!(rendered.contains("Profile-skipped proof surfaces:"));
    assert!(rendered
        .contains("`coverage_release_gate`: reserved for coverage or release policy selection"));
    assert!(rendered.contains("Deferred proof:"));
    assert!(rendered
        .contains("`ci_integration`: deferred to GitHub checks for merge-context validation"));
    assert!(rendered.contains("Escalation: not required"));
    assert!(!rendered.contains("/private/tmp/changed-files.txt"));
}

struct FinishHelperObservabilityEnvGuard;

impl FinishHelperObservabilityEnvGuard {
    fn install(log: &Path) -> Self {
        unsafe {
            std::env::set_var("ADL_OBSERVABILITY_STDERR", "0");
            std::env::set_var("ADL_OBSERVABILITY_LOG", log);
            std::env::set_var("ADL_OBSERVABILITY_HEARTBEAT_MS", "25");
        }
        Self
    }
}

impl Drop for FinishHelperObservabilityEnvGuard {
    fn drop(&mut self) {
        unsafe {
            std::env::remove_var("ADL_OBSERVABILITY_STDERR");
            std::env::remove_var("ADL_OBSERVABILITY_LOG");
            std::env::remove_var("ADL_OBSERVABILITY_HEARTBEAT_MS");
        }
    }
}

fn read_finish_helper_log_until(log: &Path, needle: &str) -> String {
    for _ in 0..10 {
        let contents = fs::read_to_string(log).unwrap_or_default();
        if contents.contains(needle) {
            return contents;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    fs::read_to_string(log).unwrap_or_default()
}

#[test]
fn finish_helper_paths_emit_subprocess_heartbeat_and_classification() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-finish-helper-heartbeat");
    let script = temp.join("slow-validation.sh");
    let log = temp.join("observability.log");
    fs::write(&script, "#!/bin/sh\nsleep 0.08\nexit 0\n").expect("write script");
    let mut perms = fs::metadata(&script).expect("metadata").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script, perms).expect("chmod");

    let _env = FinishHelperObservabilityEnvGuard::install(&log);

    run_finish_validation_status("bash", &[script.to_str().expect("script path")])
        .expect("validation command");

    let contents = read_finish_helper_log_until(&log, "result=completed");
    assert!(contents.contains("command=finish"));
    assert!(contents.contains("stage=validation_subprocess"));
    assert!(contents.contains("program=bash"));
    assert!(contents.contains("subprocess_class=shell_validation"));
    assert!(contents.contains("result=heartbeat"));
    assert!(contents.contains("result=completed"));
}

#[test]
fn finish_helper_paths_emit_failed_terminal_event_on_spawn_error() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-finish-helper-spawn-error");
    let log = temp.join("observability.log");

    let _env = FinishHelperObservabilityEnvGuard::install(&log);

    let err =
        run_finish_validation_status("definitely-not-a-real-finish-subprocess", &["--version"])
            .expect_err("spawn should fail");
    assert!(err.to_string().contains("failed to spawn"));

    let contents = read_finish_helper_log_until(&log, "result=failed");
    assert!(contents.contains("result=started"));
    assert!(contents.contains("result=failed"));
    assert!(contents.contains("reason_code=validation_subprocess_spawn_failed"));
    assert!(contents.contains("next_action_hint=check_subprocess_path_and_permissions"));
}

fn init_finish_helper_git_repo(repo: &Path) {
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(repo)
        .status()
        .expect("git init")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(repo)
        .status()
        .expect("git config name")
        .success());
    assert!(Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo)
        .status()
        .expect("git config email")
        .success());
}

#[test]
fn finish_helper_paths_restage_output_truth_skips_local_cards_root_but_stages_tracked_files() {
    let repo = unique_temp_dir("adl-finish-helper-restage-cards-root");
    let issue_ref = IssueRef::new(4262, "v0.91.6".to_string(), "finish-cards-root".to_string())
        .expect("issue ref");
    let tracked = repo.join("README.md");
    let cards_root = resolve_cards_root(&repo, None);
    let output_card = ::adl::control_plane::card_output_path(&cards_root, issue_ref.issue_number());

    fs::create_dir_all(output_card.parent().expect("output parent")).expect("cards root");
    fs::create_dir_all(issue_ref.task_bundle_dir_path(&repo)).expect("task bundle dir");
    fs::write(repo.join(".gitignore"), ".adl/\n").expect("gitignore");
    fs::write(&tracked, "changed\n").expect("tracked file");
    fs::write(&output_card, "local output truth\n").expect("output card");

    init_finish_helper_git_repo(&repo);
    assert!(Command::new("git")
        .args(["add", ".gitignore", "README.md"])
        .current_dir(&repo)
        .status()
        .expect("git add initial")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());

    fs::write(&tracked, "changed twice\n").expect("update tracked file");

    restage_finish_output_truth_paths(&repo, &repo, &issue_ref, &[tracked.clone(), output_card])
        .expect("restage should skip ignored local cards root");

    let staged = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .current_dir(&repo)
        .output()
        .expect("git diff --cached");
    assert!(staged.status.success(), "git diff --cached should succeed");
    let staged_text = String::from_utf8_lossy(&staged.stdout);
    assert!(staged_text.contains("README.md"));
    assert!(!staged_text.contains(".adl/cards/4262/output_4262.md"));
}

#[test]
fn finish_helper_paths_restage_output_truth_rejects_tracked_cards_root_paths() {
    let repo = unique_temp_dir("adl-finish-helper-restage-tracked-cards-root");
    let issue_ref = IssueRef::new(
        4263,
        "v0.91.6".to_string(),
        "tracked-cards-root".to_string(),
    )
    .expect("issue ref");
    let cards_root = resolve_cards_root(&repo, None);
    let output_card = ::adl::control_plane::card_output_path(&cards_root, issue_ref.issue_number());

    fs::create_dir_all(output_card.parent().expect("output parent")).expect("cards root");
    fs::create_dir_all(issue_ref.task_bundle_dir_path(&repo)).expect("task bundle dir");
    fs::write(repo.join(".gitignore"), ".adl/\n").expect("gitignore");
    fs::write(&output_card, "tracked output truth\n").expect("output card");

    init_finish_helper_git_repo(&repo);
    assert!(Command::new("git")
        .args(["add", ".gitignore"])
        .current_dir(&repo)
        .status()
        .expect("git add gitignore")
        .success());
    assert!(Command::new("git")
        .args(["add", "-f", ".adl/cards/4263/output_4263.md"])
        .current_dir(&repo)
        .status()
        .expect("git add forced output")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());

    fs::write(&output_card, "tracked output truth updated\n").expect("update output card");

    let err = restage_finish_output_truth_paths(
        &repo,
        &repo,
        &issue_ref,
        std::slice::from_ref(&output_card),
    )
    .expect_err("tracked cards-root path should fail closed");
    assert!(err.to_string().contains("compatibility cards path"));
    assert!(err.to_string().contains(".adl/cards/4263/output_4263.md"));
}

#[test]
fn finish_helper_paths_restage_output_truth_rejects_tracked_primary_cards_root_paths() {
    let primary = unique_temp_dir("adl-finish-helper-restage-primary-cards-root-primary");
    let worktree = unique_temp_dir("adl-finish-helper-restage-primary-cards-root-worktree");
    let issue_ref = IssueRef::new(
        4264,
        "v0.91.6".to_string(),
        "primary-tracked-cards-root".to_string(),
    )
    .expect("issue ref");
    let primary_cards_root = resolve_cards_root(&primary, None);
    let primary_output =
        ::adl::control_plane::card_output_path(&primary_cards_root, issue_ref.issue_number());

    fs::create_dir_all(primary_output.parent().expect("output parent")).expect("cards root");
    fs::write(primary.join(".gitignore"), ".adl/\n").expect("gitignore");
    fs::write(&primary_output, "tracked output truth\n").expect("output card");

    init_finish_helper_git_repo(&primary);
    assert!(Command::new("git")
        .args(["add", ".gitignore"])
        .current_dir(&primary)
        .status()
        .expect("git add gitignore")
        .success());
    assert!(Command::new("git")
        .args(["add", "-f", ".adl/cards/4264/output_4264.md"])
        .current_dir(&primary)
        .status()
        .expect("git add forced output")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&primary)
        .status()
        .expect("git commit")
        .success());

    fs::create_dir_all(worktree.join(".git")).expect("worktree git dir placeholder");
    fs::write(&primary_output, "tracked output truth updated\n").expect("update output card");

    let err = restage_finish_output_truth_paths(
        &worktree,
        &primary,
        &issue_ref,
        std::slice::from_ref(&primary_output),
    )
    .expect_err("tracked primary cards-root path should fail closed");
    assert!(err.to_string().contains("compatibility cards path"));
    assert!(err.to_string().contains(".adl/cards/4264/output_4264.md"));
}

#[test]
fn finish_helper_paths_reject_local_issue_bundle_cards_in_finish_paths() {
    let repo = unique_temp_dir("adl-finish-helper-reject-local-issue-bundle-paths");
    let issue_ref = IssueRef::new(
        4265,
        "v0.91.6".to_string(),
        "reject-local-paths".to_string(),
    )
    .expect("issue ref");
    let local_sip = issue_ref.task_bundle_input_path(&repo);
    let local_sor = issue_ref.task_bundle_output_path(&repo);
    fs::create_dir_all(local_sip.parent().expect("sip parent")).expect("bundle dir");
    fs::write(&local_sip, "sip\n").expect("write sip");
    fs::write(&local_sor, "sor\n").expect("write sor");

    let err = reject_local_issue_bundle_paths_in_finish_paths(
        &repo,
        &[
            "docs/notes.md",
            local_sip.to_str().expect("sip path"),
            local_sor.to_str().expect("sor path"),
        ],
    )
    .expect_err("local issue bundle paths should fail closed");

    assert!(err
        .to_string()
        .contains("local-only .adl task-bundle card paths"));
    assert!(err
        .to_string()
        .contains(".adl/v0.91.6/tasks/issue-4265__reject-local-paths/sip.md"));
    assert!(err
        .to_string()
        .contains(".adl/v0.91.6/tasks/issue-4265__reject-local-paths/sor.md"));
}

#[test]
fn finish_helper_paths_reject_foreign_staged_issue_bundle_mutations() {
    let repo = unique_temp_dir("adl-finish-helper-reject-foreign-issue-bundle-stage");
    let active_issue =
        IssueRef::new(4266, "v0.91.6".to_string(), "active".to_string()).expect("active issue");
    let foreign_issue =
        IssueRef::new(4267, "v0.91.6".to_string(), "foreign".to_string()).expect("foreign issue");
    let foreign_sor = foreign_issue.task_bundle_output_path(&repo);

    fs::create_dir_all(foreign_sor.parent().expect("output parent")).expect("bundle dir");
    fs::write(repo.join(".gitignore"), ".adl/\n").expect("gitignore");
    fs::write(&foreign_sor, "foreign sor\n").expect("foreign sor");

    init_finish_helper_git_repo(&repo);
    assert!(Command::new("git")
        .args(["add", ".gitignore"])
        .current_dir(&repo)
        .status()
        .expect("git add gitignore")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["add", "-f", foreign_sor.to_str().expect("foreign sor path")])
        .current_dir(&repo)
        .status()
        .expect("git add foreign sor")
        .success());

    let err = ensure_no_staged_issue_bundle_mutations(&repo, &active_issue)
        .expect_err("foreign staged bundle paths should fail");
    assert!(err
        .to_string()
        .contains("staged .adl task-bundle changes for non-active issues detected"));
    assert!(err
        .to_string()
        .contains(".adl/v0.91.6/tasks/issue-4267__foreign/sor.md"));
}

#[test]
fn finish_helper_paths_cover_markdown_and_fingerprint_surfaces() {
    let repo = unique_temp_dir("adl-finish-helper-surfaces");
    let markdown = repo.join("output.md");
    fs::write(
        &markdown,
        "## Summary\nsummary\n\n## Validation\n- ok\n\n## Tail\nignored\n",
    )
    .expect("write markdown");

    assert_eq!(
        extract_markdown_section(&markdown, "Summary").expect("summary section"),
        "summary"
    );
    assert_eq!(
        extract_markdown_section(&markdown, "Validation").expect("validation section"),
        "- ok"
    );
    assert_eq!(
        issue_bundle_issue_number_from_repo_relative(
            ".adl/v0.91.6/tasks/issue-4268__helper/sor.md"
        ),
        Some(4268)
    );
    assert_eq!(
        issue_bundle_issue_number_from_repo_relative("docs/milestones/v0.91.6/README.md"),
        None
    );
    assert!(extra_pr_body_looks_like_issue_template(
        "issue_card_schema: adl.issue.v1"
    ));
    assert!(extra_pr_body_looks_like_issue_template(
        "## Goal\nstuff\n---\nmore"
    ));
    assert!(!extra_pr_body_looks_like_issue_template(
        "regular reviewer notes"
    ));
    assert_eq!(
        finish_inputs_fingerprint(
            "[v0.91.6][tools] Example",
            "adl/src/lib.rs,docs/notes.md",
            ".adl/v0.91.6/tasks/issue-4268__helper/sip.md",
            ".adl/v0.91.6/tasks/issue-4268__helper/sor.md",
        ),
        "v0-91-6-tools-example-adl-src-lib-rs-docs-notes-md-adl-v0-91-6-tasks-issue-4268-helper-sip-md-adl-v0-91-6-tasks-issue-4268-helper-sor-md"
    );
}

#[test]
fn load_finish_validation_profile_reads_manager_json_from_repo_tooling() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-finish-load-profile");
    fs::create_dir_all(repo.join("adl/tools")).expect("tools dir");
    write_executable(
        &repo.join("adl/tools/validation_manager.py"),
        r#"#!/usr/bin/env python3
import json

print(json.dumps({
    "selected_profile": "runtime_owner_profile",
    "status": "ready_to_run",
    "pr_publication_sufficient": False,
    "run": [
        {
            "lane_id": "runtime_owner_lane",
            "command": "bash adl/tools/run_owner_validation_lane.sh runtime --build",
            "reason": "runtime_owner_surface_requires_runtime_lane",
        }
    ],
    "not_run": [
        {
            "surface": "release_gate",
            "reason": "reserved for release readiness",
        }
    ],
    "deferred": [
        {
            "surface": "ci_integration",
            "reason": "deferred to GitHub checks",
        }
    ],
    "behavior_surfaces": ["runtime"],
    "validation_dag": [],
    "estimated_cost": "medium",
    "escalation": {
        "required": True,
        "reasons": [
            {
                "lane_id": "runtime_owner_lane",
                "status": "required",
                "reason": "runtime owner proof required",
            }
        ],
    },
    "selector_plan": [],
}))
"#,
    );

    let profile = load_finish_validation_profile(
        &repo,
        &["adl/src/bin/run_v0916_integrated_runtime_soak.rs".to_string()],
    )
    .expect("load finish validation profile");

    assert_eq!(profile.selected_profile, "runtime_owner_profile");
    assert_eq!(profile.status, "ready_to_run");
    assert!(!profile.pr_publication_sufficient);
    assert_eq!(profile.run.len(), 1);
    assert_eq!(profile.run[0].lane_id, "runtime_owner_lane");
    assert_eq!(
        profile.run[0].reason,
        "runtime_owner_surface_requires_runtime_lane"
    );
    assert_eq!(profile.not_run.len(), 1);
    assert_eq!(profile.deferred.len(), 1);
    assert!(profile.escalation.required);
    assert_eq!(profile.escalation.reasons.len(), 1);
}

#[test]
fn manager_backed_profile_retains_changed_file_for_pr_fast_execution() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-finish-manager-backed-pr-fast-profile");
    fs::create_dir_all(repo.join("adl/tools")).expect("tools dir");
    fs::write(
        repo.join("adl/Cargo.toml"),
        "[package]\nname='adl'\nversion='0.1.0'\n",
    )
    .expect("cargo toml");
    fs::create_dir_all(repo.join("adl/src")).expect("src dir");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn fixture() {}\n").expect("lib rs");
    write_executable(
        &repo.join("adl/tools/check_no_tracked_adl_issue_record_residue.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 0\n",
    );
    write_executable(
        &repo.join("adl/tools/run_owner_validation_lane.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf 'owner:%s\\n' \"$1\" >> \"$FOCUSED_LOG\"\n",
    );
    write_executable(
        &repo.join("adl/tools/run_pr_fast_test_lane.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\n[ -f \"$2\" ]\nprintf 'pr-fast:%s\\n' \"$2\" >> \"$FOCUSED_LOG\"\n",
    );
    write_executable(
        &repo.join("adl/tools/validation_manager.py"),
        r#"#!/usr/bin/env python3
import json
import sys

changed_files = sys.argv[2]
print(json.dumps({
    "selected_profile": "selected_2_lane_profile",
    "status": "ready_to_run",
    "pr_publication_sufficient": True,
    "run": [
        {
            "lane_id": "csdlc_owner_lane",
            "command": "bash adl/tools/run_owner_validation_lane.sh csdlc",
            "reason": "csdlc_owner_surface_requires_csdlc_owner_lane",
        },
        {
            "lane_id": "rust_pr_fast",
            "command": f"bash adl/tools/run_pr_fast_test_lane.sh --changed-files {changed_files}",
            "reason": "bounded_rust_surface_runs_focused_nextest",
        },
    ],
    "not_run": [],
    "deferred": [],
    "escalation": {"required": False, "reasons": []},
}))
"#,
    );
    init_git_repo(&repo);

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let focused_log = repo.join("focused.log");
    let old_path = env::var("PATH").unwrap_or_default();
    let old_focused_log = env::var("FOCUSED_LOG").ok();
    let old_cwd = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("FOCUSED_LOG", &focused_log);
        env::set_current_dir(&repo).expect("set cwd");
    }

    let plan = select_finish_validation_plan_for_finish(
        4421,
        ".",
        &["adl/src/cli/pr_cmd/doctor.rs".to_string()],
    )
    .expect("manager-backed finish plan");
    let retained_changed_file = plan
        .commands
        .iter()
        .find_map(|command| {
            command
                .strip_prefix("bash adl/tools/run_pr_fast_test_lane.sh --changed-files ")
                .map(PathBuf::from)
        })
        .expect("retained changed-file manifest path");
    assert!(plan
        .commands
        .iter()
        .any(|command| command.contains("run_pr_fast_test_lane.sh --changed-files")));
    run_finish_validation_rust(&repo, &plan).expect("manager-backed profile execution");

    unsafe {
        env::set_var("PATH", old_path);
        env::set_current_dir(old_cwd).expect("restore cwd");
    }
    match old_focused_log {
        Some(value) => unsafe { env::set_var("FOCUSED_LOG", value) },
        None => unsafe { env::remove_var("FOCUSED_LOG") },
    }

    let focused_calls = fs::read_to_string(&focused_log).expect("focused log");
    assert!(focused_calls.contains("owner:csdlc"));
    assert!(focused_calls.contains("pr-fast:"));
    assert!(
        !retained_changed_file.exists(),
        "manager-backed execution should clean up the retained changed-file manifest after running"
    );
}

#[test]
fn load_finish_validation_profile_cleans_tempfile_on_invalid_manager_json() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-finish-load-profile-invalid-json");
    let tmpdir = repo.join("tmp");
    fs::create_dir_all(repo.join("adl/tools")).expect("tools dir");
    fs::create_dir_all(&tmpdir).expect("tmp dir");
    write_executable(
        &repo.join("adl/tools/validation_manager.py"),
        "#!/usr/bin/env python3\nprint('{invalid json')\n",
    );

    let old_tmpdir = env::var("TMPDIR").ok();
    unsafe {
        env::set_var("TMPDIR", &tmpdir);
    }

    let err = load_finish_validation_profile(&repo, &["adl/src/cli/pr_cmd/doctor.rs".to_string()])
        .expect_err("invalid manager json should fail");

    match old_tmpdir {
        Some(value) => unsafe { env::set_var("TMPDIR", value) },
        None => unsafe { env::remove_var("TMPDIR") },
    }

    assert!(err
        .to_string()
        .contains("validation manager returned invalid profile JSON"));
    let retained_manifests = fs::read_dir(&tmpdir)
        .expect("read temp dir")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with("finish-validation-profile-")
        })
        .count();
    assert_eq!(
        retained_manifests, 0,
        "temporary changed-file manifests should be cleaned up on parse failure"
    );
}

#[test]
fn load_finish_validation_profile_rejects_retained_changed_file_substitution() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-finish-load-profile-substituted-retained-file");
    let tmpdir = repo.join("tmp");
    let wrong_dir = repo.join("wrong");
    fs::create_dir_all(repo.join("adl/tools")).expect("tools dir");
    fs::create_dir_all(&tmpdir).expect("tmp dir");
    fs::create_dir_all(&wrong_dir).expect("wrong dir");
    let substituted = wrong_dir.join("finish-validation-profile-substituted.txt");
    write_executable(
        &repo.join("adl/tools/validation_manager.py"),
        &format!(
            r#"#!/usr/bin/env python3
import json

print(json.dumps({{
    "selected_profile": "csdlc_owner_profile",
    "status": "ready_to_run",
    "pr_publication_sufficient": False,
    "run": [
        {{
            "lane_id": "pr_fast",
            "command": "bash adl/tools/run_pr_fast_test_lane.sh --changed-files {substituted}",
            "reason": "substituted changed-files path",
        }}
    ],
    "not_run": [],
    "deferred": [],
    "behavior_surfaces": ["csdlc"],
    "validation_dag": [],
    "estimated_cost": "small",
    "escalation": {{
        "required": False,
        "reasons": [],
    }},
    "selector_plan": [],
}}))
"#,
            substituted = substituted.display()
        ),
    );

    let old_tmpdir = env::var("TMPDIR").ok();
    unsafe {
        env::set_var("TMPDIR", &tmpdir);
    }

    let err = load_finish_validation_profile_for_execution(
        &repo,
        &["adl/src/cli/pr_cmd/doctor.rs".to_string()],
    )
    .expect_err("manager-backed profile must reject substituted retained changed-file path");

    match old_tmpdir {
        Some(value) => unsafe { env::set_var("TMPDIR", value) },
        None => unsafe { env::remove_var("TMPDIR") },
    }

    assert!(err
        .to_string()
        .contains("expected ADL-created retained manifest"));
    let retained_manifests = fs::read_dir(&tmpdir)
        .expect("read temp dir")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with("finish-validation-profile-")
        })
        .count();
    assert_eq!(
        retained_manifests, 0,
        "real retained tempfile should be cleaned when manager substitutes the changed-file path"
    );
}

#[test]
fn load_finish_validation_profile_cleans_tempfile_when_profile_only_needs_rendering() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-finish-load-profile-render-cleanup");
    let tmpdir = repo.join("tmp");
    fs::create_dir_all(repo.join("adl/tools")).expect("tools dir");
    fs::create_dir_all(&tmpdir).expect("tmp dir");
    write_executable(
        &repo.join("adl/tools/validation_manager.py"),
        r#"#!/usr/bin/env python3
import json
import sys

changed_files = sys.argv[2]
print(json.dumps({
    "selected_profile": "selected_1_lane_profile",
    "status": "ready_to_run",
    "pr_publication_sufficient": True,
    "run": [
        {
            "lane_id": "rust_pr_fast",
            "command": f"bash adl/tools/run_pr_fast_test_lane.sh --changed-files {changed_files}",
            "reason": "bounded_rust_surface_runs_focused_nextest",
        }
    ],
    "not_run": [],
    "deferred": [],
    "escalation": {"required": False, "reasons": []},
}))
"#,
    );

    let old_tmpdir = env::var("TMPDIR").ok();
    unsafe {
        env::set_var("TMPDIR", &tmpdir);
    }

    let profile =
        load_finish_validation_profile(&repo, &["adl/src/cli/pr_cmd/doctor.rs".to_string()])
            .expect("render-time manager profile");

    match old_tmpdir {
        Some(value) => unsafe { env::set_var("TMPDIR", value) },
        None => unsafe { env::remove_var("TMPDIR") },
    }

    assert_eq!(profile.run.len(), 1);
    assert!(profile.run[0]
        .command
        .contains("run_pr_fast_test_lane.sh --changed-files"));
    let retained_manifests = fs::read_dir(&tmpdir)
        .expect("read temp dir")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with("finish-validation-profile-")
        })
        .filter_map(|entry| fs::read_to_string(entry.path()).ok())
        .filter(|body| body.contains("adl/src/cli/pr_cmd/doctor.rs"))
        .count();
    assert_eq!(
        retained_manifests, 0,
        "render-time profile loads should not retain this test's changed-file manifest"
    );
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

    let err = stage_selected_paths_rust(
        &repo,
        "tracked.txt,.adl/v0.86/tasks/issue-1153__rust-finish-test/sor.md",
    )
    .expect_err("task-bundle SOR in --paths should fail before staging");
    let err_text = err.to_string();
    assert!(err_text.contains("--paths includes local-only .adl task-bundle card paths"));
    assert!(err_text.contains("issue-1153__rust-finish-test/sor.md"));
    assert!(err_text.contains("use --output-card for the SOR truth surface"));
    assert!(err_text.contains("tracked repo publication inputs"));

    let dot_relative_err = stage_selected_paths_rust(
        &repo,
        "tracked.txt,./.adl/v0.86/tasks/issue-1153__rust-finish-test/srp.md",
    )
    .expect_err("dot-relative task-bundle SRP in --paths should fail");
    assert!(dot_relative_err
        .to_string()
        .contains(".adl/v0.86/tasks/issue-1153__rust-finish-test/srp.md"));

    let absolute_sor = repo
        .join(".adl/v0.86/tasks/issue-1153__rust-finish-test/sor.md")
        .display()
        .to_string();
    let absolute_err = stage_selected_paths_rust(&repo, &format!("tracked.txt,{absolute_sor}"))
        .expect_err("absolute task-bundle SOR in --paths should fail");
    assert!(absolute_err
        .to_string()
        .contains(".adl/v0.86/tasks/issue-1153__rust-finish-test/sor.md"));
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
    write_executable(
        &repo.join("adl/tools/test_pr_small_binary_delegation.sh"),
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
    write_executable(
        &repo.join("adl/tools/test_pr_small_binary_delegation.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 0\n",
    );
    init_git_repo(&repo);

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let cargo_log = temp.join("cargo-env.log");
    write_executable(
        &bin_dir.join("cargo"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf 'args=%s ADL_GITHUB_CLIENT=%s ADL_GITHUB_DISABLE_GH_FALLBACK=%s ADL_GITHUB_OCTOCRAB_BASE_URI=%s GITHUB_TOKEN=%s GH_TOKEN=%s ADL_GITHUB_TOKEN_FILE=%s ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE=%s ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT=%s\\n' \"$*\" \"${{ADL_GITHUB_CLIENT-}}\" \"${{ADL_GITHUB_DISABLE_GH_FALLBACK-}}\" \"${{ADL_GITHUB_OCTOCRAB_BASE_URI-}}\" \"${{GITHUB_TOKEN-}}\" \"${{GH_TOKEN-}}\" \"${{ADL_GITHUB_TOKEN_FILE-}}\" \"${{ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE-}}\" \"${{ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT-}}\" >> '{}'\nexit 0\n",
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
        "ADL_GITHUB_TOKEN_FILE",
        "ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE",
        "ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT",
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
        env::set_var("ADL_GITHUB_TOKEN_FILE", "/tmp/secret-token-file");
        env::set_var("ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE", "secret-service");
        env::set_var("ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT", "secret-account");
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
    assert!(!cargo_env.contains("/tmp/secret-token-file"));
    assert!(!cargo_env.contains("secret-service"));
    assert!(!cargo_env.contains("secret-account"));
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
fn finish_validation_plan_classifies_pr_fast_lane_tooling() {
    let plan = select_finish_validation_plan(
        "adl/tools/run_pr_fast_test_lane.sh,adl/tools/test_run_pr_fast_test_lane.sh",
    )
    .expect("pr fast lane tooling plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_ci_path_policy.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_run_pr_fast_test_lane.sh".to_string()));
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
    let csdlc_rendered_validation = render_default_finish_validation(&csdlc_plan, None);
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
fn finish_validation_plan_classifies_repo_quality_staleness_tooling() {
    let plan = select_finish_validation_plan(
        "adl/tools/check_repo_quality_staleness.py,adl/tools/test_check_repo_quality_staleness.sh,adl/tools/README.md,README.md,CHANGELOG.md,docs/milestones/v0.91.6/RELEASE_PLAN_v0.91.6.md,docs/milestones/v0.91.6/REVIEW_AND_VALIDATION_CHECKLIST_v0.91.6.md",
    )
    .expect("repo-quality staleness plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_check_repo_quality_staleness.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string()));
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(!plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));
}

#[test]
fn finish_validation_plan_classifies_deepseek_suitability_tooling() {
    let plan = select_finish_validation_plan(
        "adl/tools/run_v0916_agent_suitability_panel.py,adl/tools/run_v0916_deepseek_suitability.py,adl/tools/validate_v0916_agent_suitability_panel.py,adl/tools/validate_v0916_deepseek_suitability.py,adl/tools/test_v0916_deepseek_suitability.sh,adl/tools/suitability_specs/deepseek_csdlc_panel_4096.json,docs/milestones/v0.91.6/review/provider/deepseek_suitability/DEEPSEEK_C_SDLC_SUITABILITY_PROOF_2026-06-18.md",
    )
    .expect("deepseek suitability tooling plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_v0916_deepseek_suitability.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string()));
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(!plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));
}

#[test]
fn finish_validation_plan_classifies_openrouter_suitability_tooling() {
    let plan = select_finish_validation_plan(
        "adl/tools/run_v0916_agent_suitability_panel.py,adl/tools/run_v0916_deepseek_suitability.py,adl/tools/suitability_specs/openrouter_current_models_4429.json,docs/milestones/v0.91.6/review/provider/openrouter_current_models/OPENROUTER_CURRENT_MODEL_SUITABILITY_PROOF_2026-06-22.md",
    )
    .expect("openrouter suitability tooling plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_v0916_deepseek_suitability.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string()));
    assert!(plan.commands.contains(&"git diff --check".to_string()));
}

#[test]
fn finish_validation_plan_classifies_openrouter_next_tranche_suitability_tooling() {
    let plan = select_finish_validation_plan(
        "adl/tools/suitability_specs/openrouter_next_tranche_4448.json,docs/milestones/v0.91.6/review/provider/openrouter_next_tranche_4448/OPENROUTER_NEXT_TRANCHE_SUITABILITY_PROOF_2026-06-22.md",
    )
    .expect("openrouter next tranche suitability tooling plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_v0916_deepseek_suitability.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string()));
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(!plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));
}

#[test]
fn finish_validation_plan_classifies_private_endpoint_fixture_sanitation_slice() {
    let plan = select_finish_validation_plan(
        "adl/tools/demo_codex_ollama_operational_skills.sh,adl/tools/demo_v089_gemma4_issue_clerk.sh,adl/tools/test_demo_codex_ollama_operational_skills.sh,adl/tools/test_demo_codex_ollama_semantic_fallback.sh,adl/tools/test_demo_v089_gemma4_issue_clerk.sh,adl/src/provider_substrate.rs,adl/tools/validate_v0915_remote_gemma_watcher_probe.py,demos/v0.87.1/codex_ollama_operational_skills_demo.md,demos/v0.89/gemma4_issue_clerk_demo.md",
    )
    .expect("private endpoint fixture sanitation plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_demo_codex_ollama_operational_skills.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_demo_codex_ollama_semantic_fallback.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_demo_v089_gemma4_issue_clerk.sh".to_string()));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --lib provider_substrate_uses_http_transport_for_ollama_with_endpoint".to_string()
    ));
    assert!(plan.commands.contains(
        &"python3 adl/tools/validate_v0915_remote_gemma_watcher_probe.py docs/milestones/v0.91.5/review/remote_gemma_watcher".to_string()
    ));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string()));
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(!plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));
}

#[test]
fn finish_validation_plan_classifies_ci_runtime_contract_tooling() {
    let plan = select_finish_validation_plan(
        ".github/workflows/ci.yaml,adl/tools/test_ci_runtime_contracts.sh,adl/tools/run_authoritative_coverage_lane.sh,adl/tools/test_run_authoritative_coverage_lane.sh",
    )
    .expect("ci runtime contract tooling plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_ci_runtime_contracts.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_ci_path_policy.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_run_pr_fast_test_lane.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_check_coverage_impact.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string()));
    assert!(plan.commands.contains(&"git diff --check".to_string()));
}

#[test]
fn finish_validation_plan_classifies_resilience_runtime_publication_paths() {
    let agent_comms_plan = select_finish_validation_plan(
        "adl/src/agent_comms.rs,adl/src/agent_comms/carrier.inc,adl/src/agent_comms/tests.inc",
    )
    .expect("agent_comms runtime plan");
    assert_eq!(
        agent_comms_plan.mode,
        FinishValidationMode::LargerBinaryFocused
    );
    assert!(agent_comms_plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml agent_comms --lib -- --nocapture".to_string()
    ));
    assert!(agent_comms_plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh runtime --build".to_string()));
    assert!(!agent_comms_plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));

    let adapter_plan = select_finish_validation_plan("adl/src/provider_adapter.rs")
        .expect("provider adapter runtime plan");
    assert_eq!(adapter_plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(adapter_plan
        .commands
        .contains(&"cargo test --manifest-path adl/Cargo.toml --lib provider_adapter".to_string()));
    assert!(adapter_plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh runtime --build".to_string()));
    assert!(!adapter_plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));

    let provider_plan = select_finish_validation_plan("adl/src/provider_communication.rs")
        .expect("provider communication runtime plan");
    assert_eq!(
        provider_plan.mode,
        FinishValidationMode::LargerBinaryFocused
    );
    assert!(provider_plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --lib provider_communication".to_string()
    ));
    assert!(provider_plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh runtime --build".to_string()));
    assert!(!provider_plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));

    let resilience_plan =
        select_finish_validation_plan("adl/src/resilience.rs").expect("resilience runtime plan");
    assert_eq!(
        resilience_plan.mode,
        FinishValidationMode::LargerBinaryFocused
    );
    assert!(resilience_plan
        .commands
        .contains(&"cargo test --manifest-path adl/Cargo.toml --lib resilience".to_string()));
    assert!(resilience_plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh runtime --build".to_string()));
    assert!(!resilience_plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));

    let mixed_plan = select_finish_validation_plan(
        "adl/src/lib.rs,adl/src/provider_adapter.rs,adl/src/provider_communication.rs,adl/src/resilience.rs,docs/milestones/v0.91.6/features/RESILIENCE_PERSISTENCE_SLEEP_WAKE_v0.91.6.md",
    )
    .expect("mixed resilience runtime plan");
    assert_eq!(mixed_plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(mixed_plan
        .commands
        .contains(&"cargo test --manifest-path adl/Cargo.toml --lib provider_adapter".to_string()));
    assert!(mixed_plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --lib provider_communication".to_string()
    ));
    assert!(mixed_plan
        .commands
        .contains(&"cargo test --manifest-path adl/Cargo.toml --lib resilience".to_string()));
    assert!(mixed_plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh runtime --build".to_string()));
    assert!(!mixed_plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));
}

#[test]
fn finish_validation_plan_classifies_integrated_runtime_soak_runner() {
    let plan = select_finish_validation_plan(
        "adl/src/bin/run_v0916_integrated_runtime_soak.rs,docs/milestones/v0.91.6/review/runtime/V0916_INTEGRATED_RUNTIME_SOAK_PROOF_4245.md",
    )
    .expect("integrated runtime soak plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"cargo test --manifest-path adl/Cargo.toml long_lived_agent".to_string()));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml build_remote_execute_request_preserves_conversation_as_audit_metadata".to_string()
    ));
    assert!(plan
        .commands
        .contains(&"cargo test --manifest-path adl/Cargo.toml remote_exec::".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh runtime --build".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string()));
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(!plan
        .commands
        .iter()
        .any(|command| command.contains("cargo clippy")));
}

#[test]
fn finish_validation_plan_integrated_runtime_soak_runner_hits_helper_classifiers() {
    let plan = select_finish_validation_plan_for_finish(
        4245,
        "adl/src/bin/run_v0916_integrated_runtime_soak.rs,docs/milestones/v0.91.6/review/runtime",
        &[
            "adl/src/bin/run_v0916_integrated_runtime_soak.rs".to_string(),
            "docs/milestones/v0.91.6/review/runtime/V0916_INTEGRATED_RUNTIME_SOAK_PROOF_4245.md"
                .to_string(),
        ],
    )
    .expect("integrated runtime soak finish plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(
        plan.commands
            .iter()
            .any(|command| command
                .contains("bash adl/tools/run_pr_fast_test_lane.sh --changed-files"))
    );
}

#[test]
fn finish_validation_plan_classifies_rust_refactor_slices() {
    let lib_plan = select_finish_validation_plan("adl/src/lib.rs").expect("lib plan");
    assert_eq!(lib_plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(lib_plan
        .commands
        .contains(&"cargo test --manifest-path adl/Cargo.toml --bin adl".to_string()));

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
fn finish_validation_plan_classifies_github_release_tooling_slice() {
    let plan = select_finish_validation_plan("adl/src/cli/tooling_cmd/github_release.rs")
        .expect("github release tooling plan");
    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string()));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl github_release_".to_string()
    ));
}

#[test]
fn finish_validation_plan_classifies_ci_log_archive_tooling_slice() {
    let plan = select_finish_validation_plan(
        "adl/src/cli/tooling_cmd.rs,adl/src/cli/tooling_cmd/ci_log_archive.rs,adl/src/cli/tooling_cmd/tests/tooling_dispatch.rs",
    )
    .expect("ci log archive tooling plan");
    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string()));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml ci_log_archive -- --nocapture".to_string()
    ));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml tooling_cmd_dispatch_and_help_paths_cover_public_entrypoint -- --nocapture".to_string()
    ));
}

#[test]
fn finish_validation_profile_uses_actual_changed_paths_not_broad_stage_request() {
    let docs_plan = select_finish_validation_plan_for_finish(
        1153,
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
        1153,
        ".",
        &["adl/src/cli/pr_cmd/doctor.rs".to_string()],
    )
    .expect("focused actual path plan");
    assert_eq!(focused_plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(focused_plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh csdlc".to_string()));
    assert!(focused_plan.commands.iter().any(|command: &String| command
        .contains("bash adl/tools/run_pr_fast_test_lane.sh --changed-files")));
    assert!(!focused_plan.commands.iter().any(|command: &String| command
        .contains("cargo test --manifest-path adl/Cargo.toml --bin adl cli::pr_cmd")));
}

#[test]
fn finish_validation_profile_treats_issue_records_and_skill_docs_as_docs_only() {
    let plan = select_finish_validation_plan_for_finish(
        1153,
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
fn finish_validation_profile_treats_skill_schema_and_agent_manifest_as_docs_only() {
    let plan = select_finish_validation_plan_for_finish(
        1153,
        ".",
        &[
            "adl/tools/skills/sprint-review/docs/SPRINT_REVIEW_SKILL_INPUT_SCHEMA.md".to_string(),
            "adl/tools/skills/sprint-review/agents/openai.yaml".to_string(),
        ],
    )
    .expect("docs-only skill metadata plan");

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
fn finish_validation_profile_routes_unity_demo_markdown_to_contract_lane() {
    let plan = select_finish_validation_plan_for_finish(
        4030,
        ".",
        &[
            "demos/v0.91.6/unity-observatory/README.md".to_string(),
            "docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_IMPLEMENTATION_BASELINE_4030.md".to_string(),
        ],
    )
    .expect("unity demo markdown contract plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(plan.commands.iter().any(
        |command| command.contains("test_v0916_unity_observatory_local_runtime_consumption.sh")
    ));
    assert!(!plan.commands.iter().any(|command| {
        command.contains("run_authoritative_coverage_lane.sh")
            || command.contains("llvm-cov")
            || command.contains("coverage_release_gate")
    }));
}

#[test]
fn finish_validation_profile_does_not_treat_behavioral_tooling_as_docs_only() {
    let plan = select_finish_validation_plan_for_finish(
        1153,
        ".",
        &[
            "adl/tools/pr.sh".to_string(),
            "docs/milestones/v0.91.5/DOCS_ONLY_VALIDATION_BUNDLE_3736.md".to_string(),
        ],
    )
    .expect("behavioral tooling plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
}

#[test]
fn finish_validation_profile_classifies_unity_observatory_guardrail_script_as_larger_binary_focused(
) {
    let plan = select_finish_validation_plan_for_finish(
        4030,
        ".",
        &[
            "adl/tools/test_v0916_unity_observatory_baseline.sh".to_string(),
            "docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_IMPLEMENTATION_BASELINE_4030.md".to_string(),
        ],
    )
    .expect("unity observatory guardrail slice should resolve to the registered manifest lane");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string()));
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(plan
        .commands
        .iter()
        .any(|command| command.contains("test_v0916_unity_observatory_unity65_smoke.sh")));
    assert!(plan
        .commands
        .iter()
        .any(|command| command.contains("test_v0916_unity_observatory_baseline.sh")));
    assert!(plan
        .commands
        .iter()
        .any(|command| command.contains("test_v0916_unity_observatory_contract.sh")));
}

#[test]
fn finish_validation_profile_classifies_unity_observatory_scaffold_slice_as_larger_binary_focused()
{
    let plan = select_finish_validation_plan_for_finish(
        4031,
        ".",
        &[
            "demos/v0.91.6/unity-observatory/Assets/Scenes/UnityObservatory.unity".to_string(),
            "demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryBootstrap.cs"
                .to_string(),
            "demos/v0.91.6/unity-observatory/PROOF_PACKET.md".to_string(),
        ],
    )
    .expect("unity observatory scaffold plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string()));
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(plan
        .commands
        .iter()
        .any(|command| command.contains("test_v0916_unity_observatory_contract.sh")));
}

#[test]
fn finish_validation_profile_covers_unity_observatory_scaffold_lane_from_manifest() {
    let plan = select_finish_validation_plan_for_finish(
        4032,
        ".",
        &["demos/v0.91.6/unity-observatory/Assets/Scenes/UnityObservatory.unity".to_string()],
    )
    .expect("unity observatory scaffold path should be manifest-covered");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string()));
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(plan
        .commands
        .iter()
        .any(|command| command.contains("test_v0916_unity_observatory_contract.sh")));
    assert!(plan
        .commands
        .iter()
        .any(|command| command.contains("csm_observatory_cli_writes_unity_contract_bundle")));
}

#[test]
fn finish_validation_profile_classifies_unity_observatory_contract_slice_as_larger_binary_focused()
{
    let plan = select_finish_validation_plan_for_finish(
        4032,
        ".",
        &[
            "demos/v0.91.6/unity-observatory/Assets/Resources/observatory_contract.json"
                .to_string(),
            "demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryBootstrap.cs"
                .to_string(),
            "adl/src/csm_observatory.rs".to_string(),
        ],
    )
    .expect("unity observatory contract plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .iter()
        .any(|command| command.contains("test_v0916_unity_observatory_unity65_smoke.sh")));
    assert!(plan
        .commands
        .iter()
        .any(|command| command.contains("test_v0916_unity_observatory_baseline.sh")));
    assert!(plan
        .commands
        .iter()
        .any(|command| command.contains("test_v0916_unity_observatory_contract.sh")));
    assert!(plan.commands.iter().any(|command| command
        .contains("csm_observatory_cli_writes_unity_contract_bundle_and_matches_seeded_resource")));
    assert!(!plan.commands.iter().any(|command| command
        .contains("finish_validation_profile_classifies_unity_observatory_contract_slice")));
}

#[test]
fn finish_validation_profile_classifies_inhabitant_readiness_slice_as_larger_binary_focused() {
    let plan = select_finish_validation_plan_for_finish(
        4033,
        ".",
        &[
            "demos/v0.91.6/unity-observatory/Assets/Resources/observatory_contract.json"
                .to_string(),
            "demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryShellController.cs"
                .to_string(),
            "adl/src/csm_observatory.rs".to_string(),
        ],
    )
    .expect("unity observatory inhabitant-readiness plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .iter()
        .any(|command| command.contains("test_v0916_unity_observatory_contract.sh")));
    assert!(plan.commands.iter().any(|command| command
        .contains("csm_observatory_cli_writes_unity_contract_bundle_and_matches_seeded_resource")));
}

#[test]
fn finish_validation_profile_classifies_observability_consumption_slice_as_larger_binary_focused() {
    let plan = select_finish_validation_plan_for_finish(
        4034,
        ".",
        &[
            "demos/v0.91.6/unity-observatory/Assets/Resources/observatory_contract.json"
                .to_string(),
            "demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryShellController.cs"
                .to_string(),
            "adl/src/csm_observatory.rs".to_string(),
        ],
    )
    .expect("unity observatory observability-consumption plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .iter()
        .any(|command| command.contains("test_v0916_unity_observatory_contract.sh")));
    assert!(plan.commands.iter().any(|command| command
        .contains("csm_observatory_cli_writes_unity_contract_bundle_and_matches_seeded_resource")));
}

#[test]
fn finish_validation_profile_classifies_unity_observatory_repair_slice_as_larger_binary_focused() {
    let plan = select_finish_validation_plan_for_finish(
        4416,
        ".",
        &[
            "demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryBootstrap.cs"
                .to_string(),
            "demos/v0.91.6/unity-observatory/Assets/Scripts/UnityObservatoryShellController.cs"
                .to_string(),
            "demos/v0.91.6/unity-observatory/README.md".to_string(),
            "docs/milestones/v0.91.6/review/V0916_WP09_OBSERVATORY_UNITY_SPRINT_REVIEW_3974.md"
                .to_string(),
        ],
    )
    .expect("unity observatory repair plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .iter()
        .any(|command| command.contains("test_v0916_unity_observatory_contract.sh")));
    assert!(plan.commands.iter().any(|command| command
        .contains("csm_observatory_cli_writes_unity_contract_bundle_and_matches_seeded_resource")));
}

#[test]
fn finish_validation_profile_classifies_html_mobile_observatory_slice_as_small_binary_focused() {
    let plan = select_finish_validation_plan_for_finish(
        4341,
        ".",
        &[
            "adl/src/cli/pr_cmd/finish_support.rs".to_string(),
            "adl/tools/test_demo_v0904_csm_observatory_governed_prototype.sh".to_string(),
            "adl/tools/validate_csm_governed_observatory.py".to_string(),
            "demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json"
                .to_string(),
            "demos/v0.90.4/csm_observatory_governed_prototype.html".to_string(),
            "demos/v0.90.4/csm_observatory_governed_prototype.css".to_string(),
            "demos/v0.90.4/csm_observatory_governed_prototype.js".to_string(),
            "demos/v0.90.4/csm_observatory_governed_prototype.md".to_string(),
            "docs/milestones/v0.91.6/review/observatory/HTML_MOBILE_GOVERNED_OBSERVATORY_PROOF_4341.md".to_string(),
        ],
    )
    .expect("html observatory plan");

    assert_eq!(plan.mode, FinishValidationMode::SmallBinaryFocused);
    assert!(plan.commands.contains(
        &"bash adl/tools/test_demo_v0904_csm_observatory_governed_prototype.sh".to_string()
    ));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation".to_string()
    ));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_helper_paths_run_focused_local_ci_gated_validation".to_string()
    ));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_html_mobile_observatory_slice_as_small_binary_focused -- --nocapture".to_string()
    ));
}

#[test]
fn finish_validation_profile_classifies_sprint_shell_helper_tests_as_small_binary_focused() {
    let plan = select_finish_validation_plan_for_finish(
        1153,
        ".",
        &[
            "adl/tools/test_install_adl_operational_skills.sh".to_string(),
            "adl/tools/test_sprint_conductor_helpers.sh".to_string(),
            "adl/tools/test_pr_run_issue_mode.sh".to_string(),
        ],
    )
    .expect("sprint shell helper plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh csdlc".to_string()));
    assert!(plan.commands.contains(
        &"bash adl/tools/test_sprint_conductor_helpers.sh && bash adl/tools/test_install_adl_operational_skills.sh"
            .to_string()
    ));
}

#[test]
fn finish_validation_profile_keeps_public_prompt_packet_changes_focused() {
    let plan = select_finish_validation_plan_for_finish(
        1153,
        ".",
        &[
            "adl/src/cli/tooling_cmd/public_prompt_packet.rs".to_string(),
            "adl/src/cli/tooling_cmd/tests/public_prompt_packet.rs".to_string(),
            "docs/milestones/v0.91.5/features/PUBLIC_PROMPT_RECORDS_v0.91.5.md".to_string(),
        ],
    )
    .expect("public prompt packet plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(
        plan.commands
            .iter()
            .any(|command| command
                .contains("bash adl/tools/run_pr_fast_test_lane.sh --changed-files"))
    );
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
fn finish_validation_profile_classifies_github_token_loading_surfaces() {
    let err = select_finish_validation_plan_for_finish(
        4001,
        ".",
        &[
            "adl/src/cli/github_token.rs".to_string(),
            "adl/src/cli/mod.rs".to_string(),
            "adl/src/cli/tooling_cmd/github_release.rs".to_string(),
            "adl/src/cli/pr_cmd/github_client.rs".to_string(),
            "adl/src/cli/pr_cmd/github.rs".to_string(),
            "adl/src/cli/tests/pr_cmd_inline/support.rs".to_string(),
            "adl/tools/pr.sh".to_string(),
            "docs/default_workflow.md".to_string(),
        ],
    )
    .expect_err("github token loading slice should fail closed when manager requires escalation");

    let message = err.to_string();
    assert!(message.contains("validation manager reported a non-runnable profile"));
    assert!(message.contains("lane=rust_pr_fast"));
    assert!(message.contains("status=escalation_required"));
}

#[test]
fn finish_validation_profile_classifies_tokio_manifest_runtime_wave_paths() {
    let plan = select_finish_validation_plan_for_finish(
        4178,
        ".",
        &["adl/Cargo.toml".to_string(), "adl/Cargo.lock".to_string()],
    )
    .expect("tokio manifest plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string()));
    assert!(plan
        .commands
        .contains(&"cargo test --manifest-path adl/Cargo.toml pr_cmd::github".to_string()));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl github_release_".to_string()
    ));
    assert!(plan
        .commands
        .contains(&"cargo test --manifest-path adl/Cargo.toml long_lived_agent".to_string()));
}

#[test]
fn finish_validation_profile_classifies_long_lived_agent_tokio_paths() {
    let err = select_finish_validation_plan_for_finish(
        4179,
        ".",
        &[
            "adl/src/long_lived_agent.rs".to_string(),
            "adl/src/long_lived_agent/storage.rs".to_string(),
            "adl/src/long_lived_agent/tests.rs".to_string(),
            "adl/src/runtime_aws_signal.rs".to_string(),
        ],
    )
    .expect_err("long-lived tokio slice should fail closed when manager requires escalation");

    let message = err.to_string();
    assert!(message.contains("validation manager reported a non-runnable profile"));
    assert!(message.contains("lane=rust_pr_fast"));
    assert!(message.contains("status=escalation_required"));
}

#[test]
fn finish_validation_profile_classifies_long_lived_agent_continuity_adjacent_paths() {
    let err = select_finish_validation_plan_for_finish(
        4246,
        ".",
        &[
            "adl/src/demo/stock_league/model.rs".to_string(),
            "adl/src/long_lived_agent/inspection.rs".to_string(),
            "adl/src/long_lived_agent/schema.rs".to_string(),
            "adl/src/long_lived_agent/storage.rs".to_string(),
            "adl/tests/cli_smoke/agent.rs".to_string(),
            "adl/tests/demo_tests.rs".to_string(),
        ],
    )
    .expect_err(
        "long-lived continuity adjacent slice should fail closed when manager requires escalation",
    );

    let message = err.to_string();
    assert!(message.contains("validation manager reported a non-runnable profile"));
    assert!(message.contains("lane=rust_pr_fast"));
    assert!(message.contains("status=escalation_required"));
}

#[test]
fn finish_validation_profile_classifies_tokio_bootstrap_helper_paths() {
    let plan = select_finish_validation_plan_for_finish(
        4180,
        ".",
        &[
            "adl/src/cli/tokio_runtime.rs".to_string(),
            "docs/default_workflow.md".to_string(),
        ],
    )
    .expect("tokio bootstrap helper plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(
        plan.commands
            .iter()
            .any(|command| command
                .contains("bash adl/tools/run_pr_fast_test_lane.sh --changed-files"))
    );
}

#[test]
fn finish_validation_profile_classifies_remote_exec_tokio_paths() {
    let err = select_finish_validation_plan_for_finish(
        4181,
        ".",
        &[
            "adl/src/execute/runner.rs".to_string(),
            "adl/src/execute/tests.rs".to_string(),
            "adl/src/remote_exec.rs".to_string(),
            "adl/src/remote_exec/signing_support.rs".to_string(),
            "adl/src/remote_exec/types.rs".to_string(),
        ],
    )
    .expect_err("remote exec tokio slice should fail closed when manager requires escalation");

    let message = err.to_string();
    assert!(message.contains("validation manager reported a non-runnable profile"));
    assert!(message.contains("lane=rust_pr_fast"));
    assert!(message.contains("status=escalation_required"));
}

#[test]
fn finish_validation_profile_classifies_bounded_cav_tokio_paths() {
    let plan = select_finish_validation_plan_for_finish(
        4182,
        ".",
        &[
            "adl/src/continuous_verification_self_attack.rs".to_string(),
            "adl/src/cli/identity_cmd/tests/adversarial_contracts.rs".to_string(),
        ],
    )
    .expect("bounded cav tokio plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(
        plan.commands
            .iter()
            .any(|command| command
                .contains("bash adl/tools/run_pr_fast_test_lane.sh --changed-files"))
    );
}

#[test]
fn finish_validation_profile_keeps_finish_support_changes_narrow() {
    let plan = select_finish_validation_plan_for_finish(
        4177,
        ".",
        &[
            "adl/src/cli/pr_cmd/finish_support.rs".to_string(),
            "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs".to_string(),
            "docs/default_workflow.md".to_string(),
        ],
    )
    .expect("finish support plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(
        plan.commands
            .iter()
            .any(|command| command
                .contains("bash adl/tools/run_pr_fast_test_lane.sh --changed-files"))
    );
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
fn finish_validation_profile_keeps_pr_janitor_test_repair_fast() {
    let plan = select_finish_validation_plan_for_finish(
        4593,
        ".",
        &["adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs".to_string()],
    )
    .expect("narrow pr-janitor test repair plan");

    assert_eq!(plan.mode, FinishValidationMode::SmallBinaryFocused);
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(plan
        .commands
        .contains(&"cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string()));
    assert!(plan.commands.iter().any(|command| {
        command.starts_with("bash adl/tools/run_pr_fast_test_lane.sh --changed-files ")
    }));
    assert!(!plan
        .commands
        .iter()
        .any(|command| command.contains("cargo nextest")));
    assert!(!plan.commands.iter().any(|command| {
        command.contains("run_authoritative_coverage_lane.sh")
            || command.contains("llvm-cov")
            || command.contains("coverage_release_gate")
    }));
}

#[test]
fn finish_validation_profile_runs_fmt_before_broader_finish_support_lane() {
    let plan = select_finish_validation_plan_for_finish(
        4593,
        ".",
        &[
            "adl/src/cli/pr_cmd/finish_support.rs".to_string(),
            "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs".to_string(),
        ],
    )
    .expect("finish-support repair plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    let fmt_index = plan
        .commands
        .iter()
        .position(|command| command == "cargo fmt --manifest-path adl/Cargo.toml --all --check")
        .expect("fmt command should front-run focused Rust validation");
    let fast_lane_index = plan
        .commands
        .iter()
        .position(|command| {
            command.starts_with("bash adl/tools/run_pr_fast_test_lane.sh --changed-files ")
        })
        .expect("focused rust lane should run");
    assert!(fmt_index < fast_lane_index);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh csdlc".to_string()));
    assert!(!plan.commands.iter().any(|command| {
        command.contains("run_authoritative_coverage_lane.sh")
            || command.contains("llvm-cov")
            || command.contains("coverage_release_gate")
    }));
}

#[test]
fn finish_validation_profile_keeps_validation_policy_repairs_broader_but_not_full_coverage() {
    let plan = select_finish_validation_plan_for_finish(
        4593,
        ".",
        &[
            "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs".to_string(),
            "adl/tools/check_coverage_impact.sh".to_string(),
        ],
    )
    .expect("validation-policy repair plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_check_coverage_impact.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string()));
    assert!(plan.commands.iter().any(|command| {
        command.starts_with("bash adl/tools/run_pr_fast_test_lane.sh --changed-files ")
    }));
    assert!(!plan.commands.iter().any(|command| {
        command.contains("run_authoritative_coverage_lane.sh")
            || command.contains("llvm-cov")
            || command.contains("coverage_release_gate")
    }));
}

#[test]
fn finish_validation_profile_keeps_github_projection_watch_slice_focused() {
    let plan = select_finish_validation_plan_for_finish(
        4488,
        ".",
        &[
            "adl/src/cli/pr_cmd.rs".to_string(),
            "adl/src/cli/pr_cmd/github.rs".to_string(),
            "adl/src/cli/pr_cmd/github/transport.rs".to_string(),
            "adl/src/cli/pr_cmd/github/tests/watch.rs".to_string(),
            "adl/src/cli/pr_cmd/github/tests/validation.rs".to_string(),
            "adl/src/cli/tests/pr_cmd_inline/basics.rs".to_string(),
        ],
    )
    .expect("github projection/watch focused plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string()));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl cli::pr_cmd::github::tests -- --nocapture"
            .to_string()
    ));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl validation_disposition_blocks_pending_and_terminal_failures -- --nocapture"
            .to_string()
    ));
    assert!(
        !plan.commands.contains(
            &"cargo test --manifest-path adl/Cargo.toml --bin adl cli::pr_cmd".to_string()
        ),
        "github projection/watch slices should not fall back to the full pr_cmd lane"
    );
}

#[test]
fn finish_validation_profile_classifies_process_status_helper_surfaces() {
    let plan = select_finish_validation_plan_for_finish(
        0,
        ".",
        &[
            "adl/src/cli/process_cmd.rs".to_string(),
            "adl/src/cli/usage.rs".to_string(),
            "adl/tests/cli_smoke.rs".to_string(),
            "adl/tests/cli_smoke/process_status.rs".to_string(),
            "docs/tooling/PERMISSION_SAFE_PROCESS_STATUS.md".to_string(),
        ],
    )
    .expect("process status helper plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(
        plan.commands
            .iter()
            .any(|command| command
                .contains("bash adl/tools/run_pr_fast_test_lane.sh --changed-files"))
    );
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
fn finish_validation_profile_classifies_lifecycle_inline_tests() {
    let plan = select_finish_validation_plan_for_finish(
        1153,
        ".",
        &["adl/src/cli/tests/pr_cmd_inline/lifecycle/start_ready.rs".to_string()],
    )
    .expect("lifecycle inline test plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(
        plan.commands
            .iter()
            .any(|command| command
                .contains("bash adl/tools/run_pr_fast_test_lane.sh --changed-files"))
    );
}

#[test]
fn finish_validation_profile_keeps_small_binary_delegation_proof_focused() {
    let plan = select_finish_validation_plan_for_finish(
        1153,
        ".",
        &[
            "adl/tools/test_pr_small_binary_delegation.sh".to_string(),
            "docs/milestones/v0.91.5/review/tooling_adoption/PR_LIFECYCLE_SMALL_BINARIES_PROOF_3838.md"
                .to_string(),
        ],
    )
    .expect("small-binary delegation proof plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh csdlc".to_string()));
    assert!(plan.commands.contains(&"git diff --check".to_string()));
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
fn finish_validation_profile_classifies_issue_small_binary_slice() {
    let plan = select_finish_validation_plan_for_finish(
        4216,
        ".",
        &[
            "adl/Cargo.toml".to_string(),
            "adl/src/bin/adl_issue.rs".to_string(),
            "adl/src/cli/pr_cmd/finish_support.rs".to_string(),
            "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs".to_string(),
            "adl/tools/pr.sh".to_string(),
            "adl/tools/test_ci_path_policy.sh".to_string(),
            "adl/tools/test_pr_small_binary_delegation.sh".to_string(),
        ],
    )
    .expect("issue small binary focused plan");

    assert_eq!(plan.mode, FinishValidationMode::SmallBinaryFocused);
    assert!(plan
        .commands
        .contains(&"cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string()));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-issue tests::adl_issue_forwards_args_to_dispatch -- --exact --nocapture"
            .to_string()
    ));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_issue_small_binary_slice -- --exact --nocapture"
            .to_string()
    ));
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
fn finish_validation_profile_classifies_session_ledger_issue_4419_slice() {
    let plan = select_finish_validation_plan_for_finish(
        4419,
        ".",
        &[
            "adl/src/cli/pr_cmd.rs".to_string(),
            "adl/src/cli/pr_cmd/doctor.rs".to_string(),
            "adl/src/cli/pr_cmd/doctor/preflight.rs".to_string(),
            "adl/src/cli/pr_cmd/doctor/printing.rs".to_string(),
            "adl/src/cli/pr_cmd/doctor/tests.rs".to_string(),
            "adl/src/cli/pr_cmd/doctor/types.rs".to_string(),
            "adl/src/cli/pr_cmd/finish_support.rs".to_string(),
            "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs".to_string(),
            "adl/src/cli/tests/pr_cmd_inline/lifecycle/start_ready.rs".to_string(),
            "adl/src/session_ledger.rs".to_string(),
        ],
    )
    .expect("session-ledger issue focused plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string()));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml target_claim_assessment_ -- --nocapture"
            .to_string()
    ));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml doctor_preflight_ -- --nocapture".to_string()
    ));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml real_pr_start_blocks_when_another_session_claims_the_issue -- --exact --nocapture"
            .to_string()
    ));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml real_pr_start_allows_current_session_claim_and_stale_claim_history -- --exact --nocapture"
            .to_string()
    ));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::load_finish_validation_profile_cleans_tempfile_when_profile_only_needs_rendering -- --exact --nocapture"
            .to_string()
    ));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::render_default_finish_validation_includes_profile_truth_and_sanitizes_changed_files -- --exact --nocapture"
            .to_string()
    ));
}

#[test]
fn finish_validation_profile_classifies_delegate_liveness_small_binary_slice() {
    let err = select_finish_validation_plan_for_finish(
        4413,
        ".",
        &[
            "adl/tools/observability.sh".to_string(),
            "adl/tools/pr.sh".to_string(),
            "adl/tools/test_pr_delegate_cargo_fallback_liveness.sh".to_string(),
            "adl/tools/test_pr_delegate_prefers_primary_checkout_binary.sh".to_string(),
        ],
    )
    .expect_err("delegate liveness slice should fail closed when manager requires escalation");

    let message = err.to_string();
    assert!(message.contains("validation manager reported a non-runnable profile"));
    assert!(message.contains("status=escalation_required"));
}

#[test]
fn finish_validation_profile_classifies_closing_linkage_small_binary_slice() {
    let plan = select_finish_validation_plan_for_finish(
        4286,
        ".",
        &[
            "adl/Cargo.toml".to_string(),
            "adl/src/bin/adl_pr_closing_linkage.rs".to_string(),
            "adl/src/cli/pr_cmd.rs".to_string(),
            "adl/src/cli/pr_cmd/github.rs".to_string(),
            "adl/src/cli/pr_cmd/github/tests.rs".to_string(),
            "adl/src/cli/pr_cmd/github/tests/closing_linkage.rs".to_string(),
            "adl/src/cli/pr_cmd_args.rs".to_string(),
            "adl/tools/check_pr_closing_linkage.sh".to_string(),
            "adl/tools/pr.sh".to_string(),
            "adl/tools/run_owner_validation_lane.sh".to_string(),
            "adl/tools/test_pr_closing_linkage.sh".to_string(),
            "adl/tools/test_pr_small_binary_delegation.sh".to_string(),
            "docs/milestones/v0.91.6/review/github_projection/GITHUB_CSDLC_PROJECTION_MAP_4047.md"
                .to_string(),
            "adl/src/cli/pr_cmd/finish_support.rs".to_string(),
            "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs".to_string(),
            ".adl/v0.91.6/tasks/issue-4286__typed-pr-closing-linkage-rust-pvf/spp.md".to_string(),
            ".adl/v0.91.6/tasks/issue-4286__typed-pr-closing-linkage-rust-pvf/srp.md".to_string(),
            ".adl/v0.91.6/tasks/issue-4286__typed-pr-closing-linkage-rust-pvf/sor.md".to_string(),
        ],
    )
    .expect("closing linkage small binary focused plan");

    assert_eq!(plan.mode, FinishValidationMode::SmallBinaryFocused);
    assert!(plan
        .commands
        .contains(&"cargo fmt --manifest-path adl/Cargo.toml --all --check".to_string()));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-pr-closing-linkage closing_linkage -- --nocapture"
            .to_string()
    ));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_closing_linkage_small_binary_slice -- --exact --nocapture"
            .to_string()
    ));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_pr_closing_linkage.sh".to_string()));
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
fn finish_validation_profile_classifies_locked_cargo_fallback_slice() {
    let changed_paths = vec![
        "adl/Cargo.lock".to_string(),
        "adl/config/validation_lane_selector.v0.91.6.json".to_string(),
        "adl/src/cli/pr_cmd/finish_support.rs".to_string(),
        "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs".to_string(),
        "adl/tools/check_coverage_impact.sh".to_string(),
        "adl/tools/pr.sh".to_string(),
        "adl/tools/run_pr_fast_test_lane.sh".to_string(),
        "adl/tools/run_owner_validation_lane.sh".to_string(),
        "adl/tools/test_check_coverage_impact.sh".to_string(),
        "adl/tools/test_control_plane_observability.sh".to_string(),
        "adl/tools/test_five_command_regression_suite.sh".to_string(),
        "adl/tools/test_pr_run_locked_cargo_fallback_refuses_cleanly.sh".to_string(),
        "adl/tools/test_run_pr_fast_test_lane.sh".to_string(),
    ];
    let requested_paths = changed_paths.join(",");

    let plan = select_finish_validation_plan_for_finish(4306, &requested_paths, &changed_paths)
        .expect("locked Cargo fallback focused plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation"
            .to_string()
    ));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_ci_path_policy.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh csdlc".to_string()));

    let unrelated_plan =
        select_finish_validation_plan_for_finish(4305, &requested_paths, &changed_paths).expect(
            "unrelated issue should now resolve through the general unified validation contract",
        );
    assert_eq!(
        unrelated_plan.mode,
        FinishValidationMode::LargerBinaryFocused
    );
    assert!(!unrelated_plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation"
            .to_string()
    ));
    assert!(unrelated_plan
        .commands
        .contains(&"bash adl/tools/test_ci_path_policy.sh && bash adl/tools/test_select_validation_lanes.sh && bash adl/tools/test_validation_manager.sh && bash adl/tools/test_run_nessus_remote_validation.sh".to_string()));
    assert!(unrelated_plan
        .commands
        .contains(&"bash adl/tools/test_check_coverage_impact.sh".to_string()));
    assert!(unrelated_plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh csdlc".to_string()));
    assert!(unrelated_plan
        .commands
        .iter()
        .any(|command| command.contains("test_run_nessus_remote_validation.sh")));
    assert!(unrelated_plan
        .commands
        .iter()
        .any(|command| command
            .starts_with("bash adl/tools/run_pr_fast_test_lane.sh --changed-files ")));
}

#[test]
fn finish_validation_profile_classifies_wuji_ddns_slice() {
    let changed_paths = vec![
        ".gitignore".to_string(),
        "adl/src/cli/pr_cmd/finish_support.rs".to_string(),
        "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs".to_string(),
        "infra/ddns/.terraform.lock.hcl".to_string(),
        "infra/ddns/README.md".to_string(),
        "infra/ddns/client/com.agentlogic.wuji-ddns.plist".to_string(),
        "infra/ddns/client/wuji_ddns_update.sh".to_string(),
        "infra/ddns/iam.tf".to_string(),
        "infra/ddns/lambda.tf".to_string(),
        "infra/ddns/lambda/handler.py".to_string(),
        "infra/ddns/locals.tf".to_string(),
        "infra/ddns/outputs.tf".to_string(),
        "infra/ddns/providers.tf".to_string(),
        "infra/ddns/route53.tf".to_string(),
        "infra/ddns/ssm.tf".to_string(),
        "infra/ddns/tests/test_handler.py".to_string(),
        "infra/ddns/variables.tf".to_string(),
        "infra/ddns/versions.tf".to_string(),
    ];
    let requested_paths = changed_paths.join(",");

    let plan = select_finish_validation_plan_for_finish(4284, &requested_paths, &changed_paths)
        .expect("wuji ddns focused plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish wuji_ddns_slice -- --nocapture"
            .to_string()
    ));
    assert!(plan
        .commands
        .contains(&"python3 -m unittest infra/ddns/tests/test_handler.py".to_string()));
    assert!(plan
        .commands
        .contains(&"sh -n infra/ddns/client/wuji_ddns_update.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"terraform -chdir=infra/ddns fmt -check".to_string()));
    assert!(plan
        .commands
        .contains(&"terraform -chdir=infra/ddns init -backend=false".to_string()));
    assert!(plan
        .commands
        .contains(&"terraform -chdir=infra/ddns validate".to_string()));

    let unrelated_err =
        select_finish_validation_plan_for_finish(4285, &requested_paths, &changed_paths)
            .expect_err("unrelated issue should not inherit the wuji ddns focused allowance");
    assert!(unrelated_err
        .to_string()
        .contains("selector left changed paths without validation-lane coverage"));
}

#[test]
fn finish_validation_profile_classifies_wuji_ddns_installer_slice() {
    let changed_paths = vec![
        "adl/src/cli/pr_cmd/finish_support.rs".to_string(),
        "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs".to_string(),
        "infra/ddns/README.md".to_string(),
        "infra/ddns/client/install_wuji_ddns_launchd.sh".to_string(),
    ];
    let requested_paths = changed_paths.join(",");

    let plan = select_finish_validation_plan_for_finish(4330, &requested_paths, &changed_paths)
        .expect("wuji ddns installer focused plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish wuji_ddns_installer_slice -- --nocapture"
            .to_string()
    ));
    assert!(plan
        .commands
        .contains(&"sh -n infra/ddns/client/install_wuji_ddns_launchd.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"sh -n infra/ddns/client/wuji_ddns_update.sh".to_string()));

    let unrelated_err =
        select_finish_validation_plan_for_finish(4331, &requested_paths, &changed_paths)
            .expect_err("unrelated issue should not inherit the installer focused allowance");
    assert!(unrelated_err
        .to_string()
        .contains("selector left changed paths without validation-lane coverage"));
}

#[test]
fn finish_validation_runner_executes_locked_cargo_fallback_script_command() {
    let repo = unique_temp_dir("adl-pr-finish-locked-cargo-fallback-validation");
    let tools = repo.join("adl/tools");
    fs::create_dir_all(&tools).expect("tools dir");
    write_executable(
        &tools.join("check_no_tracked_adl_issue_record_residue.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\n",
    );
    write_executable(
        &tools.join("test_pr_run_locked_cargo_fallback_refuses_cleanly.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nrepo_root=\"$(cd \"$(dirname \"$0\")/../..\" && pwd)\"\necho locked-fallback-ran > \"$repo_root/locked-fallback-ran.txt\"\n",
    );

    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(&repo)
        .status()
        .expect("git init")
        .success());

    let plan = FinishValidationPlan {
        mode: FinishValidationMode::LargerBinaryFocused,
        commands: vec![
            "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
            "git diff --check".to_string(),
            "bash adl/tools/test_pr_run_locked_cargo_fallback_refuses_cleanly.sh".to_string(),
        ],
    };

    run_finish_validation_rust(&repo, &plan).expect("validation runner");
    assert_eq!(
        fs::read_to_string(repo.join("locked-fallback-ran.txt"))
            .expect("runner marker")
            .trim(),
        "locked-fallback-ran"
    );
}

#[test]
fn finish_validation_runner_executes_html_observatory_demo_script_command() {
    let repo = unique_temp_dir("adl-pr-finish-html-observatory-validation");
    let tools = repo.join("adl/tools");
    fs::create_dir_all(&tools).expect("tools dir");
    write_executable(
        &tools.join("check_no_tracked_adl_issue_record_residue.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\n",
    );
    write_executable(
        &tools.join("test_demo_v0904_csm_observatory_governed_prototype.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nrepo_root=\"$(cd \"$(dirname \"$0\")/../..\" && pwd)\"\necho html-observatory-ran > \"$repo_root/html-observatory-ran.txt\"\n",
    );

    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(&repo)
        .status()
        .expect("git init")
        .success());

    let plan = FinishValidationPlan {
        mode: FinishValidationMode::SmallBinaryFocused,
        commands: vec![
            "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
            "git diff --check".to_string(),
            "bash adl/tools/test_demo_v0904_csm_observatory_governed_prototype.sh".to_string(),
        ],
    };

    run_finish_validation_rust(&repo, &plan).expect("validation runner");
    assert_eq!(
        fs::read_to_string(repo.join("html-observatory-ran.txt"))
            .expect("runner marker")
            .trim(),
        "html-observatory-ran"
    );
}

#[test]
fn finish_validation_profile_classifies_validation_manager_slice_as_small_binary_focused() {
    let plan = select_finish_validation_plan_for_finish(
        4215,
        ".",
        &[
            "adl/tools/validation_manager.py".to_string(),
            "adl/tools/validation_manager.sh".to_string(),
            "adl/tools/test_validation_manager.sh".to_string(),
        ],
    )
    .expect("validation manager plan");

    assert_eq!(plan.mode, FinishValidationMode::SmallBinaryFocused);
    assert!(plan.commands.iter().any(|command| {
        command.contains("bash adl/tools/test_ci_path_policy.sh")
            && command.contains("bash adl/tools/test_select_validation_lanes.sh")
            && command.contains("bash adl/tools/test_validation_manager.sh")
    }));
    assert!(!plan
        .commands
        .iter()
        .any(|command| command.contains("cargo test --manifest-path adl/Cargo.toml --bin adl")));
}

#[test]
fn finish_validation_profile_fails_closed_with_manager_guidance_for_unmapped_paths() {
    let err = select_finish_validation_plan_for_finish(
        4421,
        ".",
        &["totally/unmapped/path.txt".to_string()],
    )
    .expect_err("unmapped paths should fail closed");

    let message = err.to_string();
    assert!(message.contains("validation manager reported a non-runnable profile"));
    assert!(message.contains("profile="));
    assert!(message.contains("lane=unmapped_change_surface"));
    assert!(message.contains("matched_paths=totally/unmapped/path.txt"));
    assert!(message.contains("manifest_rule=adl/config/validation_lane_selector.v0.91.6.json"));
    assert!(message.contains("remediation_hint=Add or refine a path selector"));
}

#[test]
fn finish_validation_profile_fails_closed_for_mixed_surface_with_unmapped_gap() {
    let err = select_finish_validation_plan_for_finish(
        4421,
        ".",
        &[
            "docs/milestones/v0.91.6/README.md".to_string(),
            "totally/unmapped/path.txt".to_string(),
        ],
    )
    .expect_err("mixed mapped and unmapped paths should fail closed");

    let message = err.to_string();
    assert!(message.contains("profile=docs_diff_check_profile"));
    assert!(message.contains("status=escalation_required"));
    assert!(message.contains("pr_publication_sufficient=false"));
    assert!(message.contains("lane=unmapped_change_surface"));
    assert!(message.contains("matched_paths=totally/unmapped/path.txt"));
}

#[test]
fn finish_validation_profile_accepts_workflow_metrics_backfill_slice() {
    let plan = select_finish_validation_plan_for_finish(
        4441,
        ".",
        &[
            "adl/src/cli/pr_cmd.rs".to_string(),
            "adl/src/cli/pr_cmd/github.rs".to_string(),
            "adl/src/cli/pr_cmd/github/tests/transport.rs".to_string(),
            "adl/src/cli/pr_cmd/github/tests/watch.rs".to_string(),
            "adl/src/cli/tests/pr_cmd_inline/basics.rs".to_string(),
            "adl/tools/build_v0916_workflow_metric_backfill_inventory.py".to_string(),
            "docs/milestones/v0.91.6/review/V0916_WORKFLOW_METRIC_BACKFILL_4441.json".to_string(),
        ],
    )
    .expect("workflow metrics backfill plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh csdlc".to_string()));
    assert!(plan.commands.iter().any(|command| {
        command.starts_with("bash adl/tools/run_pr_fast_test_lane.sh --changed-files ")
    }));
    assert!(plan.commands.contains(&"git diff --check".to_string()));
}

#[test]
fn finish_validation_profile_accepts_workflow_metrics_backfill_publication_slice() {
    let plan = select_finish_validation_plan_for_finish(
        4441,
        ".",
        &[
            "adl/config/validation_lane_selector.v0.91.6.json".to_string(),
            "adl/src/cli/pr_cmd.rs".to_string(),
            "adl/src/cli/pr_cmd/github.rs".to_string(),
            "adl/src/cli/pr_cmd/github/tests/transport.rs".to_string(),
            "adl/src/cli/pr_cmd/github/tests/watch.rs".to_string(),
            "adl/src/cli/tests/pr_cmd_inline/basics.rs".to_string(),
            "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs".to_string(),
            "adl/tools/build_v0916_workflow_metric_backfill_inventory.py".to_string(),
            "adl/tools/test_select_validation_lanes.sh".to_string(),
            "adl/tools/test_validation_manager.sh".to_string(),
            "docs/milestones/v0.91.6/review/V0916_WORKFLOW_METRIC_BACKFILL_4441.json".to_string(),
        ],
    )
    .expect("workflow metrics backfill publication plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh csdlc".to_string()));
    assert!(plan.commands.iter().any(|command| {
        command.contains("bash adl/tools/test_ci_path_policy.sh")
            && command.contains("bash adl/tools/test_select_validation_lanes.sh")
            && command.contains("bash adl/tools/test_validation_manager.sh")
    }));
    assert!(plan.commands.iter().any(|command| {
        command.starts_with("bash adl/tools/run_pr_fast_test_lane.sh --changed-files ")
    }));
    assert!(plan.commands.contains(&"git diff --check".to_string()));
}

#[test]
fn finish_validation_profile_classifies_validation_inventory_slice_as_small_binary_focused() {
    let err = select_finish_validation_plan_for_finish(
        4213,
        ".",
        &[
            "adl/tools/validation_inventory.py".to_string(),
            "adl/tools/validation_inventory.sh".to_string(),
            "adl/tools/test_validation_inventory.sh".to_string(),
        ],
    )
    .expect_err("validation inventory slice should fail closed when manager requires escalation");

    let message = err.to_string();
    assert!(message.contains("validation manager reported a non-runnable profile"));
    assert!(message.contains("status=escalation_required"));
}

#[test]
fn finish_validation_profile_runs_sprint_conductor_helper_validation_for_metrics_scripts() {
    let plan = select_finish_validation_plan(
        "adl/tools/skills/sprint-conductor/scripts/issue_goal_metrics.py,adl/tools/skills/sprint-conductor/scripts/record_codex_goal_tool_snapshot.py,adl/tools/test_sprint_conductor_helpers.sh,docs/default_workflow.md",
    )
    .expect("sprint conductor helper metrics plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_sprint_conductor_helpers.sh".to_string()));
}

#[test]
fn finish_helper_paths_run_sprint_conductor_helper_validation() {
    let repo = unique_temp_dir("adl-pr-finish-sprint-conductor-helper-validation");
    let tools = repo.join("adl/tools");
    fs::create_dir_all(&tools).expect("tools dir");
    write_executable(
        &tools.join("check_no_tracked_adl_issue_record_residue.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\n",
    );
    write_executable(
        &tools.join("test_sprint_conductor_helpers.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nrepo_root=\"$(cd \"$(dirname \"$0\")/../..\" && pwd)\"\necho sprint-conductor-ran > \"$repo_root/sprint-conductor-ran.txt\"\n",
    );
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(&repo)
        .status()
        .expect("git init")
        .success());

    let plan = FinishValidationPlan {
        mode: FinishValidationMode::LargerBinaryFocused,
        commands: vec![
            "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
            "git diff --check".to_string(),
            "bash adl/tools/test_sprint_conductor_helpers.sh".to_string(),
        ],
    };
    run_finish_validation_rust(&repo, &plan).expect("sprint conductor helper validation");
    assert_eq!(
        fs::read_to_string(repo.join("sprint-conductor-ran.txt"))
            .expect("runner marker")
            .trim(),
        "sprint-conductor-ran"
    );
}

#[test]
fn finish_validation_profile_classifies_slow_proof_family_split_slice() {
    let err = select_finish_validation_plan_for_finish(
        4219,
        ".",
        &[
            "adl/Cargo.toml".to_string(),
            "adl/config/slow_proof_families.v0.91.6.json".to_string(),
            "adl/src/runtime_v2/tests.rs".to_string(),
            "adl/src/runtime_v2/tests/private_state_observatory.rs".to_string(),
            "adl/tools/run_slow_proof_family.sh".to_string(),
            "adl/tools/test_slow_proof_lane_contract.sh".to_string(),
            "adl/tools/validation_inventory.py".to_string(),
            "adl/tools/test_validation_inventory.sh".to_string(),
            "adl/tools/validation_manager.py".to_string(),
            "adl/tools/test_validation_manager.sh".to_string(),
            "adl/tools/ci_path_policy.sh".to_string(),
        ],
    )
    .expect_err(
        "slow-proof family split slice should fail closed when manager requires escalation",
    );

    let message = err.to_string();
    assert!(message.contains("validation manager reported a non-runnable profile"));
    assert!(message.contains("lane=slow_proof_review"));
    assert!(message.contains("status=escalation_required"));
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
    write_executable(
        &repo.join("adl/tools/test_ci_runtime_contracts.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' runtime-contracts >> \"$FOCUSED_LOG\"\n",
    );
    write_executable(
        &repo.join("adl/tools/test_run_pr_fast_test_lane.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' pr-fast >> \"$FOCUSED_LOG\"\n",
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
    assert!(focused_calls.contains("runtime-contracts"));
    assert!(focused_calls.contains("pr-fast"));
}

#[test]
fn finish_helper_paths_run_pr_fast_lane_validation() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-pr-fast-lane-validation");
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
    write_executable(
        &repo.join("adl/tools/test_ci_path_policy.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' path-policy >> \"$FOCUSED_LOG\"\n",
    );
    write_executable(
        &repo.join("adl/tools/test_run_pr_fast_test_lane.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' pr-fast >> \"$FOCUSED_LOG\"\n",
    );
    init_git_repo(&repo);

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let cargo_log = temp.join("cargo.log");
    let focused_log = temp.join("focused.log");
    write_executable(
        &bin_dir.join("cargo"),
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
        "adl/tools/run_pr_fast_test_lane.sh,adl/tools/test_run_pr_fast_test_lane.sh",
    )
    .expect("pr fast lane plan");
    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    run_finish_validation_rust(&repo, &plan).expect("pr fast lane validation");

    unsafe {
        env::set_var("PATH", old_path);
    }
    match old_focused_log {
        Some(value) => unsafe { env::set_var("FOCUSED_LOG", value) },
        None => unsafe { env::remove_var("FOCUSED_LOG") },
    }

    assert!(
        !cargo_log.exists(),
        "pr fast lane helper validation should not invoke cargo"
    );

    let focused_calls = fs::read_to_string(&focused_log).expect("focused log");
    assert!(focused_calls.contains("path-policy"));
    assert!(focused_calls.contains("pr-fast"));
}

#[test]
fn finish_helper_paths_run_validation_selector_validation() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-validation-selector-validation");
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
    write_executable(
        &repo.join("adl/tools/test_select_validation_lanes.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' selector >> \"$FOCUSED_LOG\"\n",
    );
    init_git_repo(&repo);

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let cargo_log = temp.join("cargo.log");
    let focused_log = temp.join("focused.log");
    write_executable(
        &bin_dir.join("cargo"),
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
        "adl/config/validation_lane_selector.v0.91.6.json,adl/tools/select_validation_lanes.py,adl/tools/select_validation_lanes.sh,adl/tools/test_select_validation_lanes.sh",
    )
    .expect("validation selector plan");
    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    run_finish_validation_rust(&repo, &plan).expect("validation selector validation");

    unsafe {
        env::set_var("PATH", old_path);
    }
    match old_focused_log {
        Some(value) => unsafe { env::set_var("FOCUSED_LOG", value) },
        None => unsafe { env::remove_var("FOCUSED_LOG") },
    }

    assert!(
        !cargo_log.exists(),
        "validation selector focused validation should not invoke cargo"
    );

    let focused_calls = fs::read_to_string(&focused_log).expect("focused log");
    assert!(focused_calls.contains("selector"));
}

#[test]
fn finish_helper_paths_run_manager_backed_owner_and_pr_fast_validation() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-manager-backed-validation");
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
    write_executable(
        &repo.join("adl/tools/run_owner_validation_lane.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf 'owner:%s\\n' \"$1\" >> \"$FOCUSED_LOG\"\n",
    );
    write_executable(
        &repo.join("adl/tools/run_pr_fast_test_lane.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf 'pr-fast:%s\\n' \"$2\" >> \"$FOCUSED_LOG\"\n",
    );
    init_git_repo(&repo);

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let cargo_log = temp.join("cargo.log");
    let focused_log = temp.join("focused.log");
    let changed_files = temp.join("finish-validation-profile-safe.txt");
    fs::write(&changed_files, "M\tadl/src/cli/pr_cmd/doctor.rs\n").expect("changed files");
    write_executable(
        &bin_dir.join("cargo"),
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

    let plan = FinishValidationPlan {
        mode: FinishValidationMode::LargerBinaryFocused,
        commands: vec![
            "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
            "git diff --check".to_string(),
            "bash adl/tools/run_owner_validation_lane.sh csdlc".to_string(),
            format!(
                "bash adl/tools/run_pr_fast_test_lane.sh --changed-files {}",
                changed_files.display()
            ),
        ],
    };
    run_finish_validation_rust(&repo, &plan).expect("manager-backed owner and pr-fast validation");

    unsafe {
        env::set_var("PATH", old_path);
    }
    match old_focused_log {
        Some(value) => unsafe { env::set_var("FOCUSED_LOG", value) },
        None => unsafe { env::remove_var("FOCUSED_LOG") },
    }

    assert!(
        !cargo_log.exists(),
        "manager-backed focused validation should not invoke cargo"
    );

    let focused_calls = fs::read_to_string(&focused_log).expect("focused log");
    assert!(focused_calls.contains("owner:csdlc"));
    assert!(focused_calls.contains(&format!("pr-fast:{}", changed_files.display())));
    assert!(
        !changed_files.exists(),
        "manager-backed pr-fast helper execution should clean up the changed-file manifest"
    );
}

#[test]
fn finish_helper_paths_run_unity_observatory_soak_lane_validation() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-unity-observatory-soak-lane-validation");
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
    write_executable(
        &repo.join("adl/tools/test_v0916_unity_observatory_unity65_smoke.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' unity65-smoke:$1 >> \"$FOCUSED_LOG\"\n",
    );
    write_executable(
        &repo.join("adl/tools/test_v0916_unity_observatory_baseline.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' baseline >> \"$FOCUSED_LOG\"\n",
    );
    write_executable(
        &repo.join("adl/tools/test_v0916_unity_observatory_contract.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' contract >> \"$FOCUSED_LOG\"\n",
    );
    write_executable(
        &repo.join("adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' local-runtime >> \"$FOCUSED_LOG\"\n",
    );
    write_executable(
        &repo.join("adl/tools/test_v0916_unity_observatory_soak_integration.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' soak >> \"$FOCUSED_LOG\"\n",
    );
    init_git_repo(&repo);

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let cargo_log = temp.join("cargo.log");
    let focused_log = temp.join("focused.log");
    write_executable(
        &bin_dir.join("cargo"),
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

    let plan = FinishValidationPlan {
        mode: FinishValidationMode::LargerBinaryFocused,
        commands: vec![
            "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
            "git diff --check".to_string(),
            "bash -n adl/tools/test_v0916_unity_observatory_unity65_smoke.sh && bash adl/tools/test_v0916_unity_observatory_baseline.sh && bash adl/tools/test_v0916_unity_observatory_contract.sh && bash adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh && bash adl/tools/test_v0916_unity_observatory_soak_integration.sh && cargo test --manifest-path adl/Cargo.toml --test cli_smoke csm_observatory_cli_writes_unity_contract_bundle_and_matches_seeded_resource -- --nocapture".to_string(),
        ],
    };
    run_finish_validation_rust(&repo, &plan).expect("unity observatory soak lane validation");

    unsafe {
        env::set_var("PATH", old_path);
    }
    match old_focused_log {
        Some(value) => unsafe { env::set_var("FOCUSED_LOG", value) },
        None => unsafe { env::remove_var("FOCUSED_LOG") },
    }

    let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
    assert!(cargo_calls.contains("test --manifest-path"));
    assert!(cargo_calls.contains("--test cli_smoke"));
    assert!(cargo_calls
        .contains("csm_observatory_cli_writes_unity_contract_bundle_and_matches_seeded_resource"));

    let focused_calls = fs::read_to_string(&focused_log).expect("focused log");
    assert!(
        !focused_calls.contains("unity65-smoke"),
        "bash -n should syntax-check the Unity 6.5 smoke script without executing it"
    );
    assert!(focused_calls.contains("baseline"));
    assert!(focused_calls.contains("contract"));
    assert!(focused_calls.contains("local-runtime"));
    assert!(focused_calls.contains("soak"));
}

#[test]
fn finish_helper_rejects_manager_backed_pr_fast_changed_files_substitution() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-manager-rejects-substituted-changed-files");

    let repo = temp.join("repo");
    fs::create_dir_all(repo.join("adl/tools")).expect("adl tools dir");
    fs::write(
        repo.join("adl/Cargo.toml"),
        "[package]\nname='adl'\nversion='0.1.0'\n",
    )
    .expect("manifest");
    write_executable(
        &repo.join("adl/tools/check_no_tracked_adl_issue_record_residue.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 0\n",
    );
    write_executable(
        &repo.join("adl/tools/run_pr_fast_test_lane.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 0\n",
    );
    init_git_repo(&repo);

    let substituted = temp.join("not-adl-created.txt");
    fs::write(&substituted, "M\tadl/src/cli/pr_cmd/doctor.rs\n")
        .expect("substituted changed files");
    let plan = FinishValidationPlan {
        mode: FinishValidationMode::LargerBinaryFocused,
        commands: vec![
            "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
            "git diff --check".to_string(),
            format!(
                "bash adl/tools/run_pr_fast_test_lane.sh --changed-files {}",
                substituted.display()
            ),
        ],
    };

    let err = run_finish_validation_rust(&repo, &plan)
        .expect_err("substituted changed-files manifest should be refused");
    assert!(err
        .to_string()
        .contains("expected ADL-created finish-validation-profile-*.txt"));
    assert!(
        substituted.exists(),
        "refused substituted manifest must not be deleted"
    );
}

#[test]
fn finish_helper_paths_run_validation_manager_validation() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-validation-manager-validation");

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
    write_executable(
        &repo.join("adl/tools/test_validation_manager.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' validation-manager >> \"$FOCUSED_LOG\"\n",
    );
    init_git_repo(&repo);

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let cargo_log = temp.join("cargo.log");
    let focused_log = temp.join("focused.log");
    write_executable(
        &bin_dir.join("cargo"),
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
        "adl/tools/validation_manager.py,adl/tools/validation_manager.sh,adl/tools/test_validation_manager.sh",
    )
    .expect("validation manager plan");
    assert_eq!(plan.mode, FinishValidationMode::SmallBinaryFocused);
    run_finish_validation_rust(&repo, &plan).expect("validation manager validation");

    unsafe {
        env::set_var("PATH", old_path);
    }
    match old_focused_log {
        Some(value) => unsafe { env::set_var("FOCUSED_LOG", value) },
        None => unsafe { env::remove_var("FOCUSED_LOG") },
    }

    assert!(
        !cargo_log.exists(),
        "validation manager focused validation should not invoke cargo"
    );

    let focused_calls = fs::read_to_string(&focused_log).expect("focused log");
    assert!(focused_calls.contains("validation-manager"));
}

#[test]
fn finish_helper_paths_run_registered_runtime_owner_lane_command() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-registered-runtime-owner-lane");
    let repo = temp.join("repo");
    fs::create_dir_all(repo.join("adl/tools")).expect("adl tools dir");
    fs::create_dir_all(repo.join("adl/config")).expect("adl config dir");
    fs::write(
        repo.join("adl/Cargo.toml"),
        "[package]\nname='adl'\nversion='0.1.0'\n",
    )
    .expect("cargo toml");
    fs::write(
        repo.join("adl/config/validation_lane_selector.v0.91.6.json"),
        r#"{"schema_version":"adl.validation_lane_selector.v1","lanes":[{"id":"runtime_owner_lane","run_command":"bash adl/tools/run_owner_validation_lane.sh runtime"}]}"#,
    )
    .expect("validation manifest");
    write_executable(
        &repo.join("adl/tools/check_no_tracked_adl_issue_record_residue.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 0\n",
    );
    write_executable(
        &repo.join("adl/tools/run_owner_validation_lane.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf 'owner:%s\\n' \"$1\" >> \"$FOCUSED_LOG\"\n",
    );
    init_git_repo(&repo);

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let focused_log = temp.join("focused.log");
    let old_path = env::var("PATH").unwrap_or_default();
    let old_focused_log = env::var("FOCUSED_LOG").ok();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("FOCUSED_LOG", &focused_log);
    }

    let plan = FinishValidationPlan {
        mode: FinishValidationMode::SmallBinaryFocused,
        commands: vec![
            "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
            "git diff --check".to_string(),
            "bash adl/tools/run_owner_validation_lane.sh runtime".to_string(),
        ],
    };
    run_finish_validation_rust(&repo, &plan).expect("registered runtime owner lane validation");

    unsafe {
        env::set_var("PATH", old_path);
    }
    match old_focused_log {
        Some(value) => unsafe { env::set_var("FOCUSED_LOG", value) },
        None => unsafe { env::remove_var("FOCUSED_LOG") },
    }

    let focused_calls = fs::read_to_string(&focused_log).expect("focused log");
    assert!(focused_calls.contains("owner:runtime"));
}

#[test]
fn finish_helper_paths_run_registered_polis_wrapper_command() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-registered-polis-wrapper");
    let repo = temp.join("repo");
    fs::create_dir_all(repo.join("adl/tools")).expect("adl tools dir");
    fs::create_dir_all(repo.join("adl/config")).expect("adl config dir");
    fs::write(
        repo.join("adl/Cargo.toml"),
        "[package]\nname='adl'\nversion='0.1.0'\n",
    )
    .expect("cargo toml");
    fs::write(
        repo.join("adl/config/validation_lane_selector.v0.91.6.json"),
        r#"{"schema_version":"adl.validation_lane_selector.v1","lanes":[{"id":"local_polis_ssm_wrapper","run_command":"bash -n adl/tools/polis_status_for_ssm.sh && bash -n adl/tools/polis_status_for_ssm_qts.sh && python3 adl/tools/validate_polis_status_for_ssm_qts.py"}]}"#,
    )
    .expect("validation manifest");
    write_executable(
        &repo.join("adl/tools/check_no_tracked_adl_issue_record_residue.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 0\n",
    );
    write_executable(
        &repo.join("adl/tools/polis_status_for_ssm.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf 'polis-status\\n'\n",
    );
    write_executable(
        &repo.join("adl/tools/polis_status_for_ssm_qts.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf 'polis-status-qts\\n'\n",
    );
    fs::write(
        repo.join("adl/tools/validate_polis_status_for_ssm_qts.py"),
        "from pathlib import Path\nimport os\nPath(os.environ['FOCUSED_LOG']).write_text('validated\\n')\n",
    )
    .expect("python validator");
    init_git_repo(&repo);

    let old_focused_log = env::var("FOCUSED_LOG").ok();
    let focused_log = temp.join("focused.log");
    unsafe {
        env::set_var("FOCUSED_LOG", &focused_log);
    }

    let plan = FinishValidationPlan {
        mode: FinishValidationMode::SmallBinaryFocused,
        commands: vec![
            "bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string(),
            "git diff --check".to_string(),
            "bash -n adl/tools/polis_status_for_ssm.sh && bash -n adl/tools/polis_status_for_ssm_qts.sh && python3 adl/tools/validate_polis_status_for_ssm_qts.py".to_string(),
        ],
    };
    run_finish_validation_rust(&repo, &plan).expect("registered polis wrapper validation");

    match old_focused_log {
        Some(value) => unsafe { env::set_var("FOCUSED_LOG", value) },
        None => unsafe { env::remove_var("FOCUSED_LOG") },
    }

    assert_eq!(
        fs::read_to_string(&focused_log)
            .expect("focused log")
            .trim(),
        "validated"
    );
}

#[test]
fn finish_validation_profile_fails_closed_when_ready_profile_command_is_not_publishable() {
    let temp = unique_temp_dir("adl-pr-finish-unpublishable-registered-command");
    let repo = temp.join("repo");
    fs::create_dir_all(repo.join("adl/config")).expect("adl config dir");
    fs::write(
        repo.join("adl/config/validation_lane_selector.v0.91.6.json"),
        r#"{"schema_version":"adl.validation_lane_selector.v1","lanes":[{"id":"unsupported_lane","run_command":"bash adl/tools/not-a-real-registered-command.sh --dangerous"}]}"#,
    )
    .expect("validation manifest");
    let profile = FinishValidationProfile {
        selected_profile: "unsupported_profile".to_string(),
        status: "ready_to_run".to_string(),
        pr_publication_sufficient: true,
        run: vec![FinishValidationProfileRunItem {
            lane_id: "unsupported_lane".to_string(),
            command: "bash adl/tools/not-a-real-registered-command.sh --dangerous".to_string(),
            reason: "fixture".to_string(),
            matched_paths: vec!["adl/tools/not-a-real-registered-command.sh".to_string()],
            vpp_record: None,
        }],
        not_run: Vec::new(),
        deferred: Vec::new(),
        escalation: FinishValidationProfileEscalation {
            required: false,
            reasons: Vec::new(),
        },
    };

    let err = ensure_finish_validation_profile_is_runnable(
        &repo,
        &profile,
        &["adl/tools/not-a-real-registered-command.sh".to_string()],
    )
    .expect_err("unsupported registered command should fail closed");
    assert!(err
        .to_string()
        .contains("selected validation lane cannot be published by finish yet"));
}

#[test]
fn finish_validation_profile_accepts_ready_profile_with_registered_nessus_remote_validation_command(
) {
    let temp = unique_temp_dir("adl-pr-finish-publishable-nessus-remote-validation-command");
    let repo = temp.join("repo");
    fs::create_dir_all(repo.join("adl/config")).expect("adl config dir");
    fs::write(
        repo.join("adl/config/validation_lane_selector.v0.91.6.json"),
        r#"{"schema_version":"adl.validation_lane_selector.v1","lanes":[{"id":"validation_manager_surface","run_command":"bash adl/tools/test_ci_path_policy.sh && bash adl/tools/test_select_validation_lanes.sh && bash adl/tools/test_validation_manager.sh && bash adl/tools/test_run_nessus_remote_validation.sh"}]}"#,
    )
    .expect("validation manifest");
    let profile = FinishValidationProfile {
        selected_profile: "validation_manager_surface".to_string(),
        status: "ready_to_run".to_string(),
        pr_publication_sufficient: true,
        run: vec![FinishValidationProfileRunItem {
            lane_id: "validation_manager_surface".to_string(),
            command: "bash adl/tools/test_ci_path_policy.sh && bash adl/tools/test_select_validation_lanes.sh && bash adl/tools/test_validation_manager.sh && bash adl/tools/test_run_nessus_remote_validation.sh".to_string(),
            reason: "fixture".to_string(),
            matched_paths: vec!["adl/tools/test_run_nessus_remote_validation.sh".to_string()],
            vpp_record: None,
        }],
        not_run: Vec::new(),
        deferred: Vec::new(),
        escalation: FinishValidationProfileEscalation {
            required: false,
            reasons: Vec::new(),
        },
    };

    ensure_finish_validation_profile_is_runnable(
        &repo,
        &profile,
        &["adl/tools/test_run_nessus_remote_validation.sh".to_string()],
    )
    .expect("registered nessus remote validation command should be publishable");
}

#[test]
fn finish_runner_executes_combined_ci_policy_selector_command() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-combined-ci-policy-selector-command");

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
    write_executable(
        &repo.join("adl/tools/test_ci_path_policy.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' ci-path-policy >> \"$FOCUSED_LOG\"\n",
    );
    write_executable(
        &repo.join("adl/tools/test_select_validation_lanes.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' select-validation-lanes >> \"$FOCUSED_LOG\"\n",
    );
    write_executable(
        &repo.join("adl/tools/test_validation_manager.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' validation-manager >> \"$FOCUSED_LOG\"\n",
    );
    write_executable(
        &repo.join("adl/tools/test_run_nessus_remote_validation.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' nessus-remote-runner >> \"$FOCUSED_LOG\"\n",
    );
    init_git_repo(&repo);

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let cargo_log = temp.join("cargo.log");
    let focused_log = temp.join("focused.log");
    write_executable(
        &bin_dir.join("cargo"),
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

    let plan = FinishValidationPlan {
        mode: FinishValidationMode::SmallBinaryFocused,
        commands: vec![
            "bash adl/tools/test_ci_path_policy.sh && bash adl/tools/test_select_validation_lanes.sh && bash adl/tools/test_validation_manager.sh && bash adl/tools/test_run_nessus_remote_validation.sh".to_string(),
        ],
    };
    run_finish_validation_rust(&repo, &plan).expect("combined ci-policy selector validation");

    unsafe {
        env::set_var("PATH", old_path);
    }
    match old_focused_log {
        Some(value) => unsafe { env::set_var("FOCUSED_LOG", value) },
        None => unsafe { env::remove_var("FOCUSED_LOG") },
    }

    assert!(
        !cargo_log.exists(),
        "combined ci-policy selector command should not invoke cargo"
    );

    let focused_calls = fs::read_to_string(&focused_log).expect("focused log");
    assert!(focused_calls.contains("ci-path-policy"));
    assert!(focused_calls.contains("select-validation-lanes"));
    assert!(focused_calls.contains("validation-manager"));
    assert!(focused_calls.contains("nessus-remote-runner"));
}

#[test]
fn finish_runner_executes_chained_local_polis_selector_command() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-chained-local-polis-selector-command");

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
    write_executable(
        &repo.join("adl/tools/polis_status_for_ssm.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' polis-status-ssm >> \"$FOCUSED_LOG\"\n",
    );
    write_executable(
        &repo.join("adl/tools/polis_status_for_ssm_qts.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' polis-status-ssm-qts >> \"$FOCUSED_LOG\"\n",
    );
    fs::write(
        repo.join("adl/tools/validate_polis_status_for_ssm_qts.py"),
        "#!/usr/bin/env python3\nimport os\nfrom pathlib import Path\nPath(os.environ['FOCUSED_LOG']).write_text(Path(os.environ['FOCUSED_LOG']).read_text() + 'validate-polis-ssm-qts\\n' if Path(os.environ['FOCUSED_LOG']).exists() else 'validate-polis-ssm-qts\\n')\n",
    )
    .expect("write validate polis qts");
    init_git_repo(&repo);

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let cargo_log = temp.join("cargo.log");
    let focused_log = temp.join("focused.log");
    write_executable(
        &bin_dir.join("cargo"),
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

    let plan = FinishValidationPlan {
        mode: FinishValidationMode::SmallBinaryFocused,
        commands: vec![
            "bash -n adl/tools/polis_status_for_ssm.sh && bash -n adl/tools/polis_status_for_ssm_qts.sh && python3 adl/tools/validate_polis_status_for_ssm_qts.py".to_string(),
        ],
    };
    run_finish_validation_rust(&repo, &plan).expect("chained local polis selector validation");

    unsafe {
        env::set_var("PATH", old_path);
    }
    match old_focused_log {
        Some(value) => unsafe { env::set_var("FOCUSED_LOG", value) },
        None => unsafe { env::remove_var("FOCUSED_LOG") },
    }

    assert!(
        !cargo_log.exists(),
        "chained local polis selector command should not invoke cargo"
    );

    let focused_calls = fs::read_to_string(&focused_log).expect("focused log");
    assert_eq!(focused_calls.trim(), "validate-polis-ssm-qts");
}

#[test]
fn finish_runner_rejects_chained_local_polis_selector_command_when_shell_syntax_is_invalid() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-chained-local-polis-selector-command-invalid-shell");

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
    fs::write(
        repo.join("adl/tools/polis_status_for_ssm.sh"),
        "#!/usr/bin/env bash\nif then\n",
    )
    .expect("write invalid polis ssm shell");
    write_executable(
        &repo.join("adl/tools/polis_status_for_ssm_qts.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 0\n",
    );
    fs::write(
        repo.join("adl/tools/validate_polis_status_for_ssm_qts.py"),
        "#!/usr/bin/env python3\nprint('should-not-run')\n",
    )
    .expect("write validate polis qts");
    init_git_repo(&repo);

    let plan = FinishValidationPlan {
        mode: FinishValidationMode::SmallBinaryFocused,
        commands: vec![
            "bash -n adl/tools/polis_status_for_ssm.sh && bash -n adl/tools/polis_status_for_ssm_qts.sh && python3 adl/tools/validate_polis_status_for_ssm_qts.py".to_string(),
        ],
    };
    let err = run_finish_validation_rust(&repo, &plan)
        .expect_err("invalid polis shell syntax should fail the chained selector command");
    assert!(err.to_string().contains("bash failed with status"));
}

#[test]
fn finish_runner_executes_prompt_template_workflow_integration_command() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-prompt-template-workflow-integration-command");

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
    write_executable(
        &repo.join("adl/tools/test_prompt_template_workflow_integration.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' prompt-template-workflow >> \"$FOCUSED_LOG\"\n",
    );
    init_git_repo(&repo);

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let cargo_log = temp.join("cargo.log");
    let focused_log = temp.join("focused.log");
    write_executable(
        &bin_dir.join("cargo"),
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

    let plan = FinishValidationPlan {
        mode: FinishValidationMode::SmallBinaryFocused,
        commands: vec!["bash adl/tools/test_prompt_template_workflow_integration.sh".to_string()],
    };
    run_finish_validation_rust(&repo, &plan)
        .expect("prompt-template workflow integration validation");

    unsafe {
        env::set_var("PATH", old_path);
    }
    match old_focused_log {
        Some(value) => unsafe { env::set_var("FOCUSED_LOG", value) },
        None => unsafe { env::remove_var("FOCUSED_LOG") },
    }

    assert!(
        !cargo_log.exists(),
        "prompt-template workflow command should not invoke cargo"
    );

    let focused_calls = fs::read_to_string(&focused_log).expect("focused log");
    assert!(focused_calls.contains("prompt-template-workflow"));
}

#[test]
fn finish_helper_paths_run_validation_inventory_validation() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-validation-inventory-validation");
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
    write_executable(
        &repo.join("adl/tools/test_validation_inventory.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' validation-inventory >> \"$FOCUSED_LOG\"\n",
    );
    init_git_repo(&repo);

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let cargo_log = temp.join("cargo.log");
    let focused_log = temp.join("focused.log");
    write_executable(
        &bin_dir.join("cargo"),
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
        "adl/tools/validation_inventory.py,adl/tools/validation_inventory.sh,adl/tools/test_validation_inventory.sh",
    )
    .expect("validation inventory plan");
    assert_eq!(plan.mode, FinishValidationMode::SmallBinaryFocused);
    run_finish_validation_rust(&repo, &plan).expect("validation inventory validation");

    unsafe {
        env::set_var("PATH", old_path);
    }
    match old_focused_log {
        Some(value) => unsafe { env::set_var("FOCUSED_LOG", value) },
        None => unsafe { env::remove_var("FOCUSED_LOG") },
    }

    assert!(
        !cargo_log.exists(),
        "validation inventory focused validation should not invoke cargo"
    );

    let focused_calls = fs::read_to_string(&focused_log).expect("focused log");
    assert!(focused_calls.contains("validation-inventory"));
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
fn finish_runtime_paths_run_module_focused_validation_and_runtime_lane() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-runtime-focused-validation");
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
    write_executable(
        &repo.join("adl/tools/run_owner_validation_lane.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> \"$FOCUSED_LOG\"\n",
    );
    init_git_repo(&repo);

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let cargo_log = temp.join("cargo.log");
    let focused_log = temp.join("focused.log");
    write_executable(
        &bin_dir.join("cargo"),
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
        "adl/src/agent_comms.rs,adl/src/agent_comms/carrier.inc,adl/src/agent_comms/tests.inc,adl/src/provider_adapter.rs,adl/src/provider_communication.rs,adl/src/resilience.rs",
    )
    .expect("runtime focused plan");
    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    run_finish_validation_rust(&repo, &plan).expect("runtime focused validation");

    unsafe {
        env::set_var("PATH", old_path);
    }
    match old_focused_log {
        Some(value) => unsafe { env::set_var("FOCUSED_LOG", value) },
        None => unsafe { env::remove_var("FOCUSED_LOG") },
    }

    let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
    assert!(cargo_calls.contains("test --manifest-path"));
    assert!(cargo_calls.contains("agent_comms --lib -- --nocapture"));
    assert!(cargo_calls.contains("--lib provider_adapter"));
    assert!(cargo_calls.contains("--lib provider_communication"));
    assert!(cargo_calls.contains("--lib resilience"));
    assert!(!cargo_calls.contains("clippy --manifest-path"));

    let focused_calls = fs::read_to_string(&focused_log).expect("focused log");
    assert!(focused_calls.contains("runtime --build"));
}

#[test]
fn finish_tokio_wave_paths_run_new_focused_validation_commands() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-tokio-wave-focused-validation");
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

    let plan = select_finish_validation_plan_for_finish(
        4178,
        ".",
        &["adl/Cargo.toml".to_string(), "adl/Cargo.lock".to_string()],
    )
    .expect("tokio wave focused plan");
    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    run_finish_validation_rust(&repo, &plan).expect("tokio wave focused validation");

    unsafe {
        env::set_var("PATH", old_path);
    }

    let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
    assert!(cargo_calls.contains("fmt --manifest-path"));
    assert!(cargo_calls.contains("test --manifest-path"));
    assert!(cargo_calls.contains("pr_cmd::github"));
    assert!(cargo_calls.contains("github_release_"));
    assert!(cargo_calls.contains("long_lived_agent"));
    assert!(!cargo_calls.contains("clippy --manifest-path"));
}

#[test]
fn finish_validation_profile_classifies_version_metadata_paths_without_issue_special_case() {
    let changed_paths = vec![
        "README.md".to_string(),
        "adl/Cargo.toml".to_string(),
        "adl/Cargo.lock".to_string(),
    ];
    let requested_paths = changed_paths.join(",");

    let plan = select_finish_validation_plan_for_finish(5000, &requested_paths, &changed_paths)
        .expect("version metadata profile plan");

    assert_eq!(plan.mode, FinishValidationMode::SmallBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/check_no_tracked_adl_issue_record_residue.sh".to_string()));
    assert!(plan.commands.contains(&"git diff --check".to_string()));
    assert!(plan.commands.contains(
        &"cargo metadata --manifest-path adl/Cargo.toml --no-deps --format-version 1".to_string()
    ));
    assert!(plan.commands.contains(
        &"cargo metadata --manifest-path adl/Cargo.toml --locked --no-deps --format-version 1"
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

    let plan = select_finish_validation_plan_for_finish(
        5000,
        ".",
        &["adl/Cargo.toml".to_string(), "adl/Cargo.lock".to_string()],
    )
    .expect("manifest-only paths should be covered by validation-manager routing");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .iter()
        .any(|command| command.contains("run_pr_fast_test_lane.sh")));
}

#[test]
fn finish_validation_runner_executes_version_metadata_commands() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-version-metadata-validation");
    let repo = temp.join("repo");
    fs::create_dir_all(repo.join("adl/tools")).expect("adl tools dir");
    fs::write(
        repo.join("adl/Cargo.toml"),
        "[package]\nname='adl'\nversion='0.91.6'\n",
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

    let changed_paths = vec![
        "README.md".to_string(),
        "adl/Cargo.toml".to_string(),
        "adl/Cargo.lock".to_string(),
    ];
    let requested_paths = changed_paths.join(",");
    let plan = select_finish_validation_plan_for_finish(5000, &requested_paths, &changed_paths)
        .expect("version metadata profile plan");
    run_finish_validation_rust(&repo, &plan).expect("version metadata validation");

    unsafe {
        env::set_var("PATH", old_path);
    }

    let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
    assert!(cargo_calls.contains("metadata --manifest-path"));
    assert!(cargo_calls.contains("--no-deps --format-version 1"));
    assert!(cargo_calls.contains("--locked --no-deps --format-version 1"));
    assert!(!cargo_calls.contains("test --manifest-path"));
    assert!(!cargo_calls.contains("clippy --manifest-path"));
}

#[test]
fn finish_validation_profile_classifies_native_gws_runtime_slice() {
    let plan = select_finish_validation_plan_for_finish(
        4406,
        ".",
        &[
            "adl/Cargo.toml".to_string(),
            "adl/Cargo.lock".to_string(),
            "adl/src/lib.rs".to_string(),
            "adl/src/adl_gws_native.rs".to_string(),
            "adl/src/adl_gws_drive_sync.rs".to_string(),
            "adl/src/adl_gws_context_mirror.rs".to_string(),
            "adl/src/bin/demo_adl_gws_native_drive_sync.rs".to_string(),
            "adl/src/bin/demo_adl_gws_context_mirror.rs".to_string(),
        ],
    )
    .expect("native gws runtime plan");

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
        .contains(&"cargo test --manifest-path adl/Cargo.toml adl_gws -- --nocapture".to_string()));
    assert!(plan.commands.contains(
        &"cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish cli::pr_cmd::tests::finish::arg_render::finish_validation_profile_classifies_native_gws_runtime_slice -- --exact --nocapture".to_string()
    ));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh runtime --build".to_string()));
}

#[test]
fn finish_validation_profile_classifies_pr_cmd_prompt_and_versioned_bootstrap_paths() {
    let plan = select_finish_validation_plan(
        "adl/src/cli/pr_cmd_prompt.rs,adl/src/cli/tests/pr_cmd_inline/versioned_bootstrap.rs",
    )
    .expect("pr cmd prompt and versioned bootstrap plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(
        plan.commands
            .iter()
            .any(|command| command.contains("cargo fmt --manifest-path")),
        "larger-binary focused plan should require cargo fmt"
    );
    assert!(
        plan.commands.iter().any(|command| {
            command.contains("cargo test --manifest-path adl/Cargo.toml --bin adl cli::pr_cmd")
        }),
        "larger-binary focused plan should include pr_cmd lifecycle validation"
    );
}

#[test]
fn finish_validation_profile_classifies_control_plane_path() {
    let plan = select_finish_validation_plan("adl/src/control_plane.rs")
        .expect("control_plane larger-binary plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(
        plan.commands
            .iter()
            .any(|command| command.contains("cargo fmt --manifest-path")),
        "control-plane larger-binary plan should require cargo fmt"
    );
    assert!(
        plan.commands
            .iter()
            .any(|command| command.contains("cargo test --manifest-path adl/Cargo.toml --bin adl")),
        "control-plane larger-binary plan should include owner-binary validation"
    );
}

#[test]
fn finish_validation_profile_classifies_session_ledger_paths() {
    let plan = select_finish_validation_plan(
        "adl/src/session_ledger.rs,adl/src/cli/session_cmd.rs,adl/src/cli/tests.rs",
    )
    .expect("session ledger larger-binary plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(
        plan.commands
            .iter()
            .any(|command| command.contains("cargo fmt --manifest-path")),
        "session ledger plan should require cargo fmt"
    );
    assert!(
        plan.commands
            .iter()
            .any(|command| command.contains("cargo test --manifest-path adl/Cargo.toml --bin adl")),
        "session ledger plan should include owner-binary validation"
    );
}

#[test]
fn finish_validation_profile_classifies_prompt_template_and_structured_prompt_paths() {
    let plan = select_finish_validation_plan(
        "adl/src/cli/tooling_cmd/common.rs,adl/src/cli/tooling_cmd/prompt_template.rs,adl/src/cli/tooling_cmd/structured_prompt.rs,adl/src/cli/tooling_cmd/tests/prompt_template.rs,adl/src/cli/tooling_cmd/tests/structured_prompt.rs,adl/src/cli/tooling_cmd/tests/support.rs",
    )
    .expect("prompt-template focused plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(
        plan.commands
            .iter()
            .any(|command| command.contains("cargo fmt --manifest-path")),
        "larger-binary focused plan should require cargo fmt"
    );
    assert!(
        plan.commands.iter().any(|command| {
            command.contains(
                "cargo test --manifest-path adl/Cargo.toml --bin adl prompt_template_ -- --nocapture",
            )
        }),
        "prompt-template focused plan should include prompt_template_ validation"
    );
    assert!(
        plan.commands.iter().any(|command| {
            command.contains(
                "cargo test --manifest-path adl/Cargo.toml --bin adl structured_prompt_ -- --nocapture",
            )
        }),
        "prompt-template focused plan should include structured_prompt_ validation"
    );
}

#[test]
fn finish_validation_profile_classifies_tooling_common_path_standalone() {
    let plan = select_finish_validation_plan("adl/src/cli/tooling_cmd/common.rs")
        .expect("tooling common standalone plan");

    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(
        plan.commands.iter().any(|command| {
            command.contains(
                "cargo test --manifest-path adl/Cargo.toml --bin adl prompt_template_ -- --nocapture",
            )
        }),
        "tooling common standalone plan should include prompt_template_ validation"
    );
    assert!(
        plan.commands.iter().any(|command| {
            command.contains(
                "cargo test --manifest-path adl/Cargo.toml --bin adl structured_prompt_ -- --nocapture",
            )
        }),
        "tooling common standalone plan should include structured_prompt_ validation"
    );
}

#[test]
fn finish_scheduler_paths_run_scheduler_economics_focused_validation() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-scheduler-focused-validation");
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
        "adl/src/scheduler.rs,adl/tests/fixtures/scheduler/economics_inputs_v1.json,docs/milestones/v0.91.6/review/scheduler/SCHEDULER_ECONOMICS_INPUTS_4106.md",
    )
    .expect("scheduler focused plan");
    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    run_finish_validation_rust(&repo, &plan).expect("scheduler focused validation");

    unsafe {
        env::set_var("PATH", old_path);
    }

    let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
    assert!(cargo_calls.contains("fmt --manifest-path"));
    assert!(cargo_calls.contains("test --manifest-path"));
    assert!(cargo_calls.contains("--lib scheduler_economics"));
    assert!(!cargo_calls.contains("clippy --manifest-path"));
}

#[test]
fn finish_helper_paths_run_larger_binary_focused_validation() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-finish-larger-binary-validation");
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
    let bin_dir = repo.join("fake-bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let cargo_log = repo.join("cargo.log");
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

    let plan = select_finish_validation_plan("adl/src/cli/tooling_cmd/github_release.rs")
        .expect("larger binary focused plan");
    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    run_finish_validation_rust(&repo, &plan).expect("larger binary focused validation");

    unsafe {
        env::set_var("PATH", old_path);
    }

    let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
    assert!(cargo_calls.contains("fmt --manifest-path"));
    assert!(cargo_calls.contains("--bin adl github_release_"));
}

#[test]
fn finish_owner_binary_paths_run_owner_binary_focused_validation() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-owner-binary-focused-validation");
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

    let plan = select_finish_validation_plan("adl/src/lib.rs").expect("owner binary plan");
    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    run_finish_validation_rust(&repo, &plan).expect("owner binary focused validation");

    unsafe {
        env::set_var("PATH", old_path);
    }

    let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
    assert!(cargo_calls.contains("fmt --manifest-path"));
    assert!(cargo_calls.contains("test --manifest-path"));
    assert!(cargo_calls.contains("--bin adl"));
    assert!(!cargo_calls.contains("github_token"));
    assert!(!cargo_calls.contains(" cli::pr_cmd"));
    assert!(!cargo_calls.contains("clippy --manifest-path"));
}

#[test]
fn finish_path_ownership_registry_classifies_owner_binary_slice() {
    let plan = select_finish_validation_plan("adl/src/session_ledger.rs")
        .expect("owner binary registry plan");
    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan.commands.iter().any(|cmd| cmd.contains("--bin adl")));
}

#[test]
fn finish_path_ownership_registry_classifies_csdlc_owner_lane_contract() {
    let plan = select_finish_validation_plan("adl/tools/test_pr_run_ambiguity_policy.sh")
        .expect("csdlc owner lane registry plan");
    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .iter()
        .any(|cmd| cmd == "bash adl/tools/run_owner_validation_lane.sh csdlc"));
}

#[test]
fn finish_path_ownership_registry_preserves_shared_owner_lane_overlap() {
    let plan = select_finish_validation_plan("adl/tools/run_owner_validation_lane.sh")
        .expect("shared owner lane registry plan");
    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    assert!(plan
        .commands
        .contains(&"bash adl/tools/test_owner_validation_lane.sh".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh csdlc".to_string()));
    assert!(plan
        .commands
        .contains(&"bash adl/tools/run_owner_validation_lane.sh all --build".to_string()));
}

#[test]
fn finish_prompt_template_paths_run_prompt_template_focused_validation() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-prompt-template-focused-validation");
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
        "adl/src/cli/tooling_cmd/prompt_template.rs,adl/src/cli/tooling_cmd/structured_prompt.rs,adl/src/cli/tooling_cmd/tests/prompt_template.rs,adl/src/cli/tooling_cmd/tests/structured_prompt.rs,adl/src/cli/tooling_cmd/tests/support.rs",
    )
    .expect("prompt template plan");
    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    run_finish_validation_rust(&repo, &plan).expect("prompt template focused validation");

    unsafe {
        env::set_var("PATH", old_path);
    }

    let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
    assert!(cargo_calls.contains("fmt --manifest-path"));
    assert!(cargo_calls.contains("--bin adl prompt_template_ -- --nocapture"));
    assert!(cargo_calls.contains("--bin adl structured_prompt_ -- --nocapture"));
    assert!(!cargo_calls.contains("clippy --manifest-path"));
}

#[test]
fn finish_prompt_template_template_only_paths_run_prompt_template_focused_validation() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-prompt-template-template-only-validation");
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
        "docs/templates/prompts/current.json,docs/templates/prompts/1.0.2/spp.md,docs/templates/prompts/1.0.2/schemas/sor.structure.json",
    )
    .expect("prompt template template-only plan");
    assert_eq!(plan.mode, FinishValidationMode::LargerBinaryFocused);
    run_finish_validation_rust(&repo, &plan).expect("prompt template template-only validation");

    unsafe {
        env::set_var("PATH", old_path);
    }

    let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
    assert!(cargo_calls.contains("fmt --manifest-path"));
    assert!(cargo_calls.contains("--bin adl prompt_template_ -- --nocapture"));
    assert!(cargo_calls.contains("--bin adl structured_prompt_ -- --nocapture"));
}

#[test]
fn finish_issue_small_binary_paths_run_issue_focused_validation() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-issue-small-binary-focused-validation");
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
    write_executable(
        &repo.join("adl/tools/test_pr_small_binary_delegation.sh"),
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

    let plan = select_finish_validation_plan_for_finish(
        4216,
        ".",
        &[
            "adl/Cargo.toml".to_string(),
            "adl/src/bin/adl_issue.rs".to_string(),
            "adl/src/cli/pr_cmd/finish_support.rs".to_string(),
            "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs".to_string(),
            "adl/tools/pr.sh".to_string(),
            "adl/tools/test_ci_path_policy.sh".to_string(),
            "adl/tools/test_pr_small_binary_delegation.sh".to_string(),
        ],
    )
    .expect("issue small binary plan");
    assert_eq!(plan.mode, FinishValidationMode::SmallBinaryFocused);
    run_finish_validation_rust(&repo, &plan).expect("issue small binary focused validation");

    unsafe {
        env::set_var("PATH", old_path);
    }

    let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
    assert!(cargo_calls.contains("fmt --manifest-path"));
    assert!(cargo_calls.contains("test --manifest-path"));
    assert!(cargo_calls.contains("--bin adl-issue"));
    assert!(cargo_calls.contains("tests::adl_issue_forwards_args_to_dispatch"));
    assert!(!cargo_calls.contains("clippy --manifest-path"));
}

#[test]
fn finish_closing_linkage_paths_run_issue_focused_validation() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-closing-linkage-focused-validation");
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
    write_executable(
        &repo.join("adl/tools/test_pr_closing_linkage.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 0\n",
    );
    write_executable(
        &repo.join("adl/tools/test_pr_small_binary_delegation.sh"),
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

    let plan = select_finish_validation_plan_for_finish(
        4286,
        ".",
        &[
            "adl/Cargo.toml".to_string(),
            "adl/src/bin/adl_pr_closing_linkage.rs".to_string(),
            "adl/src/cli/pr_cmd.rs".to_string(),
            "adl/src/cli/pr_cmd/github.rs".to_string(),
            "adl/src/cli/pr_cmd/github/tests.rs".to_string(),
            "adl/src/cli/pr_cmd/github/tests/closing_linkage.rs".to_string(),
            "adl/src/cli/pr_cmd_args.rs".to_string(),
            "adl/tools/check_pr_closing_linkage.sh".to_string(),
            "adl/tools/pr.sh".to_string(),
            "adl/tools/run_owner_validation_lane.sh".to_string(),
            "adl/tools/test_pr_closing_linkage.sh".to_string(),
            "adl/tools/test_pr_small_binary_delegation.sh".to_string(),
            "docs/milestones/v0.91.6/review/github_projection/GITHUB_CSDLC_PROJECTION_MAP_4047.md"
                .to_string(),
            "adl/src/cli/pr_cmd/finish_support.rs".to_string(),
            "adl/src/cli/tests/pr_cmd_inline/finish/arg_render.rs".to_string(),
            ".adl/v0.91.6/tasks/issue-4286__typed-pr-closing-linkage-rust-pvf/spp.md".to_string(),
            ".adl/v0.91.6/tasks/issue-4286__typed-pr-closing-linkage-rust-pvf/srp.md".to_string(),
            ".adl/v0.91.6/tasks/issue-4286__typed-pr-closing-linkage-rust-pvf/sor.md".to_string(),
        ],
    )
    .expect("closing linkage small binary plan");
    assert_eq!(plan.mode, FinishValidationMode::SmallBinaryFocused);
    run_finish_validation_rust(&repo, &plan).expect("closing linkage focused validation");

    unsafe {
        env::set_var("PATH", old_path);
    }

    let cargo_calls = fs::read_to_string(&cargo_log).expect("cargo log");
    assert!(cargo_calls.contains("fmt --manifest-path"));
    assert!(cargo_calls.contains("test --manifest-path"));
    assert!(cargo_calls.contains("--bin adl-pr-closing-linkage"));
    assert!(cargo_calls.contains("closing_linkage"));
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
    let _github_env = force_gh_cli_transport_env();
    let temp = unique_temp_dir("adl-pr-finish-default-lane");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_finish_bootstrap_support_files(&repo);
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
        .args(["add", "."])
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
    let _fixture = GithubCliFixtureGuard::set(&gh_path);
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
    let _fixture = GithubCliFixtureGuard::set(&gh_path);

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

#[test]
fn real_pr_finish_restages_tracked_output_truth_written_during_validation() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-restage-output-truth");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_finish_bootstrap_support_files(&repo);
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
    fs::create_dir_all(repo.join("docs")).expect("docs dir");
    fs::create_dir_all(repo.join("adl/tools")).expect("tools dir");
    fs::write(repo.join("docs/notes.md"), "initial notes\n").expect("write docs");
    write_executable(
        &repo.join("adl/tools/validation_manager.py"),
        r#"#!/usr/bin/env python3
import json

print(json.dumps({
    "selected_profile": "docs_diff_check_profile",
    "status": "ready_to_run",
    "pr_publication_sufficient": True,
    "run": [
        {
            "lane_id": "docs_diff_check",
            "command": "git diff --check",
            "reason": "docs_only_surface_requires_diff_hygiene",
        }
    ],
    "not_run": [],
    "deferred": [
        {
            "surface": "ci_integration",
            "reason": "deferred to GitHub checks for merge-context validation",
        }
    ],
    "behavior_surfaces": ["docs_only"],
    "validation_dag": [],
    "estimated_cost": "low",
    "escalation": {"required": False, "reasons": []},
    "selector_plan": [],
}))
"#,
    );

    let issue_ref = IssueRef::new(
        1162,
        "v0.86".to_string(),
        "restage-output-truth".to_string(),
    )
    .expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = repo.join("docs/output-truth.md");
    let plan = issue_ref.task_bundle_plan_path(&repo);
    let review_policy = issue_ref.task_bundle_review_policy_path(&repo);
    write_authored_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.86][tools] Restage finish output truth",
    );
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Restage finish output truth",
        "codex/1162-restage-output-truth",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_spp(
        &plan,
        &issue_ref,
        "[v0.86][tools] Restage finish output truth",
        "codex/1162-restage-output-truth",
        &repo,
    );
    write_authored_srp(
        &review_policy,
        &issue_ref,
        "[v0.86][tools] Restage finish output truth",
        "codex/1162-restage-output-truth",
        &repo,
    );
    fs::write(
        &output,
        r#"# issue-1162

Task ID: issue-1162
Run ID: issue-1162
Version: v0.86
Title: Restage finish output truth
Branch: codex/1162-restage-output-truth
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

## PVF Lane Truth
- Initial PVF lane: `docs_only`
- Planned PVF lane: `docs_only`
- Final PVF lane: `docs_only`
- Lane change reason: `no_lane_change`

## Issue Metrics Truth
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Artifacts produced
- docs/notes.md

## Actions taken
- updated docs

## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
  - `docs/notes.md`
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
- docs/notes.md

## Decisions / Deviations
- none

## Follow-ups / Deferred work
- none
"#,
    )
    .expect("write output");

    assert!(Command::new("git")
        .args(["add", ".gitignore", "docs/notes.md", "docs/output-truth.md"])
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
        .args(["checkout", "-q", "-b", "codex/1162-restage-output-truth"])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    fs::write(repo.join("docs/notes.md"), "updated notes\n").expect("update docs");
    assert!(Command::new("git")
        .args(["add", "docs/notes.md"])
        .current_dir(&repo)
        .status()
        .expect("git add branch change")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "pre-finish docs change"])
        .current_dir(&repo)
        .status()
        .expect("git commit branch change")
        .success());
    assert!(
        !has_uncommitted_changes(&repo).expect("clean before finish"),
        "regression requires finish to sample a clean worktree before it writes validation truth"
    );

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
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr create' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1162\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1162\\n'\n  else\n    printf 'Closes #1162\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
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
        "1162".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Restage finish output truth".to_string(),
        "--paths".to_string(),
        "docs/notes.md".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &input),
        "--output".to_string(),
        path_relative_to_repo(&repo, &output),
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

    let head_output = run_capture(
        "git",
        &[
            "-C",
            path_str(&repo).expect("repo"),
            "show",
            "HEAD:docs/output-truth.md",
        ],
    )
    .expect("head output");
    assert!(
        head_output.contains("git diff --check"),
        "commit should include finish-written docs-only validation evidence"
    );
    let head_notes = run_capture(
        "git",
        &[
            "-C",
            path_str(&repo).expect("repo"),
            "show",
            "HEAD:docs/notes.md",
        ],
    )
    .expect("head notes");
    assert!(head_notes.contains("updated notes"));
}

#[test]
fn real_pr_finish_updates_existing_pr_marks_ready_and_keeps_non_closing_commit_title() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-existing-pr-ready-no-close");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_finish_bootstrap_support_files(&repo);
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
    fs::create_dir_all(repo.join("adl/src")).expect("adl src dir");

    let issue_ref = IssueRef::new(
        1174,
        "v0.86".to_string(),
        "existing-pr-ready-no-close".to_string(),
    )
    .expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    let plan = issue_ref.task_bundle_plan_path(&repo);
    let review_policy = issue_ref.task_bundle_review_policy_path(&repo);
    write_authored_issue_prompt(
        &repo,
        &issue_ref,
        "[v0.86][tools] Existing PR ready no close",
    );
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Existing PR ready no close",
        "codex/1174-existing-pr-ready-no-close",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_spp(
        &plan,
        &issue_ref,
        "[v0.86][tools] Existing PR ready no close",
        "codex/1174-existing-pr-ready-no-close",
        &repo,
    );
    write_authored_srp(
        &review_policy,
        &issue_ref,
        "[v0.86][tools] Existing PR ready no close",
        "codex/1174-existing-pr-ready-no-close",
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1174-existing-pr-ready-no-close");
    fs::write(
        repo.join("adl/src/lib.rs"),
        "pub fn placeholder() {}\npub fn existing_pr_ready() {}\n",
    )
    .expect("write change");

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
            "codex/1174-existing-pr-ready-no-close"
        ])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    fs::write(
        repo.join("adl/src/lib.rs"),
        "pub fn placeholder() {}\npub fn existing_pr_ready() { println!(\"ready\"); }\n",
    )
    .expect("update change");

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
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1174\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'baseRefName'; then\n    printf 'main\\n'\n  elif printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '\\n'\n  else\n    printf 'Non-closing lifecycle PR for issue #1174\\n'\n  fi\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr edit' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr ready' ]; then\n  exit 0\nfi\nexit 1\n",
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
        "1174".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Existing PR ready no close".to_string(),
        "--paths".to_string(),
        "adl".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &input),
        "--output".to_string(),
        path_relative_to_repo(&repo, &output),
        "--no-checks".to_string(),
        "--no-open".to_string(),
        "--no-close".to_string(),
        "--ready".to_string(),
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

    result.expect("real_pr_finish existing PR success");

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
    assert_eq!(
        head_subject.trim(),
        "[v0.86][tools] Existing PR ready no close"
    );

    let gh_log = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_log.contains("pr list"));
    assert!(gh_log.contains("pr edit"));
    assert!(!gh_log.contains("pr create"));
    assert!(gh_log.contains("pr ready"));

    let janitor_log = fs::read_to_string(&janitor_log).expect("janitor log");
    assert!(janitor_log.contains("--issue 1174"));
    assert!(janitor_log.contains("ready"));

    let closeout_log = fs::read_to_string(&closeout_log).expect("closeout log");
    assert!(closeout_log.contains("--issue 1174"));
}

#[test]
fn real_pr_finish_rejects_missing_output_card_before_publication() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-missing-output");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_finish_bootstrap_support_files(&repo);
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
    fs::create_dir_all(repo.join("adl/src")).expect("adl src dir");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write file");

    let issue_ref =
        IssueRef::new(1182, "v0.86".to_string(), "missing-output".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let plan = issue_ref.task_bundle_plan_path(&repo);
    let review_policy = issue_ref.task_bundle_review_policy_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Missing output");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Missing output",
        "codex/1182-missing-output",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_spp(
        &plan,
        &issue_ref,
        "[v0.86][tools] Missing output",
        "codex/1182-missing-output",
        &repo,
    );
    write_authored_srp(
        &review_policy,
        &issue_ref,
        "[v0.86][tools] Missing output",
        "codex/1182-missing-output",
        &repo,
    );

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
        .args(["checkout", "-q", "-b", "codex/1182-missing-output"])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let result = real_pr_finish(&[
        "1182".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Missing output".to_string(),
        "--paths".to_string(),
        "adl".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &input),
        "--output".to_string(),
        path_relative_to_repo(&repo, &issue_ref.task_bundle_output_path(&repo)),
        "--no-checks".to_string(),
        "--no-open".to_string(),
    ]);
    env::set_current_dir(prev_dir).expect("restore cwd");

    let err = result.expect_err("missing output should fail");
    assert!(err.to_string().contains("missing output card"));
}

#[test]
fn real_pr_finish_rejects_empty_output_card_before_publication() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-empty-output");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_finish_bootstrap_support_files(&repo);
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
    fs::create_dir_all(repo.join("adl/src")).expect("adl src dir");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write file");

    let issue_ref =
        IssueRef::new(1184, "v0.86".to_string(), "empty-output".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    let plan = issue_ref.task_bundle_plan_path(&repo);
    let review_policy = issue_ref.task_bundle_review_policy_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Empty output");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Empty output",
        "codex/1184-empty-output",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_spp(
        &plan,
        &issue_ref,
        "[v0.86][tools] Empty output",
        "codex/1184-empty-output",
        &repo,
    );
    write_authored_srp(
        &review_policy,
        &issue_ref,
        "[v0.86][tools] Empty output",
        "codex/1184-empty-output",
        &repo,
    );
    fs::write(&output, "").expect("write empty output");

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
        .args(["checkout", "-q", "-b", "codex/1184-empty-output"])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let result = real_pr_finish(&[
        "1184".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Empty output".to_string(),
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

    let err = result.expect_err("empty output should fail");
    assert!(err.to_string().contains("output card is empty"));
}

#[test]
fn real_pr_finish_rejects_branch_name_mismatch() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-branch-mismatch");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_finish_bootstrap_support_files(&repo);
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
    fs::create_dir_all(repo.join("adl/src")).expect("adl src dir");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write file");

    let issue_ref =
        IssueRef::new(1185, "v0.86".to_string(), "branch-mismatch".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    let plan = issue_ref.task_bundle_plan_path(&repo);
    let review_policy = issue_ref.task_bundle_review_policy_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Branch mismatch");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Branch mismatch",
        "codex/1185-branch-mismatch",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_spp(
        &plan,
        &issue_ref,
        "[v0.86][tools] Branch mismatch",
        "codex/1185-branch-mismatch",
        &repo,
    );
    write_authored_srp(
        &review_policy,
        &issue_ref,
        "[v0.86][tools] Branch mismatch",
        "codex/1185-branch-mismatch",
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1185-branch-mismatch");

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
        .args(["checkout", "-q", "-b", "codex/not-the-right-branch"])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let result = real_pr_finish(&[
        "1185".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Branch mismatch".to_string(),
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

    let err = result.expect_err("branch mismatch should fail");
    assert!(err
        .to_string()
        .contains("does not look like it matches issue #1185"));
}

#[test]
fn real_pr_finish_rejects_closed_issue_with_stale_canonical_truth() {
    let _guard = env_lock();
    let _github_env = force_gh_cli_transport_env();
    let temp = unique_temp_dir("adl-pr-finish-closed-stale-truth");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_finish_bootstrap_support_files(&repo);
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
    fs::create_dir_all(repo.join("docs")).expect("docs dir");
    fs::write(repo.join("docs/notes.md"), "initial notes\n").expect("write docs");

    let issue_ref =
        IssueRef::new(1186, "v0.86".to_string(), "closed-stale-truth".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    let plan = issue_ref.task_bundle_plan_path(&repo);
    let review_policy = issue_ref.task_bundle_review_policy_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Closed stale truth");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Closed stale truth",
        "codex/1186-closed-stale-truth",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_spp(
        &plan,
        &issue_ref,
        "[v0.86][tools] Closed stale truth",
        "codex/1186-closed-stale-truth",
        &repo,
    );
    write_authored_srp(
        &review_policy,
        &issue_ref,
        "[v0.86][tools] Closed stale truth",
        "codex/1186-closed-stale-truth",
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1186-closed-stale-truth");

    assert!(Command::new("git")
        .args(["add", "."])
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
        .args(["checkout", "-q", "-b", "codex/1186-closed-stale-truth"])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    fs::write(repo.join("docs/notes.md"), "updated notes\n").expect("update docs");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'issue view' ]; then\n  printf '{{\"state\":\"CLOSED\",\"stateReason\":\"COMPLETED\"}}\\n'\n  exit 0\nfi\nexit 1\n",
            gh_log.display()
        ),
    );
    let _fixture = GithubCliFixtureGuard::set(&gh_path);

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");
    let result = real_pr_finish(&[
        "1186".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Closed stale truth".to_string(),
        "--paths".to_string(),
        "docs/notes.md".to_string(),
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

    let err = result.expect_err("closed stale truth should fail");
    assert!(err
        .to_string()
        .contains("closed issue #1186 has stale canonical sor truth"));
}

#[test]
fn real_pr_finish_opener_failure_is_nonblocking_when_no_open_is_false() {
    let _guard = env_lock();
    let _github_env = force_gh_cli_transport_env();
    let temp = unique_temp_dir("adl-pr-finish-open-failure");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_finish_bootstrap_support_files(&repo);
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
    fs::create_dir_all(repo.join("adl/src")).expect("adl src dir");
    fs::write(
        repo.join("adl/src/lib.rs"),
        "pub fn placeholder() {}\npub fn opener_failure_path() {}\n",
    )
    .expect("write change");

    let issue_ref =
        IssueRef::new(1187, "v0.86".to_string(), "open-failure".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    let plan = issue_ref.task_bundle_plan_path(&repo);
    let review_policy = issue_ref.task_bundle_review_policy_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Open failure");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Open failure",
        "codex/1187-open-failure",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_spp(
        &plan,
        &issue_ref,
        "[v0.86][tools] Open failure",
        "codex/1187-open-failure",
        &repo,
    );
    write_authored_srp(
        &review_policy,
        &issue_ref,
        "[v0.86][tools] Open failure",
        "codex/1187-open-failure",
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1187-open-failure");

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
        .args(["checkout", "-q", "-b", "codex/1187-open-failure"])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    fs::write(
        repo.join("adl/src/lib.rs"),
        "pub fn placeholder() {}\npub fn opener_failure_path() { println!(\"open\"); }\n",
    )
    .expect("update change");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let gh_path = bin_dir.join("gh");
    let open_path = bin_dir.join("open");
    write_executable(
        &gh_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr create' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1187\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1187\\n'\n  else\n    printf 'Closes #1187\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
            gh_log.display()
        ),
    );
    let _fixture = GithubCliFixtureGuard::set(&gh_path);
    write_executable(
        &open_path,
        "#!/usr/bin/env bash\nset -euo pipefail\necho 'synthetic open failure' >&2\nexit 42\n",
    );
    let _fixture = GithubCliFixtureGuard::set(&gh_path);

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");
    let result = real_pr_finish(&[
        "1187".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Open failure".to_string(),
        "--paths".to_string(),
        "adl".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &input),
        "--output".to_string(),
        path_relative_to_repo(&repo, &output),
        "--no-checks".to_string(),
    ]);
    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    result.expect("open failure should stay non-blocking");
}

#[test]
fn real_pr_finish_merge_mode_rejects_no_checks() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-finish-merge-mode");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(&repo).expect("repo dir");
    copy_finish_bootstrap_support_files(&repo);
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
    fs::create_dir_all(repo.join("docs")).expect("docs dir");
    fs::create_dir_all(repo.join("adl/tools")).expect("tools dir");
    fs::write(repo.join("docs/notes.md"), "initial notes\n").expect("write docs");
    write_executable(
        &repo.join("adl/tools/validation_manager.py"),
        r#"#!/usr/bin/env python3
import json

print(json.dumps({
    "selected_profile": "docs_diff_check_profile",
    "status": "ready_to_run",
    "pr_publication_sufficient": True,
    "run": [
        {
            "lane_id": "docs_diff_check",
            "command": "git diff --check",
            "reason": "docs_only_surface_requires_diff_hygiene",
        }
    ],
    "not_run": [],
    "deferred": [
        {
            "surface": "ci_integration",
            "reason": "deferred to GitHub checks for merge-context validation",
        }
    ],
    "behavior_surfaces": ["docs_only"],
    "validation_dag": [],
    "estimated_cost": "low",
    "escalation": {"required": False, "reasons": []},
    "selector_plan": [],
}))
"#,
    );

    let issue_ref =
        IssueRef::new(1181, "v0.86".to_string(), "merge-mode".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    let plan = issue_ref.task_bundle_plan_path(&repo);
    let review_policy = issue_ref.task_bundle_review_policy_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Merge mode finish");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Merge mode finish",
        "codex/1181-merge-mode",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_spp(
        &plan,
        &issue_ref,
        "[v0.86][tools] Merge mode finish",
        "codex/1181-merge-mode",
        &repo,
    );
    write_authored_srp(
        &review_policy,
        &issue_ref,
        "[v0.86][tools] Merge mode finish",
        "codex/1181-merge-mode",
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1181-merge-mode");

    assert!(Command::new("git")
        .args(["add", ".gitignore", "docs/notes.md"])
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
        .args(["checkout", "-q", "-b", "codex/1181-merge-mode"])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    fs::write(repo.join("docs/notes.md"), "updated merge notes\n").expect("update docs");

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let issue_state_file = temp.join("issue_state.txt");
    fs::write(&issue_state_file, "open\n").expect("seed issue state");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr create' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1181\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1181\\n'\n  else\n    printf 'Closes #1181\\n'\n  fi\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr ready' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr merge' ]; then\n  printf 'closed\\n' > '{}'\n  exit 0\nfi\nif [ \"$1 $2\" = 'issue view' ]; then\n  if [ \"$(cat '{}')\" = 'closed' ]; then\n    printf '{{\"state\":\"CLOSED\",\"stateReason\":\"COMPLETED\"}}\\n'\n  else\n    printf '{{\"state\":\"OPEN\",\"stateReason\":null}}\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
            gh_log.display(),
            issue_state_file.display(),
            issue_state_file.display()
        ),
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr_finish(&[
        "1181".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Merge mode finish".to_string(),
        "--paths".to_string(),
        "docs/notes.md".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &input),
        "--output".to_string(),
        path_relative_to_repo(&repo, &output),
        "--no-open".to_string(),
        "--merge".to_string(),
        "--no-checks".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    let err = result.expect_err("merge without checks should fail");
    assert!(err.to_string().contains("--merge requires checks"));
}
