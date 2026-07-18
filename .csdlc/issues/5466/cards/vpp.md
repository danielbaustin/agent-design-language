# Validation Planning Prompt

Template: 1.0.0

Issue: 5466

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5466/design.md

Diagram: .csdlc/prepared/issues/5466/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-v2-publication",
    "proof_role": "Merged-head reconciliation and unchanged draft publication",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
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
      "gate6"
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

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate6`

## Failure Semantics

Reject unless the explicit GitHub PR is merged and its final head exactly matches the current clean reviewed revision.

## Handoff

Retain typed evidence before convergence.
