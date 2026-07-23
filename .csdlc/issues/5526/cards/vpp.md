# Validation Planning Prompt

Template: 1.0.0

Issue: 5526

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5526/design.md

Diagram: .csdlc/prepared/issues/5526/diagram.mmd

## Selected Lanes

[
  {
    "lane": "provider-expansion-prep",
    "proof_role": "Validate issue-local six-card/design/diagram packet shape and execution-gate language without product implementation or provider calls",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5526/validate_preparation.rb"
    ],
    "parallel_group": "provider-expansion-prep",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/5526/validate_preparation.rb`

## Failure Semantics

Fail closed on missing live WP-09 merge ancestry, active claim collision, stale generation, secret leakage risk, provider-call requirement during preparation, skipped deterministic proof, or review/publication drift.

## Handoff

Retain typed evidence before convergence.
