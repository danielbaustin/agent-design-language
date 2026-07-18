# Structured Planning Prompt

Template: 1.0.0

Issue: 5518

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement the narrow repair operation, prove fail-closed atomic behavior, repair #5516, review, publish, merge, and close out.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Design and bind the terminal repair authority",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement and test atomic terminal plan-step repair",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Repair #5516 and validate receipt parity",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Review, publish, merge, and close out",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- Separate authority issue is mandatory
- Target is closed-out and claim-free
- Only a forward step completion is allowed
- Receipt and target roll back together

## Risks

- Overbroad terminal mutation would weaken immutability
- Receipt refresh failure could split record truth

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5518/design.md

Digest: a1c8692ee17db438145a33c561c81eb756a748dab107a9403efe96724778797a

## Diagram

.csdlc/prepared/issues/5518/diagram.mmd

Digest: 2fc0be20bfe7968d65ae59096c37893c65532f51cb289603a54bf9716aced83c

## Stop Conditions

- Repair requires runtime changes
- Operation permits arbitrary terminal edits
- Rollback proof fails
- Exact review finds unresolved atomicity defects

## Handoff

Proceed only after doctor readiness.
