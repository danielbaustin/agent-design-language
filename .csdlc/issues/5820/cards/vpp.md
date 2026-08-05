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
    "lane": "guardian-lifecycle-contract",
    "proof_role": "Run the exact nonzero production Guardian lifecycle target over init, supervision, restart, durable state, degradation, shutdown, and logs.",
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
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "runtime_guardian_lifecycle",
      "--no-tests=fail"
    ],
    "parallel_group": "runtime",
    "defer_reason": null
  },
  {
    "lane": "production-guardian-api-wss-restart",
    "proof_role": "Launch the production Guardian/kernel and prove authenticated HTTPS/WSS, child kill, bounded restart, durable state, readiness, clean shutdown, and clean logs.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "bash",
      "adl/tools/validate_v092_runtime_guardian_lifecycle.sh"
    ],
    "parallel_group": "runtime",
    "defer_reason": null
  },
  {
    "lane": "native-guardian-receipts",
    "proof_role": "Recompute digest-bound macOS, Linux, and native Windows production Guardian lifecycle receipts.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5820/validate-runtime-native-receipts.rb"
    ],
    "parallel_group": "runtime",
    "defer_reason": null
  },
  {
    "lane": "exact-head-review-preflight",
    "proof_role": "Reject diff damage before exact-head review and issue-closing publication.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "review",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo nextest run --manifest-path adl-runtime/Cargo.toml --test runtime_guardian_lifecycle --no-tests=fail`
- `bash adl/tools/validate_v092_runtime_guardian_lifecycle.sh`
- `ruby .csdlc/prepared/issues/5820/validate-runtime-native-receipts.rb`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
