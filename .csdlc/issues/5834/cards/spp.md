# Structured Planning Prompt

Template: 1.0.0

Issue: 5834

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

.csdlc/prepared/issues/5834/design.md

Digest: 32faee838e6dfc5b8c5eb4121cdc1aabfea55d559069a5bff42d86c2513efba2

## Diagram

.csdlc/prepared/issues/5834/diagram.mmd

Digest: 4fbf9141c2b5d1090bf07fd82a10d5e1b4ebc70e82ffef84456cb5684d5dd2ec

## Stop Conditions

- Protected-path collision
- Contradictory dependency evidence
- Required proof cannot be produced within issue scope

## Handoff

Proceed only after doctor readiness.
