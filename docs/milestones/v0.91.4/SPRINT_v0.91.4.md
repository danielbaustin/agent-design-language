# v0.91.4 Sprint Plan

## Metadata

- Milestone: `v0.91.4`
- Version: `v0.91.4`
- Date: `2026-05-25`
- Owner: ADL maintainers
- Related issues: `#3210`, planned v0.91.4 sprint umbrellas

## Status

Sprint map opened through WP-01. Sprint 1, Sprint 2, Sprint 3, and Sprint 4
are seeded as controlled issue/card batches; the CodeFriend sidecar remains
queued until WP-01 routes it separately.

## How To Use

Use this document to seed sprint umbrellas and order child work. It is not a
claim that any sprint has started or closed.

## Sprint Overview

| Sprint | Title | Ordered Children | Goal |
| --- | --- | --- | --- |
| Sprint 1 | Lifecycle And Routing Hardening (`#3347`) | WP-01 `#3346`, WP-02 `#3348`, WP-03 `#3349`, WP-04 `#3350` | Make validators, doctor, conductor, and editor skills agree on C-SDLC state. |
| Sprint 2 | Transition Operation (`#3352`) | WP-05 `#3353`, WP-06 `#3354`, WP-07 `#3355`, WP-08 `#3356` | Make actor standing, shards, evidence, merge gates, and memory handoff repeatable. |
| Sprint 3 | Sprint Default And Metrics (`#3357`) | WP-09 `#3358`, WP-10 `#3359`, WP-11 `#3360`, WP-12 `#3361` | Make sprint execution default-safe and measure repeatability, validation-tail, proof-latency, and parallel-validation behavior. |
| Sprint 4 | Review, Remediation, Planning, And Release (`#3362`) | WP-13 `#3363`, WP-14 `#3364`, WP-15 `#3365`, WP-16 `#3366`, WP-17 `#3367`, WP-18 `#3368`, WP-19 `#3369`, WP-20 `#3370`, WP-21 `#3371` | Prove, gate, review, remediate, plan the next milestone, re-review the handoff, and close the completion milestone. |

## Sidecar Mini-Sprint

The CodeFriend pre-alpha repo/S3 welcome-page setup runs as a bounded sidecar
mini-sprint in v0.91.4:

| Sidecar | Title | Ordered Children | Goal |
| --- | --- | --- | --- |
| CodeFriend Pre-Alpha Setup | CodeFriend pre-alpha site setup | CF-PRE-01, CF-PRE-02, CF-PRE-03, CF-PRE-04 | Establish the private CodeFriend repo and a verified S3/CloudFront/HTTPS welcome page without making CodeFriend part of C-SDLC core proof. |

The sidecar may run after WP-01 has opened the v0.91.4 issue wave. It must not
interrupt the required C-SDLC closeout tail or add extra release-tail gates.

## Issue Opening Policy

The v0.91.4 issue wave is opened in sprint-sized batches so the new upfront-card
rule stays reviewable:

- Batch 0 opened WP-01 `#3346`.
- Batch 1 opened Sprint 1 `#3347` and WP-02 through WP-04 as `#3348` through
  `#3350`.
- Batch 2 opened Sprint 2 `#3352` and WP-05 through WP-08 as `#3353` through
  `#3356`; child execution waits on WP-01/Sprint 1 sequencing.
- Batch 3 opened Sprint 3 `#3357` and WP-09 through WP-12 as `#3358` through
  `#3361`; child execution waits on WP-01/Sprint 1 and Sprint 2 sequencing.
- Batch 4 opened Sprint 4 `#3362` and WP-13 through WP-21 as `#3363`
  through `#3371`; child execution waits on WP-01 and the prior sprint
  sequence.
- Batch 5 remains queued for the CodeFriend sidecar after WP-01.

Every opened issue receives all five cards upfront. `SIP`, `STP`, and `SPP`
must be design-time ready before execution starts; `SRP` and `SOR` remain
present but truthful to pre-review and pre-output lifecycle state.

## Sprint Goals

The sprint overview table above is the generator-facing sprint map. The goals
below explain the intended execution posture for each sprint without replacing
that canonical table.

## Sprint Goal

v0.91.4 should make the C-SDLC default lane reliable enough that future ADL
software-development work can use it without special ceremony or hidden local
state.

## Planned Scope

- Sprint 1: lifecycle, validator, conductor, doctor, and editor hardening.
- Sprint 2: actor standing, shard ownership, evidence convergence, signed
  trace, merge readiness, and memory handoff.
- Sprint 3: sprint default behavior, active issue migration, repeatability
  metrics, validation-tail/proof-latency handling, and drift fixtures.
- Sprint 4: proof coverage, quality gate, docs/adoption review, internal
  review, external review, remediation, next-milestone planning,
  next-milestone review, and release ceremony.
- Sidecar mini-sprint: bounded CodeFriend pre-alpha repo and static-site setup.

## Work Plan

Execute sprints in order unless a documented dependency gate requires a
bounded sidecar issue to run after WP-01. Do not let the sidecar replace,
delay invisibly, or expand the C-SDLC release-tail gates.

## Execution Policy

Every sprint must preserve:

- conductor routing for every lifecycle stage
- editor-only card edits
- bound worktree execution
- pre-PR review before publication
- closeout after issue closure
- combined-lane validation where integration risk exists
- proof coverage, quality gate, docs/adoption review, internal review,
  external review, remediation, next-milestone planning, next-milestone review
  pass, and release ceremony remain separate ordered tail issues
- sidecar product work remains separate from C-SDLC default-operation proof

## Cadence Expectations

- Keep issue execution bounded and reviewable.
- Prefer focused proof over broad test cycles when the touched surface is docs
  or planning only.
- Record validation-tail and proof-latency evidence instead of hiding long
  blocking checks behind the five-minute-sprint claim.

## Risks / Dependencies

- Sprint 1 must land before later sprints rely on default C-SDLC state truth.
- Signed trace and tracked workflow-state proof must land before release
  readiness is claimed.
- The CodeFriend sidecar depends on AWS/DNS approval and may end in a truthful
  blocked handoff.

## Demo / Review Plan

- Sprint 3 owns repeatability and validation-tail/proof-latency evidence.
- Sprint 4 owns demo/proof coverage, quality gate, docs/adoption review,
  internal review, external review, remediation, next-milestone planning,
  next-milestone review, and ceremony.
- CodeFriend sidecar proof is reviewed as product setup evidence only.

## Closeout Bar

The milestone is not complete merely because the process is documented.

It must show that the C-SDLC lane can run repeatedly with:

- correct card creation
- correct routing
- correct actor standing and shard ownership
- correct review recording
- correct SOR closeout
- correct sprint state
- correct memory handoff boundary
- measured coordination, validation-tail, proof-latency, and parallel-validation
  behavior

The CodeFriend sidecar is complete only when it has either a verified HTTPS
welcome page and handoff record or a truthful blocked handoff with AWS/DNS
approval blockers recorded.

## Exit Criteria

- Sprint umbrellas are opened with ordered child work and complete structured
  prompts.
- Sprint state cannot advance past child issues without closeout truth.
- v0.91.4 closes only after the full release-tail sequence completes.
