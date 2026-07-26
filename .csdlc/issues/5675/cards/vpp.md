# Validation Planning Prompt

Template: 1.0.0

Issue: 5675

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5675/design.md

Diagram: .csdlc/prepared/issues/5675/diagram.mmd

## Selected Lanes

[
  {
    "lane": "provider-adapter-focused",
    "proof_role": "focused adapter tests for routing, budgets, envelopes, and redaction",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "provider-live-probe",
    "proof_role": "credentialed Kimi and MiniMax adapter calls with truthful credit disposition",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 1140,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml"
    ],
    "parallel_group": "local",
    "defer_reason": "provider account balance may block successful completion"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path adl/Cargo.toml`
- `cargo test --manifest-path adl/Cargo.toml`

## Failure Semantics

Fail closed on unsupported routes, malformed envelopes, missing credentials, unbounded budgets, or redaction failures; preserve live credit blockers as evidence.

## Handoff

Retain typed evidence before convergence.
