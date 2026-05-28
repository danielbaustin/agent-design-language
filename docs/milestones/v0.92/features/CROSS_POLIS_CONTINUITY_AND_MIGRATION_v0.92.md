# v0.92 Feature: Cross-Polis Continuity And Migration Planning

## Metadata

- Feature Name: Cross-Polis Continuity And Migration Planning
- Milestone Target: `v0.92`
- Status: planned
- Related issues: `#3377`, `#3434`
- Planning template set: `docs/templates/planning/1.0.0`

## Template Rules

This is a planning feature doc. It defines bounded continuity and migration
planning surfaces without claiming production migration, federation, or
inter-polis portability.

## Purpose

Define the first bounded planning surface for preserving identity and birthday
evidence across future movement without turning v0.92 into a production
cross-polis migration milestone.

## Context

v0.92 creates first-birthday identity evidence. Later governance and polis
milestones will need to know what it would mean to carry that evidence across
runtime, polis, or jurisdiction boundaries. This feature keeps that question
visible while deferring production migration semantics.

## Coverage / Ownership

v0.92 owns the non-production continuity/migration design note and explicit
non-goals. v0.93 owns governance consequences. Later milestones own actual
cross-polis portability, transport security, and operational migration.

## Overview

The feature should describe how stable name, identity root, continuity head,
memory grounding, capability envelope, ACP profile, witnesses, and receipts
would participate in a later movement story.

Key capabilities:

- map birthday evidence to future continuity-transfer inputs
- identify evidence that cannot move without new governance decisions
- preserve ambiguity and quarantine cases
- prevent copied state from masquerading as continuity

## Design

### Core Concepts

- Continuity-transfer input: a reference to identity and birth evidence that a
  later migration design may consume.
- Ambiguity marker: explicit record that continuity is unresolved or contested.
- Non-production migration note: reviewable design text, not runtime movement.

### Architecture

- Inputs: identity record, continuity record, memory-grounding references,
  witness set, receipt, ACP profile, ACIP transport-readiness evidence.
- Outputs: bounded design note, non-goals, risk list, and handoff markers.
- Interfaces: milestone docs and future governance/migration issue seeds.
- Invariants: no production migration claim; no continuity-by-copy claim.

### Data / Artifacts

- Cross-polis continuity planning note.
- Migration non-goals table.
- Future decision checklist.

## Execution Flow

1. Gather the v0.92 birthday evidence surfaces.
2. Identify which evidence is portable as reference material.
3. Identify which evidence requires later governance or transport decisions.
4. Record ambiguity, quarantine, and copied-state rejection cases.
5. Hand the design note to v0.93 and later migration planning.

## Determinism and Constraints

- Planning output must be deterministic over cited evidence.
- Copied state must not become migration proof.
- Any production claim must be deferred to a later milestone and decision.

## Integration Points

| System / Surface | Integration Type | Description |
| --- | --- | --- |
| Identity | read | Consumes stable name, identity root, and continuity head. |
| Memory grounding | read | Uses references and redacted projections, not raw private state. |
| Governance | handoff | Reserves v0.93 decisions about continuity consequences. |
| ACIP | handoff | Reserves transport-security and schema-access implications. |

## Validation

### Demo

- Demo script(s): N/A; this is a planning feature.
- Expected behavior: N/A; proof is reviewable design consistency.

### Deterministic / Replay

- Replay requirements: N/A for planning-only output.
- Determinism guarantees: cited source evidence and explicit non-goals must
  produce the same planning boundary.

### Schema / Artifact Validation

- Schemas involved: N/A.
- Artifact checks: planning-template validation and Markdown link checks.

### Tests

- Test surfaces: N/A for planning-only feature.

### Review / Proof Surface

- Review method: manual docs review.
- Evidence location: `docs/milestones/v0.92/` and future WP-11 artifacts.

## Acceptance Criteria

- Cross-polis continuity/migration design note exists.
- The note names portable evidence and non-portable claims.
- Copied-state and ambiguity cases are explicitly handled.
- Production migration, federation, and transport-security claims remain out
  of scope.

## Risks

- Risk: migration language could imply production portability.
- Mitigation: require explicit non-goals and defer operational claims.
- Risk: copied state could be mistaken for continuity.
- Mitigation: require continuity evidence and ambiguity markers.

## Future Work

- v0.93 governance can decide what continuity evidence means socially and
  legally inside a polis.
- Later milestones can implement operational migration and cross-polis
  transport after security and trace gates are ready.

## Notes

This feature exists because the birthday record will immediately raise
movement questions, even though v0.92 should not implement movement.
