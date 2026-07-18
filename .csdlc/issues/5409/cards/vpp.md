# Validation Planning Prompt

Template: 1.0.0

Issue: 5409

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5409/retained/design.md

Diagram: .csdlc/issues/5409/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp07-review-fix",
    "proof_role": "Focused WP-07 runtime hardening regressions and proof truth",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 12000,
    "argv": [
      "bash",
      "adl/tools/run_pr_fast_coverage_lane.sh"
    ],
    "parallel_group": "wp07",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/run_pr_fast_coverage_lane.sh`

## Failure Semantics

Fail closed on forged authority and proof overclaim; document bounded downgrade rather than inventing live evidence.

## Handoff

Retain typed evidence before convergence.
