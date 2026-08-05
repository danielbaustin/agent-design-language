# Structured Planning Prompt

Template: 1.0.0

Issue: 5766

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Inventory endpoint declarations, reconcile advertised availability with mounted route truth, add focused drift-prevention tests, and update stale evidence wording if required.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inventory endpoint constants, routers, and consumers.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Patch source truth so advertised CSM availability matches actual routed behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add and run focused runtime API route inventory tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- main remains untouched
- no AWS
- no unimplemented endpoint is advertised as available
- Runtime v3 kernel readiness and CSM runtime API readiness remain distinct

## Risks

- two similarly named runtime API surfaces drift independently
- planned feature inventory is confused with availability inventory
- tests overfit one router surface and miss consumers

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5766/design.md

Digest: bc9d712af5b4017312b9d38543350f90cbe8976dac2e2ab83c7654b9757b656d

## Diagram

.csdlc/prepared/issues/5766/diagram.mmd

Digest: d8c37ec3f630ffaed294dfd462f48b6682781c8caff3140711edba942509cf35

## Stop Conditions

- active claim collision
- source surfaces require broader product API implementation
- focused tests cannot express router/inventory truth

## Handoff

Proceed only after doctor readiness.
