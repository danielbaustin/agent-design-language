# Validation Planning Prompt

Template: 1.0.0

Issue: 5762

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5762/retained/design.md

Diagram: .csdlc/issues/5762/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-terminal-sor-validation",
    "proof_role": "Runs the three terminal SOR validation repair tests together.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "store",
      "terminal_sor_validation_repair"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-all-target-tests",
    "proof_role": "Full locked all-target C-SDLC v2 regression suite.",
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
    "budget_seconds": 900,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "strict-clippy",
    "proof_role": "Strict lint proof for the C-SDLC v2 touched surface.",
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
    "budget_seconds": 900,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
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

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test store terminal_sor_validation_repair`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --all-targets`
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed, preserve artifacts, and report the typed blocker without widening scope.

## Handoff

Retain typed evidence before convergence.
