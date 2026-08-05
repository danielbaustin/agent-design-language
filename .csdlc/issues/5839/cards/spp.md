# Structured Planning Prompt

Template: 1.0.0

Issue: 5839

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify #5834, #5835, and the v0.93 allocation; inventory exact accepted artifacts; author consumer, redaction, forbidden-inference, and unresolved-decision columns; update ADR 0033 planning without acceptance; validate completeness and review from producer/consumer perspectives.

## Plan

Revision 12

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5834, #5835, and the v0.93 allocation/owner",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Inventory exact v0.92 evidence and author the row-level handoff map",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Update ADR 0033 planning and unresolved decisions without acceptance",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run completeness, path, redaction, and forbidden-inference checks",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve exact-head producer/consumer review",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Define and verify handoff rollback: restore the owned ADR plan, retain the rejected evidence map, and leave accepted v0.92 proof and v0.93 allocation unchanged.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Every handoff row resolves to exact evidence or an explicit blocker
- Every allowed use has a named v0.93 consumer
- Private inputs expose only governed projections
- Birthday evidence never establishes governance authority by itself

## Risks

- v0.93 allocation may be absent or vague
- Handoff prose may imply citizenship or standing
- Evidence could drift between draft and review

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5839/design.md

Digest: 53896fe1d51ca055df68e1a834372be8f4de1459c93410d1253306f24c2a67c9

## Diagram

.csdlc/prepared/issues/5839/diagram.mmd

Digest: 6b99d2a8f3fe9e3cb9e6d79e173dbf3833884262ae15d3fe86cc4bce9568d699

## Stop Conditions

- No v0.93 allocation/owner exists
- #5834 or #5835 is not accepted
- A required source cannot be safely projected
- Rollback would mutate accepted v0.92 evidence, fabricate a v0.93 allocation, or grant governance authority.

## Handoff

Proceed only after doctor readiness.
