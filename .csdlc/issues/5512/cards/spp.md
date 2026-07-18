# Structured Planning Prompt

Template: 1.0.0

Issue: 5512

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Detect the already bounded bridge expression, substitute owning-crate filters, and lock the exact failure into the coverage contract test.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Add the bounded owning-crate expression split",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Add the exact CI failure regression and run focused validation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Review, publish, merge, and resume #5494 CI",
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

- Only the already bounded Runtime v3/CSM bridge is specialized
- Each crate receives only selectors it owns
- Both summaries are composed
- Runtime v2 source remains untouched

## Risks

- A broad detector could alter unrelated coverage routes
- The ADL filter could omit intended CSM coverage
- Summary composition could silently lose one crate

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5512/retained/design.md

Digest: 389e313a0101bd380a9294b090884d97ad96e2933198b03831d3cdac1f5ced55

## Diagram

.csdlc/issues/5512/retained/diagram.mmd

Digest: f486496a9d419d941b64de7636f1b52946a1c1b0dcb8fdd7a2445ada3b595026

## Stop Conditions

- The repair requires Runtime v2 changes
- The bridge cannot be identified from its closed selector family
- Validation requires AWS

## Handoff

Proceed only after doctor readiness.
