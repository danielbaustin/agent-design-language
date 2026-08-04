# Structured Planning Prompt

Template: 1.0.0

Issue: 5572

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

.csdlc/issues/5572/retained/design.md

Digest: 084e7597d06a8ae2df0bc401ae171efd1d3ae9ac1503ecb41373acd19fb9e9f1

## Diagram

.csdlc/issues/5572/retained/diagram.mmd

Digest: 54d5a2f7714d00c433ffeb6078c4d7ea4c31df9881740a16e52bd908cd63f148

## Stop Conditions

- existing target authority
- stale recovery authority
- inconsistent closure evidence

## Handoff

Proceed only after doctor readiness.
