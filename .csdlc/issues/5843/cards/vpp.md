# Validation Planning Prompt

Template: 1.0.0

Issue: 5843

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5843/design.md

Diagram: .csdlc/prepared/issues/5843/diagram.mmd

## Selected Lanes

[
  {
    "lane": "complete-doc-release-universe",
    "proof_role": "Reconstruct every tracked changelog, feature, ADR, release, skill, guidance, external-launch, and v0.92 milestone surface; require evidence-bound rows, parsing, links, command output, version ownership, and redaction. [preexec_rejection exit=1 diagnostic_sha256=7b03d62fdc99cf072907826a13e1f73d3eeb1c48a6c3d067ba39e7cb839fb43f]",
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
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5843/validate-doc-release-truth.rb"
    ],
    "parallel_group": "docs",
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
      "5843"
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

- `ruby .csdlc/prepared/issues/5843/validate-doc-release-truth.rb`
- `csdlc-doctor --repo . --issue 5843`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
