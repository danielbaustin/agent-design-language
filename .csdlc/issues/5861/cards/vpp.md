# Validation Planning Prompt

Template: 1.0.0

Issue: 5861

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5861/design.md

Diagram: .csdlc/prepared/issues/5861/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-v2-preparation-binding-focused",
    "proof_role": "Prove draft, generation, readiness, dependency drift, session authority, concurrency, recovery, release, batch, migration, corruption, and path-hardening behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 12000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "preparation"
    ],
    "parallel_group": "csdlc-v2-focused",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-lifecycle-compatibility",
    "proof_role": "Prove the new preparation path preserves canonical lifecycle behavior needed after binding.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-10",
      "AC-11"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 12000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "parallel_group": "csdlc-v2-focused",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-public-contract-focused",
    "proof_role": "Prove typed commands, schemas, operator skills, installer inventory, compatibility, and deletion boundaries.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-6",
      "AC-11",
      "AC-12"
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
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a"
    ],
    "parallel_group": "csdlc-v2-contract",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-strict-lints",
    "proof_role": "Prove strict Rust lint cleanliness across the touched crate and tests.",
    "acceptance_ids": [
      "AC-11",
      "AC-12"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--tests",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "csdlc-v2-contract",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test preparation`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate2`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate10a`
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --tests -- -D warnings`

## Failure Semantics

Fail closed on stale receipts, uncertain dependency state, ambiguous migration, unsupported lock topology, overlapping path races, unowned Git artifacts, or any readiness claim not backed by the exact current semantic generation.

## Handoff

Retain typed evidence before convergence.
