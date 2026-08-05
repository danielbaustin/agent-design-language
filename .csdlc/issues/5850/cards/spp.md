# Structured Planning Prompt

Template: 1.0.0

Issue: 5850

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Freeze the full v0.92 universe, reconcile live and typed terminal truth, classify each row with one owner/action, build an acyclic retry-safe PR-to-ceremony sequence, test stale/dirty/missing/partial cases, and exact-head review the plan.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Verify WP-28 terminal ancestry and freeze the complete live plus typed v0.92 issue/PR/receipt universe.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Reconcile every row across GitHub, typed phase/SOR/receipt/claim, worktree cleanup, release dependency, owner, and action.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Topologically order PR completion, finish, claim release, cleanup, WP-29, WP-30, umbrella closeout, and handoff acceptance.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Exercise stale, red, missing-review/receipt, active-claim, dirty, partial-release, duplicate-retry, unknown, and unowned negative cases.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve exact-head plan review and publish the non-mutating closeout packet.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- No tracked work on main
- No scope absorption across work packages
- Evidence claims remain exact-revision and source-grounded

## Risks

- Dependency drift
- Scope overlap
- Insufficient real-behavior proof

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5850/design.md

Digest: 00cc6a8f66cc35db3bf44b183e35bd969379fff600118ac1cfb0f667c9b7c3cc

## Diagram

.csdlc/prepared/issues/5850/diagram.mmd

Digest: ce44b443126a9cf4b3cc12290861a6bae0cd0d02289d05af13909228de637435

## Stop Conditions

- Protected-path collision
- Contradictory dependency evidence
- Required proof cannot be produced within issue scope

## Handoff

Proceed only after doctor readiness.
