# Structured Planning Prompt

Template: 1.0.0

Issue: 5698

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Replace Runtime v3 kernel checkpoint and lifelog flat-file persistence with one redb-backed durable state authority, prove restart/corruption/identity/writer-lock behavior, run one exact review, and publish a ready PR.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind #5698 worktree and prove path ownership against active #5344",
    "acceptance_ids": [
      "AC-4",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement redb durable state module and wire checkpoint/lifelog adapters",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused proof, exact review, fixes, publication, and PR shepherding",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- redb is the single production durable state authority for kernel checkpoint and lifelog state
- no flat-file fallback after redb adoption
- transactions are atomic and restart-safe
- state root is explicit and absolute
- issue evidence remains under the #5698 worktree

## Risks

- path collision with active WP-12 #5344 protected files
- schema migration overreach
- tests accidentally proving helper APIs rather than production adapters
- Windows file-lock and database-open semantics

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/issues/5698/retained/design.md

Digest: 084aef585bb574dba031ec98f033395692dcab7fc080cbf8fb8c35bda32cee9e

## Diagram

.csdlc/issues/5698/retained/diagram.mmd

Digest: 4f69c21c6efaf9d1d23c4de644e419737cb2034a4dbc3291db0d6cc942796c8b

## Stop Conditions

- active #5344 claim remains unreleased for required Runtime v3 product paths
- typed claim collision with another active owner
- redb cannot be added without forbidden dependency churn
- production adapter cannot be wired without widening beyond declared paths
- focused persistence proof or exact review finds unresolved blockers

## Handoff

Proceed only after doctor readiness.
