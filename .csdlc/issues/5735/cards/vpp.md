# Validation Planning Prompt

Template: 1.0.0

Issue: 5735

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5735/retained/design.md

Diagram: .csdlc/issues/5735/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "merged_docs_patch",
    "proof_role": "Exact committed-patch and bounded recovery validation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
    "argv": [
      "git",
      "diff",
      "--check",
      "305269157b0c1a7d18e8f6948e67f5bd1c17ec89^",
      "305269157b0c1a7d18e8f6948e67f5bd1c17ec89"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `git diff --check 305269157b0c1a7d18e8f6948e67f5bd1c17ec89^ 305269157b0c1a7d18e8f6948e67f5bd1c17ec89`

## Failure Semantics

Fail closed on any identity, review, publication, or terminal-evidence mismatch.

## Handoff

Retain typed evidence before convergence.
