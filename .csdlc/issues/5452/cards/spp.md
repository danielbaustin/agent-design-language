# Structured Planning Prompt

Template: 1.0.0

Issue: 5452

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Inspect the current status flow, implement deterministic two-stage status composition, add mixed-result regressions, and run the focused contract proof.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Map the existing primary and summary status paths",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement fail-closed two-stage status composition",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add and run focused mixed-result regressions",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  }
]

## Invariants

- A required-stage failure must never produce wrapper success
- The implementation remains bounded to the two declared scripts
- Existing successful artifact generation remains intact

## Risks

- Shell errexit handling may bypass summary generation or overwrite the primary status
- Test seams may accidentally weaken production behavior

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5452/retained/design.md

Digest: 13807358fef099d94c02758ec21154632510bb448cdf562f33adbd52d160970e

## Diagram

.csdlc/issues/5452/retained/diagram.mmd

Digest: 948a4bc82749ac148df1dac8d6f09ebd8ac5c42e1b01856576952f7431a0a6a9

## Stop Conditions

- Required behavior cannot be tested without widening into AWS or CI workflow changes
- The current wrapper contract contradicts the issue acceptance criteria

## Handoff

Proceed only after doctor readiness.
