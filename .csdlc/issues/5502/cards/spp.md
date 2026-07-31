# Structured Planning Prompt

Template: 1.0.0

Issue: 5502

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Render and validate six cards; freeze the pure convergence boundary, exact #5499/#5498 live merge plus ancestry gates, preparation-only ownership, COTS, budgets, PVF, deterministic security invariants, and future validation; commit and push preparation only without adding preparation review churn.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Complete six cards, design, diagram, dependency and preparation validators, exact protected paths, COTS, budgets, PVF, focused preparation validation, commit, and push without product work or preparation review churn",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Wait fail-closed for #5499 and #5498 live GitHub merged revisions to become ancestors of the #5502 execution base; keep typed closeout, receipts, and claim release audit-only",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement the pure convergence component and complete deterministic identity, overlap, partial-success, replan, blocked, and authority tests",
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
    "id": "S4",
    "action": "Run full proof, exact-revision pre-PR review, typed publication, serialized merge, post-merge validation, closeout, and #5501 handoff",
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

- no product work before #5499 and #5498 terminal gates
- preparation claim protects only issue-local lifecycle and evidence paths
- outputs are exact assignment-bound, canonical, and revision-continuous
- changed assumptions produce typed replan or blocked records
- no hidden task, filesystem, network, GitHub, merge, or lifecycle authority
- partial successes and blockers are never erased
- all applicable acceptance and PVF lanes complete without deferral

## Risks

- generated summaries could hide conflicting outputs or residual blockers
- stale or forged evidence could be accepted under a valid task identity
- path overlap or dependency drift could produce unsafe integration order
- replanning could silently widen scope or loop nondeterministically
- the component could regrow into a scheduler or second lifecycle store
- summary-only evidence could overclaim review, validation, or merge authority

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/issues/5502/retained/design.md

Digest: ffbac550ad54511cc08e7ed520ad15ae3e9f96e607656a7621163e364698ae09

## Diagram

.csdlc/issues/5502/retained/diagram.mmd

Digest: c325ba1c7d2cb4f6230cd3b3da135d46799189b3c2d6ca48021dea774d8cf985

## Stop Conditions

- #5499 or #5498 lacks a live GitHub merged revision ancestral to the #5502 execution base
- an intended product path overlaps an active typed claim
- output identity, authority, or ordering cannot be proven deterministically
- implementation requires hidden mutation, a state store, scheduler, network, credentials, AWS, or Runtime v2
- a budget or required acceptance/PVF proof would be deferred

## Handoff

Proceed only after doctor readiness.
