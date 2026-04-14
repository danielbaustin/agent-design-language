# AEE Convergence Model

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning doc: `.adl/docs/v0.89planning/AEE_CONVERGENCE_MODEL.md`
- Planned WP: `WP-02`

## Purpose

Define AEE 1.0 convergence as a real ADL runtime surface rather than a retry story.

`v0.89` should make convergence legible through:
- explicit progress signals
- bounded stop conditions
- strategy changes that are visible and justified
- replayable evidence for why the loop continued or stopped

## Scope

`v0.89` should deliver:
- explicit convergence, stall, bounded-out, and policy-stop outcomes
- progress-signal families and stop-condition families
- bounded iteration records suitable for trace and review
- a reviewer-legible convergence proof surface

This feature is about bounded convergent process quality, not about promising perfect output.

## Main Runtime Commitments

- convergence is judged by explicit progress, not blind retries
- another step must be justified by progress plus policy/budget allowance
- strategy changes and decomposition must be visible rather than hidden improvisation
- termination states must be stable enough to drive later demos and review surfaces

## Non-Goals

- full autonomous self-modification
- unconstrained recursive improvement
- final cross-milestone reasoning-graph or signed-trace completion

## Dependencies

- trace and provider/memory substrate from `v0.87` / `v0.87.1`
- bounded cognition and persistence groundwork from `v0.88`
- decision and action-mediation surfaces in this milestone

## Exit Criteria

- convergence states and stop conditions are named and consistent across docs
- the runtime story distinguishes progress from mere repetition
- later `v0.89` demos can cite this doc as the contract for bounded adaptation
