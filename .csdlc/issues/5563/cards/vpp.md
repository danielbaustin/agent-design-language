# Validation Planning Prompt

Template: 1.0.0

Issue: 5563

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5563/retained/design.md

Diagram: .csdlc/issues/5563/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "gate2-design-recovery",
    "proof_role": "Typed lifecycle and atomic card recovery",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2",
      "initialized_approved_stale_design"
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

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate2 initialized_approved_stale_design`

## Failure Semantics

Fail closed without exact stale-input, CAS, claim, and reviewer evidence.

## Handoff

Retain typed evidence before convergence.
