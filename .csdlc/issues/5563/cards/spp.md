# Structured Planning Prompt

Template: 1.0.0

Issue: 5563

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Detect stale authored digests during initialized approved state, permit exact typed reapproval, and prove normal readiness resumes.

## Plan

Revision 1

## Steps

[
  {
    "id": "implement",
    "action": "Implement initialized stale-design reapproval and focused regression",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  }
]

## Invariants

- No hand-edited cards
- No readiness bypass
- No redundant initialized reapproval
- Atomic six-card state

## Risks

- Over-broad reapproval could hide unrelated card corruption.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5563/design.md

Digest: eeed304cf7c34edc67aa6b88bbecf20b2b656a593408eae8ce9b9281b768a62a

## Diagram

.csdlc/prepared/issues/5563/diagram.mmd

Digest: 66c20481a67365c9df61647104a24804abb263f9efdceafe6712d371732ead62

## Stop Conditions

- Any need to hand-edit rendered cards
- Any readiness bypass

## Handoff

Proceed only after doctor readiness.
