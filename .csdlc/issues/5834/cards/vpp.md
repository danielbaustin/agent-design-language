# Validation Planning Prompt

Template: 1.0.0

Issue: 5834

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5834/design.md

Diagram: .csdlc/prepared/issues/5834/diagram.mmd

## Selected Lanes

[
  {
    "lane": "birthday-review-packet-completeness",
    "proof_role": "Prove schema, links, digests, uniqueness, and complete WP-08 through WP-15 inventory.",
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
      "birthday_review_packet",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5834-core",
    "defer_reason": null
  },
  {
    "lane": "birthday-review-packet-negative",
    "proof_role": "Reject missing, stale, contradictory, nonterminal, unreviewed, and duplicate authority inputs.",
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
      "birthday_review_packet_negative",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5834-negative",
    "defer_reason": null
  },
  {
    "lane": "birthday-review-claim-and-path-boundary",
    "proof_role": "Reject private paths and personhood, citizenship, consciousness, governance, release, or publication overclaims.",
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
      "birthday_review_packet_claim_boundary",
      "--",
      "--nocapture"
    ],
    "parallel_group": "5834-negative",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path adl/Cargo.toml birthday_review_packet -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml birthday_review_packet_negative -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml birthday_review_packet_claim_boundary -- --nocapture`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
