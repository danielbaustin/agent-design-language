# Validation Planning Prompt

Template: 1.0.0

Issue: 5827

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5827/design.md

Diagram: .csdlc/prepared/issues/5827/diagram.mmd

## Selected Lanes

[
  {
    "lane": "continuity-chain-replay",
    "proof_role": "Prove a canonical two-or-more-cycle chain and deterministic head replay.",
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
      "continuity_record",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5827-core",
    "defer_reason": null
  },
  {
    "lane": "continuity-discontinuity-negative",
    "proof_role": "Reject substitutions, gaps, duplicates, reorderings, forged witnesses, and copied state.",
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
      "continuity_record_negative",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5827-negative",
    "defer_reason": null
  },
  {
    "lane": "continuity-portability",
    "proof_role": "Prove continuity fixtures and evidence references remain private-safe and repo-relative.",
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
      "continuity_record_portability",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5827-negative",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path adl/Cargo.toml continuity_record -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml continuity_record_negative -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml continuity_record_portability -- --nocapture`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
