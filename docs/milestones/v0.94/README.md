# v0.94 Milestone README

## Status

Forward planning. `v0.94` is not yet an active implementation milestone and has
no final issue wave.

## Purpose

`v0.94` is the planned secure-execution, trust-convergence, and temporal
self-projection milestone.

It should bind together:

- secure execution model
- policy engine and capability-aware authorization convergence
- identity/auth and trust boundary closure
- isolation and secrets/data-governance architecture
- signed/queryable trace and reasoning/provenance closure
- bounded mental time travel / temporal self-projection

## Milestone Role

`v0.94` should establish:

- secure execution as a first-class runtime architecture band
- explicit policy, identity, and trust convergence beyond the `v0.93`
  governance layer
- runtime/provider isolation and secrets/data-governance boundaries
- trace as a queryable reasoning substrate rather than only an audit surface
- bounded temporal self-projection built on chronosense, identity continuity,
  memory, simulation, and inspectable counterfactual reasoning

`v0.94` should not absorb payments, settlement, or Lightning / `x402`
integration. Those belong to `v0.94.1`.

## Dependency Boundary

`v0.94` depends on:

- `v0.92` identity, continuity, and birthday evidence
- `v0.93` citizenship, governance, and enterprise-security foundations
- earlier trace, ObsMem, chronosense, and GHB reasoning surfaces

## Scope Summary

### In scope

- secure execution model
- policy engine architecture
- identity/auth convergence
- provider trust and isolation
- sandbox/runtime isolation
- secrets and data governance
- signed/queryable trace closure
- bounded mental time travel / temporal self-projection

### Out of scope

- payments, settlement, or Lightning rails
- production financial instruments
- replacing the `v0.93` governance milestone

## Document Map

- Vision: [VISION_v0.94.md](VISION_v0.94.md)
- Design: [DESIGN_v0.94.md](DESIGN_v0.94.md)
- WBS: [WBS_v0.94.md](WBS_v0.94.md)
- Sprint plan: [SPRINT_v0.94.md](SPRINT_v0.94.md)
- Decisions: [DECISIONS_v0.94.md](DECISIONS_v0.94.md)
- Demo matrix: [DEMO_MATRIX_v0.94.md](DEMO_MATRIX_v0.94.md)
- Milestone checklist: [MILESTONE_CHECKLIST_v0.94.md](MILESTONE_CHECKLIST_v0.94.md)
- Release plan: [RELEASE_PLAN_v0.94.md](RELEASE_PLAN_v0.94.md)
- Release notes: [RELEASE_NOTES_v0.94.md](RELEASE_NOTES_v0.94.md)
- Feature plans: [features/README.md](features/README.md)

## Success Criteria

- secure execution and trust boundaries are explicit and reviewable
- signed/queryable trace has a clear tracked home
- MTT is scheduled as a named feature rather than a floating research note
- payments remain downstream in `v0.94.1`
