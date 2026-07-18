# Validation Planning Prompt

Template: 1.0.0

Issue: 5440

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: docs/reviews/v0.91.7/csdlc-v2-5440/DESIGN.md

Diagram: docs/reviews/v0.91.7/csdlc-v2-5440/DIAGRAM.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-v2-design-reapproval",
    "proof_role": "Prove phase authorization, digest refresh, audit history, and fail-closed later phases",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "parallel_group": "csdlc-v2",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate2`

## Failure Semantics

Fail closed on stale claim, stale generation, stale digest, unsupported phase, or card drift.

## Handoff

Retain typed evidence before convergence.
