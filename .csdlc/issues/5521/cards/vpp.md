# Validation Planning Prompt

Template: 1.0.0

Issue: 5521

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5521/design.md

Diagram: .csdlc/prepared/issues/5521/diagram.mmd

## Selected Lanes

[
  {
    "lane": "terminal-target-doctor",
    "proof_role": "Prove #5518 terminal parity",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "csdlc-doctor",
      "--issue",
      "5518"
    ],
    "parallel_group": "records",
    "defer_reason": null
  },
  {
    "lane": "authority-doctor",
    "proof_role": "Prove #5521 lifecycle truth",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "csdlc-doctor",
      "--issue",
      "5521"
    ],
    "parallel_group": "records",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `csdlc-doctor --issue 5518`
- `csdlc-doctor --issue 5521`

## Failure Semantics

Fail closed on stale authority, target, receipt, unexpected semantic drift, doctor failure, or any source change.

## Handoff

Retain typed evidence before convergence.
