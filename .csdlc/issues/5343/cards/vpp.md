# Validation Planning Prompt

Template: 1.0.0

Issue: 5343

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5343/design.md

Diagram: .csdlc/prepared/issues/5343/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove six typed cards, design, protected paths, transaction and rollback invariants, COTS, budgets, and root safety",
    "acceptance_ids": [
      "AC-1",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5343/validate-preparation.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "dependency-merge-gate",
    "proof_role": "Prove #5344 and #5345 live merged landing commits are ancestral to the exact execution revision and the exact #5344 handoff is accepted; receipts are audit-only",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5343/check-dependencies.rb"
    ],
    "parallel_group": "execution-gate",
    "defer_reason": null
  },
  {
    "lane": "transaction-fault-matrix",
    "proof_role": "Execute one fresh-install cutover proof covering identity, compare-and-swap, exact rollback, v1 and v2 execution, and every failure-preservation class",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5343/run-validation-lane.rb",
      "transaction-fault-matrix"
    ],
    "parallel_group": "transaction",
    "defer_reason": null
  },
  {
    "lane": "fresh-install-override",
    "proof_role": "Validate the retained report proves fresh v2 installation, final v2 selection, and retained v1",
    "acceptance_ids": [
      "AC-3",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5343/run-validation-lane.rb",
      "fresh-install-override"
    ],
    "parallel_group": "report-validation",
    "defer_reason": null
  },
  {
    "lane": "rollback-window-evidence",
    "proof_role": "Validate exact prior-byte restoration, post-rollback v1 execution, fourteen-day retention, and no deletion authority",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5343/run-validation-lane.rb",
      "rollback-window-evidence"
    ],
    "parallel_group": "report-validation",
    "defer_reason": null
  },
  {
    "lane": "cutover-budgets",
    "proof_role": "Enforce bounded orchestration, no Runtime v2 edits, no product implementation duplication, and no legacy deletion",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5343/run-validation-lane.rb",
      "cutover-budgets"
    ],
    "parallel_group": "budgets",
    "defer_reason": null
  },
  {
    "lane": "post-merge-exact",
    "proof_role": "Verify merged ancestry and rerun exact report, dependency, identity, rollback-window, budget, and no-deletion checks before WP-12 completion",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5343/run-validation-lane.rb",
      "post-merge-exact"
    ],
    "parallel_group": "post-merge",
    "defer_reason": "Run immediately after authorized merge; it does not block PR publication"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 21600

Tokens: 100000

## Commands

- `ruby .csdlc/prepared/issues/5343/validate-preparation.rb`
- `ruby .csdlc/prepared/issues/5343/check-dependencies.rb`
- `ruby .csdlc/prepared/issues/5343/run-validation-lane.rb transaction-fault-matrix`
- `ruby .csdlc/prepared/issues/5343/run-validation-lane.rb fresh-install-override`
- `ruby .csdlc/prepared/issues/5343/run-validation-lane.rb rollback-window-evidence`
- `ruby .csdlc/prepared/issues/5343/run-validation-lane.rb cutover-budgets`
- `ruby .csdlc/prepared/issues/5343/run-validation-lane.rb post-merge-exact`

## Failure Semantics

Fail closed without selector mutation, publication, merge, closeout, rollback-window start, or WP-13 handoff on an incomplete #5344/#5345 gate, claim collision, unverified fresh install, non-atomic transaction, prior-state damage, explicit-v1 or rollback failure, ambiguous window, hidden network/credential/AWS use, Runtime v2 edit, legacy deletion, secret or host-path disclosure, overclaim, budget violation, deferred acceptance, stale review, red CI, or absent post-merge proof.

## Handoff

Retain typed evidence before convergence.
