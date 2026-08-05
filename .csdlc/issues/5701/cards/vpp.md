# Validation Planning Prompt

Template: 1.0.0

Issue: 5701

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5701/retained/design.md

Diagram: .csdlc/issues/5701/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-openapi-contract",
    "proof_role": "Parse OpenAPI 3.1 artifacts, resolve references, prove route parity against Runtime v3 route inventory, and validate served discovery endpoints when router integration is authorized",
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
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "openapi_contract"
    ],
    "parallel_group": "runtime-v3-openapi",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-openapi-clippy",
    "proof_role": "Prove touched Runtime v3 test/code surface remains strict-lint clean",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "runtime-v3-openapi",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-openapi-exact-review",
    "proof_role": "Run one exact-head subagent review immediately before ready publication",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 6000,
    "argv": [
      "csdlc-review",
      "record"
    ],
    "parallel_group": "runtime-v3-openapi",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-openapi-publication",
    "proof_role": "Publish ready PR with Closes #5701",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1000,
    "argv": [
      "csdlc-publish",
      "publish"
    ],
    "parallel_group": "runtime-v3-openapi",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract`
- `cargo clippy --locked --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`
- `csdlc-review record`
- `csdlc-publish publish`

## Failure Semantics

Fail closed on claim collision, route phantom claims, stale exact review, unsupported API claims, or any #5344 protected-path edit without typed transfer/release.

## Handoff

Retain typed evidence before convergence.
