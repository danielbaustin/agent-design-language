# Structured Planning Prompt

Template: 1.0.0

Issue: 5718

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Validate exact closure evidence and atomically retain terminal authority.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Retain terminal recovery evidence",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  }
]

## Invariants

- target projection and receipt are absent before recovery
- No historical implementation, review, publication, readiness, or CI lifecycle is reconstructed by this terminal recovery.

## Risks

- remote evidence supplied to this deterministic operation must come from the typed GitHub observation surface

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5718/retained/design.md

Digest: 37d1dd915e6f55633c25837c1d659024cf74147f295fb9a73fd6d7667562912f

## Diagram

.csdlc/issues/5718/retained/diagram.mmd

Digest: fc3b9bcf52683579264882da57a1459d2fe93b2898c5e5e1e025c882e15d0725

## Stop Conditions

- existing target authority
- stale recovery authority
- inconsistent closure evidence

## Handoff

Proceed only after doctor readiness.
