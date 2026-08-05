# Structured Planning Prompt

Template: 1.0.0

Issue: 5679

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

.csdlc/issues/5679/retained/design.md

Digest: 59e6f6569f9ce110ceff9303a7589565202a241551da53b134e6c31ddc3f3584

## Diagram

.csdlc/issues/5679/retained/diagram.mmd

Digest: d15c86a393da35543dc572cf0e5d9e23edcfe93e2dc718968d01c6353977ccb9

## Stop Conditions

- existing target authority
- stale recovery authority
- inconsistent closure evidence

## Handoff

Proceed only after doctor readiness.
