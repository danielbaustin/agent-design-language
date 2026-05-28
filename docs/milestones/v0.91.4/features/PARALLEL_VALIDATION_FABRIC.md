# Parallel Validation Fabric

## Metadata

- Feature Name: Parallel Validation Fabric
- Milestone Target: `v0.91.4`
- Status: landed
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
- pre-PR validation evidence that can satisfy CI only when commit and tree
  identity prove the exact same code was tested
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

### Pre-PR Validation Evidence Reuse

The duplicate-test-cycle case is a first-class PVF target. Today a runtime PR
can run the full Rust validation profile during `pr finish`, open or update the
PR, and then immediately run the same expensive profile again in GitHub CI even
though the code has not changed.

The future PVF lane for this should convert the pre-PR validation run into a
structured evidence artifact that CI may accept only when all trust inputs still
match:

- PR head commit matches the validated commit
- tree identity matches the validated tree
- changed-path and validation-policy inputs match the current policy
- command list, tool versions, timestamps, exit codes, and log hashes are
  recorded
- evidence is fresh enough under the release policy
- workflow, CI, coverage, or validation-tooling changes do not force remote
  proof

If any check fails, CI must fall back to the full Rust validation lane. Evidence
reuse is a cache-aware proof lane, not a validation waiver. It must preserve
branch protection, human review, and closeout truth.

Issue `#3437` records this docs-only planning addition. The implementation
belongs with the PVF CI/release-gate work, currently represented by `#3403` or a
future dedicated implementation issue.

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
- Pre-PR validation evidence reuse must require exact commit/tree identity and
  must fail closed to full CI whenever identity, policy, workflow, tooling, or
  freshness checks do not prove equivalence.
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
- pre-PR validation evidence reuse is treated as a strict equivalence proof with
  full-CI fallback, not as a skip
- C-SDLC cards record proof status consistently
- release evidence cites the PVF proof surface before making completion claims

## Acceptance Criteria

- WP-10 produces an explicit Parallel Validation Fabric plan or first bounded
  proof packet.
- The fabric names proof lanes, owners, synchronization barriers, cache inputs,
  and blocked-state rules.
- The fabric includes pre-PR validation evidence reuse as a cache-aware lane
  whose acceptance requires exact commit/tree identity and automatic full-Rust
  fallback on mismatch, staleness, or policy/tooling drift.
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

## First Bounded Proof Surface

- `docs/milestones/v0.91.4/FIVE_MINUTE_SPRINT_REPEATABILITY_REPORT_2026-05-27.md`

## Landed v0.91.4 Position

WP-10 lands the first bounded PVF proof as a lane taxonomy and reporting
discipline, not as a distributed scheduler or CI replacement.

What is landed:

- explicit lane categories for focused issue-local proof, docs-only proof,
  broad integration proof, merge-gate CI, deferred proof, and sprint closeout
  truth
- explicit ordinary-PR versus release-gate routing for docs-only lanes,
  runtime PR-fast lanes, and authoritative release-only lanes
- bounded policy treatment for artifact reuse so reused proof remains visible as
  `reused` instead of being collapsed into silent success
- explicit test-authoring policy and migration guardrails so future tests are
  introduced with lane/proof metadata instead of relying on later cleanup
- explicit distinction between passed, pending, deferred, blocked, and failed
  proof
- evidence-backed demonstration that validation-tail drag can dominate merged
  wall-clock time even when issue-local proof is small

What is not landed:

- automatic pre-PR evidence reuse
- distributed proof orchestration
- automatic cache identity verification

The current PVF posture is therefore:

- landed as a truthful milestone planning/proof surface
- still planned as richer automation beyond v0.91.4
