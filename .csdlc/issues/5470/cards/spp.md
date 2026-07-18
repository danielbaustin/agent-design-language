# Structured Planning Prompt

Template: 1.0.0

Issue: 5470

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Map the existing projection/receipt writes, introduce a journal and fsync boundaries, inject interruption at every commit point, then prove deterministic recovery and idempotence.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Map terminal projection, receipt, journal, and recovery boundaries",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement synchronized journaled commit and deterministic recovery",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run fault-injection, rollback, idempotence, and contract validation",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Projection and receipt share one canonical generation and digest
- No successful return occurs before receipt bytes and parent directory are synchronized
- Recovery is deterministic and fail-closed
- Existing receipt identity and rollback semantics are preserved

## Risks

- Filesystem synchronization semantics differ across supported hosts
- Fault injection must not weaken normal lifecycle locking or receipt retention

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5470/retained/design.md

Digest: e63f569602e8bd2b07235d08458ad1454428db6c068d1e4be531f053fe43e957

## Diagram

.csdlc/issues/5470/retained/diagram.mmd

Digest: ebf87368dc3f82c763a99bbbca79ec0609a080b21863b81ccb16d66d572168af

## Stop Conditions

- Every durable write/rename interruption point has deterministic recovery proof
- Focused durability and lifecycle checks are green
- Subagent review has no actionable findings

## Handoff

Proceed only after doctor readiness.
