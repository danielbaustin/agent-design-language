# Structured Planning Prompt

Template: 1.0.0

Issue: 5779

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Extract cleanup into one standalone typed command, add compatibility indexing and parity validation, prove non-destructive classifications, then publish and finish #5779.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Initialize and bind #5779",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement standalone cleanup and legacy compatibility index",
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
    "action": "Run focused validation and exact-head review",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Publish, shepherd, and finish #5779",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Cleanup is housekeeping and never lifecycle truth
- Dirty and ambiguous paths fail non-destructively
- Current derived terminal authority is independent of legacy receipts
- Historical records and receipts are never rewritten

## Risks

- Cleanup could delete user work if path classification is incomplete
- Legacy receipts could accidentally remain competing authority
- Worktree relocation or concurrency could produce stale topology observations

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5779/design.md

Digest: 286b1824e9e5fe467106ef630ef1e8710cc2c257ddabb969b220c923a9aef91a

## Diagram

.csdlc/prepared/issues/5779/diagram.mmd

Digest: 692c322f2cba4034328e5f59df1ac9eb64c4f253385d4dee7998c1b145960fe9

## Stop Conditions

- Any claim collision on declared paths
- Any required force deletion or manual lifecycle edit
- Any parity mismatch or actionable exact-head review finding

## Handoff

Proceed only after doctor readiness.
