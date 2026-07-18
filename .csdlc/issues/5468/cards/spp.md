# Structured Planning Prompt

Template: 1.0.0

Issue: 5468

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add bounded SRP normalization inside typed terminal reconciliation, prove projection and receipt parity, then regenerate #5452 terminal truth.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Confirm current terminal reconciliation and SRP validation invariants",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Normalize completed terminal SRP status inside the existing transaction",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add focused regression and regenerate #5452 projection",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "status": "completed"
  }
]

## Invariants

- Only evidence-backed terminal SRP status is normalized
- Receipt and projected record remain identical authorities
- No AWS or remote execution is used

## Risks

- Over-broad normalization could hide incomplete review truth
- Projection and retained receipt could diverge if normalization occurs outside the atomic transaction

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5468/retained/design.md

Digest: d614eae3ff34b1fb0b24c6411a8f12a09320c1491aa6e4e6d0fc98818cce8e4e

## Diagram

.csdlc/issues/5468/retained/diagram.mmd

Digest: 79ebcaa424c684197dfc29ec4ea66420594ecaebfa80fc9f48eb6b1b91777125

## Stop Conditions

- The repair requires arbitrary terminal card mutation
- Focused lifecycle proof cannot preserve receipt rollback guarantees

## Handoff

Proceed only after doctor readiness.
