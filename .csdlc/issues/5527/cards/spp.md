# Structured Planning Prompt

Template: 1.0.0

Issue: 5527

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement the narrow repair operation, prove fail-closed atomic behavior, repair #5390, review, publish, merge, and close out.

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
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement and test atomic terminal SOR artifact-reference repair",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Repair #5390 and validate receipt parity",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Review, publish, merge, and close out",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "completed"
  }
]

## Invariants

- Separate authority issue is mandatory
- Target is closed-out and claim-free
- Replacement path is receipt-authenticated and byte-matched
- Receipt and target roll back together

## Risks

- Overbroad terminal mutation would weaken immutability
- Receipt refresh failure could split record truth
- Path-only comparison could authenticate the wrong artifact bytes

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5527/design.md

Digest: c1593baf422c79cc8ccf18cbf198fc06b7bf2d84f6c1e5453a43d4c0a1d3b52e

## Diagram

.csdlc/prepared/issues/5527/diagram.mmd

Digest: 4bb902cea59c5b483610ec80c8edab657f253fb3af614719ebed6732a3ad0c7b

## Stop Conditions

- Repair requires runtime changes
- Operation permits arbitrary terminal edits
- Rollback proof fails
- Exact review finds unresolved atomicity defects

## Handoff

Proceed only after doctor readiness.
