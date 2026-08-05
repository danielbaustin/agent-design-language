# Structured Planning Prompt

Template: 1.0.0

Issue: 5860

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Audit all children, prepare five disjoint sprint sets, integrate typed designs/cards, validate every packet and released claim, then run one independent exact-head review.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Audit all 41 children and preserve exact defect/readiness baseline",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "in_progress"
  },
  {
    "id": "S2",
    "action": "Prepare source-grounded designs and typed cards in five disjoint sprint lanes",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Integrate results, release preparation claims, and generate the 41-row readiness matrix",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run typed validation and independent exact-head readiness review; repair every finding",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- No product path changes
- No generic card accepted as ready
- No unapproved design accepted
- No active preparation claim left behind
- No child implementation begins

## Risks

- Large card count can hide generic or contradictory prose
- Parallel lanes can drift from canonical dependency truth
- Typed card edits can expose schema or claim-control defects

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5860/design.md

Digest: e1cfff83f30931bbc2de3fa7624859c0924b5661a45f6af7e416a90d45a3a707

## Diagram

.csdlc/prepared/issues/5860/diagram.mmd

Digest: 6f53c3e4107b7adf7e298a31c25674d8c8be3b2d3ac51ae58d82ed73ade33021

## Stop Conditions

- Any proposed product implementation
- Any unresolved child scope contradiction
- Any typed validation bypass
- Any overlapping preparation claim

## Handoff

Proceed only after doctor readiness.
