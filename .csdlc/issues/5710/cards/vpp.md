# Validation Planning Prompt

Template: 1.0.0

Issue: 5710

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5710/retained/design.md

Diagram: .csdlc/issues/5710/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-v2-closeout-recovery",
    "proof_role": "Prove metadata-only terminal head reconciliation, substantive-drift rejection, safe prune classification, unknown-path rejection, and existing closeout lifecycle behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate7_lifecycle"
    ],
    "parallel_group": "csdlc-v2-closeout",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate7_lifecycle`

## Failure Semantics

Fail closed on stale claim/generation, ambiguous terminal identity, unclassified dirty paths, unretained evidence, failed focused proof, or stale/missing exact-head review.

## Handoff

Retain typed evidence before convergence.
