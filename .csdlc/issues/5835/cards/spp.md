# Structured Planning Prompt

Template: 1.0.0

Issue: 5835

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify #5826, #5827, and #5834; reconcile landed schemas; author the transfer matrix and WP-04 boundary; validate links, redaction, copied-state rejection, and forbidden production claims; then obtain exact-head review.

## Plan

Revision 9

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5826, #5827, and #5834 exact dependency truth and landed schemas",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Author the transfer matrix and continuity-transfer design note",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Update only concrete v0.93 handoff inputs",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run path, redaction, copied-state, ambiguity, and forbidden-claim validation",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve exact-head review and retain proof",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Copied state is never continuity proof
- Ambiguous lineage is quarantined
- Private state moves only as governed redacted references
- Production and governance authority remain downstream

## Risks

- Migration prose could overclaim operational portability
- A transfer row could expose private state
- Landed dependency schemas may differ from planning names

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5835/design.md

Digest: 2d55588405f0f58c47150e7359b8204b3d42d3db19a82b642a4a43cb08f451a2

## Diagram

.csdlc/prepared/issues/5835/diagram.mmd

Digest: 0a3ab8f4a8150b92ec738750c11ea59db723de5e8bdd6d4a18052b93ae7e632b

## Stop Conditions

- Any dependency is not current and accepted
- Required schema or evidence path is absent
- The design cannot preserve lineage or redaction without widening scope

## Handoff

Proceed only after doctor readiness.
