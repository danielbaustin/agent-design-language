use super::*;
use crate::cli::tooling_cmd::common::{
    ensure_bool, is_repo_review_finding_title, mapping_bool, mapping_contains, mapping_mapping,
    mapping_seq_len, mapping_string, repo_review_finding_sort_key, resolve_issue_or_input_arg,
};
use serde_yaml::Mapping;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

struct TempRepo {
    path: PathBuf,
}

fn repo_root_for_tests() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("adl crate lives under repo root")
        .to_path_buf()
}

impl TempRepo {
    fn new(label: &str) -> Self {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before unix epoch")
            .as_nanos();
        let path = repo_root_for_tests()
            .join(".tmp/tooling_cmd_tests")
            .join(format!("{label}-{stamp}"));
        fs::create_dir_all(&path).expect("create temp repo root");
        Self { path }
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn write_rel(&self, rel: &str, contents: &str) -> PathBuf {
        let rel = rel.strip_prefix(".tmp/tooling_cmd_tests/").unwrap_or(rel);
        let path = self.path.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent dirs");
        }
        fs::write(&path, contents).expect("write temp file");
        path
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn valid_prompt_spec_yaml() -> String {
    r#"prompt_schema: adl.v1
actor:
  role: execution_agent
  name: codex
model:
  id: gpt-5-codex
  determinism_mode: stable
inputs:
  sections:
    - goal
    - required_outcome
    - acceptance_criteria
    - inputs
    - target_files_surfaces
    - validation_plan
    - demo_proof_requirements
    - constraints_policies
    - system_invariants
    - reviewer_checklist
    - non_goals_out_of_scope
    - notes_risks
    - instructions_to_agent
outputs:
  output_card: .adl/cards/1374/output_1374.md
  summary_style: concise_structured
constraints:
  include_system_invariants: true
  include_reviewer_checklist: true
  disallow_secrets: true
  disallow_absolute_host_paths: true
automation_hints:
  source_issue_prompt_required: true
  target_files_surfaces_recommended: true
  validation_plan_required: true
  required_outcome_type_supported: true
review_surfaces:
  - card_review_checklist.v1
  - card_review_output.v1
  - card_reviewer_gpt.v1.1
"#
    .to_string()
}

fn valid_input_card_text(issue: u32, out_rel: &str) -> String {
    format!(
        r#"# ADL Input Card

Task ID: issue-{issue:04}
Run ID: issue-{issue:04}
Version: v0.87
Title: tooling test
Branch: codex/{issue}-tooling-test

Context:
- Issue: https://github.com/danielbaustin/agent-design-language/issues/{issue}
- PR:
- Source Issue Prompt: .adl/v0.87/bodies/issue-{issue:04}-tooling-test.md
- Docs: docs/tooling/prompt-spec.md
- Other: none

## Prompt Spec
```yaml
{}
```
"#,
        valid_prompt_spec_yaml(),
    )
    .replace(".adl/cards/1374/output_1374.md", out_rel)
}

fn valid_stp_text() -> String {
    r#"---
issue_card_schema: adl.issue.v1
wp: "unassigned"
slug: "tooling-test"
title: "tooling test"
labels:
  - "track:roadmap"
issue_number: 1374
status: "draft"
action: "edit"
depends_on: []
milestone_sprint: "v0.87"
required_outcome_type:
  - "code"
repo_inputs: []
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "test"
pr_start:
  enabled: false
  slug: "tooling-test"
---

## Summary
test

## Goal
test

## Required Outcome
test

## Deliverables
test

## Acceptance Criteria
test

## Repo Inputs
test

## Dependencies
test

## Demo Expectations
test

## Non-goals
test

## Issue-Graph Notes
test

## Notes
test

## Tooling Notes
test
"#
    .to_string()
}

fn valid_sip_text(issue: u32, repo_root: &Path) -> String {
    let source = format!(".adl/v0.87/bodies/issue-{issue:04}-tooling-test.md");
    format!(
        r#"# ADL Input Card

Task ID: issue-{issue:04}
Run ID: issue-{issue:04}
Version: v0.87
Title: tooling test
Branch: codex/{issue}-tooling-test

Context:
- Issue: https://github.com/danielbaustin/agent-design-language/issues/{issue}
- PR:
- Source Issue Prompt: {source}
- Docs: docs/tooling/prompt-spec.md
- Other: none

## Agent Execution Rules
- Do not run `pr start`; the branch and worktree already exist.
- Do not delete or recreate cards.
- Do not switch branches unless explicitly instructed.
- Do not work on `main`.
- Only modify files required for the issue.
- Use repository-relative paths; avoid absolute host paths.
- Write the output record to the paired local task bundle `sor.md` path.
- If repository state is unexpected, stop and ask before attempting repository repair.

Task ID: issue-{issue:04}
Run ID: issue-{issue:04}
Version: v0.87
Title: tooling test
Branch: codex/{issue}-tooling-test
Status: DONE

## Goal
test

## Required Outcome
test

## Acceptance Criteria
test

## Inputs
test

## Target Files / Surfaces
test

## Validation Plan
test

## Demo / Proof Requirements
test

## Constraints / Policies
test

## System Invariants (must remain true)
test

## Reviewer Checklist (machine-readable hints)
test

## Non-goals / Out of scope
test

## Notes / Risks
test

## Instructions to the Agent
test

## Prompt Spec
```yaml
{}
```

## Execution
- Agent:
- Model:
- Provider:
- Start Time:
- End Time:
"#,
        valid_prompt_spec_yaml(),
    )
    .replace(
        ".adl/cards/1374/output_1374.md",
        ".adl/v0.87/tasks/issue-1374__tooling-test/sor.md",
    )
    .replace(".adl/v0.87/bodies/issue-1374-tooling-test.md", &source)
    .replace(
        "/Users/daniel/git/agent-design-language",
        &repo_root.to_string_lossy(),
    )
}

fn valid_sor_text() -> String {
    r#"# tooling-test

Task ID: issue-1374
Run ID: issue-1374
Version: v0.87
Title: tooling test
Branch: codex/1374-tooling-test
Status: DONE

## Summary
Done.

## Artifacts produced
- `docs/tooling/prompt-spec.md`

## Actions taken
- validated tool contracts

## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
  - `docs/tooling/prompt-spec.md`
- Worktree-only paths remaining: none
- Integration state: merged
- Verification scope: main_repo
- Integration method used: direct write in main repo
- Verification performed:
  - `git status`
  - `ls docs/tooling/prompt-spec.md`
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `cargo test` — exercised tooling validators
- Results:
  - PASS

## Execution
- Actor: codex
- Model: gpt-5.4-mini
- Provider: openai
- Start Time: 2026-04-07T19:00:00Z
- End Time: 2026-04-07T19:05:00Z

## Verification Summary
```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "cargo test"
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
- Determinism tests executed: `cargo test`
- Fixtures or scripts used: inline markdown fixtures
- Replay verification (same inputs -> same artifacts/order): stable
- Ordering guarantees (sorting / tie-break rules used): stable
- Artifact stability notes: no host-path leakage

## Security / Privacy Checks
- Secret leakage scan performed: yes
- Prompt / tool argument redaction verified: yes
- Absolute path leakage check: passed
- Sandbox / policy invariants preserved: yes

## Replay Artifacts
- Trace bundle path(s): `.adl/runs/tooling-test/trace.jsonl`
- Run artifact root: `.adl/runs/tooling-test`
- Replay command used for verification: `cargo test`
- Replay result: PASS

## Artifact Verification
- Primary proof surface: `docs/tooling/prompt-spec.md`
- Required artifacts present: yes
- Artifact schema/version checks: passed
- Hash/byte-stability checks: not run
- Missing/optional artifacts and rationale: none

## Decisions / Deviations
- none

## Follow-ups / Deferred work
- none
"#
    .to_string()
}

fn valid_review_markdown() -> String {
    r#"# Repo Review

## Metadata
- Review Type: targeted
- Subject: tooling bundle
- Reviewer: codex

## Scope
- Reviewed: docs/tooling/prompt-spec.md
- Not Reviewed: unrelated areas
- Review Mode: bounded
- Gate: informational

## Findings
1. [P1] Example finding.
2. [P3] Example finding.

## System-Level Assessment
The bundle is coherent.

## Recommended Action Plan
- Fix now: nothing
- Fix before milestone closeout: nothing
- Defer: nothing

## Follow-ups / Deferred Work
None.

## Final Assessment
Acceptable.
"#
    .to_string()
}

fn valid_review_output_yaml(temp_root: &Path) -> String {
    r#"review_format_version: card_review_output.v1
decision: PASS
review_target:
  input_card_path: .tmp/tooling_cmd_tests/{prefix}/input.md
  output_card_path: .tmp/tooling_cmd_tests/{prefix}/output.md
findings:
  - evidence_state: contradicted
    evidence:
      - path:.tmp/tooling_cmd_tests/{prefix}/input.md
      - artifact:.tmp/tooling_cmd_tests/{prefix}/output.md
validation_checks:
  validation_result: PASS
  commands:
    - cargo test --quiet
security_privacy_checks:
  absolute_host_paths_present: false
"#
    .replace(
        "{prefix}",
        temp_root
            .file_name()
            .expect("temp repo label")
            .to_str()
            .expect("utf-8 temp repo label"),
    )
}

fn prompt_spec_without_sections(
    include_system_invariants: Option<bool>,
    include_reviewer_checklist: Option<bool>,
) -> String {
    let mut lines = vec![
        "prompt_schema: adl.v1".to_string(),
        "actor:".to_string(),
        "  role: execution_agent".to_string(),
        "  name: codex".to_string(),
        "model:".to_string(),
        "  id: gpt-5-codex".to_string(),
        "  determinism_mode: stable".to_string(),
        "inputs: {}".to_string(),
        "outputs:".to_string(),
        "  output_card: .adl/cards/1374/output_1374.md".to_string(),
        "  summary_style: concise_structured".to_string(),
        "constraints:".to_string(),
        format!(
            "  include_system_invariants: {}",
            include_system_invariants.unwrap_or(true)
        ),
        format!(
            "  include_reviewer_checklist: {}",
            include_reviewer_checklist.unwrap_or(true)
        ),
        "  disallow_secrets: true".to_string(),
        "  disallow_absolute_host_paths: true".to_string(),
        "automation_hints:".to_string(),
        "  source_issue_prompt_required: true".to_string(),
        "  target_files_surfaces_recommended: true".to_string(),
        "  validation_plan_required: true".to_string(),
        "  required_outcome_type_supported: true".to_string(),
        "review_surfaces:".to_string(),
        "  - card_review_checklist.v1".to_string(),
        "  - card_review_output.v1".to_string(),
        "  - card_reviewer_gpt.v1.1".to_string(),
    ];
    lines.push(String::new());
    lines.join("\n")
}

fn valid_input_card_with_prompt_spec(issue: u32, out_rel: &str, prompt_spec_yaml: &str) -> String {
    format!(
        r#"# ADL Input Card

Task ID: issue-{issue:04}
Run ID: issue-{issue:04}
Version: v0.87
Title: tooling test
Branch: codex/{issue}-tooling-test

## Goal
ship it

## Required Outcome
test

## Acceptance Criteria
- keep behavior stable

## Inputs
- card

## Target Files / Surfaces
- adl/src/cli/tooling_cmd.rs

## Validation Plan
- cargo test

## Demo / Proof Requirements
- none

## Constraints / Policies
- no scope creep

## System Invariants (must remain true)
- deterministic

## Reviewer Checklist (machine-readable hints)
- check evidence

## Non-goals / Out of scope
- no product changes

## Notes / Risks
- low

## Instructions to the Agent
- stay focused

## Prompt Spec
```yaml
{prompt_spec_yaml}
```
"#
    )
    .replace(".adl/cards/1374/output_1374.md", out_rel)
}

#[test]
fn helper_validators_cover_expected_shapes() {
    assert!(is_repo_relative("docs/tooling/prompt-spec.md"));
    assert!(!is_repo_relative("/Users/daniel/file"));
    assert!(valid_task_id("issue-1374"));
    assert!(!valid_task_id("issue-13x4"));
    assert!(valid_version("v0.87"));
    assert!(valid_version("v0.87.1"));
    assert!(!valid_version("0.87"));
    assert!(valid_branch("codex/1374-demo-test"));
    assert!(!valid_branch("main"));
    assert!(valid_github_issue_url(
        "https://github.com/danielbaustin/agent-design-language/issues/1374"
    ));
    assert!(valid_github_pr_url(
        "https://github.com/danielbaustin/agent-design-language/pull/1394"
    ));
    assert!(valid_reference("docs/tooling/prompt-spec.md"));
    assert!(valid_reference("https://example.com/doc"));
    assert!(valid_iso8601_datetime("2026-04-07T19:00:00Z"));
    assert!(!valid_iso8601_datetime("2026-04-07 19:00:00"));
    assert!(is_normalized_slug("v0-87-tools-demo"));
    assert!(!is_normalized_slug("BadSlug"));
    assert_eq!(pointer_sort_key("path:foo"), (0, "path:foo".to_string()));
    assert_eq!(
        pointer_sort_key("command:foo"),
        (1, "command:foo".to_string())
    );
    assert_eq!(
        pointer_sort_key("artifact:foo"),
        (3, "artifact:foo".to_string())
    );

    let checks = vec![
        ReviewCheck {
            id: "1".to_string(),
            domain: "d".to_string(),
            severity: "high".to_string(),
            status: "FAIL".to_string(),
            title: "a".to_string(),
            evidence: vec![],
            notes: "".to_string(),
        },
        ReviewCheck {
            id: "2".to_string(),
            domain: "d".to_string(),
            severity: "low".to_string(),
            status: "PASS".to_string(),
            title: "b".to_string(),
            evidence: vec![],
            notes: "".to_string(),
        },
    ];
    assert_eq!(decision_for(&checks), "MAJOR_ISSUES");

    let mut ordered = vec!["path:a".to_string(), "artifact:b".to_string()];
    assert!(ensure_sorted_pointers(&ordered, "evidence").is_ok());
    ordered.reverse();
    assert!(ensure_sorted_pointers(&ordered, "evidence").is_err());
}

#[test]
fn common_helpers_cover_argument_and_content_guards() {
    let repo = TempRepo::new("common");
    let clean = repo.write_rel("clean.txt", "safe text");
    let secret = repo.write_rel("secret.txt", "token gho_1234567890");
    let host_path = repo.write_rel("host-path.txt", "/Users/daniel/secrets.txt");

    assert_eq!(
        resolve_issue_or_input_arg(&["--input".to_string(), clean.to_string_lossy().to_string(),])
            .expect("input path should resolve"),
        clean
    );
    assert!(resolve_issue_or_input_arg(&["--help".to_string()])
        .unwrap()
        .as_os_str()
        .is_empty());
    assert!(resolve_issue_or_input_arg(&[]).is_err());
    assert!(resolve_issue_or_input_arg(&[
        "--issue".to_string(),
        "12".to_string(),
        "--input".to_string(),
        clean.to_string_lossy().to_string(),
    ])
    .is_err());
    assert!(normalize_issue("abc").is_err());

    let absolute_clean = absolutize(&clean).expect("absolute path");
    assert!(absolute_clean.is_absolute());
    assert_eq!(
        repo_relative_display(repo.path(), &clean).expect("repo relative display"),
        "clean.txt"
    );

    ensure_file(&clean, "clean file").expect("file should exist");
    assert!(ensure_file(&repo.path().join("missing.txt"), "missing file").is_err());
    ensure_no_disallowed_content(&clean, "clean file").expect("safe content");
    assert!(ensure_no_disallowed_content(&secret, "secret file").is_err());
    assert!(ensure_no_disallowed_content(&host_path, "host path file").is_err());
    ensure_no_absolute_host_path(&clean, "sip").expect("no absolute paths");
    assert!(ensure_no_absolute_host_path(&host_path, "sip").is_err());

    assert!(contains_secret_like_token("prefix sk-abcdefgh"));
    assert!(contains_secret_like_token("ghs_1234567890"));
    assert!(!contains_secret_like_token("mask sk_short"));
    assert!(contains_absolute_host_path_in_text("/tmp/example"));
    assert!(!contains_absolute_host_path_in_text("relative/path"));

    assert!(is_repo_review_finding_title("1. [P2] Useful finding"));
    assert!(!is_repo_review_finding_title("- [P2] Useful finding"));
    assert_eq!(
        repo_review_finding_sort_key("2. [P3] later"),
        (3, "2. [P3] later".to_string())
    );
}

#[test]
fn common_mapping_helpers_cover_yaml_access_patterns() {
    let mapping: Mapping = serde_yaml::from_str(
        r#"
flag: true
name: demo
count: 7
nested:
  key: value
items:
  - one
  - two
"#,
    )
    .expect("mapping yaml");

    assert!(mapping_contains(&mapping, "flag"));
    assert_eq!(mapping_string(&mapping, "name"), Some("demo".to_string()));
    assert_eq!(mapping_string(&mapping, "count"), Some("7".to_string()));
    assert_eq!(mapping_bool(&mapping, "flag"), Some(true));
    assert_eq!(mapping_seq_len(&mapping, "items"), 2);
    assert!(mapping_mapping(&mapping, "nested").is_ok());
    assert!(mapping_mapping(&mapping, "missing").is_err());
    assert!(ensure_bool(&mapping, "flag", "flag must be bool").expect("bool key"));
    assert!(ensure_bool(&mapping, "missing", "flag must be bool").is_err());
}

#[test]
fn prompt_spec_validation_accepts_canonical_spec() {
    let spec = valid_prompt_spec_yaml();
    validate_prompt_spec(&spec).expect("canonical prompt spec should validate");
    assert_eq!(
        prompt_spec_sections(&spec),
        vec![
            "goal",
            "required_outcome",
            "acceptance_criteria",
            "inputs",
            "target_files_surfaces",
            "validation_plan",
            "demo_proof_requirements",
            "constraints_policies",
            "system_invariants",
            "reviewer_checklist",
            "non_goals_out_of_scope",
            "notes_risks",
            "instructions_to_agent",
        ]
    );
    assert_eq!(
        prompt_spec_bool(&spec, "include_system_invariants"),
        Some(true)
    );
    assert_eq!(
        prompt_spec_bool(&spec, "required_outcome_type_supported"),
        Some(true)
    );
    let extracted = extract_prompt_spec_yaml(&format!(
        "# Heading\n\n## Prompt Spec\n```yaml\n{}\n```\n",
        spec
    ))
    .expect("prompt spec block should extract");
    assert!(extracted.contains("prompt_schema: adl.v1"));
}

#[test]
fn tooling_dispatch_and_help_paths_cover_public_entrypoint() {
    let repo = TempRepo::new("dispatch");
    let input = repo.write_rel(
        ".tmp/tooling_cmd_tests/input.md",
        &valid_input_card_text(1374, ".tmp/tooling_cmd_tests/output.md"),
    );
    let output = repo.write_rel(".tmp/tooling_cmd_tests/output.md", &valid_sor_text());
    let review = repo.write_rel(".tmp/tooling_cmd_tests/review.md", &valid_review_markdown());
    let review_output = repo.write_rel(
        ".tmp/tooling_cmd_tests/review-output.yaml",
        &valid_review_output_yaml(repo.path()),
    );
    let stp = repo.write_rel(".tmp/tooling_cmd_tests/stp.md", &valid_stp_text());
    let sip = repo.write_rel(
        ".tmp/tooling_cmd_tests/sip.md",
        &valid_sip_text(1374, repo.path()),
    );
    let prompt_out = repo.path().join("prompt.txt");

    assert!(real_tooling(&[]).is_err());
    real_tooling(&["help".to_string()]).expect("help should succeed");
    assert!(real_tooling(&["unknown".to_string()]).is_err());

    real_tooling(&[
        "card-prompt".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--out".to_string(),
        prompt_out.to_string_lossy().to_string(),
    ])
    .expect("card-prompt dispatch should succeed");
    assert!(prompt_out.is_file());

    real_tooling(&[
        "lint-prompt-spec".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
    ])
    .expect("lint dispatch should succeed");
    real_tooling(&[
        "validate-structured-prompt".to_string(),
        "--type".to_string(),
        "stp".to_string(),
        "--input".to_string(),
        stp.to_string_lossy().to_string(),
    ])
    .expect("stp dispatch should succeed");
    real_tooling(&[
        "validate-structured-prompt".to_string(),
        "--type".to_string(),
        "sip".to_string(),
        "--input".to_string(),
        sip.to_string_lossy().to_string(),
    ])
    .expect("sip dispatch should succeed");
    real_tooling(&[
        "review-card-surface".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--output".to_string(),
        output.to_string_lossy().to_string(),
    ])
    .expect("review surface dispatch should succeed");
    real_tooling(&[
        "verify-review-output-provenance".to_string(),
        "--review".to_string(),
        review_output.to_string_lossy().to_string(),
    ])
    .expect("review output provenance dispatch should succeed");
    real_tooling(&[
        "verify-repo-review-contract".to_string(),
        "--review".to_string(),
        review.to_string_lossy().to_string(),
    ])
    .expect("repo review contract dispatch should succeed");
}

#[test]
fn card_prompt_covers_help_errors_and_fallback_rendering() {
    let repo = TempRepo::new("card-prompt");
    let prompt_out = repo.path().join("rendered.txt");
    let fallback_input = repo.write_rel(
        ".tmp/tooling_cmd_tests/fallback-input.md",
        &valid_input_card_with_prompt_spec(
            1402,
            ".tmp/tooling_cmd_tests/rendered.txt",
            &prompt_spec_without_sections(Some(false), Some(false)),
        ),
    );
    real_card_prompt(&["--help".to_string()]).expect("help should succeed");
    assert!(real_card_prompt(&[]).is_err());
    assert!(real_card_prompt(&["--issue".to_string()]).is_err());
    assert!(real_card_prompt(&["--input".to_string()]).is_err());
    assert!(real_card_prompt(&["--out".to_string()]).is_err());
    assert!(real_card_prompt(&["--bogus".to_string()]).is_err());
    assert!(real_card_prompt(&[
        "--issue".to_string(),
        "1402".to_string(),
        "--input".to_string(),
        fallback_input.to_string_lossy().to_string(),
    ])
    .is_err());
    assert!(real_card_prompt(&[
        "--input".to_string(),
        repo.path().join("missing.md").to_string_lossy().to_string(),
    ])
    .is_err());

    real_card_prompt(&[
        "--input".to_string(),
        fallback_input.to_string_lossy().to_string(),
        "--out".to_string(),
        prompt_out.to_string_lossy().to_string(),
    ])
    .expect("render fallback prompt");
    let rendered = fs::read_to_string(&prompt_out).expect("rendered prompt text");
    assert!(rendered.contains("Work Prompt"));
    assert!(rendered.contains("Input Card:"));
    assert!(rendered.contains("Goal\nship it"));
    assert!(rendered.contains("Instructions to the Agent\n- stay focused"));
    assert!(!rendered.contains("System Invariants (must remain true)"));
    assert!(!rendered.contains("Reviewer Checklist (machine-readable hints)"));
}

#[test]
fn structured_prompt_validators_accept_canonical_cards() {
    let stp = valid_stp_text();
    let sip = valid_sip_text(1374, Path::new("/Users/daniel/git/agent-design-language"));
    let sor = valid_sor_text();

    validate_stp_text(&stp).expect("canonical STP should validate");
    validate_sip_text(&sip, Path::new("sip.md")).expect("canonical SIP should validate");
    validate_sor_text(&sor, Some("completed")).expect("canonical SOR should validate");

    assert!(markdown_has_heading(&stp, "Summary"));
    assert!(markdown_has_heading(&sip, "Validation Plan"));
    assert!(markdown_has_heading(&sor, "Artifacts produced"));
    assert_eq!(
        markdown_field(&stp, "slug").map(|value| value.trim_matches('"').to_string()),
        Some("tooling-test".to_string())
    );
    assert_eq!(
        markdown_block_field(&sip, "Context", "Issue"),
        Some("https://github.com/danielbaustin/agent-design-language/issues/1374".to_string())
    );
    assert_eq!(
        markdown_section_body(&sor, "Summary").unwrap().trim(),
        "Done."
    );
    assert!(split_front_matter(&stp).is_ok());
}

#[test]
fn review_commands_validate_and_render_expected_surfaces() {
    let repo = TempRepo::new("review");
    let input = repo.write_rel(
        ".tmp/tooling_cmd_tests/input.md",
        &valid_input_card_text(1374, ".tmp/tooling_cmd_tests/output.md"),
    );
    let output = repo.write_rel(".tmp/tooling_cmd_tests/output.md", &valid_sor_text());
    let review = repo.write_rel(".tmp/tooling_cmd_tests/review.md", &valid_review_markdown());
    let review_output = repo.write_rel(
        ".tmp/tooling_cmd_tests/review-output.yaml",
        &valid_review_output_yaml(repo.path()),
    );

    assert!(
        real_card_prompt(&["--input".to_string(), input.to_string_lossy().to_string()]).is_ok()
    );
    assert!(
        real_lint_prompt_spec(&["--input".to_string(), input.to_string_lossy().to_string()])
            .is_ok()
    );
    assert!(real_review_card_surface(&[
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--output".to_string(),
        output.to_string_lossy().to_string(),
    ])
    .is_ok());

    let input_ref = display_card_ref(&input).expect("display reference should be derived");
    assert!(input_ref.ends_with("input.md"));

    assert!(real_verify_review_output_provenance(&[
        "--review".to_string(),
        review_output.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(real_verify_repo_review_contract(&[
        "--review".to_string(),
        review.to_string_lossy().to_string(),
    ])
    .is_ok());
}

#[test]
fn validate_structured_prompt_accepts_all_three_prompt_types() {
    let repo = TempRepo::new("structured");
    let stp = repo.write_rel(".tmp/tooling_cmd_tests/stp.md", &valid_stp_text());
    let sip = repo.write_rel(
        ".tmp/tooling_cmd_tests/sip.md",
        &valid_sip_text(1374, repo.path()),
    );
    let sor = repo.write_rel(".tmp/tooling_cmd_tests/sor.md", &valid_sor_text());

    assert!(real_validate_structured_prompt(&[
        "--type".to_string(),
        "stp".to_string(),
        "--input".to_string(),
        stp.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(real_validate_structured_prompt(&[
        "--type".to_string(),
        "sip".to_string(),
        "--input".to_string(),
        sip.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(real_validate_structured_prompt(&[
        "--type".to_string(),
        "sor".to_string(),
        "--input".to_string(),
        sor.to_string_lossy().to_string(),
        "--phase".to_string(),
        "completed".to_string(),
    ])
    .is_ok());
}

#[test]
fn tooling_dispatch_accepts_help_and_rejects_unknown_subcommands() {
    assert!(real_tooling(&["help".to_string()]).is_ok());
    assert!(real_tooling(&["--help".to_string()]).is_ok());
    assert!(real_tooling(&[]).is_err());
    assert!(real_tooling(&["unknown-subcommand".to_string()]).is_err());
}

#[test]
fn tooling_dispatch_routes_public_subcommands() {
    let repo = TempRepo::new("dispatch");
    let input = repo.write_rel(
        ".tmp/tooling_cmd_tests/input.md",
        &valid_input_card_text(1374, ".tmp/tooling_cmd_tests/output.md"),
    );
    let output = repo.write_rel(".tmp/tooling_cmd_tests/output.md", &valid_sor_text());
    let review = repo.write_rel(".tmp/tooling_cmd_tests/review.md", &valid_review_markdown());
    let review_output = repo.write_rel(
        ".tmp/tooling_cmd_tests/review-output.yaml",
        &valid_review_output_yaml(repo.path()),
    );
    let prompt_out = repo.path().join("prompt.txt");

    assert!(real_tooling(&[
        "card-prompt".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--out".to_string(),
        prompt_out.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(prompt_out.is_file());

    assert!(real_tooling(&[
        "lint-prompt-spec".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(real_tooling(&[
        "validate-structured-prompt".to_string(),
        "--type".to_string(),
        "sor".to_string(),
        "--input".to_string(),
        output.to_string_lossy().to_string(),
        "--phase".to_string(),
        "completed".to_string(),
    ])
    .is_ok());
    assert!(real_tooling(&[
        "review-card-surface".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--output".to_string(),
        output.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(real_tooling(&[
        "verify-review-output-provenance".to_string(),
        "--review".to_string(),
        review_output.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(real_tooling(&[
        "verify-repo-review-contract".to_string(),
        "--review".to_string(),
        review.to_string_lossy().to_string(),
    ])
    .is_ok());
}

#[test]
fn card_prompt_covers_help_and_argument_validation_branches() {
    let repo = TempRepo::new("card-prompt");
    let input = repo.write_rel(
        ".tmp/tooling_cmd_tests/input.md",
        &valid_input_card_text(1374, ".tmp/tooling_cmd_tests/output.md"),
    );
    let out = repo.path().join("prompt.txt");

    assert!(real_card_prompt(&["--help".to_string()]).is_ok());
    assert!(real_card_prompt(&[
        "--input".to_string(),
        input.to_string_lossy().to_string(),
        "--out".to_string(),
        out.to_string_lossy().to_string(),
    ])
    .is_ok());
    assert!(out.is_file());

    assert!(real_card_prompt(&[]).is_err());
    assert!(real_card_prompt(&[
        "--issue".to_string(),
        "1374".to_string(),
        "--input".to_string(),
        input.to_string_lossy().to_string(),
    ])
    .is_err());
    assert!(real_card_prompt(&["--issue".to_string()]).is_err());
    assert!(real_card_prompt(&["--input".to_string()]).is_err());
    assert!(real_card_prompt(&["--out".to_string()]).is_err());
    assert!(real_card_prompt(&["--bogus".to_string()]).is_err());
}

#[test]
fn common_helpers_cover_safety_and_path_branches() {
    let root = repo_root_for_tests();
    let nested = root.join("adl/src/cli/tooling_cmd.rs");

    assert!(contains_absolute_host_path_in_text(
        "/Users/example/project"
    ));
    assert!(!contains_absolute_host_path_in_text("relative/path"));
    assert!(contains_secret_like_token("prefix sk-abcdefgh suffix"));
    assert!(contains_secret_like_token("ghp_exampletoken"));
    assert!(!contains_secret_like_token("sk-short"));

    assert_eq!(normalize_issue("1402").expect("issue"), 1402);
    assert!(normalize_issue("14x2").is_err());

    assert_eq!(
        repo_relative_display(&root, &nested).expect("repo relative"),
        "adl/src/cli/tooling_cmd.rs"
    );
    assert!(absolutize(Path::new("adl/src/cli/tooling_cmd.rs"))
        .expect("absolutize")
        .is_absolute());
    assert!(ensure_file(&nested, "tooling").is_ok());
    assert!(ensure_file(&root.join("adl/src/cli/missing.rs"), "missing").is_err());

    let repo = TempRepo::new("common");
    let clean = repo.write_rel("clean.md", "no secrets here\nrelative/path\n");
    let secret = repo.write_rel("secret.md", "token ghp_secretvalue\n");
    let abs = repo.write_rel("abs.md", "/Users/daniel/private\n");

    assert!(ensure_no_disallowed_content(&clean, "clean").is_ok());
    assert!(ensure_no_disallowed_content(&secret, "secret").is_err());
    assert!(ensure_no_absolute_host_path(&clean, "sip").is_ok());
    assert!(ensure_no_absolute_host_path(&abs, "sip").is_err());
}
