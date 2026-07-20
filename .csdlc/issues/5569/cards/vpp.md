# Validation Planning Prompt

Template: 1.0.0

Issue: 5569

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5569/retained/design.md

Diagram: .csdlc/issues/5569/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "terminal-plan-parity",
    "proof_role": "Typed terminal record and receipt consistency",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5547"
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

- `csdlc-doctor --repo . --issue 5547`

## Failure Semantics

Fail closed on any record, receipt, generation, digest, claim, or evidence mismatch.

## Handoff

Retain typed evidence before convergence.
