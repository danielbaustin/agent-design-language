# Validation Planning Prompt

Template: 1.0.0

Issue: 5812

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5812/design.md

Diagram: .csdlc/prepared/issues/5812/diagram.mmd

## Selected Lanes

[
  {
    "lane": "freedom-gate-clippy",
    "proof_role": "Prove the exact defaults and named production binary are behaviorally correct and Clippy-clean with warnings denied.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl-gws-context-mirror",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "rust",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject unrelated whitespace changes and support exact-head review.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo clippy --locked --manifest-path adl/Cargo.toml --bin adl-gws-context-mirror -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on semantic drift, lint failure, unrelated changes, or missing focused proof.

## Handoff

Retain typed evidence before convergence.
