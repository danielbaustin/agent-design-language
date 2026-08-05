# Validation Planning Prompt

Template: 1.0.0

Issue: 5832

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5832/design.md

Diagram: .csdlc/prepared/issues/5832/diagram.mmd

## Selected Lanes

[
  {
    "lane": "schema-catalog-json-parity",
    "proof_role": "Validate protobuf schemas, catalog derivation, deterministic JSON rules, golden semantic round trips, and compatibility negotiation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
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
      "contracts",
      "--test",
      "protocol_adapters"
    ],
    "parallel_group": "protocol-contract",
    "defer_reason": null
  },
  {
    "lane": "wss-auth-negative",
    "proof_role": "Prove auth, signed-control, capability, origin, malformed/oversized, replay, wrong-runtime, backpressure, reconnect, and typed-error behavior.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
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
      "adl-runtime/Cargo.toml",
      "--test",
      "runtime_api_wss"
    ],
    "parallel_group": "protocol-contract",
    "defer_reason": null
  },
  {
    "lane": "real-full-duplex-wss",
    "proof_role": "Exchange authenticated protobuf and JSON ACIP/A2A messages bidirectionally over the production Rustls WSS endpoint and retain correlated trace evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-5",
      "AC-6"
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
      "runtime_api_wss",
      "--",
      "--ignored",
      "--exact",
      "real_acip_a2a_full_duplex"
    ],
    "parallel_group": "protocol-live",
    "defer_reason": "Requires completed schema/carrier implementation and a running production Runtime endpoint."
  },
  {
    "lane": "native-protocol-platforms",
    "proof_role": "Repeat schema, negotiation, auth, denial, and real-carrier checks on macOS, Linux, and native Windows.",
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
    "defer_reason": "Requires native platform runners after implementation; absent evidence remains blocked."
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

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test contracts --test protocol_adapters`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test runtime_api_wss`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test runtime_api_wss -- --ignored --exact real_acip_a2a_full_duplex`
- `bash adl/tools/run_owner_validation_lane.sh runtime`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
