# Validation Planning Prompt

Template: 1.0.0

Issue: 5658

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5658/design.md

Diagram: .csdlc/prepared/issues/5658/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-lifecycle-focused",
    "proof_role": "Prove bound-worktree lifecycle materialization and fail-closed primary-main behavior",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "-p",
      "csdlc-v2",
      "gate7_lifecycle",
      "--test",
      "gate7_lifecycle"
    ],
    "parallel_group": "csdlc-v2",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove the patch is whitespace-clean",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test -p csdlc-v2 gate7_lifecycle --test gate7_lifecycle`
- `git diff --check`

## Failure Semantics

Fail closed on any lifecycle-root ambiguity, root-main write attempt outside explicit bootstrap/read-only operation, stale claim, stale digest, lock mismatch, or exact-revision mismatch.

## Handoff

Retain typed evidence before convergence.
