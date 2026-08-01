# Validation Planning Prompt

Template: 1.0.0

Issue: 5756

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/designs/5756/design.md

Diagram: .csdlc/designs/5756/diagram.mmd

## Selected Lanes

[
  {
    "lane": "provider-minimax-tests",
    "proof_role": "Focused MiniMax adapter regressions for structured 1008 billing classification on success and non-success HTTP responses.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--offline",
      "--manifest-path",
      "adl/Cargo.toml",
      "provider_adapter::tests::minimax",
      "--lib"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "provider-non-minimax-tests",
    "proof_role": "Focused OpenAI, Anthropic, DeepSeek, and generic hosted provider regressions for bare 1008 non-billing classification.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--offline",
      "--manifest-path",
      "adl/Cargo.toml",
      "provider_adapter::tests::non_minimax",
      "--lib"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "strict-clippy",
    "proof_role": "Strict lint proof for the touched provider adapter crate surface.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "clippy",
      "--offline",
      "--manifest-path",
      "adl/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
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

- `cargo test --offline --manifest-path adl/Cargo.toml provider_adapter::tests::minimax --lib`
- `cargo test --offline --manifest-path adl/Cargo.toml provider_adapter::tests::non_minimax --lib`
- `cargo clippy --offline --manifest-path adl/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed, preserve request/evidence artifacts, and report the typed blocker without widening scope.

## Handoff

Retain typed evidence before convergence.
