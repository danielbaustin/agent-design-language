use super::*;
use crate::cli::pr_cmd_cards::{validate_bootstrap_output_card, write_output_card};
use crate::cli::pr_cmd_prompt::{infer_wp_from_title, render_generated_issue_prompt};
use crate::cli::pr_cmd_validate::bootstrap_stub_reason;
use crate::cli::tests::env_lock as cli_env_lock;
use adl::control_plane::{card_input_path, card_stp_path};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir(label: &str) -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("{label}-{now}-{}", std::process::id()));
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    let guard = cli_env_lock();
    unsafe {
        env::set_var("ADL_PR_JANITOR_DISABLE", "1");
        env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "1");
    }
    guard
}

#[test]
fn env_lock_disables_post_merge_closeout_by_default() {
    let _guard = env_lock();
    assert_eq!(
        env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok().as_deref(),
        Some("1")
    );
}

fn write_executable(path: &Path, content: &str) {
    fs::write(path, content).expect("write executable");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path).expect("metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms).expect("chmod");
    }
}

fn init_git_repo(dir: &Path) {
    assert!(Command::new("git")
        .arg("init")
        .arg("-q")
        .current_dir(dir)
        .status()
        .expect("git init")
        .success());
    assert!(Command::new("git")
        .args([
            "remote",
            "add",
            "origin",
            "https://github.com/owner/repo.git"
        ])
        .current_dir(dir)
        .status()
        .expect("git remote add")
        .success());
}

fn copy_bootstrap_support_files(repo: &Path) {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf();
    env::set_var("ADL_TOOLING_MANIFEST_ROOT", &workspace_root);
    let tools_dir = repo.join("adl/tools");
    let templates_dir = repo.join("adl/templates/cards");
    let schemas_dir = repo.join("adl/schemas");
    fs::create_dir_all(&tools_dir).expect("tools dir");
    fs::create_dir_all(&templates_dir).expect("templates dir");
    fs::create_dir_all(&schemas_dir).expect("schemas dir");

    let files = [
        (
            workspace_root.join("adl/tools/card_paths.sh"),
            tools_dir.join("card_paths.sh"),
        ),
        (
            workspace_root.join("adl/tools/validate_structured_prompt.sh"),
            tools_dir.join("validate_structured_prompt.sh"),
        ),
        (
            workspace_root.join("adl/tools/lint_prompt_spec.sh"),
            tools_dir.join("lint_prompt_spec.sh"),
        ),
        (
            workspace_root.join("adl/tools/check_no_tracked_adl_issue_record_residue.sh"),
            tools_dir.join("check_no_tracked_adl_issue_record_residue.sh"),
        ),
        (
            workspace_root.join("adl/tools/attach_post_merge_closeout.sh"),
            tools_dir.join("attach_post_merge_closeout.sh"),
        ),
        (
            workspace_root.join("adl/templates/cards/input_card_template.md"),
            templates_dir.join("input_card_template.md"),
        ),
        (
            workspace_root.join("adl/templates/cards/output_card_template.md"),
            templates_dir.join("output_card_template.md"),
        ),
        (
            workspace_root.join("adl/schemas/structured_task_prompt.contract.yaml"),
            schemas_dir.join("structured_task_prompt.contract.yaml"),
        ),
        (
            workspace_root.join("adl/schemas/structured_implementation_prompt.contract.yaml"),
            schemas_dir.join("structured_implementation_prompt.contract.yaml"),
        ),
        (
            workspace_root.join("adl/schemas/structured_output_record.contract.yaml"),
            schemas_dir.join("structured_output_record.contract.yaml"),
        ),
    ];

    for (src, dst) in files {
        fs::copy(src, &dst).expect("copy support file");
        #[cfg(unix)]
        if dst.extension().is_none() || dst.to_string_lossy().ends_with(".sh") {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&dst).expect("metadata").permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&dst, perms).expect("chmod");
        }
    }
}

fn write_authored_issue_prompt(repo: &Path, issue_ref: &IssueRef, title: &str) {
    let path = issue_ref.issue_prompt_path(repo);
    fs::create_dir_all(path.parent().expect("issue prompt parent")).expect("create body dir");
    let content = format!(
            "---\nissue_card_schema: adl.issue.v1\nwp: \"unassigned\"\nslug: \"{slug}\"\ntitle: \"{title}\"\nlabels:\n  - \"track:roadmap\"\n  - \"area:tools\"\n  - \"type:task\"\n  - \"version:v0.86\"\nissue_number: {issue}\nstatus: \"active\"\naction: \"edit\"\ndepends_on: []\nmilestone_sprint: \"unplanned\"\nrequired_outcome_type:\n  - \"code\"\nrepo_inputs:\n  - \"https://github.com/example/repo/issues/{issue}\"\ncanonical_files: []\ndemo_required: false\ndemo_names: []\nissue_graph_notes:\n  - \"Authored for test coverage.\"\npr_start:\n  enabled: true\n  slug: \"{slug}\"\n---\n\n# {title}\n\n## Summary\n\nAuthored prompt for lifecycle validation tests.\n\n## Goal\n\nMake the issue prompt authored enough that lifecycle commands should accept it.\n\n## Required Outcome\n\nThis test issue ships code only.\n\n## Deliverables\n\n- authored issue prompt content\n\n## Acceptance Criteria\n\n- lifecycle validation accepts this source prompt\n\n## Repo Inputs\n\n- https://github.com/example/repo/issues/{issue}\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- bootstrap placeholder content\n\n## Issue-Graph Notes\n\n- test fixture\n\n## Notes\n\n- generated inside unit tests\n\n## Tooling Notes\n\n- authored fixture, not bootstrap fallback\n",
            slug = issue_ref.slug(),
            title = title,
            issue = issue_ref.issue_number()
        );
    fs::write(path, content).expect("write authored prompt");
}

fn write_authored_sip(
    path: &Path,
    issue_ref: &IssueRef,
    title: &str,
    branch: &str,
    source_prompt: &Path,
    repo_root: &Path,
) {
    let source_rel = path_relative_to_repo(repo_root, source_prompt);
    let content = format!(
            "# ADL Input Card\n\nTask ID: {task_id}\nRun ID: {task_id}\nVersion: v0.86\nTitle: {title}\nBranch: {branch}\n\nContext:\n- Issue: https://github.com/example/repo/issues/{issue}\n- PR: none\n- Source Issue Prompt: {source_rel}\n- Docs: none\n- Other: none\n\n## Agent Execution Rules\n- Do not run `pr start`; the branch and worktree already exist.\n- Only modify files required for the issue.\n\n## Prompt Spec\n```yaml\nprompt_schema: adl.v1\nactor:\n  role: execution_agent\n  name: codex\nmodel:\n  id: gpt-5-codex\n  determinism_mode: stable\ninputs:\n  sections:\n    - goal\n    - required_outcome\n    - acceptance_criteria\n    - inputs\n    - target_files_surfaces\n    - validation_plan\n    - demo_proof_requirements\n    - constraints_policies\n    - system_invariants\n    - reviewer_checklist\n    - non_goals_out_of_scope\n    - notes_risks\n    - instructions_to_agent\noutputs:\n  output_card: .adl/v0.86/tasks/{bundle}/sor.md\n  summary_style: concise_structured\nconstraints:\n  include_system_invariants: true\n  include_reviewer_checklist: true\n  disallow_secrets: true\n  disallow_absolute_host_paths: true\nautomation_hints:\n  source_issue_prompt_required: true\n  target_files_surfaces_recommended: true\n  validation_plan_required: true\n  required_outcome_type_supported: true\nreview_surfaces:\n  - card_review_checklist.v1\n  - card_review_output.v1\n  - card_reviewer_gpt.v1.1\n```\n\nExecution:\n- Agent: codex\n- Provider: openai\n- Tools allowed: git, cargo\n- Sandbox / approvals: workspace-write\n- Source issue-prompt slug: {slug}\n- Required outcome type: code\n- Demo required: false\n\n## Goal\n\nBlock lifecycle execution when prompts are still bootstrap stubs.\n\n## Required Outcome\n\n- This issue must ship code and tests.\n\n## Acceptance Criteria\n\n- lifecycle commands reject placeholder prompt content\n\n## Inputs\n- issue body\n- task bundle cards\n\n## Target Files / Surfaces\n- adl/src/cli/pr_cmd.rs\n- adl/tools/pr.sh\n\n## Validation Plan\n- Required commands: cargo test --manifest-path Cargo.toml pr_cmd -- --nocapture\n- Required tests: targeted lifecycle validation coverage\n- Required artifacts / traces: none\n- Required reviewer or demo checks: none\n\n## Demo / Proof Requirements\n- Required demo(s): none\n- Required proof surface(s): command failure behavior and tests\n- If no demo is required, say why: tooling guardrail only\n\n## Constraints / Policies\n- Determinism requirements: stable error messages for the same stub input\n- Security / privacy requirements: no secrets or absolute host paths\n- Resource limits (time/CPU/memory/network): standard local test limits\n\n## System Invariants (must remain true)\n- Deterministic execution for identical inputs.\n- No hidden state or undeclared side effects.\n- Artifacts remain replay-compatible with the replay runner.\n- Trace artifacts contain no secrets, prompts, tool arguments, or absolute host paths.\n- Artifact schema changes are explicit and approved.\n\n## Reviewer Checklist (machine-readable hints)\n```yaml\ndeterminism_required: true\nnetwork_allowed: false\nartifact_schema_change: false\nreplay_required: true\nsecurity_sensitive: true\nci_validation_required: true\n```\n\n## Card Automation Hooks (prompt generation)\n- Prompt source fields:\n  - Goal\n  - Required Outcome\n  - Acceptance Criteria\n- Generation requirements:\n  - Deterministic output for identical input card content\n  - Preserve traceability back to the source issue prompt\n\n## Non-goals / Out of scope\n- rewriting historical issues automatically\n\n## Notes / Risks\n- none\n\n## Instructions to the Agent\n- Read the linked source issue prompt before starting work.\n- Do the work described above.\n- Write results to the paired output card file.\n",
            task_id = issue_ref.task_issue_id(),
            title = title,
            branch = branch,
            issue = issue_ref.issue_number(),
            source_rel = source_rel,
            bundle = issue_ref.task_bundle_dir_name(),
            slug = issue_ref.slug(),
        );
    fs::write(path, content).expect("write authored sip");
}

fn write_completed_sor_fixture(path: &Path, branch: &str) {
    let body = format!(
        r#"# rust-finish-test

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1153
Run ID: issue-1153
Version: v0.86
Title: rust-finish-test
Branch: {branch}
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: Test
- Start Time: 2026-03-29T20:19:06Z
- End Time: 2026-03-29T20:19:09Z

## Summary

Finish test summary.

## Artifacts produced
- Code:
  - `adl/src/cli/pr_cmd.rs`
- Generated runtime artifacts: not_applicable for this tooling task

## Actions taken
- Added Rust finish handling.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: branch-local validation before draft PR publication
- Verification performed:
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd`
    Verified Rust `pr` command tests.
- Result: PASS

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout <branch> -- <path>` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- `Integration state` describes lifecycle state of the integrated artifact set, not where verification happened.
- `Verification scope` describes where the verification commands were run.
- `worktree_only` means at least one required path still exists only outside the main repository path.
- `pr_open` should pair with truthful `Worktree-only paths remaining` content; list those paths when they still exist only in the worktree or say `none` only when the branch contents are fully represented in the main repository path.
- If `Integration state` is `pr_open`, verify the actual proof artifacts rather than only the containing directory or card path.
- If `Integration method used` is `direct write in main repo`, `Verification scope` should normally be `main_repo` unless the deviation is explained.
- If `Verification scope` and `Integration method used` differ in a non-obvious way, explain the difference in one line.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By `pr finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- Validation commands and their purpose:
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd`
    Verified Rust `pr` command tests.
- Results:
  - PASS

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

Rules:
- Replace the example values below with one actual final value per field.
- Do not leave pipe-delimited enum menus or placeholder text in a finished record.

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "cargo test --manifest-path adl/Cargo.toml pr_cmd"
  determinism:
    status: PASS
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: PASS
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: PASS
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed:
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd`
- Fixtures or scripts used:
  - direct Rust unit coverage
- Replay verification (same inputs -> same artifacts/order):
  - PASS
- Ordering guarantees (sorting / tie-break rules used):
  - Stable section ordering.
- Artifact stability notes:
  - not_applicable beyond deterministic record rendering.

## Security / Privacy Checks
- Secret leakage scan performed:
  - Verified test output uses repo-relative paths only.
- Prompt / tool argument redaction verified:
  - Verified issue template text is not emitted in PR bodies.
- Absolute path leakage check:
  - PASS
- Sandbox / policy invariants preserved:
  - PASS

## Replay Artifacts
- Trace bundle path(s): not_applicable for this tooling task
- Run artifact root: not_applicable for this tooling task
- Replay command used for verification:
  - `cargo test --manifest-path adl/Cargo.toml pr_cmd`
- Replay result:
  - PASS

## Artifact Verification
- Primary proof surface:
  - `adl/src/cli/pr_cmd.rs`
- Required artifacts present:
  - yes
- Artifact schema/version checks:
  - none
- Hash/byte-stability checks:
  - not_applicable
- Missing/optional artifacts and rationale:
  - none

## Decisions / Deviations
- Kept the fixture minimal while satisfying completed-phase validation.

## Follow-ups / Deferred work
- none
"#
    );
    fs::write(path, body).expect("write completed sor fixture");
}

#[test]
fn write_output_card_emits_truthful_pre_run_scaffold() {
    let _guard = env_lock();
    let repo = unique_temp_dir("adl-pr-bootstrap-output-card");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let issue_ref = IssueRef::new(1442, "v0.90.4", "normalize-child-sors").expect("issue ref");
    let output = issue_ref.task_bundle_output_path(&repo);
    fs::create_dir_all(output.parent().expect("output parent")).expect("create bundle dir");

    write_output_card(
        &repo,
        &output,
        &issue_ref,
        "[v0.90.4][tools] Normalize child SORs during WP-01 issue-wave opening",
        "codex/1442-normalize-child-sors",
    )
    .expect("write bootstrap output");

    validate_bootstrap_output_card(
        &repo,
        1442,
        "normalize-child-sors",
        "codex/1442-normalize-child-sors",
        &output,
    )
    .expect("bootstrap output should validate");

    let text = fs::read_to_string(&output).expect("read output");
    assert!(text.contains("Pre-run output scaffold initialized during issue-wave opening."));
    assert!(text.contains("Local ignored output-card scaffold"));
    assert!(text.contains("Integration method used: direct write in main repo for the local ignored pre-run record; tracked implementation artifacts do not exist yet"));
    assert!(text.contains("Verification scope: main_repo"));
    assert!(text.contains("Issue-wave opening emits a truthful pre-run SOR scaffold instead of leaving raw template residue for later cleanup."));
    assert!(!text.contains("none | list explicitly"));
    assert!(!text.contains("PASS | FAIL"));
    assert!(!text.contains("worktree | pr_branch | main_repo"));
}

mod basics;
mod finish;
mod lifecycle;
mod repo_helpers;
