# Validation Planning Prompt

Template: 1.0.0

Issue: 5497

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5497/retained/design.md

Diagram: .csdlc/issues/5497/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "umbrella-readiness",
    "proof_role": "Validate child inventory, canonical order, issue-local scope, and non-blocking receipt policy",
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
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor",
      "--root",
      ".",
      "--issue",
      "5497"
    ],
    "parallel_group": "coordination",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor --root . --issue 5497`

## Failure Semantics

Fail closed without mutating child state when ordering, ancestry, interface, claim, or authority truth is missing or ambiguous.

## Handoff

Retain typed evidence before convergence.
