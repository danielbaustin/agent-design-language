# ADL Input Card

Task ID:
Run ID:
Version:
Title:
Branch:

Context:
- Issue:
- PR:
- Source Issue Prompt: <required repo-relative reference or URL>
- Docs: <required freeform value or 'none'>
- Other: <optional note or 'none'>

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
  output_card: .adl/<scope>/tasks/<task-id>__<slug>/sor.md
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

Prepare the linked issue prompt and review surfaces for truthful pre-run review before execution is bound.

## Required Outcome

- Keep the linked issue prompt, input card, and output record aligned for review.
- Preserve truthful lifecycle state until `pr run` binds the branch and worktree.

## Acceptance Criteria

- The linked source issue prompt is reviewable and structurally valid.
- The card bundle does not imply a branch or worktree exists before `pr run`.
- Validation and proof expectations are recorded or explicitly marked not applicable.

## Inputs
- linked source issue prompt
- root task bundle cards
- current repository state before execution binding

## Target Files / Surfaces
- files, docs, tests, commands, schemas, and artifacts named by the linked source issue prompt, once execution is bound

## Validation Plan
- Commands to run before execution: structured prompt/card validation only, unless the source issue prompt explicitly requires a pre-run proof.
- Commands to run during execution: derive the exact command set from the linked issue prompt and repo state after `pr run` binds the worktree.
- Tests to run: execute the smallest proving test set for the required outcome during execution.
- Artifacts or traces: produce or update the proof surfaces required by the linked issue prompt during execution.
- Reviewer checks: capture any manual review or demo checks in the output card after execution.

## Demo / Proof Requirements
- Demo set: follow the linked issue prompt.
- Proof surfaces: use the proof surfaces named by the linked issue prompt and output card once execution is bound.
- No-demo rationale: if no demo is required, explain why in the output card during execution.

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

- Refine this card if the linked source issue prompt changes materially before execution begins.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run the repo-native issue-mode `pr run` flow and then perform the work described above.
- Write results to the paired output card file during execution.
