# Structured Planning Prompt

Template: 1.0.0

Issue: 5658

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Read the current v2 lifecycle root handling, add the smallest root/materialization guard and regression, run focused Rust tests on FastWork, perform exact-head review, publish, merge, and close out.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect current bind/store lifecycle root behavior and locate the unsafe primary-main write path",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement bound-worktree materialization and fail-closed primary-main guard",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused regression for newly-created worktree with ignored or absent .csdlc state",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused FastWork validation, review, publish, merge, and closeout",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- The implementation root and lifecycle root must match after binding
- Primary checkout on main must remain clean for non-bootstrap issue execution
- Existing claim, lock, and exact-revision protections stay fail-closed

## Risks

- Overbroad main-write guard could block legitimate bootstrap or read-only operations
- Materialization could accidentally fork lifecycle truth instead of moving it to the bound root
- Regression may need careful temp/worktree isolation to avoid local disk writes

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5658/retained/design.md

Digest: 189b48e4d4e95c977ca91cf04fc0d9cb51b22c9fd9273dff485ce5c40ef6481d

## Diagram

.csdlc/issues/5658/retained/diagram.mmd

Digest: 8f635a68bed24f3a68337c6a60d9a9e535f28e0d91d389ded22b2ff91c1b8380

## Stop Conditions

- Fix requires rewriting historical issue records
- Fix requires weakening claim, lock, or exact-revision checks
- FastWork is unavailable for worktree or Rust output
- Scope expands beyond typed v2 lifecycle roots

## Handoff

Proceed only after doctor readiness.
