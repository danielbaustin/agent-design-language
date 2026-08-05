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
    "lane": "acip-schema-roundtrip-negatives",
    "proof_role": "Run exact nonzero ACIP schema, catalog, protobuf/JSON round-trip, negotiation, replay, malformed, oversized, and denied-dispatch tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
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
      "runtime_api_wss",
      "--no-tests=fail"
    ],
    "parallel_group": "runtime",
    "defer_reason": null
  },
  {
    "lane": "production-acip-wss",
    "proof_role": "Launch production Guardian/kernel and prove real authenticated Rustls WSS binary/JSON full-duplex exchange, correlation, backpressure, reconnect, and typed errors.",
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
      "adl/tools/validate_v092_acip_wss.sh"
    ],
    "parallel_group": "runtime",
    "defer_reason": null
  },
  {
    "lane": "native-acip-receipts",
    "proof_role": "Recompute exact-revision macOS, Linux, and native Windows ACIP/WSS receipts with binary/schema/transcript digests and nonzero exchanges/negatives. [preexec_rejection exit=1 diagnostic_sha256=de72b4d18c37dc62c7f280fdab91fc7844cfa5030f3b65582551714534d8e2dc]",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5832/validate-acip-native-receipts.rb"
    ],
    "parallel_group": "runtime",
    "defer_reason": null
  },
  {
    "lane": "exact-head-review-preflight",
    "proof_role": "Reject diff damage before exact-head review.",
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

- `cargo nextest run --manifest-path adl-runtime/Cargo.toml --test runtime_api_wss --no-tests=fail`
- `bash adl/tools/validate_v092_acip_wss.sh`
- `ruby .csdlc/prepared/issues/5832/validate-acip-native-receipts.rb`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
