# Validation Planning Prompt

Template: 1.0.0

Issue: 5498

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5498/retained/design.md

Diagram: .csdlc/issues/5498/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "[release gate: required] Prove six-card, design, diagram, dependency, scope, COTS, budget, privacy, and no-product-change preparation truth",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5498/validate-preparation.rb"
    ],
    "parallel_group": "local-control",
    "defer_reason": null
  },
  {
    "lane": "dependency-gate",
    "proof_role": "[release gate: optional during preparation; required before implementation] Fail closed until #5499 and final WP-09 gate #5349 are live-merged into origin/main, dependency revisions are ancestral to the execution base, and adjacent path owners confirm disjoint reservations; retained closeout receipts are audit-only",
    "acceptance_ids": [
      "AC-2",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5498/check-dependencies.rb"
    ],
    "parallel_group": "local-control",
    "defer_reason": null
  },
  {
    "lane": "task-adapter-contract",
    "proof_role": "[release gate: required before publication] Run focused all-target task-adapter tests and strict Clippy from FastWork after dependency gates open",
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
    "resource_profile": "medium",
    "budget_seconds": 180,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5498/validate-task-adapter.sh"
    ],
    "parallel_group": "task-adapter-local",
    "defer_reason": "Preparation only: do not select until #5499 and #5349 are live-merged into origin/main, dependency revisions are ancestral to the execution base, adjacent path owners confirm disjoint reservations, and the product manifest exists"
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "[release gate: required] Verify the exact committed preparation patch contains only issue-local lifecycle paths",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5498/check-preparation-diff.sh"
    ],
    "parallel_group": "local-control",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/5498/validate-preparation.rb`
- `ruby .csdlc/prepared/issues/5498/check-dependencies.rb`
- `bash .csdlc/prepared/issues/5498/validate-task-adapter.sh`
- `bash .csdlc/prepared/issues/5498/check-preparation-diff.sh`

## Failure Semantics

Fail closed on missing or stale lifecycle input, unresolved dependencies, task or owner collision, ambiguous authority, transcript leakage, nondeterministic output, absent retained receipts, or budget breach; blocked preparation never becomes product authority.

## Handoff

Retain typed evidence before convergence.
