# Structured Planning Prompt

Template: 1.0.0

Issue: 5840

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify dependency revisions; correct stale WP-20 ownership while preserving WP-21/WP-21A; replace candidate rows only with accepted exact-revision evidence; build the AEE artifact index; add fail-closed coverage validation; resolve exact-head review.

## Plan

Revision 8

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5836, #5837, #5838, and #5839 at accepted exact revisions",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Reconcile matrix, coverage, activation, and AEE index rows",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Correct WP-20 ownership while preserving WP-21/WP-21A",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement and run positive, negative, and platform coverage checks",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve exact-head proof-index review",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Every accepted row names an exact revision, command, artifact, owner, and review state
- Positive evidence never substitutes for a required negative lane
- Platform-specific evidence is not generalized silently
- AEE uncertainty is recorded as a blocker, not relabeled as proof

## Risks

- Dependency proof may be incomplete or superseded
- Stale ownership prose may conflict with the live wave
- Coverage rows may overstate platform or AEE support

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5840/design.md

Digest: 9431c5b9ecbb320ef0b0e61b86d1b686c9ab7f5e9d0093c7a63126478c3bcf40

## Diagram

.csdlc/prepared/issues/5840/diagram.mmd

Digest: 57f3fcc1a7b7aac2e97794d4e761093a49874adc02355f69436b4811bb6538e2

## Stop Conditions

- A required dependency lacks accepted exact-revision evidence
- WP-20 ownership cannot be reconciled without changing the live wave
- A claimed artifact cannot be independently inspected

## Handoff

Proceed only after doctor readiness.
