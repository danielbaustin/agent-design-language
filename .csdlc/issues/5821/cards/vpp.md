# Validation Planning Prompt

Template: 1.0.0

Issue: 5821

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5821/design.md

Diagram: .csdlc/prepared/issues/5821/diagram.mmd

## Selected Lanes

[
  {
    "lane": "child-wave-ledger",
    "proof_role": "Parse the retained design and require exactly sixteen ordered child identities, unique protected paths, resolvable child dependencies, and the separate implementation umbrella contract.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5821/validate-child-wave.rb"
    ],
    "parallel_group": "planning-gate",
    "defer_reason": null
  },
  {
    "lane": "architecture-threat-packet-hygiene",
    "proof_role": "Reject malformed or whitespace-damaged architecture, threat-model, ledger, diagram, and issue-card changes before independent review.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "planning-gate",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5821/validate-child-wave.rb`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
