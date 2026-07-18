# Validation Planning Prompt

Template: 1.0.0

Issue: 5404

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5404/design.md

Diagram: .csdlc/prepared/issues/5404/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp12-review-fix",
    "proof_role": "Focused WP-12 review-fix validators and regression checks",
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
    "parallel_group": "wp12",
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

Fail closed on security/protocol overclaim; document downgraded truth rather than inventing proof.

## Handoff

Retain typed evidence before convergence.
