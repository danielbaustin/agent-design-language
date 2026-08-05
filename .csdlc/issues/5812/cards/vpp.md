# Validation Planning Prompt

Template: 1.0.0

Issue: 5812

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5812/design.md

Diagram: .csdlc/prepared/issues/5812/diagram.mmd

## Selected Lanes

[
  {
    "lane": "freedom-gate-module-tests",
    "proof_role": "Prove both defaults and unsafe retained-artifact rejection remain behaviorally unchanged.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 400,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--lib",
      "csm_freedom_gate::tests"
    ],
    "parallel_group": "rust",
    "defer_reason": null
  },
  {
    "lane": "freedom-gate-clippy",
    "proof_role": "Reproduce and eliminate the exact production-binary warnings with warnings denied.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 500,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl-gws-context-mirror",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "rust",
    "defer_reason": null
  },
  {
    "lane": "rust-format",
    "proof_role": "Reject unintended Rust formatting churn.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 100,
    "budget_tokens": 500,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl/Cargo.toml",
      "--all",
      "--",
      "--check"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "exact-path-scope-negative",
    "proof_role": "Reject Cargo metadata, dependency, Google Drive, and every unrelated product path rather than checking whitespace only.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5812/validate-path-scope.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors after the exact-path gate passes.",
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

- `cargo test --locked --manifest-path adl/Cargo.toml --lib csm_freedom_gate::tests`
- `cargo clippy --locked --manifest-path adl/Cargo.toml --bin adl-gws-context-mirror -- -D warnings`
- `cargo fmt --manifest-path adl/Cargo.toml --all -- --check`
- `ruby .csdlc/prepared/issues/5812/validate-path-scope.rb`
- `git diff --check`

## Failure Semantics

Fail closed on semantic drift, lint failure, unrelated changes, or missing focused proof.

## Handoff

Retain typed evidence before convergence.
