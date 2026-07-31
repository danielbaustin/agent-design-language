# Validation Planning Prompt

Template: 1.0.0

Issue: 4758

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/4758/retained/design.md

Diagram: .csdlc/issues/4758/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp14-launch-prep",
    "proof_role": "Validate issue-local preparation packet shape and dependency-gate language without implementation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/4758/validate_preparation.rb"
    ],
    "parallel_group": "wp14-preparation",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/4758/validate_preparation.rb`

## Failure Semantics

Fail closed on missing six-card packet, missing design or diagram, implementation-state advancement, ambiguous #5384 execution gate, or receipt-gated execution language.

## Handoff

Retain typed evidence before convergence.
