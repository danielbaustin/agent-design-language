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
    "lane": "chrome-trusted-live-observatory",
    "proof_role": "Use the issue-delivered Playwright validator to open real Chrome against the live HTTPS Observatory and Runtime, reject interstitials or TLS console/network errors, and prove HTML, health, readiness, and feed access with browser-visible evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "node",
      "adl/tools/validate_v092_browser_trusted_observatory.mjs",
      "--browser",
      "chrome",
      "--require-trusted-tls",
      "--runtime-url",
      "https://localhost:20997",
      "--observatory-url",
      "https://localhost:8765"
    ],
    "parallel_group": "live-browser",
    "defer_reason": "The named validator is an issue 5800 implementation deliverable and requires explicit operator trust plus both real HTTPS listeners."
  },
  {
    "lane": "verified-endpoint-probe",
    "proof_role": "Independently verify the live Runtime HTTPS identity and endpoint responses without substituting curl for browser trust.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
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
    "parallel_group": "live-browser",
    "defer_reason": "Requires the real certificate identity and Runtime listener; this lane cannot satisfy AC-1."
  },
  {
    "lane": "native-platform-trust-matrix",
    "proof_role": "Run the same issue-delivered browser validator on native macOS, Linux, and Windows evidence lanes and fail closed on any missing required platform disposition.",
    "acceptance_ids": [
      "AC-1",
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
      "node",
      "adl/tools/validate_v092_browser_trusted_observatory.mjs",
      "--require-native-platform-evidence",
      "macos,linux,windows"
    ],
    "parallel_group": "platform",
    "defer_reason": "Requires native runners and host trust setup; missing native proof remains blocked, never inferred from macOS."
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

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test local_tls`
- `node adl/tools/validate_v092_browser_trusted_observatory.mjs --browser chrome --require-trusted-tls --runtime-url https://localhost:20997 --observatory-url https://localhost:8765`
- `curl --config .csdlc/prepared/issues/5800/curl-observatory-https.conf`
- `node adl/tools/validate_v092_browser_trusted_observatory.mjs --require-native-platform-evidence macos,linux,windows`
- `git diff --check`

## Failure Semantics

Fail closed on certificate warnings, trust bypasses, SAN mismatch, partial replacement, private-material exposure, or HTTP/HTTPS configuration drift.

## Handoff

Retain typed evidence before convergence.
