# Validation Planning Prompt

Template: 1.0.0

Issue: 5841

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5841/design.md

Diagram: .csdlc/prepared/issues/5841/diagram.mmd

## Selected Lanes

[
  {
    "lane": "language-compiler-characterization",
    "proof_role": "Preserve parser/compiler deterministic identity, diagnostics, limits, and characterization parity.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-v2/Cargo.toml",
      "-p",
      "adl-language",
      "-p",
      "adl-compiler"
    ],
    "parallel_group": "parity",
    "defer_reason": null
  },
  {
    "lane": "engine-runtime-negative",
    "proof_role": "Preserve bounded scheduling, failure/resume, port contracts, and runtime negative behavior for touched owners.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-v2/Cargo.toml",
      "-p",
      "adl-engine"
    ],
    "parallel_group": "parity",
    "defer_reason": null
  },
  {
    "lane": "strict-rust-quality",
    "proof_role": "Prove touched active Rust owners remain warning-free, formatted, and workspace-compatible.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 7000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-v2/Cargo.toml",
      "--workspace",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "lint",
    "defer_reason": null
  },
  {
    "lane": "diff-metrics-platform",
    "proof_role": "Validate patch hygiene and retain before/after LoC, dependency, duplication, and required macOS/Linux CI evidence.",
    "acceptance_ids": [
      "AC-2",
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
  },
  {
    "lane": "typed-card-doctor",
    "proof_role": "Validate the exact rendered six-card bundle, cross-card references, digests, statuses, and canonical issue record.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5841"
    ],
    "parallel_group": "typed-readback",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path adl-v2/Cargo.toml -p adl-language -p adl-compiler`
- `cargo test --locked --manifest-path adl-v2/Cargo.toml -p adl-engine`
- `cargo clippy --locked --manifest-path adl-v2/Cargo.toml --workspace --all-targets -- -D warnings`
- `git diff --check`
- `csdlc-doctor --repo . --issue 5841`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
