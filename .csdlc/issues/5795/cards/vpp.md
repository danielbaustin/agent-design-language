# Validation Planning Prompt

Template: 1.0.0

Issue: 5795

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5795/design.md

Diagram: .csdlc/prepared/issues/5795/diagram.mmd

## Selected Lanes

[
  {
    "lane": "shepherd-adapter-regressions",
    "proof_role": "Prove governed Shepherd admission, authorization, malformed input, timeout/cancellation, and post-failure usability with deterministic tests.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
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
      "assembly"
    ],
    "parallel_group": "runtime",
    "defer_reason": null
  },
  {
    "lane": "real-local-shepherd-smoke",
    "proof_role": "Prove one real configured local MLX/Gemma response through the governed Runtime and Observatory path.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "shepherd_local_model",
      "--",
      "--ignored",
      "--exact",
      "real_local_model_smoke"
    ],
    "parallel_group": "local-model",
    "defer_reason": "Requires the explicitly configured local MLX/Gemma model and completed production-path test target."
  },
  {
    "lane": "exact-head-diff-hygiene",
    "proof_role": "Reject unrelated changes and support exact-head review.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
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

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test assembly`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test shepherd_local_model -- --ignored --exact real_local_model_smoke`
- `git diff --check`

## Failure Semantics

Fail closed on unavailable model, timeout, malformed command, unauthorized mutation, policy bypass, status ambiguity, or fake-only success; keep the Runtime and Observatory usable after failure.

## Handoff

Retain typed evidence before convergence.
