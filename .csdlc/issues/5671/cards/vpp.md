# Validation Planning Prompt

Template: 1.0.0

Issue: 5671

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5671/retained/design.md

Diagram: .csdlc/issues/5671/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "provider-profile-focused",
    "proof_role": "registry, expansion, setup, and mocked Anthropic request proof",
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
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "provider_"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "provider-build-focused",
    "proof_role": "compile the touched Rust provider surface",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "check",
      "--manifest-path",
      "adl/Cargo.toml"
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

- `cargo test --manifest-path adl/Cargo.toml provider_`
- `cargo check --manifest-path adl/Cargo.toml`

## Failure Semantics

Fail closed on model, endpoint, credential, or proof drift; repair only the bounded provider surface.

## Handoff

Retain typed evidence before convergence.
