# Structured Planning Prompt

Template: 1.0.0

Issue: 5843

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare the exact issue scope, implement the required outcome, run focused proof, resolve one bounded review, and publish with closing linkage.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Prepare exact scope, design, paths, and validation plan",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the required outcome and focused proof",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Resolve one bounded pre-PR review and publish with closing linkage",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- No tracked work on main
- No scope absorption across work packages
- Evidence claims remain exact-revision and source-grounded

## Risks

- Dependency drift
- Scope overlap
- Insufficient real-behavior proof

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5843/design.md

Digest: 45228ff77724fc17702904d5ae76334e81f10864c905136354d64b37411e7c44

## Diagram

.csdlc/prepared/issues/5843/diagram.mmd

Digest: 73e6a862f5895da6242eb3738982b2fa1e53fa5b49a9df1fb7e32e8f3f56c0c8

## Stop Conditions

- Protected-path collision
- Contradictory dependency evidence
- Required proof cannot be produced within issue scope

## Handoff

Proceed only after doctor readiness.
