# Validation Planning Prompt

Template: 1.0.0

Issue: 5423

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5423/retained/design.md

Diagram: .csdlc/issues/5423/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "register-diff-integrity",
    "proof_role": "Prove the register patch is whitespace-clean",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 100,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "terminal-evidence-links",
    "proof_role": "Verify each promoted remediation row names terminal retained evidence",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 300,
    "argv": [
      "rg",
      "-n",
      "5403|5406|5407",
      "docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md"
    ],
    "parallel_group": "records",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `git diff --check`
- `rg -n 5403|5406|5407 docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md`

## Failure Semantics

Fail closed on missing terminal evidence or concurrent protected-path ownership.

## Handoff

Retain typed evidence before convergence.
