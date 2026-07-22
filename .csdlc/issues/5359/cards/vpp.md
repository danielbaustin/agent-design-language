# Validation Planning Prompt

Template: 1.0.0

Issue: 5359

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5359/design.md

Diagram: .csdlc/prepared/issues/5359/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-doctor",
    "proof_role": "Validate the generated typed C-SDLC v2 preparation packet",
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
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5359"
    ],
    "parallel_group": "release-tail-preparation",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5359`

## Failure Semantics

Fail closed on missing live predecessor merge, stale ancestry, missing review inputs, or unsupported v0.92 claims.

## Handoff

Retain typed evidence before convergence.
