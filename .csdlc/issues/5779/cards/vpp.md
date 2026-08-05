# Validation Planning Prompt

Template: 1.0.0

Issue: 5779

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5779/design.md

Diagram: .csdlc/prepared/issues/5779/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-v2-cleanup",
    "proof_role": "Prove safe cleanup classifications, idempotent removal, concurrency, and legacy compatibility parity",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--test",
      "gate_cleanup"
    ],
    "parallel_group": "csdlc-v2-cleanup",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-cleanup-clippy",
    "proof_role": "Prove warning-free standalone cleanup implementation",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "csdlc-v2-cleanup",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_cleanup`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --locked --all-targets -- -D warnings`

## Failure Semantics

Fail closed on stale claim, dirty or ambiguous topology, legacy parity mismatch, failed focused proof, or stale review.

## Handoff

Retain typed evidence before convergence.
