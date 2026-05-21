# v0.91.3 Milestone README

## Status

Active milestone package authored from `#3099` and opened by `WP-01` / `#3199`.

The v0.91.3 issue wave is open. The initial wave opened `#3199` through
`#3214`; follow-up correction `#3225` added the missing closeout-tail gates as
`#3226` through `#3230` and Sprint 4 as `#3231`.

## Purpose

v0.91.3 is the first Cognitive SDLC implementation milestone.

Its job is to prove one bounded, reviewable Cognitive State Transition using the
corrected ADL card lifecycle:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

The milestone should turn the v0.91.2 card-lifecycle repair work into a real
vertical slice: transition manifest, transition DAG, shard plan, evidence
bundle, review result, merge gate, outcome truth, and ObsMem handoff.

In this milestone, `SPP` means the issue-local operative execution plan. It is
the tracked equivalent of `/plan` for one issue or transition: current step,
next step, required proof, stop/replan conditions, and explicit bounds. It is
not sprint orchestration, review-result truth, or output truth.

## Milestone Role

v0.91.3 proves the crown jewel.

It should make C-SDLC tangible without pretending the whole operating model is
finished. v0.91.4 owns repeatability, hardening, enforcement, and default
adoption for future ADL software-development issues.

## Scope

In scope:

- one canonical Cognitive State Transition schema and manifest surface
- one transition DAG and shard coordination model
- one evidence bundle and review synthesis shape
- one governed merge-readiness gate that preserves GitHub issue/PR/CI truth
- one ObsMem handoff contract for SRP review results and SOR outcome truth
- one five-minute-sprint first proof surface over a bounded change
- the complete closeout tail: proof coverage, quality gate, docs review pass,
  internal review, external review, remediation, next milestone planning, final
  next milestone review pass, and release ceremony
- validators and demos sufficient to prove the first slice
- docs that let future agents execute the slice without rediscovering TBD notes

Out of scope:

- making C-SDLC the default for all future development
- broad autonomous engineering
- replacing GitHub pull requests, CI, branch protection, or human review
- solving the full Software Development Polis
- treating speed as a reason to weaken review, replay, or closeout truth

## Source Map

This package is grounded in:

- `docs/cognitive-sdlc/README.md`
- `C_SDLC_TRACKED_SOURCE_PACKAGE_v0.91.3.md`
- `docs/tooling/card-lifecycle.md`
- `docs/tooling/structured-prompt-contracts.md`
- `docs/tooling/srp-sor-obsmem-handoff-v0.91.2.md`

The local `.adl/docs/TBD/cognitive-sdlc/` notes are drafting history. The
tracked C-SDLC docs home and tracked source package are the branch-verifiable
source map for the milestone.
When durable workflow records are introduced, the first-slice target namespace
is `workflow/c-sdlc/v0.91.3/`; local `.adl/` copies remain execution cache or
staging state, not sufficient public proof by themselves.

## Document Map

- WBS: [WBS_v0.91.3.md](WBS_v0.91.3.md)
- Sprint plan: [SPRINT_v0.91.3.md](SPRINT_v0.91.3.md)
- Active issue wave: [WP_ISSUE_WAVE_v0.91.3.yaml](WP_ISSUE_WAVE_v0.91.3.yaml)
- Execution readiness:
  [WP_EXECUTION_READINESS_v0.91.3.md](WP_EXECUTION_READINESS_v0.91.3.md)
- Feature proof coverage:
  [FEATURE_PROOF_COVERAGE_v0.91.3.md](FEATURE_PROOF_COVERAGE_v0.91.3.md)
- Demo matrix: [DEMO_MATRIX_v0.91.3.md](DEMO_MATRIX_v0.91.3.md)
- Quality gate: [QUALITY_GATE_v0.91.3.md](QUALITY_GATE_v0.91.3.md)
- Next milestone handoff:
  [NEXT_MILESTONE_HANDOFF_v0.91.3.md](NEXT_MILESTONE_HANDOFF_v0.91.3.md)
- Tracked C-SDLC source package:
  [C_SDLC_TRACKED_SOURCE_PACKAGE_v0.91.3.md](C_SDLC_TRACKED_SOURCE_PACKAGE_v0.91.3.md)
- C-SDLC canonical docs:
  [../../cognitive-sdlc/README.md](../../cognitive-sdlc/README.md)
- Feature index: [features/README.md](features/README.md)

## Success Criteria

v0.91.3 is ready to close when the project has one convincing, evidence-backed
Cognitive State Transition that:

- uses the canonical `SIP -> STP -> SPP -> SRP -> SOR` lifecycle
- records a transition manifest and transition DAG
- separates serial work, parallel shards, and synchronization barriers
- produces a reviewable evidence bundle
- records SRP review results and SOR outcome truth
- treats `SPP` as public/tracked issue-local operative plan truth
- preserves GitHub issue, PR, CI, branch, human review, and closeout discipline
- records an ObsMem handoff boundary
- keeps the C-SDLC planning sources public, tracked, and auditable
- defines or proves the `workflow/c-sdlc/v0.91.3/` namespace for durable
  workflow records
- records trace/proof references that can become signed trace bundles in
  v0.91.4
- demonstrates measurable coordination and review behavior without claiming
  full C-SDLC adoption
