# Structured Planning Prompt

Template: 1.0.0

Issue: 5695

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Read the current octocrab state mapping, implement an explicit classification table, add focused state coverage, validate the touched crate, obtain exact review, and publish only after review.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind issue 5695 with disjoint source and card paths",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Replace wildcard mergeability fallback with explicit pending-state mapping",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused tests for every supported mergeability variant and merge fail-closed behavior",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused validation and exact review before publication",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Only behind denotes stale-base ancestry
- blocked and unstable are pending rather than stale
- Dirty remains conflicted
- Unknown remains waiting
- Merge eligibility still requires required checks and review

## Risks

- octocrab may add a new enum variant
- classification tests can omit a state
- pending states can be mistaken for merge eligibility

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5695/design.md

Digest: 24f656bfc0bafcd82c8bc9db001d949366843f0ae5ad148e2fbf03646ccd0f02

## Diagram

.csdlc/prepared/issues/5695/diagram.mmd

Digest: af0bc26dc9fc7700b2d9a027755e16783c0bccb7b39a10b446cee8d9e161b95d

## Stop Conditions

- The enum variants cannot be verified from the pinned dependency
- The change requires merge-authority policy changes
- Work would touch provider, Runtime, AWS, or unrelated CI surfaces

## Handoff

Proceed only after doctor readiness.
