# Structured Planning Prompt

Template: 1.0.0

Issue: 5406

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add narrowly typed lifecycle operations, prove their guards and portability, retain the historical authority packet, review, and merge before returning to #5403.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Define typed request and semantic-operation contracts",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement collision and lifecycle guards with focused tests",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Retain portable historical lifecycle authority evidence",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Review merge and apply operations back to #5403",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- All mutations remain typed and audited
- Claim overlap fails closed
- Card lifecycle transitions remain monotonic
- v1 sunset paths do not return

## Risks

- Overbroad claim amendment
- Post-execution card mutation could erase historical intent
- Proof-role replacement could overstate validation

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/issues/5406/retained/design.md

Digest: f7b944b576eed6f7cb93f6966fcdd46cffb4eef66f6eeb11f814a328211d35d4

## Diagram

.csdlc/issues/5406/retained/diagram.mmd

Digest: f1ab1b5d46837cec8164e9c002562f1365bd32b8302a9b8e4ddf78ba138864a3

## Stop Conditions

- Any operation bypasses collision or lifecycle guards
- Rendered cards require hand editing
- v1 command surfaces reappear
- Focused or full csdlc-v2 tests fail

## Handoff

Proceed only after doctor readiness.
