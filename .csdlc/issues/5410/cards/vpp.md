# Validation Planning Prompt

Template: 1.0.0

Issue: 5410

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: docs/reviews/v0.91.7/runtime-v3-5410/DESIGN.md

Diagram: docs/reviews/v0.91.7/runtime-v3-5410/DIAGRAM.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-full",
    "proof_role": "Prove live assembly, continuity, qualified time, control shutdown, and all crate behavior",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets"
    ],
    "parallel_group": "rust",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-strict",
    "proof_role": "Prove warning-free all-target integration",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "rust",
    "defer_reason": null
  },
  {
    "lane": "inventory-regression",
    "proof_role": "Prove the generator is deterministic and rejects stale or malformed inventory inputs",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "python3",
      "-m",
      "unittest",
      "adl-runtime-kernel/tools/test_generate_runtime_inventory.py"
    ],
    "parallel_group": "inventory",
    "defer_reason": null
  },
  {
    "lane": "inventory-current",
    "proof_role": "Fail closed unless the retained Runtime v3 counts match tracked source",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "python3",
      "adl-runtime-kernel/tools/generate_runtime_inventory.py",
      "--check"
    ],
    "parallel_group": "inventory",
    "defer_reason": null
  },
  {
    "lane": "patch-integrity",
    "proof_role": "Prove the bounded implementation patch has no whitespace defects",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 200,
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

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --all-targets`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`
- `python3 -m unittest adl-runtime-kernel/tools/test_generate_runtime_inventory.py`
- `python3 adl-runtime-kernel/tools/generate_runtime_inventory.py --check`
- `git diff --check`

## Failure Semantics

Fail closed on missing bindings, invalid signed continuity, unqualified time, stale generated inventory, budget breach, or review findings.

## Handoff

Retain typed evidence before convergence.
