# v0.91.2 Sprint-Conductor Execution Plan

## Status

Planned operating model for `v0.91.2`. This document does not open the
milestone by itself. It defines how the milestone should be executed once the
issue wave is seeded.

## Purpose

`v0.91.2` should use `sprint-conductor` as the default execution mode for
coherent issue clusters instead of treating the whole milestone as one manual
issue queue.

The goal is to preserve the benefits we just observed in the bounded
`v0.91.1` sprint runs:

- one active child issue at a time
- cleaner card and branch truth
- stronger sprint review and closeout
- less manual orchestration drift

This plan is intentionally about execution shape, not feature scope.

## Operating Decision

Use the milestone as the roadmap unit, but use bounded sprint-conductor
umbrellas as the execution unit.

Do not run all of `v0.91.2` as one giant autonomous sprint. Instead, break it
into coherent sprint-management umbrellas with explicit ordered issue lists.

## Operating Principles

These principles apply to every sprint umbrella, every child issue, and every
lifecycle stage in this operating model:

- use `workflow-conductor` or `sprint-conductor` for every issue and lifecycle
  stage
- edit cards only through the editor-skill path
- always work in a bound worktree on a specific branch, never on `main`
- always review code or work product with a bounded review subagent before
  opening the PR, then fix all findings immediately or explicitly disposition
  them if they are not actionable in scope
- always perform the necessary closeout tasks after the issue is closed

These are not optional style preferences. They are the operating discipline for
the `v0.91.2` conductor experiment.

## Preconditions

Before relying on this operating model:

- `WP-01` must land first as the milestone-opening design/docs/planning pass
- the `v0.91.2` issue wave should be opened and carded
- child issues should have truthful `STP`, `SIP`, `SPP`, `SRP`, and `SOR`
  surfaces
- docs-only issues should preserve docs-only validation/finish paths
- `#2956` sprint-conductor hardening should be landed or explicitly accepted as
  an execution dependency/risk

## Sprint 0: Milestone Opening

Child issues:

- `WP-01` Design pass (milestone docs + planning)

Why this cluster exists:

- `WP-01` is the opening gate for the whole milestone
- it seeds the issue wave, cards, and planning truth required by every later
  umbrella
- it should complete before the conductor-heavy execution band begins

## Default Sprint Umbrellas

### Sprint 1: Benchmark And Test-Cycle Recovery

Child issues:

- `WP-02` UTS + ACC multi-model benchmark harness
- `WP-03` Provider-native tool-call comparison
- `WP-04` Runtime/test-cycle recovery
- `WP-05` Coverage gate ergonomics

Why this cluster exists:

- it is one tightly related benchmark and cycle-time pressure-release band
- the dependency spine is clear
- the value of sequential closeout is high

### Sprint 2: Review Product, Workspace Bridge, And Modernization

Child issues:

- `WP-06` CodeBuddy review packet productization
- `WP-07` Review heuristics skill and demos
- `WP-08` Google Workspace CMS bridge demo
- `WP-09` Rust-native GWS adapter boundary
- `WP-10` Moderne / OpenRewrite LST modernization demo

Why this cluster exists:

- these are bounded product/demo surfaces
- they share the same canonical-authority and operator-gating concerns
- they benefit from one reviewed sprint closeout rather than five isolated
  tool/demo stories

### Sprint 3A: Runtime Ergonomics And Repo Workflow Surfaces

Child issues:

- `WP-11` Speculative decoding prototype
- `WP-12` Repo visibility follow-on
- `WP-16` Workflow guardrails hardening

Why this cluster exists:

- these issues all affect how the repo/runtime feels to operate
- they are closer to one another than to the publication lane
- they are likely to benefit from one concentrated review pass

### Sprint 3B: Publication, Paper Packet, And Doc Hygiene

Child issues:

- `WP-13` Publication program package
- `WP-14` General intelligence paper packet
- `WP-15` Rustdoc and doc cleanup

Why this cluster exists:

- these issues share a documentation/public-intellectual surface
- they should stay bounded away from runtime/tooling hardening concerns
- they can use lighter docs-oriented validation while still receiving sprint
  review and closeout

### Sprint 4: Review, Remediation, Planning, And Release

Child issues:

- `WP-17` Demo matrix and proof coverage
- `WP-18` Coverage / quality gate
- `WP-19` Docs + review pass
- `WP-20` Internal review
- `WP-21` External / 3rd-party review
- `WP-22` Review findings remediation
- `WP-23` Next milestone planning
- `WP-24` Release ceremony

Release-tail note:

- if we later decide to preserve the extra pre-ceremony next-milestone review
  pass pattern from `v0.91.1`, add that step explicitly to the `v0.91.2`
  WBS, sprint plan, and issue wave before execution rather than silently
  assuming it exists

Why this cluster exists:

- the review/release tail is already a state machine
- conductor sequencing is especially valuable here
- this is where omitted closeout steps do the most damage

## Execution Rules

Every `v0.91.2` sprint umbrella should follow these rules:

- one active child issue at a time
- do not start issue `N+1` before issue `N` is fully closed out
- use `sprint-conductor` for orchestration only, not as a replacement for the
  issue lifecycle skills
- use the normal issue lifecycle for the child work:
  `workflow-conductor`, `pr-ready`, `pr-run`, `pr-finish`, `pr-janitor`,
  `pr-closeout`
- treat healthy open PRs as waiting state, not automatic failure state
- stop immediately on scope drift or dependency ambiguity instead of widening
  the sprint implicitly

## Preflight Rules

Before any sprint starts:

- all child issues must exist
- all structured cards must validate
- ordered dependencies must be explicit
- demo/proof expectations must already be written down
- repo inputs and canonical files must be truthful
- docs-only children should be marked for narrow docs-oriented validation

If preflight fails, repair the cards first. Do not start the sprint with known
bundle drift.

## Review Rules

Every sprint should end with bounded review appropriate to the changed surface.

Minimum expectation:

- code review when implementation changed
- test review when validation behavior changed
- docs review when tracked docs changed materially
- synthesis of findings, non-findings, unresolved questions, and residual risk

Sprint-discovered follow-ons should normally be recorded as bounded follow-up
issues rather than silently blocking all sprint closure, unless the sprint
policy explicitly marks them as must-land-before-close.

## Closeout Rules

Every sprint should produce:

- a sprint closeout note
- ordered issue and PR trail
- residual risk summary
- any timing, coverage, or Rust-tracker metrics that meaningfully apply

Do not leave review-tail, release-tail, or handoff work implicit.

## Success Criteria For The Experiment

This operating model should be considered successful if `v0.91.2` shows:

- fewer manual orchestration interventions
- fewer card-drift repairs during live execution
- faster issue-to-issue movement inside coherent clusters
- cleaner sprint closeout truth
- no reduction in review quality or release-tail discipline

## Non-goals

- do not turn `sprint-conductor` into a parallel execution engine
- do not run the entire milestone as one opaque autonomous wave
- do not use sprint umbrellas to hide dependency ambiguity
- do not weaken validation just to preserve sprint throughput

## Notes

- The main optimization is better issue-wave shape, not more aggressive model
  autonomy.
- `v0.91.2` should treat sprint-conductor as the default for bounded clusters,
  especially mini-sprints, remediation bands, docs/review lanes, and coherent
  feature tranches.
- If a sprint becomes too heterogeneous, split it rather than forcing the
  conductor to carry unrelated work under one umbrella.
- Within one sprint umbrella, child issues run sequentially under the slow
  path. If operators choose to run multiple sprint umbrellas concurrently, that
  must be an explicit milestone-level decision and must not break the
  one-active-child-issue rule inside each umbrella.
