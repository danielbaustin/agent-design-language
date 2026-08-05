# Validation Planning Prompt

Template: 1.0.0

Issue: 5663

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5663/retained/design.md

Diagram: .csdlc/issues/5663/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-local-adapters-assembly",
    "proof_role": "Prove real Agent execution, scheduler retirement/reuse, checkpoint byte persistence/restore with integrity and identity checks, live owner and duplicate-waiter cancellation semantics, safe configured storage locking with ownership/stale/partial-lock recovery, production assembly wiring, ingress dispatch, and fail-closed external transports.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 840,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "/Volumes/FastWork/adl-wp-5663/target",
      "--test",
      "assembly"
    ],
    "parallel_group": "runtime-v3-local-adapters",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-local-adapters-governed",
    "proof_role": "Prove governed Runtime v3 restart, checkpoint, lifelog, scheduler, shepherd, cancellation, provider, and shutdown behavior remains green after local adapter correction.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "/Volumes/FastWork/adl-wp-5663/target",
      "--test",
      "governed_operations"
    ],
    "parallel_group": "runtime-v3-local-adapters",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-local-adapters-clippy",
    "proof_role": "Prove strict all-target Rust lint cleanliness for the touched Runtime v3 kernel crate.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "/Volumes/FastWork/adl-wp-5663/target",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "runtime-v3-local-adapters",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-local-adapters-loc",
    "proof_role": "Retain physical LoC measurement for touched source and test paths; before 3796, after 3791, net -5.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--numstat",
      "origin/main",
      "--",
      "adl-runtime-kernel/src/assembly.rs",
      "adl-runtime-kernel/src/bin/adl-runtime-kernel.rs",
      "adl-runtime-kernel/src/governed_operations.rs",
      "adl-runtime-kernel/src/operations.rs",
      "adl-runtime-kernel/tests/assembly.rs",
      "adl-runtime-kernel/tests/operations.rs"
    ],
    "parallel_group": "runtime-v3-local-adapters",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-local-adapters-diff-check",
    "proof_role": "Prove tracked diff whitespace hygiene after regenerating evidence logs.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "runtime-v3-local-adapters",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --target-dir /Volumes/FastWork/adl-wp-5663/target --test assembly`
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --target-dir /Volumes/FastWork/adl-wp-5663/target --test governed_operations`
- `cargo clippy --locked --manifest-path adl-runtime-kernel/Cargo.toml --target-dir /Volumes/FastWork/adl-wp-5663/target --all-targets -- -D warnings`
- `git diff --numstat origin/main -- adl-runtime-kernel/src/assembly.rs adl-runtime-kernel/src/bin/adl-runtime-kernel.rs adl-runtime-kernel/src/governed_operations.rs adl-runtime-kernel/src/operations.rs adl-runtime-kernel/tests/assembly.rs adl-runtime-kernel/tests/operations.rs`
- `git diff --check`

## Failure Semantics

Fail closed on claim collision, stale generation, missing dependency ancestry, skipped proof, degraded/receipt-only production behavior, external transport scope creep, or non-negative LoC delta.

## Handoff

Retain typed evidence before convergence.
