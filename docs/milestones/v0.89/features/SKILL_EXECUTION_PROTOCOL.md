# Skill Execution Protocol

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning doc: `.adl/docs/v0.89planning/SKILL_EXECUTION_PROTOCOL.md`
- Planned WP: `WP-06`

## Purpose

Define the concrete invocation lifecycle for skills in ADL.

## Scope

`v0.89` should standardize:
- invocation identity and context
- input validation expectations
- execution lifecycle stages
- output and error contracts
- trace emission requirements

## Main Runtime Commitments

- invocation structure is explicit and bounded
- inputs are validated before execution
- invocation outputs, artifacts, and errors are reviewer-legible
- skill execution can be cited as a deterministic protocol rather than an ad hoc shell habit

## Runtime Contract

`WP-06` now makes the bounded skill invocation lifecycle reviewer-visible inside the canonical
control-path proof set.

The canonical proof surfaces are:
- `control_path/skill_execution_protocol.json`
- `control_path/skill_model.json`
- `control_path/action_proposals.json`
- `control_path/mediation.json`
- `control_path/final_result.json`
- `control_path/summary.txt`

The bounded runtime semantics are:
- invocation identity is explicit and linked to the selected proposal plus the commitment-gate
  decision
- input validation remains a runtime concern rather than an implicit trust in model output
- execution authority is derived from runtime mediation, not from the presence of a `skill_call`
- output and error contracts are reviewer-legible through mediation, final result, and trace
  surfaces
- trace visibility is required before privileged execution can proceed

The lifecycle stages declared in `control_path/skill_execution_protocol.json` are:
- `proposed`
- `validated`
- `authorized`
- `trace_visible`
- `ready_for_execution`

The bounded invocation record makes these fields explicit:
- `invocation_id`
- `skill_id`
- `proposal_id`
- `decision_id`
- `invocation_kind`
- `invocation_context`
- `input_validation_expectation`
- `lifecycle_state`
- `authorization_decision`
- `output_contract_surfaces`
- `error_outcome_vocabulary`
- `trace_expectation`
- `temporal_anchor`

This keeps the invocation lifecycle explicit and deterministic enough to review without pretending
`WP-06` already owns the full composition graph, parallel multi-skill orchestration, or later
governance philosophy work.

## Non-Goals

- the full composition graph substrate
- higher-level governance philosophy

## Dependencies

- Skill Model
- Action Mediation Layer
- trace and review surfaces

## Exit Criteria

- the milestone package has a stable skill-invocation contract
- future issue wave seeding can map protocol concerns to code and test surfaces
