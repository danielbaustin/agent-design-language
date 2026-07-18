# Structured Planning Prompt

Template: 1.0.0

Issue: 5514

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Retain exact-expression routing, expand the ADL partition to the complete valid CSM family, lock the command contract, and rerun #5504 CI.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Expand the exact bridge ADL partition to every valid owning selector",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Strengthen the command-level regression contract and validate locally",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Review, merge, rebase #5504, and prove the changed-source gate",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Only the exact canonical mixed expression is specialized
- Each workspace receives only tests it owns
- Coverage thresholds are unchanged
- Runtime v2 and AWS remain outside scope

## Risks

- A retained selector could still name the wrong binary
- A weak test could verify only parse success rather than suite completeness
- A broad detector could alter unrelated expressions

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5514/design.md

Digest: 5bffbfb561d33dc342a1030ca018ec053fe51d3195ecdb8c4f419ab0ccf10c11

## Diagram

.csdlc/prepared/issues/5514/diagram.mmd

Digest: b620e1d89dd7dffe09577589bdd6b73c7bcfb69e7c4fd1a0a698ad4e86643e8d

## Stop Conditions

- The repair requires production runtime changes
- The repair requires Runtime v2 changes
- Validation requires AWS

## Handoff

Proceed only after doctor readiness.
