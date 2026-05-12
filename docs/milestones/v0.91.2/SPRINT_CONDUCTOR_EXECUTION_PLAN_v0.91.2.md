# v0.91.2 Sprint-Conductor Execution Plan

## Status

Active operating model for `v0.91.2`. This document did not open the milestone
by itself; WP-01 opened the issue wave and sprint umbrellas from the canonical
milestone package.

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

Use the milestone as the roadmap unit, and use the already-defined milestone
sprints as the sprint-conductor execution units.

Do not run all of `v0.91.2` as one giant autonomous sprint. Also do not invent
a second sprint topology on top of the existing milestone package. Instead,
execute the canonical Sprint 1 through Sprint 4 structure through bounded
sprint-management umbrellas with explicit ordered issue lists taken directly
from the existing sprint plan and issue wave.

## Source-Of-Truth Rule

This document is an execution overlay, not a replacement milestone plan.

The sprint and issue membership source of truth remains:

- `docs/milestones/v0.91.2/SPRINT_v0.91.2.md`
- `docs/milestones/v0.91.2/WBS_v0.91.2.md`
- `docs/milestones/v0.91.2/WP_ISSUE_WAVE_v0.91.2.yaml`

If the milestone sprint structure or issue membership ever changes, update
those canonical docs first and then align this execution plan to them.

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
- the `v0.91.2` issue wave is opened and carded
- child issues should have truthful `STP`, `SIP`, `SPP`, `SRP`, and `SOR`
  surfaces
- docs-only issues should preserve docs-only validation/finish paths
- `#2956` sprint-conductor hardening should be landed or explicitly accepted as
  an execution dependency/risk

## Canonical Sprint Execution Model

This section preserves the existing `v0.91.2` sprint structure and explains
how sprint-conductor should execute it.

### Sprint 1: Benchmark And Test-Cycle Recovery

Sprint umbrella: `#3025`

Child issues:

- `WP-01` / `#3000` Design pass (milestone docs + planning)
- `WP-02` / `#3001` UTS + ACC multi-model benchmark harness
- `WP-03` / `#3002` Provider-native tool-call comparison
- `WP-04` / `#3003` Runtime/test-cycle recovery
- `WP-05` / `#3004` Coverage gate ergonomics

Why this sprint exists:

- `WP-01` is the opening gate for the whole milestone and must close before
  the rest of the sprint proceeds
- it is one tightly related benchmark and cycle-time pressure-release band
- the dependency spine is clear
- the value of sequential closeout is high

### Sprint 2: Review Product, Workspace Bridge, And Modernization

Sprint umbrella: `#3026`

Child issues:

- `WP-06` / `#3005` CodeBuddy review packet productization
- `WP-07` / `#3006` Review heuristics skill and demos
- `WP-08` / `#3007` Google Workspace CMS bridge demo
- `WP-09` / `#3008` Rust-native GWS adapter boundary
- `WP-10` / `#3009` Moderne / OpenRewrite LST modernization demo

Why this sprint exists:

- these are bounded product/demo surfaces
- they share the same canonical-authority and operator-gating concerns
- they benefit from one reviewed sprint closeout rather than five isolated
  tool/demo stories

### Sprint 3: Runtime Ergonomics, Publication, Docs, And Workflow Guardrails

Sprint umbrella: `#3027`

Child issues:

- `WP-11` / `#3010` Speculative decoding prototype
- `WP-12` / `#3011` Repo visibility follow-on
- `WP-13` / `#3012` Publication program package
- `WP-14` / `#3013` General intelligence paper packet
- `WP-15` / `#3014` Rustdoc and doc cleanup
- `WP-16` / `#3015` Workflow guardrails hardening

Why this sprint exists:

- it is the milestone's mixed ergonomics/publication/docs/guardrails band as
  already defined in the canonical sprint plan
- it combines runtime ergonomics, reviewer-facing docs work, and workflow
  hardening in one explicit sequential sprint
- sprint-conductor should preserve one active child issue at a time even though
  the issue types vary across runtime, docs, and tooling surfaces

### Sprint 4: Review, Remediation, Planning, And Release

Sprint umbrella: `#3028`

Child issues:

- `WP-17` / `#3016` Demo matrix and proof coverage
- `WP-18` / `#3017` Coverage / quality gate
- `WP-19` / `#3018` Docs + review pass
- `WP-20` / `#3019` Internal review
- `WP-21` / `#3020` External / 3rd-party review
- `WP-22` / `#3021` Review findings remediation
- `WP-23` / `#3022` Next milestone planning
- `WP-24` / `#3023` Release ceremony

Release-tail notes:

- if we later decide to preserve the extra pre-ceremony next-milestone review
  pass pattern from `v0.91.1`, add that step explicitly to the `v0.91.2`
  WBS, sprint plan, and issue wave before execution rather than silently
  assuming it exists
- `WP-22` may legitimately produce follow-up issues based on `WP-21` findings,
  but that does not change the canonical Sprint 4 issue list; route those
  follow-ons explicitly instead of treating them as a hidden sprint rewrite

Why this sprint exists:

- the review/release tail is already a state machine
- conductor sequencing is especially valuable here
- this is where omitted closeout steps do the most damage

## Execution Rules

Every canonical `v0.91.2` sprint should follow these rules under
`sprint-conductor`:

- one active child issue at a time
- if an active child issue has a healthy open PR, classify it as
  `waiting_for_review` rather than as an automatic failure state
- while a child issue is in `waiting_for_review`, the sprint is paused at that
  child; do not start issue `N+1`
- start issue `N+1` only after issue `N` is merged or otherwise closed and
  then fully closeouted locally
- use `sprint-conductor` for orchestration only, not as a replacement for the
  issue lifecycle skills
- use the normal issue lifecycle for the child work:
  `workflow-conductor`, `pr-ready`, `pr-run`, `pr-finish`, `pr-janitor`,
  `pr-closeout`
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

Additional Sprint 1 rule:

- `WP-01` must close before `WP-02` through `WP-05` begin, because the rest of
  the milestone depends on the opening planning/card/package truth it creates

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

- The main optimization is better execution discipline inside the existing
  issue-wave shape, not more aggressive model autonomy.
- `v0.91.2` should treat sprint-conductor as the default for the canonical
  Sprint 1 through Sprint 4 umbrellas already defined by the milestone package.
- If a canonical sprint later proves too heterogeneous, change the canonical
  milestone sprint docs first, then realign this plan; do not silently fork the
  structure here.
- Within one sprint umbrella, child issues run sequentially under the slow
  path. If operators choose to run multiple sprint umbrellas concurrently, that
  must be an explicit milestone-level decision and must not break the
  one-active-child-issue rule inside each umbrella.
