# Validation Planning Prompt

Template: 1.0.0

Issue: 5542

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5542/retained/design.md

Diagram: .csdlc/issues/5542/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp17-post-merge-docs",
    "proof_role": "Prove closeout truth, bridge precedence, date semantics, links, structured docs, and diff hygiene",
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
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/4644/validate_docs_alignment.rb",
      "."
    ],
    "parallel_group": "docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/4644/validate_docs_alignment.rb .`

## Failure Semantics

Fail closed on stale live truth, active claim collision, bridge bypass, ambiguous date semantics, or validation failure.

## Handoff

Retain typed evidence before convergence.
