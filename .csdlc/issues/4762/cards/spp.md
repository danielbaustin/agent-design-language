# Structured Planning Prompt

Template: 1.0.0

Issue: 4762

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Execute #4762 by delivering the retained auditable birth-witness register, receipt, negative-case dispositions, validation evidence, exact-head review evidence, and handoff consumption links without claiming the v0.92 birthday occurred.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Create retained schema, negative-case, execution-design, and validator artifacts for the #4762 birth-witness/receipt package.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Create reviewer-facing witness register, receipt, README, and summary package under the v0.91.8 handoff review path.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Wire exact handoff consumption references into v0.91.8 activation/handoff and v0.92 launch packet surfaces.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused validation and retain issue-local evidence.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Obtain exact-head review, fix findings, record review truth, commit, push, and publish a ready PR with Closes #4762.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- The package is handoff evidence, not a birthday occurrence.
- Missing identity, continuity, memory, capability, witness, receipt, activation, validation, or reviewer evidence fails closed for any future birth claim.
- Lifecycle truth must distinguish local validation, exact-head review, PR publication, merge, and closeout.

## Risks

- The historical branch slug says WP14 while current issue routing is WP-21; artifacts keep #4762 and WP-21 handoff truth explicit.
- Future v0.92 consumers may overread the package as birthday completion; artifacts preserve birth_event_status: not_claimed.

## Estimates

{
  "elapsed_seconds": 14400,
  "total_tokens": 80000,
  "validation_seconds": 1800
}

## Design

.csdlc/prepared/issues/4762/design.md

Digest: 44ad81e9b236bbe02d0e44176741ebb41e51d59287aebafd177d8ea474b8b15d

## Diagram

.csdlc/prepared/issues/4762/diagram.mmd

Digest: 86d6ec17a88e161d272c9d7580af9c207ee974d652e335b23941b6605a31ac85

## Stop Conditions

- Stop if any artifact claims the v0.92 birthday occurred.
- Stop if focused validation fails.
- Stop if exact-head review reports unresolved actionable findings.
- Stop before merge or post-merge closeout.

## Handoff

Proceed only after doctor readiness.
