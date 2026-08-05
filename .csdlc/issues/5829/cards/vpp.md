# Validation Planning Prompt

Template: 1.0.0

Issue: 5829

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5829/design.md

Diagram: .csdlc/prepared/issues/5829/diagram.mmd

## Selected Lanes

[
  {
    "lane": "capability-envelope-canonical",
    "proof_role": "Prove complete canonical envelopes bound to identity and exact evidence revision.",
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
      "capability_envelope",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5829-core",
    "defer_reason": null
  },
  {
    "lane": "capability-authority-and-secret-negative",
    "proof_role": "Reject stale provenance, undeclared capability, escalation, missing limits, and secret-like content.",
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
      "capability_envelope_negative",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5829-negative",
    "defer_reason": null
  },
  {
    "lane": "capability-path-portability",
    "proof_role": "Prove envelope evidence remains credential-free and repo-relative.",
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
      "capability_envelope_portability",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5829-negative",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path adl/Cargo.toml capability_envelope -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml capability_envelope_negative -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml capability_envelope_portability -- --nocapture`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
