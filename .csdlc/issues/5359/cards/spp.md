# Structured Planning Prompt

Template: 1.0.0

Issue: 5359

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare now; execute only after #5355 is live-merged and ancestral.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Verify typed preparation packet and #5355 live merge plus ancestry",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Review v0.92 inputs for blockers and overclaims",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Record dispositions and release-tail handoff truth",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Preserve blockers or release WP-23 without preparation-scope mutations",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
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

Digest: 5619d3b151e28b39156f65b20941bd27f585395588a923d27395d9674c0a884a

## Diagram

.csdlc/prepared/issues/5359/diagram.mmd

Digest: a6408f350363eeb048885dacec70599034f331539abc51ed41f0383aac07c794

## Stop Conditions

- #5355 not live-merged
- #5355 merge not ancestral
- review input missing
- unsupported v0.92 claim found

## Handoff

Proceed only after doctor readiness.
