# Structured Planning Prompt

Template: 1.0.0

Issue: 5837

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

.csdlc/prepared/issues/5837/design.md

Digest: bab20e749946ab47caaff4f71a51475c223a68a42e68464b4b1153baa9d43137

## Diagram

.csdlc/prepared/issues/5837/diagram.mmd

Digest: 6e998bbfc4e909d907a0aedb02e0083df704316714c6bf03c5a82c5f8dcf9363

## Stop Conditions

- Protected-path collision
- Contradictory dependency evidence
- Required proof cannot be produced within issue scope

## Handoff

Proceed only after doctor readiness.
