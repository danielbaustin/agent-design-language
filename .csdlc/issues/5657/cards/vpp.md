# Validation Planning Prompt

Template: 1.0.0

Issue: 5657

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5657/retained/design.md

Diagram: .csdlc/issues/5657/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-launch",
    "proof_role": "Run focused configuration, readiness, Observatory, authenticated WebSocket, and Guardian lifecycle tests against the actual Rust kernel",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "parallel_group": "runtime-v3-launch",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml`

## Failure Semantics

Fail closed before readiness on missing real adapters, invalid config, TLS failure, secret leakage, route mismatch, unauthenticated WebSocket, lifecycle leak, test failure, or stale evidence.

## Handoff

Retain typed evidence before convergence.
