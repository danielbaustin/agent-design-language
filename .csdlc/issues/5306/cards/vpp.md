# Validation Planning Prompt

Template: 1.0.0

Issue: 5306

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5306/retained/design.md

Diagram: .csdlc/issues/5306/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-v2-full",
    "proof_role": "Prove independent v2 remains green after each deletion slice",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 6000,
    "budget_tokens": 30000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "parallel_group": "local-rust",
    "defer_reason": null
  },
  {
    "lane": "eligibility-recompute",
    "proof_role": "Recompute exact removal and retained-surface truth",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 1200,
    "budget_tokens": 5000,
    "argv": [
      "csdlc-eligibility",
      "evaluate",
      "--repo",
      ".",
      "--request",
      "csdlc-v2/operator/eligibility-request.json"
    ],
    "parallel_group": "authority",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml`
- `csdlc-eligibility evaluate --repo . --request csdlc-v2/operator/eligibility-request.json`

## Failure Semantics

Fail closed with zero additional deletion; restore or repair only the current bounded slice.

## Handoff

Retain typed evidence before convergence.
