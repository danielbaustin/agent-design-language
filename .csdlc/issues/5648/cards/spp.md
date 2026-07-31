# Structured Planning Prompt

Template: 1.0.0

Issue: 5648

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add the typed request and atomic store operation, test fail-closed CAS/authority behavior, review the exact head, and publish the narrow tooling fix.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Implement typed revoke request/result and atomic store operation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run focused tests, Clippy, exact review, and publish",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- CAS mismatch never mutates state
- phase is unchanged
- audit records prior owner and authority
- no automatic claim stealing

## Risks

- operator misuse
- stale request
- claim collision

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5648/design.md

Digest: 0e446588d03734935d2f8e1cc35060784bd7ff9cfe4e7debb60cdf30c76b3b2b

## Diagram

.csdlc/prepared/issues/5648/diagram.mmd

Digest: 8654a9fdc5a4985d4fccb3aac5db0a9ed52ca2f74c0e0af117030b49955b7df0

## Stop Conditions

- missing authority marker
- stale generation or digest
- claim id mismatch
- test failure

## Handoff

Proceed only after doctor readiness.
