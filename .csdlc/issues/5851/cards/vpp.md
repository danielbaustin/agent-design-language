# Validation Planning Prompt

Template: 1.0.0

Issue: 5851

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5851/design.md

Diagram: .csdlc/prepared/issues/5851/diagram.mmd

## Selected Lanes

[
  {
    "lane": "independent-derived-comparison",
    "proof_role": "Revalidate the WP-28A live universe, rebuild its material fields independently, and require exact row and source-digest equality.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5851/validate-readiness-review.rb",
      "comparison"
    ],
    "parallel_group": "comparison",
    "defer_reason": null
  },
  {
    "lane": "digest-bound-handoff-review",
    "proof_role": "Bind reviewer identity, reviewed HEAD, every reviewed artifact digest, finding disposition, and candidate-only v0.93 claim boundary.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5851/validate-readiness-review.rb",
      "handoff"
    ],
    "parallel_group": "handoff",
    "defer_reason": null
  },
  {
    "lane": "exercised-review-negatives",
    "proof_role": "Require the full missing, stale, red, active-claim, absent-receipt, dirty, partial, duplicate, premature-closeout, and activation negative universe to produce validator failures.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5851/validate-readiness-review.rb",
      "negative"
    ],
    "parallel_group": "negative",
    "defer_reason": null
  },
  {
    "lane": "typed-card-doctor",
    "proof_role": "Validate the exact rendered six-card bundle, cross-card references, digests, statuses, and canonical issue record.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5851"
    ],
    "parallel_group": "typed-readback",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/5851/validate-readiness-review.rb comparison`
- `ruby .csdlc/prepared/issues/5851/validate-readiness-review.rb handoff`
- `ruby .csdlc/prepared/issues/5851/validate-readiness-review.rb negative`
- `csdlc-doctor --repo . --issue 5851`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
