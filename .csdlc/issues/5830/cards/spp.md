# Structured Planning Prompt

Template: 1.0.0

Issue: 5830

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement and prove WP-13 canonical evidence maps with revision-linked updates, bounded projections, privacy, and explicit non-reputation/personhood boundaries.

## Plan

Revision 15

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5827/#5828/#5829 and current Runtime v3 cognition/governance inputs, then claim the exact cognitive_profile.rs, lib.rs, tests, fixture, feature, and evidence paths.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the versioned profile, allowed evidence categories, revision-linked updates, privacy policy, projections, and fixtures.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run canonical update replay, stale/forbidden/root-mismatch negatives, privacy projection, and non-reputation claim lanes.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve one bounded exact-head review and publish only with correct base and Closes #5830 linkage.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Every profile field cites an allowed current evidence category and digest.
- Every update links the prior revision and explains additions and removals.
- Public projection is strictly narrower than the internal redacted evidence map.

## Risks

- Free-form labels could escape evidence constraints.
- Profile updates could drop provenance or revision linkage.
- Public projection could leak private evidence or imply reputation and rights.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5830/design.md

Digest: fb2baa8e667f94ac4a4d7a45cf2f963eca132d757264de1765793548618391c2

## Diagram

.csdlc/prepared/issues/5830/diagram.mmd

Digest: 6b9a9b2ae6cf9491279ce9ea531331cc7b98069e510d08bd2aad9422595fef47

## Stop Conditions

- Any declared dependency or evidence category is not current and verifiable.
- A requested field cannot cite an allowed evidence source.
- Privacy or non-reputation negative proof cannot fail closed.

## Handoff

Proceed only after doctor readiness.
