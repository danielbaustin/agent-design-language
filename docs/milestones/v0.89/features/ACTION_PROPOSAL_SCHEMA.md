# Action Proposal Schema

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning doc: `.adl/docs/v0.89planning/ACTION_PROPOSAL_SCHEMA.md`
- Planned WP: `WP-05`

## Purpose

Define the canonical contract by which model cognition may express intent to act without gaining execution authority.

## Scope

`v0.89` should standardize:
- proposal kinds
- required and conditional fields
- proposal validation expectations
- the non-authoritative relationship between intent and execution

## Main Runtime Commitments

- proposals are structured and bounded
- invalid proposals are rejected deterministically
- proposals feed the Action Mediation Layer rather than bypassing it

## Non-Goals

- the full decision-event record
- the entire post-approval execution lifecycle

## Dependencies

- Action Mediation Layer
- Decision Schema
- Skill invocation surfaces

## Exit Criteria

- the milestone package uses a consistent intent contract
- action-proposal handling is strong enough to drive future implementation and review surfaces
