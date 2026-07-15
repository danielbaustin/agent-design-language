# Validation Planning Prompt

Template: 1.0.0

Issue: 5390

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5390/design.md

Diagram: .csdlc/issues/5390/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-control",
    "proof_role": "Native TLS, actual-port, and CORS behavior",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "control"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-configuration",
    "proof_role": "Fail-closed TLS init and release-boundary configuration",
    "acceptance_ids": [
      "AC-2",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "configuration"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "format-and-review",
    "proof_role": "Formatting, diff integrity, and bounded pre-publication review",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "parallel_group": "review",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test control`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test configuration`
- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check`

## Failure Semantics

Fail closed on TLS loading, HTTPS transport, bound-address, CORS, or release-boundary regressions.

## Handoff

Retain typed evidence before convergence.
