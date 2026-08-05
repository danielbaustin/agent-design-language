# Structured Planning Prompt

Template: 1.0.0

Issue: 5655

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add one Rust command surface, prove exact mutation reconciliation, then publish only exact reviewed green work.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Implement typed issue action request/response boundary",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Add focused reconciliation tests and operator contract",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Review, publish, merge, and closeout",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- one Rust command surface
- existing token resolver is the only secret source
- remote mutation is followed by exact readback
- ambiguous results never produce unverified lifecycle truth

## Risks

- remote mutation may succeed before transport failure
- duplicate comments or labels without durable markers
- issue identity drift between remote readback and local commit

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5655/design.md

Digest: c647a5b1052d3180ec06f8bc031da15fe96938d178475b092054fcbd7e259139

## Diagram

.csdlc/prepared/issues/5655/diagram.mmd

Digest: 48408b9fa70ce197e2b06e72c327e0aac90b7bf6222df0aeac9badfe3147abfa

## Stop Conditions

- connector, wrapper, raw gh, shell, Python, AWS, or Runtime is required
- remote result cannot be reconciled exactly
- protected-path collision or stale claim
- actionable review finding remains open

## Handoff

Proceed only after doctor readiness.
