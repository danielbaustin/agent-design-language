# Validation Planning Prompt

Template: 1.0.0

Issue: 4760

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/4760/design.md

Diagram: .csdlc/prepared/issues/4760/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prep-doctor",
    "proof_role": "Focused v2 state integrity check only.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "csdlc-doctor",
      "--issue",
      "4760"
    ],
    "parallel_group": "prep",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `csdlc-doctor --issue 4760`

## Failure Semantics

Fail closed and report the exact v2 doctor or init error; do not repair outside the prep boundary.

## Handoff

Retain typed evidence before convergence.
