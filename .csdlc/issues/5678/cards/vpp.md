# Validation Planning Prompt

Template: 1.0.0

Issue: 5678

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5678/design.md

Diagram: .csdlc/prepared/issues/5678/diagram.mmd

## Selected Lanes

[
  {
    "lane": "opus-runbook-contract",
    "proof_role": "Verify documented adapter flags, JSON fields, credential handling, and review truth boundaries against source text",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/test_opus_review_runbook.sh"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "opus-runbook-diff-hygiene",
    "proof_role": "Verify the bounded documentation diff has no whitespace corruption",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_opus_review_runbook.sh`
- `git diff --check`

## Failure Semantics

Fail closed on stale interface claims, missing schema evidence, secret or absolute-path leakage, scope drift, or failed focused validation.

## Handoff

Retain typed evidence before convergence.
