# Validation Planning Prompt

Template: 1.0.0

Issue: 5468

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5468/retained/design.md

Diagram: .csdlc/issues/5468/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-csdlc-v2-lifecycle",
    "proof_role": "Prove terminal SRP status normalization and retained receipt parity",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate7_lifecycle",
      "terminal_reconciliation"
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

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate7_lifecycle terminal_reconciliation`

## Failure Semantics

Fail closed on any card validation, receipt parity, digest, rollback, or focused lifecycle regression failure.

## Handoff

Retain typed evidence before convergence.
