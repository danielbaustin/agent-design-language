# Structured Planning Prompt

Template: 1.0.0

Issue: 5355

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare now; execute only after #5362 is live-merged and ancestral.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Refresh #5362 live issue/PR state, fetch origin, and block unless the observed #5362 merge commit is ancestral to refreshed origin/main and the exact #5355 execution base.",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Read accepted WP-21 outputs and canonical v0.91.8 planning inputs, then author only the WP-21A closeout-planning packet and review-ready handoff.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused docs, YAML, link/crosswalk, typed C-SDLC, and diff hygiene checks; record exact commands and evidence without claiming deferred proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Obtain exact-head bounded review before any publication and hand WP-22 forward only when blockers are absent or explicitly preserved.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- live merge plus ancestry is the dependency gate
- receipts audit-only
- no preparation review churn
- no implementation in preparation

## Risks

- historical preparation evidence could be mistaken for current truth
- canonical document inventory may be incomplete
- handoff language may overclaim v0.92 readiness

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5355/design.md

Digest: e4dd6dbc4b549584aace09c823c15e65144e547ee7b0ebf42f25cecf4ff1efae

## Diagram

.csdlc/prepared/issues/5355/diagram.mmd

Digest: aa86d8f320c6149366895acd1a90371502b605da677d6887a2e8c85cd859d0dd

## Stop Conditions

- #5362 not live-merged
- #5362 merge not ancestral
- canonical document missing
- handoff would overclaim

## Handoff

Proceed only after doctor readiness.
