# Parallel Validation Fabric

## Metadata

- Feature Name: Parallel Validation Fabric
- Milestone Target: `v0.91.4`
- Status: planned
- Planned WP Home: WP-10, with WP-14 quality-gate validation
- Template: `docs/templates/planning/1.0.0/feature_doc.md`

## Template Rules

This feature doc is a planned contract. It is not implementation evidence until
the owning WP produces tracked proof.

## Purpose

Make validation parallelizable without making proof vague.

The fabric decomposes validation into issue-local, shardable, cache-aware, and
asynchronously reviewable lanes while preserving a truthful distinction between
proof that has passed, proof that is pending, proof that is deferred, and proof
that blocks continuation.

## Context

The v0.91.3 C-SDLC work made short coordinated sprint execution plausible, but
also exposed a validation-tail bottleneck: a five-minute sprint is not
operationally convincing if every bounded transition waits on one monolithic
long-running proof gate.

PVF is the v0.91.4 answer to that bottleneck. It should preserve the rigor of
the existing validation story while making proof lanes explicit, reviewable,
and parallel where the evidence allows it.

## Coverage / Ownership

- WP-10 owns the first PVF plan or bounded proof packet.
- WP-13 maps PVF into the demo/proof surface.
- WP-14 checks that PVF cannot hide pending, deferred, blocked, or failed proof.
- Release evidence records what PVF proved, what remains future work, and what
  still blocks continuation.

## Overview

PVF is a coordination layer for validation evidence. It does not replace CI,
branch protection, human review, or closeout. Instead, it classifies proof into
lanes and records each lane's status in C-SDLC surfaces:

- issue-local proof that can run immediately
- shardable proof that can run in parallel
- cache-aware proof that can be reused only with valid inputs
- deferred proof that does not block the current transition
- blocking proof that must pass before continuation

## Design

Each validation lane should declare:

- owner or responsible issue
- input files, fixtures, or evidence roots
- cache key or invalidation rule when applicable
- synchronization barrier, if any
- expected proof artifact
- status vocabulary for pass, fail, pending, deferred, and blocked

Aggregated status must be derived from lane truth. A green aggregate cannot
hide a failing, pending, or blocked lane.

## Execution Flow

1. `SPP` identifies required proof lanes and stop/replan conditions.
2. Execution runs issue-local proof first.
3. Parallel proof lanes run when dependencies and cache inputs allow.
4. `SRP` records review findings, including pending or deferred proof.
5. `SOR` records what passed, what remains pending, what was deferred, and what
   blocked continuation.
6. WP-14 and release evidence verify that the final milestone claim does not
   overstate PVF status.

## Determinism and Constraints

- Proof lanes must be reproducible from tracked inputs whenever possible.
- Cache reuse must name the inputs that make reuse valid.
- Async proof cannot be treated as passed until evidence exists.
- Pending/deferred proof must remain visible to reviewers.
- PVF must not weaken the existing C-SDLC review, merge, or closeout gates.

## Integration Points

- `docs/milestones/v0.91.4/WBS_v0.91.4.md`
- `docs/milestones/v0.91.4/WP_ISSUE_WAVE_v0.91.4.yaml`
- `docs/milestones/v0.91.4/FEATURE_PROOF_COVERAGE_v0.91.4.md`
- `docs/milestones/v0.91.4/DEMO_MATRIX_v0.91.4.md`
- `docs/milestones/v0.91.4/QUALITY_GATE_v0.91.4.md`
- `SPP`, `SRP`, and `SOR` card status/proof fields

## Validation

WP-10 should produce a PVF runbook or fixture proof. WP-14 should validate that:

- each lane has an owner, input, proof artifact, and status
- aggregate status does not hide failed, pending, deferred, or blocked proof
- C-SDLC cards record proof status consistently
- release evidence cites the PVF proof surface before making completion claims

## Acceptance Criteria

- WP-10 produces an explicit Parallel Validation Fabric plan or first bounded
  proof packet.
- The fabric names proof lanes, owners, synchronization barriers, cache inputs,
  and blocked-state rules.
- `SPP` records required proof and stop/replan conditions before execution.
- `SRP` records review findings and pending/deferred proof without converting
  them into success.
- `SOR` records what passed, what remains pending, what is deferred, and what
  blocked continuation.
- Aggregated status cannot hide a failing, pending, or blocked shard.
- WP-14 checks that the fabric is represented in feature proof coverage, demo
  evidence, quality-gate evidence, and release evidence.

## Risks

- PVF could become a way to hide unfinished proof behind a nicer dashboard.
- Cache reuse could become unsound if invalidation rules are not explicit.
- Async proof could create merge-time ambiguity unless status gates are strict.

## Future Work

Later milestones may turn the first PVF proof into richer automation, UI,
dashboarding, or distributed validation machinery. v0.91.4 only needs enough
to make the C-SDLC default path truthful and repeatable.

## Notes

Source notes:

- `.adl/docs/TBD/cognitive-sdlc/FIVE_MINUTE_SPRINT_TEST_CYCLE_NOTE_2026-05-22.md`
- `.adl/docs/TBD/cognitive-sdlc/C_SDLC_AND_LONG_TESTING_TAIL.md`
