# Structured Planning Prompt

Template: 1.0.0

Issue: 5848

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

.csdlc/prepared/issues/5848/design.md

Digest: 12a8c97a937e85ff1ddbc6220b4346fa081484323af96736c09f3f840f3bce81

## Diagram

.csdlc/prepared/issues/5848/diagram.mmd

Digest: 25b8dec2b759e8dfba87fd2c6055a1f6a8063c69efc0115abfc869b27b849aed

## Stop Conditions

- Protected-path collision
- Contradictory dependency evidence
- Required proof cannot be produced within issue scope

## Handoff

Proceed only after doctor readiness.
