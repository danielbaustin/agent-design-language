# ADR 0029: C-SDLC Default Software-Development Lane

- Status: Accepted
- Date: 2026-06-23
- Accepted in: v0.91.6
- Candidate source: docs/architecture/adr/0029-c-sdlc-default-software-development-lane.md
- Target milestone: v0.91.4
- Related issues: #3099, #3100, #3444
- Related ADRs: ADR 0018, ADR 0024, ADR 0028
- Source evidence:
  - `docs/milestones/v0.91.4/DESIGN_v0.91.4.md`
  - `docs/milestones/v0.91.4/DECISIONS_v0.91.4.md`
  - `docs/milestones/v0.91.4/features/COGNITIVE_SDLC_DEFAULT_OPERATION.md`
  - `docs/milestones/v0.91.4/features/SPRINT_CONDUCTOR_DEFAULT_CSDL_LANE.md`

## Context

ADR 0018 made `SPP` and `SRP` durable workflow artifacts. ADR 0024 made
conductor-first lifecycle routing, bound issue worktrees, editor-only card
mutation, pre-PR review, and closeout architecture policy. ADR 0028 set the
tracked-workflow-state and signed-trace direction for C-SDLC.

The v0.91.3 first slice proved that the C-SDLC can structure ADL issue work,
but default operation requires more than one successful sprint. v0.91.4 is the
completion milestone: validators, conductor routing, editor skills, sprint
state, review evidence, signed trace proof, merge-readiness gates, and closeout
must agree before ADL treats C-SDLC as the normal software-development lane.

## Decision

ADL should treat C-SDLC as the default software-development lane for future ADL
issue work once the v0.91.4 completion criteria are met.

The default lane requires:

- workflow-conductor routing for each issue and lifecycle stage
- five issue-local cards in the canonical order `SIP -> STP -> SPP -> SRP -> SOR`
- design-time-ready `SIP`, `STP`, and `SPP` before execution binding
- editor-skill-only card normalization
- bound worktree execution, not tracked issue work on `main`
- bounded pre-PR review with finding disposition before publication
- merge-readiness and PR-gate truth checks before release claims
- closeout truth after issue closure
- tracked durable workflow records and signed trace proof where default
  operation is claimed

## Consequences

### Positive

- Makes ADL's software-development process auditable and repeatable.
- Gives future agents a concrete default lane instead of a collection of
  operator habits.
- Reduces process drift by putting deterministic parts under cards, validators,
  conductor routing, and closeout.
- Creates a stable foundation for the five-minute sprint demo and later C-SDLC
  paper work.

### Negative

- Simple issues carry more upfront bookkeeping than ad hoc development.
- Default operation makes conductor, card editors, validators, and closeout
  tooling architecture-sensitive surfaces.
- The process must continue to distinguish planned, in-flight, proven, and
  closed-out claims.

## Alternatives Considered

### Keep C-SDLC optional

This would reduce process overhead, but it would preserve the drift that
v0.91.3 and v0.91.4 are designed to remove.

### Adopt C-SDLC only for large issues

This would weaken repeatability. C-SDLC needs to be usable for ordinary issue
work or it will not become the stable ADL development substrate.

## Validation Notes

This candidate should be reviewed against the v0.91.4 design, decision log,
feature coverage, demo matrix, quality gate, and final release evidence. It
should not be accepted until the milestone has proof that default-operation
claims are not merely aspirational.

## Non-Claims

- This ADR does not claim v0.91.4 is already complete.
- This ADR does not remove human review, GitHub PRs, CI, or branch protection.
- This ADR does not make speed a substitute for proof.
- This ADR does not require optional product or workspace sidecars to be part
  of C-SDLC core operation.
