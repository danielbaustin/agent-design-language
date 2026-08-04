# Validation Planning Prompt

Template: 1.0.0

Issue: 5352

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5352/retained/design.md

Diagram: .csdlc/issues/5352/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "handoff-document-contract",
    "proof_role": "Exact row-bound handoff contract",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5352/validate_handoff.rb",
      "--final"
    ],
    "parallel_group": "wp21-focused",
    "defer_reason": null
  },
  {
    "lane": "dependency-ancestry",
    "proof_role": "Recorded-baseline exact merge ancestry",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5352/validate_dependency_ancestry.rb",
      "--final"
    ],
    "parallel_group": "wp21-focused",
    "defer_reason": null
  },
  {
    "lane": "implemented-packet",
    "proof_role": "Current implemented lifecycle packet truth",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5352/validate_implemented.rb"
    ],
    "parallel_group": "wp21-focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/5352/validate_handoff.rb --final`
- `ruby .csdlc/prepared/issues/5352/validate_dependency_ancestry.rb --final`
- `ruby .csdlc/prepared/issues/5352/validate_implemented.rb`

## Failure Semantics

Fail closed on baseline drift, substituted row identity, missing ancestry, stale lifecycle proof, unresolved review findings, or out-of-scope publication.

## Handoff

Retain typed evidence before convergence.
