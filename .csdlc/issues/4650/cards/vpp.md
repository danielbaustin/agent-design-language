# Validation Planning Prompt

Template: 1.0.0

Issue: 4650

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/4650/design.md

Diagram: .csdlc/prepared/issues/4650/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prep-doctor",
    "proof_role": "Validate typed C-SDLC prep state and issue-local cards",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "csdlc-doctor",
      "--issue",
      "4650"
    ],
    "parallel_group": "prep",
    "defer_reason": null
  },
  {
    "lane": "artifact-integrity",
    "proof_role": "Validate tracked ceremony artifacts including newly added files",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "git",
      "diff",
      "--cached",
      "--check"
    ],
    "parallel_group": "prep",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `csdlc-doctor --issue 4650`
- `git diff --cached --check`

## Failure Semantics

Fail closed on stale live truth, missing retained evidence, overclaim, or dependency ambiguity.

## Handoff

Retain typed evidence before convergence.
