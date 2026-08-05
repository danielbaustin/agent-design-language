# Validation Planning Prompt

Template: 1.0.0

Issue: 5780

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5780/design.md

Diagram: .csdlc/prepared/issues/5780/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-v2-terminal-authority-deletion",
    "proof_role": "Prove obsolete terminal mutation authority is absent while legacy data remains readable",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
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
      "gate_terminal_authority_deletion"
    ],
    "parallel_group": "csdlc-v2-deletion",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-complete",
    "proof_role": "Prove the complete independent v2 crate after deletion",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 12000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked"
    ],
    "parallel_group": "csdlc-v2-complete",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-strict-clippy",
    "proof_role": "Prove warning-free code and tests after authority deletion",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
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
    "parallel_group": "csdlc-v2-deletion",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_terminal_authority_deletion`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --locked`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --locked --all-targets -- -D warnings`

## Failure Semantics

Fail closed on stale claim, legacy compatibility drift, surviving mutation authority, failed full tests or Clippy, dirty lockfile, or stale review.

## Handoff

Retain typed evidence before convergence.
