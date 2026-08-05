# Validation Planning Prompt

Template: 1.0.0

Issue: 5842

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5842/design.md

Diagram: .csdlc/prepared/issues/5842/diagram.mmd

## Selected Lanes

[
  {
    "lane": "semantic-quality-matrix",
    "proof_role": "Require the exact feature and critical-path denominator; validate issue/PR/review/merge ancestry plus semantic validation, negative, integration, platform, and typed-terminal evidence at the reviewed SHA. [preexec_rejection exit=1 diagnostic_sha256=c8553118295145212ff2b53c59e52efe32ae3bd96440a4bbe54a948978d9da9c]",
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
      ".csdlc/prepared/issues/5842/validate-quality-gate.rb",
      "matrix"
    ],
    "parallel_group": "gate",
    "defer_reason": null
  },
  {
    "lane": "reconstructed-quality-negatives",
    "proof_role": "Execute forged evidence classes through the real matrix validator and require digest-bound rejection. [preexec_rejection exit=1 diagnostic_sha256=4c157b5ba213a95a8291346057c269ed2edc4db0f70c71d20c3c49417a430889]",
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
      ".csdlc/prepared/issues/5842/validate-quality-gate.rb",
      "negative"
    ],
    "parallel_group": "negative",
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
      "5842"
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

- `ruby .csdlc/prepared/issues/5842/validate-quality-gate.rb matrix`
- `ruby .csdlc/prepared/issues/5842/validate-quality-gate.rb negative`
- `csdlc-doctor --repo . --issue 5842`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
