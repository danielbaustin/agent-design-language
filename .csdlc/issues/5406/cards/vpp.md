# Validation Planning Prompt

Template: 1.0.0

Issue: 5406

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5406/retained/design.md

Diagram: .csdlc/issues/5406/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-v2-full",
    "proof_role": "Prove typed lifecycle and card contracts remain deterministic and green",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 20000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "parallel_group": "rust",
    "defer_reason": null
  },
  {
    "lane": "docs-integrity",
    "proof_role": "Prove retained authority packet and patch integrity",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml`
- `git diff --check`

## Failure Semantics

Fail closed on overlap, stale generation, invalid status transition, malformed validation lane, non-portable evidence, or v1 surface restoration.

## Handoff

Retain typed evidence before convergence.
