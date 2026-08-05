# Validation Planning Prompt

Template: 1.0.0

Issue: 5825

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5825/design.md

Diagram: .csdlc/prepared/issues/5825/diagram.mmd

## Selected Lanes

[
  {
    "lane": "birthday-contract-and-fixtures",
    "proof_role": "Prove one valid birth packet plus deterministic canonical decision output.",
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
    "budget_seconds": 500,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "birthday_contract",
      "--",
      "--nocapture"
    ],
    "parallel_group": "birthday-core",
    "defer_reason": null
  },
  {
    "lane": "not-a-birthday-negative-matrix",
    "proof_role": "Reject lifecycle lookalikes and every required-evidence omission with stable reasons.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 500,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "birthday_negative",
      "--",
      "--nocapture"
    ],
    "parallel_group": "birthday-negative",
    "defer_reason": null
  },
  {
    "lane": "claim-and-path-boundary",
    "proof_role": "Reject private or host paths and unsupported public claims.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 200,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "birthday_claim_boundary",
      "--",
      "--nocapture"
    ],
    "parallel_group": "birthday-negative",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path adl/Cargo.toml birthday_contract -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml birthday_negative -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml birthday_claim_boundary -- --nocapture`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
