# Validation Planning Prompt

Template: 1.0.0

Issue: 5487

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5487/retained/design.md

Diagram: .csdlc/issues/5487/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc_terminal_design_repair",
    "proof_role": "typed authority, atomic rollback, and materialization",
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
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate7_lifecycle"
    ],
    "parallel_group": "csdlc-v2",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate7_lifecycle`

## Failure Semantics

Fail closed on authority, identity, hash, receipt, or AST mismatch; never partially repair terminal truth.

## Handoff

Retain typed evidence before convergence.
