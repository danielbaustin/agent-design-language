# v0.92 Feature: Identity, Stable Name, and Continuity

## Metadata

- Feature Name: Identity, Stable Name, and Continuity
- Milestone Target: `v0.92`
- Status: planned
- Related issues: `#3377`, `#3434`
- Planning template set: `docs/templates/planning/1.0.0`

## Template Rules

This is a planning feature doc. It defines required identity surfaces without
claiming implementation has landed.

## Status

Forward-planning feature contract for `v0.92`.

Related readiness issue: `#3377`.

## Purpose

Define the identity-bearing substrate that turns bounded runtime activity into
durable named continuity rather than anonymous or purely process-local
execution.

## Context

Birth needs identity that is more durable than process state. v0.92 should use
prior lineage and citizen-state primitives while defining the birthday-specific
identity root, stable name, and continuity proof.

## Coverage / Ownership

v0.92 owns the identity record and continuity evidence needed for the first
birthday. v0.90.3 retains ownership of lower-level citizen-state primitives.

## Overview

The feature should define how a named agent remains the same identity across
bounded cycles and how ambiguous continuity is represented.

## Design

The identity record should include stable name, identity root, aliases,
origin event, continuity head, memory grounding references, capability
reference, witness reference, and redaction policy.

## Execution Flow

1. Create identity root and stable name.
2. Attach continuity evidence across bounded cycles.
3. Reject lifecycle events that lack continuity.
4. Feed identity evidence into the birthday packet.

## Determinism and Constraints

Continuity must be evidence-based. Copied state, wake, and process restart
must not become identity continuity without the required record and witnesses.

## Integration Points

- v0.90.3 lineage/state primitives.
- v0.92 memory grounding and capability envelope.
- v0.92 birthday record.
- v0.93 governance handoff.

## Validation

Validation should include valid identity records, continuity-cycle fixtures,
ambiguous continuity cases, and negative lifecycle cases.

## Source Inputs

- `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`
- `docs/milestones/v0.92/README.md`
- `docs/milestones/v0.92/WBS_v0.92.md`
- `docs/planning/ROADMAP_RUNTIME_V2_AND_BIRTHDAY_BOUNDARY.md`
- `#3377`

## Scope

This feature should establish:

- stable names and alias policy
- identity root and continuity head semantics
- evidence-based continuity across bounded cycles
- separation between startup, wake, snapshot, admission, and true identity
  continuity
- downstream handoff into `v0.93` governance rather than governance-by-name
  alone

## Acceptance Criteria

- Identity record contract exists.
- Stable names and aliases are represented.
- Continuity across bounded cycles is evidence-backed.
- Startup, wake, snapshot, admission, and copied-state cases do not pass as
  continuity without evidence.

## Risks

- A display name could be mistaken for identity. Mitigation: require identity
  root and continuity head.
- Continuity could become magical. Mitigation: require lineage and witness
  evidence.

## Future Work

v0.93 can use identity evidence for governance. Later milestones can expand
cross-polis migration and portability.

## Notes

This feature should keep the identity surface practical and auditable.

## Non-goals

- legal personhood
- constitutional citizenship by mere existence
- replacing `v0.90.3` state/lineage primitives

## Completion Target

`v0.92`
