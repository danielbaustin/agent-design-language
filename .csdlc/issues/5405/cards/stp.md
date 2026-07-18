# Structured Task Prompt

Template: 1.0.0

Issue: 5405

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Resolve the three #5403 WP-13 findings without redesigning the scheduler or adding broad guild/Godel runtime work.

## Deliverables

- Corrected guild and Godel truth wording
- Economics duplicate-policy rejection
- Regression validation for duplicate semantics
- Parent closeout/handoff truth alignment

## Acceptance

1. Guild truth is downgraded to boundary_proven or a real producer/consumer path is implemented and proven
2. Godel docs consistently say admission readiness and retain not-invoked truth
3. Economics validators reject duplicates and retain regression tests
4. Parent closeout and v0.92 handoff agree with corrected claims

## Dependencies

- #5403 review packet
- WP-13 #4753/#4754/#4755 review packets
- Runtime v2 economics/civilization boundary validator

## Inputs

- docs/milestones/v0.91.7/review/wp13_guild_foundation_boundary_4755.md
- docs/milestones/v0.91.7/review/wp13_godel_constructability_boundary_4753.md
- docs/milestones/v0.91.7/review/wp13_economics_civilization_boundary_4754.md
- docs/milestones/v0.91.7/review/wp13_closeout_4640.md
- adl/src/runtime_v2/economics_civilization_boundary.rs
- adl/src/runtime_v2/tests/economics_civilization_boundary.rs

## Non Goals

- Full guild producer/consumer runtime implementation unless already trivial and bounded
- Hosted provider invocation proof
- Economics boundary redesign
