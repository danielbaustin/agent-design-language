# Validation Planning Prompt

Template: 1.0.0

Issue: 5839

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5839/design.md

Diagram: .csdlc/prepared/issues/5839/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp19-map-completeness",
    "proof_role": "Prove every required source row resolves to exact evidence or a blocker and names a consumer.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "ruby",
      ".csdlc/evidence/5839/validate-governance-handoff.rb"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "wp19-negative-governance",
    "proof_role": "Reject implicit approval, private-state exposure, ADR acceptance, citizenship, standing, and governance-completion claims.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "ruby",
      ".csdlc/evidence/5839/validate-governance-handoff.rb",
      "--negative"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "wp19-diff-review",
    "proof_role": "Prove clean patch structure before producer/consumer exact-head review.",
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

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/evidence/5839/validate-governance-handoff.rb`
- `ruby .csdlc/evidence/5839/validate-governance-handoff.rb --negative`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
