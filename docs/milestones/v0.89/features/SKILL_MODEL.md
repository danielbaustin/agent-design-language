# Skill Model

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning input: local `v0.89` planning corpus
- Milestone home: `WP-06`

## Purpose

Make skills a first-class ADL concept rather than an informal prompt-packaging convention.

## Scope

`v0.89` should define:
- what a skill is and is not
- skill identity and purpose
- input and output contracts
- stop conditions and reviewability expectations
- the relationship among aptitudes, capabilities, and skills

## Main Runtime Commitments

- skills are bounded reusable execution units
- skill definitions are distinct from skill invocations
- skills are part of explicit system intelligence rather than hidden model behavior

## Runtime Contract

`WP-06` now makes the bounded skill definition reviewer-visible inside the canonical control-path
proof set.

The canonical proof surfaces are:
- `control_path/skill_model.json`
- `control_path/action_proposals.json`
- `control_path/mediation.json`
- `control_path/summary.txt`

The bounded runtime semantics are:
- a skill is an explicit reusable execution unit, not just an implicit prompt habit
- skill definitions are distinct from selected invocations and remain non-authoritative until
  runtime mediation authorizes execution
- the current bounded slice records the selected execution-unit kind so reviewers can distinguish a
  governed `skill_call` from a `tool_call`, memory operation, or non-skill outcome
- the skill model artifact explicitly distinguishes skills from provider capabilities and raw
  aptitudes rather than collapsing them into one hidden category

The current `control_path/skill_model.json` contract makes these fields explicit:
- `skill_id`
- `selection_status`
- `purpose`
- `bounded_role`
- `input_contract_fields`
- `output_contract_surfaces`
- `stop_condition`
- `distinguished_from`
- `temporal_anchor`

This keeps the definition of a skill stable enough for review and future implementation without
pretending `WP-06` already owns multi-skill composition, learning-band skill mutation, or the
later aptitude/capability taxonomy work.

## Non-Goals

- full multi-skill composition semantics
- all runtime substrate details
- later learning/identity-band work

## Dependencies

- Action Mediation Layer
- Skill Execution Protocol
- later aptitude/capability work in `v0.92`

## Exit Criteria

- the milestone package has one stable conceptual definition of a skill
- later implementation work can distinguish skills from provider capabilities and raw aptitudes
