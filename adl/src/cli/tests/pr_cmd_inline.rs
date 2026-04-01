use super::*;
use std::env;
use std::sync::{Mutex, OnceLock};
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

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
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
            "https://github.com/danielbaustin/agent-design-language.git"
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
fn render_generated_issue_prompt_preserves_bootstrap_contract() {
    let content = render_generated_issue_prompt(
        1151,
        "v0-86-tools-implement-rust-owned-pr-init-and-pr-create-workflow-surfaces",
        "[v0.86][tools] Implement Rust-owned pr init and pr create workflow surfaces",
        "track:roadmap,type:task,area:tooling,version:v0.86",
        "https://github.com/example/repo/issues/1151",
    );
    assert!(content.contains("issue_number: 1151"));
    assert!(content.contains(
        "slug: \"v0-86-tools-implement-rust-owned-pr-init-and-pr-create-workflow-surfaces\""
    ));
    assert!(content.contains("required_outcome_type:\n  - \"code\""));
    assert!(content
        .contains("Bootstrap-generated issue body created from the requested title and labels"));
    assert!(content.contains(
            "This body should be concrete enough that `gh issue view` is usable immediately after creation."
        ));
}

#[test]
fn load_issue_prompt_parses_front_matter_and_body() {
    let dir = unique_temp_dir("adl-pr-load-prompt");
    let path = dir.join("issue.md");
    fs::write(
            &path,
            "---\ntitle: \"Example\"\nlabels:\n  - \"track:roadmap\"\nissue_number: 42\n---\n\n# Heading\n\nBody\n",
        )
        .expect("write");

    let doc = load_issue_prompt(&path).expect("load");
    assert_eq!(doc.front_matter.title, "Example");
    assert_eq!(doc.front_matter.issue_number, 42);
    assert_eq!(doc.front_matter.labels, vec!["track:roadmap"]);
    assert!(doc.body.starts_with("# Heading"));
}

#[test]
fn normalize_labels_csv_replaces_version_label() {
    let labels = normalize_labels_csv("track:roadmap,type:task,version:v0.3,area:tooling", "v0.86");
    assert_eq!(labels, "track:roadmap,type:task,area:tooling,version:v0.86");
}

#[test]
fn infer_repo_from_remote_supports_https_and_ssh() {
    assert_eq!(
        infer_repo_from_remote("https://github.com/danielbaustin/agent-design-language.git"),
        Some("danielbaustin/agent-design-language".to_string())
    );
    assert_eq!(
        infer_repo_from_remote("git@github.com:danielbaustin/agent-design-language.git"),
        Some("danielbaustin/agent-design-language".to_string())
    );
    assert_eq!(
        infer_repo_from_remote("https://example.com/not-github.git"),
        None
    );
}

#[test]
fn infer_wp_from_title_extracts_tag_or_defaults() {
    assert_eq!(
        infer_wp_from_title("[v0.86][WP-15] Implement local agent demo program"),
        "WP-15"
    );
    assert_eq!(infer_wp_from_title("No work package tag"), "unassigned");
}

#[test]
fn infer_required_outcome_type_prefers_docs_tests_and_demo_signals() {
    assert_eq!(
        infer_required_outcome_type("track:roadmap,area:docs", "[v0.86][WP-01] Example"),
        "docs"
    );
    assert_eq!(
        infer_required_outcome_type("track:roadmap,type:test", "[v0.86][WP-01] Example"),
        "tests"
    );
    assert_eq!(
        infer_required_outcome_type("track:roadmap,area:demo", "[v0.86][WP-01] Example"),
        "demo"
    );
    assert_eq!(
        infer_required_outcome_type("track:roadmap,area:runtime", "[v0.86][WP-01] Example"),
        "code"
    );
}

#[test]
fn version_can_be_inferred_from_labels_or_title() {
    assert_eq!(
        version_from_labels_csv("track:roadmap,version:v0.86,area:tools"),
        Some("v0.86".to_string())
    );
    assert_eq!(
        version_from_title("[v0.86][WP-15] Implement local agent demo program"),
        Some("v0.86".to_string())
    );
    assert_eq!(version_from_title("No version title"), None);
}

#[test]
fn resolve_issue_body_uses_inline_text_default_and_file() {
    assert_eq!(
        resolve_issue_body(Some("custom body".to_string()), None).expect("body"),
        "custom body"
    );
    assert_eq!(resolve_issue_body(None, None).expect("default body"), "");

    let dir = unique_temp_dir("adl-pr-body-file");
    let path = dir.join("body.md");
    fs::write(&path, "body from file").expect("write body");
    assert_eq!(
        resolve_issue_body(None, Some(&path)).expect("file body"),
        "body from file"
    );
}

#[test]
fn resolve_issue_body_rejects_stdin_and_missing_file() {
    let err = resolve_issue_body(None, Some(Path::new("-"))).expect_err("stdin unsupported");
    assert!(err.to_string().contains("--body-file - is not supported"));

    let missing = PathBuf::from("/definitely/missing/body.md");
    let err = resolve_issue_body(None, Some(&missing)).expect_err("missing file");
    assert!(err.to_string().contains("--body-file not found"));
}

#[test]
fn parse_issue_number_from_url_accepts_issue_url_and_rejects_other_suffixes() {
    assert_eq!(
        parse_issue_number_from_url("https://github.com/example/repo/issues/1151")
            .expect("issue number"),
        1151
    );
    assert!(
        parse_issue_number_from_url("https://github.com/example/repo/issues/not-a-number").is_err()
    );
}

#[test]
fn path_relative_to_repo_returns_relative_or_absolute_when_outside_repo() {
    let repo_root = Path::new("/tmp/example-repo");
    let inside = Path::new("/tmp/example-repo/.adl/cards/1151/input_1151.md");
    let outside = Path::new("/var/tmp/elsewhere.md");
    assert_eq!(
        path_relative_to_repo(repo_root, inside),
        ".adl/cards/1151/input_1151.md"
    );
    assert_eq!(
        path_relative_to_repo(repo_root, outside),
        "/var/tmp/elsewhere.md"
    );
}

#[test]
fn parse_init_args_accepts_bootstrap_flags() {
    let parsed = parse_init_args(&[
        "1151".to_string(),
        "--title".to_string(),
        "Example".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("parse");
    assert_eq!(parsed.issue, 1151);
    assert_eq!(parsed.title_arg.as_deref(), Some("Example"));
    assert_eq!(parsed.version.as_deref(), Some("v0.86"));
}

#[test]
fn parse_create_args_accepts_issue_creation_flags() {
    let parsed = parse_create_args(&[
        "--title".to_string(),
        "[v0.86][tools] New init path".to_string(),
        "--slug".to_string(),
        "new-init-path".to_string(),
        "--body".to_string(),
        "## Goal\n- test".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:tools".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("parse");
    assert_eq!(
        parsed.title_arg.as_deref(),
        Some("[v0.86][tools] New init path")
    );
    assert_eq!(parsed.slug.as_deref(), Some("new-init-path"));
    assert_eq!(parsed.body.as_deref(), Some("## Goal\n- test"));
    assert_eq!(
        parsed.labels.as_deref(),
        Some("track:roadmap,type:task,area:tools")
    );
    assert_eq!(parsed.version.as_deref(), Some("v0.86"));
}

#[test]
fn parse_init_args_rejects_unknown_arg() {
    let err = parse_init_args(&["1151".to_string(), "--bogus".to_string()]).expect_err("err");
    assert!(err.to_string().contains("init: unknown arg"));
}

#[test]
fn parse_create_args_rejects_missing_title_and_conflicting_body_inputs() {
    let err = parse_create_args(&[]).expect_err("missing title");
    assert!(err.to_string().contains("create: --title is required"));

    let err = parse_create_args(&[
        "--title".to_string(),
        "Example".to_string(),
        "--body".to_string(),
        "a".to_string(),
        "--body-file".to_string(),
        "body.md".to_string(),
    ])
    .expect_err("conflicting body inputs");
    assert!(err
        .to_string()
        .contains("create: pass only one of --body or --body-file"));
}

#[test]
fn real_pr_dispatch_rejects_missing_and_unknown_subcommands() {
    let err = real_pr(&[]).expect_err("missing subcommand");
    assert!(err
        .to_string()
        .contains("pr requires a subcommand: create | init | start | ready | preflight | finish"));

    let err = real_pr(&["bogus".to_string()]).expect_err("unknown subcommand");
    assert!(err.to_string().contains("unknown pr subcommand: bogus"));
}

#[test]
fn parse_ready_args_accepts_flags_and_rejects_unknown_arg() {
    let parsed = parse_ready_args(&[
        "1152".to_string(),
        "--slug".to_string(),
        "ready-test".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--no-fetch-issue".to_string(),
    ])
    .expect("parse ready");
    assert_eq!(parsed.issue, 1152);
    assert_eq!(parsed.slug.as_deref(), Some("ready-test"));
    assert_eq!(parsed.version.as_deref(), Some("v0.86"));
    assert!(parsed.no_fetch_issue);

    let err = parse_ready_args(&["1152".to_string(), "--bogus".to_string()]).expect_err("err");
    assert!(err.to_string().contains("ready: unknown arg"));
}

#[test]
fn parse_preflight_args_accepts_flags_and_rejects_unknown_arg() {
    let parsed = parse_preflight_args(&[
        "1173".to_string(),
        "--slug".to_string(),
        "preflight-test".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--no-fetch-issue".to_string(),
    ])
    .expect("parse preflight");
    assert_eq!(parsed.issue, 1173);
    assert_eq!(parsed.slug.as_deref(), Some("preflight-test"));
    assert_eq!(parsed.version.as_deref(), Some("v0.86"));
    assert!(parsed.no_fetch_issue);

    let err = parse_preflight_args(&["1173".to_string(), "--bogus".to_string()]).expect_err("err");
    assert!(err.to_string().contains("preflight: unknown arg"));
}

#[test]
fn parse_start_args_accepts_prefix_and_rejects_unknown_arg() {
    let parsed = parse_start_args(&[
        "1152".to_string(),
        "--prefix".to_string(),
        "codex".to_string(),
        "--slug".to_string(),
        "start-test".to_string(),
        "--title".to_string(),
        "Start Test".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--no-fetch-issue".to_string(),
        "--allow-open-pr-wave".to_string(),
    ])
    .expect("parse start");
    assert_eq!(parsed.issue, 1152);
    assert_eq!(parsed.prefix, "codex");
    assert_eq!(parsed.slug.as_deref(), Some("start-test"));
    assert_eq!(parsed.title_arg.as_deref(), Some("Start Test"));
    assert_eq!(parsed.version.as_deref(), Some("v0.86"));
    assert!(parsed.no_fetch_issue);
    assert!(parsed.allow_open_pr_wave);

    let err = parse_start_args(&["1152".to_string(), "--bogus".to_string()]).expect_err("err");
    assert!(err.to_string().contains("start: unknown arg"));
}

#[test]
fn real_pr_init_seeds_stp_from_generated_source_prompt() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-real-init");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
        "init".to_string(),
        "1151".to_string(),
        "--slug".to_string(),
        "v0-86-tools-init-test".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Init test".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    result.expect("real_pr init");

    let issue_ref = IssueRef::new(
        1151,
        "v0.86".to_string(),
        "v0-86-tools-init-test".to_string(),
    )
    .expect("issue ref");
    let stp_path = issue_ref.task_bundle_stp_path(&repo);
    let source_path = issue_ref.issue_prompt_path(&repo);
    let sip_path = issue_ref.task_bundle_input_path(&repo);
    let sor_path = issue_ref.task_bundle_output_path(&repo);
    assert!(stp_path.is_file());
    assert!(source_path.is_file());
    assert!(sip_path.is_file());
    assert!(sor_path.is_file());
    let stp = fs::read_to_string(&stp_path).expect("read stp");
    assert!(stp.contains("issue_number: 1151"));
    assert!(stp.contains("title: \"[v0.86][tools] Init test\""));
}

#[test]
fn real_pr_init_existing_stp_is_left_untouched() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-real-init-existing");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
    let issue_ref = IssueRef::new(
        1151,
        "v0.86".to_string(),
        "v0-86-tools-init-existing".to_string(),
    )
    .expect("issue ref");
    let stp_path = issue_ref.task_bundle_stp_path(&repo);
    let sip_path = issue_ref.task_bundle_input_path(&repo);
    let sor_path = issue_ref.task_bundle_output_path(&repo);
    fs::create_dir_all(stp_path.parent().expect("parent")).expect("bundle dir");
    fs::write(&stp_path, "sentinel\n").expect("write sentinel");

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let result = real_pr(&[
        "init".to_string(),
        "1151".to_string(),
        "--slug".to_string(),
        "v0-86-tools-init-existing".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Init existing".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);
    env::set_current_dir(prev_dir).expect("restore cwd");
    result.expect("real_pr init existing");
    assert_eq!(
        fs::read_to_string(&stp_path).expect("read stp"),
        "sentinel\n"
    );
    assert!(sip_path.is_file());
    assert!(sor_path.is_file());
}

#[test]
fn real_pr_create_creates_issue_without_bootstrapping_bundle() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-real-create");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = repo.join("gh.log");
    let issue_body_log = repo.join("issue_body.log");
    let gh_path = bin_dir.join("gh");
    write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [[ \"$1 $2\" == \"issue create\" ]]; then\n  i=1\n  while [[ $i -le $# ]]; do\n    arg=\"${{@:$i:1}}\"\n    if [[ \"$arg\" == \"--body\" ]]; then\n      next=$((i+1))\n      printf '%s' \"${{@:$next:1}}\" > '{}'\n      break\n    fi\n    i=$((i+1))\n  done\n  printf 'https://github.com/example/repo/issues/1202\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
                gh_log.display(),
                issue_body_log.display()
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
            "create".to_string(),
            "--title".to_string(),
            "[v0.86][tools] Simplified init path".to_string(),
            "--slug".to_string(),
            "v0-86-tools-simplified-init-path".to_string(),
            "--body".to_string(),
            "## Summary\n\nTighten lifecycle validation for issue creation.\n\n## Goal\n\nMake create reject bodies that cannot become valid source prompts.\n\n## Required Outcome\n\nThis issue ships tooling code and tests.\n\n## Deliverables\n\n- create-path validation\n\n## Acceptance Criteria\n\n- invalid issue bodies are rejected early\n\n## Repo Inputs\n\n- adl/src/cli/pr_cmd.rs\n\n## Dependencies\n\n- none\n\n## Demo Expectations\n\n- none\n\n## Non-goals\n\n- lifecycle redesign\n\n## Issue-Graph Notes\n\n- test fixture\n\n## Notes\n\n- authored test body\n\n## Tooling Notes\n\n- should pass source-prompt validation\n".to_string(),
            "--labels".to_string(),
            "track:roadmap,type:task,area:tools".to_string(),
            "--version".to_string(),
            "v0.86".to_string(),
        ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    result.expect("real_pr create");

    let gh_calls = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_calls.contains("issue create"));
    assert!(gh_calls.contains("--label"));
    assert!(gh_calls.contains("version:v0.86"));
    let source = repo.join(".adl/v0.86/bodies/issue-1202-v0-86-tools-simplified-init-path.md");
    assert!(
        source.is_file(),
        "create should write the local source prompt"
    );
    let prompt = fs::read_to_string(&source).expect("read source prompt");
    assert!(prompt.contains("issue_number: 1202"));
    assert!(prompt.contains("## Summary"));
    assert!(prompt.contains("## Tooling Notes"));
    assert!(
        !repo.join(".adl/v0.86/tasks").exists(),
        "create should not bootstrap the local task bundle"
    );
    let issue_body = fs::read_to_string(&issue_body_log).expect("issue body");
    assert!(issue_body.contains("## Summary"));
    assert!(issue_body.contains("## Tooling Notes"));
}

#[test]
fn real_pr_create_generates_concrete_body_when_none_is_supplied() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-real-create-generated-body");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let issue_body_log = repo.join("issue_body.log");
    let gh_path = bin_dir.join("gh");
    write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"issue create\" ]]; then\n  i=1\n  while [[ $i -le $# ]]; do\n    arg=\"${{@:$i:1}}\"\n    if [[ \"$arg\" == \"--body\" ]]; then\n      next=$((i+1))\n      printf '%s' \"${{@:$next:1}}\" > '{}'\n      break\n    fi\n    i=$((i+1))\n  done\n  printf 'https://github.com/example/repo/issues/1203\\n'\n  exit 0\nfi\nif [[ \"$1 $2\" == \"issue edit\" ]]; then\n  exit 0\nfi\nexit 1\n",
                issue_body_log.display()
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
        "create".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Generated issue body".to_string(),
        "--slug".to_string(),
        "v0-86-tools-generated-issue-body".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:tools".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    result.expect("real_pr create");

    let issue_body = fs::read_to_string(&issue_body_log).expect("issue body");
    assert!(issue_body.contains("## Goal"));
    assert!(issue_body.contains("## Acceptance Criteria"));
    assert!(!issue_body.contains("## Goal\n-"));
    assert!(!issue_body.contains("## Acceptance Criteria\n-"));
    let source = repo.join(".adl/v0.86/bodies/issue-1203-v0-86-tools-generated-issue-body.md");
    let prompt = fs::read_to_string(&source).expect("read source prompt");
    assert!(prompt.contains("issue_number: 1203"));
    assert!(prompt.contains("## Goal"));
    assert!(!prompt.contains("## Goal\n-"));
}

#[test]
fn real_pr_create_rejects_issue_body_that_cannot_pass_source_prompt_validation() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-real-create-invalid-body");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
        &gh_path,
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 99\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let err = real_pr(&[
        "create".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Invalid issue body".to_string(),
        "--slug".to_string(),
        "v0-86-tools-invalid-issue-body".to_string(),
        "--body".to_string(),
        "## Goal\n\nmissing required sections\n".to_string(),
        "--labels".to_string(),
        "track:roadmap,type:task,area:tools".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect_err("invalid issue body should fail before gh issue create");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }

    assert!(err
        .to_string()
        .contains("create: issue body cannot satisfy source-prompt validation"));
}

#[test]
fn real_pr_start_bootstraps_worktree_and_ready_passes() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(1152, "v0.86", "rust-start-ready-test").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust start ready test");

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
    assert!(issue_ref.task_bundle_stp_path(&worktree).is_file());
    assert!(issue_ref.task_bundle_input_path(&worktree).is_file());
    assert!(issue_ref.task_bundle_output_path(&worktree).is_file());
    let root_cards = resolve_cards_root(&repo, None);
    assert!(card_input_path(&root_cards, 1152)
        .symlink_metadata()
        .is_ok());
    assert!(card_output_path(&root_cards, 1152)
        .symlink_metadata()
        .is_ok());
}

#[test]
fn real_pr_ready_succeeds_when_invoked_from_started_worktree() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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
    env::set_current_dir(&repo).expect("chdir");
    let issue_ref = IssueRef::new(1198, "v0.86", "ready-worktree-cwd").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Ready worktree cwd");
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
    ready.expect("ready from worktree");
}

#[test]
fn real_pr_preflight_reports_open_milestone_prs() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-preflight");
    init_git_repo(&repo);
    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
            &gh_path,
            "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"pr list\" ]]; then\n  cat <<'JSON'\n[{\"number\":1169,\"title\":\"[v0.86][runtime] Sprint 3A: Make WP-06 fast / slow paths drive real runtime behavior\",\"url\":\"https://example.test/pr/1169\",\"headRefName\":\"codex/1161-v0-86-runtime-sprint-3a-make-wp-06-fast-slow-paths-drive-real-runtime-behavior\",\"baseRefName\":\"main\",\"isDraft\":true}]\nJSON\n  exit 0\nfi\nexit 1\n",
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
        "preflight".to_string(),
        "1173".to_string(),
        "--slug".to_string(),
        "v0-86-tools-preflight".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
        "--no-fetch-issue".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    result.expect("preflight");
}

#[test]
fn real_pr_start_blocks_when_open_milestone_pr_wave_exists() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_path = bin_dir.join("gh");
    write_executable(
            &gh_path,
            "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1 $2\" == \"pr list\" ]]; then\n  cat <<'JSON'\n[{\"number\":1169,\"title\":\"[v0.86][runtime] Sprint 3A: Make WP-06 fast / slow paths drive real runtime behavior\",\"url\":\"https://example.test/pr/1169\",\"headRefName\":\"codex/1161-v0-86-runtime-sprint-3a-make-wp-06-fast-slow-paths-drive-real-runtime-behavior\",\"baseRefName\":\"main\",\"isDraft\":true}]\nJSON\n  exit 0\nfi\nexit 1\n",
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
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
    .expect_err("start should block on open PR wave");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    assert!(err
        .to_string()
        .contains("start: unresolved open PR wave detected for v0.86"));
    assert!(err.to_string().contains("#1169 [draft]"));
}

#[test]
fn real_pr_ready_requires_slug_when_local_state_missing() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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
        "--no-open".to_string(),
    ])
    .expect("parse finish");
    assert_eq!(parsed.issue, 1153);
    assert_eq!(parsed.title, "Example");
    assert_eq!(parsed.paths, "adl,docs");
    assert!(parsed.no_checks);
    assert!(parsed.ready);
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
fn real_pr_finish_creates_draft_pr_and_commits_branch_changes() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-finish-create");
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
        .args(["checkout", "-q", "-b", "codex/1153-rust-finish-test",])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let issue_ref =
        IssueRef::new(1153, "v0.86".to_string(), "rust-finish-test".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust finish test");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Rust finish test",
        "codex/1153-rust-finish-test",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1153-rust-finish-test");

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
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr create' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1159\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1153\\n'\n  else\n    printf 'Closes #1153\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
                gh_log.display()
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
        "finish".to_string(),
        "1153".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Rust finish test".to_string(),
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
    result.expect("real_pr finish");

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
    assert!(head_subject.contains("[v0.86][tools] Rust finish test (Closes #1153)"));
    assert!(Command::new("git")
        .args([
            "--git-dir",
            path_str(&origin).expect("origin"),
            "rev-parse",
            "--verify",
            "refs/heads/codex/1153-rust-finish-test",
        ])
        .status()
        .expect("verify pushed branch")
        .success());
    let gh_calls = fs::read_to_string(&gh_log).expect("read gh log");
    assert!(gh_calls.contains("pr create"));
    assert!(gh_calls.contains("pr view -R danielbaustin/agent-design-language https://github.com/danielbaustin/agent-design-language/pull/1159 --json closingIssuesReferences --jq .closingIssuesReferences[]?.number"));
}

#[test]
fn real_pr_finish_syncs_completed_output_to_root_bundle_and_review_mirror() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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

#[test]
fn real_pr_finish_updates_existing_pr_and_marks_ready() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-finish-edit");
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
        .args(["checkout", "-q", "-b", "codex/1153-rust-finish-test-edit",])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let issue_ref = IssueRef::new(
        1153,
        "v0.86".to_string(),
        "rust-finish-test-edit".to_string(),
    )
    .expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Rust finish test edit");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "[v0.86][tools] Rust finish test edit",
        "codex/1153-rust-finish-test-edit",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1153-rust-finish-test-edit");

    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write change");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "existing branch commit"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());

    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let gh_path = bin_dir.join("gh");
    write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'repo view --json' ]; then\n  printf 'danielbaustin/agent-design-language\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr list' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1160\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr edit' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr ready' ]; then\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1153\\n'\n  else\n    printf 'Closes #1153\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
                gh_log.display()
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let result = real_pr(&[
        "finish".to_string(),
        "1153".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Rust finish test edit".to_string(),
        "--paths".to_string(),
        "adl".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &input),
        "--output".to_string(),
        path_relative_to_repo(&repo, &output),
        "--no-checks".to_string(),
        "--ready".to_string(),
        "--no-open".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    result.expect("real_pr finish edit");

    let gh_calls = fs::read_to_string(&gh_log).expect("read gh log");
    assert!(gh_calls.contains("pr edit -R danielbaustin/agent-design-language https://github.com/danielbaustin/agent-design-language/pull/1160 --title [v0.86][tools] Rust finish test edit --body-file"));
    assert!(gh_calls.contains("pr ready -R danielbaustin/agent-design-language https://github.com/danielbaustin/agent-design-language/pull/1160"));
}

#[test]
fn finish_helper_paths_cover_nonempty_and_staged_checks() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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
}

#[test]
fn finish_helper_paths_cover_ahead_count_and_batch_checks() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-finish-batch-checks");
    let origin = temp.join("origin.git");
    let repo = temp.join("repo");
    fs::create_dir_all(repo.join("adl")).expect("adl dir");
    fs::write(
        repo.join("adl/Cargo.toml"),
        "[package]\nname='adl'\nversion='0.1.0'\n",
    )
    .expect("cargo toml");
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

#[test]
fn finish_helper_paths_cover_pr_lookup_and_closing_linkage() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-finish-gh-helpers");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let gh_log = temp.join("gh.log");
    let gh_path = bin_dir.join("gh");
    write_executable(
            &gh_path,
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2\" = 'pr list' ]; then\n  printf 'https://github.com/danielbaustin/agent-design-language/pull/1159\\n'\n  exit 0\nfi\nif [ \"$1 $2\" = 'pr view' ]; then\n  if printf '%s ' \"$@\" | grep -q 'closingIssuesReferences'; then\n    printf '1153\\n'\n  else\n    printf 'Closes #1153\\n'\n  fi\n  exit 0\nfi\nexit 1\n",
                gh_log.display()
            ),
        );
    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    let pr = current_pr_url(
        "danielbaustin/agent-design-language",
        "codex/1153-rust-finish-test",
    )
    .expect("pr url");
    assert_eq!(
        pr.as_deref(),
        Some("https://github.com/danielbaustin/agent-design-language/pull/1159")
    );
    assert!(pr_has_closing_linkage(
        "danielbaustin/agent-design-language",
        "https://github.com/danielbaustin/agent-design-language/pull/1159",
        1153
    )
    .expect("closing linkage"));
    ensure_pr_closing_linkage(
        "danielbaustin/agent-design-language",
        "https://github.com/danielbaustin/agent-design-language/pull/1159",
        1153,
        false,
    )
    .expect("ensure linkage");
    ensure_pr_closing_linkage(
        "danielbaustin/agent-design-language",
        "https://github.com/danielbaustin/agent-design-language/pull/1159",
        1153,
        true,
    )
    .expect("no close skips");

    unsafe {
        env::set_var("PATH", old_path);
    }
    let gh_calls = fs::read_to_string(&gh_log).expect("gh log");
    assert!(gh_calls.contains("pr list -R danielbaustin/agent-design-language --head codex/1153-rust-finish-test --state open --json url --jq .[0].url"));
}

#[test]
fn real_pr_finish_rejects_main_and_no_changes_paths() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-finish-errors");
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
    assert!(Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&repo)
        .status()
        .expect("git branch")
        .success());
    fs::create_dir_all(repo.join("adl/src")).expect("adl src");
    fs::write(repo.join("adl/src/lib.rs"), "pub fn placeholder() {}\n").expect("write source");
    let issue_ref =
        IssueRef::new(1153, "v0.86".to_string(), "rust-finish-test".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    fs::write(issue_ref.task_bundle_input_path(&repo), "# input\n").expect("input");
    write_completed_sor_fixture(
        &issue_ref.task_bundle_output_path(&repo),
        "codex/1153-rust-finish-test",
    );
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "init on main"])
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
        .args(["push", "-q", "-u", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());
    let issue_ref =
        IssueRef::new(1153, "v0.86".to_string(), "rust-finish-test".to_string()).expect("ref");
    let bundle_dir = issue_ref.task_bundle_dir_path(&repo);
    fs::create_dir_all(&bundle_dir).expect("bundle dir");
    let stp = issue_ref.task_bundle_stp_path(&repo);
    let input = issue_ref.task_bundle_input_path(&repo);
    let output = issue_ref.task_bundle_output_path(&repo);
    write_authored_issue_prompt(&repo, &issue_ref, "Example");
    fs::copy(issue_ref.issue_prompt_path(&repo), &stp).expect("seed stp");
    write_authored_sip(
        &input,
        &issue_ref,
        "Example",
        "codex/1153-rust-finish-test",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_completed_sor_fixture(&output, "codex/1153-rust-finish-test");
    assert!(Command::new("git")
        .args(["add", "-A"])
        .current_dir(&repo)
        .status()
        .expect("git add")
        .success());
    assert!(Command::new("git")
        .args(["commit", "-q", "-m", "seed finish bundle"])
        .current_dir(&repo)
        .status()
        .expect("git commit")
        .success());
    assert!(Command::new("git")
        .args(["push", "-q", "origin", "main"])
        .current_dir(&repo)
        .status()
        .expect("git push")
        .success());

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let main_err = real_pr(&[
        "finish".to_string(),
        "1153".to_string(),
        "--title".to_string(),
        "Example".to_string(),
        "--no-checks".to_string(),
        "--no-open".to_string(),
    ])
    .expect_err("main should be rejected");
    assert!(main_err.to_string().contains("refusing to run on main"));
    env::set_current_dir(prev_dir).expect("restore cwd");

    assert!(Command::new("git")
        .args(["checkout", "-q", "-b", "codex/1153-rust-finish-test"])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");
    let no_change_err = real_pr(&[
        "finish".to_string(),
        "1153".to_string(),
        "--title".to_string(),
        "Example".to_string(),
        "--input".to_string(),
        path_relative_to_repo(&repo, &input),
        "--output".to_string(),
        path_relative_to_repo(&repo, &output),
        "--no-checks".to_string(),
        "--no-open".to_string(),
    ])
    .expect_err("no changes should fail");
    env::set_current_dir(prev_dir).expect("restore cwd");
    assert!(no_change_err.to_string().contains("Nothing to PR."));
}

#[test]
fn real_pr_finish_rejects_not_started_output_card_before_publication() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
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
fn default_repo_falls_back_to_local_name_when_remote_and_gh_are_unavailable() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-default-repo-fallback");
    assert!(Command::new("git")
        .arg("init")
        .arg("-q")
        .current_dir(&repo)
        .status()
        .expect("git init")
        .success());

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
        &bin_dir.join("gh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nexit 1\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let inferred = default_repo(&repo).expect("default repo");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    assert_eq!(
        inferred,
        format!("local/{}", repo.file_name().unwrap().to_string_lossy())
    );
}

#[test]
fn default_repo_uses_gh_repo_when_remote_is_unparseable() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-default-repo-gh");
    assert!(Command::new("git")
        .arg("init")
        .arg("-q")
        .current_dir(&repo)
        .status()
        .expect("git init")
        .success());

    let bin_dir = repo.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
        &bin_dir.join("gh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf 'owner/example\\n'\n",
    );

    let old_path = env::var("PATH").unwrap_or_default();
    let prev_dir = env::current_dir().expect("cwd");
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    env::set_current_dir(&repo).expect("chdir");

    let inferred = default_repo(&repo).expect("default repo");

    env::set_current_dir(prev_dir).expect("restore cwd");
    unsafe {
        env::set_var("PATH", old_path);
    }
    assert_eq!(inferred, "owner/example");
}

#[test]
fn fetch_origin_main_with_fallback_reuses_local_origin_main_and_errors_when_missing() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-fetch-fallback");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
            &bin_dir.join("git"),
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2 $3\" = 'fetch origin main' ]; then\n  exit 1\nfi\nif [ \"$1 $2 $3 $4\" = 'rev-parse --verify --quiet origin/main' ]; then\n  if [ \"${HAS_ORIGIN_MAIN:-0}\" = '1' ]; then\n    exit 0\n  fi\n  exit 1\nfi\nexit 1\n",
        );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("HAS_ORIGIN_MAIN", "1");
    }
    fetch_origin_main_with_fallback().expect("should reuse local origin/main");

    unsafe {
        env::set_var("HAS_ORIGIN_MAIN", "0");
    }
    let err = fetch_origin_main_with_fallback().expect_err("missing origin/main should fail");
    unsafe {
        env::set_var("PATH", old_path);
        env::remove_var("HAS_ORIGIN_MAIN");
    }
    assert!(err
        .to_string()
        .contains("fetch origin main failed and origin/main is unavailable locally"));
}

#[test]
fn ensure_worktree_for_branch_rejects_branch_checked_out_elsewhere() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-worktree-conflict");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
            &bin_dir.join("git"),
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  cat <<'EOF'\nworktree /tmp/main\nHEAD deadbeef\nbranch refs/heads/main\n\nworktree /tmp/existing\nHEAD cafefood\nbranch refs/heads/codex/1153-test\nEOF\n  exit 0\nfi\nexit 1\n",
        );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    let err = ensure_worktree_for_branch(Path::new("/tmp/requested"), "codex/1153-test")
        .expect_err("conflicting worktree should fail");
    unsafe {
        env::set_var("PATH", old_path);
    }
    assert!(err.to_string().contains("already checked out in worktree"));
    assert!(err.to_string().contains("/tmp/existing"));
}

#[test]
fn ensure_local_branch_exists_covers_existing_remote_and_new_branch_paths() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-ensure-branch");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let git_log = temp.join("git.log");
    write_executable(
            &bin_dir.join("git"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\ncase \"$*\" in\n  'show-ref --verify --quiet refs/heads/codex/existing') exit 0 ;;\n  'show-ref --verify --quiet refs/heads/codex/remote') exit 1 ;;\n  'ls-remote --exit-code --heads origin codex/remote') exit 0 ;;\n  'branch --track codex/remote origin/codex/remote') exit 0 ;;\n  'show-ref --verify --quiet refs/heads/codex/new') exit 1 ;;\n  'ls-remote --exit-code --heads origin codex/new') exit 1 ;;\n  'branch codex/new origin/main') exit 0 ;;\n  *) exit 1 ;;\nesac\n",
                git_log.display()
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }

    ensure_local_branch_exists("codex/existing").expect("existing local branch");
    ensure_local_branch_exists("codex/remote").expect("remote tracking branch");
    ensure_local_branch_exists("codex/new").expect("new branch from origin/main");

    unsafe {
        env::set_var("PATH", old_path);
    }
    let log = fs::read_to_string(&git_log).expect("git log");
    assert!(log.contains("show-ref --verify --quiet refs/heads/codex/existing"));
    assert!(log.contains("branch --track codex/remote origin/codex/remote"));
    assert!(log.contains("branch codex/new origin/main"));
}

#[test]
fn issue_version_prefers_labels_and_falls_back_to_title() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-issue-version");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
            &bin_dir.join("gh"),
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2 $3 $4\" = 'issue view 1153 -R' ]; then\n  case \"${GH_MODE:-labels}\" in\n    labels) printf 'track:roadmap\\nversion:v0.86\\n' ;;\n    title) printf '[v0.89][WP-15] Demo issue\\n' ;;\n    *) printf 'track:roadmap\\n' ;;\n  esac\n  exit 0\nfi\nexit 1\n",
        );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("GH_MODE", "labels");
    }
    assert_eq!(
        issue_version(1153, "owner/repo").expect("labels"),
        Some("v0.86".to_string())
    );
    unsafe {
        env::set_var("GH_MODE", "title");
    }
    assert_eq!(
        issue_version(1153, "owner/repo").expect("title"),
        Some("v0.89".to_string())
    );
    unsafe {
        env::set_var("PATH", old_path);
        env::remove_var("GH_MODE");
    }
}

#[test]
fn current_pr_url_filters_empty_and_null_results() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-current-url");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
            &bin_dir.join("gh"),
            "#!/usr/bin/env bash\nset -euo pipefail\ncase \"${GH_PR_LIST_MODE:-url}\" in\n  null) printf 'null\\n' ;;\n  empty) printf '\\n' ;;\n  *) printf 'https://github.com/example/repo/pull/1\\n' ;;\nesac\n",
        );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("GH_PR_LIST_MODE", "url");
    }
    assert_eq!(
        current_pr_url("owner/repo", "codex/test").expect("url"),
        Some("https://github.com/example/repo/pull/1".to_string())
    );
    unsafe {
        env::set_var("GH_PR_LIST_MODE", "null");
    }
    assert_eq!(
        current_pr_url("owner/repo", "codex/test").expect("null"),
        None
    );
    unsafe {
        env::set_var("GH_PR_LIST_MODE", "empty");
    }
    assert_eq!(
        current_pr_url("owner/repo", "codex/test").expect("empty"),
        None
    );
    unsafe {
        env::set_var("PATH", old_path);
        env::remove_var("GH_PR_LIST_MODE");
    }
}

#[test]
fn branch_checked_out_worktree_path_returns_none_without_match() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-worktree-none");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
            &bin_dir.join("git"),
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  cat <<'EOF'\nworktree /tmp/main\nHEAD deadbeef\nbranch refs/heads/main\nEOF\n  exit 0\nfi\nexit 1\n",
        );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
    }
    assert_eq!(
        branch_checked_out_worktree_path("codex/missing").expect("none"),
        None
    );
    unsafe {
        env::set_var("PATH", old_path);
    }
}

#[test]
fn ensure_worktree_for_branch_reuses_matching_path_and_creates_new_one() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let temp = unique_temp_dir("adl-pr-worktree-reuse-create");
    let bin_dir = temp.join("bin");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    let git_log = temp.join("git.log");
    write_executable(
            &bin_dir.join("git"),
            &format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" >> '{}'\nif [ \"$1 $2 $3\" = 'worktree list --porcelain' ]; then\n  if [ \"${{WT_MODE:-reuse}}\" = 'reuse' ]; then\n    cat <<'EOF'\nworktree /tmp/reuse-me\nHEAD deadbeef\nbranch refs/heads/codex/reuse\nEOF\n    exit 0\n  fi\n  printf 'worktree /tmp/main\\nHEAD deadbeef\\nbranch refs/heads/main\\n'\n  exit 0\nfi\nif [ \"$1 $2 $3\" = 'worktree add /tmp/create-me' ]; then\n  mkdir -p /tmp/create-me\n  exit 0\nfi\nexit 1\n",
                git_log.display()
            ),
        );

    let old_path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));
        env::set_var("WT_MODE", "reuse");
    }
    ensure_worktree_for_branch(Path::new("/tmp/reuse-me"), "codex/reuse").expect("reuse");

    unsafe {
        env::set_var("WT_MODE", "create");
    }
    let create_path = Path::new("/tmp/create-me");
    let _ = fs::remove_dir_all(create_path);
    ensure_worktree_for_branch(create_path, "codex/create").expect("create");

    unsafe {
        env::set_var("PATH", old_path);
        env::remove_var("WT_MODE");
    }
    let log = fs::read_to_string(&git_log).expect("git log");
    assert!(log.contains("worktree add /tmp/create-me codex/create"));
}

#[test]
fn validate_issue_prompt_exists_rejects_missing_file() {
    let missing = unique_temp_dir("adl-pr-missing-prompt").join("missing.md");
    let err = validate_issue_prompt_exists(&missing).expect_err("missing prompt");
    assert!(err
        .to_string()
        .contains("missing canonical source issue prompt"));
}

#[test]
fn resolve_issue_prompt_path_accepts_legacy_issue_bodies_location() {
    let repo = unique_temp_dir("adl-pr-legacy-prompt-path");
    let issue_ref = IssueRef::new(1197, "v0.86".to_string(), "legacy-ready-source".to_string())
        .expect("issue ref");
    let legacy = issue_ref.legacy_issue_prompt_path(&repo);
    fs::create_dir_all(legacy.parent().expect("legacy parent")).expect("legacy dir");
    fs::write(
        &legacy,
        "---\nissue_card_schema: adl.issue.v1\n---\n\n# x\n",
    )
    .expect("legacy");

    let resolved = resolve_issue_prompt_path(&repo, &issue_ref).expect("resolved");
    assert_eq!(resolved, legacy);
}

#[test]
fn real_pr_start_rejects_missing_slug_or_empty_sanitized_title_in_no_fetch_mode() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-start-preconditions");
    init_git_repo(&repo);
    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");

    let missing_slug = real_pr(&[
        "start".to_string(),
        "1152".to_string(),
        "--no-fetch-issue".to_string(),
    ])
    .expect_err("missing slug should fail");
    assert!(missing_slug
        .to_string()
        .contains("start: --slug is required when --no-fetch-issue is set"));

    let bad_title = real_pr(&[
        "start".to_string(),
        "1152".to_string(),
        "--title".to_string(),
        "!!!".to_string(),
        "--no-fetch-issue".to_string(),
    ])
    .expect_err("empty sanitized title should fail");
    env::set_current_dir(prev_dir).expect("restore cwd");
    assert!(bad_title
        .to_string()
        .contains("start: --title produced empty slug after sanitization"));
}

#[test]
fn real_pr_ready_accepts_started_issue_when_output_branch_is_bootstrap_placeholder() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-ready-branch-placeholder");
    let origin = repo.join("origin.git");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);
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
    let issue_ref = IssueRef::new(1198, "v0.86", "ready-branch-placeholder").expect("issue ref");
    write_authored_issue_prompt(&repo, &issue_ref, "[v0.86][tools] Ready branch placeholder");

    real_pr(&[
        "start".to_string(),
        "1198".to_string(),
        "--slug".to_string(),
        "ready-branch-placeholder".to_string(),
        "--title".to_string(),
        "[v0.86][tools] Ready branch placeholder".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ])
    .expect("real_pr start");

    let root_output = issue_ref.task_bundle_output_path(&repo);
    let worktree = issue_ref.default_worktree_path(&repo, None);
    let root_sip = issue_ref.task_bundle_input_path(&repo);
    let wt_sip = issue_ref.task_bundle_input_path(&worktree);
    write_authored_sip(
        &root_sip,
        &issue_ref,
        "[v0.86][tools] Ready branch placeholder",
        "codex/1198-ready-branch-placeholder",
        &issue_ref.issue_prompt_path(&repo),
        &repo,
    );
    write_authored_sip(
        &wt_sip,
        &issue_ref,
        "[v0.86][tools] Ready branch placeholder",
        "codex/1198-ready-branch-placeholder",
        &issue_ref.issue_prompt_path(&worktree),
        &worktree,
    );
    let wt_output = issue_ref.task_bundle_output_path(&worktree);
    for path in [&root_output, &wt_output] {
        let text = fs::read_to_string(path).expect("sor");
        fs::write(
            path,
            text.replace(
                "Branch: codex/1198-ready-branch-placeholder",
                "Branch: TBD (run pr.sh start 1198)",
            ),
        )
        .expect("rewrite sor");
    }

    let ready = real_pr(&[
        "ready".to_string(),
        "1198".to_string(),
        "--slug".to_string(),
        "ready-branch-placeholder".to_string(),
        "--no-fetch-issue".to_string(),
        "--version".to_string(),
        "v0.86".to_string(),
    ]);

    env::set_current_dir(prev_dir).expect("restore cwd");
    ready.expect("ready should accept placeholder output branch");
}

#[test]
fn bootstrap_stub_reason_detects_issue_prompt_and_sip_templates() {
    let issue_prompt = "# x\n\n## Summary\n\nBootstrap-generated local source prompt for issue #1.\n\n## Goal\n\nTranslate the GitHub issue into the canonical local STP/task-bundle flow and refine this prompt before execution as needed.\n\n## Acceptance Criteria\n\n- something\n";
    assert_eq!(
        bootstrap_stub_reason(issue_prompt, PromptSurfaceKind::IssuePrompt),
        Some("bootstrap-generated issue prompt template text")
    );

    let sip = "# ADL Input Card\n\n## Goal\n\nReal goal\n\n## Acceptance Criteria\n\n- one\n\n## Required Outcome\n\n- State whether this issue must ship code, docs, tests, demo artifacts, or a combination.\n";
    assert_eq!(
        bootstrap_stub_reason(sip, PromptSurfaceKind::Sip),
        Some("unrefined SIP template guidance")
    );
}

#[cfg(unix)]
#[test]
fn ensure_git_metadata_writable_rejects_unwritable_git_dir() {
    use std::os::unix::fs::PermissionsExt;

    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-git-metadata-write");
    init_git_repo(&repo);
    let prev_dir = env::current_dir().expect("cwd");
    env::set_current_dir(&repo).expect("chdir");

    let git_dir = repo.join(".git");
    let refs_dir = git_dir.join("refs");
    let heads_dir = refs_dir.join("heads");
    let git_mode = fs::metadata(&git_dir)
        .expect("git metadata")
        .permissions()
        .mode();
    let refs_mode = fs::metadata(&refs_dir)
        .expect("refs metadata")
        .permissions()
        .mode();
    let heads_mode = fs::metadata(&heads_dir)
        .expect("heads metadata")
        .permissions()
        .mode();

    fs::set_permissions(&git_dir, fs::Permissions::from_mode(0o555)).expect("chmod git");
    fs::set_permissions(&refs_dir, fs::Permissions::from_mode(0o555)).expect("chmod refs");
    fs::set_permissions(&heads_dir, fs::Permissions::from_mode(0o555)).expect("chmod heads");

    let err = ensure_git_metadata_writable().expect_err("unwritable git dir should fail");

    fs::set_permissions(&heads_dir, fs::Permissions::from_mode(heads_mode)).expect("restore heads");
    fs::set_permissions(&refs_dir, fs::Permissions::from_mode(refs_mode)).expect("restore refs");
    fs::set_permissions(&git_dir, fs::Permissions::from_mode(git_mode)).expect("restore git");
    env::set_current_dir(prev_dir).expect("restore cwd");

    assert!(err.to_string().contains("git metadata directory"));
    assert!(err
        .to_string()
        .contains("restore write access to git metadata before rerunning"));
}

#[test]
fn ensure_primary_checkout_on_main_handles_dirty_and_clean_non_main_states() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-primary-main");
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
    fs::write(repo.join("README.md"), "hello\n").expect("write readme");
    assert!(Command::new("git")
        .args(["add", "README.md"])
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
        .args(["checkout", "-q", "-b", "codex/1153-test"])
        .current_dir(&repo)
        .status()
        .expect("git checkout")
        .success());

    fs::write(repo.join("README.md"), "dirty\n").expect("dirty write");
    let err = ensure_primary_checkout_on_main(&repo).expect_err("dirty non-main should fail");
    assert!(err.to_string().contains("with local changes"));

    assert!(Command::new("git")
        .args(["restore", "README.md"])
        .current_dir(&repo)
        .status()
        .expect("git restore")
        .success());
    ensure_primary_checkout_on_main(&repo).expect("clean non-main should switch");
    let branch = current_branch(&repo).expect("branch");
    assert_eq!(branch, "main");
}

#[test]
fn ensure_bootstrap_cards_creates_bundle_and_compat_links() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-bootstrap-cards");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let issue_ref =
        IssueRef::new(1153, "v0.86".to_string(), "rust-finish-test".to_string()).expect("ref");
    let source_path = issue_ref.issue_prompt_path(&repo);
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("mkdir");
    fs::write(
            &source_path,
            "---\ntitle: \"[v0.86][tools] Bootstrap cards\"\nlabels:\n  - \"track:roadmap\"\nissue_number: 1153\n---\n\n# Body\n",
        )
        .expect("write source");

    let (bundle_input, bundle_output) = ensure_bootstrap_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Bootstrap cards",
        "codex/1153-rust-finish-test",
        &source_path,
    )
    .expect("bootstrap cards");

    assert!(bundle_input.is_file());
    assert!(bundle_output.is_file());
    let compat_input = card_input_path(&resolve_cards_root(&repo, None), 1153);
    let compat_output = card_output_path(&resolve_cards_root(&repo, None), 1153);
    assert!(compat_input.symlink_metadata().is_ok());
    assert!(compat_output.symlink_metadata().is_ok());
    assert_eq!(
        field_line_value(&bundle_input, "Branch").expect("input branch"),
        "codex/1153-rust-finish-test"
    );
    assert_eq!(
        field_line_value(&bundle_output, "Status").expect("output status"),
        "IN_PROGRESS"
    );
    let bundle_input_text = fs::read_to_string(&bundle_input).expect("bundle input");
    assert_eq!(
        bootstrap_stub_reason(&bundle_input_text, PromptSurfaceKind::Sip),
        None
    );
}

#[test]
fn ensure_bootstrap_cards_rewrites_existing_bootstrap_stub_input_card() {
    let _guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let repo = unique_temp_dir("adl-pr-bootstrap-cards-rewrite");
    init_git_repo(&repo);
    copy_bootstrap_support_files(&repo);

    let issue_ref = IssueRef::new(
        1154,
        "v0.86".to_string(),
        "rewrite-bootstrap-sip".to_string(),
    )
    .expect("ref");
    let source_path = issue_ref.issue_prompt_path(&repo);
    fs::create_dir_all(source_path.parent().expect("source parent")).expect("mkdir");
    fs::write(
            &source_path,
            "---\ntitle: \"[v0.86][tools] Rewrite bootstrap SIP\"\nlabels:\n  - \"track:roadmap\"\nissue_number: 1154\n---\n\n## Summary\nx\n## Goal\nx\n## Required Outcome\nx\n## Deliverables\nx\n## Acceptance Criteria\nx\n## Repo Inputs\nx\n## Dependencies\nx\n## Demo Expectations\nx\n## Non-goals\nx\n## Issue-Graph Notes\nx\n## Notes\nx\n## Tooling Notes\nx\n",
        )
        .expect("write source");

    let bundle_input = issue_ref.task_bundle_input_path(&repo);
    fs::create_dir_all(bundle_input.parent().expect("input parent")).expect("mkdir");
    fs::write(
            &bundle_input,
            "# ADL Input Card\n\n## Goal\n\n\n## Required Outcome\n\n- State whether this issue must ship code, docs, tests, demo artifacts, or a combination.\n\n## Acceptance Criteria\n\n\n",
        )
        .expect("write stub input");

    let (repaired_input, _) = ensure_bootstrap_cards(
        &repo,
        &issue_ref,
        "[v0.86][tools] Rewrite bootstrap SIP",
        "codex/1154-rewrite-bootstrap-sip",
        &source_path,
    )
    .expect("bootstrap cards");

    let repaired_text = fs::read_to_string(repaired_input).expect("read repaired input");
    assert_eq!(
        bootstrap_stub_reason(&repaired_text, PromptSurfaceKind::Sip),
        None
    );
}
