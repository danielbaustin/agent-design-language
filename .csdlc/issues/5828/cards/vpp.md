# Validation Planning Prompt

Template: 1.0.0

Issue: 5828

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5828/design.md

Diagram: .csdlc/prepared/issues/5828/diagram.mmd

## Selected Lanes

[
  {
    "lane": "memory-palace-topology",
    "proof_role": "Prove identity/continuity-bound topology, bounded selection, and overflow integration.",
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
      "memory_palace",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5828-core",
    "defer_reason": null
  },
  {
    "lane": "memory-palace-negative-replay",
    "proof_role": "Reject stale, hash-mismatched, discontinuous, unauthorized, or nondeterministically ordered context.",
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
      "memory_palace_negative",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5828-negative",
    "defer_reason": null
  },
  {
    "lane": "memory-palace-platform-portability",
    "proof_role": "Prove relative-path fixtures and equivalent output across supported platforms.",
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
      "memory_palace_portability",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5828-negative",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path adl/Cargo.toml memory_palace -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml memory_palace_negative -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml memory_palace_portability -- --nocapture`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
