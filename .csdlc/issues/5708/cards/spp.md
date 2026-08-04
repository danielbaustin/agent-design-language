# Structured Planning Prompt

Template: 1.0.0

Issue: 5708

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Validate and review the exact merged documentation head, reconcile PR #5709, and retain terminal evidence.

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
- Public launch claims remain assigned to follow-up issue #5711.

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

.csdlc/issues/5708/retained/design.md

Digest: 9784671a4137e538c4caf23874fef04d8208d67e9cac8536e7b6ecdf50be5972

## Diagram

.csdlc/issues/5708/retained/diagram.mmd

Digest: 01ec038a0453f5358691860c05ccc0cc1249df22876654ea70c263ba0aaac857

## Stop Conditions

- The issue worktree is not at exact implementation head.
- The typed binary rejects the recovery transition.
- Remote PR identity differs from the recorded issue and branch.

## Handoff

Proceed only after doctor readiness.
