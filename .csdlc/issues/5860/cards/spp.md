# Structured Planning Prompt

Template: 1.0.0

Issue: 5860

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Audit all children, prepare six disjoint sprint sets, integrate typed designs and cards, validate every packet and released claim, then run independent exact-head review.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Audit all 58 execution issues and preserve the exact defect and readiness baseline",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Prepare source-grounded designs and typed cards in six disjoint sprint lanes, with #5855 owning WP-04 architecture and security gating and #5862 separately owning WP-04 implementation coordination after #5821 is terminal",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Integrate results, release preparation claims, and generate the exact 58-row readiness and ownership evidence",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run typed validation and independent exact-head readiness review, then repair every actionable finding",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
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

Digest: efb288c53c47046c56e512a9cd277b4b8803965a595fa2870c253b36e429d748

## Diagram

.csdlc/prepared/issues/5860/diagram.mmd

Digest: e2a3e34b6407cb62639a89ed201ac32976da3a2ce4a7f270cdc8a6bac3423209

## Stop Conditions

- Any proposed product implementation
- Any unresolved child scope contradiction
- Any typed validation bypass
- Any overlapping preparation claim

## Handoff

Proceed only after doctor readiness.
