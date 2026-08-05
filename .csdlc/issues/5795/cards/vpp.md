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
    "lane": "shepherd-governed-contract",
    "proof_role": "Prove signed and capability admission, bounds, deterministic adapter behavior, status classification, timeout, cancellation, and redaction.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "governed_operations",
      "--test",
      "protocol_adapters"
    ],
    "parallel_group": "shepherd-contract",
    "defer_reason": null
  },
  {
    "lane": "runtime-wss-negative",
    "proof_role": "Prove malformed, unauthorized, wrong-runtime, timeout, and post-failure read-stream behavior over Runtime API/WSS.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "runtime_api_wss"
    ],
    "parallel_group": "shepherd-contract",
    "defer_reason": null
  },
  {
    "lane": "real-local-model-smoke",
    "proof_role": "Invoke the explicitly configured Apple Metal/MLX Gemma model through the production adapter and retain correlated redacted Runtime and adapter evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
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
    "defer_reason": "Requires the completed production adapter and explicitly configured local MLX/Gemma model; absence is blocked, not passed."
  },
  {
    "lane": "real-shepherd-browser-roundtrip",
    "proof_role": "Use the issue-delivered Playwright validator to submit a unique governed message from real Chrome, prove Runtime invokes the configured MLX/Gemma adapter, and render the same non-retained real_local_model correlation in the Observatory.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 12000,
    "argv": [
      "node",
      "adl/tools/validate_v092_shepherd_browser_roundtrip.mjs",
      "--browser",
      "chrome",
      "--require-real-local-model",
      "--require-governed-ingress",
      "--require-correlated-browser-result"
    ],
    "parallel_group": "local-model",
    "defer_reason": "The named validator is an issue 5795 implementation deliverable and requires trusted HTTPS, Runtime, Observatory, and the configured real model."
  },
  {
    "lane": "exact-head-hygiene",
    "proof_role": "Reject unrelated changes and support exact-head review.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
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

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test governed_operations --test protocol_adapters`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test runtime_api_wss`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test shepherd_local_model -- --ignored --exact real_local_model_smoke`
- `node adl/tools/validate_v092_shepherd_browser_roundtrip.mjs --browser chrome --require-real-local-model --require-governed-ingress --require-correlated-browser-result`
- `git diff --check`

## Failure Semantics

Fail closed on unavailable model, timeout, malformed command, unauthorized mutation, policy bypass, status ambiguity, or fake-only success; keep the Runtime and Observatory usable after failure.

## Handoff

Retain typed evidence before convergence.
