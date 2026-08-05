# Validation Planning Prompt

Template: 1.0.0

Issue: 5778

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5778/design.md

Diagram: .csdlc/prepared/issues/5778/diagram.mmd

## Selected Lanes

[
  {
    "lane": "finish-characterization",
    "proof_role": "Prove exact-head, terminal disposition, idempotency, interruption, concurrency, and legacy compatibility behavior",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_finish"
    ],
    "parallel_group": "finish-focused",
    "defer_reason": null
  },
  {
    "lane": "lifecycle-regression",
    "proof_role": "Preserve Gate 7 legacy lifecycle reads and claim safety while the new finish path is added",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate7_lifecycle"
    ],
    "parallel_group": "finish-regression",
    "defer_reason": null
  },
  {
    "lane": "finish-quality",
    "proof_role": "Prove warning-free C-SDLC v2 code across all targets",
    "acceptance_ids": [
      "AC-7"
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
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "finish-quality",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_finish`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate7_lifecycle`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed before merge on any stale digest, claim, review, head, check, publication, repository, PR, or no-PR authority mismatch; after a proven remote terminal event, retries must converge without requiring tracked state.

## Handoff

Retain typed evidence before convergence.
