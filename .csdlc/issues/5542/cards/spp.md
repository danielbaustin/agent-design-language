# Structured Planning Prompt

Template: 1.0.0

Issue: 5542

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Repair non-overlapping canonical docs, extend the validator, reconcile the register after claim release, then review and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Confirm live #4644/#5539 truth, remaining gate set, bridge authority, and coordination boundaries",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Repair canonical closeout, bridge-precedence, and date-semantics entrypoints",
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
    "action": "Extend and run focused documentation validation with retained evidence",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Record exact review, publish, merge when green, and close out truthfully",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- Closed and merged states come from live GitHub evidence
- v0.91.8 is the bridge authority before v0.92
- Sibling release gates stay explicit
- No AWS use

## Risks

- Concurrent WP-18 work can conflict on the sprint-review register
- Historical creation dates can be mistaken for current verification dates
- Direct-v0.92 language can bypass the reviewed v0.91.8 bridge

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5542/retained/design.md

Digest: fab781fa23c18fe3049476dfadc24785057e8e01555d71a1ff4c51d49b9deb0e

## Diagram

.csdlc/issues/5542/retained/diagram.mmd

Digest: 8288260a3a03c8047f3a6278a2c91ee884da009b7e4610925218bbbd329564f4

## Stop Conditions

- A required path remains actively claimed by another issue without handoff
- Live issue or PR state contradicts the proposed closeout wording
- The repair would require sibling implementation or AWS

## Handoff

Proceed only after doctor readiness.
