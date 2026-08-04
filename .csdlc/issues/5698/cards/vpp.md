# Validation Planning Prompt

Template: 1.0.0

Issue: 5698

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5698/retained/design.md

Diagram: .csdlc/issues/5698/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-redb-state-focused",
    "proof_role": "real checkpoint/lifelog redb restart, transaction, identity, corruption, and writer-lock proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "target",
      "--test",
      "durable_state"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-assembly-focused",
    "proof_role": "adapter-level proof that checkpoint/lifelog production calls use redb state",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "target",
      "--test",
      "assembly"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-clippy",
    "proof_role": "strict lint proof for touched Runtime v3 crate",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "target",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --target-dir target --test durable_state`
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --target-dir target --test assembly`
- `cargo clippy --locked --manifest-path adl-runtime-kernel/Cargo.toml --target-dir target --all-targets -- -D warnings`

## Failure Semantics

Fail closed on claim collision, partial transaction, corrupt state, unsupported schema, identity mismatch, hidden flat-file fallback, /private/tmp use, or exact review finding.

## Handoff

Retain typed evidence before convergence.
