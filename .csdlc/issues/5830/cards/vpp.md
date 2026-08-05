# Validation Planning Prompt

Template: 1.0.0

Issue: 5830

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5830/design.md

Diagram: .csdlc/prepared/issues/5830/diagram.mmd

## Selected Lanes

[
  {
    "lane": "cognitive-profile-canonical",
    "proof_role": "Prove canonical profile creation, update linkage, and bounded projections.",
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
      "cognitive_profile",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5830-core",
    "defer_reason": null
  },
  {
    "lane": "cognitive-profile-privacy-negative",
    "proof_role": "Reject stale, forbidden, mismatched, unexplained, or private evidence.",
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
      "cognitive_profile_negative",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5830-negative",
    "defer_reason": null
  },
  {
    "lane": "cognitive-profile-non-reputation",
    "proof_role": "Reject diagnosis, reputation, standing, rights, personhood, and consciousness inference.",
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
      "cognitive_profile_claim_boundary",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5830-negative",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path adl/Cargo.toml cognitive_profile -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml cognitive_profile_negative -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml cognitive_profile_claim_boundary -- --nocapture`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
