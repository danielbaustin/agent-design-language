# Structured Planning Prompt

Template: 1.0.0

Issue: 5838

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify #5832, #5834, and #5836; select two real independently configured providers; run the identical scenario and retain redacted ACIP traces; execute malformed, denied, interrupted, unavailable, loss, and substitution cases; validate source/provider truth and review exact head.

## Plan

Revision 8

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5832, #5834, and #5836 exact contracts and select two real providers",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run the identical scenario and retain redacted ACIP traces",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Execute provider failure and no-substitution negatives",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Generate and validate the matrix, artifact index, redaction, and platform posture",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve exact-head review",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Every positive column is a real provider invocation
- The scenario and ACIP operation contract are identical across providers
- One provider failure does not terminate Runtime or unrelated agents
- Retained artifacts are credential-free and redacted

## Risks

- A provider may be unavailable or capability-incompatible
- Adapters may hide provider-specific semantics
- Trace redaction may remove evidence needed for comparison

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5838/design.md

Digest: 9edd5d793d7fb009b2b655fb774846175ed5c156159bf964921ada07082264f5

## Diagram

.csdlc/prepared/issues/5838/diagram.mmd

Digest: 66b8593a65f5605cd6e903d1954da188a9dada0abb3d75d1b6b1324da92a4f29

## Stop Conditions

- Fewer than two real compatible providers are available
- #5832/#5836 contracts are not landed
- No safe redacted trace can prove semantic equivalence

## Handoff

Proceed only after doctor readiness.
