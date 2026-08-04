# Validation Planning Prompt

Template: 1.0.0

Issue: 5733

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5733/retained/design.md

Diagram: .csdlc/issues/5733/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "v0918-demo-matrix-validator",
    "proof_role": "Focused deterministic validation for owner, evidence, disposition, and contradiction coverage in the v0.91.8 demo matrix and feature-proof coverage docs.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1,
    "argv": [
      "python3",
      "adl/tools/validate_v0918_demo_matrix.py"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "docs-diff-hygiene",
    "proof_role": "Git diff whitespace hygiene for the bounded documentation and validator changes.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 adl/tools/validate_v0918_demo_matrix.py`
- `git diff --check`

## Failure Semantics

Fail closed on stale issue/proof claims, missing owner, missing evidence/disposition, contradictory matrix rows, validator failure, or stale exact-head review.

## Handoff

Retain typed evidence before convergence.
