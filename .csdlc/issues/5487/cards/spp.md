# Structured Planning Prompt

Template: 1.0.0

Issue: 5487

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Model an explicit terminal repair request, validate authority and hashes, journal the receipt/artifact transaction, prove rollback and materialization, then repair #5467 through the typed route.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Map terminal receipt and retained artifact ownership and define the typed repair contract",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement atomic repair, rollback, and reconcile-terminal materialization",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused tests and repair #5467 through the typed route",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- No repair without explicit authority and exact CAS identity
- Artifact hashes are verified before and after the transaction
- Receipt and retained artifacts never expose mixed generations
- Failure is fail-closed and rollback-safe

## Risks

- Terminal receipts are intentionally immutable outside this narrow typed route
- AST and rendered-card digests must remain synchronized
- Repair must not reopen closed-out lifecycle state

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5487/retained/design.md

Digest: 81a40365f0d648c8c8d149c38d161a04d795654827a41cabb955c3eea07f89b6

## Diagram

.csdlc/issues/5487/retained/diagram.mmd

Digest: 7e76d4342be9738403c6ebf17ba7949033daaf9c768be8c2d310f5407ea5b439

## Stop Conditions

- Atomic success and rollback are proven
- #5467 retained artifacts are repaired through the typed route
- Focused tests and subagent review are green

## Handoff

Proceed only after doctor readiness.
