# Validation Planning Prompt

Template: 1.0.0

Issue: 5541

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5541/retained/design.md

Diagram: .csdlc/issues/5541/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "gate10a-authority-guidance",
    "proof_role": "Final selector, current operational guidance consistency, and review-ready exact scope",
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
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a"
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

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate10a`

## Failure Semantics

Fail closed if any current operational surface routes through a sunset command.

## Handoff

Retain typed evidence before convergence.
