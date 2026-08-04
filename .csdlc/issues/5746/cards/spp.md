# Structured Planning Prompt

Template: 1.0.0

Issue: 5746

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bind #5746 to the aggregate branch, record the already-completed typed projection execution and validation, record independent review, publish a ready PR, and shepherd it.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind #5746 to the dedicated aggregate branch and exact protected projection set",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Materialize valid retained receipts through typed reconcile-terminal",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run receipt equality, doctor, and diff/path validation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run bounded independent aggregate review and fix actionable findings",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Publish and shepherd the ready PR closing #5746",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Receipt identity is authoritative
- No projection is included without doctor and receipt equality
- Excluded blockers remain non-claims
- Only issue #5746 is closed by publication

## Risks

- A stale or invalid receipt could project false terminal truth
- Aggregate scope could accidentally include unrelated paths
- Excluded issues could be mistaken for completed closeout
- Post-merge pruning could discard dirty worktrees

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/issues/5746/retained/design.md

Digest: 8d95791fbb571dac98ff3b1cf368407de2cc53c3d103b7a82f5f4d73d4834f36

## Diagram

.csdlc/issues/5746/retained/diagram.mmd

Digest: b88938a6e13d755d9e75154fbc82b5459c03af20c731127fcf7e50d71e174445

## Stop Conditions

- Any receipt/index mismatch
- Any doctor or diff-hygiene failure
- Any unrelated path in the aggregate diff
- Any actionable review finding
- Any requirement to edit main, force prune, or fabricate lifecycle history

## Handoff

Proceed only after doctor readiness.
