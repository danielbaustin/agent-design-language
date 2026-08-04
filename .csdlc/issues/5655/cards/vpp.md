# Validation Planning Prompt

Template: 1.0.0

Issue: 5655

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5655/design.md

Diagram: .csdlc/prepared/issues/5655/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-github-actions",
    "proof_role": "Run focused Rust tests covering issue mutation, exact readback, permission failure, ambiguity, and identity mismatch",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "parallel_group": "github-actions",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml`

## Failure Semantics

Fail closed on invalid requests, stale claims, permission failures, ambiguous remote outcomes, identity mismatch, test failure, or missing exact readback.

## Handoff

Retain typed evidence before convergence.
