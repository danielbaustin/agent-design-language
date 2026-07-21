# Validation Planning Prompt

Template: 1.0.0

Issue: 5587

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5587/design.md

Diagram: .csdlc/prepared/issues/5587/diagram.mmd

## Selected Lanes

[
  {
    "lane": "gws-drive-sync-tests",
    "proof_role": "Prove native contract, exact readback, approval, and failure behavior",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "adl_gws_drive_sync"
    ],
    "parallel_group": "gws-focused",
    "defer_reason": null
  },
  {
    "lane": "gws-context-mirror-tests",
    "proof_role": "Prove recursive path preservation and truthful reporting",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "adl_gws_context_mirror"
    ],
    "parallel_group": "gws-focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test adl_gws_drive_sync`
- `cargo test adl_gws_context_mirror`

## Failure Semantics

Fail closed on missing approval, credentials, path escape, ambiguous remote children, API failure, or content mismatch.

## Handoff

Retain typed evidence before convergence.
