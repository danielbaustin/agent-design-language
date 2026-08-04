# Structured Planning Prompt

Template: 1.0.0

Issue: 5548

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bind a dedicated issue worktree now, then later inspect Gate 2 fixture setup, apply the narrow architecture-correct repair, prove Gate 2 and csdlc-v2 locked tests, obtain review, and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect Gate 2 fixture setup and terminal recovery entry points",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the narrow fixture or store-initialization repair",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused Gate 2 proof and full csdlc-v2 locked test proof",
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
    "id": "S4",
    "action": "Run exact-revision review before PR publication",
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

- no tracked work on root main
- no weakening terminal receipt/common-directory invariants
- no implementation in the preparation commit
- no interaction with #5558 dirty worktree

## Risks

- test fixture shortcuts could mask real repository terminal recovery defects
- store initialization changes could widen beyond Gate 2 tests
- dirty #5558 worktree must remain untouched

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/issues/5548/retained/design.md

Digest: 873f98160fa0cde08b69b1506d00363f4d3a88a8c41a0bb691e014bbca0e2b8c

## Diagram

.csdlc/issues/5548/retained/diagram.mmd

Digest: 2a82386e4638a5d60a833968403eebe6f338449161129252903101d5a1be2c9a

## Stop Conditions

- another active #5548 worktree or branch appears
- typed v2 init or bind rejects the preparation state
- repair scope expands beyond Gate 2 fixture/root initialization behavior

## Handoff

Proceed only after doctor readiness.
