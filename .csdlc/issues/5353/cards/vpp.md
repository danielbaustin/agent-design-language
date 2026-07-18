# Validation Planning Prompt

Template: 1.0.0

Issue: 5353

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: docs/architecture/csdlc-v2/wp01/5353/DESIGN.md

Diagram: docs/architecture/csdlc-v2/wp01/5353/DIAGRAM.mmd

## Selected Lanes

[
  {
    "lane": "focused-gate2",
    "proof_role": "Prove issue-local initialization and dual digest refresh behavior",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "focused-gate7",
    "proof_role": "Prove lifecycle integration remains compatible with initialization repair",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate7_lifecycle"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "v2-suite",
    "proof_role": "Prove the independent v2 workspace remains green",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "parallel_group": "suite",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate2`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate7_lifecycle`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml`

## Failure Semantics

Fail closed with typed error and no publication when initialization or digest truth is incomplete.

## Handoff

Retain typed evidence before convergence.
