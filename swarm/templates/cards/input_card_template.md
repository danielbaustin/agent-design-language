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
