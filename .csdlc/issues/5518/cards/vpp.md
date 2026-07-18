# Validation Planning Prompt

Template: 1.0.0

Issue: 5518

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5518/design.md

Diagram: .csdlc/prepared/issues/5518/diagram.mmd

## Selected Lanes

[
  {
    "lane": "terminal-plan-repair-tests",
    "proof_role": "Prove forward-only atomic repair and rollback",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "terminal_plan_repair"
    ],
    "parallel_group": "rust",
    "defer_reason": null
  },
  {
    "lane": "target-doctor",
    "proof_role": "Prove #5516 terminal plan and receipt parity",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "csdlc-doctor",
      "--issue",
      "5516"
    ],
    "parallel_group": "records",
    "defer_reason": null
  },
  {
    "lane": "exact-review",
    "proof_role": "Prove bounded implementation and terminal truth",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "review",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml terminal_plan_repair`
- `csdlc-doctor --issue 5516`
- `git diff --check`

## Failure Semantics

Fail closed on stale identity, missing authority, nonterminal target, invalid step transition, receipt mismatch, rollback failure, runtime changes, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
