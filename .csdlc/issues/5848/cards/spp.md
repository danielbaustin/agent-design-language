# Structured Planning Prompt

Template: 1.0.0

Issue: 5848

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Freeze the complete review-finding universe, preserve provenance while deduplicating true duplicates, assign exact owner-aligned fix slices, validate and review each fix, reconcile exact PR/merge/quality-gate truth, and block downstream planning until every actionable item is proven.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Verify WP-26 terminal ancestry and freeze the complete internal/external finding universe.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Deduplicate only identical failure modes, preserve provenance/disagreement, and assign exact owner-aligned remediation slices.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement each slice through its owner lifecycle with focused positive/negative, rollback, and required platform/security/privacy proof.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Record exact fix/review/PR/merge identity, re-run affected WP-22 rows, and reconcile release-facing claims.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve exact-head disposition review and block or release WP-28 truthfully.",
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

.csdlc/prepared/issues/5848/design.md

Digest: 7e9a76d8e112db28949f0a751b367196ec048b80e4e104c723f22fccc528bba8

## Diagram

.csdlc/prepared/issues/5848/diagram.mmd

Digest: bbd55a23ecab369dc5724109636f2fa3e89c73129f97ce62f2d8b3ee87b0df77

## Stop Conditions

- Protected-path collision
- Contradictory dependency evidence
- Required proof cannot be produced within issue scope

## Handoff

Proceed only after doctor readiness.
