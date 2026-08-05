# Structured Planning Prompt

Template: 1.0.0

Issue: 5773

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

.csdlc/issues/5773/retained/design.md

Digest: 856dd7cf8753d8ea51e63008b70e8b49c25cb01bdb4883dd4a2aedf9edb876b6

## Diagram

.csdlc/issues/5773/retained/diagram.mmd

Digest: ba2230afef21812281d54dbf5fba6c9d7c20c88c5113a5f52c19455456c1b891

## Stop Conditions

- existing target authority
- stale recovery authority
- inconsistent closure evidence

## Handoff

Proceed only after doctor readiness.
