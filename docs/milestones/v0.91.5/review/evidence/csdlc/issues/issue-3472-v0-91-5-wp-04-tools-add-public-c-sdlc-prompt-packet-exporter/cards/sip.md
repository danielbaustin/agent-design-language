# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.0/sip.md`

Task ID: issue-3472
Run ID: issue-3472
Version: v0.91.5
Title: [v0.91.5][WP-04][tools] Add public C-SDLC prompt packet exporter
Branch: codex/3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter
Card Status: ready
Generated: 2026-06-04T19:39:09Z

Context:
- Issue: https://github.com/danielbaustin/agent-design-language/issues/3472
- PR:
- Source Issue Prompt: .adl/v0.91.5/bodies/issue-3472-v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter.md
- Docs: none
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
  output_card: .adl/v0.91.5/tasks/issue-3472__v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter/sor.md
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
- Source issue-prompt slug: v0-91-5-wp-04-tools-add-public-c-sdlc-prompt-packet-exporter
- Required outcome type: combination
- Demo required: false

## Goal

Execute the linked issue prompt in the bound issue worktree.

## Required Outcome

Ship the required outcome described by the linked source issue prompt.

## Acceptance Criteria

Satisfy the acceptance criteria in the linked source issue prompt and record focused proof in SOR.

## Inputs

Linked source issue prompt; root task bundle cards; current repository state.

## Target Files / Surfaces

Files, docs, tests, commands, schemas, and artifacts named by the linked source issue prompt.

## Validation Plan

Run the smallest proving validation for the touched surface and record exact commands in SOR.

## Demo / Proof Requirements

Follow demo and proof requirements from the linked source issue prompt.

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
  - Deterministic output for identical SIP content
  - No secrets, tokens, or absolute host paths in generated prompt text
  - Preserve traceability back to the source issue prompt
  - Preserve explicit required-outcome and demo/proof requirements

## Non-goals / Out of scope

Do not widen scope beyond the linked source issue prompt.

## Notes / Risks

Refine this card if the linked source issue prompt changes materially before execution begins.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run the repo-native issue-mode `pr run` flow and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
