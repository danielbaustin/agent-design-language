# Validation Planning Prompt

Template: 1.0.0

Issue: 4761

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/4761/retained/design.md

Diagram: .csdlc/issues/4761/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "capability-envelope-validator",
    "proof_role": "Fail-closed validation of the #4761 capability envelope, retained source inventory, consumer surfaces, digest integrity, and explicit non-claims.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/evidence/4761/capability-envelope/validate_capability_envelope.rb"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace and patch hygiene errors in the capability-envelope artifacts, lifecycle records, and consumer docs.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
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

- `ruby .csdlc/evidence/4761/capability-envelope/validate_capability_envelope.rb`
- `git diff --check`

## Failure Semantics

Fail closed and report the exact v2 doctor or init error; do not repair outside the prep boundary.

## Handoff

Retain typed evidence before convergence.
