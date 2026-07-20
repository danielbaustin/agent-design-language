# Structured Planning Prompt

Template: 1.0.0

Issue: 5551

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Establish exact authority, repair #5527 S2-S4, and validate terminal parity.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Establish and bind exact repair authority",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Repair #5527 S2-S4 and validate terminal parity",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  }
]

## Invariants

- No source files change
- #5527 remains closed-out and claim-free
- Only S2-S4 advance
- Receipt and local record remain exact

## Risks

- Stale receipt CAS must fail closed
- Unexpected semantic drift must stop publication

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5551/design.md

Digest: 16120fff5843e545d492463285a55139fb36b25ca275a35466e09d29360854b5

## Diagram

.csdlc/prepared/issues/5551/diagram.mmd

Digest: ad4057c83aa0bcdb2d532d0be1cf90bb38e56d43c8427c6246d28dd2a26f2ade

## Stop Conditions

- Any source file changes
- Any #5527 field besides generation, digests, audit, and S2-S4 changes
- Receipt parity or doctor fails

## Handoff

Proceed only after doctor readiness.
