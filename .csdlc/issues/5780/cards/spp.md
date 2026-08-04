# Structured Planning Prompt

Template: 1.0.0

Issue: 5780

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Characterize the legacy authority, delete its operator surface and writers, preserve only read compatibility, replace obsolete tests with negative and parity guards, validate, review, publish, and finish #5780.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Initialize and bind #5780 with the approved deletion design",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Delete the legacy terminal authority and operator surface",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Prove legacy read parity and current lifecycle integrity",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Complete exact-head review, publish, shepherd, and finish #5780",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Exact-head finish semantics remain unchanged
- Cleanup remains independent of terminal truth
- Legacy terminal data remains readable but never writable
- Historical records and evidence remain immutable

## Risks

- Receipt writer and reader code may be interleaved
- Store deletion may accidentally affect current non-terminal persistence
- Public schemas or manifests may retain hidden mutation authority
- Obsolete tests may mask missing negative guards

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5780/design.md

Digest: a97805f1a8bcb6d698c1bf1184efc8a11a503e56c4c89a2cdd90ff8340a2a533

## Diagram

.csdlc/prepared/issues/5780/diagram.mmd

Digest: 61b9c28713bc386b6396146ff636e91f5f6f815d19e8b19f0bd8047c456f0fee

## Stop Conditions

- Any claim collision on declared paths
- Any required historical record rewrite
- Any legacy compatibility outcome drift
- Any actionable exact-head review finding

## Handoff

Proceed only after doctor readiness.
