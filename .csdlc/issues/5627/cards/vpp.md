# Validation Planning Prompt

Template: 1.0.0

Issue: 5627

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5627/retained/design.md

Diagram: .csdlc/issues/5627/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "four-command-focused",
    "proof_role": "Prove combined atomic operations, exact scope, direct-ready publication, active-draft compatibility, measurements, and unchanged closeout",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--tests",
      "gate4",
      "gate5",
      "gate6",
      "gate7_lifecycle"
    ],
    "parallel_group": "csdlc-v2-four-command",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --tests gate4 gate5 gate6 gate7_lifecycle`

## Failure Semantics

Fail closed with zero state writes on validation, claim, generation, scope, review, publication-identity, or closeout proof failure.

## Handoff

Retain typed evidence before convergence.
