# Validation Planning Prompt

Template: 1.0.0

Issue: 5509

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5509/retained/design.md

Diagram: .csdlc/issues/5509/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "ci-path-policy-contracts",
    "proof_role": "Prove narrow route selection and fail-closed fallback",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "parallel_group": "tooling-contracts",
    "defer_reason": null
  },
  {
    "lane": "pr-fast-runner-contracts",
    "proof_role": "Prove independent test and coverage execution",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_run_pr_fast_test_lane.sh"
    ],
    "parallel_group": "tooling-contracts",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/test_ci_path_policy.sh`
- `bash adl/tools/test_run_pr_fast_test_lane.sh`

## Failure Semantics

Fail closed if the bounded path family is incomplete, either crate is omitted, Runtime v2 is selected, or unrelated mixed-crate shapes become focused.

## Handoff

Retain typed evidence before convergence.
