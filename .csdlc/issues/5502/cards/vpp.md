# Validation Planning Prompt

Template: 1.0.0

Issue: 5502

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5502/retained/design.md

Diagram: .csdlc/issues/5502/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove six-card, design, diagram, live dependency rule, scope, COTS, budget, PVF, and no-product-change preparation truth",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5502/validate-preparation.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "dependency-live-merge-ancestry",
    "proof_role": "Fail closed until #5499 and #5498 have live GitHub merged revisions that are ancestors of the #5502 execution base; typed closeout, receipts, and claim release are audit-only",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5502/check-dependencies.rb"
    ],
    "parallel_group": "execution-gate",
    "defer_reason": "Run when implementation is requested; it must fail closed until both dependencies are live merged and ancestral"
  },
  {
    "lane": "convergence-contract",
    "proof_role": "Run offline deterministic identity, overlap, ordering, partial-success, replan, blocked, and non-authority tests",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 120,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5502/run-validation-lane.rb",
      "convergence-contract"
    ],
    "parallel_group": "implementation",
    "defer_reason": "Mandatory after live merged ancestral dependencies and implementation; not selected during preparation"
  },
  {
    "lane": "post-merge-exact",
    "proof_role": "Re-run dependency ancestry, deterministic contracts, budgets, exact revision identity, CI, and #5501 handoff after authorized merge",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5502/run-validation-lane.rb",
      "post-merge-exact"
    ],
    "parallel_group": "post-merge",
    "defer_reason": "Mandatory after authorized merge and before closeout"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/5502/validate-preparation.rb`
- `ruby .csdlc/prepared/issues/5502/check-dependencies.rb`
- `ruby .csdlc/prepared/issues/5502/run-validation-lane.rb convergence-contract`
- `ruby .csdlc/prepared/issues/5502/run-validation-lane.rb post-merge-exact`

## Failure Semantics

Fail closed without implementation, publication, merge, closeout, or #5501 handoff on incomplete dependencies, claim collision, stale/forged/overlapping/out-of-scope output, changed assumptions without typed replan, hidden authority, nondeterminism, deferred proof, red CI, stale review, or budget breach.

## Handoff

Retain typed evidence before convergence.
