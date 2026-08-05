# Validation Planning Prompt

Template: 1.0.0

Issue: 5748

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5748/design.md

Diagram: .csdlc/prepared/issues/5748/diagram.mmd

## Selected Lanes

[
  {
    "lane": "terminal-inventory-and-full-receipt-integrity",
    "proof_role": "Verify exact-head owner-binary provenance, the complete retained live closed v0.91.8 universe, claim-free receipt-backed terminal projections, remote merged-PR and observed-head parity, zero unresolved exceptions, and the explicit per-issue closeout/prune result report.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 9000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5748/validate-final-inventory.sh"
    ],
    "parallel_group": "local-inventory",
    "defer_reason": null
  },
  {
    "lane": "inventory-path-guard-regression",
    "proof_role": "Prove final, parent-component, and dangling symlinks fail closed in the aggregate validator.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5748/validate-final-inventory.sh",
      "--self-test-path-guards"
    ],
    "parallel_group": "local-fast",
    "defer_reason": null
  },
  {
    "lane": "terminal-receipt-doctor-regression",
    "proof_role": "Prove doctor rejects tampered receipt digests, authored-artifact drift, and symlinked receipts while accepting canonical terminal authority.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate7_lifecycle",
      "no_pr_closeout_produces_doctor_valid_terminal_state",
      "--",
      "--exact"
    ],
    "parallel_group": "local-fast",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-current-full-suite",
    "proof_role": "Run every current C-SDLC v2 library, binary, integration, lifecycle, GitHub, and doc-test target at the final source revision.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 2400,
    "budget_tokens": 16000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "parallel_group": "current-source-rust",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-current-strict-clippy",
    "proof_role": "Reject warnings across every current C-SDLC v2 target at the final source revision.",
    "acceptance_ids": [
      "AC-3",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "current-source-rust",
    "defer_reason": null
  },
  {
    "lane": "aggregate-diff-hygiene",
    "proof_role": "Reject whitespace errors across the complete origin/main aggregate change.",
    "acceptance_ids": [
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main..HEAD"
    ],
    "parallel_group": "local-fast",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash .csdlc/prepared/issues/5748/validate-final-inventory.sh`
- `bash .csdlc/prepared/issues/5748/validate-final-inventory.sh --self-test-path-guards`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate7_lifecycle no_pr_closeout_produces_doctor_valid_terminal_state -- --exact`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml`
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`
- `git diff --check origin/main..HEAD`

## Failure Semantics

Fail closed on missing receipt, stale identity, unsupported disposition correction, dirty-worktree conflict, doctor failure, receipt mismatch, or any forbidden route.

## Handoff

Retain typed evidence before convergence.
