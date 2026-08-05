# Validation Planning Prompt

Template: 1.0.0

Issue: 5873

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5873/design.md

Diagram: .csdlc/prepared/issues/5873/diagram.mmd

## Selected Lanes

[
  {
    "lane": "exact-child-tests",
    "proof_role": "Exact nextest target distributed_placement proves deterministic selection, capacity limits, stale input denial, no eligible target, policy bounds, and fenced-node exclusion.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_placement",
      "--no-tests=fail"
    ],
    "parallel_group": "child",
    "defer_reason": null
  },
  {
    "lane": "exact-revision-proof-receipt",
    "proof_role": "Recompute source, command, nonzero test, artifact, negative-case, and native receipt bindings. [preexec_rejection exit=1 diagnostic_sha256=e4f942cb06eb5978c9eaa2d637af4b9677a8243f40c2a23f516d58b2d1466e7d]",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5873/validate-proof-receipt.rb"
    ],
    "parallel_group": "receipt",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo nextest run --manifest-path adl-runtime/Cargo.toml --test distributed_placement --no-tests=fail`
- `ruby .csdlc/prepared/issues/5873/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on stale dependencies, path overlap, zero tests, invalid evidence, insecure fallback, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
