# Structured Planning Prompt

Template: 1.0.0

Issue: 5404

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Inspect the WP-12 records and validators, repair overclaims or stale assumptions, add focused regression checks, then validate the corrected proof surface.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Map each #5403 WP-12 finding to the exact record or validator surface",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Apply bounded truth/validator repairs",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused WP-12 validation and record retained evidence",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Security/CAV proof claims stay evidence-bound
- Synthetic proof artifacts cannot masquerade as operational audit evidence
- Validators fail closed when required live proof is absent

## Risks

- Review findings may require downgrading milestone truth rather than implementing new runtime paths
- Issue-state wording may drift from live GitHub state

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5404/design.md

Digest: bcf2460905e3b06e3b268b83e3f976e28e912860a418c9aaa2a345df2e20949e

## Diagram

.csdlc/prepared/issues/5404/diagram.mmd

Digest: c64dba65d2a52bbfb5b54816e68f7c90c56ca70a5dc45a7e45e4fde38e094967

## Stop Conditions

- A finding requires operator-approved scope expansion
- A required live proof cannot be produced locally
- Focused validators fail in a way that points to unrelated runtime defects

## Handoff

Proceed only after doctor readiness.
