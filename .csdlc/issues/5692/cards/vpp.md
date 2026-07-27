# Validation Planning Prompt

Template: 1.0.0

Issue: 5692

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5692/design.md

Diagram: .csdlc/prepared/issues/5692/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-publication-tests",
    "proof_role": "focused policy/verifier regression proof",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--test",
      "gate6"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "format-and-diff",
    "proof_role": "diff hygiene",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 250,
    "argv": [
      "git",
      "diff",
      "--check"
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

- `cargo test --locked --test gate6`
- `git diff --check`

## Failure Semantics

Fail closed on missing closing keyword, stale typed state, failed focused proof, or review findings.

## Handoff

Retain typed evidence before convergence.
