# Structured Planning Prompt

Template: 1.0.0

Issue: 5843

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Consume the passing WP-22 matrix, inventory canonical docs and claims, classify stale/planned/blocked/unsupported truth, make narrow evidence-linked corrections, validate links/formats/commands/non-claims, and complete exact-head docs review.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Verify the passing WP-22 matrix is terminal and ancestral; pin the docs-review evidence base.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Inventory canonical root, milestone, feature, ADR, release, skill, and agent-guidance claims and ownership.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Classify current/stale/planned/blocked/unsupported/historical statements and apply narrow evidence-linked corrections.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Build docs-review and justified ADR-candidate packets; validate formats, links, commands, versions, ownership, evidence, and non-claims.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve exact-head docs review and publish the closing PR without review or ceremony authority.",
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
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5843/design.md

Digest: df65cd33fb590ad53114ea67e65143866b543df1051bdc091438abad53636205

## Diagram

.csdlc/prepared/issues/5843/diagram.mmd

Digest: e9443389dc374e5a02a2f30234e94766f6f7c6cebf18268f6671e69c5bc97795

## Stop Conditions

- Protected-path collision
- Contradictory dependency evidence
- Required proof cannot be produced within issue scope

## Handoff

Proceed only after doctor readiness.
