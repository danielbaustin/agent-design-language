# Structured Planning Prompt

Template: 1.0.0

Issue: 5728

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Validate and review the exact merged documentation head, reconcile PR #5729, and retain terminal evidence.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Validate the exact committed documentation patch.",
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
    "action": "Record bounded exact-head review evidence.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Reconcile the existing merged PR and retain terminal evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- The implementation and merge SHAs remain exact.
- No product or planning content changes during recovery.
- Memory Palace remains deferred under ADR 0051.

## Risks

- Retroactive lifecycle recovery could overstate historical validation or review.
- A stale branch could differ from the published head.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5728/retained/design.md

Digest: e2b4bd34fc5f9d671ef95c54396d862cf36fc6ae280a14fe9abd0f8be33f18a8

## Diagram

.csdlc/issues/5728/retained/diagram.mmd

Digest: d60f572805e6599dab28aa1e7930283df706ce863a8fffe920f19e42e8199f73

## Stop Conditions

- The issue worktree is not at exact implementation head.
- The typed binary rejects the recovery transition.
- Remote PR identity differs from the recorded issue and branch.

## Handoff

Proceed only after doctor readiness.
