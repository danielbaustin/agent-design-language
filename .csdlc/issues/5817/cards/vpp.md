# Validation Planning Prompt

Template: 1.0.0

Issue: 5817

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5817/design.md

Diagram: .csdlc/prepared/issues/5817/diagram.mmd

## Selected Lanes

[
  {
    "lane": "v092-planning-package",
    "proof_role": "Validate YAML, Markdown links, canonical versions, dependency acyclicity, typed card structure, scope, and diff hygiene",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 5000,
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

Seconds: 7200

Tokens: 50000

## Commands

- `git diff --check`

## Failure Semantics

Fail closed on contradictory evidence, duplicates, cyclic dependencies, invalid cards, or scope widening; record named gaps instead of completion claims.

## Handoff

Retain typed evidence before convergence.
