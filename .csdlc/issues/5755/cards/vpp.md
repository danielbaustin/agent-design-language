# Validation Planning Prompt

Template: 1.0.0

Issue: 5755

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5755/design.md

Diagram: .csdlc/prepared/issues/5755/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-protocol-adapters",
    "proof_role": "Prove protocol adapter authenticated TLS/equivalent negative and positive paths.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "protocol_adapters"
    ],
    "parallel_group": "runtime-v3-security",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-control-body-limit",
    "proof_role": "Prove oversized Runtime control requests are rejected at the route boundary.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "control"
    ],
    "parallel_group": "runtime-v3-security",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-security-diff-check",
    "proof_role": "Prove textual diff hygiene for Runtime v3 security repair.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "runtime-v3-security",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-security-exact-review",
    "proof_role": "Record exact-head review for #5755 before publication and #5664 closeout consumption.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-review",
      "record"
    ],
    "parallel_group": "runtime-v3-security",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test protocol_adapters`
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test control`
- `git diff --check`
- `.adl/bin/csdlc-v2/csdlc-review record`

## Failure Semantics

Fail closed if either security blocker remains unproven or exact-head review is stale.

## Handoff

Retain typed evidence before convergence.
