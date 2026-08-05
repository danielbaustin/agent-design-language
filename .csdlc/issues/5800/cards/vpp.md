# Validation Planning Prompt

Template: 1.0.0

Issue: 5800

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5800/design.md

Diagram: .csdlc/prepared/issues/5800/diagram.mmd

## Selected Lanes

[
  {
    "lane": "local-tls-contract",
    "proof_role": "Prove generation, SAN, expiry, Rustls pair, permissions, atomic replacement, and last-valid-pair negatives.",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-6"
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
      "local_tls"
    ],
    "parallel_group": "tls-contract",
    "defer_reason": null
  },
  {
    "lane": "observatory-https-integration",
    "proof_role": "Prove the configured separate Observatory and Runtime endpoints through verified curl, HTML, health, readiness, and feed access.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/test_v0917_html_observatory_integrated_proof.sh"
    ],
    "parallel_group": "local-live",
    "defer_reason": "Run after the implementation session starts the real HTTPS listeners with the supported certificate identity."
  },
  {
    "lane": "trusted-browser-macos",
    "proof_role": "Retain Chrome trust and no-warning evidence for the actual localhost Observatory and Runtime feed.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "curl",
      "--config",
      ".csdlc/prepared/issues/5800/curl-observatory-https.conf"
    ],
    "parallel_group": "local-live",
    "defer_reason": "Requires explicit operator trust installation and browser-visible live endpoints on macOS."
  },
  {
    "lane": "platform-trust",
    "proof_role": "Run or disposition the same trust, probe, and negative contract on Linux and native Windows.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
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
    "defer_reason": "Requires native Linux and Windows runners after implementation; absent runner evidence remains blocked, not passed."
  },
  {
    "lane": "exact-head-hygiene",
    "proof_role": "Reject unrelated changes and support exact-head review.",
    "acceptance_ids": [
      "AC-6",
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

- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test local_tls`
- `bash adl/tools/test_v0917_html_observatory_integrated_proof.sh`
- `curl --config .csdlc/prepared/issues/5800/curl-observatory-https.conf`
- `bash adl/tools/run_owner_validation_lane.sh runtime`
- `git diff --check`

## Failure Semantics

Fail closed on certificate warnings, trust bypasses, SAN mismatch, partial replacement, private-material exposure, or HTTP/HTTPS configuration drift.

## Handoff

Retain typed evidence before convergence.
