use super::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub(super) struct TempRepo {
    path: PathBuf,
}

pub(super) fn repo_root_for_tests() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("adl crate lives under repo root")
        .to_path_buf()
}

impl TempRepo {
    pub(super) fn new(label: &str) -> Self {
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

    pub(super) fn path(&self) -> &PathBuf {
        &self.path
    }

    pub(super) fn write_rel(&self, rel: &str, contents: &str) -> PathBuf {
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

pub(super) fn write_runtime_review_fixture(repo: &TempRepo) -> PathBuf {
    let root = repo_root_for_tests();
    let review_root = repo.path().join("runtime-review");
    fs::create_dir_all(&review_root).expect("review root");

    let review_readme = repo.write_rel(
        "runtime-review/README.md",
        "# D8\n\nPrimary proof surface:\n- `demo_manifest.json`\n\nSecondary proof surfaces:\n- `operator`\n\nReviewer walkthrough:\n- Review D6 first for the canonical operator entrypoint.\n- Then inspect D7 for persistence, pause-state, and continuity evidence.\n",
    );
    let operator_readme = repo.write_rel("operator/README.md", "# operator\n");
    let operator_summary = repo.write_rel("operator/runtime/runs/demo/run_summary.json", "{}\n");
    let operator_status = repo.write_rel("operator/runtime/runs/demo/run_status.json", "{}\n");
    let operator_trace = repo.write_rel("operator/runtime/runs/demo/logs/trace_v1.json", "{}\n");
    let runtime_state_readme = repo.write_rel("runtime_state/README.md", "# runtime state\n");
    let paused_status = repo.write_rel("runtime_state/runtime/runs/paused/run_status.json", "{}\n");
    let paused_pause = repo.write_rel("runtime_state/runtime/runs/paused/pause_state.json", "{}\n");
    let paused_trace = repo.write_rel(
        "runtime_state/runtime/runs/paused/logs/trace_v1.json",
        "{}\n",
    );
    let complete_status = repo.write_rel(
        "runtime_state/runtime/runs/complete/run_status.json",
        "{}\n",
    );
    let complete_trace = repo.write_rel(
        "runtime_state/runtime/runs/complete/logs/trace_v1.json",
        "{}\n",
    );
    let runtime_marker = repo.write_rel("runtime_state/runtime/runtime_environment.json", "{}\n");
    let manifest_path = review_root.join("demo_manifest.json");

    let rel = |path: &Path| repo_relative_display(&root, path).expect("repo relative fixture path");
    let manifest = format!(
        concat!(
            "{{\n",
            "  \"review_surface_version\": \"adl.runtime_review_surface.v1\",\n",
            "  \"milestone\": \"v0.87.1\",\n",
            "  \"demo_id\": \"D8\",\n",
            "  \"review_root\": \"{review_root}\",\n",
            "  \"review_readme\": \"{review_readme}\",\n",
            "  \"primary_proof_surface\": \"{manifest_path}\",\n",
            "  \"demo_packages\": [\n",
            "    {{\n",
            "      \"demo_id\": \"D6\",\n",
            "      \"title\": \"Operator Invocation Surface\",\n",
            "      \"review_readme\": \"{operator_readme}\",\n",
            "      \"primary_proof_surface\": \"{operator_summary}\",\n",
            "      \"secondary_proof_surfaces\": [\n",
            "        \"{operator_status}\",\n",
            "        \"{operator_trace}\"\n",
            "      ]\n",
            "    }},\n",
            "    {{\n",
            "      \"demo_id\": \"D7\",\n",
            "      \"title\": \"Runtime State / Persistence Discipline\",\n",
            "      \"review_readme\": \"{runtime_state_readme}\",\n",
            "      \"primary_proof_surface\": \"{paused_status}\",\n",
            "      \"secondary_proof_surfaces\": [\n",
            "        \"{paused_pause}\",\n",
            "        \"{paused_trace}\",\n",
            "        \"{complete_status}\",\n",
            "        \"{complete_trace}\",\n",
            "        \"{runtime_marker}\"\n",
            "      ]\n",
            "    }}\n",
            "  ]\n",
            "}}\n"
        ),
        review_root = rel(&review_root),
        review_readme = rel(&review_readme),
        manifest_path = rel(&manifest_path),
        operator_readme = rel(&operator_readme),
        operator_summary = rel(&operator_summary),
        operator_status = rel(&operator_status),
        operator_trace = rel(&operator_trace),
        runtime_state_readme = rel(&runtime_state_readme),
        paused_status = rel(&paused_status),
        paused_pause = rel(&paused_pause),
        paused_trace = rel(&paused_trace),
        complete_status = rel(&complete_status),
        complete_trace = rel(&complete_trace),
        runtime_marker = rel(&runtime_marker),
    );
    fs::write(&manifest_path, manifest).expect("write runtime review manifest");
    review_root
}

pub(super) fn valid_prompt_spec_yaml() -> String {
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

pub(super) fn valid_input_card_text(issue: u32, out_rel: &str) -> String {
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

pub(super) fn valid_stp_text() -> String {
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

pub(super) fn valid_sip_text(issue: u32, repo_root: &Path) -> String {
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

pub(super) fn valid_sor_text() -> String {
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

pub(super) fn valid_review_markdown() -> String {
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

pub(super) fn valid_review_output_yaml(temp_root: &Path) -> String {
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

pub(super) fn prompt_spec_without_sections(
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

pub(super) fn valid_input_card_with_prompt_spec(
    issue: u32,
    out_rel: &str,
    prompt_spec_yaml: &str,
) -> String {
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
