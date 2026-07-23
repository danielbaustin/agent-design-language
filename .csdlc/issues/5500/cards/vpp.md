# Validation Planning Prompt

Template: 1.0.0

Issue: 5500

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5500/design.md

Diagram: .csdlc/prepared/issues/5500/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove six-card, design, diagram, dependency, scope, COTS, budget, security, and no-product-change preparation truth",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5500/validate-preparation.rb"
    ],
    "parallel_group": "preparation-local",
    "defer_reason": null
  },
  {
    "lane": "dependency-gate",
    "proof_role": "Fail closed until #5498 and final WP-09 gate #5349 are live-merged on origin/main and ancestral to the #5500 execution base; typed closeout receipts are audit evidence only",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5500/check-dependencies.rb"
    ],
    "parallel_group": "preparation-local",
    "defer_reason": null
  },
  {
    "lane": "dashboard-contract",
    "proof_role": "Run offline deterministic dashboard fixture, security, mobile-layout, syntax, and non-authority tests after dependency admission",
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
      "bash",
      ".csdlc/prepared/issues/5500/validate-dashboard.sh"
    ],
    "parallel_group": "dashboard-local",
    "defer_reason": "Preparation only: do not select until #5498 and #5349 are live-merged on origin/main and ancestral to the #5500 execution base"
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Verify exact issue-branch preparation patch hygiene",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5500/check-preparation-diff.sh"
    ],
    "parallel_group": "preparation-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/5500/validate-preparation.rb`
- `ruby .csdlc/prepared/issues/5500/check-dependencies.rb`
- `bash .csdlc/prepared/issues/5500/validate-dashboard.sh`
- `bash .csdlc/prepared/issues/5500/check-preparation-diff.sh`

## Failure Semantics

Fail closed on absent or stale terminal dependencies, overlapping paths, malformed or oversized input, unsupported schema, unsafe URL/origin, freshness ambiguity, credential exposure, mutation capability, authority confusion, nondeterministic output, or budget breach; blocked preparation never grants product authority.

## Handoff

Retain typed evidence before convergence.
