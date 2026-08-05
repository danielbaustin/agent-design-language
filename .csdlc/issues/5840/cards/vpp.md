# Validation Planning Prompt

Template: 1.0.0

Issue: 5840

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5840/design.md

Diagram: .csdlc/prepared/issues/5840/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp20-coverage-positive",
    "proof_role": "Prove exact-revision parity across matrix, coverage, activation, and AEE index rows.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_v092_demo_proof_coverage.sh",
      "--positive"
    ],
    "parallel_group": "proof-index",
    "defer_reason": null
  },
  {
    "lane": "wp20-coverage-negative",
    "proof_role": "Reject missing artifacts, duplicate owners, planned-as-passed, synthetic proof, and unsupported platform claims.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_v092_demo_proof_coverage.sh",
      "--negative"
    ],
    "parallel_group": "proof-index",
    "defer_reason": null
  },
  {
    "lane": "wp20-diff-review",
    "proof_role": "Prove clean patch structure before exact-head review.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/test_v092_demo_proof_coverage.sh --positive`
- `bash adl/tools/test_v092_demo_proof_coverage.sh --negative`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
