# Structured Planning Prompt

Template: 1.0.0

Issue: 5849

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Consume terminal remediation, inventory the v0.93 candidate corpus, map every prerequisite to exact evidence/blocker/owner/non-claim, reconcile decision-ready candidate planning, validate dependencies and claim boundaries, and hand the reviewed packet to WP-28A.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Verify WP-27 terminal ancestry and freeze final v0.92 quality/review/remediation inputs.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Inventory the complete v0.93 candidate corpus, dependencies, decisions, stale assumptions, owners, and evidence hooks.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Map every prerequisite to exact evidence, blocker, follow-on, or non-claim and reconcile candidate documents.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Validate formats, links, dependencies, owners, acceptance hooks, redaction, and activation/legal/certification negative boundaries.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve exact-head docs review and hand the packet to WP-28A.",
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

.csdlc/prepared/issues/5849/design.md

Digest: e2a54eff73538d18c5b3267dac4ba7b03f92189d08d369c6c5072b89807b47b2

## Diagram

.csdlc/prepared/issues/5849/diagram.mmd

Digest: 71c4234b11ef676b8124bc3a101e7670f42ee734598fb52da48405843f96b070

## Stop Conditions

- Protected-path collision
- Contradictory dependency evidence
- Required proof cannot be produced within issue scope

## Handoff

Proceed only after doctor readiness.
