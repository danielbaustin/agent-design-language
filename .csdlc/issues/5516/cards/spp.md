# Structured Planning Prompt

Template: 1.0.0

Issue: 5516

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Initialize the narrow authority, run typed terminal design repair, validate digests and lifecycle truth, review, publish, merge, and close out.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Prepare corrected design and diagram sources",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Apply typed terminal design repair and validate",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Review, publish, merge, and close out",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- Terminal repair uses typed C-SDLC v2 operations
- Runtime source remains byte-identical to origin/main
- Runtime v3 weather ownership remains explicit

## Risks

- Manual edits could break terminal receipt and card digest parity
- Historical planning prose could be mistaken for current architecture truth

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5516/retained/design.md

Digest: d35dc188153a0cfb654d3bc99e5fef6e3672974f725e60d4f8dbd189f4e9a13d

## Diagram

.csdlc/issues/5516/retained/diagram.mmd

Digest: d2edc9d5ff9516383e01aa249b3b10fca568be5db3a38f210f8b2681960b16b0

## Stop Conditions

- Repair requires runtime source changes
- Typed terminal receipt validation fails
- Exact review finds unresolved architecture contradiction

## Handoff

Proceed only after doctor readiness.
