# Structured Planning Prompt

Template: 1.0.0

Issue: 5873

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify gates, implement the exclusive slice, run exact proving tests and negatives, validate rollback, resolve review, and close through child authority.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5821 terminal ancestry, dependency receipts, exact paths, and source contracts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the bounded WP-04.11 outcome in the exclusive paths.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run exact positive, negative, failure, recovery, and receipt validation.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve independent review and complete child-owned publication and closeout.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Exclusive paths remain disjoint
- Guardian stays process 0
- No insecure or Runtime v2 fallback
- Queues and waits remain bounded
- Evidence is exact-revision and digest bound

## Risks

- Dependency contract drift
- Cross-child path overlap
- False-green zero-test selection
- Self-attested platform or recovery evidence

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5873/design.md

Digest: 7f42e52ce2c3834807b6f599baba3c438a88fab1f664caf7a18f2ef4f7a1f3bb

## Diagram

.csdlc/prepared/issues/5873/diagram.mmd

Digest: d12c6bcbba3c49db3c0c5366bbfccb2704148ea353385c42e49ac369e7553216

## Stop Conditions

- #5821 is not terminal
- A dependency is not terminal
- Any declared path overlaps an active claim
- The exact test target is absent or selects zero tests
- Scope or rollback authority must widen

## Handoff

Proceed only after doctor readiness.
