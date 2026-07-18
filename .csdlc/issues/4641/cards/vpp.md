# Validation Planning Prompt

Template: 1.0.0

Issue: 4641

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/4641/retained/design.md

Diagram: .csdlc/issues/4641/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prep-doctor",
    "proof_role": "Validate typed C-SDLC prep state and issue-local cards",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "4641"
    ],
    "parallel_group": "prep",
    "defer_reason": null
  },
  {
    "lane": "artifact-integrity",
    "proof_role": "Validate future tracked artifacts are clean after execution",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
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
    "parallel_group": "prep",
    "defer_reason": "Run after execution artifacts are written"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 4641`
- `git diff --check`

## Failure Semantics

Fail closed on stale live truth, missing retained evidence, overclaim, or dependency ambiguity.

## Handoff

Retain typed evidence before convergence.
