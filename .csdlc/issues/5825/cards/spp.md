# Structured Planning Prompt

Template: 1.0.0

Issue: 5825

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement and prove the deterministic WP-08 birth decision, including complete disqualifying cases and explicit public-claim boundaries.

## Plan

Revision 8

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5818 and #5819 terminal receipts, inspect the named Runtime v2 and milestone sources, and narrow exact protected paths before editing.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the versioned birth-decision contract, one valid fixture, and the complete table-driven disqualifier matrix.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run deterministic focused, negative, path-hygiene, and public-claim boundary lanes and retain exact-revision evidence.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve one bounded exact-head review and publish only with correct base and Closes #5825 linkage.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Birth decisions are deterministic over canonical inputs and fail closed on missing or contradictory evidence.
- Startup, wake, restore, snapshot, copied state, migration, and admission are never sufficient birth evidence.
- Existing v0.91.x birthday non-claims and downstream work-package authority remain unchanged.

## Risks

- A lifecycle lookalike could be accepted if the disqualifier matrix is incomplete.
- Shared Runtime v2 paths may collide with adjacent sprint implementation.
- Narrative language may overstate personhood, citizenship, governance, migration, or launch readiness.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5825/design.md

Digest: 2aad71a9b6c268efedd68fcae15e49dabc706fc42428eb048f61ff0b4d748b34

## Diagram

.csdlc/prepared/issues/5825/diagram.mmd

Digest: af3ccfe3d575efcd6afdf4b661f1a73ffef8cd1ac35a93da986e2d10d55ed3a9

## Stop Conditions

- Either dependency lacks terminal receipt-backed proof.
- The exact protected paths collide with another live claim.
- The complete negative matrix or exact-revision evidence cannot be produced within WP-08 scope.

## Handoff

Proceed only after doctor readiness.
