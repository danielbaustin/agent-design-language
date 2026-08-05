# Structured Planning Prompt

Template: 1.0.0

Issue: 5663

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bind a disjoint issue worktree from current origin/main, measure the owned surface, replace receipt-only local adapter behavior in assembly, add focused tests, prove net LoC reduction, run strict Clippy, and obtain exact review before publication.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Initialize and bind #5663 with a protected path set disjoint from active Runtime claims",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Measure before physical LoC for the owned assembly surface",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement durable bounded local adapter behavior and remove superseded receipt-only duplication",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused tests, strict Clippy, and after physical LoC measurement",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run exact pre-PR review, fix actionable findings, and publish only after review passes",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Production readiness remains fail-closed for missing required adapter bindings
- Lifelog is evidence, not authority
- Checkpoint restore is authenticated by local state integrity and rejects corrupt state
- Canonical ingress remains the local domain-work entrypoint
- External transport adapters remain non-goals

## Risks

- Active stale claims could overlap if the implementation needs to widen beyond assembly.rs and tests/assembly.rs
- Durable behavior could accidentally become fixture-only if checkpoint and lifelog paths are not exercised through production executors
- Deleting duplicate paths could change external transport behavior if scope is not held tightly
- Net LoC reduction could fail if tests expand faster than implementation shrinks

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/issues/5663/retained/design.md

Digest: 8dbec1cb6e092383069f56bab7993e0472ae3f5c9085e5d6ecba7db4aee12efc

## Diagram

.csdlc/issues/5663/retained/diagram.mmd

Digest: 0a25cd071002b9a9c002719ecb9bc4a82e9cce0c93cd51b102b4680324ce1e9a

## Stop Conditions

- Any typed claim collision on protected source paths
- Any need to edit main, use AWS, use Python wrappers, or mutate external transport scope
- Any implementation path requiring operations.rs, governed_operations.rs, launch binary, config, control, Observatory, or adl-runtime edits without explicit claim amendment
- Any inability to prove net LoC reduction

## Handoff

Proceed only after doctor readiness.
