# Structured Planning Prompt

Template: 1.0.0

Issue: 5727

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Initialize and bind #5727, implement a typed CAS reacquire path, update dormant-state doctor truth, add focused lifecycle and #5354 regression proof, validate, review, and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect current claim validation, release, recovery, doctor, binary routing, and #5354 reproduction state.",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement typed compare-and-swap claim reacquisition with full collision and binding validation.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Update doctor classification and add focused released, expired, collision, stale, binding, audit, and #5354 tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused C-SDLC validation and exact-head review, fix findings, and publish with Closes #5727.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Lifecycle phase never rewinds during claim reacquisition.
- Audit history remains append-only.
- Only one overlapping live writer may exist.
- Mutations require a valid live covering claim.

## Risks

- Dormant classification could accidentally authorize writes without a claim.
- Incomplete overlap validation could admit concurrent writers.
- Reacquisition could mutate phase or discard release evidence.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5727/design.md

Digest: 8ffd654085218d958d779afc8dcd90d6ea629e3fe67444e5d58bd231017b8b6b

## Diagram

.csdlc/prepared/issues/5727/diagram.mmd

Digest: a9576bc71e055fe939e8e384efb4e5edd5f962015a11c79cea049c60fa40c5c8

## Stop Conditions

- The primary checkout is no longer clean on main.
- A live overlapping claim owns the implementation paths.
- Typed initialization or binding reports stale or corrupt state.
- Focused lifecycle regression proof fails outside bounded #5727 scope.

## Handoff

Proceed only after doctor readiness.
