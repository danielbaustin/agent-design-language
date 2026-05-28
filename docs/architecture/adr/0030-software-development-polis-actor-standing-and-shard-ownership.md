# ADR 0030 Candidate: Software Development Polis Actor Standing And Shard Ownership

- Status: Candidate
- Target milestone: v0.91.4
- Related issues: #3353, #3415, #3416, #3417, #3418, #3419, #3444
- Related ADRs: ADR 0024, ADR 0028, Candidate ADR 0029 if accepted
- Source evidence:
  - `docs/milestones/v0.91.4/features/SOFTWARE_DEVELOPMENT_POLIS_AND_ACTOR_STANDING.md`
  - `docs/milestones/v0.91.4/features/SHARD_OWNERSHIP_AND_INTERFACE_FREEZE.md`
  - `docs/milestones/v0.91.4/DESIGN_v0.91.4.md`
  - `docs/milestones/v0.91.4/DECISIONS_v0.91.4.md`

## Context

C-SDLC is not only a sequence of cards. It is a governed software-development
polis in which human operators, conductors, card editors, shard workers,
reviewers, verifiers, and closeout owners can all participate.

Parallel work is unsafe unless actors have bounded standing and shard
ownership is explicit. Without those boundaries, one agent can silently absorb
another role, widen scope, collide with another shard, or overclaim review and
merge readiness.

## Decision

ADL should model C-SDLC software-development work as a governed polis with
explicit actor standing, authority boundaries, shard ownership, and
interface-freeze rules.

The policy requires:

- actor and role references for material lifecycle actions
- standing classes for operator, conductor, editor, shard worker, reviewer,
  verifier, publisher, merge owner, and closeout owner
- bounded authority for planning, editing, reviewing, publishing, merging, and
  closeout
- shard ownership records that name writable paths, read-only context,
  dependencies, interface-freeze points, and proof obligations
- blocked-state reporting when write surfaces overlap or ownership is unclear
- explicit human/operator authority boundaries that cannot be silently
  delegated

## Consequences

### Positive

- Makes multi-agent work inspectable before it becomes chaotic.
- Gives reviewers a way to tell who owned which shard and proof obligation.
- Supports parallel execution without hiding conflicts or authority drift.
- Clarifies that standing is evidence-bound, not inferred from chat context.

### Negative

- Shard planning adds upfront coordination work.
- Tooling must represent blocked, superseded, revoked, and waiting-for-review
  standing states truthfully.
- Operators must resist treating the polis model as permission for unbounded
  autonomy.

## Alternatives Considered

### Let agents coordinate informally

This works only while work is small and human memory is fresh. It fails when
parallel work, review, and closeout need auditability.

### Assign ownership only at PR review time

That is too late. Shard ownership and interface boundaries must be clear before
parallel work starts.

## Validation Notes

This candidate should be reviewed against the v0.91.4 Software Development
Polis proof packet, actor-standing fixtures, shard-conflict fixtures, and
repeatability demo evidence when available.

## Non-Claims

- This ADR does not create legal personhood, employment status, or external
  organizational authority.
- This ADR does not authorize unbounded parallel issue execution.
- This ADR does not replace human integration review.
- This ADR does not treat speed as evidence of correctness.
