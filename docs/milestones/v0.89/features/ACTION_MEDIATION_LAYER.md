# Action Mediation Layer

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning doc: `.adl/docs/v0.89planning/ACTION_MEDIATION_LAYER.md`
- Planned WP: `WP-05`

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
