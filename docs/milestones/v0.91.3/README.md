# v0.91.3 Milestone README

## Status

Planned milestone package authored from `#3099`.

The milestone issue wave is not open yet. Work package issue numbers and sprint
umbrella issue numbers remain `pending` until the v0.91.3 issue wave is seeded
through the normal conductor lifecycle.

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
- validators and demos sufficient to prove the first slice
- docs that let future agents execute the slice without rediscovering TBD notes

Out of scope:

- making C-SDLC the default for all future development
- broad autonomous engineering
- replacing GitHub pull requests, CI, branch protection, or human review
- solving the full Software Development Polis
- depending on the Google Workspace CMS bridge as required infrastructure
- treating speed as a reason to weaken review, replay, or closeout truth

## Source Map

This package is grounded in:

- `C_SDLC_TRACKED_SOURCE_PACKAGE_v0.91.3.md`
- `docs/tooling/card-lifecycle.md`
- `docs/tooling/structured-prompt-contracts.md`
- `docs/tooling/srp-sor-obsmem-handoff-v0.91.2.md`
- `docs/milestones/v0.91.2/features/GOOGLE_WORKSPACE_CMS_BRIDGE.md`

The local `.adl/docs/TBD/cognitive-sdlc/` notes are drafting history. The
tracked source package is the branch-verifiable source map for the milestone.

## Document Map

- WBS: [WBS_v0.91.3.md](WBS_v0.91.3.md)
- Sprint plan: [SPRINT_v0.91.3.md](SPRINT_v0.91.3.md)
- Planned issue wave: [WP_ISSUE_WAVE_v0.91.3.yaml](WP_ISSUE_WAVE_v0.91.3.yaml)
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
- Feature index: [features/README.md](features/README.md)

## Success Criteria

v0.91.3 is ready to close when the project has one convincing, evidence-backed
Cognitive State Transition that:

- uses the canonical `SIP -> STP -> SPP -> SRP -> SOR` lifecycle
- records a transition manifest and transition DAG
- separates serial work, parallel shards, and synchronization barriers
- produces a reviewable evidence bundle
- records SRP review results and SOR outcome truth
- preserves GitHub issue, PR, CI, branch, human review, and closeout discipline
- records an ObsMem handoff boundary
- keeps the C-SDLC planning sources public, tracked, and auditable
- records trace/proof references that can become signed trace bundles in
  v0.91.4
- demonstrates measurable coordination and review behavior without claiming
  full C-SDLC adoption
