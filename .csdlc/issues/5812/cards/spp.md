# Structured Planning Prompt

Template: 1.0.0

Issue: 5812

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Change only the two defaults, run focused behavior and lint proof, review the exact diff, and publish with closing linkage.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Confirm current source, lint reproduction, and exact fail-closed default tests",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Apply only the two eager-default substitutions and run focused test, format, and Clippy proof",
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
    "action": "Review the exact two-line candidate and publish with closing linkage",
    "acceptance_ids": [
      "AC-6",
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

Digest: e6684cc405f9772581c043ab4a87b8513800988539344974fa0ac4390d0bad44

## Diagram

.csdlc/prepared/issues/5812/diagram.mmd

Digest: ecc400f15bce3baa144b59854cd774554bc04d7e8747d44fecb4093ad83cbc82

## Stop Conditions

- Observed code no longer matches the issue evidence
- Focused tests expose a semantic difference
- Protected-path collision

## Handoff

Proceed only after doctor readiness.
