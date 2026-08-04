# Structured Planning Prompt

Template: 1.0.0

Issue: 5755

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add the smallest protocol-adapter client-auth/equivalent boundary and explicit Runtime control body limit, then validate with focused protocol/control tests and exact-head review.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Patch protocol adapter security boundary and tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Patch Runtime control body bound and oversized-body tests.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused validation, exact-head review, publish and shepherd PR.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- No plaintext credentials or credential material in tracked artifacts.
- No terminal closeout for #5664 until exact-head review confirms these blockers are resolved.
- Keep changes bounded to Runtime v3 protocol/control security.

## Risks

- mTLS changes can over-widen protocol API surface if not kept minimal.
- Body-limit tests must prove the route boundary, not only helper parsing.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/issues/5755/retained/design.md

Digest: a8c1bd11379408c7a249903471993aedd5bb9258c02ffba12233a5fc40e0bdea

## Diagram

.csdlc/issues/5755/retained/diagram.mmd

Digest: 22247405f2acf729e51863c830ec650eb2f9112f300673ee2cc84fab06c2cc14

## Stop Conditions

- Stop if the repair requires AWS or external infrastructure.
- Stop if typed lifecycle cannot bind #5755 without hand editing generated state.
- Stop if exact-head review finds an unresolved P1/P2 security blocker.

## Handoff

Proceed only after doctor readiness.
