# Structured Planning Prompt

Template: 1.0.0

Issue: 5613

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare and bind; implement the narrow typed transaction; validate and review; apply the typed issue 5591 repair; materialize exact terminal commits; prove fresh-checkout truth; publish, merge, and close out.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Prepare, review, initialize, and bind issue 5613",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement typed atomic terminal SOR validation-result repair",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Apply portable issue 5591 repair and materialize exact terminal projections",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused, full, fresh-checkout, and exact-review proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Publish, merge, post-merge validate, and close out",
    "acceptance_ids": [
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- Terminal targets never regain a claim or leave closed_out
- Original PR identity and terminal disposition never change
- Record and receipt digests remain mutually consistent
- All target mutations are exact-CAS and atomic
- No machine-local evidence path survives in issue 5591 terminal SOR

## Risks

- A broad repair API could become an unsafe terminal editor
- A partial receipt write could strand projection and receipt truth
- Commit materialization could overwrite newer terminal evidence
- Portable wording could overstate what actually ran

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5613/design.md

Digest: b01c122e173be0d1192a373fb9c9052b07529e19950e86516cfb68f725b26362

## Diagram

.csdlc/prepared/issues/5613/diagram.mmd

Digest: e6ab2cf6c47f5315004a82f111182fecfe05e3ac99b76a55a3d64fbacd006b9b

## Stop Conditions

- Any claim collision not discharged by a valid retained terminal receipt
- Any need to reopen a terminal target
- Any need to hand-edit a terminal record or receipt
- Any Runtime, ADL-v2, AWS, or unrelated milestone scope
- Any new crate dependency

## Handoff

Proceed only after doctor readiness.
