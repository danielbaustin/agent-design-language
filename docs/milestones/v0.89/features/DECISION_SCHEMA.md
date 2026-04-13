# Decision Schema

## Metadata
- Milestone: `v0.89`
- Status: `Planned`
- Source planning doc: `.adl/docs/v0.89planning/DECISION_SCHEMA.md`
- Planned WP: `WP-04`

## Purpose

Define the bounded record shape that makes ADL decisions legible in trace, review, and governance.

## Scope

`v0.89` should establish the semantic record contract for:
- decision identity
- proposal or action under review
- outcome class
- decision maker or office
- policy / constraint bindings
- rationale
- downstream consequence
- temporal anchor

## Main Runtime Commitments

- decisions that matter architecturally must have an architecturally legible record
- outcome classes are not collapsed into generic success/failure
- rationale and constraint bindings remain reviewable

## Non-Goals

- full negotiation transcript capture
- every specialized decision subtype

## Dependencies

- Decision Surfaces
- Freedom Gate v2
- trace / review surfaces

## Exit Criteria

- reviewers can answer what was decided, why, and under what constraints
- decision-event language is consistent across planning docs and future issue wave seeding
