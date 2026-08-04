# Structured Planning Prompt

Template: 1.0.0

Issue: 5748

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Exhaustively classify the live closed v0.91.8 issue set; recover, transport, or reconcile terminal authority only through typed C-SDLC v2; materialize the origin/main terminal delta on the dedicated #5748 branch; and retain exact-head fail-closed exceptions whenever implementation or evidence does not justify a receipt.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Inventory remote disposition, local projection, worktree ownership, and retained receipt state for every target",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Create or repair terminal receipts only through issue-local typed routes, including the named special cases",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Materialize validated receipts in the #5748 authority worktree and prove doctor, receipt, artifact, and diff hygiene",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  }
]

## Invariants

- retained receipt identity is the terminal authority
- tracked projections remain claim-free and closed_out after reconciliation
- remote PR or no-PR disposition is never inferred away
- dirty and foreign-owned work remains intact
- all generated lifecycle mutation is typed and atomic

## Risks

- receipt identity can conflict with stale tracked projection truth
- false closed_no_pr disposition may lack a supported typed correction route
- retained authored-artifact paths can be non-portable or inconsistent
- active issue claims can collide with aggregate projection ownership

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5748/design.md

Digest: 513e0ade860af26b44c538de5e0a83cc73756e41774a8d6bb72c31d6304c6bc8

## Diagram

.csdlc/prepared/issues/5748/diagram.mmd

Digest: 8c28e943adbdc36a168acdfca5b0dae21bafe4c56983dbd48d16bcdf9e413b85

## Stop Conditions

- typed receipt identity conflict
- missing supported repair route
- dirty or foreign-owned worktree would need destructive cleanup
- remote disposition cannot be verified
- doctor or receipt equality fails after typed reconciliation

## Handoff

Proceed only after doctor readiness.
