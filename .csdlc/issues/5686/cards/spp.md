# Structured Planning Prompt

Template: 1.0.0

Issue: 5686

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bind #5686 on current origin/main, apply the retained projection commits, compare terminal truth to the canonical receipt, run focused validation and exact-head review, then publish a narrow PR.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind #5686 on current origin/main and verify receipt and retained commit identities",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Apply the retained #5662 terminal projection and resolve only baseline-compatible lifecycle conflicts",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused typed validation and compare the projected record with the canonical receipt",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run bounded exact-head review, fix findings, and publish a PR targeting main",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Canonical receipt remains immutable
- #5662 remains closed_out
- Projection values remain receipt-matching
- No implementation source changes
- Main changes only through PR merge

## Risks

- Current main may conflict with the retained projection commits
- Terminal lifecycle records may include generated values that must remain exact
- The typed lifecycle may lack a dedicated post-closeout projection route

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5686/design.md

Digest: 079de390db78dcf9b2f9143fc3abf5328bb8795d32b17d4c368451f9756099f4

## Diagram

.csdlc/prepared/issues/5686/diagram.mmd

Digest: aa11d6057a4fcb0e6e48cacfa9cb6229517a84b3ba35f626cfb6c1ed20eef139

## Stop Conditions

- The retained record digest differs from the canonical receipt
- Applying the projection requires implementation changes
- A protected-path collision exists
- Focused lifecycle validation reports irreconcilable terminal truth

## Handoff

Proceed only after doctor readiness.
