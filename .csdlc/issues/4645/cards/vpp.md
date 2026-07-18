# Validation Planning Prompt

Template: 1.0.0

Issue: 4645

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/4645/design.md

Diagram: .csdlc/prepared/issues/4645/diagram.mmd

## Selected Lanes

[
  {
    "lane": "review-prep-card-validation",
    "proof_role": "Validate typed #4645 C-SDLC state and preparation-only review inputs",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-validate",
      "--repo",
      ".",
      "--issue",
      "4645"
    ],
    "parallel_group": "review-prep",
    "defer_reason": null
  },
  {
    "lane": "review-artifact-integrity",
    "proof_role": "Validate future review artifacts are whitespace-clean and retained on tracked paths",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "review-prep",
    "defer_reason": "Run after the internal review artifacts are written"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `.adl/bin/csdlc-v2/csdlc-validate --repo . --issue 4645`
- `git diff --check`

## Failure Semantics

Fail closed on stale issue truth, missing retained evidence, overclaim, or unowned blocker ambiguity.

## Handoff

Retain typed evidence before convergence.
