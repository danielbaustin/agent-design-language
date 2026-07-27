# Validation Planning Prompt

Template: 1.0.0

Issue: 5670

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5670/design.md

Diagram: .csdlc/prepared/issues/5670/diagram.mmd

## Selected Lanes

[
  {
    "lane": "coverage-shard-contracts",
    "proof_role": "Run focused coverage runner and CI topology contracts plus diff hygiene.",
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
    "budget_seconds": 180,
    "budget_tokens": 1200,
    "argv": [
      "bash",
      "adl/tools/test_run_authoritative_coverage_lane.sh"
    ],
    "parallel_group": "coverage-contracts",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/test_run_authoritative_coverage_lane.sh`

## Failure Semantics

Fail closed on missing shard evidence, stale or duplicate shard IDs, weakened coverage gates, local-disk fallback, failed contracts, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
