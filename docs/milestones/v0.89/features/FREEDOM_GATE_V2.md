# Freedom Gate v2

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning doc: `.adl/docs/v0.89planning/FREEDOM_GATE_V2.md`
- Planned WP: `WP-03`

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
