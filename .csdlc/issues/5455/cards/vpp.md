# Validation Planning Prompt

Template: 1.0.0

Issue: 5455

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5455/design.md

Diagram: .csdlc/prepared/issues/5455/diagram.mmd

## Selected Lanes

[
  {
    "lane": "owner-provenance",
    "proof_role": "Gate 10A install, provenance, and stable editor execution",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a"
    ],
    "parallel_group": "csdlc-v2-focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate10a`

## Failure Semantics

Fail closed before stable owner-binary use.

## Handoff

Retain typed evidence before convergence.
