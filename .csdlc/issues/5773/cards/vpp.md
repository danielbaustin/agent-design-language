# Validation Planning Prompt

Template: 1.0.0

Issue: 5773

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5773/retained/design.md

Diagram: .csdlc/issues/5773/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "terminal-evidence",
    "proof_role": "Validate observed closure evidence",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 100,
    "argv": [
      "csdlc-closeout",
      "recover-recordless",
      "--issue",
      "5773",
      "--pr",
      "5774"
    ],
    "parallel_group": "terminal-recovery",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `csdlc-closeout recover-recordless --issue 5773 --pr 5774`

## Failure Semantics

Fail closed without exact, internally consistent terminal evidence.

## Handoff

Retain typed evidence before convergence.
