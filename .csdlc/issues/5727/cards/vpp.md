# Validation Planning Prompt

Template: 1.0.0

Issue: 5727

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5727/retained/design.md

Diagram: .csdlc/issues/5727/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "claim_reacquisition",
    "proof_role": "Focused Rust lifecycle proof for typed claim reacquisition and dormant doctor truth",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2",
      "claim"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "issue_5354_reproduction",
    "proof_role": "Prove the prepared #5354 record can reacquire through the typed binary and return to doctor PASS",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate7_lifecycle",
      "reacquire"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate2 claim`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate7_lifecycle reacquire`

## Failure Semantics

Fail closed on stale generation or digest, invalid claim identity, non-resumable phase, live overlap, missing audit preservation, write authorization regression, or failed #5354 reproduction.

## Handoff

Retain typed evidence before convergence.
