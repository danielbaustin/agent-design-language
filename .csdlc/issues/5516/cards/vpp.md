# Validation Planning Prompt

Template: 1.0.0

Issue: 5516

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5516/retained/design.md

Diagram: .csdlc/issues/5516/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "terminal-design-repair",
    "proof_role": "Prove typed design and diagram replacement with regenerated digests",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "csdlc-closeout",
      "repair-design",
      "--request",
      ".csdlc/prepared/issues/5516/repair-5494-design.json"
    ],
    "parallel_group": "records",
    "defer_reason": null
  },
  {
    "lane": "terminal-record-doctor",
    "proof_role": "Prove closed-out record and docs remain structurally valid",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "csdlc-doctor",
      "--issue",
      "5494"
    ],
    "parallel_group": "records",
    "defer_reason": null
  },
  {
    "lane": "terminal-review",
    "proof_role": "Prove the exact docs and records diff is architecture-truthful",
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
    "parallel_group": "review",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `csdlc-closeout repair-design --request .csdlc/prepared/issues/5516/repair-5494-design.json`
- `csdlc-doctor --issue 5494`
- `git diff --check`

## Failure Semantics

Fail closed on manual terminal mutation, stale Runtime v2-only claims, digest mismatch, runtime source changes, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
