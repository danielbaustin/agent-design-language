# Validation Planning Prompt

Template: 1.0.0

Issue: 5544

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5544/design.md

Diagram: .csdlc/prepared/issues/5544/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-doctor-5544",
    "proof_role": "Validate typed C-SDLC issue state and generated cards",
    "acceptance_ids": [
      "AC-1",
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
      ".adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5544"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Check edited artifacts for whitespace and patch hygiene",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local",
    "defer_reason": "Run after execution artifacts are written"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5544`
- `git diff --check`

## Failure Semantics

Fail closed on stale live truth, unresolved ownership collision, missing evidence, or release-readiness overclaim.

## Handoff

Retain typed evidence before convergence.
