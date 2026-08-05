# Validation Planning Prompt

Template: 1.0.0

Issue: 5815

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5815/design.md

Diagram: .csdlc/prepared/issues/5815/diagram.mmd

## Selected Lanes

[
  {
    "lane": "docs-migration-plan",
    "proof_role": "Validate inventory wording, link references, Markdown hygiene, and diff scope",
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
    "budget_tokens": 2000,
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

- `git diff --check`

## Failure Semantics

Fail closed if inventory is ambiguous, scope widens, or the plan implies transfer authorization.

## Handoff

Retain typed evidence before convergence.
