# Validation Planning Prompt

Template: 1.0.0

Issue: 5653

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5653/retained/design.md

Diagram: .csdlc/issues/5653/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "readme-focused",
    "proof_role": "Markdown structure, link targets, badge branch, and stale-version wording",
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
    "budget_tokens": 800,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local",
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

## Failure Semantics

Fail closed and preserve truthful release boundaries.

## Handoff

Retain typed evidence before convergence.
