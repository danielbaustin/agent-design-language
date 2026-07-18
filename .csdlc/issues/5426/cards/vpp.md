# Validation Planning Prompt

Template: 1.0.0

Issue: 5426

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: docs/reviews/v0.91.7/csdlc-v2-5426/DESIGN.md

Diagram: docs/reviews/v0.91.7/csdlc-v2-5426/DIAGRAM.mmd

## Selected Lanes

[
  {
    "lane": "focused-rust-tests",
    "proof_role": "Prove both validation supersession directions and unchanged fail-closed behavior",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "validation"
    ],
    "parallel_group": "rust",
    "defer_reason": null
  },
  {
    "lane": "rust-format",
    "proof_role": "Prove Rust formatting for the changed C-SDLC v2 surface",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 200,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--",
      "--check"
    ],
    "parallel_group": "format",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml validation`
- `cargo fmt --manifest-path csdlc-v2/Cargo.toml -- --check`

## Failure Semantics

Fail closed on ambiguous identity, any latest non-passing validation, test failure, or lifecycle mismatch.

## Handoff

Retain typed evidence before convergence.
