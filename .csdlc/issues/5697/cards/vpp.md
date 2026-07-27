# Validation Planning Prompt

Template: 1.0.0

Issue: 5697

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5697/design.md

Diagram: .csdlc/prepared/issues/5697/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-chronosense-assembly",
    "proof_role": "Prove Chronosense trusted_time wiring, fail-closed behavior, and startup order",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "assembly"
    ],
    "parallel_group": "runtime-v3-chronosense",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-chronosense-governed-operations",
    "proof_role": "Prove governed operation call sites share the live RuntimeRecorder-backed trusted time",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "governed_operations"
    ],
    "parallel_group": "runtime-v3-chronosense",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-chronosense-clippy",
    "proof_role": "Prove strict lint cleanliness for the touched Runtime v3 kernel crate",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "runtime-v3-chronosense",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-chronosense-exact-review",
    "proof_role": "Record one exact-head review before ready publication",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "csdlc-review",
      "record"
    ],
    "parallel_group": "runtime-v3-chronosense",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-chronosense-ready-publication",
    "proof_role": "Publish one ready PR whose body closes issue #5697",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1000,
    "argv": [
      "csdlc-publish",
      "publish"
    ],
    "parallel_group": "runtime-v3-chronosense",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test assembly`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test governed_operations`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`
- `csdlc-review record`
- `csdlc-publish publish`

## Failure Semantics

Fail closed on claim collision, stale generation, #5663 lifecycle mutation, skipped proof, or stale/missing exact-head review.

## Handoff

Retain typed evidence before convergence.
