# Structured Planning Prompt

Template: 1.0.0

Issue: 5594

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Run parallel read-only truth inventories, synthesize one canonical reconciliation, repair bounded planning/readiness defects, validate, review, and publish the WP-01 gate before downstream execution.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Collect live issue, PR, sprint umbrella, card, and canonical-document inventories in parallel",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Synthesize the dependency graph, collision map, critical path, and sprint readiness dispositions",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Repair bounded canonical planning, issue routing, umbrella, ownership, and readiness defects",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused validation and exact-revision review, then publish the readiness gate",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- no tracked work on main
- no downstream implementation before reviewed readiness
- no more than four writable issue actors after WP-01
- one serialized integration and merge queue
- external shadows remain read-only evidence producers
- no AWS and no raw gh

## Risks

- stale issue state can make prose-only readiness misleading
- overlapping sprint children can create parallel write collisions
- card preparation can accidentally become downstream implementation
- external-agent output can be mistaken for lifecycle authority

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/issues/5594/retained/design.md

Digest: ea5c7237efa120a65e9c075e95cc04809e86b95104cbc4ef61df5012229cb513

## Diagram

.csdlc/issues/5594/retained/diagram.mmd

Digest: 30964b8fae8c816e39be22d095395d5fc322e3fc200c4ed35e4fb8a75a59316a

## Stop Conditions

- live issue truth cannot be obtained through ADL owner binaries
- another active claim or worktree owns #5594
- a required repair would widen scope into implementation
- sprint ownership cannot be made non-overlapping without operator decision

## Handoff

Proceed only after doctor readiness.
