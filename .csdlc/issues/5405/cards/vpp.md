# Validation Planning Prompt

Template: 1.0.0

Issue: 5405

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5405/retained/design.md

Diagram: .csdlc/issues/5405/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp13-review-fix",
    "proof_role": "Focused WP-13 claim truth and Runtime v2 economics duplicate validation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 12000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "runtime_v2_economics_civilization_boundary"
    ],
    "parallel_group": "wp13",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path adl/Cargo.toml runtime_v2_economics_civilization_boundary`

## Failure Semantics

Fail closed on overclaim; prefer bounded downgrade truth over invented integration.

## Handoff

Retain typed evidence before convergence.
