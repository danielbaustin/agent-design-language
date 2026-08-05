# Validation Planning Prompt

Template: 1.0.0

Issue: 5791

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .adl/local-artifacts/5791-bootstrap/design.md

Diagram: .adl/local-artifacts/5791-bootstrap/diagram.mmd

## Selected Lanes

[
  {
    "lane": "review_artifact_validation",
    "proof_role": "focused review artifact whitespace validation",
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
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "review",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `git diff --check`

## Failure Semantics

fail_closed_on_stale_revision_or_missing_issue_truth

## Handoff

Retain typed evidence before convergence.
