# Skill Model

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning doc: `.adl/docs/v0.89planning/SKILL_MODEL.md`
- Planned WP: `WP-06`

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
