# Validation Planning Prompt

Template: 1.0.0

Issue: 5833

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5833/design.md

Diagram: .csdlc/prepared/issues/5833/diagram.mmd

## Selected Lanes

[
  {
    "lane": "birth-witness-and-receipt",
    "proof_role": "Prove policy-complete exact-candidate witnesses and deterministic accepted/rejected receipts.",
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
      "birth_witness_receipt",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5833-core",
    "defer_reason": null
  },
  {
    "lane": "witness-equivocation-security-negative",
    "proof_role": "Reject duplicate, stale, forged, equivocal, unauthorized, and mismatched witnesses.",
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
      "birth_witness_receipt_negative",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5833-negative",
    "defer_reason": null
  },
  {
    "lane": "receipt-privacy-and-claim-boundary",
    "proof_role": "Reject raw-state leakage and premature birth, citizenship, legal, or governance claims.",
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
      "birth_receipt_claim_boundary",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5833-negative",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path adl/Cargo.toml birth_witness_receipt -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml birth_witness_receipt_negative -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml birth_receipt_claim_boundary -- --nocapture`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
