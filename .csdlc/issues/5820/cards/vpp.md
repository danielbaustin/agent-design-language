# Validation Planning Prompt

Template: 1.0.0

Issue: 5820

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5820/design.md

Diagram: .csdlc/prepared/issues/5820/diagram.mmd

## Selected Lanes

[
  {
    "lane": "guardian-contract",
    "proof_role": "Prove init parsing, process ownership, restart/backoff, signal forwarding, bounded capture, and terminal states.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "guardian_cli"
    ],
    "parallel_group": "runtime-unit",
    "defer_reason": null
  },
  {
    "lane": "kernel-configuration-and-state",
    "proof_role": "Prove authoritative configuration, bounded assembly, durable checkpoint/restart, readiness, and dependency degradation.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "configuration",
      "--test",
      "durable_state",
      "--test",
      "kernel"
    ],
    "parallel_group": "runtime-unit",
    "defer_reason": null
  },
  {
    "lane": "guardian-live-lifecycle",
    "proof_role": "Launch the production Guardian/kernel, exercise health/readiness, authenticated HTTPS/WSS, child failure, restart, shutdown, state recovery, and clean logs.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/run_runtime_v3_guardian_soak.sh"
    ],
    "parallel_group": "runtime-live",
    "defer_reason": "Run after implementation with installed owner binaries and issue-local state/log roots."
  },
  {
    "lane": "native-platform-lifecycle",
    "proof_role": "Repeat start-stop, configuration failure, recovery, signal, shutdown, state, and log checks on macOS, Linux, and native Windows.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/run_owner_validation_lane.sh",
      "runtime"
    ],
    "parallel_group": "platform",
    "defer_reason": "Requires native CI runners after implementation; missing platform evidence remains blocked."
  },
  {
    "lane": "exact-head-hygiene",
    "proof_role": "Reject unrelated changes and support exact-head review and closing linkage.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test guardian_cli`
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test configuration --test durable_state --test kernel`
- `bash adl/tools/run_runtime_v3_guardian_soak.sh`
- `bash adl/tools/run_owner_validation_lane.sh runtime`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
