# Validation Planning Prompt

Template: 1.0.0

Issue: 5826

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5826/design.md

Diagram: .csdlc/prepared/issues/5826/diagram.mmd

## Selected Lanes

[
  {
    "lane": "identity-record-canonical",
    "proof_role": "Prove canonical valid identity records and deterministic root and alias ordering.",
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
      "identity_record",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5826-core",
    "defer_reason": null
  },
  {
    "lane": "identity-negative-and-privacy",
    "proof_role": "Reject root ambiguity, alias collision, provenance mismatch, substitution, and private disclosure.",
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
      "identity_record_negative",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5826-negative",
    "defer_reason": null
  },
  {
    "lane": "identity-path-portability",
    "proof_role": "Prove retained identity references are redaction-safe and repo-relative.",
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
      "identity_record_portability",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5826-negative",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path adl/Cargo.toml identity_record -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml identity_record_negative -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml identity_record_portability -- --nocapture`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
