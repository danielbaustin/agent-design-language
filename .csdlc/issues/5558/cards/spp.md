# Structured Planning Prompt

Template: 1.0.0

Issue: 5558

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Remove stale live routes, strengthen active guidance guard coverage, and run Gate 10A as part of the C-SDLC owner lane.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Replace remaining live CLI and operator documentation references to v1 routes",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Remove the editor start route and update its active compatibility tests and demo surfaces",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add the actual Gate 10A proof to the C-SDLC owner lane",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused guidance tests and the full C-SDLC owner validation lane",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- No tracked changes on main
- Typed v2 lifecycle state only
- Historical evidence remains unchanged

## Risks

- Active documentation outside the original guard can retain executable v1 instructions
- Owner-lane proof may drift from the final-authority test

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5558/retained/design.md

Digest: d896cf8aacc8812eca00458a27a92efde8712761eaba7a53ada619cc6dadfe24

## Diagram

.csdlc/issues/5558/retained/diagram.mmd

Digest: 61bef6ab06dc1b76292aa0d596571744eb4b2d4ec3b141cb99ebfc0e9ec99829

## Stop Conditions

- A required path is owned by another live claim
- The correction requires changing historical evidence

## Handoff

Proceed only after doctor readiness.
