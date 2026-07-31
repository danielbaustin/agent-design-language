# Validation Planning Prompt

Template: 1.0.0

Issue: 5691

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5691/design.md

Diagram: .csdlc/prepared/issues/5691/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-observability-focused",
    "proof_role": "Rust unit/integration proof for tracing, Vector config, status, redaction, drain, and auditor",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "target",
      "--test",
      "observability"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-clippy",
    "proof_role": "strict Rust lint proof for touched Runtime v3 crate",
    "acceptance_ids": [
      "AC-1",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "target",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "pinned-vector-proof",
    "proof_role": "real repo-pinned Vector config validation and local output/OTLP exchange proof",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 6000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/vector",
      "validate",
      "--config-yaml",
      "adl-runtime-kernel/vector/runtime-v3.yaml"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --target-dir target --test observability`
- `cargo clippy --locked --manifest-path adl-runtime-kernel/Cargo.toml --target-dir target --all-targets -- -D warnings`
- `/Users/daniel/git/agent-design-language/.adl/bin/vector validate --config-yaml adl-runtime-kernel/vector/runtime-v3.yaml`

## Failure Semantics

Fail closed on missing Vector, invalid config, remote/export-health overclaim, unredacted secret, incomplete drain, malformed log, or exact-head review finding.

## Handoff

Retain typed evidence before convergence.
