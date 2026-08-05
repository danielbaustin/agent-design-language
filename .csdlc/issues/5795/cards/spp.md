# Structured Planning Prompt

Template: 1.0.0

Issue: 5795

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Stabilize the Runtime/Observatory boundary, implement the local provider path and truthful statuses, prove real and negative behavior, then resolve exact-head review.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Freeze the governed message, provider, status, and evidence contracts after WP-03 and TLS stabilization",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the local provider path, Observatory control, and deterministic and negative tests",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run the real local model smoke, resolve exact-head review, and publish",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Shepherd cannot bypass governed Runtime policy
- Real-model and test-double status are never conflated
- No network/cloud dependency is introduced
- No tracked work on main

## Risks

- Local model availability is mistaken for implementation success
- Runtime and Observatory contracts drift concurrently
- A fake response receives production credit
- Timeout or cancellation leaves stale work

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5795/design.md

Digest: f553a522074f53957494c36a8c33d10d3c7850b609287b2f83dd3d2b62874ec2

## Diagram

.csdlc/prepared/issues/5795/diagram.mmd

Digest: 1198da4b3642ca2b934f89155fa4ae5f5137664cab166949f925b41a84c676cc

## Stop Conditions

- WP-03 or issue 5800 control surfaces are unstable
- The local provider cannot be invoked without bypassing policy
- Real execution evidence cannot be distinguished from a test double
- Protected-path collision

## Handoff

Proceed only after doctor readiness.
