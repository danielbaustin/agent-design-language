# Validation Planning Prompt

Template: 1.0.0

Issue: 5860

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5860/design.md

Diagram: .csdlc/prepared/issues/5860/diagram.mmd

## Selected Lanes

[
  {
    "lane": "v092-readiness-matrix",
    "proof_role": "Reject placeholder or generic cards and prove exact design, card, dependency, ownership, live-contract, artifact-digest, and doctor readiness for all 58 execution issues while excluding #5861",
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
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 18000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5860/validate-v092-readiness.rb",
      "--verify-live"
    ],
    "parallel_group": "readiness-integration",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5860/validate-v092-readiness.rb --verify-live`

## Failure Semantics

Fail closed on any placeholder, generic plan, pending design approval, invalid card, active preparation claim, or product path change.

## Handoff

Retain typed evidence before convergence.
