# Structured Planning Prompt

Template: 1.0.0

Issue: 5407

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Correct each claim surface, retain complete sprint closure evidence, validate exact wording and coverage, then run independent review.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Correct build-action-log scope and typed-v2 CLI authority",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Assemble complete #5036 child and PR closeout synthesis",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Withdraw or prove the #5037 material hosted-speedup claim",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused validation and exact-revision independent review",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  }
]

## Invariants

- Gate 10D2 v1_sunset remains authoritative
- Implemented and planned behavior stay distinct
- All declared sprint children remain visible
- Local timing is not hosted performance evidence

## Risks

- Narrowing claims may leave desired future integration unimplemented
- Historical PR descriptions may overstate current authority
- Closeout synthesis can drift from live GitHub state

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5407/retained/design.md

Digest: 2d56ddd900547a66c82e264f4b88079cff5da2e98f7dbfb41d0af6a139fe6643

## Diagram

.csdlc/issues/5407/retained/diagram.mmd

Digest: d139a50f00713a5a75dedf621513cff27c1cde5f1d61fd51fa5ded760e595157

## Stop Conditions

- all four findings have explicit dispositions
- every declared child has retained closure evidence
- no active operator guidance invokes sunset v1 commands
- independent review has no unresolved actionable findings

## Handoff

Proceed only after doctor readiness.
