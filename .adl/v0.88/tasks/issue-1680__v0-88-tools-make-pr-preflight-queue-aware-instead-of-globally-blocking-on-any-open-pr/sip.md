# ADL Input Card

Task ID: issue-1680
Run ID: issue-1680
Version: v0.88
Title: [v0.88][tools] Make PR preflight queue-aware instead of globally blocking on any open PR
Branch: codex/1680-v0-88-tools-make-pr-preflight-queue-aware-instead-of-globally-blocking-on-any-open-pr

Context:
- Issue: https://github.com/danielbaustin/agent-design-language/issues/1680
- PR:
- Source Issue Prompt: .adl/v0.88/bodies/issue-1680-v0-88-tools-make-pr-preflight-queue-aware-instead-of-globally-blocking-on-any-open-pr.md
- Docs: none
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
  output_card: .adl/v0.88/tasks/issue-1680__v0-88-tools-make-pr-preflight-queue-aware-instead-of-globally-blocking-on-any-open-pr/sor.md
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

Execute the linked issue prompt in this started worktree without rerunning bootstrap commands.

## Required Outcome

- Ship the required outcome type recorded in the linked source issue prompt.
- Keep the linked issue prompt, repository changes, and output record aligned.

## Acceptance Criteria

- The implementation satisfies the linked source issue prompt.
- Validation and proof surfaces named below are completed or explicitly marked not applicable.

## Inputs
- linked source issue prompt
- root and worktree task bundle cards
- current repository state for this branch

## Target Files / Surfaces
- files, docs, tests, commands, schemas, and artifacts named by the linked source issue prompt

## Validation Plan
- Commands to run: derive the exact command set from the linked issue prompt and repo state; record what actually ran in the output card.
- Tests to run: execute the smallest proving test set for the required outcome.
- Artifacts or traces: produce or update the proof surfaces required by the linked issue prompt.
- Reviewer checks: capture any manual review or demo checks in the output card.

## Demo / Proof Requirements
- Demo set: follow the linked issue prompt.
- Proof surfaces: use the proof surfaces named by the linked issue prompt and output card.
- No-demo rationale: if no demo is required, explain why in the output card.

## Constraints / Policies
- Determinism: keep behavior stable for identical inputs unless the issue explicitly changes semantics.
- Security and privacy: do not introduce secrets, tokens, prompts, tool arguments, or absolute host paths.
- Resource limits: prefer the smallest command and test surface that proves the issue is complete.

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

- Refine this card if the linked source issue prompt changes materially before implementation begins.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do the work described above.
- Write results to the paired output card file.
