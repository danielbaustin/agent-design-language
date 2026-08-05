# Structured Planning Prompt

Template: 1.0.0

Issue: 5801

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Review the current topology, centralize deterministic path/PVF classification, deduplicate heavy and coverage lanes, preserve exact-head source safety, bound metadata-only reuse, prove stale-run behavior, and validate platform contracts.

## Plan

Revision 10

## Steps

[
  {
    "id": "S1",
    "action": "Retain and cryptographically validate Gemini 3.1 Pro review artifacts and dispose every actionable finding",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement focused/slow routing, coverage deduplication, metadata-lineage reuse, and cancellation safety",
    "acceptance_ids": [
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
    "action": "Run review-packet, policy, coverage, lifecycle, platform, and exact-head proof",
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

- Unknown or mixed source changes select the stronger proving lane
- Source/product exact-head review and merge evidence is never reused across substantive drift
- Each PR class runs one authoritative coverage topology
- Required-check names and branch protection remain stable
- Machine-readable outputs and adl_event stderr separation remain intact

## Risks

- A docs or metadata classifier accidentally suppresses source proof
- Coverage shards duplicate work or aggregate different revisions
- Cancellation hides a missing final required state
- Metadata lineage accepts substantive or renamed source drift
- Platform-specific path handling diverges

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5801/design.md

Digest: b182f8d0bfc34dd38efb21e7b225e28961504bf532fd1fa436fafe7930b3a46a

## Diagram

.csdlc/prepared/issues/5801/diagram.mmd

Digest: 38d2b37695ebf0a973ad18715373b5388f65f5eb6d61f68fafb821966de2ecb2

## Stop Conditions

- WP-02 destination CI state is not stable
- A proposed route weakens required source proof
- Required-check identity or branch protection must change
- Unknown path classification would default to less proof
- Coverage artifacts cannot prove one exact revision and authority

## Handoff

Proceed only after doctor readiness.
