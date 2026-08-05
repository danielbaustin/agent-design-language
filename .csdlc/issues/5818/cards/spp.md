# Structured Planning Prompt

Template: 1.0.0

Issue: 5818

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Inventory authoritative current surfaces, update v0.92 documentation and version declarations, preserve historical evidence, run focused parity and structure proof, then obtain exact-revision review.

## Plan

Revision 12

## Steps

[
  {
    "id": "S1",
    "action": "Build the fixed-denominator canonical-surface and version-authority inventory",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Update current documentation, feature inventory, links, and version metadata without historical drift",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run activation validation, locked ADL Cargo metadata, diff hygiene, and exact-revision review",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- v0.92 active does not mean v0.92 features complete
- v0.91.8 remains the latest completed release until separate release authority changes it
- Historical evidence retains original versions and claims
- No tracked work on main and no generated-file hand editing

## Risks

- A broad replacement rewrites historical truth
- Version declarations drift across workspace members
- Current links point to absent or operator-local paths
- Cargo.lock regeneration introduces unrelated churn

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5818/design.md

Digest: 34752158fdf79f56549f51b677b35e2ad077d326cec9c9c146759fe1efe1320a

## Diagram

.csdlc/prepared/issues/5818/diagram.mmd

Digest: 031adc526b6fb3e7420d6f84f360751febe5858232cbb45d62cdb50c059ad1a9

## Stop Conditions

- Ambiguous authoritative version owner
- Generated-file ownership cannot be identified
- Historical-preservation scan reports a rewritten evidence surface
- Protected-path collision with another active issue

## Handoff

Proceed only after doctor readiness.
