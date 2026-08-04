# Structured Planning Prompt

Template: 1.0.0

Issue: 5363

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare now; execute only after #5357 is live-merged and ancestral.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Refresh live issue #5363, current origin/main, and v0.91.8 release-tail sequence before execution.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Verify WP-19 #5357 live merge state and prove the observed merge SHA is ancestral to the exact #5363 execution base.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Verify completed child evidence for #5548 and #5558 against current origin/main ancestry instead of treating either child as open implementation.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Inventory accepted internal and external review findings from WP-18/WP-19 and classify each as accepted fix, routed non-goal, blocker, or unsupported claim.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Apply only accepted remediation scope and run focused validation for each fix class.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Run integrated release preflight, record exact blockers or pass evidence, and obtain exact-revision review before any PR or WP-21 handoff.",
    "acceptance_ids": [
      "AC-6"
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

Digest: 3e6bc78728cc429f73aa595e368db31eebde14b7ebe493740f348777d5932248

## Diagram

.csdlc/prepared/issues/5363/diagram.mmd

Digest: b326f87f19713b9a93c895460d9eaae69f1e46971f92f5c10e17fce34cb488c5

## Stop Conditions

- #5357 not live-merged
- #5357 merge not ancestral
- accepted finding scope unclear
- preflight would require unrelated product work

## Handoff

Proceed only after doctor readiness.
