# Validation Planning Prompt

Template: 1.0.0

Issue: 5853

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5853/design.md

Diagram: .csdlc/prepared/issues/5853/diagram.mmd

## Selected Lanes

[
  {
    "lane": "build-acceleration-experiment-contract",
    "proof_role": "Validate the frozen manifest, raw trial completeness, cache classification, statistical decision table, proof parity, canary, cost controls, and observation-or-cleanup record.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5853/validate-experiment.rb"
    ],
    "parallel_group": "experiment-analysis",
    "defer_reason": null
  },
  {
    "lane": "exact-head-diff-hygiene",
    "proof_role": "Reject whitespace errors in the exact reviewed candidate.",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5853/validate-experiment.rb`
- `git diff --check`

## Failure Semantics

Fail closed on incomplete entry gates, incomparable trials, missing samples, proof drift, access expansion, budget breach, or false acceleration claims; preserve the standard-runner route and specific evidence instead of degrading.

## Handoff

Retain typed evidence before convergence.
