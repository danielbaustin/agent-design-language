# ADL Input Card

Task ID: issue-2704
Run ID: issue-2704
Version: v0.90.5
Title: [v0.90.5][records] Repair missing/duplicate task-bundle records: #2683 and #2699 duplicate residue
Branch: codex/2704-v0-90-5-records-repair-missing-duplicate-task-bundle-records

Context:
- Issue: https://github.com/danielbaustin/agent-design-language/issues/2704
- PR:
- Source Issue Prompt: .adl/v0.90.5/bodies/issue-2704-v0-90-5-records-repair-missing-duplicate-task-bundle-records.md
- Docs: none
- Other: bound worktree `.worktrees/adl-wp-2704`

## Agent Execution Rules
- This issue is not started yet; do not assume a branch or worktree already exists.
- Do not run `pr start`; use the current issue-mode `pr run` flow only if execution later becomes necessary.
- Do not delete or recreate cards.
- Do not switch branches unless explicitly instructed.
- Do not work on `main`.
- Only modify files required for the issue.
- Use repository-relative paths; avoid absolute host paths.
- Write the output record to the paired local task bundle `sor.md` path.
- If repository state is unexpected, stop and ask before attempting repository repair.

## Prompt Spec
```yaml
prompt_schema: adl.v1
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
  output_card: .adl/v0.90.5/tasks/issue-2704__v0-90-5-records-repair-missing-duplicate-task-bundle-records/sor.md
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
```

Reviewer protocol IDs are versioned and order-sensitive:
1. checklist contract
2. output artifact contract
3. reviewer behavior contract

Prompt Spec contract notes:
- Supported section IDs and machine-readable field semantics are defined in `docs/tooling/prompt-spec.md`.
- Missing required Prompt Spec keys or required boolean `automation_hints` fields should fail lint.
- Prompt generation must preserve declared section order rather than heuristic extraction.

Execution:
- Agent:
- Provider:
- Tools allowed:
- Sandbox / approvals:
- Source issue-prompt slug:
- Required outcome type:
- Demo required:

## Goal

Execute the bounded records-surgery pass for `#2704` so the named local `#2683` and `#2699` records surfaces become truthful and guard-clean.

## Required Outcome

- Keep the linked issue prompt, input card, and output record aligned with the actual records-only scope for this issue.
- Update only the named local `.adl` records surfaces and preserve truthful run-bound lifecycle state while the repair is in progress.

## Acceptance Criteria

- The linked source issue prompt is concrete and reviewable.
- The card bundle reflects the already-bound execution branch/worktree.
- The validation plan is narrowed to records-only checks and does not imply runtime or coverage execution.

## Inputs
- linked source issue prompt
- local task bundles for `#2683` and `#2699`
- current GitHub issue/PR metadata for `#2683`, `#2699`, and `#2704`
- current local guard status before and after the records cleanup

## Target Files / Surfaces
- `.adl/v0.90.5/bodies/issue-2704-v0-90-5-records-repair-missing-duplicate-task-bundle-records.md`
- `.adl/v0.90.5/tasks/issue-2704__v0-90-5-records-repair-missing-duplicate-task-bundle-records/stp.md`
- `.adl/v0.90.5/tasks/issue-2704__v0-90-5-records-repair-missing-duplicate-task-bundle-records/sip.md`
- `.adl/v0.90.5/tasks/issue-2704__v0-90-5-records-repair-missing-duplicate-task-bundle-records/sor.md`
- `.adl/v0.90.5/tasks/issue-2683__v0-90-5-daily-coverage-blockers-2026-05-02/stp.md`
- `.adl/v0.90.5/tasks/issue-2683__v0-90-5-daily-coverage-blockers-2026-05-02/sip.md`
- `.adl/v0.90.5/tasks/issue-2683__v0-90-5-daily-coverage-blockers-2026-05-02/sor.md`
- `.adl/v0.90.5/tasks/issue-2699__v0-90-5-tools-stop-pr-finish-tests-from-leaking-orphaned-post-merge-closeout-watchers/sor.md`
- `adl/tools/check_milestone_closed_issue_sor_truth.sh`
- `adl/tools/check_no_tracked_adl_issue_record_residue.sh`

## Validation Plan
- Commands to run during execution: use GitHub metadata reads plus the local closeout-truth and residue guards only.
- Tests to run: none beyond the bounded records guard commands because this issue changes no runtime behavior.
- Artifacts or traces: updated local `.adl` issue records plus clean guard output.
- Reviewer checks: confirm `#2683` reflects merged PR `#2691`, `#2699` reflects duplicate closure to `#2700`, and no new broken local task path is introduced.

## Demo / Proof Requirements
- Demo set: not applicable.
- Proof surfaces: GitHub lifecycle metadata for `#2683`/PR `#2691` and `#2699`, plus the local records guard commands.
- No-demo rationale: this issue is records surgery only; no product or runtime surface changes.

## Constraints / Policies
- Determinism: keep behavior stable for identical inputs unless the issue explicitly changes semantics.
- Security and privacy: do not introduce secrets, tokens, prompts, tool arguments, or absolute host paths.
- Resource limits: prefer the smallest metadata and guard-command surface that proves the local records are clean.

## System Invariants (must remain true)
- Deterministic execution for identical inputs.
- No hidden state or undeclared side effects.
- Artifacts remain replay-compatible with the replay runner.
- Trace artifacts contain no secrets, prompts, tool arguments, or absolute host paths.
- Artifact schema changes are explicit and approved.

## Reviewer Checklist (machine-readable hints)
```yaml
determinism_required: true
network_allowed: false
artifact_schema_change: false
replay_required: true
security_sensitive: true
ci_validation_required: true
```

## Card Automation Hooks (prompt generation)
- Prompt source fields:
  - Goal
  - Required Outcome
  - Acceptance Criteria
  - Inputs
  - Target Files / Surfaces
  - Validation Plan
  - Demo / Proof Requirements
  - Constraints / Policies
  - System Invariants
  - Reviewer Checklist
- Generation requirements:
  - Deterministic output for identical input card content
  - No secrets, tokens, or absolute host paths in generated prompt text
  - Preserve traceability back to the source issue prompt
  - Preserve explicit required-outcome and demo/proof requirements

## Non-goals / Out of scope

- unrelated repository repair
- changing the source issue prompt without recording it explicitly

## Notes / Risks

- Refine this card if the linked source issue prompt changes materially before execution begins.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Keep the work bounded to the named local records surfaces and guards.
- Do not widen into `#2700` implementation, runtime fixes, or repo-wide records sweeps.
- Write the execution result to the paired output card file truthfully after the records cleanup and guard reruns.
