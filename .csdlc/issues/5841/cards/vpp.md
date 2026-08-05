# Validation Planning Prompt

Template: 1.0.0

Issue: 5841

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5841/design.md

Diagram: .csdlc/prepared/issues/5841/diagram.mmd

## Selected Lanes

[
  {
    "lane": "exact-source-refactor-proof",
    "proof_role": "Restrict edits to the declared control and observability source/tests, require behavior invariants and rollback, run focused tests plus strict fmt/clippy, reject LoC/duplication regression, and bind native macOS/Linux proof to HEAD.",
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
    "budget_seconds": 1800,
    "budget_tokens": 9000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5841/validate-refactor-selection.rb"
    ],
    "parallel_group": "refactor",
    "defer_reason": null
  },
  {
    "lane": "typed-card-doctor",
    "proof_role": "Validate the exact rendered six-card bundle, cross-card references, design approval, digests, statuses, and canonical issue record.",
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
      "5841"
    ],
    "parallel_group": "typed-readback",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/5841/validate-refactor-selection.rb`
- `csdlc-doctor --repo . --issue 5841`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
