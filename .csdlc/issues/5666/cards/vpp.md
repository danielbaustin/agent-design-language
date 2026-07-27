# Validation Planning Prompt

Template: 1.0.0

Issue: 5666

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5666/design.md

Diagram: .csdlc/prepared/issues/5666/diagram.mmd

## Selected Lanes

[
  {
    "lane": "throughput-fast-lane-contract",
    "proof_role": "Run focused shell contract test and diff hygiene for policy links and required invariants.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "bash",
      "adl/tools/test_developer_throughput_fast_lane.sh"
    ],
    "parallel_group": "docs-policy",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_developer_throughput_fast_lane.sh`

## Failure Semantics

Fail closed on missing policy, broken links, absent FastWork/no-local-fallback language, broad CI/runtime scope, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
