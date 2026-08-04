# Structured Planning Prompt

Template: 1.0.0

Issue: 5359

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare now; execute only after #5355 is live-merged and ancestral.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Recheck live #5359/#5355/#5362 state, origin/main, and the v0.91.8 release-tail sequence before execution; stop if #5355 is not merged and ancestral.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Inventory the exact v0.92 handoff and activation inputs, including the WP-21 and WP-21A outputs after they close.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Produce the WP-22 planning review packet with blocker, stale-assumption, overclaim, non-claim, and WP-23 disposition sections.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused preparation and future execution validation lanes without treating deferred predecessor proof as passed.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Preserve the forbidden-surface boundary and stop at pushed preparation branch with no PR, publication, execution, or closeout.",
    "acceptance_ids": [
      "AC-5",
      "AC-7"
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

- review may approve stale assumptions
- v0.92 opening may be implied without evidence
- closeout planning gaps may be hidden

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5359/design.md

Digest: 7c5fc27776f34c353580c9d3682023c3ccc8ad5869731f5c171c07048cbdc85a

## Diagram

.csdlc/prepared/issues/5359/diagram.mmd

Digest: b5f44e22483066f54e4178775ba56b6b2db756a89d50631ff58c85c41bf1bed4

## Stop Conditions

- #5355 not live-merged
- #5355 merge not ancestral
- review input missing
- unsupported v0.92 claim found

## Handoff

Proceed only after doctor readiness.
