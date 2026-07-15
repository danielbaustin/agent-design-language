# Validation Planning Prompt

Template: 1.0.0

Issue: 5330

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: docs/architecture/runtime_v3_fast_validation_5330.md

Diagram: docs/architecture/runtime_v3_fast_validation_5330.mmd

## Selected Lanes

[
  {
    "lane": "runtime_v3_fast",
    "proof_role": "selector and focused Runtime v3 validation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      "bash",
      "adl/tools/run_runtime_v3_fast_validation_lane.sh"
    ],
    "parallel_group": "runtime-v3",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/run_runtime_v3_fast_validation_lane.sh`

## Failure Semantics

Fail closed on unmapped Runtime v3 paths or missing focused proof commands; preserve legacy lanes for mixed diffs.

## Handoff

Retain typed evidence before convergence.
