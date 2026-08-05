# Structured Planning Prompt

Template: 1.0.0

Issue: 5847

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify WP-25, build and redact the exact packet manifest, compute/freeze its digest, validate reviewer authority and claims, dispatch with operator approval, reject stale/missing output, and retain the received report plus routed findings index.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Verify WP-25 terminal ancestry and resolve entry blockers to a coherent external-review packet.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Build/redact the exact source manifest, normalized handoff metadata, digest, reviewer authority, questions, and return schema.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Validate and freeze the packet, obtain operator dispatch authorization, and record truthful send state.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Reject stale or missing output; retain the received reviewer report unchanged and index every finding for WP-27.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve exact-head review and publish the closing packet without release or remediation authority.",
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

.csdlc/prepared/issues/5847/design.md

Digest: 87f9be3d8bb3567165a85171d7f8322ccf7133fd3c840e4253bc390bc6a5624c

## Diagram

.csdlc/prepared/issues/5847/diagram.mmd

Digest: 3f27fc4f521fdd57a23ed1afaa29417434e08b8ac93e33e58d1a106069d94cd1

## Stop Conditions

- Protected-path collision
- Contradictory dependency evidence
- Required proof cannot be produced within issue scope

## Handoff

Proceed only after doctor readiness.
