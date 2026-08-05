# Validation Planning Prompt

Template: 1.0.0

Issue: 5842

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5842/design.md

Diagram: .csdlc/prepared/issues/5842/diagram.mmd

## Selected Lanes

[
  {
    "lane": "complete-feature-matrix",
    "proof_role": "Require exactly the 13 indexed v0.92 feature documents and nonempty exact implementation, review, merge, positive, negative, integration, platform, and terminal evidence for every accepted row.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5842/validate-quality-gate.rb",
      "matrix"
    ],
    "parallel_group": "gate",
    "defer_reason": null
  },
  {
    "lane": "exercised-prohibited-evidence",
    "proof_role": "Execute the quality gate against all eight prohibited evidence classes and require each forged case to fail with digest-bound observed output.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5842/validate-quality-gate.rb",
      "negative"
    ],
    "parallel_group": "negative",
    "defer_reason": null
  },
  {
    "lane": "gate-packet-hygiene",
    "proof_role": "Validate tracked gate packet whitespace after the proving matrix and negative validators run.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
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
    "parallel_group": "hygiene",
    "defer_reason": null
  },
  {
    "lane": "typed-card-doctor",
    "proof_role": "Validate the exact rendered six-card bundle, cross-card references, digests, statuses, and canonical issue record.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5842"
    ],
    "parallel_group": "typed-readback",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/5842/validate-quality-gate.rb matrix`
- `ruby .csdlc/prepared/issues/5842/validate-quality-gate.rb negative`
- `git diff --check`
- `csdlc-doctor --repo . --issue 5842`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
