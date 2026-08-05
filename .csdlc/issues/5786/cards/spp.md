# Structured Planning Prompt

Template: 1.0.0

Issue: 5786

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
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5786/design.md

Digest: a8bdc7791a07d9dc6cbc50239115bad292609037e01949282a6c0acd75ee174d

## Diagram

.csdlc/prepared/issues/5786/diagram.mmd

Digest: 7e9fec9389fd1fb70076e8ecfe9717199033c94fc3382022268a3e25d0a8e300

## Stop Conditions

- Protected-path collision
- Contradictory dependency evidence
- Required proof cannot be produced within issue scope

## Handoff

Proceed only after doctor readiness.
