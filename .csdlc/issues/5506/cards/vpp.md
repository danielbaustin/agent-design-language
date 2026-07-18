# Validation Planning Prompt

Template: 1.0.0

Issue: 5506

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5506/retained/design.md

Diagram: .csdlc/issues/5506/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "coverage-impact-contracts",
    "proof_role": "Prove source-to-risk mapping and expression selection",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "parallel_group": "local-tooling",
    "defer_reason": null
  },
  {
    "lane": "pr-fast-coverage-contracts",
    "proof_role": "Prove auth-only and mixed manifest routing",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/test_run_pr_fast_coverage_lane.sh"
    ],
    "parallel_group": "local-tooling",
    "defer_reason": null
  },
  {
    "lane": "tooling-review-boundary",
    "proof_role": "Prove the exact reviewed diff is limited to coverage tooling before publication",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local-review",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_check_coverage_impact.sh`
- `bash adl/tools/test_run_pr_fast_coverage_lane.sh`
- `git diff --check`

## Failure Semantics

Fail closed if auth source is unmapped, auth-only tests target the legacy workspace, mixed selectors skip ADL tests, or runtime source changes.

## Handoff

Retain typed evidence before convergence.
