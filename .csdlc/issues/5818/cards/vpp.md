# Validation Planning Prompt

Template: 1.0.0

Issue: 5818

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5818/design.md

Diagram: .csdlc/prepared/issues/5818/diagram.mmd

## Selected Lanes

[
  {
    "lane": "activation-evidence-contract",
    "proof_role": "Require the fixed canonical denominator, parse structured data, resolve links, compare versions, and reject historical changes.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5818/validate-activation.rb"
    ],
    "parallel_group": "docs-contract",
    "defer_reason": null
  },
  {
    "lane": "cargo-version-parity",
    "proof_role": "Prove locked ADL workspace/package metadata is internally consistent from the repository root.",
    "acceptance_ids": [
      "AC-3",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "metadata",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--format-version",
      "1"
    ],
    "parallel_group": "metadata",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors and support exact-revision review.",
    "acceptance_ids": [
      "AC-7"
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

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/5818/validate-activation.rb`
- `cargo metadata --locked --manifest-path adl/Cargo.toml --format-version 1`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
