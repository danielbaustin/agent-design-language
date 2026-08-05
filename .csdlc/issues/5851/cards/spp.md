# Structured Planning Prompt

Template: 1.0.0

Issue: 5851

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Freeze the WP-28A packet, independently rebuild and compare its universe/DAG, review v0.93 evidence/owners/non-claims, exercise terminal and ceremony negatives, record evidence-backed findings, and publish an exact-head disposition for WP-30.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Verify WP-28A terminal ancestry and freeze the exact packet manifest, universe, DAG, and handoff inputs.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Independently rebuild the expected universe/DAG and compare omissions, duplicates, stale identities, cycles, and owner gaps.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Review v0.93 prerequisite evidence, candidate status, owners, acceptance hooks, and governance/security/legal non-claims.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Exercise terminal, cleanup, ceremony, retry, and premature-activation negative scenarios and record findings.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve actionable findings at the current SHA and publish the exact-head result for WP-30.",
    "acceptance_ids": [
      "AC-5",
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

.csdlc/prepared/issues/5851/design.md

Digest: 826a850ac20b59d23d060dc00eaba813fe60899e89877137ede0d33d6e5367e5

## Diagram

.csdlc/prepared/issues/5851/diagram.mmd

Digest: 684da2a4d408951a88dc77fd84f07952b5e795bb91e83392370c723c1b647bee

## Stop Conditions

- Protected-path collision
- Contradictory dependency evidence
- Required proof cannot be produced within issue scope

## Handoff

Proceed only after doctor readiness.
