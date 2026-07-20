# Validation Planning Prompt

Template: 1.0.0

Issue: 5495

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5495/retained/design.md

Diagram: .csdlc/issues/5495/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "gate5-publication-metadata",
    "proof_role": "review/publication stale-loop regression and fail-closed source drift",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5"
    ],
    "parallel_group": "csdlc-v2",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate5`

## Failure Semantics

Fail closed on unknown paths, malformed proof, revision mismatch, or substantive changes.

## Handoff

Retain typed evidence before convergence.
