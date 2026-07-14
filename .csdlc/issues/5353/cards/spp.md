# Structured Planning Prompt

Template: 1.0.0

Issue: 5353

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Reproduce both defects, repair atomically, add focused tests, then validate and review.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Reproduce issue-local initialization and stale diagram digest defects",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Implement typed atomic initialization and complete digest refresh",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Run focused tests, doctor, review, and closeout proof",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- no orphan issue directory is mistaken for a record
- design and diagram digests change together
- failed mutation leaves recoverable typed state

## Risks

- partial filesystem mutation
- stale projection references
- overbroad lifecycle changes

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

docs/architecture/csdlc-v2/wp01/5353/DESIGN.md

Digest: d4825727db87bfa198188d8e1f557fef4cf939978a48547f80c0e7b15ce39aee

## Diagram

docs/architecture/csdlc-v2/wp01/5353/DIAGRAM.mmd

Digest: fae5cc41fad9c10ef7885ca424e288ad22602253d3aa17e3cf51155575136933

## Stop Conditions

- typed atomicity cannot be preserved
- doctor remains stale
- scope widens beyond v2 lifecycle

## Handoff

Proceed only after doctor readiness.
