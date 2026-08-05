# Validation Planning Prompt

Template: 1.0.0

Issue: 5645

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5645/retained/design.md

Diagram: .csdlc/issues/5645/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-merge-focused",
    "proof_role": "command contract and merge safety",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "merge"
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

- `cargo test --manifest-path csdlc-v2/Cargo.toml merge`

## Failure Semantics

Fail closed and report a typed error.

## Handoff

Retain typed evidence before convergence.
