# Validation Planning Prompt

Template: 1.0.0

Issue: 5648

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5648/design.md

Diagram: .csdlc/prepared/issues/5648/diagram.mmd

## Selected Lanes

[
  {
    "lane": "claim-revoke-focused",
    "proof_role": "typed request, CAS, authorization, audit, and unchanged-phase safety",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "claim_revoke"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml claim_revoke`

## Failure Semantics

Fail closed and preserve the canonical record.

## Handoff

Retain typed evidence before convergence.
