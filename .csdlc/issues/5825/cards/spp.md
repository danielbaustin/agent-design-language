# Structured Planning Prompt

Template: 1.0.0

Issue: 5825

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement and prove the deterministic WP-08 birth decision, including complete disqualifying cases and explicit public-claim boundaries.

## Plan

Revision 26

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5817/#5801 terminal receipts and inspect adl-runtime-kernel contract/proof authorities before claiming the exact birthday.rs, lib.rs, tests, fixture, feature, and evidence paths.",
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
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Birth decisions are deterministic over canonical inputs and fail closed on missing or contradictory evidence.
- Startup, wake, restore, snapshot, copied state, migration, and admission are never sufficient birth evidence.
- Existing v0.91.x birthday non-claims and downstream work-package authority remain unchanged.

## Risks

- The birth decision could accept a lifecycle lookalike or incomplete evidence packet.
- The shared adl-runtime-kernel/src/lib.rs registration could collide with adjacent sequential sprint work.
- Claim-boundary prose could overstate personhood, citizenship, governance, migration, or launch readiness.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5825/design.md

Digest: 8e2add832a4f060967b714c1c3f0d776b5b0d5098ed5bd8f4f4cb0ae07164ec1

## Diagram

.csdlc/prepared/issues/5825/diagram.mmd

Digest: af3ccfe3d575efcd6afdf4b661f1a73ffef8cd1ac35a93da986e2d10d55ed3a9

## Stop Conditions

- WP-01/#5817 or WP-02A/#5801 terminal proof is stale, missing, or contradictory.
- Execution requires paths outside birthday.rs, lib.rs registration, tests/birthday.rs, tests/fixtures/birthday/, the WP-08 feature contract, or .csdlc/evidence/5825/ without explicit replan.
- Any declared negative case lacks deterministic fail-closed proof.

## Handoff

Proceed only after doctor readiness.
