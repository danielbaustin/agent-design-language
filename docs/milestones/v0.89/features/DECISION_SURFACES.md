# Decision Surfaces

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning doc: `.adl/docs/v0.89planning/DECISION_SURFACES.md`
- Planned WP: `WP-04`

## Purpose

Make ADL points of choice explicit, bounded, and reviewable.

This feature defines where the runtime is permitted to:
- accept
- reject
- defer
- escalate
- reroute

## Scope

`v0.89` should identify and standardize decision surfaces across:
- pre-execution authorization
- commitment formation
- recovery / continuity handling
- delegation and routing
- review and merge gates
- policy-sensitive actions

## Main Runtime Commitments

- decision points are architectural surfaces, not hidden prompt behavior
- decision states are governed, named, and traceable
- later demos and review docs can point to concrete moments of choice

## Non-Goals

- the full decision-event wire format
- full delegation protocol
- full negotiation mechanics

## Dependencies

- Freedom Gate v2
- Action Mediation Layer
- Decision Schema

## Exit Criteria

- the milestone package consistently names the main decision states
- WBS and demo planning can refer to concrete decision surfaces without ambiguity
