# ADR 0031 Candidate: C-SDLC Multi-Agent Parallel Execution Boundary

- Status: Candidate
- Target milestone: v0.91.4
- Related issues: #3444
- Related ADRs: ADR 0024, ADR 0028, Candidate ADR 0029 if accepted, Candidate ADR 0030 if accepted
- Source evidence:
  - `docs/milestones/v0.91.4/DESIGN_v0.91.4.md`
  - `docs/milestones/v0.91.4/features/SHARD_OWNERSHIP_AND_INTERFACE_FREEZE.md`
  - `docs/milestones/v0.91.4/features/SOFTWARE_DEVELOPMENT_POLIS_AND_ACTOR_STANDING.md`
  - `docs/milestones/v0.91.4/features/PROCESS_DRIFT_REGRESSION_FIXTURES.md`

## Context

The C-SDLC is intended to support parallelism. A single-threaded sprint can be
useful, but it will not realize the process's full value. ADL needs multiple
agents to work on bounded shards without losing conductor authority, review
independence, merge truth, or closeout truth.

The risk is that multi-agent execution can amplify every process failure:
wrong checkout edits, hidden scope expansion, stale cards, unreviewed PRs,
colliding writes, and overclaimed closeout.

## Decision

ADL should support C-SDLC multi-agent parallel execution only behind explicit
boundaries.

The boundary requires:

- a conductor-owned sprint or issue plan that assigns shards before execution
- one bound branch/worktree per executable shard unless an issue explicitly
  declares a safe shared surface
- declared write scopes and read-only context for each shard
- interface-freeze checkpoints for shared contracts
- independent review of shard outputs before integration
- aggregate sprint or issue state derived from child truth, not from chat
  summaries
- blocked-state escalation when ownership, dependencies, review, or proof is
  ambiguous

## Consequences

### Positive

- Makes parallel execution a governed capability instead of a race.
- Preserves the existing conductor, card, review, PR, and closeout discipline.
- Lets future C-SDLC demos demonstrate real acceleration without hiding risk.
- Gives PVF and sprint-conductor work a clear execution boundary.

### Negative

- Parallelism requires more planning than one-agent execution.
- Shard conflict detection and integration review become mandatory quality
  surfaces.
- Some issues should remain single-threaded when shard boundaries are unclear.

## Alternatives Considered

### Fully autonomous parallel agents

This would be fast in the small and dangerous in the large. ADL's architecture
requires governed autonomy, not invisible autonomy.

### Keep all sprints single-threaded

This is easier to manage but leaves much of the C-SDLC value unrealized and
keeps the project exposed to long wall-clock delays.

## Validation Notes

This candidate should be reviewed against v0.91.4 shard-ownership proof,
five-minute sprint repeatability evidence, process-drift regression fixtures,
and any multi-agent sprint conductor proof packets created during the
milestone.

## Non-Claims

- This ADR does not allow agents to silently create issues, bypass the
  conductor, skip review, merge PRs, or edit outside their shard.
- This ADR does not require every issue to be parallelized.
- This ADR does not claim distributed execution machinery is complete.
