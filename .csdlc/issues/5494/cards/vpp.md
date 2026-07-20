# Validation Planning Prompt

Template: 1.0.0

Issue: 5494

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5494/retained/design.md

Diagram: .csdlc/issues/5494/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v2-focused",
    "proof_role": "Prove supervision, topology, soak, and authentication behavior",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml"
    ],
    "parallel_group": "local-runtime",
    "defer_reason": null
  },
  {
    "lane": "csm-api-focused",
    "proof_role": "Prove the integrated Runtime v2 readiness consumer",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "csm_runtime_api"
    ],
    "parallel_group": "local-api",
    "defer_reason": null
  },
  {
    "lane": "wp07a-closeout-truth",
    "proof_role": "Prove the retained #5409 repair packet and canonical register disposition match reviewed implementation truth",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local-docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path adl-runtime/Cargo.toml`
- `cargo test --manifest-path adl/Cargo.toml csm_runtime_api`
- `git diff --check`

## Failure Semantics

Fail closed on static-only topology, missing observed health, non-behavioral soak proof, unbounded overlap, weakened revocation, or scope expansion.

## Handoff

Retain typed evidence before convergence.
