# Validation Planning Prompt

Template: 1.0.0

Issue: 5719

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5719/retained/design.md

Diagram: .csdlc/issues/5719/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "ci_path_policy_contracts",
    "proof_role": "focused selector and workflow contract validation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_ci_path_policy.sh`

## Failure Semantics

Fail closed on any selector ambiguity that could skip full coverage for source/runtime/provider/tooling behavior.

## Handoff

Retain typed evidence before convergence.
