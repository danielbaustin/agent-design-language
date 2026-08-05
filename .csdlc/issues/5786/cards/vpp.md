# Validation Planning Prompt

Template: 1.0.0

Issue: 5786

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5786/design.md

Diagram: .csdlc/prepared/issues/5786/diagram.mmd

## Selected Lanes

[
  {
    "lane": "pinned-deletion-denominator",
    "proof_role": "Reconstruct the immutable pre-change adl/src path, blob, file-count, and LoC denominator from its ancestral Git SHA; require complete dispositions, derived reduction and references, clean install, and native macOS/Linux exact-head proof. [preexec_rejection exit=1 diagnostic_sha256=d956afaaec4fe86f37e70358e697484a58e6fa8e5c34e80fbc23786c38157c50]",
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
    "budget_seconds": 1800,
    "budget_tokens": 9000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5786/validate-reduction.rb"
    ],
    "parallel_group": "reduction",
    "defer_reason": null
  },
  {
    "lane": "typed-card-doctor",
    "proof_role": "Validate the exact rendered six-card bundle, cross-card references, design approval, digests, statuses, and canonical issue record.",
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
      "5786"
    ],
    "parallel_group": "typed-readback",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/5786/validate-reduction.rb`
- `csdlc-doctor --repo . --issue 5786`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
