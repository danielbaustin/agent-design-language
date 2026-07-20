# Structured Planning Prompt

Template: 1.0.0

Issue: 5569

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bind #5569, apply four exact terminal step repairs, validate parity and doctor, review, and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "repair",
    "action": "Repair four #5547 terminal plan steps and prove parity",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  }
]

## Invariants

- No hand-edited cards
- No fabricated evidence
- Receipt and projection CAS
- No AWS or Spot

## Risks

- Completing an unproven step would fabricate lifecycle truth

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5569/retained/design.md

Digest: 4ccd1b246a65bb5dc475aa6c4f0bf607e4c238a1481c7078261489a297fa2c52

## Diagram

.csdlc/issues/5569/retained/diagram.mmd

Digest: 6e86ac1f0e6dff62099dd5f22f65e75678534cbe39584a7975ca990a4c500074

## Stop Conditions

- Any missing retained step evidence
- Any need to hand-edit cards

## Handoff

Proceed only after doctor readiness.
