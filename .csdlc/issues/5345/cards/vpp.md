# Validation Planning Prompt

Template: 1.0.0

Issue: 5345

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5345/retained/design.md

Diagram: .csdlc/issues/5345/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove six typed cards, reviewed design/diagram, exact dependency and protected-path gates, selector/rollback invariants, COTS, budgets, PVF, no-deferral, and root safety",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5345/validate-preparation.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "cli-focused",
    "proof_role": "Prove all command success, malformed input, upstream error, stable JSON stdout/stderr separation, exit codes, and delegation boundaries",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5345/validate-cli.sh",
      "focused"
    ],
    "parallel_group": "cli-local",
    "defer_reason": "Execute only after every upstream dependency gate is terminal and product implementation exists"
  },
  {
    "lane": "cli-quality",
    "proof_role": "Prove formatting, strict Clippy, forbidden source/dependency absence, and thin module boundaries",
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
      "bash",
      ".csdlc/prepared/issues/5345/validate-cli.sh",
      "quality"
    ],
    "parallel_group": "cli-local",
    "defer_reason": "Execute only after every upstream dependency gate is terminal and product implementation exists"
  },
  {
    "lane": "selector-installer",
    "proof_role": "Prove exact install identity, idempotence, atomic selection, stale/concurrent/interrupted failure preservation, explicit verified rollback, and deterministic receipts",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 7000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5345/validate-cli.sh",
      "install-selector"
    ],
    "parallel_group": "selector",
    "defer_reason": "Execute only after every upstream dependency gate is terminal and product implementation exists"
  },
  {
    "lane": "cli-budgets",
    "proof_role": "Enforce receipt/ancestry gate, exact COTS and forbidden dependencies, implementation/test/module LoC, test count, complete offline proof, clean exact revision, and 600-second ceiling",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5345/validate-cli.sh",
      "budgets"
    ],
    "parallel_group": "cli-budget",
    "defer_reason": "Execute at the exact clean implementation revision with ADL_WP10_EXPECTED_HEAD and measured dependency, LoC, test-count, module-growth, and duration evidence"
  },
  {
    "lane": "post-merge-exact",
    "proof_role": "Validate the exact integrated tree, stable installer, selector and rollback transaction, dependency/LoC/test/time budgets, exact revision identity, and retained green required CI checks before typed closeout",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 10000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5345/validate-cli.sh",
      "post-merge"
    ],
    "parallel_group": "post-merge",
    "defer_reason": "Execute only after authorized merge with ADL_WP10_EXPECTED_HEAD and retained ADL_WP10_CI_EVIDENCE; mandatory before closeout"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5345/validate-preparation.rb`
- `bash .csdlc/prepared/issues/5345/validate-cli.sh focused`
- `bash .csdlc/prepared/issues/5345/validate-cli.sh quality`
- `bash .csdlc/prepared/issues/5345/validate-cli.sh install-selector`
- `bash .csdlc/prepared/issues/5345/validate-cli.sh budgets`
- `bash .csdlc/prepared/issues/5345/validate-cli.sh post-merge`

## Failure Semantics

Fail closed without implementation, publication, merge, or closeout on an incomplete dependency gate, duplicate command authority, implicit network or selector behavior, invalid installation identity, non-atomic selector mutation, prior-state damage, unverified rollback, host-path or secret disclosure, forbidden dependency, unsupported LoC/module/test/time variance, deferred acceptance, stale review, red CI, or absent post-merge proof.

## Handoff

Retain typed evidence before convergence.
