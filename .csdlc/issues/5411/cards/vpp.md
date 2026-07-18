# Validation Planning Prompt

Template: 1.0.0

Issue: 5411

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5411/retained/design.md

Diagram: .csdlc/issues/5411/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-full",
    "proof_role": "Prove pressure monitoring, signed continuity, release evidence semantics, and all Runtime v3 kernel contracts",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 12000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets"
    ],
    "parallel_group": "runtime-v3",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-guardian",
    "proof_role": "Prove Unix process-tree containment, bounded capture, restart, and signal forwarding",
    "acceptance_ids": [
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "guardian::tests"
    ],
    "parallel_group": "runtime-v3-guardian",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-quality",
    "proof_role": "Prove formatting, warnings, and implementation budget truth",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "runtime-v3-quality",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --all-targets`
- `cargo test --manifest-path adl-runtime/Cargo.toml guardian::tests`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed: preserve the running process when continuity cannot commit, report the terminal error, and do not promote non-executed evidence.

## Handoff

Retain typed evidence before convergence.
