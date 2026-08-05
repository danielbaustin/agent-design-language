# Structured Planning Prompt

Template: 1.0.0

Issue: 5831

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement and prove WP-13A's evaluation-to-policy-to-graph DAG, including accepted and rejected paths, deterministic replay, bounded Runtime v3 integration, and rollback history.

## Plan

Revision 8

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5818, #5830, #5104, and Runtime v3 requalification, then inspect exact loop, graph, bridge, and policy contracts.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement evaluation bindings, adaptation deltas, proposals, policy decisions, accepted/rejected mutation, durable history, and rollback records.",
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
      "AC-6",
      "AC-7"
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

Digest: 24460a2aafdbc8f6076f5503f41bb945f313268fb975783efcbdbed3b0baaaf6

## Diagram

.csdlc/prepared/issues/5831/diagram.mmd

Digest: 4830e2ee5a1da3b7673f5b3ef82de34dad490fdea86817cf0483f37930c2bdc5

## Stop Conditions

- Any dependency or Runtime v3 qualification evidence is stale or missing.
- A shared loop/graph schema change lacks explicit versioning.
- Rejected-path, replay, bounds, or rollback proof cannot be produced.

## Handoff

Proceed only after doctor readiness.
