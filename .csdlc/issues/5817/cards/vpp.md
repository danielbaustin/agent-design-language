# Validation Planning Prompt

Template: 1.0.0

Issue: 5817

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5817/design.md

Diagram: .csdlc/prepared/issues/5817/diagram.mmd

## Selected Lanes

[
  {
    "lane": "v092-planning-package",
    "proof_role": "Validate YAML and v0.92 identity, all milestone-local Markdown links, dependency acyclicity, 39 unique WP mappings, 41 initialized child and supporting records, 492 typed card artifacts and schema identities, five complete sprint umbrellas, required feature contracts, delivery gates, active-wave wording, and scope alignment.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5817/validate-v092-package.rb"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-loop-requalification",
    "proof_role": "Requalify the historical #5104 loop contract against current Runtime v3 bounded execution, replay, cancellation, checkpoint, mutation, and forgery-rejection behavior.",
    "acceptance_ids": [
      "AC-3"
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
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "/Volumes/FastWork/adl-wp-5817/target",
      "--test",
      "reasoning"
    ],
    "parallel_group": "runtime",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors in the complete WP-01 candidate.",
    "acceptance_ids": [
      "AC-6",
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
    "parallel_group": "docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5817/validate-v092-package.rb`
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --target-dir /Volumes/FastWork/adl-wp-5817/target --test reasoning`
- `git diff --check`

## Failure Semantics

Fail closed on contradictory evidence, duplicates, cyclic dependencies, invalid cards, or scope widening; record named gaps instead of completion claims.

## Handoff

Retain typed evidence before convergence.
