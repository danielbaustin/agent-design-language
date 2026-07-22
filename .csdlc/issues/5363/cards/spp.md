# Structured Planning Prompt

Template: 1.0.0

Issue: 5363

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare now; execute only after #5357 is live-merged and ancestral.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Verify typed preparation packet and #5357 live merge plus ancestry",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Inventory accepted findings and fix only accepted scope",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused and integrated preflight proof",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Record exact blockers or release WP-21 without preparation-scope mutations",
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

- review findings may be stale
- preflight may expose separate owner work
- unsupported release claims could be hidden in checklist prose

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5363/design.md

Digest: dccf65698eb9db306de735bdb37c53c6a1a6fe7282a5127476a99cc6a78c0db0

## Diagram

.csdlc/prepared/issues/5363/diagram.mmd

Digest: 55f29bf52d9d8e66b22df7f27bc3958d08e6c19bd7a88b6da409a852e1e58290

## Stop Conditions

- #5357 not live-merged
- #5357 merge not ancestral
- accepted finding scope unclear
- preflight would require unrelated product work

## Handoff

Proceed only after doctor readiness.
