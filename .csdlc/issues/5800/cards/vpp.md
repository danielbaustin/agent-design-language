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
    "proof_role": "Run focused Rust tests for certificate generation, reuse, replacement, trust inputs, and negative cases.",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-6",
      "AC-7"
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
      "adl-runtime/Cargo.toml",
      "--test",
      "local_tls"
    ],
    "parallel_group": "tls",
    "defer_reason": null
  },
  {
    "lane": "observatory-https-health",
    "proof_role": "Prove verified Observatory HTML access at the configured HTTPS endpoint; browser trust evidence is retained alongside this lane.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "curl",
      "--config",
      ".csdlc/prepared/issues/5800/curl-observatory-https.conf"
    ],
    "parallel_group": "browser",
    "defer_reason": "Requires the configured local runtime and trusted Observatory server to be running."
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

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test local_tls`
- `curl --config .csdlc/prepared/issues/5800/curl-observatory-https.conf`
- `git diff --check`

## Failure Semantics

Fail closed on certificate warnings, trust bypasses, SAN mismatch, partial replacement, private-material exposure, or HTTP/HTTPS configuration drift.

## Handoff

Retain typed evidence before convergence.
