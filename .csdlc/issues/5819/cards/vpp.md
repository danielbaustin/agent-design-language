# Validation Planning Prompt

Template: 1.0.0

Issue: 5819

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5819/design.md

Diagram: .csdlc/prepared/issues/5819/diagram.mmd

## Selected Lanes

[
  {
    "lane": "migration-evidence-contract",
    "proof_role": "Validate the exact ordered five-repository destination set, per-destination receipts, zero unexplained drift, both untouched negative controls, and the two-file four-link website cutover.",
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
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5819/validate-migration-evidence.rb"
    ],
    "parallel_group": "migration-contract",
    "defer_reason": null
  },
  {
    "lane": "github-five-destination-live-proof",
    "proof_role": "Run the retained execution script that queries and records all five live agent-logic destinations, never only ADL.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5819/verify-live-repositories.rb"
    ],
    "parallel_group": "github-live",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors in tracked integration and evidence changes.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/5819/validate-migration-evidence.rb`
- `ruby .csdlc/prepared/issues/5819/verify-live-repositories.rb`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
