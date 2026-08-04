# Validation Planning Prompt

Template: 1.0.0

Issue: 5766

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5766/design.md

Diagram: .csdlc/prepared/issues/5766/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-api-focused",
    "proof_role": "focused tests for endpoint inventory and routed API truth",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
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

- `cargo test --manifest-path adl/Cargo.toml`

## Failure Semantics

Fail closed on inventory/router mismatch, fake mounted readiness, or focused runtime API test failure.

## Handoff

Retain typed evidence before convergence.
