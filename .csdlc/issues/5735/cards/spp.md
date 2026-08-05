# Structured Planning Prompt

Template: 1.0.0

Issue: 5735

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Validate and review the exact merged documentation head, reconcile PR #5736, and retain terminal evidence.

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
- No publication claim is widened.

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

.csdlc/issues/5735/retained/design.md

Digest: d28eb00a9799085a11f9278dea60b7268b8700a30dc1e699ee052a666978b16a

## Diagram

.csdlc/issues/5735/retained/diagram.mmd

Digest: e4f8fb9e4bdcf12c4fce13fe8dea9acecca12662a3b54e33ee973e94c7cad3c0

## Stop Conditions

- The issue worktree is not at exact implementation head.
- The typed binary rejects the recovery transition.
- Remote PR identity differs from the recorded issue and branch.

## Handoff

Proceed only after doctor readiness.
