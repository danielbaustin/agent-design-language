# Validation Planning Prompt

Template: 1.0.0

Issue: 5548

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5548/retained/design.md

Diagram: .csdlc/issues/5548/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-gate2",
    "proof_role": "Prove the Gate 2 fixtures reach intended assertions and pass",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-locked",
    "proof_role": "Prove the full C-SDLC v2 locked test suite after the focused repair",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
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

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate2`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml`

## Failure Semantics

Fail closed on lifecycle ambiguity, fixture shortcuts that weaken real-repository terminal recovery, #5558 collision, raw gh/AWS requirement, or preparation scope expanding into implementation.

## Handoff

Retain typed evidence before convergence.
