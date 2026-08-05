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
    "proof_role": "Prove signed/capability admission, message bounds, deterministic adapter behavior, status classification, timeout, cancellation, and redaction.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
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
    "budget_seconds": 600,
    "budget_tokens": 4000,
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
    "proof_role": "Invoke the explicitly configured Apple Metal/MLX Gemma model through the production adapter and retain correlated redacted response evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
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
    "defer_reason": "Requires the completed adapter and explicitly configured local MLX/Gemma model; missing availability is blocked or deferred, not passed."
  },
  {
    "lane": "observatory-live-roundtrip",
    "proof_role": "Prove the separate browser client sends a governed message and renders the real correlated result without gaining signing/provider authority.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/test_v0917_html_observatory_integrated_proof.sh"
    ],
    "parallel_group": "local-model",
    "defer_reason": "Requires trusted HTTPS plus the running Runtime and configured local model."
  },
  {
    "lane": "exact-head-hygiene",
    "proof_role": "Reject unrelated changes and support exact-head review.",
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

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test governed_operations --test protocol_adapters`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test runtime_api_wss`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test shepherd_local_model -- --ignored --exact real_local_model_smoke`
- `bash adl/tools/test_v0917_html_observatory_integrated_proof.sh`
- `git diff --check`

## Failure Semantics

Fail closed on unavailable model, timeout, malformed command, unauthorized mutation, policy bypass, status ambiguity, or fake-only success; keep the Runtime and Observatory usable after failure.

## Handoff

Retain typed evidence before convergence.
