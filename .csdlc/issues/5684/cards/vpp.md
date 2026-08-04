# Validation Planning Prompt

Template: 1.0.0

Issue: 5684

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5684/design.md

Diagram: .csdlc/issues/5684/diagram.mmd

## Selected Lanes

[
  {
    "lane": "shared-resilience",
    "proof_role": "shared retry/backoff crate compiles and behaves deterministically",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-resilience/Cargo.toml"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "github-split",
    "proof_role": "split binaries reject wrong surfaces and issue marker readback is exact",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "install-coexistence",
    "proof_role": "Gate 10A and stable install verify every required owner binary",
    "acceptance_ids": [
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "docs-guidance",
    "proof_role": "Current operator docs and skills route split GitHub surfaces and forbid deleted prompt-wrapper invocation",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a",
      "current_bootstrap_guidance_does_not_call_deleted_prompt_wrapper"
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

- `cargo test --manifest-path adl-resilience/Cargo.toml`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_github_actions`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate10a`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate10a current_bootstrap_guidance_does_not_call_deleted_prompt_wrapper`

## Failure Semantics

Fail closed; do not publish if install/coexistence or split-routing proof is missing.

## Handoff

Retain typed evidence before convergence.
