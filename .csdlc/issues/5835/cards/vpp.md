# Validation Planning Prompt

Template: 1.0.0

Issue: 5835

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5835/design.md

Diagram: .csdlc/prepared/issues/5835/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp17-doc-contract",
    "proof_role": "Prove all transfer rows, source paths, and WP-04/v0.93 boundaries are complete.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "ruby",
      ".csdlc/evidence/5835/validate-continuity-transfer.rb"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "wp17-negative-semantics",
    "proof_role": "Reject copied/conflicting state, raw-memory transfer, and production/governance overclaim.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "ruby",
      ".csdlc/evidence/5835/validate-continuity-transfer.rb",
      "--negative"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "wp17-diff-review",
    "proof_role": "Prove clean patch structure before exact-head review.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/evidence/5835/validate-continuity-transfer.rb`
- `ruby .csdlc/evidence/5835/validate-continuity-transfer.rb --negative`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
