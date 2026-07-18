# Structured Planning Prompt

Template: 1.0.0

Issue: 5427

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add the typed identity operation, test validation and atomicity, then use it to repair #5353 and validate the resulting cards.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Map identity projection and mutation boundaries",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement typed operation and focused regressions",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Repair #5353 and run v2 validation",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- All six cards and canonical issue identity share one version
- CAS protects the mutation
- Failed validation leaves prior state intact

## Risks

- Existing identity schema may require a compatibility-preserving operation shape
- #5353 may have unrelated stale card content

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5427/design.md

Digest: 466ae2bb60432db07310e17f7ee398b820574150f97cb39fecf67618d9d78f22

## Diagram

.csdlc/prepared/issues/5427/diagram.mmd

Digest: 58fdfa7edeec79f2bb33de2b26a8aed64354afcf090b644eb45c884c842cae66

## Stop Conditions

- Repair requires changing non-identity content
- Atomicity cannot be proven with focused tests
- #5353 state is unavailable or conflicts with the issue scope

## Handoff

Proceed only after doctor readiness.
