# Validation Planning Prompt

Template: 1.0.0

Issue: 5664

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5664/design.md

Diagram: .csdlc/prepared/issues/5664/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-protocol-black-box",
    "proof_role": "Run deterministic local black-box protocol adapter tests",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
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
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "protocol_adapters"
    ],
    "parallel_group": "runtime-v3-protocol",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-protocol-clippy",
    "proof_role": "Run strict Clippy for Runtime v3 protocol adapter changes",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--all-features",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "runtime-v3-protocol",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-protocol-loc",
    "proof_role": "Measure before/after physical LoC and net reduction",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/report_runtime_v3_loc.sh"
    ],
    "parallel_group": "runtime-v3-protocol",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Verify patch whitespace and path hygiene",
    "acceptance_ids": [
      "AC-6"
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
    "parallel_group": "local-control",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test protocol_adapters`
- `cargo clippy --locked --manifest-path adl-runtime-kernel/Cargo.toml --all-targets --all-features -- -D warnings`
- `bash adl/tools/report_runtime_v3_loc.sh`
- `git diff --check`

## Failure Semantics

Fail closed on claim collision, missing #5659 ancestry, degraded or receipt-only production adapter behavior, plaintext credential artifacts, unbounded retry/timeout/cancellation/shutdown, replay acceptance, AWS use, protected-path overlap, failing focused validation, or unresolved exact-review findings.

## Handoff

Retain typed evidence before convergence.
