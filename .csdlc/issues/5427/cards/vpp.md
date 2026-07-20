# Validation Planning Prompt

Template: 1.0.0

Issue: 5427

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5427/retained/design.md

Diagram: .csdlc/issues/5427/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-card-identity",
    "proof_role": "Typed identity round-trip, rejection, atomicity, and #5353 validation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "parallel_group": "csdlc-v2-focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml`

## Failure Semantics

Fail closed on malformed identity or partial projection; preserve prior canonical state and evidence.

## Handoff

Retain typed evidence before convergence.
