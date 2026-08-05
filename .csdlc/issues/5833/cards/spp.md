# Structured Planning Prompt

Template: 1.0.0

Issue: 5833

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement and prove WP-15 exact-candidate witness consensus and deterministic redacted receipts with anti-equivocation, authority, and premature-claim protections.

## Plan

Revision 18

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5826 through #5830 and #4762, then inspect exact private-state witness and birthday candidate contracts.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement witness-set validation, exact candidate binding, accepted/rejected receipt derivation, caveats, redaction, and fixtures.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run canonical witness/receipt tests plus equivocation, duplicate, stale, forged, authority, privacy, and premature-claim negatives.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve one bounded exact-head review and publish only with correct base and Closes #5833 linkage.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- Witnesses bind the exact candidate and evidence digests and are distinct where policy requires.
- Receipts derive only from validated decisions and preserve caveats and rejection reasons.
- Raw private state never enters reviewer or citizen-facing projections.

## Risks

- Duplicate or equivocal witnesses could appear as consensus.
- A stale or mismatched candidate digest could receive a valid-looking receipt.
- Receipt prose could leak private evidence or claim birth prematurely.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5833/design.md

Digest: 03a5c8906927d3af970da1061ec00f53abbd3d56275f83f8809de2d184268bde

## Diagram

.csdlc/prepared/issues/5833/diagram.mmd

Digest: 0255242ebe93f2857c2a9dc5f2d0c1a083fa373b340935cbfe34749941f0d599

## Stop Conditions

- Any sprint-gate dependency or #4762 is not verifiable.
- The candidate digest cannot be bound across all witnesses.
- Privacy, equivocation, authority, or premature-claim negatives cannot fail closed.

## Handoff

Proceed only after doctor readiness.
