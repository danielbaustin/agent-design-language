# Structured Planning Prompt

Template: 1.0.0

Issue: 5306

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Revalidate authority, remove one exact owner slice, prove v2, review, merge, and recompute before continuing.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Revalidate D1 evidence and explicit deletion approval",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Apply one exact bounded manifest slice while preserving sunset surfaces",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Run independent proof, exact-revision review, and recompute removal truth",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- No unapproved deletion
- Useful retained code is allowed with owner and justification
- LoC targets cannot suppress needed code
- Every decision remains rollback/review visible

## Risks

- Overbroad path deletion
- Stale approval evidence
- False confidence from aggregate LoC

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

docs/architecture/csdlc-v2/gate10d2/DESIGN.md

Digest: 9b7c6ff1029d318ff44bdf91b6cfa939f69a4be9396b0a711361937bf68812d1

## Diagram

docs/architecture/csdlc-v2/gate10d2/DIAGRAM.mmd

Digest: dda9acbc40f48002da88859feb8fe7f7162f9a6a3ea08db58d8e4f631fd95b07

## Stop Conditions

- Eligibility is false or stale
- Approval is missing or mismatched
- A protected sunset path enters the slice
- Focused or full v2 proof fails

## Handoff

Proceed only after doctor readiness.
