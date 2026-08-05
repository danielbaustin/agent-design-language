# Validation Planning Prompt

Template: 1.0.0

Issue: 5854

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5854/design.md

Diagram: .csdlc/prepared/issues/5854/diagram.mmd

## Selected Lanes

[
  {
    "lane": "v092-sprint-package",
    "proof_role": "Sprint membership, packet completeness, child authority boundaries, and review-ready coordination truth",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5817/validate-v092-package.rb"
    ],
    "parallel_group": "v092-docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/5817/validate-v092-package.rb`

## Failure Semantics

Fail closed on overlapping ownership, unmet serial gates, missing child readiness, or unsupported completion claims.

## Handoff

Retain typed evidence before convergence.
