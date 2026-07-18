# Structured Planning Prompt

Template: 1.0.0

Issue: 5440

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Extend the existing approve-design phase guard, retain its atomic digest refresh, and prove allowed and rejected phases.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Extend approve-design authorization to bound and implemented",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Add deterministic allowed and rejected phase tests",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Run focused validation and independent review",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Exact-revision review authority is never retained after an unreviewed design change
- Card projections remain renderer-generated and digest-consistent
- Transitions are append-only

## Risks

- Accidentally allowing stale reviewed work
- Failing to refresh the diagram digest
- Breaking initialized approval

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5440/retained/design.md

Digest: e55e6c28f27d759dc82b6649b21118dd3a72959ab62e310dadf468443212e9a3

## Diagram

.csdlc/issues/5440/retained/diagram.mmd

Digest: 382de225279586ed470844adb28d61d81d7cf22c5762555a18d0f6290adc4c8d

## Stop Conditions

- Any solution requires manual card mutation
- Later lifecycle phases would silently retain review truth
- Scope expands beyond csdlc-v2

## Handoff

Proceed only after doctor readiness.
