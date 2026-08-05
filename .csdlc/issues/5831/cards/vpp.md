# Validation Planning Prompt

Template: 1.0.0

Issue: 5831

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5831/design.md

Diagram: .csdlc/prepared/issues/5831/diagram.mmd

## Selected Lanes

[
  {
    "lane": "adaptive-learning-accepted-rejected",
    "proof_role": "Prove accepted and rejected policy paths with durable linked history.",
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
      "adaptive_learning_dag",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5831-core",
    "defer_reason": null
  },
  {
    "lane": "adaptive-learning-replay-negative",
    "proof_role": "Reject forged history, substituted state, discontinuous resume, unauthorized mutation, and rollback mismatch.",
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
      "adaptive_learning_dag_negative",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5831-negative",
    "defer_reason": null
  },
  {
    "lane": "adaptive-learning-runtime-v3",
    "proof_role": "Prove branch-built Runtime v3 bounds, cancellation, replay, and integration.",
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
      "adaptive_learning_runtime_v3",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5831-negative",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path adl/Cargo.toml adaptive_learning_dag -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml adaptive_learning_dag_negative -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml adaptive_learning_runtime_v3 -- --nocapture`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
