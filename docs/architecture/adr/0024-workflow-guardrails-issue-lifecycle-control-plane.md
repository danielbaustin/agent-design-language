# ADR 0024 Candidate: Workflow Guardrails And Issue Lifecycle Control Plane

- Status: Candidate
- Target milestone: v0.91.2
- Related issues: #2986, #3069, #3089, #3101, #3118, #3124
- Related ADRs: ADR 0018

## Context

Recent milestone failures were not only implementation bugs. Several were
control-plane failures: work done on the wrong checkout, PRs not opened, local
cards drifting from GitHub truth, sprint closeout overstating cleanliness, and
review findings landing only after work was assumed complete.

v0.91.2 adds `AGENTS.md`, workflow-conductor discipline, editor-only card
normalization, sprint-conductor improvements, and a clarified card lifecycle:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

ADR 0018 records the existence and semantics of the structured planning and
review artifacts. This ADR candidate records the surrounding lifecycle control
plane.

## Decision

ADL treats conductor-first lifecycle routing, bound issue worktrees, editor-only
card mutation, pre-PR subagent review, truthful PR publication, janitor routing,
and post-merge closeout as architecture policy rather than optional operator
habit.

## Requirements

- Use `workflow-conductor` for issue and lifecycle routing.
- Execute tracked issue work in a bound worktree and branch, never directly on
  `main`.
- Mutate cards through the matching editor skill, not opportunistic hand edits.
- Use `SIP -> STP -> SPP -> SRP -> SOR` as the canonical issue lifecycle.
- Run bounded pre-PR review before publication and fix actionable findings.
- Use PR janitor work only for real PR blockers, not broad scope expansion.
- Perform closeout after merge or closure so GitHub, cards, artifacts, and
  repo truth agree.
- Fail closed when the workflow state is unsafe or ambiguous.

## Consequences

### Positive

- Makes ADL issue work more reproducible and reviewable.
- Reduces repeated drift between cards, PRs, and milestone docs.
- Gives future agents one durable control-plane policy to follow.

### Negative

- Small changes require more explicit workflow discipline.
- Editor and conductor skills become architecture-sensitive infrastructure.
- Local ignored card storage remains a transitional weakness until ADR 0028 or
  a successor tracked-state decision lands.

## Alternatives Considered

### Treat workflow rules as operator preference

This has repeatedly failed. The process is now product infrastructure.

### Collapse all lifecycle skills into one mega-conductor

That would reduce visible steps, but it would hide responsibility and make
review harder.

## Validation Notes

This candidate should be reviewed against root `AGENTS.md`, the C-SDLC
mini-sprint results, sprint-conductor repairs, editor-skill contracts, and
workflow-guardrail evidence.

## Non-Claims

- This ADR does not claim the tracked-state migration is complete.
- This ADR does not eliminate human judgment.
- This ADR does not authorize silent merge, silent closeout, or destructive
  cleanup of user work.
