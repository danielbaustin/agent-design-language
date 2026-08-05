# Validation Planning Prompt

Template: 1.0.0

Issue: 5848

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5848/design.md

Diagram: .csdlc/prepared/issues/5848/diagram.mmd

## Selected Lanes

[
  {
    "lane": "canonical-remediation-truth",
    "proof_role": "Reconstruct the complete internal plus external finding universe, validate every disposition and accepted-risk authority, read live remediation PR head/review/check/merge state and typed terminal truth, then rerun all affected quality and release validators.",
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
    "budget_seconds": 960,
    "budget_tokens": 8500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5848/validate-remediation-regressions.rb"
    ],
    "parallel_group": "remediation",
    "defer_reason": null
  },
  {
    "lane": "typed-card-doctor",
    "proof_role": "Validate the exact six-card bundle and design approval.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5848"
    ],
    "parallel_group": "typed-readback",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/5848/validate-remediation-regressions.rb`
- `csdlc-doctor --repo . --issue 5848`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
