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

.csdlc/prepared/issues/5670/design.md

Digest: 630eb9e612ad8fda5b6d0b49cef712e11dba30bb48e37150bd616076cacd86b4

## Diagram

.csdlc/prepared/issues/5670/diagram.mmd

Digest: f277a099a81868b6807297b57149b00cde667977296789be5bf782695cfa9181

## Stop Conditions

- FastWork is unavailable
- typed lifecycle cannot bind #5670
- implementation requires AWS or threshold reduction
- contracts show final gate semantics weakened
- review finds unresolved actionables

## Handoff

Proceed only after doctor readiness.
