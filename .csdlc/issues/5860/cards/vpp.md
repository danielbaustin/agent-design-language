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
    "proof_role": "Prove the exact 58-issue documentation-only denominator, rollback, card, dependency, ownership, live-contract, artifact-digest, doctor, and preparation-control contract while excluding #5861",
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
  },
  {
    "lane": "v092-typed-doctor-parity",
    "proof_role": "Recompute all 58 typed doctor reports and reject pinned handoff evidence drift",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5860/validate-v092-doctors.rb"
    ],
    "parallel_group": "readiness-controls",
    "defer_reason": null
  },
  {
    "lane": "wp04-child-wave-preparation",
    "proof_role": "Prove the live WP-04 child mapping, approvals, null claims, ownership, rollback, and operator-visible umbrella contract",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5821/validate-child-wave.rb"
    ],
    "parallel_group": "readiness-controls",
    "defer_reason": null
  },
  {
    "lane": "wp04-implementation-wave-preparation",
    "proof_role": "Prove the sixteen-child WP-04 preparation contract and terminal integration gate required before WP-14 handoff",
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
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5862/validate-implementation-wave.rb"
    ],
    "parallel_group": "readiness-controls",
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
- `ruby .csdlc/prepared/issues/5860/validate-v092-doctors.rb`
- `ruby .csdlc/prepared/issues/5821/validate-child-wave.rb`
- `ruby .csdlc/prepared/issues/5862/validate-implementation-wave.rb`

## Failure Semantics

Fail closed on any placeholder, generic plan, pending design approval, invalid card, active preparation claim, or product path change.

## Handoff

Retain typed evidence before convergence.
