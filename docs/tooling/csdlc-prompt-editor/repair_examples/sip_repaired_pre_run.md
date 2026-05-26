# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.0/sip.md`

Task ID: issue-4001
Run ID: issue-4001
Version: v0.91.4
Title: [example][SIP] Pre-run repair
Branch: not bound yet
Card Status: ready
Generated: 2026-05-26T12:00:00Z

Context:
- Issue: https://github.com/danielbaustin/agent-design-language/issues/4001
- PR:
- Source Issue Prompt: .adl/v0.91.4/bodies/example-sip-repair.md
- Docs: docs/tooling/csdlc-prompt-editor/README.md
- Other: none

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

## Lifecycle Semantics
- Lifecycle stage: `SIP`
- Activation state: active after issue-intent review.
- Next stage: `STP`, where the selected task or solution is made explicit.
- Legacy compatibility: older references may call this an input card, but new
  issue work should treat it as the Structured Issue Prompt.

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
  output_card: .adl/v0.91.4/tasks/issue-4001__example-sip-repair/sor.md
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

## Execution
- Agent:
- Provider:
- Tools allowed:
- Sandbox / approvals:
- Source issue-prompt slug: example-sip-repair
- Required outcome type: code
- Demo required: false

## Goal

Repair a pre-run input card so it is specific, validator-clean, and does not
invent execution.

## Required Outcome

Produce a truthful `SIP` that can hand off to `STP` without branch, target, or
validation-plan drift.

## Acceptance Criteria

The repaired card keeps `Branch: not bound yet`, points at concrete repo
surfaces, and gives a bounded validation plan.

## Inputs

Linked source issue prompt, task bundle path, and repository-local editor docs.

## Target Files / Surfaces

`docs/tooling/csdlc-prompt-editor/`; `.adl/v0.91.4/tasks/issue-4001__example-sip-repair/`

## Validation Plan

Run the matching structured prompt validator for `sip` at bootstrap phase.

## Demo / Proof Requirements

None. Proof is the repaired card plus validator pass.

## Constraints / Policies

- Follow `AGENTS.md`.
- Use workflow-conductor for lifecycle routing.
- Edit cards only with editor skills.
- Work only in the bound issue worktree after `pr run`.
- Keep validation focused on the touched surface.

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
ci_validation_required: false
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
  - Deterministic output for identical SIP content
  - No secrets, tokens, or absolute host paths in generated prompt text
  - Preserve traceability back to the source issue prompt
  - Preserve explicit required-outcome and demo/proof requirements

## Non-goals / Out of scope

Do not claim the issue is already branch-bound or execution-complete.

## Notes / Risks

If issue routing changes materially, this card should be re-edited before
execution begins.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run the repo-native issue-mode `pr run` flow and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
