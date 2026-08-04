# Structured Planning Prompt

Template: 1.0.0

Issue: 5624

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare and bind; add failing topology regressions; implement exact canonical resolution; validate and review; publish, merge, post-merge validate, close out, retain the receipt, and prune only through the repaired guard.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Prepare, review, initialize, and bind the issue-local typed lifecycle",
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
    "action": "Add focused failing regressions and implement exact canonical topology resolution",
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
    "action": "Run FastWork proof, exact-revision review, and fix every actionable finding",
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
    "action": "Publish, shepherd exact-head checks, merge, post-merge validate, close out, retain receipt, and guarded-prune if safe",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Eligibility requires exact branch, canonical root, and live Git topology identity
- Dirty worktrees remain ineligible
- Invalid and ambiguous paths fail with unsafe_checkout
- Terminal evidence and retained receipt bytes are unchanged

## Risks

- A permissive relative-path rule could prune a same-suffix checkout
- A sentinel special case could ignore branch or live topology identity
- A repair could accidentally rewrite immutable terminal evidence
- A test could prove only a helper while missing command-level behavior

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/5624/retained/design.md

Digest: cff74b3020ec2e81e97296726d4d6333b471397c999d18f596682865d33435ff

## Diagram

.csdlc/issues/5624/retained/diagram.mmd

Digest: d9191dc1bfad775a15feeaa8e1c011cf9c51d4e30d3efa07369fc4abbccdbca9

## Stop Conditions

- Any need to mutate #5340 or migrate terminal records
- Any protected-path collision
- Any need to change Runtime code, use AWS, or add a dependency
- Any proposed rule that accepts suffix-only or traversal-bearing identity

## Handoff

Proceed only after doctor readiness.
