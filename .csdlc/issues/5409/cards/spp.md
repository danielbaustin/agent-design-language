# Structured Planning Prompt

Template: 1.0.0

Issue: 5409

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Map the #5121 topology boundary, integrate production supervision/readiness and proactive credential renewal, then retain assembled-runtime soak proof.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Map the #5121 topology, readiness, credential, and soak surfaces",
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
    "action": "Implement runtime-owned topology, readiness, and credential renewal changes",
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
    "action": "Run focused runtime/proof validation and record retained evidence",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Production assembly and reported supervised topology agree
- Readiness observes every required component/channel
- Credential renewal preserves overlap before expiry
- Soak proof is retained and reproducible

## Risks

- API Gateway live resources may require a bounded downgrade if the full matrix cannot be exercised
- The final gate may remain blocked until an explicit operator disposition exists

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/issues/5409/retained/design.md

Digest: b334a90e062a7a63b62f49592afe43dd01031ba51be1fda9902814822ea252a3

## Diagram

.csdlc/issues/5409/retained/diagram.mmd

Digest: efb71e84624a77422732103e8e9bd91cf760c4ced239ccbdf3d11db31f75766f

## Stop Conditions

- A finding requires new AWS infrastructure or operator-approved scope expansion
- A required live proof cannot be produced or truthfully downgraded
- Focused validation exposes an unrelated runtime defect

## Handoff

Proceed only after doctor readiness.
