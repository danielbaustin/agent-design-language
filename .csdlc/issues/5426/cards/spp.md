# Structured Planning Prompt

Template: 1.0.0

Issue: 5426

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Define one reusable latest-observation rule, apply it to all terminal validation checks, add focused tests, validate, review, and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Implement shared latest-observation terminal validation semantics",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Add and run focused regression tests and formatting",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Complete independent review and lifecycle publication",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- Validation history remains append-only
- The latest observation for a logical validation is authoritative
- Any current non-passing required validation fails closed

## Risks

- An overly broad identity could collapse distinct validations
- Divergent readiness and card validation logic could reintroduce drift

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

docs/reviews/v0.91.7/csdlc-v2-5426/DESIGN.md

Digest: c4dcb826af2864338d6f39dbbd21de3c5a2dfdcaad04932990648e74fd9ad983

## Diagram

docs/reviews/v0.91.7/csdlc-v2-5426/DIAGRAM.mmd

Digest: 37f058dfe07cc4e362abf37be46fe05d9853e931c82adae046bde313cded4f7e

## Stop Conditions

- The change requires destructive history mutation
- Focused tests reveal ambiguous logical identity
- Scope expands outside C-SDLC v2

## Handoff

Proceed only after doctor readiness.
