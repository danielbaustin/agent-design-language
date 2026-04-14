# Skill Execution Protocol

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning input: local `v0.89` planning corpus
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
