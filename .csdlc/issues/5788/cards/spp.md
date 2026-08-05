# Structured Planning Prompt

Template: 1.0.0

Issue: 5788

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Harden the two owner build paths, add fixture-only shell regressions, review exact head, and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Implement current inventory and lock guard",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run focused shell contracts and exact review",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Publish with Closes #5788",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Exact pre-invocation lock bytes are authoritative
- No v1-sunset target returns to the inventory

## Risks

- Restoration could overwrite pre-existing user changes
- Inventory could omit a current operational binary

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5788/design.md

Digest: 8337a903b8ba0bdd7e7dcf89b9cab4578b6dcb26927aca1e368a21efcf473dee

## Diagram

.csdlc/prepared/issues/5788/diagram.mmd

Digest: 80cb9f38fa23be1b10d13e135847a44ba0d27470b5cd0afb7695e6d0c09955b2

## Stop Conditions

- Any dependency mutation is required
- Any protected-path collision is reported

## Handoff

Proceed only after doctor readiness.
