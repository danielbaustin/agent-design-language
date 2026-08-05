# Structured Planning Prompt

Template: 1.0.0

Issue: 5831

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement and prove WP-13A's evaluation-to-policy-to-graph DAG, including accepted and rejected paths, deterministic replay, bounded Runtime v3 integration, and rollback history.

## Plan

Revision 17

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5818, #5830, #5104, and Runtime v3 requalification, then inspect adl-runtime-kernel reasoning, cognition, governance, and durable-state authorities.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement evaluation, adaptation delta, proposal, policy decision, accepted/rejected mutation, durable history, and rollback in adl-runtime-kernel/src/adaptive_learning.rs.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run accepted/rejected focused tests, deterministic/forged-history replay negatives, resource bounds, resume continuity, and branch-built Runtime v3 integration.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve one bounded exact-head review and publish only with correct base and Closes #5831 linkage.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Missing evidence never becomes feedback and rejected proposals never mutate state or graph.
- Same durable history replays identically with continuous prefix and state hashes.
- Loop bounds, cancellation, policy authority, and rejected proposal history remain intact.

## Risks

- Proposal evaluation could mutate state before policy disposition.
- Resume or replay could accept a discontinuous or substituted prefix.
- Rollback could erase rejected history or restore mismatched hashes.

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5831/design.md

Digest: f82c7a3282a3ff4344045c4af4be1eb54f26f778334b6fa94171dc01276f3b6e

## Diagram

.csdlc/prepared/issues/5831/diagram.mmd

Digest: 859dbad5aa913e1a5f65375bd3b1ea19e42c33ed9c4c5d264053d6763a54d057

## Stop Conditions

- Any dependency or Runtime v3 qualification evidence is stale or missing.
- Any proposed implementation path targets adl/src/runtime_v2/ rather than adl-runtime-kernel.
- A shared Runtime v3 reasoning, cognition, governance, or durable-state edit lacks explicit versioning and a widened collision-checked claim.
- Rejected-path, replay, bounds, or rollback proof cannot be produced.

## Handoff

Proceed only after doctor readiness.
