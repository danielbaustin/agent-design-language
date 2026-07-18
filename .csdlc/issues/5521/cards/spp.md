# Structured Planning Prompt

Template: 1.0.0

Issue: 5521

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Establish exact authority, repair #5518 S4, and validate terminal parity.

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
    "action": "Repair #5518 S4 and validate terminal parity",
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
- #5518 remains closed-out and claim-free
- Only S4 advances
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

.csdlc/prepared/issues/5521/design.md

Digest: bca59d332085ea9340f70b847ad980d51593af155c7347e8174bb5d5ee1d470e

## Diagram

.csdlc/prepared/issues/5521/diagram.mmd

Digest: 2647cf680f0ee8f816487afaa37263f02cc8c6c7a632bafc1d56c186626f353b

## Stop Conditions

- Any source file changes
- Any #5518 field besides generation, digests, audit, and S4 changes
- Receipt parity or doctor fails

## Handoff

Proceed only after doctor readiness.
