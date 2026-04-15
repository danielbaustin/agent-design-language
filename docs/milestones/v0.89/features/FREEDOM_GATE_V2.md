# Freedom Gate v2

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning input: local `v0.89` planning corpus
- Milestone home: `WP-03`

## Purpose

Deepen the existing bounded Freedom Gate into a richer judgment surface that still preserves ADL's core rule:

> constraint lives in the substrate, not in the temperament of any single model.

## Scope

`v0.89` should strengthen the gate from a minimal decision boundary into a structured judgment boundary with:
- explicit allow / defer / refuse / escalate behavior
- richer consequence and policy evaluation
- signal-aware gate inputs
- clearer relationship to action mediation and decision records

## Main Runtime Commitments

- the gate remains bounded, inspectable, and replayable
- gate decisions are coupled to decision surfaces and decision records
- refusal, defer, and escalation are legitimate governed outcomes, not exceptions
- the gate consumes richer runtime context without becoming hidden model temperament

## Runtime Contract

`WP-03` now treats Freedom Gate as a concrete runtime and reviewer surface instead of a planning-only note.

The bounded runtime slice makes these judgment surfaces explicit:
- decision outcomes: `allow`, `defer`, `refuse`, and `escalate`
- consequence-aware gate input context alongside policy and evaluation signals
- reviewer-facing judgment metadata:
  - `judgment_boundary`
  - `required_follow_up`
  - `decision_record_kind`
- linked proof artifacts in the runtime-control bundle:
  - `learning/freedom_gate.v1.json`
  - `control_path/final_result.json`
  - `control_path/summary.txt`

The intended semantics are:
- `allow` means bounded commitment can proceed
- `defer` means bounded context or review is still missing, but escalation is not yet required
- `refuse` means policy or risk blocks commitment outright
- `escalate` means the runtime must surface a higher-judgment path instead of silently deferring

This keeps the gate as a substrate boundary rather than turning it into prompt rhetoric or an implicit model temperament.

## Non-Goals

- a complete moral philosophy engine
- full later constitutional/governance band completion
- adversarial red/blue runtime implementation, which belongs to `v0.89.1`

## Dependencies

- bounded Freedom Gate baseline from `v0.86`
- instinct and bounded-agency groundwork from `v0.88`
- decision/action mediation work in `v0.89`

## Exit Criteria

- `v0.89` planning docs agree on the gate's widened outcome vocabulary
- demos and WBS rows can cite a concrete gate contract instead of a vague future claim
- carry-forward to later constitutional/governance bands is explicit
- the runtime-control proof surface exposes judgment boundary and required follow-up metadata
