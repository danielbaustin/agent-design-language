# Structured Planning Prompt

Template: 1.0.0

Issue: 5711

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

.csdlc/issues/5711/retained/design.md

Digest: 1c7815354c658a4da1dd8f04c4f77a442f582a861d218ba44d50afeb15b93bef

## Diagram

.csdlc/issues/5711/retained/diagram.mmd

Digest: 3a88b1b465e1e516dcc6dd62bd9e840b883855d42598857e1140717e6a83e6b2

## Stop Conditions

- existing target authority
- stale recovery authority
- inconsistent closure evidence

## Handoff

Proceed only after doctor readiness.
