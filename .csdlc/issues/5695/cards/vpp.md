# Validation Planning Prompt

Template: 1.0.0

Issue: 5695

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5695/retained/design.md

Diagram: .csdlc/issues/5695/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "github-pr-state-classification",
    "proof_role": "Exercise explicit mergeability-state classification and fail-closed merge predicates",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "test",
      "-p",
      "csdlc-v2",
      "github::tests"
    ],
    "parallel_group": "rust",
    "defer_reason": null
  },
  {
    "lane": "github-pr-state-format",
    "proof_role": "Verify formatting and strict lint for the touched Rust source",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "fmt",
      "--check"
    ],
    "parallel_group": "rust",
    "defer_reason": null
  },
  {
    "lane": "github-pr-state-diff-hygiene",
    "proof_role": "Verify the bounded diff has no whitespace corruption",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "rust",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test -p csdlc-v2 github::tests`
- `cargo fmt --check`
- `git diff --check`

## Failure Semantics

Fail closed on unclassified variants, stale-base misclassification, merge-gate weakening, scope drift, or failed focused validation.

## Handoff

Retain typed evidence before convergence.
