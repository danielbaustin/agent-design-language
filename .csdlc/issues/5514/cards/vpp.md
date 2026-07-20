# Validation Planning Prompt

Template: 1.0.0

Issue: 5514

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5514/retained/design.md

Diagram: .csdlc/issues/5514/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "pr-fast-coverage-contract",
    "proof_role": "Prove complete workspace partitioning and exact command construction",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_run_pr_fast_coverage_lane.sh"
    ],
    "parallel_group": "tooling-contracts",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_run_pr_fast_coverage_lane.sh`

## Failure Semantics

Fail closed if any canonical ADL CSM selector is discarded, any foreign selector reaches an owning workspace, exact matching weakens, thresholds change, or Runtime v2/AWS enters scope.

## Handoff

Retain typed evidence before convergence.
