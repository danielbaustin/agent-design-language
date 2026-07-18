# Structured Planning Prompt

Template: 1.0.0

Issue: 5423

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Inventory terminal remediation evidence, update only qualifying register rows, validate unchanged nonterminal rows, and complete exact-revision review.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inventory terminal retained evidence for register remediation rows",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Apply the bounded canonical register reconciliation",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Complete focused validation and exact-revision independent review",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- Historical findings remain visible
- Only terminal remediation evidence can promote a row
- Other sessions' active work remains untouched

## Risks

- A live issue may be closed without terminal lifecycle truth
- Concurrent remediation sessions may finish during this pass

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5423/retained/design.md

Digest: e8471fb230d0908eb43425697c6e24c59b52ddea6db2dc2c4dfbf09166c7e620

## Diagram

.csdlc/issues/5423/retained/diagram.mmd

Digest: 6b266de02092cdd482103b5744d940e1b304866ec8c19db74442d065040548b1

## Stop Conditions

- The canonical register path is claimed by another active issue
- Terminal evidence is ambiguous or internally inconsistent
- Scope expands beyond review register reconciliation

## Handoff

Proceed only after doctor readiness.
