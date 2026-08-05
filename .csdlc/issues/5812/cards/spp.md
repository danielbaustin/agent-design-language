# Structured Planning Prompt

Template: 1.0.0

Issue: 5812

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Change only the two defaults, run focused behavior and lint proof, review the exact diff, and publish with closing linkage.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Confirm the two warnings and exact default semantics",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Apply the two-line correction and run focused proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Resolve exact-head review and publish",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Freedom Gate output is unchanged
- No unrelated source changes
- No tracked work on main

## Risks

- Accidental semantic default change
- Unrelated formatting churn
- Validation targets the wrong binary

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5812/design.md

Digest: d259e90a5aadf5b1cd805f4495d30ec0a42d39dd756f2d62b0a8a63be8b98f5d

## Diagram

.csdlc/prepared/issues/5812/diagram.mmd

Digest: ff62ba79fb6d92d3c3e3d837678c8f146cdae9b793ab34ad77402ecdc2ebbdf5

## Stop Conditions

- Observed code no longer matches the issue evidence
- Focused tests expose a semantic difference
- Protected-path collision

## Handoff

Proceed only after doctor readiness.
