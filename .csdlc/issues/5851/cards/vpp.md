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
    "lane": "independent-live-universe",
    "proof_role": "Independently reconstruct the issue denominator from the wave and reread every GitHub issue/PR/check/review plus typed terminal/claim state; reject upstream-row slicing. [preexec_rejection exit=1 diagnostic_sha256=9f5d21d76b31ce684331da6064f53c458830d54ed7afe254f135b8080a66832f]",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 420,
    "budget_tokens": 3800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5851/validate-readiness-review.rb",
      "comparison"
    ],
    "parallel_group": "comparison",
    "defer_reason": null
  },
  {
    "lane": "handoff-review",
    "proof_role": "Validate exact-head reviewer identity, findings dispositions, artifact digests, and v0.93 non-activation boundaries. [preexec_rejection exit=1 diagnostic_sha256=277dfcf8324b19f5028dda3387ec4673e000557794de264430f4ff9a2d987287]",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
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
      "handoff"
    ],
    "parallel_group": "handoff",
    "defer_reason": null
  },
  {
    "lane": "readiness-negatives",
    "proof_role": "Run every negative fixture through the actual independent comparison or handoff validator and require digest-bound failure. [preexec_rejection exit=1 diagnostic_sha256=13ca224eba10443b1e2f4f81446f6dd6bbaa71e51715513fc3e05fb3b1f4e732]",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 360,
    "budget_tokens": 3300,
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
    "proof_role": "Validate the exact six-card bundle and design approval.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
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
