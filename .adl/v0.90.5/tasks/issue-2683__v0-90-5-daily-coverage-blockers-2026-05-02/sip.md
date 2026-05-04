# ADL Input Card

Task ID: issue-2683
Run ID: issue-2683
Version: v0.90.5
Title: [v0.90.5] Daily coverage blockers: 2026-05-02
Branch: codex/2683-v0-90-5-daily-coverage-blockers-2026-05-02

Context:
- Issue: https://github.com/danielbaustin/agent-design-language/issues/2683
- PR: https://github.com/danielbaustin/agent-design-language/pull/2691
- Source Issue Prompt: .adl/v0.90.5/bodies/issue-2683-v0-90-5-daily-coverage-blockers-2026-05-02.md
- Docs: none
- Other: none

## Agent Execution Rules
- This issue is closed/completed; implementation branch/worktree lifecycle is finished.
- Do not run `pr start`; the issue has already completed its lifecycle.
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
  output_card: .adl/v0.90.5/tasks/issue-2683__v0-90-5-daily-coverage-blockers-2026-05-02/sor.md
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

Preserve truthful closed/merged local card state for the historical daily coverage blocker issue after merged PR `#2691`.

## Required Outcome

- Keep the linked issue prompt, input card, and output record aligned with the closed issue and merged PR state.
- Preserve truthful lifecycle state without implying new implementation work is still pending.

## Acceptance Criteria

- The linked source issue prompt is reviewable.
- The card bundle records branch `codex/2683-v0-90-5-daily-coverage-blockers-2026-05-02` as already published through merged PR `#2691`.
- The local records no longer imply bootstrap-only or pre-run state for this closed issue.

## Inputs
- linked source issue prompt
- local STP/SIP/SOR bundle
- current GitHub issue and merged PR metadata

## Target Files / Surfaces
- `.adl/v0.90.5/tasks/issue-2683__v0-90-5-daily-coverage-blockers-2026-05-02/stp.md`
- `.adl/v0.90.5/tasks/issue-2683__v0-90-5-daily-coverage-blockers-2026-05-02/sip.md`
- `.adl/v0.90.5/tasks/issue-2683__v0-90-5-daily-coverage-blockers-2026-05-02/sor.md`
- GitHub issue `#2683`
- merged PR `#2691`

## Validation Plan
- Commands to run during closeout normalization: validate the local SOR structure and confirm the closed issue plus merged PR linkage.
- Tests to run: none in this local continuity pass.
- Artifacts or traces: updated local records only.
- Reviewer checks: verify the local records no longer contradict merged PR `#2691`.

## Demo / Proof Requirements
- Demo set: not applicable.
- Proof surfaces: GitHub metadata for issue `#2683` and PR `#2691`.
- No-demo rationale: this pass normalizes local records only.

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
- Do not reopen or re-execute the daily coverage blocker issue.
- Treat merged PR `#2691` as the publication truth and update the paired output card accordingly.
