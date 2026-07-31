# Validation Planning Prompt

Template: 1.0.0

Issue: 5624

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5624/retained/design.md

Diagram: .csdlc/issues/5624/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "guarded-prune-focused",
    "proof_role": "Prove exact issue-local, relative, absolute, malformed, wrong-checkout, dirty, and receipt-stability behavior",
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
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5624/run_focused_validation.sh"
    ],
    "parallel_group": "csdlc-v2-prune",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash .csdlc/prepared/issues/5624/run_focused_validation.sh`

## Failure Semantics

Fail closed on any non-exact topology, malformed path, receipt drift, validation failure, unresolved review finding, or protected-path expansion.

## Handoff

Retain typed evidence before convergence.
