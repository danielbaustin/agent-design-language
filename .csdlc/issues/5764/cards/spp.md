# Structured Planning Prompt

Template: 1.0.0

Issue: 5764

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add or document the canonical readiness probe, make weather freshness explicit in readiness/observatory truth, update README watcher instructions, and prove with focused Runtime v3 route tests.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect current Runtime v3 route and Observatory feed behavior",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement the minimal readiness/weather truth repair",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Update operator/Observatory docs and watcher recipe",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused validation, review, publish, and shepherd",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Runtime v3 read surfaces remain public only where already public
- Control mutation remains signed-command-only
- Weather degradation must not be hidden behind a green top-line claim
- Overnight monitoring must not depend on AWS

## Risks

- Adding /v1/ready could duplicate health semantics unless the response is intentionally bounded
- Treating stale weather as fatal could make readiness noisy if the sampler intentionally degrades

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5764/retained/design.md

Digest: 44ed87fe9c967bce4ade59f66480c6a5724f2ef40f1900628446b128a5ef4584

## Diagram

.csdlc/issues/5764/retained/diagram.mmd

Digest: 4f710f360f216d7d02e888008dd5d79408d964cb09a269d9b90fea29b5d28202

## Stop Conditions

- Protected path collision
- Route tests fail unexpectedly outside touched surfaces
- Fix requires runtime/security model expansion

## Handoff

Proceed only after doctor readiness.
