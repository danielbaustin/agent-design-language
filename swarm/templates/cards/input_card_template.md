# ADL Input Card

Task ID:
Run ID:
Version:
Title:
Branch:

Context:
- Issue:
- PR:
- Docs:
- Other:

## Agent Execution Rules
- Do not run `pr start`; the branch and worktree already exist.
- Do not delete or recreate cards.
- Do not switch branches unless explicitly instructed.
- Do not work on `main`.
- Only modify files required for the issue.
- Use repository-relative paths; avoid absolute host paths.
- Write the output card to the paired `.adl/cards/<issue>/output_<issue>.md` path.
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
    - acceptance_criteria
    - inputs
    - constraints_policies
    - system_invariants
    - reviewer_checklist
outputs:
  output_card: .adl/cards/<issue>/output_<issue>.md
  summary_style: concise_structured
constraints:
  include_system_invariants: true
  include_reviewer_checklist: true
  disallow_secrets: true
  disallow_absolute_host_paths: true
review_surfaces:
  - card_review_checklist.v1
  - card_review_output.v1
  - card_reviewer_gpt.v1.1
```

Execution:
- Agent:
- Provider:
- Tools allowed:
- Sandbox / approvals:

## Goal

## Acceptance Criteria

## Inputs
- 

## Constraints / Policies
- Determinism requirements:
- Security / privacy requirements:
- Resource limits (time/CPU/memory/network):

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
  - Acceptance Criteria
  - Inputs
  - Constraints / Policies
  - System Invariants
  - Reviewer Checklist
- Generation requirements:
  - Deterministic output for identical input card content
  - No secrets, tokens, or absolute host paths in generated prompt text

## Non-goals / Out of scope

## Notes / Risks

## Instructions to the Agent
- Read this file.
- Do the work described above.
- Write results to the paired output card file.
