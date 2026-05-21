# Software Development Polis And Actor Standing

## Status

Planned `v0.91.4` feature.

## Purpose

Make the C-SDLC polis model operational enough that repeated five-minute
sprints can coordinate human and AI actors without hiding authority, ownership,
or accountability.

C-SDLC treats software development as a governed polis: human operators,
reviewers, conductor agents, editor agents, shard workers, and review agents
can all participate, but each actor must have scoped standing, declared
responsibility, bounded authority, and reviewable outcomes.

## Scope

This feature covers:

- actor and role references in transition records
- standing classes for operator, conductor, editor, shard worker, reviewer,
  verifier, and closeout owner
- authority boundaries for planning, editing, reviewing, publishing, merging,
  and closeout
- shard ownership records that bind actors to write scopes and proof duties
- standing transitions such as admitted, active, waiting for review, blocked,
  superseded, closed out, or revoked for the transition
- evidence required before an actor can claim a role or advance a transition
- human/operator authority boundaries that cannot be delegated silently

## Acceptance Criteria

- C-SDLC transition records identify actor and role references for every
  actor that materially changes, reviews, verifies, publishes, merges, or
  closes out the transition.
- Shard plans name owners, writable paths, dependencies, interface-freeze
  constraints, and proof obligations.
- Standing changes are evidence-bound and cannot be inferred from chat-only
  context or local-only artifacts.
- Conductor and editor routing preserve role boundaries instead of letting one
  actor silently absorb every lifecycle function.
- Review and merge-readiness gates distinguish author, reviewer, verifier,
  operator, and closeout responsibilities.
- Durable actor-standing records are tracked under the C-SDLC workflow
  namespace when they are used for governance, review, proof, memory, or
  release evidence.
- The five-minute-sprint repeatability demo shows actor standing and shard
  ownership without granting unbounded autonomy.

## Non-Goals

- This feature does not create legal personhood, employment status, or external
  organizational authority.
- This feature does not replace human review, branch protection, or operator
  judgment.
