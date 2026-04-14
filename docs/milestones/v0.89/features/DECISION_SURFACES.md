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

## Runtime Contract

`WP-04` now treats the main runtime decision points as a reviewer-facing proof surface rather
than only a planning abstraction.

The bounded runtime slice makes these decision surfaces explicit:
- `delegation_and_routing.route_selection`
- `recovery_continuity.reframing`
- `pre_execution_authorization.commitment_gate`

The reviewer-facing proof surface is:
- `control_path/decisions.json` as the canonical decision-surface and decision-record artifact
- `control_path/arbitration.json`, `control_path/reframing.json`, and `control_path/freedom_gate.json`
  as the linked stage-local sources
- `control_path/summary.txt` as the compact human-readable companion

The intended semantics are:
- route selection may `accept` the fast path or `reroute` into a slower bounded path
- reframing may `accept` the current frame or `reroute` through bounded reframing
- commitment gating may `accept`, `reject`, `defer`, or `escalate` before action commitment

This keeps points of choice explicit without pulling full action mediation or later governance
bands into `WP-04`.

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
- the runtime-control proof set exposes a reviewer-facing decision-surface artifact
