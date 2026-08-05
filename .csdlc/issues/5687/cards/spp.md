# Structured Planning Prompt

Template: 1.0.0

Issue: 5687

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

.csdlc/issues/5687/retained/design.md

Digest: acf242ca7b24b7e01ac8a4a83bf8f32b6338b4d8b2282d81f6b141c9169fc710

## Diagram

.csdlc/issues/5687/retained/diagram.mmd

Digest: de2627101e9f957410bd62c164f8e6781933453e4055198f6d9b023d1d9c4e56

## Stop Conditions

- existing target authority
- stale recovery authority
- inconsistent closure evidence

## Handoff

Proceed only after doctor readiness.
