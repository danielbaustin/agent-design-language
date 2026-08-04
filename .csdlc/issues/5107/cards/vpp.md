# Validation Planning Prompt

Template: 1.0.0

Issue: 5107

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5107/retained/design.md

Diagram: .csdlc/issues/5107/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-doc-contract",
    "proof_role": "Prove the #5107 queue cites exact platform inputs, preserves the #5104 historical-input boundary, keeps graph-mutation non-claims, and does not request child implementation issues.",
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
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5107/validate_preparation.rb"
    ],
    "parallel_group": "planning-contract",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/5107/validate_preparation.rb`

## Failure Semantics

Fail closed on stale typed state, overclaims, missing exact input revisions, hidden implementation, child-issue creation, stale review, publication drift, or merge attempts.

## Handoff

Retain typed evidence before convergence.
