# Structured Planning Prompt

Template: 1.0.0

Issue: 5495

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Define safe lifecycle metadata paths, derive automatic proof only for metadata-only commit transitions, and prove source drift remains blocked.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Define and implement bounded lifecycle metadata classification",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Integrate automatic metadata-only proof into repository review guard",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add #4641-style regression proof and validate merged guard behavior",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Only explicit recognized lifecycle metadata paths are non-substantive
- A source or retained-design change cannot pass automatic proof
- Explicit malformed proof cannot be upgraded by automatic classification

## Risks

- Over-broad metadata paths could mask real work
- Publication commits can touch several generated card surfaces
- Revision parsing must remain exact and commit-bound

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5495/retained/design.md

Digest: 68e7e87969ff9a80c12e15a24e6fde8d4c4f1b9bd1f6dd7329ce94edf680a965

## Diagram

.csdlc/issues/5495/retained/diagram.mmd

Digest: b30d358fcc11c8e15db132997d4600948b7acbe41b21974f8d19989b3b6633e5

## Stop Conditions

- Focused regression tests pass
- Substantive drift remains blocked
- Clippy and diff checks pass

## Handoff

Proceed only after doctor readiness.
