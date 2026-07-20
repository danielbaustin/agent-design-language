# Validation Planning Prompt

Template: 1.0.0

Issue: 5547

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5547/retained/design.md

Diagram: .csdlc/issues/5547/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-v2-doctor",
    "proof_role": "Validate generated #5547 v2 cards and lifecycle state.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5547"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "focused-csdlc-identity-tests",
    "proof_role": "Run focused Rust tests if #5547 changes C-SDLC identity behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "test",
      "-p",
      "csdlc-v2",
      "publication"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5547`
- `cargo test -p csdlc-v2 publication`

## Failure Semantics

Fail closed: do not publish #5547 unless the identity contract, ownership split plan, validation truth, and any deferred residual routing are explicit.

## Handoff

Retain typed evidence before convergence.
