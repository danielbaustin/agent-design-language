# Structured Planning Prompt

Template: 1.0.0

Issue: 5670

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Split hosted workspace coverage into deterministic shard producers and keep adl-coverage-hosted as the authoritative aggregation gate.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Add runner shard controls and focused runner contracts",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Update CI hosted coverage topology and aggregation contracts",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Validate, review, publish, shepherd, merge, and close out",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- final required coverage gate remains authoritative
- shard evidence is isolated and deterministic
- coverage thresholds and ownership filtering are not weakened
- local validation uses FastWork only

## Risks

- workflow matrix fan-out can make check naming or required-check semantics confusing
- artifact naming mistakes can hide missing shard evidence
- runner shard controls can accidentally reduce test scope if not contract-tested

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/issues/5670/retained/design.md

Digest: 764bab4003984fb00686e5406ca95b56ee787d28cfa247daca83a01675c8fed3

## Diagram

.csdlc/issues/5670/retained/diagram.mmd

Digest: e5669ad5aed97dc369bbe6effc648fe333271a8cf1076a975d6de6897313d1a3

## Stop Conditions

- FastWork is unavailable
- typed lifecycle cannot bind #5670
- implementation requires AWS or threshold reduction
- contracts show final gate semantics weakened
- review finds unresolved actionables

## Handoff

Proceed only after doctor readiness.
