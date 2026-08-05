# Validation Planning Prompt

Template: 1.0.0

Issue: 5764

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5764/retained/design.md

Diagram: .csdlc/issues/5764/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-control-routes",
    "proof_role": "focused Runtime v3 route and readiness/weather proof",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "control",
      "observatory_https_reads_are_public_and_report_weather_freshness"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "docs-and-diff",
    "proof_role": "docs/diff hygiene",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test control observatory_https_reads_are_public_and_report_weather_freshness`
- `git diff --check`

## Failure Semantics

Fail closed on stale lifecycle state, ambiguous weather/readiness semantics, failed route tests, or review findings.

## Handoff

Retain typed evidence before convergence.
