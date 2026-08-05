# Validation Planning Prompt

Template: 1.0.0

Issue: 5610

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5610/retained/design.md

Diagram: .csdlc/issues/5610/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "coverage-summary-merge-contract",
    "proof_role": "Prove safe canonicalization and fail-closed ownership traversal",
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
      "adl/tools/test_merge_coverage_summaries.sh"
    ],
    "parallel_group": "coverage-contracts",
    "defer_reason": null
  },
  {
    "lane": "ci-runtime-contract",
    "proof_role": "Prove coupled CI coverage orchestration remains unchanged",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "parallel_group": "coverage-contracts",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_merge_coverage_summaries.sh`
- `bash adl/tools/test_ci_runtime_contracts.sh`

## Failure Semantics

Fail closed on any repository-prefix or owned-root escape, malformed summary, duplicate canonical filename, metric error, or output-write failure.

## Handoff

Retain typed evidence before convergence.
