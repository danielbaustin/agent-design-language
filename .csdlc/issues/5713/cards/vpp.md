# Validation Planning Prompt

Template: 1.0.0

Issue: 5713

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5713/retained/design.md

Diagram: .csdlc/issues/5713/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-local-tls-focused",
    "proof_role": "Prove rcgen local self-signed TLS bootstrap, reuse, replacement, and external preservation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "local_tls"
    ],
    "parallel_group": "runtime-v3-local-tls",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-local-tls-fmt",
    "proof_role": "Prove Runtime v3 kernel formatting stays clean",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--all",
      "--check"
    ],
    "parallel_group": "runtime-v3-local-tls",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-local-tls-diff-check",
    "proof_role": "Prove tracked text changes have clean whitespace",
    "acceptance_ids": [
      "AC-8",
      "AC-10"
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
    "parallel_group": "runtime-v3-local-tls",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-local-tls-exact-review",
    "proof_role": "Record one exact GPT-5.5 pre-PR review before ready publication",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "csdlc-review",
      "record"
    ],
    "parallel_group": "runtime-v3-local-tls",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-local-tls-ready-publication",
    "proof_role": "Publish one ready PR whose body closes issue #5713",
    "acceptance_ids": [
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1000,
    "argv": [
      "csdlc-publish",
      "publish"
    ],
    "parallel_group": "runtime-v3-local-tls",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path adl-runtime/Cargo.toml --test local_tls`
- `cargo fmt --manifest-path adl-runtime/Cargo.toml --all --check`
- `git diff --check`
- `csdlc-review record`
- `csdlc-publish publish`

## Failure Semantics

Fail closed on claim collision, stale generation, implicit production local TLS, private-key leakage, failed focused proof, unavailable exact GPT-5.5 review, or stale/missing review before publication.

## Handoff

Retain typed evidence before convergence.
