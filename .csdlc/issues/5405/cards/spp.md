# Structured Planning Prompt

Template: 1.0.0

Issue: 5405

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Inspect WP-13 claim records, correct overclaims, add duplicate policy-row rejection to the owning Runtime v2 economics boundary, then run focused Rust and document validation.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Map #5403 WP-13 findings to claim records and the owning Runtime v2 economics validator",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Repair docs/records and duplicate semantic-policy validation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run focused regression proof and update retained evidence",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  }
]

## Invariants

- WP-13 claim status remains evidence-bound
- Admission readiness is not equivalent to live provider invocation
- Duplicate semantic policy entries fail validation

## Risks

- Closeout records may need multiple claim-status updates
- Economics boundary duplicate rejection must preserve valid canonical packets

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/issues/5405/retained/design.md

Digest: 94d15fb356b31f02b0d46cc67eb69d558668f1783d2090be1b75914e4866ed68

## Diagram

.csdlc/issues/5405/retained/diagram.mmd

Digest: e6532f69e14e124f0eff40d721bb0a891acdfa8f2a26925fb4ae643f9ecbefd9

## Stop Conditions

- Real guild integration is required but outside the approved issue scope
- Economics duplicate rejection breaks existing valid boundary packets
- Parent closeout truth needs operator decision rather than mechanical repair

## Handoff

Proceed only after doctor readiness.
