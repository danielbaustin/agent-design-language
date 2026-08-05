# Validation Planning Prompt

Template: 1.0.0

Issue: 5821

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5821/design.md

Diagram: .csdlc/prepared/issues/5821/diagram.mmd

## Selected Lanes

[
  {
    "lane": "live-child-wave-ledger",
    "proof_role": "Validate #5862 plus exactly #5863-#5878, complete owner/dependency/path/proof/rollback fields, acyclic dependencies, exclusive paths, approved designs, and null claims.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5821/validate-child-wave.rb"
    ],
    "parallel_group": "planning-gate",
    "defer_reason": "Requires live GitHub read access through the typed v2 issue owner binary."
  },
  {
    "lane": "architecture-security-review",
    "proof_role": "Validate required architecture/threat coverage and an independent accepted exact-packet review with recomputed artifact digests. [preexec_rejection exit=1 diagnostic_sha256=bab4c559fb9e19d99506c5e93a5ce3aa7a2610b934cd40213ab52d93467c6f87]",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5821/validate-architecture-security-review.rb"
    ],
    "parallel_group": "planning-gate",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5821/validate-child-wave.rb`
- `ruby .csdlc/prepared/issues/5821/validate-architecture-security-review.rb`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
