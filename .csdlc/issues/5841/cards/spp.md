# Structured Planning Prompt

Template: 1.0.0

Issue: 5841

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

After WP-21 lands, rebuild the active Rust hotspot inventory, select exact files and invariants, capture characterization proof, refactor one owner at a time, and prove behavior, negative cases, LoC, lint, tests, and platform parity.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Verify WP-20/WP-21 terminal ancestry and rank post-deletion Rust hotspots from current source evidence.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Select the smallest exact-file set, owners, behavior invariants, metrics, tests, and rollback boundary.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Capture characterization proof and refactor one ownership boundary without public behavior change.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused parity/negative tests, touched-workspace tests, strict Clippy, formatting, metrics, and platform CI.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve exact-head review and publish the bounded closing PR with retained inventory and residual hotspots.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- No tracked work on main
- No scope absorption across work packages
- Evidence claims remain exact-revision and source-grounded

## Risks

- Dependency drift
- Scope overlap
- Insufficient real-behavior proof

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5841/design.md

Digest: 496006b4eec4a88d4cdfaedcc42b6ef74950990044da86858c82e0d90b680e22

## Diagram

.csdlc/prepared/issues/5841/diagram.mmd

Digest: b4df00a4d3d743cffd8848bba7bb5c20d5b4d4641915587f6a72b37044e3c611

## Stop Conditions

- Protected-path collision
- Contradictory dependency evidence
- Required proof cannot be produced within issue scope

## Handoff

Proceed only after doctor readiness.
