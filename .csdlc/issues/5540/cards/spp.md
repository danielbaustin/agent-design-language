# Structured Planning Prompt

Template: 1.0.0

Issue: 5540

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

.csdlc/issues/5540/retained/design.md

Digest: f121448bb8d7f344168e9ead6fda83f1519f170ae14221b56197d9978518d880

## Diagram

.csdlc/issues/5540/retained/diagram.mmd

Digest: cb2ed7fd1698e30dd1ec39e7ae98edff57a6cc2e1ce7ca3c5893003c068d0c84

## Stop Conditions

- existing target authority
- stale recovery authority
- inconsistent closure evidence

## Handoff

Proceed only after doctor readiness.
