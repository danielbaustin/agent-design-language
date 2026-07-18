# Structured Planning Prompt

Template: 1.0.0

Issue: 5412

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Authenticate both state paths first, add focused forgery/lineage tests, route the real soak through a bounded release lane, then close the LoC finding with a reproducible disposition.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Implement signed checkpoint canonicalization and verification",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Require verified accepted-lineage membership for private projection",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add and prove the bounded real-soak release lane",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Recount source and retain reviewed LoC disposition/reduction plan",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- No unauthenticated state enters restore or projection
- Lineage membership is proven by the owning lineage object
- Ordinary PR validation remains bounded
- No default-cutover claim is introduced

## Risks

- Checkpoint schema evolution can break retained fixtures
- Projection API changes can require caller migration
- The real soak may expose latent timing failures
- LoC reduction can tempt unrelated refactoring

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5412/design.md

Digest: 4e517a54c8639a8255091cc924403fa5b66721b42878507b50c1a03d12ddcc27

## Diagram

.csdlc/prepared/issues/5412/diagram.mmd

Digest: c760b13c5f9ce35098e218cbd2c7d26a5537aaa23d8339391b5b31d4224a8cf6

## Stop Conditions

- Compatibility requires trusting unsigned checkpoints
- Lineage membership cannot be proven without widening authority
- Soak lane would become a mandatory ordinary-PR cost
- LoC work requires unrelated product redesign

## Handoff

Proceed only after doctor readiness.
