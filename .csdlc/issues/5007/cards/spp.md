# Structured Planning Prompt

Template: 1.0.0

Issue: 5007

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare #5007 as a proof-gated ADR acceptance handoff: current main integrated, all six cards and design/diagram completed, one bounded GPT-5.5 preparation review retained, and execution left blocked on actual #4760 Memory Palace proof.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Integrate current origin/main 51bc5ae51b57c19dbab693af1c5a45142995f4e5 into the existing #5007 preparation branch and verify the worktree path/branch.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Repair all six cards, design, and diagram to the issue-specific proof-gated ADR acceptance plan with exact dependencies, intended paths, COTS, budgets, PVF lanes, rollback, and no-deferral boundaries.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run one bounded GPT-5.5 preparation review over the preparation packet and fix actionable preparation findings.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused preparation validation and record stale claim/closeout reconciliation as an execution-time gate, not a preparation blocker.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- Preparation never writes the accepted ADR candidate or claims Memory Palace architecture acceptance.
- Every later ADR claim must map to retained implementation proof from #4760 plus relevant Chronosense/ObsMem/runtime evidence.
- Stale claim reconciliation and typed closeout receipts remain execution-time lifecycle work, not preparation blockers.
- No `/private/tmp` use; all build/scratch/validation output stays in the worktree or `/Volumes/FastWork`.
- Future execution must recheck `origin/main`, claim ownership, and dependency proof at its exact head.

## Risks

- #4760 may remain open or may close without proof strong enough for ADR acceptance.
- Closed dependency issues #4765/#4768/#4771 may not by themselves prove the exact Memory Palace boundary #5007 needs.
- ADR numbering may advance before execution; if so, the intended candidate path must be renumbered before drafting.
- The current preparation claim is stale; execution must acquire a fresh typed claim before mutation.

## Estimates

{
  "elapsed_seconds": 2700,
  "total_tokens": 18000,
  "validation_seconds": 900
}

## Design

.csdlc/prepared/issues/5007/design.md

Digest: 34b16fad35bd3cc21d9f3274b4e38297c0539e42714d840615fb8f78861dd398

## Diagram

.csdlc/prepared/issues/5007/diagram.mmd

Digest: df6dd8d0462e234ce4bc5e358a9ae19a0f7e9c0c35bba5ad20bf718e2d8ac8e7

## Stop Conditions

- #4760 lacks actual completed implementation proof when execution is requested.
- Execution would require drafting the ADR during this preparation-only pass.
- Any command attempts to write on `main` or under `/private/tmp`.
- Future execution cannot acquire a fresh non-stale issue claim.
- A review finding shows the preparation packet overclaims proof or hides a deferral.

## Handoff

Proceed only after doctor readiness.
