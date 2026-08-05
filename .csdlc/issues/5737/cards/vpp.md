# Validation Planning Prompt

Template: 1.0.0

Issue: 5737

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5737/retained/design.md

Diagram: .csdlc/issues/5737/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "gate2-authority-recovery-focused",
    "proof_role": "Focused Gate 2 regression for unrelated stale terminal identity, stale projection filtering, reacquire, and live overlap behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2",
      "stale_terminal_identity"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-strict-clippy",
    "proof_role": "Strict Clippy over the C-SDLC v2 crate for the touched lifecycle/store/test surface.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
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

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate2 stale_terminal_identity`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed on stale CAS, protected-path collision, missing lifecycle evidence, failed focused tests, failed Clippy, or stale exact-head review.

## Handoff

Retain typed evidence before convergence.
