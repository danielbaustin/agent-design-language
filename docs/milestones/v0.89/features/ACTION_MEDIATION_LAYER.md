# Action Mediation Layer

## Metadata
- Milestone: `v0.89`
- Status: `Landed`
- Source planning input: local `v0.89` planning corpus
- Milestone home: `WP-05`

## Purpose

Define the authority boundary between model-emitted intent and runtime-authorized action.

The core ADL claim here is:

> models propose actions; the runtime decides and executes.

## Scope

`v0.89` should formalize:
- action proposals as non-authoritative intent
- validation / approval / rejection / revision / defer flow
- linkage from decision surfaces into actual skill/tool execution
- trace visibility for approved and rejected actions

## Main Runtime Commitments

- model output does not directly execute privileged effects
- mediation is explicit and reviewable
- action resolution and execution semantics remain deterministic enough to replay

## Runtime Contract

`WP-05` now makes the authority boundary reviewer-visible inside the bounded runtime-control proof
set.

The canonical proof surfaces are:
- `control_path/action_proposals.json`
- `control_path/mediation.json`
- `control_path/decisions.json`
- `control_path/freedom_gate.json`
- `control_path/summary.txt`

The bounded runtime semantics are:
- models or model-like cognitive surfaces produce non-authoritative action proposals
- the runtime mediation layer owns approval, rejection, defer, and escalation outcomes
- Freedom Gate remains the runtime authority for commitment in the current bounded slice
- approved execution is intentionally left abstract at the selected-candidate family level until
  `WP-06` lands the fuller skill invocation contract

The mediation outcome vocabulary used in `control_path/mediation.json` is:
- `approved`
- `rejected`
- `deferred`
- `escalated`

This keeps the cognition-to-authority boundary explicit without pretending `WP-05` already owns
the entire downstream skill execution lifecycle or later governance layers.

## Non-Goals

- every provider-specific interaction detail
- the full constitutional umbrella
- the entire skill substrate in one document

## Dependencies

- Action Proposal Schema
- Decision Surfaces and Decision Schema
- Skill Model / Skill Execution Protocol

## Exit Criteria

- the milestone package clearly separates cognition from authority
- later code, tests, and demos have a stable contract for model-intent crossing into execution
- the runtime-control proof set emits a stable mediation artifact that records runtime authority,
  outcome, and required follow-up
