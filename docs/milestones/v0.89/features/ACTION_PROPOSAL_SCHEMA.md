# Action Proposal Schema

## Metadata
- Milestone: `v0.89`
- Status: `Landed`
- Source planning input: local `v0.89` planning corpus
- Milestone home: `WP-05`

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

## Runtime Contract

`WP-05` now defines a bounded reviewer-facing proposal contract for the action-intent boundary.

The canonical runtime proof surface is:
- `control_path/action_proposals.json`

The bounded proposal schema used there makes these fields explicit:
- `proposal_id`
- `kind`
- `target`
- `arguments`
- `intent`
- `content`
- `confidence`
- `requires_approval`
- `metadata`
- `non_authoritative`
- `temporal_anchor`

The bounded proposal-kind vocabulary for `v0.89` is:
- `tool_call`
- `skill_call`
- `memory_read`
- `memory_write`
- `final_answer`
- `refuse`
- `defer`

The reviewer-facing semantics are:
- every proposal is explicitly non-authoritative
- the current bounded slice anchors proposals to selected candidate families rather than full live
  skill/tool invocation ids
- proposal validation and authority handoff remain the responsibility of the runtime mediation
  layer rather than the proposing model surface

This keeps action intent structured and reviewable without pretending `WP-05` already owns the
full skill invocation protocol, provider-native function calling, or later-band negotiation and
constitutional work.

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
- the control-path proof set emits a stable non-authoritative action-proposal artifact
