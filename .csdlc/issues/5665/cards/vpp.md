# Validation Planning Prompt

Template: 1.0.0

Issue: 5665

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5665/retained/design.md

Diagram: .csdlc/issues/5665/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-wss-focused",
    "proof_role": "Run focused Rust tests for real WSS auth, bidirectional frames, rotation, revocation, shutdown, health states, telemetry limits, and matrix truth",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3500,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml"
    ],
    "parallel_group": "runtime-v3-wss",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-strict-clippy",
    "proof_role": "Run strict Clippy on the touched adl-runtime crate",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "runtime-v3-wss",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path adl-runtime/Cargo.toml`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed on protected-path overlap, missing real WSS proof, unsupported telemetry claim, unresolved feature matrix row, failed validation, failed strict Clippy, missing LoC reduction, or unresolved exact-review finding.

## Handoff

Retain typed evidence before convergence.
